pub mod config;
pub mod database;
pub mod error;
pub mod nats;

// Re-export commonly used types
pub use config::AppConfig;
pub use database::Database;
pub use error::{AppError, Result};
pub use nats::NatsClient;

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
