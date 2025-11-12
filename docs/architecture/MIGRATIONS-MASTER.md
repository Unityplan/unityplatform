# Database Migrations Master Plan

**Last Updated:** November 12, 2025  
**Current Version:** 20251112000005  
**Database:** PostgreSQL 15+ with TimescaleDB

---

## Overview

This document defines the database migration strategy for the UnityPlan platform's microservices architecture with multi-pod deployment support.

**Key Principles:**
1. **Data Sovereignty** - Personal data stays in territory pods
2. **Global Uniqueness** - Usernames/emails unique across all pods  
3. **Performance** - Minimize cross-service queries
4. **Federation** - Support cross-territory communication
5. **Holochain Ready** - Migration path to decentralized storage

---

## Migration Structure

```
services/shared-lib/migrations/
├── 20251111000001_mvp_core_schema.sql          # ✅ Core tables (30 tables)
├── 20251112000001_update_language_proficiency_schema.sql  # ✅ Language updates
├── 20251112000002_fix_users_settings_schema.sql           # ✅ Settings fixes
├── 20251112000003_fix_notification_settings_schema.sql    # ✅ Notification settings
├── 20251112000004_create_data_exports_table.sql           # ✅ GDPR exports
├── 20251112000005_create_account_deletion_requests_table.sql  # ✅ GDPR deletion
└── [Future migrations below]
```

---

## Schema Distribution

### Global Schema (Shared Across All Pods)

**Purpose:** Global uniqueness, territory registry, shared catalogs

| Table | Service Owner | Purpose |
|-------|---------------|---------|
| `global.territories` | territory-service | Pod registry |
| `global.username_registry` | auth-service | Global username uniqueness |
| `global.email_registry` | auth-service | Global email uniqueness |
| `global.invitation_token_registry` | invitation-service | Global token uniqueness |
| `global.badge_definitions` | badge-service | Shared badge catalog (future) |
| `global.courses` | course-service | Shared course catalog (future) |
| `global.course_lessons` | course-service | Course content (future) |
| `global.forum_rooms` | forum-service | Global forum registry (future) |
| `global.translation_resources` | translation-service | Shared translations (future) |

**Total:** 4 tables (existing), 5 tables (future)

---

### Territory Schema (Per-Pod Data)

**Purpose:** User data sovereignty, territory-specific content

| Service | Tables | Data Type |
|---------|--------|-----------|
| **auth-service** | users, refresh_tokens | Authentication |
| **user-service** | users_profiles, users_profile_links, users_language_proficiency, user_connections, data_exports, account_deletion_requests, file_uploads, activities, audit_log | User data + GDPR |
| **settings-service** | users_settings, users_notification_settings | User preferences |
| **invitation-service** | invitation_tokens, invitation_uses | Territory invitations |
| **notification-service** | notifications | User notifications |
| **community-service** | communities, community_members, roles, role_assignments | Territory communities |
| **badge-service** | badge_awards, badge_progress | User achievements |
| **event-service** | community_events, event_rsvps | Territory events |
| **course-service** | course_enrollments, course_progress | User course progress |
| **forum-service** | forum_memberships | User forum participation |
| **ipfs-service** | file_uploads (shared) | File metadata |

**Total:** 30 tables (existing in core migration)

---

## Service-Specific Migration Plans

### Phase 1 Services (MVP - Weeks 1-3)

#### 1. auth-service

**Migrations:**
- ✅ `20251111000001` - users, refresh_tokens, global registries
- 📋 `20251120000001` - email_verification_tokens (future)
- 📋 `20251120000002` - password_reset_tokens (future)
- 📋 `20251120000003` - login_attempts (rate limiting, future)

**Details:** See [auth-service/MIGRATIONS.md](./auth-service/MIGRATIONS.md)

---

#### 2. user-service

**Migrations:**
- ✅ `20251111000001` - users_profiles, users_profile_links, users_language_proficiency
- ✅ `20251112000001` - Language proficiency schema updates
- ✅ `20251112000004` - data_exports table (GDPR Article 20)
- ✅ `20251112000005` - account_deletion_requests (GDPR Article 17)
- 📋 `20251120000004` - user_connections enhancements (future)
- 📋 `20251120000005` - activities table enhancements (future)

**Details:** See [user-service/DATABASE.md](./user-service/DATABASE.md)

---

