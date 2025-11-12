# Error Handling - Unity Platform

**Version:** 0.1.0-alpha.1  
**Date:** November 12, 2025  
**Status:** Implementation Planning

---

## Overview

This document defines error handling standards for Unity Platform services across different deployment phases.

---

## Standard Error Response Format

All services must return errors in this consistent JSON format:

```json
{
  "error": "error_code",
  "message": "Human-readable error message",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": 1699800000,
  "details": {}
}
```

### Implementation

```rust
use serde::Serialize;
use chrono::Utc;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
            request_id: None,
            timestamp: Utc::now().timestamp(),
            details: None,
        }
    }
    
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
    
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}
```

---

## HTTP Status Codes

### 4xx Client Errors

#### 400 Bad Request

Invalid input data that cannot be processed.

```json
{
  "error": "bad_request",
  "message": "Invalid input data",
  "request_id": "550e8400-...",
  "timestamp": 1699800000,
  "details": {
    "field": "email",
    "issue": "invalid format"
  }
}
```

```rust
return Err(actix_web::error::ErrorBadRequest(
    ErrorResponse::new("bad_request", "Invalid email format")
        .with_request_id(request_id)
        .with_details(serde_json::json!({
            "field": "email",
            "issue": "invalid format"
        }))
));
```

#### 401 Unauthorized

Missing or invalid authentication credentials.

```json
{
  "error": "unauthorized",
  "message": "Missing or invalid Authorization header",
  "request_id": "550e8400-...",
  "timestamp": 1699800000
}
```

```rust
return Err(actix_web::error::ErrorUnauthorized(
    ErrorResponse::new("unauthorized", "Invalid JWT token")
        .with_request_id(request_id)
));
```

#### 403 Forbidden

Authenticated but not authorized to perform action.

```json
{
  "error": "forbidden",
  "message": "Account inactive or deleted",
  "request_id": "550e8400-...",
  "timestamp": 1699800000
}
```

```rust
if user.deleted_at.is_some() {
    return Err(actix_web::error::ErrorForbidden(
        ErrorResponse::new("forbidden", "Account has been deleted")
            .with_request_id(request_id)
    ));
}
```

#### 404 Not Found

Resource does not exist.

```json
{
  "error": "not_found",
  "message": "User not found",
  "request_id": "550e8400-...",
  "timestamp": 1699800000
}
```

```rust
let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        actix_web::error::ErrorNotFound(
            ErrorResponse::new("not_found", format!("User {} not found", user_id))
                .with_request_id(request_id)
        )
    })?;
```

#### 422 Unprocessable Entity

Validation failed on the request data.

```json
{
  "error": "validation_failed",
  "message": "Validation failed",
  "request_id": "550e8400-...",
  "timestamp": 1699800000,
  "details": [
    {
      "field": "username",
      "message": "Username already taken"
    },
    {
      "field": "email",
      "message": "Email format is invalid"
    }
  ]
}
```

```rust
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateUserRequest {
    #[validate(length(min = 3, max = 30))]
    username: String,
    
    #[validate(email)]
    email: Option<String>,
}

async fn create_user(req: Json<CreateUserRequest>) -> Result<HttpResponse> {
    if let Err(validation_errors) = req.validate() {
        let details = validation_errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                serde_json::json!({
                    "field": field,
                    "message": errors.first().unwrap().message
                })
            })
            .collect::<Vec<_>>();
        
        return Err(actix_web::error::ErrorUnprocessableEntity(
            ErrorResponse::new("validation_failed", "Validation failed")
                .with_request_id(request_id)
                .with_details(serde_json::json!(details))
        ));
    }
    
    // ... create user ...
}
```

#### 429 Too Many Requests

Rate limit exceeded.

```json
{
  "error": "rate_limit_exceeded",
  "message": "Too many requests. Try again in 60 seconds",
  "request_id": "550e8400-...",
  "timestamp": 1699800000,
  "retry_after": 60
}
```

```rust
if count > rate_limit {
    return Err(actix_web::error::ErrorTooManyRequests(
        ErrorResponse::new(
            "rate_limit_exceeded",
            format!("Too many requests. Try again in {} seconds", retry_after)
        )
        .with_request_id(request_id)
        .with_details(serde_json::json!({
            "retry_after": retry_after
        }))
    ));
}
```

### 5xx Server Errors

#### 500 Internal Server Error

Unexpected server error occurred.

**Phase 1 (Development):**

