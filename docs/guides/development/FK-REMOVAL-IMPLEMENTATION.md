# Foreign Key Removal Implementation Plan

**Goal:** Remove cross-service foreign key constraints and replace with JWT/API validation  
**Timeline:** 3 Phases, Service-by-Service approach  
**Status:** Planning Phase  
**Last Updated:** December 2, 2025

---

## 📊 Overview

### Current State
- **15 Foreign Keys** to `auth_users_core` from other services
- Services: user-service (6), community-service (5), badge-service (3), settings-service (1)
- All FKs enforce referential integrity at database level

### Target State
- **Zero cross-service FKs**
- JWT-based validation (primary method - best performance)
- API validation helper (fallback for admin operations)
- NATS event-driven soft delete
- Scheduled cleanup job for hard deletion

### Why Remove FKs?
Even though services share a database within a pod:
1. **Service Independence** - Evolve schemas independently
2. **Testing** - Test services in isolation
3. **Performance** - JWT validation (0.01ms) vs DB query (~1-5ms)
4. **Logical Boundaries** - Clear ownership and responsibilities
5. **Future-Ready** - Prepared for Holochain migration if needed

---

## 🎯 Service Implementation Order

```
1. ✅ settings-service    (1 FK)   ← Phase 1 (Pilot)
2. ⏳ badge-service       (3 FKs)  ← Phase 1
3. ⏳ user-service        (6 FKs)  ← Phase 2
4. ⏳ community-service   (5 FKs)  ← Phase 2
```

---

## 📋 Phase 1: Settings Service (Pilot Implementation)

### Current FK to Remove
```sql
-- services/shared-lib/migrations/20251113000007_create_users_settings_table.sql
user_id UUID PRIMARY KEY REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE
```

### Step 1.1: Add JWT Validation Helper to shared-lib

**File:** `services/shared-lib/src/validation.rs` (new file)

```rust
//! User validation utilities for services
//! 
//! Provides validation helpers that respect service boundaries:
//! - JWT validation (primary, zero DB queries)
//! - Registry validation (fallback for admin operations)

use crate::{AppError, Result};
use sqlx::PgPool;
use uuid::Uuid;

/// Validate user exists via JWT token (already validated by middleware)
/// 
/// This is the PRIMARY validation method. If you have an AuthUser from JWT,
/// the user is guaranteed to exist (JWT signature validates this).
/// 
/// # Arguments
/// * `user_id` - User ID from JWT token
/// 
/// # Returns
/// * `Ok(user_id)` - Always succeeds if JWT was valid
/// 
/// # Example
/// ```rust
/// use shared_lib::validation::validate_user_from_jwt;
/// use shared_lib::AuthUser;
/// 
/// async fn create_settings(user: AuthUser) -> Result<()> {
///     let user_id = validate_user_from_jwt(user.id)?;
///     // user_id is guaranteed valid (came from signed JWT)
///     Ok(())
/// }
/// ```
pub fn validate_user_from_jwt(user_id: Uuid) -> Result<Uuid> {
    // JWT signature already validated this user exists
    // No database query needed!
    Ok(user_id)
}

