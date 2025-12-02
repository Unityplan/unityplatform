# Foreign Key Removal Implementation Plan

**Date:** December 2, 2025  
**Status:** Planning Phase  
**Goal:** Remove all cross-service foreign keys to auth_users_core

---

## Strategy Summary

**Validation:**
- Primary: JWT-based validation (best performance)
- Fallback: API validation helper for admin operations

**Cleanup:**
- Immediate: NATS events for soft delete
- Scheduled: Cron job in task-scheduler-service for hard cleanup

**Timeline:** All 3 phases (dev environment allows DB reset)

**Scope:** Service-by-service implementation

---

## Services & FK Count

| Service | FKs to Remove | Complexity | Order |
|---------|---------------|------------|-------|
| settings-service | 1 | Low | 1st |
| badge-service | 3 | Medium | 2nd |
| user-service | 6 | Medium | 3rd |
| community-service | 5 | High | 4th |
| **Total** | **15** | - | - |

---

## Phase 1: Preparation (Per Service)

### For Each Service:

**1.1 Add JWT Validation**
```rust
// Ensure handlers use AuthUser extractor
async fn handler(user: AuthUser, ...) -> Result<HttpResponse> {
    // user.id is validated by JWT middleware
    // No need to query auth-service
}
```

**1.2 Add API Validation Helper (if needed)**
```rust
// For admin operations without JWT context
use shared_lib::validation::validate_user_exists;

if !validate_user_exists(user_id, &territory, pool).await? {
    return Err(AppError::NotFound("User not found".into()));
}
```

**1.3 Add Tests**
- [ ] Test JWT validation works
- [ ] Test graceful handling of invalid user_id
- [ ] All existing tests still pass

---

## Phase 2: Migration (Per Service)

### Template Migration

```sql
-- 20251202_remove_{service}_foreign_keys.sql

-- Remove FK constraint
ALTER TABLE territory_dk.{service}_{table} 
    DROP CONSTRAINT IF EXISTS {constraint_name};

-- Add documentation
COMMENT ON COLUMN territory_dk.{service}_{table}.user_id IS 
    'User ID validated via JWT token. No FK constraint for service independence.';

-- Success message
DO $$
BEGIN
    RAISE NOTICE '✅ Removed FK: {service}_{table}.user_id -> auth_users_core';
END $$;
```

### Test Migration Applied

```rust
#[tokio::test]
async fn test_no_fks_to_auth_users_core_in_{service}() {
    let fks = sqlx::query!(
        "SELECT tc.table_name, kcu.column_name
         FROM information_schema.table_constraints tc 
         JOIN information_schema.key_column_usage kcu
           ON tc.constraint_name = kcu.constraint_name
         JOIN information_schema.constraint_column_usage ccu
           ON ccu.constraint_name = tc.constraint_name
         WHERE tc.constraint_type = 'FOREIGN KEY'
           AND tc.table_schema = 'territory_dk'
           AND tc.table_name LIKE '{service}_%'
           AND ccu.table_name = 'auth_users_core'"
    ).fetch_all(pool).await?;
    
    assert_eq!(fks.len(), 0, "Found FK violations in {service}");
}
```

---

## Phase 3: Cleanup Strategy

### 3.1 NATS Event-Driven Soft Delete

**auth-service publishes:**
```rust
// When user requests account deletion
nats.publish("user.deletion.requested", json!({
    "user_id": user_id,
    "territory": territory,
    "requested_at": Utc::now()
})).await;
```

**Each service subscribes:**
```rust
// settings-service example
nats.subscribe("user.deletion.requested", |event| async move {
    // Soft delete: mark as deleted, don't remove data yet
    sqlx::query(
        "UPDATE settings_users_settings 
         SET deleted_at = NOW() 
         WHERE user_id = $1"
    )
    .bind(event.user_id)
    .execute(pool)
    .await?;
}).await;
```

### 3.2 Task Scheduler Service (Hard Cleanup)

**New service:** `task-scheduler-service`

