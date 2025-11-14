pub mod circuit_breaker;
pub mod config;
pub mod database;
pub mod error;
pub mod jwt;
pub mod metrics;
pub mod middleware;
pub mod nats;
pub mod permission;
pub mod shutdown;

// Re-export commonly used types
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerError, CircuitState};
pub use config::AppConfig;
pub use database::Database;
pub use error::{AppError, Result};
pub use jwt::{AuthUser, Claims};
pub use metrics::MetricsCollector;
pub use nats::NatsClient;
pub use permission::{PermissionChecker, RequireAnyPermission, RequirePermission};
pub use shutdown::{shutdown_grace_period, shutdown_signal};

// Re-export middleware
pub use middleware::{
    cors, error_response_handler, ErrorResponse, LoggingMiddleware, RateLimitMiddleware, RequestId,
    RequestIdMiddleware, SecurityHeadersMiddleware, ValidatedJson, ValidatedPath, ValidatedQuery,
};

/// Version information embedded at build time
pub mod version {
    /// Service version from Cargo.toml
    pub const VERSION: &str = env!("SERVICE_VERSION");

    /// Service name from Cargo.toml
    pub const NAME: &str = env!("SERVICE_NAME");

    /// Build timestamp (Unix epoch seconds)
    pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

    /// Git commit hash (short)
    pub const GIT_HASH: &str = match option_env!("GIT_HASH") {
        Some(hash) => hash,
        None => "unknown",
    };

    /// Full version string with git hash
    pub fn full_version() -> String {
        format!("{} ({})", VERSION, GIT_HASH)
    }

    /// Version info for health checks and logging
    pub fn info() -> serde_json::Value {
        serde_json::json!({
            "name": NAME,
            "version": VERSION,
            "git_hash": GIT_HASH,
            "build_timestamp": BUILD_TIMESTAMP,
        })
    }
}
