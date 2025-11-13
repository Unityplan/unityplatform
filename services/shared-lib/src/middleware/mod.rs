// Middleware modules for Unity Platform services
// Following docs/architecture/services/shared-lib/MIDDLEWARE.md

pub mod cors;
pub mod error_handler;
pub mod logging;
pub mod rate_limit;
pub mod request_id;
pub mod security_headers;
pub mod validation;

// Re-exports
pub use error_handler::{error_response_handler, ErrorResponse};
pub use logging::LoggingMiddleware;
pub use rate_limit::RateLimitMiddleware;
pub use request_id::{RequestId, RequestIdMiddleware};
pub use security_headers::SecurityHeadersMiddleware;
pub use validation::{ValidatedJson, ValidatedPath, ValidatedQuery};

/// Phase 1 (Development) middleware stack
/// - Verbose logging with DEBUG level
/// - Detailed error messages with stack traces
/// - Request ID tracking
pub fn phase1_middleware() -> Vec<actix_web::middleware::Logger> {
    vec![actix_web::middleware::Logger::new(
        r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
    )]
}

/// Phase 2 (Production) middleware stack
/// - INFO/WARN/ERROR logging only
/// - Generic error messages (security)
/// - Request ID tracking
/// - Sensitive data redaction
pub fn phase2_middleware() -> Vec<actix_web::middleware::Logger> {
    vec![actix_web::middleware::Logger::new(
        r#"%a "%r" %s %b %T"#, // No User-Agent or Referer in production
    )]
}
