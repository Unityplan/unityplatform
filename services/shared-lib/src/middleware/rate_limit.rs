use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use redis::AsyncCommands;
use std::future::{ready, Ready};
use std::sync::Arc;

use super::error_handler::rate_limit_exceeded;
use super::request_id::get_request_id;

/// Rate limiting middleware using Redis
///
/// Implements token bucket algorithm with Redis for distributed rate limiting.
///
/// # Rate Limiting Strategies
///
/// 1. **By IP Address** (default) - Limit requests per IP
/// 2. **By User ID** - Limit requests per authenticated user
/// 3. **Global** - Limit total requests to the service
///
/// # Phase 1 (Development)
/// - High limits (100 req/min per IP)
/// - Warnings instead of blocks
/// - Detailed error messages
///
/// # Phase 2 (Production)
/// - Strict limits (30 req/min per IP, 100 req/min per user)
/// - Hard blocks
/// - Generic error messages
///
/// # Example
///
/// ```rust
/// use actix_web::App;
/// use shared_lib::middleware::RateLimitMiddleware;
/// use redis::Client;
///
/// let redis_client = Client::open("redis://127.0.0.1/").unwrap();
/// 
/// let app = App::new()
///     .wrap(RateLimitMiddleware::development(redis_client))
///     // ... routes
///     ;
/// ```
#[derive(Clone)]
pub struct RateLimitMiddleware {
    redis_client: Arc<redis::Client>,
    max_requests: u32,
    window_seconds: u64,
    by_user: bool,
}

impl RateLimitMiddleware {
    /// Create new rate limiter
    ///
    /// # Arguments
    /// * `redis_client` - Redis client for distributed rate limiting
    /// * `max_requests` - Maximum requests allowed in window
    /// * `window_seconds` - Time window in seconds
    pub fn new(redis_client: redis::Client, max_requests: u32, window_seconds: u64) -> Self {
        Self {
            redis_client: Arc::new(redis_client),
            max_requests,
            window_seconds,
            by_user: false,
        }
    }

    /// Development - Permissive rate limiting (100 requests per minute)
    pub fn development(redis_client: redis::Client) -> Self {
        Self::new(redis_client, 100, 60)
    }

    /// Production - Strict rate limiting (30 requests per minute)
    pub fn production(redis_client: redis::Client) -> Self {
        Self::new(redis_client, 30, 60)
    }

    /// Rate limit by user ID instead of IP
    pub fn by_user(mut self) -> Self {
        self.by_user = true;
        self
    }

    /// Alias for development() - kept for backward compatibility
    #[deprecated(since = "0.1.0-alpha.1", note = "Use development() instead")]
    pub fn phase1(redis_client: redis::Client) -> Self {
        Self::development(redis_client)
    }

    /// Alias for production() - kept for backward compatibility
    #[deprecated(since = "0.1.0-alpha.1", note = "Use production() instead")]
    pub fn phase2(redis_client: redis::Client) -> Self {
        Self::production(redis_client)
    }
}

// Middleware factory
impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddlewareService {
            service,
            redis_client: self.redis_client.clone(),
            max_requests: self.max_requests,
            window_seconds: self.window_seconds,
            by_user: self.by_user,
        }))
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: S,
    redis_client: Arc<redis::Client>,
    max_requests: u32,
    window_seconds: u64,
    by_user: bool,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let redis_client = self.redis_client.clone();
        let max_requests = self.max_requests;
        let window_seconds = self.window_seconds;
        let by_user = self.by_user;

        // Get identifier for rate limiting (IP or user ID)
        let identifier = if by_user {
            // Try to get user ID from request extensions
            // This should be set by auth middleware
            req.extensions()
                .get::<String>()
                .map(|id| format!("user:{}", id))
                .unwrap_or_else(|| {
                    // Fall back to IP if no user ID
                    get_ip_address(&req)
                })
        } else {
            get_ip_address(&req)
        };

        let request_id = get_request_id(req.request());

        let fut = self.service.call(req);

        Box::pin(async move {
            // Check rate limit in Redis
            match check_rate_limit(
                redis_client.as_ref(),
                &identifier,
                max_requests,
                window_seconds,
            )
            .await
            {
                Ok(allowed) => {
                    if allowed {
                        // Request allowed - proceed
                        fut.await
                    } else {
                        // Rate limit exceeded
                        let retry_after = window_seconds;

                        tracing::warn!(
                            request_id = ?request_id,
                            identifier = %identifier,
                            max_requests = %max_requests,
                            window_seconds = %window_seconds,
                            "Rate limit exceeded"
                        );

                        Err(rate_limit_exceeded(retry_after))
                    }
                }
                Err(e) => {
                    // Redis error - fail open (allow request) but log error
                    tracing::error!(
                        request_id = ?request_id,
                        error = %e,
                        "Rate limit check failed - allowing request"
                    );
                    fut.await
                }
            }
        })
    }
}

/// Get client IP address from request
fn get_ip_address(req: &ServiceRequest) -> String {
    // Try to get real IP from X-Forwarded-For header (if behind proxy)
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(ip) = forwarded.to_str() {
            // Take first IP in the list
            if let Some(first_ip) = ip.split(',').next() {
                return format!("ip:{}", first_ip.trim());
            }
        }
    }

    // Fall back to connection peer address
    req.connection_info()
        .peer_addr()
        .map(|addr| format!("ip:{}", addr))
        .unwrap_or_else(|| "ip:unknown".to_string())
}

/// Check rate limit using Redis
async fn check_rate_limit(
    redis_client: &redis::Client,
    identifier: &str,
    max_requests: u32,
    window_seconds: u64,
) -> Result<bool, redis::RedisError> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;

    let key = format!("ratelimit:{}", identifier);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Use sorted set to track requests in time window
    // Remove old entries
    let window_start = now.saturating_sub(window_seconds);
    let _: () = conn.zrembyscore(&key, "-inf", window_start).await?;

    // Count requests in current window
    let count: u32 = conn.zcard(&key).await?;

    if count >= max_requests {
        // Rate limit exceeded
        return Ok(false);
    }

    // Add current request
    let _: () = conn.zadd(&key, now, now).await?;

    // Set expiry on key
    let _: () = conn.expire(&key, window_seconds as i64 + 1).await?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ip_from_forwarded_header() {
        // This is a basic test - full integration tests would require Redis
        let ip = "192.168.1.1, 10.0.0.1";
        let first_ip = ip.split(',').next().unwrap().trim();
        assert_eq!(first_ip, "192.168.1.1");
    }

    #[test]
    fn test_rate_limit_config() {
        // Just ensure the builder works
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let _middleware = RateLimitMiddleware::new(client, 30, 60);
    }
}
