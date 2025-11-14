# Permission System Documentation

**Module:** `shared_lib::permission`  
**Version:** 0.1.0-alpha.1  
**Status:** ✅ Implemented - Phase 1 Complete

---

## Overview

The permission system provides badge-based role-based access control (RBAC) for Unity Platform services. Permissions are granted through badges, and checked via middleware that queries the badge-service database.

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Service Request Flow                      │
└─────────────────────────────────────────────────────────────┘

1. User Request → JWT Middleware (extracts user_id)
                         ↓
2. PermissionChecker Middleware (checks permissions)
                         ↓
3. Query: global.badge_registry + territory_XX.user_badges
                         ↓
4. Check: Does user have badge with required permission?
                         ↓
         ┌─────────────┴──────────────┐
         YES                          NO
         ↓                            ↓
   Continue Request            403 Forbidden
```

---

## Components

### 1. PermissionChecker

**Location:** `services/shared-lib/src/permission/checker.rs`

Queries the database to verify if a user has required permissions through their badges.

```rust
use shared_lib::PermissionChecker;

// Initialize
let permission_checker = PermissionChecker::new(
    database.clone(),
    "dk".to_string(), // territory code
);

// Check single permission
let has_perm = permission_checker
    .has_permission(user_id, "portal:manage")
    .await?;

// Check any of multiple permissions
let has_any = permission_checker
    .has_any_permission(user_id, &["content:moderate", "portal:manage"])
    .await?;

// Check all of multiple permissions
let has_all = permission_checker
    .has_all_permissions(user_id, &["portal:manage", "portal:deploy"])
    .await?;
```

#### Features

- **Caching:** 5-minute TTL cache to reduce database queries
- **Hierarchical Permissions:** `portal:*` grants all `portal:X` permissions
- **Territory-Aware:** Queries `territory_{code}.user_badges`
- **Badge Integration:** Reads `grants_permissions` JSONB from `global.badge_registry`

#### Database Queries

The PermissionChecker performs joins across:

- `global.badge_registry` - Badge definitions with permissions
- `territory_{code}.user_badges` - User badge ownership

```sql
SELECT br.grants_permissions
FROM territory_dk.user_badges ub
JOIN global.badge_registry br ON ub.badge_id = br.id
WHERE ub.user_id = $1 AND br.is_active = true
```

### 2. RequirePermission Middleware

**Location:** `services/shared-lib/src/permission/middleware.rs`

Actix-web middleware that enforces permission requirements on routes.

```rust
use actix_web::web;
use shared_lib::permission::RequirePermission;

// Protect entire scope
web::scope("/admin")
    .wrap(RequirePermission::new("portal:manage"))
    .route("/settings", web::post().to(update_settings))
    .route("/users", web::get().to(list_users))
```

#### Request Flow

1. Extract `user_id` from request extensions (set by JWT middleware)
2. Extract `PermissionChecker` from app data
3. Call `permission_checker.has_permission(user_id, permission)`
4. If true → continue request
5. If false → return `403 Forbidden`

#### Error Responses

```json
// No authentication
{
  "error": "Authentication required"
}

// Missing permission
{
  "error": "Missing required permission: portal:manage"
}
```

### 3. RequireAnyPermission Middleware

Allows access if user has **any** of the specified permissions.

```rust
use shared_lib::permission::RequireAnyPermission;

web::scope("/moderate")
    .wrap(RequireAnyPermission::new(vec![
        "content:moderate",
        "portal:manage",
    ]))
    .route("/reports", web::get().to(view_reports))
