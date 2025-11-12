use actix_web::web::Bytes;
use image::{imageops::FilterType, ImageFormat};
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

/// Avatar size configurations
pub struct AvatarSizes {
    pub thumbnail: u32, // 64x64 for lists
    pub small: u32,     // 128x128 for cards
    pub medium: u32,    // 256x256 for profiles
    pub large: u32,     // 512x512 for full view
}

impl Default for AvatarSizes {
    fn default() -> Self {
        Self {
            thumbnail: 64,
            small: 128,
            medium: 256,
            large: 512,
        }
    }
}

/// Storage service for managing user avatars
pub struct StorageService {
    base_path: PathBuf,
    sizes: AvatarSizes,
}

impl StorageService {
    /// Create a new storage service
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
            sizes: AvatarSizes::default(),
        }
    }

    /// Save an avatar image with multiple sizes
    pub async fn save_avatar(
        &self,
        user_id: Uuid,
        image_data: Bytes,
        format: ImageFormat,
    ) -> Result<String, StorageError> {
        // Create user directory if it doesn't exist
        let user_dir = self.base_path.join(user_id.to_string());
        fs::create_dir_all(&user_dir)
            .await
            .map_err(|e| StorageError::Io(e.to_string()))?;

        // Load and validate image
        let img = image::load_from_memory(&image_data)
            .map_err(|e| StorageError::ImageProcessing(e.to_string()))?;

        // Determine file extension
        let ext = match format {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::WebP => "webp",
            _ => return Err(StorageError::UnsupportedFormat),
        };

        // Save original (capped at large size)
        let original_path = user_dir.join(format!("avatar.{}", ext));
        let resized_original = if img.width() > self.sizes.large || img.height() > self.sizes.large
        {
            img.resize(self.sizes.large, self.sizes.large, FilterType::Lanczos3)
        } else {
            img.clone()
        };

        tokio::task::spawn_blocking({
            let path = original_path.clone();
            move || resized_original.save(path)
        })
        .await
        .map_err(|e| StorageError::Io(e.to_string()))?
        .map_err(|e| StorageError::ImageProcessing(e.to_string()))?;

        // Generate thumbnail
        let thumbnail = img.resize(
            self.sizes.thumbnail,
            self.sizes.thumbnail,
            FilterType::Lanczos3,
        );
        let thumb_path = user_dir.join(format!("avatar-thumb.{}", ext));
        tokio::task::spawn_blocking({
            let path = thumb_path.clone();
            move || thumbnail.save(path)
        })
        .await
        .map_err(|e| StorageError::Io(e.to_string()))?
        .map_err(|e| StorageError::ImageProcessing(e.to_string()))?;

        // Generate small size
        let small = img.resize(self.sizes.small, self.sizes.small, FilterType::Lanczos3);
        let small_path = user_dir.join(format!("avatar-small.{}", ext));
        tokio::task::spawn_blocking({
            let path = small_path.clone();
            move || small.save(path)
        })
        .await
        .map_err(|e| StorageError::Io(e.to_string()))?
        .map_err(|e| StorageError::ImageProcessing(e.to_string()))?;

        // Generate medium size
        let medium = img.resize(self.sizes.medium, self.sizes.medium, FilterType::Lanczos3);
        let medium_path = user_dir.join(format!("avatar-medium.{}", ext));
        tokio::task::spawn_blocking({
            let path = medium_path.clone();
            move || medium.save(path)
        })
        .await
        .map_err(|e| StorageError::Io(e.to_string()))?
        .map_err(|e| StorageError::ImageProcessing(e.to_string()))?;

        // Return URL path for the avatar
        Ok(format!("/avatars/{}/avatar.{}", user_id, ext))
    }

    /// Delete all avatar files for a user
    pub async fn delete_avatar(&self, user_id: Uuid) -> Result<(), StorageError> {
        let user_dir = self.base_path.join(user_id.to_string());

        if user_dir.exists() {
            fs::remove_dir_all(&user_dir)
                .await
                .map_err(|e| StorageError::Io(e.to_string()))?;
        }

        Ok(())
    }

    /// Get the file path for an avatar
    pub fn get_avatar_path(&self, user_id: Uuid, size: Option<&str>) -> PathBuf {
        let user_dir = self.base_path.join(user_id.to_string());

        // Try different file formats
        let filename = match size {
            Some("thumbnail") | Some("thumb") => "avatar-thumb",
            Some("small") => "avatar-small",
            Some("medium") => "avatar-medium",
            _ => "avatar",
        };

        // Check for existing files with different extensions
        for ext in &["png", "jpg", "webp"] {
            let path = user_dir.join(format!("{}.{}", filename, ext));
            if path.exists() {
                return path;
            }
        }

        // Default to png if not found
        user_dir.join(format!("{}.png", filename))
    }

    /// Check if avatar exists for a user
    pub async fn avatar_exists(&self, user_id: Uuid) -> bool {
        let path = self.get_avatar_path(user_id, None);
        path.exists()
    }

    /// Validate image data before processing
    pub fn validate_image(data: &[u8]) -> Result<ImageFormat, StorageError> {
        // Check file signature
        let format =
            image::guess_format(data).map_err(|e| StorageError::ImageProcessing(e.to_string()))?;

        // Only allow specific formats
        match format {
            ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP => Ok(format),
            _ => Err(StorageError::UnsupportedFormat),
        }
    }

    /// Get maximum file size (in bytes)
    pub const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5MB
}

/// Storage service errors
#[derive(Debug)]
pub enum StorageError {
    Io(String),
    ImageProcessing(String),
    UnsupportedFormat,
    FileTooLarge,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::ImageProcessing(e) => write!(f, "Image processing error: {}", e),
            Self::UnsupportedFormat => write!(
                f,
                "Unsupported image format. Only PNG, JPEG, and WebP are allowed"
            ),
            Self::FileTooLarge => write!(f, "File too large. Maximum size is 5MB"),
        }
    }
}

impl std::error::Error for StorageError {}

impl actix_web::ResponseError for StorageError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;

        match self {
            Self::UnsupportedFormat | Self::FileTooLarge => {
                actix_web::HttpResponse::build(StatusCode::BAD_REQUEST).json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            _ => actix_web::HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(
                serde_json::json!({
                    "error": "Internal server error"
                }),
            ),
        }
    }
}