```json
{
  "error": "internal_server_error",
  "message": "Database connection failed: connection timeout",
  "request_id": "550e8400-...",
  "timestamp": 1699800000,
  "details": {
    "debug": "Error: connection timeout at ..."
  }
}
```

**Phase 2 (Production):**

```json
{
  "error": "internal_server_error",
  "message": "An unexpected error occurred",
  "request_id": "550e8400-...",
  "timestamp": 1699800000
}
```

```rust
pub fn error_handler(
    err: actix_web::Error,
    req: &HttpRequest,
) -> actix_web::Error {
    let request_id = req.extensions()
        .get::<RequestId>()
        .map(|r| r.0.clone());
    
    let is_dev = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string()) == "development";
    
    let error_response = if is_dev {
        // Phase 1: Show detailed errors for debugging
        ErrorResponse {
            error: "internal_server_error".to_string(),
            message: err.to_string(),
            request_id,
            timestamp: Utc::now().timestamp(),
            details: Some(serde_json::json!({
                "debug": format!("{:?}", err)
            })),
        }
    } else {
        // Phase 2: Hide internals for security
        ErrorResponse {
            error: "internal_server_error".to_string(),
            message: "An unexpected error occurred".to_string(),
            request_id,
            timestamp: Utc::now().timestamp(),
            details: None,
        }
    };
    
    // Log the full error internally
    tracing::error!(
        request_id = ?request_id,
        error = ?err,
        "Internal server error"
    );
    
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::InternalServerError().json(error_response)
    ).into()
}
```

#### 503 Service Unavailable

Service temporarily unavailable (dependency down, circuit breaker open).

```json
{
  "error": "service_unavailable",
  "message": "Service temporarily unavailable",
  "request_id": "550e8400-...",
  "timestamp": 1699800000,
  "retry_after": 30
}
```

```rust
// Database connection failed
if pool.acquire().await.is_err() {
    return Err(actix_web::error::ErrorServiceUnavailable(
        ErrorResponse::new(
            "service_unavailable",
            "Database temporarily unavailable"
        )
        .with_request_id(request_id)
        .with_details(serde_json::json!({
            "retry_after": 30
        }))
    ));
}
```

---

## Error Logging

### Phase 1 (Development)

Log everything with detailed information:

```rust
use tracing::{error, warn, info, debug};

// Error-level: Unexpected errors
error!(
    request_id = %request_id,
    user_id = %user_id,
    error = %e,
    backtrace = ?e.backtrace(),
    "Failed to create user"
);

// Warn-level: Expected errors (validation, not found, etc.)
warn!(
    request_id = %request_id,
    username = %username,
    "Username already taken"
);

// Info-level: Normal operations
info!(
    request_id = %request_id,
    user_id = %user_id,
    "User created successfully"
);

// Debug-level: Detailed tracing
debug!(
    request_id = %request_id,
    query = %sql,
    params = ?params,
    "Executing database query"
);
```

### Phase 2 (Production)

Log errors with sensitive data redacted:

```rust
// ✅ Good: Redact sensitive data
error!(
    request_id = %request_id,
    user_id = %user_id,
    email = "[REDACTED]",  // Don't log emails
    error = %e,
    "Failed to create user"
);

// ❌ Bad: Logging sensitive data
error!(
    request_id = %request_id,
    email = %email,  // Security issue!
    password = %password,  // Never log passwords!
    error = %e,
    "Failed to create user"
);
```

### Sensitive Data to Redact

- ✅ Emails
- ✅ Passwords (never log)
- ✅ JWT tokens
- ✅ Session IDs
- ✅ API keys
- ✅ IP addresses (GDPR)
- ✅ Personal information (names, addresses, etc.)

---

## Error Handling Patterns

### Database Errors

```rust
async fn get_user(user_id: Uuid, pool: &PgPool) -> Result<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                Error::NotFound(format!("User {} not found", user_id))
            }
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                Error::Conflict("Username already exists".to_string())
            }
            _ => Error::Internal(format!("Database error: {}", e)),
        })
}
```

### Cross-Pod Errors

```rust
async fn get_remote_user(user_id: Uuid, pod_url: &str) -> Result<User> {
    reqwest::get(&format!("{}/api/v1/users/{}", pod_url, user_id))
        .await
        .map_err(|e| match e.status() {
            Some(StatusCode::NOT_FOUND) => Error::NotFound("User not found".to_string()),
            Some(StatusCode::SERVICE_UNAVAILABLE) => Error::ServiceUnavailable,
            _ => Error::RemoteError(e.to_string()),
        })?
        .json::<User>()
        .await
        .map_err(|e| Error::ParseError(e.to_string()))
}
```