#### 3. settings-service

**Migrations:**
- ✅ `20251111000001` - users_settings, users_notification_settings
- ✅ `20251112000002` - Settings schema fixes
- ✅ `20251112000003` - Notification settings fixes

**No additional migrations needed.** Tables exist in core schema.

---

#### 4. invitation-service

**Migrations:**
- ✅ `20251111000001` - invitation_tokens, invitation_uses, global registry

**No additional migrations needed.** Tables exist in core schema.

---

#### 5. notification-service

**Migrations:**
- ✅ `20251111000001` - notifications table

**No additional migrations needed.** Tables exist in core schema.

---

### Phase 2 Services (Future)

#### 6. community-service

**Migrations:**
- ✅ `20251111000001` - communities, community_members, roles, role_assignments
- 📋 `20251201000001` - community_invitations (future)
- 📋 `20251201000002` - community_posts (future)
- 📋 `20251201000003` - community_tags (future)

**Status:** Core tables exist, future enhancements needed.

---

#### 7. badge-service

**Migrations:**
- ✅ `20251111000001` - badge_definitions (territory), badge_awards, badge_progress
- 📋 `20251201000004` - global.badge_definitions (move to global schema)
- 📋 `20251201000005` - badge_criteria table (dynamic criteria)

**Note:** Badge definitions should move to global schema for sharing across territories.

---

#### 8. territory-service

**Migrations:**
- ✅ `20251111000001` - global.territories
- 📋 `20251201000006` - global.territory_stats (statistics)
- 📋 `20251201000007` - global.territory_settings (feature flags)

---

#### 9. event-service

**Migrations:**
- ✅ `20251111000001` - community_events, event_rsvps
- 📋 `20251201000008` - event_reminders (future)
- 📋 `20251201000009` - event_attendance (check-in tracking)

---

#### 10. course-service

**Migrations:**
- 📋 `20251215000001` - global.courses (course catalog)
- 📋 `20251215000002` - global.course_lessons (lesson content)
- 📋 `20251215000003` - territory.course_enrollments (user enrollments)
- 📋 `20251215000004` - territory.course_progress (lesson completion)
- 📋 `20251215000005` - territory.course_certificates (GDPR: user owns certificates)

**Status:** Full LMS implementation required.

---

#### 11. forum-service

**Migrations:**
- 📋 `20251220000001` - global.forum_rooms (global forum registry)
- 📋 `20251220000002` - territory.forum_memberships (user participation)
- 📋 `20251220000003` - territory.forum_bookmarks (saved posts)

**Note:** Primary storage is Matrix homeserver (Synapse database). PostgreSQL only for metadata.

---

#### 12. translation-service

**Migrations:**
- 📋 `20251220000004` - global.translation_resources (shared translations)
- 📋 `20251220000005` - global.translation_votes (community voting)
- 📋 `20251220000006` - global.auto_translations (LibreTranslate cache)
- 📋 `20251220000007` - global.translation_contributors (contributor tracking)

**Status:** Full i18n system implementation.

---

#### 13. ipfs-service

**Migrations:**
- ✅ `20251111000001` - file_uploads (shared with user-service)
- 📋 `20251220000008` - territory.user_storage_quotas (quota management)
- 📋 `20251220000009` - territory.ipfs_pins (pin tracking)

---

## Multi-Pod Deployment Strategy

### Single-Pod Deployment (Denmark Only)

**Command:**
```bash
cd services/shared-lib
sqlx migrate run --database-url "postgresql://unityplan:password@localhost:5432/unityplan_dk"
```

**Result:**
- `global.*` schema with 4 tables
- `territory_dk.*` schema with 30 tables

**Use Case:** MVP development, single country deployment

---

### Multi-Pod Deployment (Denmark, Norway, Sweden)

**Option A: Shared Database (Development)**

**Single Database, Multiple Schemas:**
```sql
CREATE SCHEMA IF NOT EXISTS global;
CREATE SCHEMA IF NOT EXISTS territory_dk;
CREATE SCHEMA IF NOT EXISTS territory_no;
CREATE SCHEMA IF NOT EXISTS territory_se;
```

**Command:**
```bash
# Create all schemas in one database
sqlx migrate run --database-url "postgresql://unityplan:password@localhost:5432/unityplan_multi"
```

