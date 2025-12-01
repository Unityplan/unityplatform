# Database Migrations Master Plan

**Last Updated:** November 18, 2025  
**Current Version:** 20251118000009  
**Database:** PostgreSQL 15+ with TimescaleDB  
**Migration Strategy:** Service-specific migrations with dependency management  
**Naming Convention:** Service-prefixed table names for clear ownership

---

## Overview

This document defines the database migration strategy for the Unity Platform's microservices architecture with multi-pod deployment support.

**Architecture Context:**

- **All services within a pod share ONE PostgreSQL database**
- **Two schemas:** `global` (cross-pod data) and `territory_{code}` (pod-specific data)
- **Services independently deployable** (Docker containers)
- **Database connection is shared infrastructure** (like NATS, Redis)
- **Consistent naming convention** for easy navigation and clear ownership

**Key Principles:**

1. **Service Independence** - Each service owns its migrations and tables
2. **Data Sovereignty** - Personal data stays in territory pods
3. **Global Uniqueness** - Usernames/emails unique across all pods
4. **Clear Naming** - Service-prefixed table names (e.g., `auth_users_core`, `user_users_profiles`)
5. **No Foreign Keys Between Services** - Maintains service boundaries despite shared database
6. **JWT-Based Authentication** - Services validate JWTs locally without database queries
7. **Holochain Ready** - Migration path to decentralized storage

---

## Table Naming Convention

### **Global Schema Tables**

**Pattern:** `registry_{resource}`

**Purpose:** Shared catalogs and cross-territory uniqueness constraints

**Examples:**

- `registry_badge` - Badge catalog shared across all territories
- `registry_email` - Email uniqueness enforcement
- `registry_invitation` - Invitation token uniqueness enforcement
- `registry_territories` - Territory registry
- `registry_username` - Username uniqueness enforcement

**Benefits:**

- Tables group together alphabetically in database tools
- Clear indication of global vs. territory-specific data
- Easy to identify registry/catalog tables

### **Territory Schema Tables**

**Pattern:** `{service}_{entity}_{data}`

**Structure:**

- **{service}** - Service name (auth, user, badge, territory, etc.)
- **{entity}** - Primary entity type (users, badges, territories, etc.)
- **{data}** - Specific data type (core, settings, profiles, etc.)

**Examples:**

**Auth Service:**

- `auth_users_core` - Core authentication data
- `auth_users_refresh_tokens` - JWT refresh tokens

**User Service:**

- `user_users_profiles` - Extended user profiles
- `user_users_settings` - User preferences
- `user_users_profile_language_proficiency` - Language skills
- `user_users_profile_links` - External profile links
- `user_users_connections` - Social connections
- `user_users_data_exports` - GDPR data exports
- `user_users_account_deletion_requests` - GDPR deletion requests

**Badge Service:**

- `badge_users_badges` - Awarded badges
- `badge_users_progress` - Progress tracking

**Territory Service:**

- `territory_territories_settings` - Territory configuration
- `territory_territories_managers` - Manager assignments
- `territory_territories_stats` - Aggregated statistics

**Invitation Service:**

- `invitation_invitations_tokens` - Invitation token storage
- `invitation_invitations_uses` - Usage tracking

**Benefits:**

- **Instant ownership identification** - See service name first
- **Alphabetical grouping** - Tables from same service appear together
- **Clear scope** - Understand entity and data type at a glance
- **Self-documenting** - No need to reference external docs to understand purpose
- **Future-proof** - Easy to add new services following same pattern

---

## Authentication & Validation Strategy

### **Shared Database, Independent Services**

All services within a pod connect to the **same PostgreSQL database**, but maintain service independence through architectural patterns:

```
Denmark Pod (Single Database):
┌─────────────────────────────────────┐
│ PostgreSQL: unityplatform_dk            │
├─────────────────────────────────────┤
│ global.territories                  │
│ global.username_registry            │
│ global.email_registry               │
├─────────────────────────────────────┤
│ territory_dk.auth_users_core ← auth │
│ territory_dk.user_users_profiles ← user │
│ territory_dk.community_communities ← comm │
└─────────────────────────────────────┘
     ↑      ↑      ↑      ↑
     │      │      │      │
  auth-   user-  sett-  comm-
  service service ings   service
```

### **JWT Validation (No Database Queries)**

**99% of requests use JWT validation only:**

```rust
// All services use shared JWT middleware
use shared_lib::middleware::jwt_auth_middleware;

HttpServer::new(|| {
    App::new()
        .wrap(jwt_auth_middleware)  // Validates JWT signature (~0.01ms)
        .service(my_handler)         // No database query needed!
})
```

**Performance:** ~0.01ms (cryptographic validation only, zero database load)