```

---

## Setup Guide

### Step 1: Initialize PermissionChecker

```rust
use shared_lib::{PermissionChecker, Database};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... config, database setup ...
    
    let database = Database::new(...).await?;
    
    // Initialize permission checker
    let permission_checker = PermissionChecker::new(
        database.clone(),
        config.territory_code(), // e.g., "dk"
    );
    
    // Create server with PermissionChecker in app data
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(permission_checker.clone()))
            // ... rest of setup
    })
}
```

### Step 2: Add Permission Middleware

```rust
use shared_lib::permission::{RequirePermission, RequireAnyPermission};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Public routes - no middleware
        .route("/public", web::get().to(public_handler))
        
        // Protected scope
        .service(
            web::scope("/admin")
                .wrap(RequirePermission::new("portal:manage"))
                .route("/settings", web::post().to(update_settings))
        )
        
        // Per-route protection
        .route(
            "/moderate",
            web::post()
                .to(moderate_content)
                .wrap(RequireAnyPermission::new(vec![
                    "content:moderate",
                    "portal:manage"
                ]))
        );
}
```

### Step 3: Register Service Badges

Services should register their role badges on startup:

```rust
async fn register_service_badges(config: &AppConfig) {
    let badge_service_url = std::env::var("BADGE_SERVICE_URL")
        .unwrap_or_else(|_| format!("http://{}:8007", config.server.host));
    
    let badge = json!({
        "slug": "service-manager",
        "name": "Service Manager",
        "description": "Manages service settings",
        "icon": "⚙️",
        "category": "role",
        "criteriaType": "manual",
        "rarity": "epic",
        "grantsPermissions": ["service:manage", "service:settings:manage"]
    });
    
    reqwest::Client::new()
        .post(format!("{}/api/v1/badges/register", badge_service_url))
        .json(&badge)
        .send()
        .await?;
}
```

---

## Permission Naming Convention

Use hierarchical permission strings with `:` separator:

```
<service>:<resource>:<action>
```

### Examples

- `portal:manage` - Full portal management
- `portal:users:view` - View portal users
- `portal:users:manage` - Manage portal users
- `territory:manage` - Full territory management
- `territory:settings:manage` - Manage territory settings
- `content:moderate` - Moderate content
- `course:create` - Create courses

### Wildcards

The system supports wildcard matching:

- `portal:*` → Grants all `portal:X` permissions
- `portal:users:*` → Grants all `portal:users:X` permissions

**Implementation:** Uses `starts_with()` matching in PermissionChecker

---

## Badge Categories

| Category | Purpose | Example Badges |
|----------|---------|----------------|
| `role` | Management & access roles | Platform Manager, Territory Manager, Portal Developer |
| `achievement` | User accomplishments | Course Completion, Community Builder |
| `code_of_conduct` | Platform agreements | Code of Conduct Acceptance |
| `special` | Limited/event badges | Early Adopter, Beta Tester |

**Only `role` category badges should grant permissions.**

---

## Database Schema

### global.badge_registry

```sql
CREATE TABLE global.badge_registry (
    id UUID PRIMARY KEY,
    slug VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    icon VARCHAR(100) NOT NULL,
    category VARCHAR(50) NOT NULL,
    criteria_type VARCHAR(50) NOT NULL,
    criteria_value INTEGER,
    rarity VARCHAR(20) NOT NULL,
    is_renewable BOOLEAN DEFAULT false,
    renewal_days INTEGER,
    grants_permissions JSONB DEFAULT '[]'::jsonb,  -- <-- Permission grants
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### territory_{code}.user_badges

```sql
CREATE TABLE territory_dk.user_badges (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    badge_id UUID NOT NULL,
    awarded_at TIMESTAMPTZ DEFAULT NOW(),
    awarded_by UUID,
    is_featured BOOLEAN DEFAULT false,
    UNIQUE(user_id, badge_id)
);
```

---

## Caching Strategy

The PermissionChecker uses an LRU cache with:

- **Capacity:** 1000 entries
- **TTL:** 5 minutes (300 seconds)
- **Key:** `{user_id}:{permission}`
- **Value:** `bool` (has permission)

### Cache Invalidation

Currently automatic via TTL. Future improvements:

- Invalidate on badge award/revoke
- NATS event-driven cache invalidation
- Per-user cache clear endpoint

---

## Security Considerations

### 1. JWT Middleware Dependency

PermissionChecker middleware **requires** JWT middleware to run first:

```rust
App::new()
    // JWT middleware MUST come before permission middleware
    .wrap(JwtMiddleware)
    .service(
        web::scope("/admin")
            .wrap(RequirePermission::new("portal:manage"))
    )
```

### 2. Database Connection Required

PermissionChecker needs database access:

- Ensure Database is initialized and connected
- Handle database errors gracefully
- Consider read replicas for high-traffic services

### 3. Cache Timing Attacks

5-minute cache means:

- Permission changes take up to 5 minutes to propagate
- Revoked badges may still grant access for up to 5 minutes
- Critical operations should bypass cache (future enhancement)

### 4. Permission Naming

Use specific permissions to limit blast radius:

- ❌ Bad: `*:*` (all permissions)
- ❌ Bad: `portal:*` for translator role
- ✅ Good: `portal:translations:edit` for translator role

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_permission_check() {
        let db = setup_test_db().await;
        let checker = PermissionChecker::new(db, "test".to_string());
        
        // User with portal-manager badge
        let has_perm = checker
            .has_permission(test_user_id, "portal:manage")
            .await
            .unwrap();
        
        assert!(has_perm);
    }
}
```

### Integration Tests

```rust
#[actix_web::test]
async fn test_protected_endpoint() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(permission_checker))
            .route(
                "/admin",
                web::get()
                    .to(admin_handler)
                    .wrap(RequirePermission::new("portal:manage"))
            )
    ).await;
    
    // Without permission
    let req = test::TestRequest::get()
        .uri("/admin")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
    
    // With permission (user has portal-manager badge)
    let req = test::TestRequest::get()
        .uri("/admin")
        .insert_header(("Authorization", format!("Bearer {}", valid_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}
```

---

## Troubleshooting

### "PermissionChecker not found in app data"

**Cause:** PermissionChecker not added to app data  
**Solution:**

```rust
.app_data(web::Data::new(permission_checker.clone()))
```

### "Authentication required"

**Cause:** No JWT token or user_id not in request extensions  
**Solution:** Ensure JWT middleware runs before permission middleware

### Cache Not Updating

**Cause:** 5-minute TTL hasn't expired  
**Solution:** Wait 5 minutes or restart service to clear cache

### Permission Always Denied

**Cause:** Badge not granted, badge inactive, or wrong permission string  
**Solution:**

1. Check user has badge: `SELECT * FROM territory_dk.user_badges WHERE user_id = '...'`
2. Check badge permissions: `SELECT grants_permissions FROM global.badge_registry WHERE slug = '...'`
3. Verify permission string matches exactly

---

## Examples

See working implementations:

- `services/territory-service/src/main.rs` - Badge registration
- `docs/examples/portal-service-badges.json` - Badge definitions
- `scripts/register-portal-badges.sh` - Batch registration
- `docs/guides/development/permission-system-usage.md` - Complete guide

---

**Last Updated:** November 14, 2025  
**Module Owner:** Core Team  
**Status:** Production Ready ✅