**Result:**
- One database with 4 schemas
- Easy development/testing
- Not production-ready (single point of failure)

---

**Option B: Separate Databases (Production)**

**Denmark Pod:**
```bash
sqlx migrate run --database-url "postgresql://unityplan:password@denmark-db:5432/unityplan_dk"
```

**Norway Pod:**
```bash
sqlx migrate run --database-url "postgresql://unityplan:password@norway-db:5432/unityplan_no"
```

**Sweden Pod:**
```bash
sqlx migrate run --database-url "postgresql://unityplan:password@sweden-db:5432/unityplan_se"
```

**Result:**
- Each pod has its own database server
- Full data sovereignty
- Geographic distribution
- Production-ready

---

### Global Schema Replication

**Problem:** Global schema must be consistent across all pods.

**Solutions:**

**Option 1: PostgreSQL Logical Replication**
```sql
-- Primary pod (Denmark)
CREATE PUBLICATION global_data FOR SCHEMA global;

-- Replica pods (Norway, Sweden)
CREATE SUBSCRIPTION global_data_sub
CONNECTION 'host=denmark-db port=5432 dbname=unityplan_dk'
PUBLICATION global_data;
```

**Option 2: Application-Level Sync**
- Global inserts broadcast via NATS
- All pods update local global schema
- Eventually consistent

**Option 3: Shared Global Database**
- All pods connect to centralized global schema
- Territory schemas remain local
- Hybrid approach

**Recommendation:** Start with Option 3 (simplest), migrate to Option 2 (decentralized) for production.

---

## Cross-Schema Foreign Keys

### Problem

PostgreSQL foreign keys don't work across databases:

```sql
-- This FAILS in multi-database setup:
CREATE TABLE territory_dk.user_connections (
    follower_id UUID REFERENCES territory_dk.users(id),  -- ✅ Same schema
    following_id UUID REFERENCES territory_no.users(id)  -- ❌ Different database
);
```

### Solution: Application-Level Integrity

**Pattern:**
```rust
// Check foreign key in application code
async fn follow_user(follower_id: Uuid, following_id: Uuid, following_territory: &str) -> Result<()> {
    // 1. Validate following_id exists in their territory
    let user_exists = call_api(
        &format!("https://{}.unityplan.org/api/v1/users/{}", following_territory, following_id)
    ).await?;
    
    if !user_exists {
        return Err(Error::UserNotFound);
    }
    
    // 2. Create connection in follower's territory
    sqlx::query!(
        "INSERT INTO territory_dk.user_connections (follower_id, following_id, connection_type)
         VALUES ($1, $2, 'follow')",
        follower_id, following_id
    ).execute(&pool).await?;
    
    Ok(())
}
```

---

## Migration Rollback Strategy

### Down Migrations

**Core Schema:**
```sql
-- 20251111000001_mvp_core_schema.down.sql
DROP SCHEMA IF EXISTS territory_dk CASCADE;
DROP SCHEMA IF EXISTS global CASCADE;
```

**Individual Tables:**
```sql
-- 20251112000004_create_data_exports_table.down.sql
DROP TABLE IF EXISTS territory_dk.data_exports;
```

**Command:**
```bash
sqlx migrate revert --database-url "postgresql://..."
```

---

## Testing Strategy

### Local Development

**Docker Compose:**
```yaml
services:
  postgres:
    image: timescale/timescaledb:latest-pg15
    environment:
      POSTGRES_DB: unityplan_dk
      POSTGRES_USER: unityplan
      POSTGRES_PASSWORD: dev_password
    volumes:
      - ./services/shared-lib/migrations:/migrations
```

**Run Migrations:**
```bash
docker-compose up -d postgres
cd services/shared-lib
sqlx migrate run
```

---

### Integration Tests

**Test Multi-Pod Registration:**
```rust
#[tokio::test]
async fn test_cross_pod_username_uniqueness() {
    // Denmark pod
    let dk_pool = PgPool::connect("postgresql://localhost/unityplan_dk").await.unwrap();
    register_user(&dk_pool, "alice", "dk").await.unwrap();
    
    // Norway pod
    let no_pool = PgPool::connect("postgresql://localhost/unityplan_no").await.unwrap();
    let result = register_user(&no_pool, "alice", "no").await;
    
    // Should fail - username exists in global registry
    assert!(result.is_err());
}
```

---

## Performance Optimizations