### **Optional Database Check (Critical Operations)**

**1% of requests need real-time user status:**

```rust
// Critical operations (account deletion, security changes)
async fn delete_account(auth: AuthUser, pool: &PgPool) -> Result<()> {
    // Check user still exists and active
    let user = sqlx::query!(
        "SELECT deleted_at FROM territory_dk.users WHERE id = $1",
        auth.id
    ).fetch_optional(pool).await?;
    
    match user {
        Some(u) if u.deleted_at.is_none() => {
            // User active - proceed
        },
        _ => return Err(Error::UserNotFound)
    }
    
    // Delete account...
}
```

**Performance:** ~0.5-2ms (database query), acceptable for critical operations

### **Why No Foreign Keys Between Services?**

Despite sharing a database, services avoid foreign keys for:

1. **Service Independence** - Can deploy/test services separately
2. **Schema Evolution** - Change auth-service schema without breaking user-service
3. **Multi-Pod Future** - Cross-pod user relationships can't use FKs (different databases)
4. **Holochain Migration** - Aligns with decentralized architecture

**Example:**

```sql
-- ❌ Don't do this (tight coupling)
CREATE TABLE territory_dk.users_profiles (
    user_id UUID REFERENCES territory_dk.users(id)  -- FK constraint
);

-- ✅ Do this (service independence)
CREATE TABLE territory_dk.users_profiles (
    user_id UUID PRIMARY KEY  -- No FK
);
COMMENT ON COLUMN users_profiles.user_id IS 
    'References territory_dk.users(id) - validated via JWT middleware. 
     No FK constraint for service independence.';
```

**Validation happens in code, not database:**

```rust
// JWT middleware already validated user exists at login
// No need for FK constraint
async fn update_profile(auth: AuthUser, data: ProfileData) -> Result<()> {
    // auth.id already validated by JWT
    sqlx::query!("UPDATE users_profiles SET ... WHERE user_id = $1", auth.id)
        .execute(&pool).await
}
```

**See [shared-lib/AUTHENTICATION.md](services/shared-lib/AUTHENTICATION.md) for complete authentication architecture.**

---

## Migration Structure

### Current Migration Files (November 15, 2025)

**Location:** `services/shared-lib/migrations/`

```
services/shared-lib/migrations/
├── 20251112000001_create_global_schema.sql              # ✅ Global schema + registries
├── 20251112000002_create_territory_schema_template.sql  # ✅ Territory schema
├── 20251112000003_auth_core_tables.sql                  # ✅ Auth service tables
├── 20251113000004_user_service_tables.sql               # ✅ User service tables
├── 20251113000005_territory_service_tables.sql          # ✅ Territory service tables
├── 20251113000006_badge_service_tables.sql              # ✅ Badge service tables
├── 20251113000007_create_users_settings_table.sql       # ✅ User settings table
├── 20251116000008_create_global_language_registry.sql   # ✅ Language registry
└── 20251118000009_invitation_service_tables.sql         # ✅ Invitation service tables
```

**Archived Migrations (Pre-Naming Convention):**

- **Location:** `services/shared-lib/migrations-archive/`
- **Purpose:** Reference for pre-November 15, 2025 table names
- **Status:** Superseded by current migrations with new naming convention

### Migration Naming Format

**Format:** `YYYYMMDDHHMMSS_descriptive_name.sql`

**Examples:**

- `20251112000001_create_global_schema.sql`
- `20251113000004_user_service_tables.sql`

**Ordering:**

- Migrations run in chronological order by filename
- Dependencies enforced through ordering
- Each migration includes dependency comments

---

## Current Database Schema (November 15, 2025)

### Global Schema (5 tables)

**Purpose:** Cross-territory shared data and uniqueness enforcement

| Table | Columns | Purpose | Owner |
|-------|---------|---------|-------|
| `registry_badge` | 15 | Global badge catalog across all territories | badge-service |
| `registry_email` | 5 | Email uniqueness enforcement | auth-service |
| `registry_invitation` | 4 | Invitation token uniqueness enforcement | invitation-service |
| `registry_territories` | 12 | Territory/pod registry | territory-service |
| `registry_username` | 4 | Username uniqueness enforcement | auth-service |

### Territory Schema (16 tables)

**Purpose:** Territory-specific user data (data sovereignty)

**Auth Service (2 tables):**

| Table | Columns | Purpose |
|-------|---------|---------|
| `auth_users_core` | 11 | User authentication credentials |
| `auth_users_refresh_tokens` | 9 | JWT refresh token management |

**User Service (7 tables):**