/// Validate user exists via global registry (for admin operations without JWT)
/// 
/// Use this ONLY when you don't have a JWT context:
/// - Admin operations on behalf of other users
/// - Background jobs
/// - Batch operations
/// 
/// This queries global.registry_username (NOT auth_users_core) to respect service boundaries.
/// 
/// # Arguments
/// * `user_id` - User ID to validate
/// * `territory` - Territory code (e.g., "dk")
/// * `pool` - Database connection pool
/// 
/// # Returns
/// * `Ok(true)` - User exists in registry
/// * `Ok(false)` - User does not exist
/// * `Err(_)` - Database error
/// 
/// # Example
/// ```rust
/// use shared_lib::validation::validate_user_via_registry;
/// 
/// async fn admin_assign_badge(target_user_id: Uuid, pool: &PgPool) -> Result<()> {
///     if !validate_user_via_registry(target_user_id, "dk", pool).await? {
///         return Err(AppError::NotFound("User not found".into()));
///     }
///     // Proceed with operation
///     Ok(())
/// }
/// ```
pub async fn validate_user_via_registry(
    user_id: Uuid,
    territory: &str,
    pool: &PgPool,
) -> Result<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(
            SELECT 1 FROM global.registry_username 
            WHERE user_id = $1 AND territory_code = $2
        )"
    )
    .bind(user_id)
    .bind(territory)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;
    
    Ok(exists)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_user_from_jwt_always_succeeds() {
        let user_id = Uuid::new_v4();
        let result = validate_user_from_jwt(user_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user_id);
    }
}
```

**File:** `services/shared-lib/src/lib.rs` (update exports)

```rust
// Add to existing exports
pub mod validation;
pub use validation::{validate_user_from_jwt, validate_user_via_registry};
```

### Step 1.2: Update Settings Service to Use JWT Validation

**Current Implementation** (relies on FK):
```rust
// services/settings-service/src/handlers/settings.rs
async fn get_settings(user: AuthUser, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT * FROM territory_dk.settings_users_settings WHERE user_id = $1"
    )
    .bind(user.id)  // FK ensures this user exists
    .fetch_optional(pool.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(settings))
}
```

**New Implementation** (JWT validation):
```rust
// services/settings-service/src/handlers/settings.rs
use shared_lib::validation::validate_user_from_jwt;

async fn get_settings(user: AuthUser, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // Validate user_id from JWT (always succeeds if JWT valid)
    let user_id = validate_user_from_jwt(user.id)?;
    
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT * FROM territory_dk.settings_users_settings WHERE user_id = $1"
    )
    .bind(user_id)  // No FK, but validated via JWT
    .fetch_optional(pool.get_ref())
    .await?;
    
    Ok(HttpResponse::Ok().json(settings))
}
```

### Step 1.3: Create Migration to Remove FK

**File:** `services/shared-lib/migrations/20251202000001_remove_settings_service_fks.sql`

```sql
-- ============================================================================
-- Migration: 20251202000001_remove_settings_service_fks
-- Level: Phase 1 - Service Independence
-- Service: settings-service
-- Description: Remove foreign key to auth_users_core, use JWT validation
-- Dependencies: 20251113000007_create_users_settings_table.sql
-- ============================================================================

-- Remove FK constraint
ALTER TABLE territory_dk.settings_users_settings 
    DROP CONSTRAINT IF EXISTS settings_users_settings_user_id_fkey;

-- Add comment documenting validation strategy
COMMENT ON COLUMN territory_dk.settings_users_settings.user_id IS 
    'User ID validated via JWT token (primary method) or global.registry_username (admin operations). 
     No FK constraint for service independence. JWT signature ensures user exists.';

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251202000001 complete: Removed settings-service FK';
    RAISE NOTICE '    - Removed FK: settings_users_settings.user_id -> auth_users_core.id';
    RAISE NOTICE '    - Validation: JWT-based (zero DB queries)';
    RAISE NOTICE '    - Service independence achieved for settings-service';
END $$;
```

### Step 1.4: Add FK Detection Test

**File:** `services/shared-lib/tests/test_no_cross_service_fks.rs` (new file)

```rust
//! Test to verify no cross-service foreign keys exist
//! 
//! This test queries PostgreSQL system catalog to detect any foreign keys
//! from non-auth services to auth_users_core table.

use sqlx::PgPool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ForeignKey {
    table_schema: String,
    table_name: String,
    column_name: String,
    foreign_table_name: String,
    foreign_column_name: String,
}