### Indexes

**Global Schema:**
```sql
-- Fast username lookups
CREATE INDEX idx_username_registry_username ON global.username_registry(username);  -- PRIMARY KEY
CREATE INDEX idx_username_registry_territory ON global.username_registry(territory_code);

-- Fast email lookups
CREATE INDEX idx_email_registry_email ON global.email_registry(email);  -- PRIMARY KEY
```

**Territory Schema:**
```sql
-- Fast user queries
CREATE INDEX idx_users_username ON territory_dk.users(username);
CREATE INDEX idx_users_email ON territory_dk.users(email) WHERE email IS NOT NULL;

-- Fast profile queries
CREATE INDEX idx_users_profiles_search ON territory_dk.users_profiles USING GIN(
    to_tsvector('english', COALESCE(display_name, '') || ' ' || COALESCE(bio, ''))
);

-- Fast connection queries
CREATE INDEX idx_user_connections_follower ON territory_dk.user_connections(follower_id, connection_type);
CREATE INDEX idx_user_connections_following ON territory_dk.user_connections(following_id, connection_type);
```

---

### Partitioning (Future)

**Time-Series Tables:**
```sql
-- Partition notifications by month
CREATE TABLE territory_dk.notifications (
    ...
) PARTITION BY RANGE (created_at);

CREATE TABLE territory_dk.notifications_2025_11 PARTITION OF territory_dk.notifications
FOR VALUES FROM ('2025-11-01') TO ('2025-12-01');

CREATE TABLE territory_dk.notifications_2025_12 PARTITION OF territory_dk.notifications
FOR VALUES FROM ('2025-12-01') TO ('2026-01-01');
```

**Benefits:**
- Faster queries (scan only relevant partitions)
- Easier maintenance (drop old partitions)
- Better performance for large datasets

---

## Holochain Migration Path

### Current (PostgreSQL)

```
Global Schema → Shared across pods (uniqueness)
Territory Schema → Per-pod data (sovereignty)
```

### Future (Holochain DHT)

```
Global DHT → Distributed hash table (username/email registry)
Source Chain → User's personal data (profiles, settings)
DHT Entries → Shared data (courses, translations)
```

### Migration Strategy

**Phase 1:** PostgreSQL (current)  
**Phase 2:** Hybrid (PostgreSQL + Holochain)
- Keep global registries in PostgreSQL
- Move user data to Holochain source chains
- Use Holochain for course content, forums

**Phase 3:** Full Holochain
- Global DHT replaces global schema
- Source chains replace territory schemas
- PostgreSQL only for caching/search indexes

---

## Security Considerations

### Password Hashing

**Algorithm:** Argon2id  
**Storage:** Only in territory schema (`users.password_hash`)  
**Never:** In global schema or logs

### Sensitive Data

**Personal Data (GDPR):**
- Stored in territory schema only
- User owns their pod's data
- Right to erasure (soft delete → hard delete after 30 days)
- Right to portability (data export to JSON)

**Global Schema:**
- Only metadata (usernames, emails, territories)
- No sensitive content
- Publicly discoverable information

---

## Checklist

### Existing Migrations

- [x] Global schema (territories, registries)
- [x] Territory schema (30 tables)
- [x] Users & authentication
- [x] User profiles & connections
- [x] Settings & notifications
- [x] Communities & members
- [x] Badges & awards
- [x] Invitations
- [x] Events & RSVPs
- [x] File uploads
- [x] GDPR (data exports, deletion)

### Future Migrations

- [ ] Email verification tokens
- [ ] Password reset tokens
- [ ] Login attempts (rate limiting)
- [ ] Global badge definitions
- [ ] Course catalog (global)
- [ ] Forum rooms (global)
- [ ] Translation resources (global)
- [ ] Storage quotas
- [ ] Improved analytics

---

**Next Steps:**
1. Review this master plan
2. Create service-specific MIGRATIONS.md files
3. Test multi-pod deployment locally
4. Document federation patterns
5. Plan Holochain migration

**References:**
- [Database Schema Design](../database-schema-design.md)
- [Multi-Pod Architecture](../multi-pod-architecture.md)
- [User Data Sovereignty](../user-data-sovereignty.md)

---

**Last Updated:** November 12, 2025  
**Maintainer:** Platform Team  
**Status:** Active Development (Phase 1)