| Table | Columns | Purpose |
|-------|---------|---------|
| `user_users_profiles` | 11 | Extended user profiles (bio, avatar, interests, skills) |
| `user_users_settings` | 17 | User preferences, privacy, notifications |
| `user_users_profile_language_proficiency` | 13 | Language skills (4 dimensions: spoken/written/reading/listening) |
| `user_users_profile_links` | 9 | External profile links (max 10 per user) |
| `user_users_connections` | 6 | Social connections (follow/block) |
| `user_users_data_exports` | 11 | GDPR Article 20: Data portability (7-day expiration) |
| `user_users_account_deletion_requests` | 12 | GDPR Article 17: Right to erasure (30-day grace period) |

**Badge Service (2 tables):**

| Table | Columns | Purpose |
|-------|---------|---------|
| `badge_users_badges` | 12 | Badges awarded to users (with renewal tracking) |
| `badge_users_progress` | 6 | Progress towards earning achievement badges |

**Territory Service (3 tables):**

| Table | Columns | Purpose |
|-------|---------|---------|
| `territory_territories_settings` | 14 | Territory configuration (sovereignty source of truth) |
| `territory_territories_managers` | 6 | Manager assignments (requires badge + entry) |
| `territory_territories_stats` | 8 | Aggregated statistics (read-only, calculated) |

**Invitation Service (2 tables):**

| Table | Columns | Purpose |
|-------|---------|---------|
| `invitation_invitations_tokens` | 12 | Invitation token storage (manager-only creation) |
| `invitation_invitations_uses` | 5 | Invitation usage tracking (trust graph) |

**Total:** 21 tables (5 global + 16 territory-specific)

---

## Migration Execution Guide

### Running Migrations

**Automated Script (Recommended):**

```bash
# Run all migrations in order
cd /path/to/workspace
bash scripts/db/setup-database.sh
```

**Manual Execution:**

```bash
# Run individual migration
docker exec service-postgres-dk psql -U unityplatform -d unityplatform_dk -f /path/to/migration.sql

# Or from host
psql -h localhost -p 5432 -U unityplatform -d unityplatform_dk -f migration.sql
```

### Creating New Migrations

**Naming Convention:**

```
YYYYMMDDHHMMSS_descriptive_name.sql
```

**Template:**

```sql
-- ============================================================================
-- Migration: YYYYMMDDHHMMSS_descriptive_name.sql
-- Level: N (Foundation/Core/Features)
-- Service: service-name
-- Description: Brief description
-- Dependencies: Previous migration filename
-- ============================================================================

-- Your SQL here

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration YYYYMMDDHHMMSS complete: Description';
    RAISE NOTICE '    - Created table_name_1';
    RAISE NOTICE '    - Created table_name_2';
END $$;
```

**Remember:**

- Follow naming convention: `{service}_{entity}_{data}` for territory tables
- Follow naming convention: `registry_{resource}` for global tables
- Include dependency comments
- Add table/column comments for documentation
- Test migration on clean database before committing

---
├── 20251112000004_create_data_exports_table.sql          # ✅ GDPR exports
├── 20251112000005_create_account_deletion_requests_table.sql # ✅ GDPR deletion
└── archived/                                             # Old migrations

```

### New Service-Specific Migrations (Recommended)

```

services/shared-lib/migrations/service-migrations/
├── README.md                          # Migration execution guide
├── 00-territory-service/              # Level 0: Foundation (run first)
│   └── 001_create_territories.sql
├── 01-auth-service/                   # Level 1: Core Identity (run second)
│   ├── 001_create_global_registries.sql
│   └── 002_create_users_tables.sql
├── 02-user-service/                   # Level 2: User Data (parallel)
│   ├── 001_create_profiles.sql
│   ├── 002_create_profile_links.sql
│   ├── 003_create_language_proficiency.sql
│   ├── 004_create_connections.sql
│   ├── 005_create_gdpr_tables.sql
│   └── 006_create_audit_tables.sql
├── 02-settings-service/               # Level 2: Settings (parallel)
│   └── 001_create_settings_tables.sql
├── 02-invitation-service/             # Level 2: Invitations (parallel)
│   ├── 001_create_global_registry.sql
│   └── 002_create_invitation_tables.sql
├── 02-notification-service/           # Level 2: Notifications (parallel)
│   └── 001_create_notifications.sql
├── 03-community-service/              # Level 3: Communities (after Level 2)
│   └── [future migrations]
└── 04-{feature}-service/              # Level 4: Features (parallel)

```

**See:** `services/shared-lib/migrations/service-migrations/README.md` for complete execution guide

---

## Migration Execution Order

### Dependencies Graph

```

Level 0 (Foundation):
  └─> territory-service (global.territories)

