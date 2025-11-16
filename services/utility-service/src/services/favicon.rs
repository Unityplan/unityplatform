use anyhow::{Context, Result};
use image::imageops::FilterType;
use image::ImageFormat;
use redis::AsyncCommands;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use url::Url;

const CACHE_TTL: u64 = 604800; // 7 days in seconds
const FETCH_TIMEOUT: u64 = 5; // 5 seconds
const MAX_FILE_SIZE: usize = 1_048_576; // 1MB

pub struct FaviconService {
    http_client: Client,
    redis_client: redis::Client,
}

impl FaviconService {
    pub fn new(redis_client: redis::Client) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(FETCH_TIMEOUT))
            .user_agent("UnityPlatform-Utility/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            redis_client,
        }
    }

    /// Fetch a favicon for the given URL and size
    pub async fn get_favicon(&self, url_str: &str, size: u32) -> Result<(Vec<u8>, String, bool)> {
        // Parse and validate URL
        let url = Url::parse(url_str).context("Invalid URL format")?;
        self.validate_url(&url)?;

        let domain = url.host_str().context("No host in URL")?.to_string();
        let cache_key = format!("favicon:{}:{}", domain, size);

        // Check cache first
        if let Ok(cached) = self.get_from_cache(&cache_key).await {
            info!(domain = %domain, size = size, "Favicon cache HIT");
            let etag = self.compute_etag(&cached);
            return Ok((cached, etag, true)); // true = cache hit
        }

        info!(domain = %domain, size = size, "Favicon cache MISS - fetching");

        // Fetch favicon
        match self.fetch_and_process_favicon(&url, size).await {
            Ok(favicon_data) => {
                // Store in cache
                if let Err(e) = self.store_in_cache(&cache_key, &favicon_data).await {
                    warn!(error = ?e, "Failed to cache favicon");
                }

                let etag = self.compute_etag(&favicon_data);
                Ok((favicon_data, etag, false)) // false = cache miss
            }
            Err(e) => {
                error!(domain = %domain, error = ?e, "Failed to fetch favicon");
                // Return default icon
                let default_icon = self.generate_default_icon(size)?;
                let etag = self.compute_etag(&default_icon);
                Ok((default_icon, etag, false))
            }
        }
    }

    /// Validate URL to prevent SSRF attacks
    fn validate_url(&self, url: &Url) -> Result<()> {
        // Must be HTTP or HTTPS
        if !matches!(url.scheme(), "http" | "https") {
            anyhow::bail!("URL must be HTTP or HTTPS");
        }

        // Must have a host
        let host = url.host_str().context("URL must have a host")?;

        // Block localhost and private IPs
        if host == "localhost"
            || host == "127.0.0.1"
            || host == "::1"
            || host.starts_with("192.168.")
            || host.starts_with("10.")
            || host.starts_with("172.16.")
            || host.starts_with("172.17.")
            || host.starts_with("172.18.")
            || host.starts_with("172.19.")
            || host.starts_with("172.20.")
            || host.starts_with("172.21.")
            || host.starts_with("172.22.")
            || host.starts_with("172.23.")
            || host.starts_with("172.24.")
            || host.starts_with("172.25.")
            || host.starts_with("172.26.")
            || host.starts_with("172.27.")
            || host.starts_with("172.28.")
            || host.starts_with("172.29.")
            || host.starts_with("172.30.")
            || host.starts_with("172.31.")
        {
            anyhow::bail!("Cannot fetch favicon from localhost or private IP addresses");
        }

        Ok(())
    }

    /// Fetch favicon from website and process it
    async fn fetch_and_process_favicon(&self, url: &Url, size: u32) -> Result<Vec<u8>> {
        // Try common favicon locations
        let favicon_urls = vec![
            format!("{}://{}/favicon.ico", url.scheme(), url.host_str().unwrap()),
            format!("{}://{}/favicon.png", url.scheme(), url.host_str().unwrap()),
            format!(
                "https://www.google.com/s2/favicons?domain={}&sz={}",
                url.host_str().unwrap(),
                size
            ),
        ];

        for favicon_url in favicon_urls {
            debug!(url = %favicon_url, "Attempting to fetch favicon");

            match self.http_client.get(&favicon_url).send().await {
                Ok(response) if response.status().is_success() => {
                    let content_length = response.content_length().unwrap_or(0);
                    if content_length > MAX_FILE_SIZE as u64 {
                        warn!(size = content_length, "Favicon too large, skipping");
                        continue;
                    }

                    match response.bytes().await {
                        Ok(bytes) => {
                            if bytes.len() > MAX_FILE_SIZE {
                                warn!(size = bytes.len(), "Favicon too large after download");
                                continue;
                            }

                            // Process the image (resize if needed)
                            match self.process_image(&bytes, size) {
                                Ok(processed) => {
                                    info!(url = %favicon_url, "Successfully fetched and processed favicon");
                                    return Ok(processed);
                                }
                                Err(e) => {
                                    warn!(error = ?e, url = %favicon_url, "Failed to process image");
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            warn!(error = ?e, "Failed to read response bytes");
                            continue;
                        }
                    }
                }
                Ok(response) => {
                    debug!(status = %response.status(), "Non-success status");
                    continue;
                }
                Err(e) => {
                    debug!(error = ?e, "Failed to fetch favicon");
                    continue;
                }
            }
        }

        anyhow::bail!("Could not fetch favicon from any source")
    }

    /// Process and resize image to target size
    fn process_image(&self, data: &[u8], target_size: u32) -> Result<Vec<u8>> {
        let img = image::load_from_memory(data).context("Failed to decode image")?;

        // Resize to target size
        let resized = img.resize_exact(target_size, target_size, FilterType::Lanczos3);

        // Convert to PNG
        let mut buffer = Cursor::new(Vec::new());
        resized
            .write_to(&mut buffer, ImageFormat::Png)
            .context("Failed to encode PNG")?;

        Ok(buffer.into_inner())
    }

    /// Generate a default icon (simple colored square with first letter of domain)
    fn generate_default_icon(&self, size: u32) -> Result<Vec<u8>> {
        use image::{Rgba, RgbaImage};

        // Create a simple gray square
        let mut img = RgbaImage::from_pixel(size, size, Rgba([200, 200, 200, 255]));

        // Add a border
        for i in 0..size {
            img.put_pixel(i, 0, Rgba([150, 150, 150, 255]));
            img.put_pixel(i, size - 1, Rgba([150, 150, 150, 255]));
            img.put_pixel(0, i, Rgba([150, 150, 150, 255]));
            img.put_pixel(size - 1, i, Rgba([150, 150, 150, 255]));
        }

        let mut buffer = Cursor::new(Vec::new());
        img.write_to(&mut buffer, ImageFormat::Png)
            .context("Failed to encode default icon")?;

        Ok(buffer.into_inner())
    }

    /// Get favicon from Redis cache
    async fn get_from_cache(&self, key: &str) -> Result<Vec<u8>> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .context("Failed to connect to Redis")?;

        let data: Vec<u8> = conn.get(key).await.context("Cache miss")?;

        // Don't return empty cached data
        if data.is_empty() {
            anyhow::bail!("Cached data is empty");
        }

        Ok(data)
    }

    /// Store favicon in Redis cache
    async fn store_in_cache(&self, key: &str, data: &[u8]) -> Result<()> {
        // Don't cache empty data
        if data.is_empty() {
            anyhow::bail!("Cannot cache empty data");
        }

        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .context("Failed to connect to Redis")?;

        let _: () = conn
            .set_ex(key, data, CACHE_TTL)
            .await
            .context("Failed to set cache")?;

        Ok(())
    }

    /// Compute ETag for content
    fn compute_etag(&self, data: &[u8]) -> String {
        use base64::Engine;
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        format!(
            "\"{}\"",
            base64::engine::general_purpose::STANDARD.encode(&hash[..16])
        )
    }

    /// Check Redis health
    pub async fn check_redis_health(&self) -> (bool, Option<f64>) {
        let start = std::time::Instant::now();

        match self.redis_client.get_multiplexed_async_connection().await {
            Ok(mut conn) => {
                match redis::cmd("PING").query_async::<String>(&mut conn).await {
                    Ok(_) => {
                        let latency = start.elapsed().as_secs_f64() * 1000.0; // Convert to ms
                        (true, Some(latency))
                    }
                    Err(_) => (false, None),
                }
            }
            Err(_) => (false, None),
        }
    }
}