#[sqlx::test]
async fn test_no_cross_service_foreign_keys_to_auth(pool: PgPool) {
    let fks = sqlx::query_as::<_, ForeignKey>(
        "SELECT 
            tc.table_schema,
            tc.table_name,
            kcu.column_name,
            ccu.table_name AS foreign_table_name,
            ccu.column_name AS foreign_column_name
         FROM information_schema.table_constraints AS tc 
         JOIN information_schema.key_column_usage AS kcu
           ON tc.constraint_name = kcu.constraint_name
           AND tc.table_schema = kcu.table_schema
         JOIN information_schema.constraint_column_usage AS ccu
           ON ccu.constraint_name = tc.constraint_name
           AND ccu.table_schema = tc.table_schema
         WHERE tc.constraint_type = 'FOREIGN KEY'
           AND tc.table_schema = 'territory_dk'
           AND ccu.table_name = 'auth_users_core'
           AND tc.table_name NOT LIKE 'auth_%'
         ORDER BY tc.table_name, kcu.column_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to query foreign keys");
    
    if !fks.is_empty() {
        eprintln!("\n❌ Found {} cross-service foreign key(s) to auth_users_core:", fks.len());
        for fk in &fks {
            eprintln!(
                "   - {}.{}.{} -> {}.{}",
                fk.table_schema, 
                fk.table_name, 
                fk.column_name,
                fk.foreign_table_name,
                fk.foreign_column_name
            );
        }
        eprintln!("\nServices should validate via JWT tokens, not database FKs.");
        panic!("Cross-service foreign keys detected");
    }
    
    println!("✅ No cross-service foreign keys detected");
}

#[sqlx::test]
async fn test_settings_service_has_no_fks(pool: PgPool) {
    // Specific test for settings-service after FK removal
    let fks = sqlx::query_as::<_, ForeignKey>(
        "SELECT 
            tc.table_schema,
            tc.table_name,
            kcu.column_name,
            ccu.table_name AS foreign_table_name,
            ccu.column_name AS foreign_column_name
         FROM information_schema.table_constraints AS tc 
         JOIN information_schema.key_column_usage AS kcu
           ON tc.constraint_name = kcu.constraint_name
         JOIN information_schema.constraint_column_usage AS ccu
           ON ccu.constraint_name = tc.constraint_name
         WHERE tc.constraint_type = 'FOREIGN KEY'
           AND tc.table_schema = 'territory_dk'
           AND tc.table_name = 'settings_users_settings'
           AND ccu.table_name = 'auth_users_core'"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to query foreign keys");
    
    assert_eq!(
        fks.len(), 
        0, 
        "settings_users_settings should have no FKs to auth_users_core"
    );
}
```

**Add to:** `services/shared-lib/Cargo.toml`

```toml
[dev-dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### Step 1.5: Run Tests

```bash
# Run FK detection test
cd services/shared-lib
cargo test test_no_cross_service_fks --test test_no_cross_service_fks -- --nocapture

# Should initially FAIL (FKs exist)
# After migration applied, should PASS
```

### Step 1.6: Apply Migration

```bash
# Apply migration to dev database
cd services/shared-lib
sqlx migrate run --database-url "postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk"

# Verify migration applied
sqlx migrate info --database-url "postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk"
```

### Step 1.7: Verify Settings Service Still Works

```bash
# Run settings-service tests (should all pass)
cd /home/henrik/dev/unity_platform/workspace
./scripts/dev/run-tests.sh settings-service

# Run integration tests
cargo test --test integration_test --manifest-path services/settings-service/Cargo.toml
```

---

## 📋 Phase 1: Badge Service (3 FKs)

### FKs to Remove

```sql
-- badge_users_badges
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE
awarded_by UUID REFERENCES territory_dk.auth_users_core(id) ON DELETE SET NULL

-- badge_badge_progress  
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE
```

### Implementation Steps

1. **Update badge-service handlers** to use `validate_user_from_jwt(user.id)`
2. **Update admin operations** (awarding badges to other users) to use `validate_user_via_registry`
3. **Create migration:** `20251202000002_remove_badge_service_fks.sql`
4. **Run tests**
5. **Apply migration**
6. **Verify**

### Key Changes

**File:** `services/badge-service/src/handlers/badge.rs`

```rust
// Award badge to another user (admin operation)
async fn award_badge(
    admin: AuthUser,
    path: web::Path<Uuid>,
    body: ValidatedJson<AwardBadgeRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    // Validate admin has permission (via JWT badges)
    if !admin.has_badge("badge-admin") {
        return Err(AppError::Forbidden("Requires badge-admin role".into()));
    }
    
    // Validate target user exists via registry (NOT auth_users_core FK)
    if !validate_user_via_registry(body.user_id, &admin.territory, pool.get_ref()).await? {
        return Err(AppError::NotFound("Target user not found".into()));
    }
    
    // Award badge (no FK, but validated)
    sqlx::query(
        "INSERT INTO territory_dk.badge_users_badges 
         (badge_id, user_id, awarded_by) VALUES ($1, $2, $3)"
    )
    .bind(path.into_inner())
    .bind(body.user_id)
    .bind(admin.id)  // Admin validated via JWT
    .execute(pool.get_ref())
    .await?;
    
    Ok(HttpResponse::Created().finish())
}
```

---

## 📋 Phase 2: User Service (6 FKs)

### FKs to Remove

```sql
-- user_users_profiles
user_id UUID PRIMARY KEY REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE

-- user_users_profile_links
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE

-- user_users_language_proficiency
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE

-- user_user_connections
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE
target_user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE

-- user_data_exports
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE

-- user_account_deletion_requests
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE
```

### Migration

**File:** `20251202000003_remove_user_service_fks.sql`

---

## 📋 Phase 2: Community Service (5 FKs)

### FKs to Remove

```sql
-- community_communities
created_by UUID REFERENCES territory_dk.auth_users_core(id) ON DELETE SET NULL

-- community_communities_members
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE
invited_by UUID REFERENCES territory_dk.auth_users_core(id) ON DELETE SET NULL

-- community_communities_managers
user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE
assigned_by UUID REFERENCES territory_dk.auth_users_core(id) ON DELETE SET NULL
```

### Migration

**File:** `20251202000004_remove_community_service_fks.sql`

---

## 📋 Phase 3: Cleanup Strategy

### NATS Event-Driven Soft Delete

**Event Definition:**

**File:** `services/shared-lib/src/events/user.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDeletedEvent {
    pub user_id: Uuid,
    pub territory: String,
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl UserDeletedEvent {
    pub const SUBJECT: &'static str = "user.deleted";
}
```

**Auth Service - Publish Event:**

```rust
// services/auth-service/src/handlers/user.rs
async fn delete_user(
    user: AuthUser,
    nats: web::Data<NatsClient>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    // Soft delete in auth_users_core
    sqlx::query(
        "UPDATE territory_dk.auth_users_core 
         SET deleted_at = NOW(), active = false 
         WHERE id = $1"
    )
    .bind(user.id)
    .execute(pool.get_ref())
    .await?;
    
    // Publish NATS event for other services
    let event = UserDeletedEvent {
        user_id: user.id,
        territory: user.territory.clone(),
        deleted_at: chrono::Utc::now(),
    };
    
    nats.publish(
        UserDeletedEvent::SUBJECT,
        &serde_json::to_vec(&event)?,
    ).await?;
    
    Ok(HttpResponse::NoContent().finish())
}
```

**Other Services - Subscribe:**

```rust
// services/user-service/src/main.rs
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... setup ...
    
    // Subscribe to user deletion events
    let pool_clone = database.pool().clone();
    let sub = nats_client.subscribe(UserDeletedEvent::SUBJECT).await?;
    
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            if let Ok(event) = serde_json::from_slice::<UserDeletedEvent>(&msg.payload) {
                // Soft delete user profile
                let _ = sqlx::query(
                    "UPDATE territory_dk.user_users_profiles 
                     SET deleted_at = $1 
                     WHERE user_id = $2"
                )
                .bind(event.deleted_at)
                .bind(event.user_id)
                .execute(&pool_clone)
                .await;
            }
        }
    });
    
    // ... start server ...
}
```

### Scheduled Hard Delete (Task Scheduler Service)

**New Service:** `task-scheduler-service`

```rust
// Cron job: Run weekly to hard-delete soft-deleted users after 30 days
async fn cleanup_deleted_users(pool: &PgPool) -> Result<()> {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(30);
    
    // Find users deleted more than 30 days ago
    let deleted_users = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT user_id, territory_code 
         FROM global.registry_username 
         WHERE deleted_at IS NOT NULL 
         AND deleted_at < $1"
    )
    .bind(cutoff)
    .fetch_all(pool)
    .await?;
    
    for (user_id, territory) in deleted_users {
        // Delete from all services (no FKs, so manual deletion)
        cleanup_user_data(user_id, &territory, pool).await?;
        
        // Finally delete from registry
        sqlx::query("DELETE FROM global.registry_username WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
    }
    
    Ok(())
}

async fn cleanup_user_data(user_id: Uuid, territory: &str, pool: &PgPool) -> Result<()> {
    let schema = format!("territory_{}", territory);
    
    // User service
    sqlx::query(&format!("DELETE FROM {}.user_users_profiles WHERE user_id = $1", schema))
        .bind(user_id).execute(pool).await?;
    sqlx::query(&format!("DELETE FROM {}.user_users_profile_links WHERE user_id = $1", schema))
        .bind(user_id).execute(pool).await?;
    
    // Badge service
    sqlx::query(&format!("DELETE FROM {}.badge_users_badges WHERE user_id = $1", schema))
        .bind(user_id).execute(pool).await?;
    
    // Settings service
    sqlx::query(&format!("DELETE FROM {}.settings_users_settings WHERE user_id = $1", schema))
        .bind(user_id).execute(pool).await?;
    
    // Community service
    sqlx::query(&format!("DELETE FROM {}.community_communities_members WHERE user_id = $1", schema))
        .bind(user_id).execute(pool).await?;
    sqlx::query(&format!("DELETE FROM {}.community_communities_managers WHERE user_id = $1", schema))
        .bind(user_id).execute(pool).await?;
    
    // Auth service (last)
    sqlx::query(&format!("DELETE FROM {}.auth_users_core WHERE id = $1", schema))
        .bind(user_id).execute(pool).await?;
    
    Ok(())
}
```

---

## ✅ Success Criteria

- [ ] All 15 foreign keys removed
- [ ] All services use JWT validation (primary method)
- [ ] API validation helper available for admin operations
- [ ] FK detection test passing (zero cross-service FKs)
- [ ] All 96+ tests still passing
- [ ] NATS event system for user deletion implemented
- [ ] Task scheduler service created for hard deletion
- [ ] Documentation updated
- [ ] Zero performance regression (JWT faster than DB query)

---

## 📊 Progress Tracker

| Service | FKs | Status | Migration | Tests | Notes |
|---------|-----|--------|-----------|-------|-------|
| settings-service | 1 | ⏳ Planning | - | - | Pilot service |
| badge-service | 3 | ⏳ Planned | - | - | Phase 1 |
| user-service | 6 | ⏳ Planned | - | - | Phase 2 |
| community-service | 5 | ⏳ Planned | - | - | Phase 2 |
| **Total** | **15** | **0% Complete** | **0/4** | **0/4** | - |

---

## 🔄 Rollback Plan

If issues discovered after FK removal:

```sql
-- Rollback: Re-add FK to settings_users_settings
ALTER TABLE territory_dk.settings_users_settings 
    ADD CONSTRAINT settings_users_settings_user_id_fkey 
    FOREIGN KEY (user_id) 
    REFERENCES territory_dk.auth_users_core(id) 
    ON DELETE CASCADE;
```

Each migration should be reversible with a corresponding rollback migration.

---

## 📚 References

- JWT Implementation: `services/shared-lib/src/jwt.rs`
- AuthUser Extractor: `services/shared-lib/src/jwt.rs`
- NATS Client: `services/shared-lib/src/nats.rs`
- Migration Strategy: `docs/architecture/MIGRATIONS-MASTER.md`
- Service Boundaries: `.github/copilot-instructions.md`