Level 1 (Core Identity):
  └─> auth-service (depends on: global.territories)
      ├─> global.username_registry
      ├─> global.email_registry  
      └─> territory_{code}.users

Level 2 (User Data - can run in parallel):
  ├─> user-service (depends on: territory_{code}.users)
  ├─> settings-service (depends on: territory_{code}.users)
  ├─> invitation-service (depends on: territory_{code}.users)
  └─> notification-service (depends on: territory_{code}.users)

Level 3 (Communities):
  └─> community-service (depends on: territory_{code}.users)

Level 4 (Features - can run in parallel):
  ├─> badge-service (depends on: users, communities)
  ├─> event-service (depends on: users, communities)
  ├─> course-service (depends on: users, communities)
  └─> forum-service (depends on: users, communities)

```

### Execution Commands

```bash
# Quick start: Use the migration script
./services/shared-lib/migrations/service-migrations/execute_migrations.sh dk

# Or run manually in order:

# Level 0: Foundation
psql -d unityplatform -f service-migrations/00-territory-service/001_create_territories.sql

# Level 1: Core Identity  
psql -d unityplatform -f service-migrations/01-auth-service/001_create_global_registries.sql
psql -d unityplatform -v territory_code=dk -f service-migrations/01-auth-service/002_create_users_tables.sql

# Level 2: User Data (can run in parallel)
psql -d unityplatform -v territory_code=dk -f service-migrations/02-user-service/001_create_profiles.sql &
psql -d unityplatform -v territory_code=dk -f service-migrations/02-settings-service/001_create_settings_tables.sql &
psql -d unityplatform -v territory_code=dk -f service-migrations/02-invitation-service/001_create_global_registry.sql &
psql -d unityplatform -v territory_code=dk -f service-migrations/02-notification-service/001_create_notifications.sql &
wait  # Wait for all to complete
```

---

## Inter-Service Dependencies

### Foreign Keys vs API Calls

**Microservices Principle:** Services should not have database-level dependencies on each other.

**Strategy:** Remove foreign key constraints between services, use API validation instead.

| Dependency Type | Old Approach (Monolith) | New Approach (Microservices) |
|----------------|------------------------|------------------------------|
| **User exists** | `FOREIGN KEY (user_id) REFERENCES users(id)` | Call auth-service API to validate |
| **Community exists** | `FOREIGN KEY (community_id) REFERENCES communities(id)` | Call community-service API to validate |
| **Territory exists** | `FOREIGN KEY (territory_code) REFERENCES territories(code)` | OK - global.territories is shared |

### Example: Removing FK Constraint

**Before (Monolithic):**

```sql
CREATE TABLE territory_dk.users_profiles (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    display_name VARCHAR(100),
    ...
);
```

**After (Microservices):**

```sql
CREATE TABLE territory_dk.users_profiles (
    user_id UUID PRIMARY KEY,  -- NO FOREIGN KEY CONSTRAINT
    display_name VARCHAR(100),
    ...
);

COMMENT ON COLUMN territory_dk.users_profiles.user_id IS 
    'User ID validated via auth-service API before insert. No FK constraint for service independence.';
```

**Validation in Code:**

```rust
// In user-service
async fn create_profile(user_id: Uuid, data: ProfileData) -> Result<Profile> {
    // Validate user exists via API call to auth-service
    let user = auth_client.verify_user_exists(user_id).await?;
    
    // Only create profile if user exists
    create_profile_in_db(user_id, data).await
}
```

### Eventual Consistency via NATS

**Pattern:** Use NATS events for data synchronization between services.

**Example: User Deletion**

```rust
// auth-service publishes event when user deleted
nats.publish("user.deleted", UserDeletedEvent { user_id, territory_code }).await;

// user-service subscribes and cleans up related data
nats.subscribe("user.deleted", |event| {
    delete_user_profiles(event.user_id).await;
    delete_user_connections(event.user_id).await;
}).await;

// settings-service subscribes and cleans up
nats.subscribe("user.deleted", |event| {
    delete_user_settings(event.user_id).await;
}).await;
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
| `global.badge_registry` | badge-service | Shared badge catalog (future) |
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
| **auth-service** | auth_users_core, auth_refresh_tokens | Authentication |
| **user-service** | user_users_profiles, user_users_profile_links, user_users_language_proficiency, user_user_connections, user_data_exports, user_account_deletion_requests, user_file_uploads, user_activities, user_audit_log | User data + GDPR |
| **settings-service** | settings_users_settings, settings_users_notification_settings | User preferences |
| **invitation-service** | invitation_invitations_tokens, invitation_invitations_uses | Territory invitations |
| **notification-service** | notification_notifications | User notifications |
| **community-service** | community_communities, community_communities_members, community_communities_managers, community_communities_settings, community_communities_badge_requirements | Territory communities |
| **badge-service** | badge_users_badges, badge_badge_progress | User achievements |
| **event-service** | event_community_events, event_event_rsvps | Territory events |
| **course-service** | course_enrollments, course_progress | User course progress |
| **forum-service** | forum_memberships | User forum participation |
| **ipfs-service** | ipfs_file_uploads | File metadata |

