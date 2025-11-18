# Service Implementation Guide

**Version:** 0.1.0  
**Last Updated:** 2025-11-18  
**Target:** Unity Platform microservices following shared-lib middleware patterns

This guide provides step-by-step instructions for implementing business logic in a scaffolded Unity Platform service. Use after running `scripts/dev/scaffold-service.sh`.

---

## Table of Contents

1. [Overview](#overview)
2. [Middleware Configuration](#middleware-configuration)
3. [Database Patterns](#database-patterns)
4. [Request/Response Models](#requestresponse-models)
5. [Handler Implementation](#handler-implementation)
6. [Business Logic Services](#business-logic-services)
7. [NATS Event Publishing](#nats-event-publishing)
8. [Error Handling](#error-handling)
9. [Authentication & Authorization](#authentication--authorization)
10. [Testing](#testing)
11. [Examples](#examples)

---

## Overview

### Service Structure

```
src/
├── main.rs              # Server setup, middleware, routes (✅ Done by scaffold)
├── lib.rs               # Module exports (✅ Done by scaffold)
├── handlers/            # HTTP request handlers (⏳ Implement)
│   ├── mod.rs           # Handler modules and route configuration
│   └── {domain}.rs      # Domain-specific handlers
├── models/              # Request/Response types (⏳ Implement)
│   ├── mod.rs           # Model exports
│   ├── request.rs       # Request DTOs with validation
│   └── response.rs      # Response DTOs
└── services/            # Business logic (⏳ Implement)
    ├── mod.rs           # Service exports
    └── {domain}_service.rs  # Domain service with database access
```

### Implementation Flow

1. **Models First:** Define request/response types with validation
2. **Handlers Second:** Implement HTTP endpoints using models
3. **Services Third:** Implement business logic with database access
4. **Wire Together:** Connect handlers to services, update main.rs OpenAPI docs
5. **Test:** Write unit and integration tests

---

## Middleware Configuration

### Available Middleware (Priority Order)

The scaffold includes **all middleware by default**. Remove what you don't need:

```rust
// Priority 1: Request tracking and logging
.wrap(LoggingMiddleware::development_with_metrics(metrics_collector.clone()))
.wrap(RequestIdMiddleware)

// Priority 2: Security and rate limiting
.wrap(SecurityHeadersMiddleware::development())
.wrap(cors::development())
.wrap(RateLimitMiddleware::development(redis_client.clone()))
```

### When to Remove Middleware

| Middleware | Remove If | Keep If |
|------------|-----------|---------|
| `LoggingMiddleware` | **Never** - always needed | Always |
| `RequestIdMiddleware` | **Never** - always needed | Always |
| `SecurityHeadersMiddleware` | Internal-only service | Public-facing API |
| `cors::development()` | Backend-to-backend only | Frontend calls this service |
| `RateLimitMiddleware` | Internal-only + trusted | Public endpoints exist |

### Production vs Development

```rust
// Development (lenient, verbose logging)
.wrap(LoggingMiddleware::development_with_metrics(metrics_collector.clone()))
.wrap(SecurityHeadersMiddleware::development())
.wrap(cors::development())
.wrap(RateLimitMiddleware::development(redis_client.clone()))

// Production (strict, JSON logging)
.wrap(LoggingMiddleware::production_with_metrics(metrics_collector.clone()))
.wrap(SecurityHeadersMiddleware::production())
.wrap(cors::production(vec!["https://app.unityplatform.dk"]))
.wrap(RateLimitMiddleware::production(redis_client.clone()))
```

**Note:** The scaffold uses development mode. Switch to production in deployment configs.

---

## Database Patterns

### Territory-Aware Queries

Unity Platform uses **multi-tenant PostgreSQL schemas** (`territory_dk`, `territory_no`, etc.).

#### Pattern 1: Query Territory Schema

```rust
// ❌ WRONG: Hard-coded schema
let result = sqlx::query!(
    "SELECT * FROM invitation_invitations_tokens WHERE token = $1",
    token
)
.fetch_one(pool)
.await?;

// ✅ CORRECT: Use Database helper with territory code
let territory = "dk"; // Get from config or request context
let result = db
    .territory_query(
        territory,
        "SELECT * FROM invitation_invitations_tokens WHERE token = $1"
    )
    .bind(token)
    .fetch_one()
    .await?;

// ✅ ALTERNATIVE: Manual schema prefix
let query = format!(
    "SELECT * FROM territory_{}.invitation_invitations_tokens WHERE token = $1",
    territory
);
let result = sqlx::query(&query)
    .bind(token)
    .fetch_one(pool)
    .await?;
```

**Reference:** See `shared-lib/src/database.rs` for `Database::territory_query()` helper.

#### Pattern 2: Global Registry Checks

Some tables are in the `global` schema (username_registry, email_registry, etc.):

```rust
// Check if username exists globally (cross-territory)
let exists: bool = sqlx::query_scalar(
    "SELECT EXISTS(SELECT 1 FROM global.username_registry WHERE username = $1)"
)
.bind(&username)
.fetch_one(pool)
.await?;

if exists {
    return Err(AppError::Conflict("Username already taken".to_string()));
}
```

**Reference:** See `auth-service/src/handlers/auth.rs:85-120` for registration with global checks.

#### Pattern 3: Transactions (Multi-Table Operations)

```rust
use sqlx::Transaction;

// Start transaction
let mut tx = db.pool().begin().await?;

// Insert into territory schema
let query = format!(
    "INSERT INTO territory_{}.invitation_invitations_tokens (token, created_by, max_uses) 
     VALUES ($1, $2, $3) RETURNING id",
    territory
);
let invitation_id: Uuid = sqlx::query_scalar(&query)
    .bind(&token)
    .bind(&created_by)
    .bind(max_uses)
    .fetch_one(&mut *tx)
    .await?;

// Insert into global registry
sqlx::query(
    "INSERT INTO global.registry_invitation (token) VALUES ($1)"
)
.bind(&token)
.execute(&mut *tx)
.await?;

// Commit transaction
tx.commit().await?;
```

**Reference:** See `auth-service/src/handlers/auth.rs:185-210` for transaction pattern.

---

## Request/Response Models

### Models with Validation

Use `validator` crate for automatic request validation:

```rust
// src/models/request.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateInvitationRequest {
    /// Maximum number of uses (0 = unlimited)
    #[validate(range(min = 0, max = 1000))]
    pub max_uses: i32,
    
    /// Expiration in days (1-365)
    #[validate(range(min = 1, max = 365))]
    pub expires_in_days: Option<i32>,
    
    /// Optional metadata (JSON)
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ValidateInvitationRequest {
    /// Invitation token (16-255 chars)
    #[validate(length(min = 16, max = 255))]
    pub token: String,
}
```

### Response Models

```rust
// src/models/response.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateInvitationResponse {
    pub id: Uuid,
    pub token: String,
    pub created_by: Uuid,
    pub max_uses: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub invite_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidateInvitationResponse {
    pub valid: bool,
    pub invitation_id: Option<Uuid>,
    pub uses_remaining: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
}
```

### Export Models

```rust
// src/models/mod.rs
pub mod request;
pub mod response;

pub use request::*;
pub use response::*;
```

**Reference:** See `user-service/src/models/` for complete examples.

---

## Handler Implementation

### Public Endpoint (No Auth)

```rust
// src/handlers/invitation.rs
use crate::models::{ValidateInvitationRequest, ValidateInvitationResponse};
use actix_web::{post, web, HttpResponse};
use shared_lib::{Database, Result, ValidatedJson};

/// Validate invitation token (public endpoint)
#[utoipa::path(
    post,
    path = "/api/v1/invitation/validate",
    tag = "invitation",
    request_body = ValidateInvitationRequest,
    responses(
        (status = 200, description = "Validation result", body = ValidateInvitationResponse),
        (status = 400, description = "Invalid request")
    )
)]
#[post("/validate")]
pub async fn validate_invitation(
    db: web::Data<Database>,
    body: ValidatedJson<ValidateInvitationRequest>, // Auto-validates!
) -> Result<HttpResponse> {
    // Business logic here
    let token = &body.token;
    
    // TODO: Call service layer
    // let result = InvitationService::new(db.get_ref().clone())
    //     .validate(token)
    //     .await?;
    
    Ok(HttpResponse::Ok().json(ValidateInvitationResponse {
        valid: true,
        invitation_id: None,
        uses_remaining: None,
        expires_at: None,
    }))
}
```

### Protected Endpoint (Requires JWT)

```rust
use shared_lib::{AuthUser, Database, Result, ValidatedJson};
use actix_web::HttpRequest;

/// Create invitation (protected - manager only)
#[utoipa::path(
    post,
    path = "/api/v1/invitation",
    tag = "invitation",
    request_body = CreateInvitationRequest,
    responses(
        (status = 201, description = "Invitation created", body = CreateInvitationResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - manager role required")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("")]
pub async fn create_invitation(
    req: HttpRequest,
    db: web::Data<Database>,
    body: ValidatedJson<CreateInvitationRequest>,
) -> Result<HttpResponse> {
    // Extract authenticated user from JWT
    let auth_user = AuthUser::from_request(&req)?;
    
    // Check permission (example - actual implementation in service layer)
    // if !auth_user.has_permission("invitation.create") {
    //     return Err(AppError::Forbidden("Manager role required".to_string()));
    // }
    
    // Business logic
    let created_by = auth_user.user_id;
    
    // TODO: Call service layer
    
    Ok(HttpResponse::Created().json(CreateInvitationResponse {
        id: uuid::Uuid::new_v4(),
        token: "placeholder".to_string(),
        created_by,
        max_uses: body.max_uses,
        expires_at: None,
        invite_url: "https://app.unityplatform.dk/register?token=...".to_string(),
    }))
}
```

### Route Configuration

```rust
// src/handlers/mod.rs
pub mod invitation;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(invitation::validate_invitation)
       .service(invitation::create_invitation);
}
```

### Wire to main.rs

```rust
// In main.rs, update the service scope:
.service(
    web::scope("/invitation")
        .configure(invitation_service::handlers::configure)
)
```

**Reference:** See `user-service/src/handlers/profile.rs` for complete handler examples.

---

## Business Logic Services

### Service Structure

```rust
// src/services/invitation_service.rs
use shared_lib::{AppError, Database, Result};
use uuid::Uuid;
use chrono::{Duration, Utc};

pub struct InvitationService {
    db: Database,
}

impl InvitationService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Generate unique invitation token
    pub async fn generate_token(&self) -> Result<String> {
        use rand::Rng;
        
        loop {
            // Generate token: XXXX-XXXX-XXXX-XXXX
            let token = format!(
                "{:04X}-{:04X}-{:04X}-{:04X}",
                rand::thread_rng().gen::<u16>(),
                rand::thread_rng().gen::<u16>(),
                rand::thread_rng().gen::<u16>(),
                rand::thread_rng().gen::<u16>()
            );
            
            // Check global uniqueness
            let exists: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM global.registry_invitation WHERE token = $1)"
            )
            .bind(&token)
            .fetch_one(self.db.pool())
            .await?;
            
            if !exists {
                return Ok(token);
            }
            // If exists, loop and try again (very rare)
        }
    }

    /// Create new invitation
    pub async fn create(
        &self,
        created_by: Uuid,
        max_uses: i32,
        expires_in_days: Option<i32>,
        territory: &str,
    ) -> Result<(Uuid, String)> {
        let token = self.generate_token().await?;
        let expires_at = expires_in_days.map(|days| {
            Utc::now() + Duration::days(days as i64)
        });
        
        // Start transaction
        let mut tx = self.db.pool().begin().await?;
        
        // Insert into territory schema
        let query = format!(
            "INSERT INTO territory_{}.invitation_invitations_tokens 
             (token, created_by, max_uses, expires_at, is_active)
             VALUES ($1, $2, $3, $4, true)
             RETURNING id",
            territory
        );
        let id: Uuid = sqlx::query_scalar(&query)
            .bind(&token)
            .bind(created_by)
            .bind(max_uses)
            .bind(expires_at)
            .fetch_one(&mut *tx)
            .await?;
        
        // Insert into global registry
        sqlx::query("INSERT INTO global.registry_invitation (token) VALUES ($1)")
            .bind(&token)
            .execute(&mut *tx)
            .await?;
        
        // Commit
        tx.commit().await?;
        
        Ok((id, token))
    }

    /// Validate invitation token
    pub async fn validate(&self, token: &str, territory: &str) -> Result<bool> {
        let query = format!(
            "SELECT is_active, expires_at, uses_count, max_uses
             FROM territory_{}.invitation_invitations_tokens
             WHERE token = $1",
            territory
        );
        
        let row: Option<(bool, Option<chrono::DateTime<Utc>>, i32, i32)> = 
            sqlx::query_as(&query)
            .bind(token)
            .fetch_optional(self.db.pool())
            .await?;
        
        match row {
            None => Ok(false), // Token not found
            Some((is_active, expires_at, uses_count, max_uses)) => {
                // Check if active
                if !is_active {
                    return Ok(false);
                }
                
                // Check expiration
                if let Some(exp) = expires_at {
                    if Utc::now() > exp {
                        return Ok(false);
                    }
                }
                
                // Check usage limits (0 = unlimited)
                if max_uses > 0 && uses_count >= max_uses {
                    return Ok(false);
                }
                
                Ok(true)
            }
        }
    }
}
```

### Export Services

```rust
// src/services/mod.rs
pub mod invitation_service;

pub use invitation_service::InvitationService;
```

**Reference:** See `user-service/src/services/profile.rs` for complete service examples.

---

## NATS Event Publishing

### Event Envelope (Standard)

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct PlatformEvent<T> {
    event_id: Uuid,
    event_type: String,
    timestamp: DateTime<Utc>,
    territory: Option<String>,
    payload: T,
}

impl<T> PlatformEvent<T> {
    fn new(event_type: String, territory: Option<String>, payload: T) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type,
            timestamp: Utc::now(),
            territory,
            payload,
        }
    }
}
```

### Publishing Events

```rust
// Event payload
#[derive(Debug, Serialize, Deserialize)]
struct InvitationCreatedPayload {
    invitation_id: Uuid,
    created_by: Uuid,
    max_uses: i32,
}

// Publish after successful database commit
let event = PlatformEvent::new(
    "invitation.created".to_string(),
    Some(territory.to_string()),
    InvitationCreatedPayload {
        invitation_id: id,
        created_by,
        max_uses,
    },
);

nats_client
    .publish("invitation.created", serde_json::to_vec(&event)?)
    .await?;

tracing::info!(
    invitation_id = %id,
    event_id = %event.event_id,
    "Published invitation.created event"
);
```

**Reference:**

- See `auth-service/src/handlers/auth.rs:145-175` for user.registered event
- See `docs/architecture/NATS-EVENTS.md` for event specifications

---

## Error Handling

### AppError Variants

```rust
// From shared-lib/src/error.rs
pub enum AppError {
    Database(sqlx::Error),           // Database errors
    Config(config::ConfigError),     // Config errors
    Nats(String),                    // NATS errors
    Serialization(serde_json::Error),// JSON errors
    Auth(String),                    // Authentication errors
    Validation(String),              // Request validation errors
    Conflict(String),                // Resource conflicts (409)
    NotFound(String),                // Resource not found (404)
    Unauthorized(String),            // Not authenticated (401)
    Forbidden(String),               // Not authorized (403)
    Internal(String),                // Internal server errors (500)
}
```

### When to Use Each Error

| Error | Use When | HTTP Status | Example |
|-------|----------|-------------|---------|
| `Conflict` | Resource already exists | 409 | "Username already taken" |
| `NotFound` | Resource doesn't exist | 404 | "Invitation not found" |
| `Unauthorized` | Missing/invalid JWT | 401 | "Invalid authentication token" |
| `Forbidden` | Lacks permission | 403 | "Manager role required" |
| `Validation` | Bad input format | 400 | "Email format invalid" |
| `Internal` | Unexpected errors | 500 | "Failed to generate token" |

### Error Handling Pattern

```rust
use shared_lib::{AppError, Result};

// ✅ CORRECT: Return Result<T>
pub async fn create_invitation(...) -> Result<CreateInvitationResponse> {
    // Check conflicts
    if token_exists {
        return Err(AppError::Conflict("Token already exists".to_string()));
    }
    
    // Check authorization
    if !has_permission {
        return Err(AppError::Forbidden("Manager role required".to_string()));
    }
    
    // Check not found
    let user = find_user(id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    
    // Database errors are auto-converted via From trait
    let result = sqlx::query(...)
        .fetch_one(pool)
        .await?; // AppError::Database if fails
    
    Ok(result)
}
```

**Reference:** See `shared-lib/src/error.rs` for complete AppError definition.

---

## Authentication & Authorization

### Extract Authenticated User

```rust
use shared_lib::AuthUser;
use actix_web::HttpRequest;

async fn protected_endpoint(req: HttpRequest) -> Result<HttpResponse> {
    // Extract user from JWT (auto-validates token)
    let auth_user = AuthUser::from_request(&req)?;
    
    // Access user info
    let user_id = auth_user.user_id;
    let username = &auth_user.username;
    let territory = &auth_user.territory;
    
    // Use in business logic
    // ...
}
```

### Permission Checks

```rust
// TODO: Implement permission system (not yet in shared-lib)
// For now, manual checks:

if !auth_user.roles.contains(&"manager".to_string()) {
    return Err(AppError::Forbidden("Manager role required".to_string()));
}
```

**Reference:** See `shared-lib/src/jwt.rs` for AuthUser implementation.

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_format() {
        let token = generate_token_sync();
        assert_eq!(token.len(), 19); // XXXX-XXXX-XXXX-XXXX
        assert_eq!(token.matches('-').count(), 3);
    }

    #[tokio::test]
    async fn test_validation_logic() {
        // Test expired invitation
        let expires_at = Some(Utc::now() - Duration::days(1));
        assert!(!is_valid(true, expires_at, 0, 10));
        
        // Test max uses reached
        assert!(!is_valid(true, None, 10, 10));
        
        // Test valid invitation
        assert!(is_valid(true, None, 5, 10));
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
#[cfg(test)]
mod integration_tests {
    use actix_web::test;

    #[actix_web::test]
    async fn test_create_invitation_endpoint() {
        // Setup test app
        let app = test::init_service(App::new().configure(configure)).await;
        
        // Make request
        let req = test::TestRequest::post()
            .uri("/api/v1/invitation")
            .set_json(&CreateInvitationRequest {
                max_uses: 5,
                expires_in_days: Some(30),
                metadata: None,
            })
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);
    }
}
```

---

## Examples

### Complete Invitation Service Example

See the following files for reference:

#### 1. Models

```rust
// src/models/request.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateInvitationRequest {
    #[validate(range(min = 0, max = 1000))]
    pub max_uses: i32,
    
    #[validate(range(min = 1, max = 365))]
    pub expires_in_days: Option<i32>,
    
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ValidateInvitationRequest {
    #[validate(length(min = 16, max = 255))]
    pub token: String,
}
```

```rust
// src/models/response.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateInvitationResponse {
    pub id: Uuid,
    pub token: String,
    pub invite_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidateInvitationResponse {
    pub valid: bool,
    pub uses_remaining: Option<i32>,
}
```

#### 2. Handlers

```rust
// src/handlers/invitation.rs
use crate::models::*;
use crate::services::InvitationService;
use actix_web::{post, web, HttpRequest, HttpResponse};
use shared_lib::{AuthUser, Database, Result, ValidatedJson};

#[post("/validate")]
pub async fn validate_invitation(
    db: web::Data<Database>,
    body: ValidatedJson<ValidateInvitationRequest>,
) -> Result<HttpResponse> {
    let service = InvitationService::new(db.get_ref().clone());
    let valid = service.validate(&body.token, "dk").await?;
    
    Ok(HttpResponse::Ok().json(ValidateInvitationResponse {
        valid,
        uses_remaining: None,
    }))
}

#[post("")]
pub async fn create_invitation(
    req: HttpRequest,
    db: web::Data<Database>,
    body: ValidatedJson<CreateInvitationRequest>,
) -> Result<HttpResponse> {
    let auth_user = AuthUser::from_request(&req)?;
    let service = InvitationService::new(db.get_ref().clone());
    
    let (id, token) = service.create(
        auth_user.user_id,
        body.max_uses,
        body.expires_in_days,
        &auth_user.territory,
    ).await?;
    
    Ok(HttpResponse::Created().json(CreateInvitationResponse {
        id,
        token: token.clone(),
        invite_url: format!("https://app.unityplatform.dk/register?token={}", token),
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(validate_invitation)
       .service(create_invitation);
}
```

#### 3. Services

```rust
// src/services/invitation_service.rs
use shared_lib::{AppError, Database, Result};
use uuid::Uuid;
use chrono::{Duration, Utc};

pub struct InvitationService {
    db: Database,
}

impl InvitationService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn generate_token(&self) -> Result<String> {
        // See "Business Logic Services" section above
    }

    pub async fn create(
        &self,
        created_by: Uuid,
        max_uses: i32,
        expires_in_days: Option<i32>,
        territory: &str,
    ) -> Result<(Uuid, String)> {
        // See "Business Logic Services" section above
    }

    pub async fn validate(&self, token: &str, territory: &str) -> Result<bool> {
        // See "Business Logic Services" section above
    }
}
```

#### 4. Update main.rs

```rust
// Add to #[openapi(paths(...))]
user_service::handlers::invitation::validate_invitation,
user_service::handlers::invitation::create_invitation,

// Add to #[openapi(components(schemas(...)))]
CreateInvitationRequest,
CreateInvitationResponse,
ValidateInvitationRequest,
ValidateInvitationResponse,

// Add to service scope
.service(
    web::scope("/invitation")
        .configure(invitation_service::handlers::invitation::configure)
)
```

---

## Quick Reference

### Checklist for New Endpoint

- [ ] Create request model with `#[derive(Validate, ToSchema)]`
- [ ] Create response model with `#[derive(ToSchema)]`
- [ ] Export models in `src/models/mod.rs`
- [ ] Implement handler with `#[utoipa::path(...)]`
- [ ] Add handler to `configure()` in `src/handlers/mod.rs`
- [ ] Implement business logic in service layer
- [ ] Add path to `#[openapi(paths(...))]` in main.rs
- [ ] Add schemas to `#[openapi(components(schemas(...)))]` in main.rs
- [ ] Add service scope to routes in main.rs
- [ ] Test build: `cargo build -p {service-name}`
- [ ] Test endpoint: `curl http://localhost:{port}/api/v1/...`
- [ ] Check Swagger UI: `http://localhost:{port}/swagger-ui/`
- [ ] Write unit tests
- [ ] Write integration tests

### Common Imports

```rust
// Handlers
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use shared_lib::{AuthUser, Database, NatsClient, Result, ValidatedJson};
use utoipa;

// Models
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

// Services
use shared_lib::{AppError, Database, Result};
use sqlx::Transaction;
use uuid::Uuid;
```

---

## Additional Resources

- **Middleware Guide:** `docs/architecture/services/shared-lib/MIDDLEWARE.md`
- **Database Migrations:** `docs/architecture/MIGRATIONS-MASTER.md`
- **NATS Events:** `docs/architecture/NATS-EVENTS.md`
- **Auth Service Example:** `services/auth-service/src/`
- **User Service Example:** `services/user-service/src/`
- **Shared Library:** `services/shared-lib/src/`

---

**Last Updated:** 2025-11-18  
**Maintainer:** Unity Platform Team
