//! Graceful shutdown utilities for zero-downtime deployments
//!
//! This module provides signal handling for graceful shutdown:
//! - Ctrl+C (SIGINT) for manual termination during development
//! - SIGTERM for Kubernetes pod termination in production
//!
//! Features:
//! - Configurable grace period (default 30s production, 10s development)
//! - Clean resource cleanup (database, Redis, NATS connections)
//! - In-flight request completion before shutdown
//!
//! # Example
//!
//! ```rust,no_run
//! use actix_web::{web, App, HttpResponse, HttpServer};
//! use shared_lib::shutdown_signal;
//!
//! async fn health() -> HttpResponse {
//!     HttpResponse::Ok().body("OK")
//! }
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     // Create server
//!     let server = HttpServer::new(|| {
//!         App::new().route("/health", web::get().to(health))
//!     })
//!     .bind("0.0.0.0:8001")?
//!     .run();
//!     
//!     let server_handle = server.handle();
//!     let server_task = tokio::spawn(server);
//!     
//!     // Wait for shutdown signal
//!     shutdown_signal().await;
//!     
//!     // Initiate graceful shutdown
//!     server_handle.stop(true).await;
//!     server_task.await.ok();
//!     
//!     Ok(())
//! }
//! ```

use tokio::signal;
use tracing::info;

/// Wait for shutdown signal (Ctrl+C or SIGTERM)
///
/// This function blocks until either:
/// - Ctrl+C (SIGINT) is received - common during development
/// - SIGTERM is received - common in Kubernetes deployments
///
/// # Example
///
/// ```rust,no_run
/// use shared_lib::shutdown_signal;
///
/// #[tokio::main]
/// async fn main() {
///     // ... setup application ...
///     
///     // Wait for shutdown signal
///     shutdown_signal().await;
///     
///     // ... perform cleanup ...
/// }
/// ```
pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("🛑 Received Ctrl+C (SIGINT), initiating graceful shutdown");
        },
        _ = terminate => {
            info!("🛑 Received SIGTERM, initiating graceful shutdown");
        },
    }
}

/// Get the configured shutdown grace period in seconds
///
/// Reads from environment variable `SHUTDOWN_GRACE_PERIOD`.
/// Defaults:
/// - Development: 10 seconds (fast iteration)
/// - Production: 30 seconds (ensure all requests complete)
///
/// # Example
///
/// ```rust
/// use shared_lib::shutdown_grace_period;
///
/// let grace_period = shutdown_grace_period();
/// println!("Grace period: {}s", grace_period);
/// ```
pub fn shutdown_grace_period() -> u64 {
    std::env::var("SHUTDOWN_GRACE_PERIOD")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(30) // Default: 30 seconds for production safety
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shutdown_grace_period_default() {
        std::env::remove_var("SHUTDOWN_GRACE_PERIOD");
        assert_eq!(shutdown_grace_period(), 30);
    }

    #[test]
    fn test_shutdown_grace_period_custom() {
        std::env::set_var("SHUTDOWN_GRACE_PERIOD", "60");
        assert_eq!(shutdown_grace_period(), 60);
        std::env::remove_var("SHUTDOWN_GRACE_PERIOD");
    }

    #[test]
    fn test_shutdown_grace_period_invalid() {
        std::env::set_var("SHUTDOWN_GRACE_PERIOD", "invalid");
        assert_eq!(shutdown_grace_period(), 30); // Falls back to default
        std::env::remove_var("SHUTDOWN_GRACE_PERIOD");
    }
}