**Total:** Territory-scoped tables with service prefixes (naming convention: `{service}_{entity}_{data}`)

---

## Service-Specific Migration Details

### Level 0: territory-service (Foundation)

**Migration Directory:** `service-migrations/00-territory-service/`

**Tables Created:**

- `global.territories` - Territory/pod registry

**Dependencies:** None (runs first)

**Migrations:**

- `001_create_territories.sql` - Create global territories table

**Details:** See [services/territory-service/DATABASE.md](services/territory-service/DATABASE.md)

---

### Level 1: auth-service (Core Identity)

**Migration Directory:** `service-migrations/01-auth-service/`

**Tables Created:**

- `global.username_registry` - Global username uniqueness
- `global.email_registry` - Global email uniqueness  
- `territory_{code}.users` - User authentication data
- `territory_{code}.refresh_tokens` - JWT refresh tokens

**Dependencies:**

- Requires: `global.territories` (from territory-service)

**Migrations:**

- `001_create_global_registries.sql` - Global username/email registries
- `002_create_users_tables.sql` - Users and refresh tokens (per territory)

**Future Migrations:**

- `003_create_email_verification.sql` - Email verification tokens
- `004_create_password_reset.sql` - Password reset tokens  
- `005_create_login_attempts.sql` - Rate limiting table

**Details:** See [services/auth-service/MIGRATIONS.md](services/auth-service/MIGRATIONS.md)

---

### Level 2: user-service (User Data)

**Migration Directory:** `service-migrations/02-user-service/`

**Tables Created:**

- `territory_{code}.users_profiles` - User profile data
- `territory_{code}.users_profile_links` - Social media links
- `territory_{code}.users_language_proficiency` - Language skills
- `territory_{code}.user_connections` - Friend connections
- `territory_{code}.data_exports` - GDPR data exports (Article 20)
- `territory_{code}.account_deletion_requests` - GDPR deletion requests (Article 17)
- `territory_{code}.file_uploads` - File metadata (IPFS CIDs)
- `territory_{code}.activities` - User activity log
- `territory_{code}.audit_log` - Audit trail

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- **Validation:** API calls to auth-service (NO foreign keys)

**Migrations:**

- `001_create_profiles.sql` - User profiles table
- `002_create_profile_links.sql` - Social media links
- `003_create_language_proficiency.sql` - Language skills
- `004_create_connections.sql` - User connections
- `005_create_gdpr_tables.sql` - Data exports and deletion requests
- `006_create_audit_tables.sql` - Activity and audit logs

**Details:** See [services/user-service/DATABASE.md](services/user-service/DATABASE.md)

---

### Level 2: settings-service (User Settings)

**Migration Directory:** `service-migrations/02-settings-service/`

**Tables Created:**

- `territory_{code}.users_settings` - User preferences
- `territory_{code}.users_notification_settings` - Notification preferences

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- **Validation:** API calls to auth-service (NO foreign keys)

**Migrations:**

- `001_create_settings_tables.sql` - Both settings tables

**Details:** See [services/settings-service/DATABASE.md](services/settings-service/DATABASE.md)

---

### Level 2: invitation-service (Invitations)

**Migration Directory:** `service-migrations/02-invitation-service/`

**Tables Created:**

- `global.invitation_token_registry` - Global token uniqueness
- `territory_{code}.invitation_tokens` - Territory invitation tokens
- `territory_{code}.invitation_uses` - Token usage tracking

**Dependencies:**

- Requires: `global.territories` (from territory-service)
- Requires: `territory_{code}.users` (from auth-service)
- **Validation:** API calls to auth-service (NO foreign keys)

**Migrations:**

- `001_create_global_registry.sql` - Global invitation token registry
- `002_create_invitation_tables.sql` - Territory invitation tables

**Details:** See [services/invitation-service/DATABASE.md](services/invitation-service/DATABASE.md)

---

### Level 2: notification-service (Notifications)

**Migration Directory:** `service-migrations/02-notification-service/`

**Tables Created:**

- `territory_{code}.notifications` - User notifications

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- **Validation:** NATS events (NO foreign keys, NO API calls)

**Migrations:**

- `001_create_notifications.sql` - Notifications table

**Rationale:** Event-driven, loosely coupled via NATS