### Event Processing Errors

```rust
async fn process_event(event: UserDeletedEvent) -> Result<()> {
    // Try to process event
    match cleanup_user_data(event.user_id).await {
        Ok(_) => {
            tracing::info!(
                event_id = %event.event_id,
                user_id = %event.user_id,
                "Event processed successfully"
            );
            Ok(())
        }
        Err(e) => {
            tracing::error!(
                event_id = %event.event_id,
                user_id = %event.user_id,
                error = %e,
                "Event processing failed"
            );
            
            // Don't crash the service - log and continue
            // Event will be redelivered by NATS
            Err(e)
        }
    }
}
```

---

## Error Metrics

Track error rates for monitoring:

```rust
use prometheus::IntCounterVec;

lazy_static! {
    static ref ERROR_COUNTER: IntCounterVec = IntCounterVec::new(
        Opts::new("errors_total", "Total errors by type"),
        &["error_type", "status_code"]
    ).unwrap();
}

pub fn record_error(error_type: &str, status_code: u16) {
    ERROR_COUNTER
        .with_label_values(&[error_type, &status_code.to_string()])
        .inc();
}
```

**Grafana Alerts:**

- Error rate > 5% → Warning
- Error rate > 10% → Critical
- 5xx errors > 1% → Critical

---

## User-Friendly Error Messages

### Localization

Support multiple languages via `Accept-Language` header:

```rust
#[derive(Serialize)]
struct LocalizedError {
    error: String,
    message: String,
    localized_message: String,
}

fn get_localized_message(error_code: &str, language: &str) -> String {
    match (error_code, language) {
        ("user_not_found", "da") => "Bruger ikke fundet".to_string(),
        ("user_not_found", "no") => "Bruker ikke funnet".to_string(),
        ("user_not_found", "sv") => "Användare hittades inte".to_string(),
        ("user_not_found", _) => "User not found".to_string(),
        _ => error_code.to_string(),
    }
}
```

### Generic vs Specific Messages

**Phase 1 (Development):** Specific error messages for debugging

```json
{
  "error": "database_error",
  "message": "Failed to insert user: duplicate key violation on username_idx"
}
```

**Phase 2 (Production):** Generic error messages for security

```json
{
  "error": "conflict",
  "message": "Username already taken"
}
```

---

## Error Categories

### Custom Error Enum

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Service unavailable")]
    ServiceUnavailable,
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Remote service error: {0}")]
    RemoteError(String),
}

// Convert to HTTP responses
impl actix_web::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::ValidationFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            Self::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(ErrorResponse::new(
                self.error_code(),
                self.to_string(),
            ))
    }
}
```

---

## Best Practices

### 1. Always Include Request ID

```rust
// ✅ Good
ErrorResponse::new("not_found", "User not found")
    .with_request_id(request_id)

// ❌ Bad (missing request ID)
ErrorResponse::new("not_found", "User not found")
```

### 2. Log Before Returning Error

```rust
// ✅ Good
tracing::error!(request_id = %request_id, error = %e, "Failed to process request");
return Err(error_response);

// ❌ Bad (no logging)
return Err(error_response);
```

### 3. Don't Expose Internal Details in Production

```rust
// ✅ Good (Phase 2)
"Database temporarily unavailable"

// ❌ Bad (leaking internals)
"Connection pool exhausted: max_connections=20, current=20"
```

### 4. Use Appropriate HTTP Status Codes

```rust
// ✅ Good
404 for resource not found
422 for validation errors
500 for unexpected errors

// ❌ Bad
200 with {"error": "..."}  // Don't do this!
```

---

## Summary

### Phase 1 (Development)

- **Detailed errors** with stack traces
- **DEBUG logging** enabled
- **Specific messages** for debugging
- **No sensitive data redaction** (it's localhost)

### Phase 2 (Production)

- **Generic errors** (hide internals)
- **INFO/WARN/ERROR logging** only
- **Generic messages** for security
- **Sensitive data redaction** (emails, tokens, IPs)
- **Error tracking** (Sentry)
- **Metrics and alerts** (Prometheus + Grafana)

---

**Related Documentation:**

- [Middleware Guide](./shared-lib/MIDDLEWARE.md)
- [Inter-Service Communication](./INTER-SERVICE-COMMUNICATION.md)
- [Caching Strategy](./CACHING-STRATEGY.md)
- [Observability](./OBSERVABILITY.md)

---

**Last Updated:** November 12, 2025  
**Status:** Ready for implementation