```rust
// Runs daily via cron (e.g., 2 AM)
async fn cleanup_deleted_users(pool: &PgPool) {
    // Clean up users deleted > 30 days ago
    let cutoff = Utc::now() - Duration::days(30);
    
    // Settings
    sqlx::query(
        "DELETE FROM settings_users_settings 
         WHERE deleted_at < $1"
    )
    .bind(cutoff)
    .execute(pool)
    .await?;
    
    // Badge
    sqlx::query(
        "DELETE FROM badge_users_badges 
         WHERE deleted_at < $1"
    )
    .bind(cutoff)
    .execute(pool)
    .await?;
    
    // User profiles
    sqlx::query(
        "DELETE FROM user_users_profiles 
         WHERE deleted_at < $1"
    )
    .bind(cutoff)
    .execute(pool)
    .await?;
    
    // Communities (members/managers)
    sqlx::query(
        "DELETE FROM community_communities_members 
         WHERE deleted_at < $1"
    )
    .bind(cutoff)
    .execute(pool)
    .await?;
    
    tracing::info!("Cleanup complete: removed users deleted before {}", cutoff);
}
```

### 3.3 Update Tables for Soft Delete

```sql
-- Add deleted_at column to all user-related tables
ALTER TABLE territory_dk.settings_users_settings 
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE territory_dk.badge_users_badges 
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE territory_dk.user_users_profiles 
    ADD COLUMN deleted_at TIMESTAMPTZ;

-- Create index for cleanup query performance
CREATE INDEX idx_settings_users_deleted 
    ON territory_dk.settings_users_settings(deleted_at) 
    WHERE deleted_at IS NOT NULL;
```

---

## Implementation Checklist

### Settings Service (Pilot)
- [ ] Phase 1: Add JWT validation
- [ ] Phase 1: Update handlers
- [ ] Phase 1: Tests pass
- [ ] Phase 2: Create migration
- [ ] Phase 2: Apply migration (dev DB)
- [ ] Phase 2: Add FK detection test
- [ ] Phase 2: All tests pass
- [ ] Phase 3: Add deleted_at column
- [ ] Phase 3: Add NATS subscription
- [ ] Phase 3: Test soft delete flow
- [ ] Document learnings for next service

### Badge Service
- [ ] Phase 1: Add JWT validation
- [ ] Phase 1: Update handlers (3 tables)
- [ ] Phase 1: Tests pass
- [ ] Phase 2: Create migration
- [ ] Phase 2: Apply migration
- [ ] Phase 2: Add FK detection test
- [ ] Phase 3: Add deleted_at columns
- [ ] Phase 3: Add NATS subscription
- [ ] Phase 3: Test cleanup

### User Service
- [ ] Phase 1: Add JWT validation
- [ ] Phase 1: Update handlers (6 tables)
- [ ] Phase 1: Tests pass
- [ ] Phase 2: Create migration
- [ ] Phase 2: Apply migration
- [ ] Phase 2: Add FK detection test
- [ ] Phase 3: Add deleted_at columns
- [ ] Phase 3: Add NATS subscription
- [ ] Phase 3: Test cleanup

### Community Service
- [ ] Phase 1: Add JWT validation
- [ ] Phase 1: Update handlers (5 tables, includes managers)
- [ ] Phase 1: Tests pass
- [ ] Phase 2: Create migration
- [ ] Phase 2: Apply migration
- [ ] Phase 2: Add FK detection test
- [ ] Phase 3: Add deleted_at columns
- [ ] Phase 3: Add NATS subscription
- [ ] Phase 3: Test cleanup

### Task Scheduler Service (New)
- [ ] Scaffold new service
- [ ] Implement cron job infrastructure
- [ ] Add cleanup function
- [ ] Configure schedule (daily 2 AM)
- [ ] Add monitoring/logging
- [ ] Test cleanup execution
- [ ] Deploy alongside other services

---

## Testing Strategy