**Details:** See [services/notification-service/DATABASE.md](services/notification-service/DATABASE.md)

---

### Level 3: community-service (Communities)

**Migration Directory:** `service-migrations/03-community-service/`

**Tables Created:**

- `territory_{code}.communities` - Community definitions
- `territory_{code}.community_members` - Membership records
- `territory_{code}.roles` - Community roles
- `territory_{code}.role_assignments` - Role assignments
- `territory_{code}.community_role_elections` - Democratic elections
- `territory_{code}.community_role_election_votes` - Election votes

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- **Validation:** API calls to auth-service (NO foreign keys)

**Migrations:**

- `001_create_communities.sql` - Communities table
- `002_create_members.sql` - Community members
- `003_create_roles.sql` - Roles and assignments
- `004_create_elections.sql` - Democratic elections

**Priority:** Phase 2 (not MVP)

**Details:** See [services/community-service/DATABASE.md](services/community-service/DATABASE.md)

---

### Level 4: Feature Services (Phase 2)

#### badge-service

**Tables Created:**

- `global.badge_registry` - Shared badge catalog (optional)
- `territory_{code}.badge_awards` - User badge awards
- `territory_{code}.badge_progress` - Badge progress tracking

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- Requires: `territory_{code}.communities` (from community-service)

**Priority:** Phase 2

---

#### event-service

**Tables Created:**

- `territory_{code}.community_events` - Community events
- `territory_{code}.event_rsvps` - Event RSVPs

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- Requires: `territory_{code}.communities` (from community-service)

**Priority:** Phase 2

---

#### course-service

**Tables Created:**

- `global.courses` - Shared course catalog (optional)
- `global.course_lessons` - Shared course content (optional)
- `territory_{code}.course_enrollments` - User enrollments
- `territory_{code}.course_progress` - User progress

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- Requires: `territory_{code}.badge_awards` (from badge-service) - prerequisites

**Priority:** Phase 2

---

#### forum-service

**Tables Created:**

- `global.forum_rooms` - Global forum registry (optional)
- `territory_{code}.forum_memberships` - User forum memberships

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- Requires: `territory_{code}.communities` (from community-service)

**Priority:** Phase 2

---

#### translation-service

**Tables Created:**

- `global.translation_resources` - Shared translation strings
- `territory_{code}.translation_cache` - Translation cache

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)

**Priority:** Phase 2

---

## Migration Execution

### Option 1: Automated Script (Recommended)

```bash
# Execute all migrations for a territory in correct order
cd services/shared-lib/migrations/service-migrations
./execute_migrations.sh dk  # For Denmark territory
./execute_migrations.sh no  # For Norway territory
```

### Option 2: Manual Execution

```bash
# Set variables
export DB_NAME="unityplatform"
export TERRITORY_CODE="dk"

# Level 0: Foundation
psql -d $DB_NAME -f 00-territory-service/001_create_territories.sql

# Level 1: Core Identity
psql -d $DB_NAME -f 01-auth-service/001_create_global_registries.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 01-auth-service/002_create_users_tables.sql

# Level 2: User Data (can run in parallel)
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-user-service/001_create_profiles.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-user-service/002_create_profile_links.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-user-service/003_create_language_proficiency.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-user-service/004_create_connections.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-user-service/005_create_gdpr_tables.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-user-service/006_create_audit_tables.sql

psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-settings-service/001_create_settings_tables.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-invitation-service/001_create_global_registry.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-invitation-service/002_create_invitation_tables.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-notification-service/001_create_notifications.sql

# Level 3+: Future migrations
# ...
```

### Option 3: Docker Compose Integration

```yaml
# docker-compose.yml
services:
  postgres:
    image: timescale/timescaledb:latest-pg15
    volumes:
      - ./services/shared-lib/migrations/service-migrations:/migrations
    environment:
      - TERRITORY_CODE=dk
  
  migration-runner:
    image: postgres:15
    depends_on:
      - postgres
    volumes:
      - ./services/shared-lib/migrations/service-migrations:/migrations
    command: /migrations/execute_migrations.sh dk
```

---

## Testing Migrations

### Test Individual Service Migrations

```bash
# Create test database
createdb unityplatform_test

# Run specific service migrations
psql -d unityplatform_test -f 00-territory-service/001_create_territories.sql
psql -d unityplatform_test -f 01-auth-service/001_create_global_registries.sql

# Verify tables created
psql -d unityplatform_test -c "\dt global.*"
psql -d unityplatform_test -c "\dt territory_dk.*"

# Drop test database
dropdb unityplatform_test
```

### Test Migration Dependencies

