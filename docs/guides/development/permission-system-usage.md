# Permission System Usage Guide

## Overview

The Unity Platform uses a badge-based permission system where permissions are granted through badges. Services can register their role badges and use permission middleware to protect endpoints.

## Architecture Components

### 1. PermissionChecker (shared-lib)

Verifies user permissions by checking badge ownership and granted permissions.

### 2. Permission Middleware (shared-lib)

- `RequirePermission` - Requires a specific permission
- `RequireAnyPermission` - Requires at least one of multiple permissions

### 3. Badge Registration Endpoint (badge-service)

`POST /api/v1/badges/register` - Services register their role badges on startup

## Service Setup

### Step 1: Register Role Badges on Startup

```rust
// In your service's main.rs

async fn register_service_badges(config: &AppConfig) {
    use serde_json::json;
    
    let badge_service_url = std::env::var("BADGE_SERVICE_URL")
        .unwrap_or_else(|_| format!("http://{}:8007", config.server.host));
    
    let badge_payload = json!({
        "slug": "service-manager",
        "name": "Service Manager",
        "description": "Grants full management access to service",
        "icon": "⚙️",
        "category": "role",
        "criteriaType": "manual",
        "criteriaValue": null,
        "rarity": "epic",
        "isRenewable": false,
        "renewalDays": null,
        "grantsPermissions": [
            "service:manage",
            "service:settings:manage",
            "service:users:manage"
        ]
    });
    
    let client = reqwest::Client::new();
    match client
        .post(format!("{}/api/v1/badges/register", badge_service_url))
        .json(&badge_payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            tracing::info!("✅ Service badges registered");
        }
        Ok(response) => {
            tracing::warn!("⚠️  Failed to register badges: HTTP {}", response.status());
        }
        Err(e) => {
            tracing::warn!("⚠️  Could not reach badge-service: {}", e);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... config, database, nats setup ...
    
    // Register badges after NATS connection
    register_service_badges(&config).await;
    
    // ... rest of setup ...
}
```

### Step 2: Initialize PermissionChecker

```rust
use shared_lib::PermissionChecker;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... setup ...
    
    let database = Database::new(...).await?;
    
    // Initialize permission checker
    let permission_checker = PermissionChecker::new(
        database.clone(),
        "dk".to_string(), // territory code
    );
    
    // Create HTTP server
    let server = HttpServer::new(move || {
        App::new()
            // Add PermissionChecker to app data
            .app_data(web::Data::new(permission_checker.clone()))
            // ... middleware ...
            .configure(routes)
    })
    .bind(server_addr)?
    .run();
    
    // ... rest ...
}
```

### Step 3: Protect Endpoints with Middleware

```rust
use actix_web::{web, HttpResponse};
use shared_lib::permission::{RequirePermission, RequireAnyPermission};

// Require a single permission
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(RequirePermission::new("portal:manage"))
            .route("/settings", web::post().to(update_settings))
            .route("/users", web::get().to(list_users))
    );
}

// Require any of multiple permissions
pub fn configure_moderator_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/moderate")
            .wrap(RequireAnyPermission::new(vec![
                "content:moderate",
                "portal:manage",
            ]))
            .route("/reports", web::get().to(view_reports))
            .route("/actions", web::post().to(take_action))
    );
}

// Per-route protection
pub fn configure_mixed_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Public route - no middleware
            .route("/public", web::get().to(public_endpoint))
            // Protected route - requires permission
            .route(
                "/admin-only",
                web::post()
                    .to(admin_endpoint)
                    .wrap(RequirePermission::new("portal:manage"))
            )
    );
}
```

## Permission Naming Convention

Use hierarchical permission strings with `:` separator:

```
service:action
service:resource:action
```

### Examples

- `portal:manage` - Full portal management
- `portal:users:manage` - Manage portal users only
- `territory:manage` - Full territory management
- `territory:settings:manage` - Manage territory settings only
- `content:moderate` - Moderate content
- `content:translate` - Translate content

### Wildcard Support

The PermissionChecker supports hierarchical matching:

- Badge with `portal:*` grants `portal:manage`, `portal:users:manage`, etc.
- Badge with `portal:users:*` grants `portal:users:manage`, `portal:users:view`, etc.

## Badge Registration Examples

### Portal Service Badges

```json
{
  "slug": "portal-manager",
  "name": "Portal Manager",
  "description": "Grants full management access to portal administration",
  "icon": "🏛️",
  "category": "role",
  "criteriaType": "manual",
  "rarity": "epic",
  "grantsPermissions": [
    "portal:manage",
    "portal:users:manage",
    "portal:settings:manage",
    "portal:content:manage",
    "portal:services:manage"
  ]
}
```

### Territory Service Badge

```json
{
  "slug": "territory-manager",
  "name": "Territory Manager",
  "description": "Grants full management access to territory administration",
  "icon": "🌍",
  "category": "role",
  "criteriaType": "manual",
  "rarity": "epic",
  "grantsPermissions": [
    "territory:manage",
    "territory:settings:manage",
    "territory:users:manage",
    "territory:federation:manage"
  ]
}
```

## Testing Permissions

### 1. Award Badge to User

```bash
curl -X POST http://localhost:8007/api/v1/badges/award \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -d '{
    "userId": "user-uuid-here",
    "badgeSlug": "portal-manager",
    "reason": "Promoted to portal manager"
  }'
```

### 2. Make Authenticated Request

```bash
curl http://localhost:8000/api/v1/admin/settings \
  -H "Authorization: Bearer $USER_TOKEN"
```

### Expected Responses

- **200 OK** - User has required permission
- **403 Forbidden** - User lacks required permission

  ```json
  {
    "error": "Missing required permission: portal:manage"
  }
  ```

- **401 Unauthorized** - No authentication token provided

## Security Considerations

1. **Always use HTTPS in production** - JWT tokens contain sensitive data
2. **Set appropriate token expiration** - Default is 24 hours
3. **Use specific permissions** - Avoid overly broad wildcards like `*:*`
4. **Audit badge assignments** - Track who gets which badges and why
5. **Regular permission reviews** - Remove badges from users who no longer need them

## Troubleshooting

### "PermissionChecker not found in app data"

- Ensure you added `PermissionChecker` to app data:

  ```rust
  .app_data(web::Data::new(permission_checker.clone()))
  ```

### "Authentication required"

- Request must include valid JWT token in Authorization header
- Verify JWT middleware is configured before permission middleware

### "Permission check failed"

- Check database connectivity
- Verify badge_registry and user_badges tables exist
- Check logs for detailed error messages

### Badge registration fails

- Ensure badge-service is running on expected port (8007)
- Check BADGE_SERVICE_URL environment variable
- Verify network connectivity between services

## Complete Example

See working implementation in:

- `services/territory-service/src/main.rs` - Badge registration
- `docs/examples/portal-service-badges.json` - Badge definitions
- `scripts/register-portal-badges.sh` - Batch badge registration
