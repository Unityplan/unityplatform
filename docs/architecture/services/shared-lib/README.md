# Shared Library (shared-lib)

**Version:** 0.1.0-alpha.1  
**Status:** Production Ready  
**Language:** Rust  
**Purpose:** Shared code, utilities, and middleware for all Unity Platform services

---

## Overview

The `shared-lib` crate provides common functionality used across all microservices in the Unity Platform, including:

- **Authentication middleware** (JWT validation)
- **Permission system** (badge-based RBAC)
- **Database utilities** (connection pools, migrations)
- **Error handling** (standardized error types)
- **NATS client** (message bus integration)
- **Configuration** (environment-based config)
- **Logging** (structured logging with tracing)
- **Middleware** (security, rate limiting, CORS, validation)

---

## Documentation

- **[AUTHENTICATION.md](AUTHENTICATION.md)** - JWT validation strategy, middleware patterns, security best practices
- **[PERMISSION.md](PERMISSION.md)** - Badge-based permission system, RBAC middleware, role management ✨ NEW
- **[MIDDLEWARE.md](MIDDLEWARE.md)** - Request logging, security headers, rate limiting, CORS
- **[NATS-EVENTS.md](NATS-EVENTS.md)** - Message bus integration, event patterns
- **[DATABASE.md](DATABASE.md)** - Database connection management, migration strategy (TODO)
- **[ERRORS.md](ERRORS.md)** - Error handling patterns, custom error types (TODO)

---

## Key Components

### 1. Authentication Middleware

**JWT-based authentication for all services.**

```rust
use shared_lib::middleware::jwt_auth_middleware;

HttpServer::new(|| {
    App::new()
        .wrap(jwt_auth_middleware)  // Validates JWT for all routes
        .service(my_handler)
})
```

**See [AUTHENTICATION.md](AUTHENTICATION.md) for complete guide.**

---

### 2. Permission System ✨ NEW

**Badge-based role-based access control (RBAC).**

```rust
use shared_lib::{PermissionChecker, permission::{RequirePermission, RequireAnyPermission}};

// Initialize
let permission_checker = PermissionChecker::new(database.clone(), "dk".to_string());

// Protect routes
App::new()
    .app_data(web::Data::new(permission_checker))
    .service(
        web::scope("/admin")
            .wrap(RequirePermission::new("portal:manage"))
            .route("/settings", web::post().to(update_settings))
    )
```

**Features:**

- Badge-based permission grants
- Hierarchical permissions (`portal:*` grants all `portal:X`)
- 5-minute cache with TTL
- Service-owned role badge registration

**See [PERMISSION.md](PERMISSION.md) for complete guide.**

---

### 3. Database Utilities

**PostgreSQL connection pool management.**

```rust
use shared_lib::database::create_pool;

let pool = create_pool(&config.database_url).await?;
```

---

### 3. Error Handling

**Standardized error types across services.**

```rust
use shared_lib::error::{AppError, Result};

pub async fn my_handler() -> Result<HttpResponse> {
    Err(AppError::NotFound("Resource not found".into()))
}
```

---

### 4. NATS Client

**Message bus integration for event-driven architecture.**

```rust
use shared_lib::nats::NatsClient;

let nats = NatsClient::connect(&config.nats_url).await?;

// Publish event
nats.publish("user.deleted", UserDeletedEvent { user_id }).await?;

// Subscribe to events
nats.subscribe("user.deleted", |event: UserDeletedEvent| {
    handle_user_deletion(event).await
}).await;
```

---

### 5. Configuration

**Environment-based configuration loading.**

```rust
use shared_lib::config::Config;

let config = Config::from_env()?;
```

---

### 6. Logging

**Structured logging with tracing.**

```rust
use shared_lib::logging::init_logging;

init_logging("user-service", &config)?;

tracing::info!("Service started");
tracing::error!("Failed to connect to database");
```

---

## Usage in Services

All services depend on `shared-lib`:

```toml
# services/user-service/Cargo.toml
[dependencies]
shared-lib = { path = "../shared-lib" }
```

---

## Development

### Building

```bash
cd services/shared-lib
cargo build
```

### Testing

```bash
cargo test
```

### Documentation

```bash
cargo doc --open
```

---

## Version History

See [CHANGELOG.md](../../../../services/shared-lib/CHANGELOG.md) for version history.

---

## Related Services

- [auth-service](../auth-service/README.md) - Uses shared JWT middleware
- [user-service](../user-service/README.md) - Uses shared database utilities
- [settings-service](../settings-service/README.md) - Uses shared NATS client
- All other services depend on shared-lib

---

## Architecture Context

See [Platform Architecture](../../README.md) for overall system design.