```bash
# Try running Level 2 without Level 1 (should fail gracefully)
createdb unityplatform_test
psql -d unityplatform_test -f 02-user-service/001_create_profiles.sql
# Expected: ERROR - territory_dk schema does not exist

# Run in correct order
psql -d unityplatform_test -f 00-territory-service/001_create_territories.sql
psql -d unityplatform_test -f 01-auth-service/001_create_global_registries.sql
psql -d unityplatform_test -v territory_code=dk -f 01-auth-service/002_create_users_tables.sql
psql -d unityplatform_test -v territory_code=dk -f 02-user-service/001_create_profiles.sql
# Expected: SUCCESS
```

---

## Rollback Strategy

Each migration should have a corresponding `.down.sql` file for rollback.

**Example:**

```
02-user-service/
├── 001_create_profiles.sql        # Up migration
├── 001_create_profiles.down.sql   # Down migration (rollback)
├── 002_create_profile_links.sql
└── 002_create_profile_links.down.sql
```

**Rollback command:**

```bash
psql -d unityplatform -f 02-user-service/001_create_profiles.down.sql
```

---

## Migration Best Practices

### 1. Idempotent Migrations

Always use `IF NOT EXISTS` for safety:

```sql
CREATE SCHEMA IF NOT EXISTS global;
CREATE TABLE IF NOT EXISTS global.territories (...);
CREATE INDEX IF NOT EXISTS idx_territories_status ON global.territories(status);
```

### 2. No Cross-Service Foreign Keys

❌ **Bad:**

```sql
CREATE TABLE territory_dk.users_profiles (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id),  -- BREAKS SERVICE INDEPENDENCE
    ...
);
```

✅ **Good:**

```sql
CREATE TABLE territory_dk.users_profiles (
    user_id UUID PRIMARY KEY,  -- NO FK - validated via API
    ...
);

COMMENT ON COLUMN territory_dk.users_profiles.user_id IS 
    'Validated via auth-service API. No FK for service independence.';
```

### 3. Document Dependencies

Always add comments explaining validation:

```sql
COMMENT ON TABLE territory_dk.users_profiles IS 
    'User profiles. user_id validated via auth-service API before insert.';
```

### 4. Use Variables for Territory Code

```sql
-- Enable psql variables
CREATE SCHEMA IF NOT EXISTS territory_:territory_code;

CREATE TABLE territory_:territory_code.users (
    ...
);
```

**Execute with:**

```bash
psql -d unityplatform -v territory_code=dk -f migration.sql
```

---

## Transition from Monolithic Migrations

### Current State (Monolithic)

All tables created in single migration: `20251111000001_mvp_core_schema.sql`

### Future State (Service-Specific)

Tables split across service-owned migrations in `service-migrations/`

### Migration Path

1. ✅ Keep existing monolithic migrations (already applied)
2. ✅ Create service-specific migrations (in progress)
3. 📋 For new environments: Use service-specific migrations
4. 📋 For existing environments: Continue with monolithic migrations
5. 📋 Eventually: Remove foreign key constraints between services
6. 📋 Add API validation logic in service code

**No Breaking Changes:** Both migration strategies will coexist during transition.

---

## Next Steps

1. ✅ Create service-migrations directory structure
2. ✅ Document migration execution order
3. ✅ Define inter-service dependency strategy (API calls, no FKs)
4. 📋 Extract migrations from monolithic file into service-specific files
5. 📋 Create migration execution script
6. 📋 Update service DATABASE.md files with migration references
7. 📋 Add migration tests
8. 📋 Update docker-compose for automated migrations

---

**Last Updated:** November 12, 2025  
**Maintained By:** Development Team  
**Status:** Migration separation in progress

**See Also:**

- [service-migrations/README.md](../../services/shared-lib/migrations/service-migrations/README.md) - Detailed execution guide
- [services/auth-service/MIGRATIONS.md](services/auth-service/MIGRATIONS.md) - Auth service migrations
- [Database Schema Design](.archived/20251111-database-schema-design.md) - Original design (archived)

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
sqlx migrate run --database-url "postgresql://unityplatform:password@localhost:5432/unityplatform_dk"
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
sqlx migrate run --database-url "postgresql://unityplatform:password@localhost:5432/unityplatform_multi"
```

**Result:**

- One database with 4 schemas
- Easy development/testing
- Not production-ready (single point of failure)

---

**Option B: Separate Databases (Production)**

**Denmark Pod:**

```bash
sqlx migrate run --database-url "postgresql://unityplatform:password@denmark-db:5432/unityplatform_dk"
```

**Norway Pod:**

```bash
sqlx migrate run --database-url "postgresql://unityplatform:password@norway-db:5432/unityplatform_no"
```

**Sweden Pod:**

```bash
sqlx migrate run --database-url "postgresql://unityplatform:password@sweden-db:5432/unityplatform_se"
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
CONNECTION 'host=denmark-db port=5432 dbname=unityplatform_dk'
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
        &format!("https://{}.unityplatform.org/api/v1/users/{}", following_territory, following_id)
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
      POSTGRES_DB: unityplatform_dk
      POSTGRES_USER: unityplatform
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
    let dk_pool = PgPool::connect("postgresql://localhost/unityplatform_dk").await.unwrap();
    register_user(&dk_pool, "alice", "dk").await.unwrap();
    
    // Norway pod
    let no_pool = PgPool::connect("postgresql://localhost/unityplatform_no").await.unwrap();
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

