use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Query parameters for favicon fetching
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FaviconQuery {
    /// Website URL (e.g., https://github.com)
    #[validate(url, length(min = 1, max = 2048))]
    pub url: String,

    /// Icon size in pixels (16, 32, 64, or 128)
    #[validate(custom(function = "validate_icon_size"))]
    #[serde(default = "default_icon_size")]
    pub size: u32,
}

fn default_icon_size() -> u32 {
    32
}

fn validate_icon_size(size: u32) -> Result<(), validator::ValidationError> {
    if matches!(size, 16 | 32 | 64 | 128) {
        Ok(())
    } else {
        Err(validator::ValidationError::new(
            "Invalid icon size. Must be 16, 32, 64, or 128.",
        ))
    }
}

/// Health check response
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub uptime: u64,
    pub redis: RedisHealth,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RedisHealth {
    pub connected: bool,
    pub latency_ms: Option<f64>,
}

/// Readiness check response
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReadyResponse {
    pub ready: bool,
    pub service: String,
}