### 1. FK Detection Test (All Services)
```rust
#[tokio::test]
async fn test_no_cross_service_foreign_keys() {
    let fks = query_foreign_keys_to_auth_users_core(pool).await?;
    assert_eq!(fks.len(), 0, "Cross-service FKs found: {:?}", fks);
}
```

### 2. JWT Validation Test
```rust
#[actix_web::test]
async fn test_jwt_validation_prevents_invalid_user() {
    // Create JWT with non-existent user_id
    let fake_user_id = Uuid::new_v4();
    let token = create_jwt_with_user_id(fake_user_id);
    
    // Try to create profile
    let resp = test_request(&app)
        .uri("/api/v1/profiles")
        .bearer_auth(token)
        .post()
        .await;
    
    // Should succeed (JWT is valid)
    // Service relies on JWT signature, not DB check
    assert_eq!(resp.status(), 201);
}
```

### 3. Soft Delete Flow Test
```rust
#[tokio::test]
async fn test_user_deletion_flow() {
    // 1. Create user data
    let user_id = create_test_user().await;
    create_user_settings(user_id).await;
    
    // 2. Publish deletion event
    nats.publish("user.deletion.requested", json!({
        "user_id": user_id,
        "territory": "dk"
    })).await;
    
    // 3. Wait for NATS processing
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // 4. Verify soft delete
    let settings = get_user_settings(user_id).await?;
    assert!(settings.deleted_at.is_some());
    
    // 5. Verify data still exists
    assert!(settings.is_some());
}
```

---

## Rollback Strategy

### Per-Service Rollback

```sql
-- If issues found after removing FK
ALTER TABLE territory_dk.{service}_{table} 
    ADD CONSTRAINT {service}_{table}_user_id_fkey 
    FOREIGN KEY (user_id) 
    REFERENCES territory_dk.auth_users_core(id) 
    ON DELETE CASCADE;
```

### Full Rollback

```bash
# Reset database to before FK removal
psql unityplatform_dk < backup_before_fk_removal.sql

# Re-run migrations up to last working state
sqlx migrate run --source services/shared-lib/migrations
```

---

## Success Metrics

- [ ] Zero cross-service foreign keys detected
- [ ] All 96+ tests passing
- [ ] Zero compiler warnings
- [ ] JWT validation working (no auth-service queries)
- [ ] NATS soft delete working
- [ ] Task scheduler cleanup working
- [ ] Services independently deployable
- [ ] Documentation updated
- [ ] Performance improvement measured (reduced DB queries)

---

## Timeline Estimate

| Service | Prep | Migration | Cleanup | Total |
|---------|------|-----------|---------|-------|
| settings-service | 1 day | 0.5 day | 1 day | 2.5 days |
| badge-service | 1 day | 0.5 day | 0.5 day | 2 days |
| user-service | 2 days | 0.5 day | 0.5 day | 3 days |
| community-service | 2 days | 1 day | 0.5 day | 3.5 days |
| task-scheduler | - | - | 2 days | 2 days |
| **Total** | **6 days** | **2.5 days** | **4.5 days** | **13 days** |

**Recommended Sprint:** 3 weeks (buffer for testing & issues)

---

## Next Steps

1. **Review & Approve Plan** ✓ (You confirmed)
2. **Start Settings Service** (Pilot)
   - Implement Phase 1 (JWT validation)
   - Create Phase 2 migration
   - Test FK detection
3. **Iterate to Next Service**
4. **Build Task Scheduler**
5. **Complete All Services**
6. **Update Documentation**

---

## Questions Resolved

✅ Validation Strategy: JWT primary, API fallback  
✅ Cleanup Strategy: NATS soft delete + scheduled hard cleanup  
✅ Timeline: All 3 phases  
✅ Scope: Service-by-service  
✅ Testing: FK detection test + validation tests  

---

## Ready to Start?

**Shall we begin with settings-service Phase 1?**

This will involve:
1. Checking current handler implementation
2. Ensuring AuthUser extractor is used
3. Adding validation helper to shared-lib
4. Writing FK detection test
5. Verifying all tests pass

**Proceed? (Y/N)**
