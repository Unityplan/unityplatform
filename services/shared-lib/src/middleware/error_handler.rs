use actix_web::{http::StatusCode, HttpRequest, HttpResponse};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use std::fmt;

use super::request_id::get_request_id;
use crate::error::AppError;

/// Standard error response format
/// Following docs/architecture/services/ERROR-HANDLING.md
#[derive(Serialize, Debug, Clone)]
pub struct ErrorResponse {
    /// Error code (e.g., "not_found", "validation_failed")
    pub error: String,

    /// Human-readable error message
    pub message: String,

    /// Request ID for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Unix timestamp
    pub timestamp: i64,

    /// Additional error details (Phase 1 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

impl ErrorResponse {
    /// Create new error response
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
            request_id: None,
            timestamp: Utc::now().timestamp(),
            details: None,
        }
    }

    /// Add request ID
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// Add error details (Phase 1 only)
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Global error response handler
///
/// Converts errors to standard JSON format based on deployment phase:
///
/// # Phase 1 (Development)
/// - Detailed error messages
/// - Stack traces in details
/// - Full error context
///
/// # Phase 2 (Production)
/// - Generic error messages
/// - No stack traces
/// - Sensitive data redacted
///
/// # Example
///
/// ```rust
/// use actix_web::{App, web};
/// use shared_lib::middleware::error_handler::error_response_handler;
///
/// let app = App::new()
///     .app_data(web::JsonConfig::default()
///         .error_handler(|err, req| {
///             error_response_handler(actix_web::Error::from(err), req)
///         }));
/// ```
pub fn error_response_handler(err: actix_web::Error, req: &HttpRequest) -> actix_web::Error {
    let request_id = get_request_id(req);

    // Check if we're in development mode
    let is_dev =
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()) == "development";

    // Extract the underlying error
    let error_response = if let Some(app_error) = err.as_error::<AppError>() {
        // Our custom AppError
        app_error_to_response(app_error, request_id.as_deref(), is_dev)
    } else {
        // Generic actix-web error
        generic_error_to_response(&err, request_id.as_deref(), is_dev)
    };

    // Log the error
    tracing::error!(
        request_id = ?request_id,
        error = ?err,
        "Request error"
    );

    // Convert to HTTP response
    let status = determine_status_code(&err);
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::build(status).json(error_response),
    )
    .into()
}

/// Convert AppError to ErrorResponse
fn app_error_to_response(err: &AppError, request_id: Option<&str>, is_dev: bool) -> ErrorResponse {
    let (error_code, message) = match err {
        AppError::NotFound(msg) => ("not_found", msg.clone()),
        AppError::Unauthorized(msg) => ("unauthorized", msg.clone()),
        AppError::Forbidden(msg) => ("forbidden", msg.clone()),
        AppError::Validation(msg) => ("validation_failed", msg.clone()),
        AppError::Conflict(msg) => ("conflict", msg.clone()),
        AppError::Database(e) => {
            if is_dev {
                ("database_error", format!("Database error: {}", e))
            } else {
                (
                    "internal_server_error",
                    "An unexpected error occurred".to_string(),
                )
            }
        }
        AppError::Internal(msg) => {
            if is_dev {
                ("internal_server_error", msg.clone())
            } else {
                (
                    "internal_server_error",
                    "An unexpected error occurred".to_string(),
                )
            }
        }
        _ => {
            if is_dev {
                ("internal_server_error", err.to_string())
            } else {
                (
                    "internal_server_error",
                    "An unexpected error occurred".to_string(),
                )
            }
        }
    };

    let mut response = ErrorResponse::new(error_code, message);

    if let Some(rid) = request_id {
        response = response.with_request_id(rid.to_string());
    }

    // Add debug details in Phase 1
    if is_dev {
        response = response.with_details(json!({
            "debug": format!("{:?}", err)
        }));
    }

    response
}

/// Convert generic error to ErrorResponse
fn generic_error_to_response(
    err: &actix_web::Error,
    request_id: Option<&str>,
    is_dev: bool,
) -> ErrorResponse {
    let message = if is_dev {
        err.to_string()
    } else {
        "An unexpected error occurred".to_string()
    };

    let mut response = ErrorResponse::new("internal_server_error", message);

    if let Some(rid) = request_id {
        response = response.with_request_id(rid.to_string());
    }

    if is_dev {
        response = response.with_details(json!({
            "debug": format!("{:?}", err)
        }));
    }

    response
}

/// Determine HTTP status code from error
fn determine_status_code(err: &actix_web::Error) -> StatusCode {
    if let Some(app_error) = err.as_error::<AppError>() {
        match app_error {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Database(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    } else {
        // Try to extract status from response error
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

/// Helper to create common error responses

/// 400 Bad Request
pub fn bad_request(message: impl Into<String>) -> actix_web::Error {
    actix_web::error::ErrorBadRequest(ErrorResponse::new("bad_request", message.into()))
}

/// 401 Unauthorized
pub fn unauthorized(message: impl Into<String>) -> actix_web::Error {
    actix_web::error::ErrorUnauthorized(ErrorResponse::new("unauthorized", message.into()))
}

/// 403 Forbidden
pub fn forbidden(message: impl Into<String>) -> actix_web::Error {
    actix_web::error::ErrorForbidden(ErrorResponse::new("forbidden", message.into()))
}

/// 404 Not Found
pub fn not_found(message: impl Into<String>) -> actix_web::Error {
    actix_web::error::ErrorNotFound(ErrorResponse::new("not_found", message.into()))
}

/// 422 Validation Failed
pub fn validation_failed(
    message: impl Into<String>,
    details: Option<serde_json::Value>,
) -> actix_web::Error {
    let mut response = ErrorResponse::new("validation_failed", message.into());
    if let Some(d) = details {
        response = response.with_details(d);
    }
    actix_web::error::ErrorUnprocessableEntity(response)
}

/// 429 Rate Limit Exceeded
pub fn rate_limit_exceeded(retry_after: u64) -> actix_web::Error {
    actix_web::error::ErrorTooManyRequests(
        ErrorResponse::new(
            "rate_limit_exceeded",
            format!("Too many requests. Try again in {} seconds", retry_after),
        )
        .with_details(json!({
            "retry_after": retry_after
        })),
    )
}

/// 503 Service Unavailable
pub fn service_unavailable(message: impl Into<String>) -> actix_web::Error {
    actix_web::error::ErrorServiceUnavailable(ErrorResponse::new(
        "service_unavailable",
        message.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_creation() {
        let err = ErrorResponse::new("test_error", "Test message");
        assert_eq!(err.error, "test_error");
        assert_eq!(err.message, "Test message");
        assert!(err.request_id.is_none());
        assert!(err.details.is_none());
    }

    #[test]
    fn test_error_response_with_request_id() {
        let err = ErrorResponse::new("test_error", "Test message")
            .with_request_id("test-123".to_string());
        assert_eq!(err.request_id, Some("test-123".to_string()));
    }

    #[test]
    fn test_error_response_with_details() {
        let err = ErrorResponse::new("test_error", "Test message")
            .with_details(json!({"field": "username"}));
        assert!(err.details.is_some());
    }
}