- [ ] Email verification tokens → `auth_users_email_verification`
- [ ] Password reset tokens → `auth_users_password_reset`
- [ ] Login attempts (rate limiting) → `auth_users_login_attempts`
- [ ] Global badge definitions → Already created as `registry_badge`
- [ ] Course catalog (global) → `registry_course`
- [ ] Forum rooms (global) → `registry_forum_room`
- [ ] Translation resources (global) → `registry_translation`
- [ ] Storage quotas → `user_users_storage_quotas`
- [ ] Improved analytics → `territory_territories_analytics`

---

## Future Service Migration Guidelines

### Creating New Service Tables

When adding tables for a new service, follow these guidelines:

**1. Determine Schema:**

- **Global Schema** if: Cross-territory data, uniqueness constraint, shared catalog
- **Territory Schema** if: User data, territory-specific, data sovereignty

**2. Apply Naming Convention:**

**Global Schema:**

```sql
-- Pattern: registry_{resource}
CREATE TABLE global.registry_{resource} (...);

-- Examples:
CREATE TABLE global.registry_course (...);       -- Course catalog
CREATE TABLE global.registry_translation (...);  -- Translation resources
CREATE TABLE global.registry_forum_room (...);   -- Forum room templates
```

**Territory Schema:**

```sql
-- Pattern: {service}_{entity}_{data}
CREATE TABLE territory_dk.{service}_{entity}_{data} (...);

-- Examples (Course Service):
CREATE TABLE territory_dk.course_users_enrollments (...);    -- User enrollments
CREATE TABLE territory_dk.course_users_progress (...);       -- Progress tracking
CREATE TABLE territory_dk.course_users_certificates (...);   -- Awarded certificates

-- Examples (Forum Service):
CREATE TABLE territory_dk.forum_topics_posts (...);          -- Forum posts
CREATE TABLE territory_dk.forum_users_subscriptions (...);   -- Topic subscriptions
CREATE TABLE territory_dk.forum_topics_moderation (...);     -- Moderation actions

-- Examples (Event Service):
CREATE TABLE territory_dk.event_events_core (...);           -- Event details
CREATE TABLE territory_dk.event_users_rsvps (...);          -- RSVP tracking
CREATE TABLE territory_dk.event_events_attendance (...);     -- Attendance records
```

**3. Add Comments:**

```sql
COMMENT ON TABLE territory_dk.course_users_enrollments IS 
    'User course enrollments. Tracks which users are enrolled in which courses.';

COMMENT ON COLUMN territory_dk.course_users_enrollments.user_id IS 
    'References territory_dk.auth_users_core(id) - validated via JWT middleware. No FK constraint for service independence.';
```

**4. Create Indexes:**

```sql
-- Always index foreign reference columns
CREATE INDEX idx_course_users_enrollments_user 
    ON territory_dk.course_users_enrollments(user_id);

-- Index commonly queried columns
CREATE INDEX idx_course_users_enrollments_course 
    ON territory_dk.course_users_enrollments(course_id);
```

**5. Add Migration Success Message:**

```sql
DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251120000001 complete: Course service tables created';
    RAISE NOTICE '    - course_users_enrollments';
    RAISE NOTICE '    - course_users_progress';
    RAISE NOTICE '    - course_users_certificates';
END $$;
```

### Benefits of Consistent Naming

✅ **Instant Service Identification** - See which service owns the table at a glance  
✅ **Alphabetical Grouping** - Tables from same service appear together in DB tools  
✅ **Self-Documenting** - Table name describes service, entity, and data type  
✅ **Easy Navigation** - Developers can find tables without consulting docs  
✅ **Future-Proof** - Pattern scales to dozens of services  
✅ **Clear Ownership** - No ambiguity about which service maintains which tables

---

**Last Updated:** November 15, 2025  
**Current Version:** 20251113000007  
**Tables:** 18 (4 global + 14 territory)  
**Maintainer:** Platform Team  
**Status:** Active Development (Phase 1)
