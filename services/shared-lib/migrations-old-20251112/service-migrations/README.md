# Service-Specific Migration Strategy

**Created:** November 12, 2025  
**Status:** Migration separation in progress  
**Purpose:** Split monolithic migrations into service-owned migrations with proper dependency management

---

## Overview

This directory contains service-specific database migrations organized by execution order. Each service owns its tables and can be deployed independently while respecting dependencies.

## Directory Structure

```
service-migrations/
├── README.md (this file)
├── 00-territory-service/        # Level 0: Foundation
│   └── 001_create_territories.sql
├── 01-auth-service/             # Level 1: Core Identity
│   ├── 001_create_global_registries.sql
│   └── 002_create_users_tables.sql
├── 02-user-service/             # Level 2: User Data (parallel)
│   ├── 001_create_profiles.sql
│   ├── 002_create_profile_links.sql
│   ├── 003_create_language_proficiency.sql
│   ├── 004_create_connections.sql
│   ├── 005_create_gdpr_tables.sql
│   └── 006_create_audit_tables.sql
├── 02-settings-service/         # Level 2: Settings (parallel)
│   └── 001_create_settings_tables.sql
├── 02-invitation-service/       # Level 2: Invitations (parallel)
│   ├── 001_create_global_registry.sql
│   └── 002_create_invitation_tables.sql
├── 02-notification-service/     # Level 2: Notifications (parallel)
│   └── 001_create_notifications.sql
├── 03-community-service/        # Level 3: Communities
├── 04-badge-service/            # Level 4: Features
├── 04-event-service/            # Level 4: Features
├── 04-course-service/           # Level 4: Features
└── 04-forum-service/            # Level 4: Features
```

---

## Migration Execution Order

### Level 0: Foundation (MUST run first)

**Service:** territory-service  
**Tables:** `global.territories`  
**Dependencies:** None  
**Rationale:** All services reference territories

```bash
# Run territory-service migrations first
psql -f 00-territory-service/001_create_territories.sql
```

---

### Level 1: Core Identity (MUST run second)

**Service:** auth-service  
**Tables:**

- `global.username_registry`
- `global.email_registry`
- `territory_{code}.users`
- `territory_{code}.refresh_tokens`

**Dependencies:**

- Requires: `global.territories` (from territory-service)

**Rationale:** All other services depend on users existing

```bash
# Run auth-service migrations
psql -f 01-auth-service/001_create_global_registries.sql
psql -f 01-auth-service/002_create_users_tables.sql
```

---

### Level 2: User Data (CAN run in parallel)

These services can run in any order or in parallel since they all depend only on `territory_{code}.users`.

#### user-service

**Tables:**

- `territory_{code}.users_profiles`
- `territory_{code}.users_profile_links`
- `territory_{code}.users_language_proficiency`
- `territory_{code}.user_connections`
- `territory_{code}.data_exports`
- `territory_{code}.account_deletion_requests`
- `territory_{code}.file_uploads`
- `territory_{code}.activities`
- `territory_{code}.audit_log`

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- **NO foreign key constraints** - uses API calls for validation

**Rationale:** User profiles extend user identity but don't block other services

```bash
psql -f 02-user-service/001_create_profiles.sql
psql -f 02-user-service/002_create_profile_links.sql
psql -f 02-user-service/003_create_language_proficiency.sql
psql -f 02-user-service/004_create_connections.sql
psql -f 02-user-service/005_create_gdpr_tables.sql
psql -f 02-user-service/006_create_audit_tables.sql
```

#### settings-service

**Tables:**

- `territory_{code}.users_settings`
- `territory_{code}.users_notification_settings`

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- **NO foreign key constraints** - uses API calls for validation

```bash
psql -f 02-settings-service/001_create_settings_tables.sql
```

#### invitation-service

**Tables:**

- `global.invitation_token_registry`
- `territory_{code}.invitation_tokens`
- `territory_{code}.invitation_uses`

**Dependencies:**

- Requires: `global.territories` (from territory-service)
- Requires: `territory_{code}.users` (from auth-service)
- **NO foreign key constraints** - uses API calls for validation

```bash
psql -f 02-invitation-service/001_create_global_registry.sql
psql -f 02-invitation-service/002_create_invitation_tables.sql
```

#### notification-service

**Tables:**

- `territory_{code}.notifications`

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- **NO foreign key constraints** - uses NATS events

**Rationale:** Notifications are event-driven and loosely coupled

```bash
psql -f 02-notification-service/001_create_notifications.sql
```

---

### Level 3: Communities (MUST run after Level 2)

**Service:** community-service  
**Tables:**

- `territory_{code}.communities`
- `territory_{code}.community_members`
- `territory_{code}.roles`
- `territory_{code}.role_assignments`
- `territory_{code}.community_role_elections`
- `territory_{code}.community_role_election_votes`

**Dependencies:**

- Requires: `territory_{code}.users` (from auth-service)
- **NO foreign key constraints** - uses API calls

**Rationale:** Other features depend on communities existing

```bash
psql -f 03-community-service/001_create_communities.sql
psql -f 03-community-service/002_create_members.sql
psql -f 03-community-service/003_create_roles.sql
psql -f 03-community-service/004_create_elections.sql
```

---

### Level 4: Features (CAN run in parallel after Level 3)

These services can run in any order since they depend on users and communities but not each other.

#### badge-service

#### event-service

#### course-service

#### forum-service

---

## Inter-Service Communication Strategy

### Foreign Keys vs API Calls

**Problem:** Microservices should not have direct database dependencies on each other.

**Solution:** Remove cross-service foreign key constraints and use API calls instead.

#### Example: user-service referencing users

**❌ Old Way (Monolith):**

```sql
CREATE TABLE territory_dk.users_profiles (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    ...
);
```

**✅ New Way (Microservices):**

```sql
CREATE TABLE territory_dk.users_profiles (
    user_id UUID PRIMARY KEY,  -- NO FOREIGN KEY
    ...
);

-- Add comment explaining validation
COMMENT ON COLUMN territory_dk.users_profiles.user_id IS 
    'User ID validated via auth-service API. No FK constraint for service independence.';
```

**Validation Logic:**

```rust
// In user-service when creating profile
async fn create_profile(user_id: Uuid, data: ProfileData) -> Result<Profile> {
    // Call auth-service to verify user exists
    let user = auth_client.get_user(user_id).await?;
    
    // Only create profile if user exists
    sqlx::query!(
        "INSERT INTO territory_dk.users_profiles (user_id, ...) VALUES ($1, ...)",
        user_id
    )
    .execute(&pool)
    .await?;
    
    Ok(profile)
}
```

---

## Data Consistency

### Eventual Consistency via NATS Events

**Pattern:** When critical data changes in one service, publish events for other services to react.

**Example: User Deletion**

```rust
// In auth-service when user is deleted
async fn delete_user(user_id: Uuid) -> Result<()> {
    // Delete user from auth-service
    sqlx::query!("DELETE FROM territory_dk.users WHERE id = $1", user_id)
        .execute(&pool)
        .await?;
    
    // Publish event for other services to clean up their data
    nats_client.publish(
        "user.deleted",
        UserDeletedEvent { user_id, territory_code: "dk" }
    ).await?;
    
    Ok(())
}

// In user-service listening to NATS events
async fn handle_user_deleted(event: UserDeletedEvent) {
    // Clean up user profile data
    sqlx::query!(
        "DELETE FROM territory_dk.users_profiles WHERE user_id = $1",
        event.user_id
    )
    .execute(&pool)
    .await?;
}
```

---

## Migration Execution Script

```bash
#!/bin/bash
# execute_migrations.sh

set -e  # Exit on error

TERRITORY_CODE="${1:-dk}"  # Default to Denmark
DB_NAME="unityplatform"

echo "Executing service migrations for territory: $TERRITORY_CODE"

# Level 0: Foundation
echo "=== Level 0: Foundation ==="
psql -d $DB_NAME -f 00-territory-service/001_create_territories.sql

# Level 1: Core Identity
echo "=== Level 1: Core Identity ==="
psql -d $DB_NAME -f 01-auth-service/001_create_global_registries.sql
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 01-auth-service/002_create_users_tables.sql

# Level 2: User Data (parallel execution)
echo "=== Level 2: User Data ==="
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-user-service/001_create_profiles.sql &
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-settings-service/001_create_settings_tables.sql &
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-invitation-service/001_create_global_registry.sql &
psql -d $DB_NAME -v territory_code=$TERRITORY_CODE -f 02-notification-service/001_create_notifications.sql &
wait  # Wait for all parallel migrations to complete

echo "✅ All migrations completed successfully!"
```

---

## Benefits of This Approach

### 1. Service Independence

- Each service can be deployed independently
- No database-level coupling between services
- Services can be scaled separately

### 2. Clear Dependencies

- Migration order makes dependencies explicit
- Level-based organization shows what can run in parallel
- Easy to understand the dependency graph

### 3. Flexibility

- New services can be added without touching existing migrations
- Services can be moved to separate databases in the future
- Supports gradual microservices transition

### 4. Testing

- Each service's migrations can be tested independently
- Integration tests can verify API-based validation
- Easier to set up test databases for individual services

---

## Migration to Production

### Initial Setup (Cold Start)

1. Run Level 0 migrations (territory-service)
2. Run Level 1 migrations (auth-service)
3. Run Level 2 migrations (parallel)
4. Run Level 3+ migrations as needed

### Existing Database (Hot Migration)

If you already have the monolithic schema:

1. Migrations are idempotent - safe to re-run
2. Use `CREATE TABLE IF NOT EXISTS` for safety
3. Drop foreign key constraints that cross service boundaries
4. Add API validation logic in services

```sql
-- Example: Remove FK constraint
ALTER TABLE territory_dk.users_profiles 
DROP CONSTRAINT IF EXISTS users_profiles_user_id_fkey;

-- Add comment explaining validation
COMMENT ON COLUMN territory_dk.users_profiles.user_id IS 
    'User ID validated via auth-service API. FK constraint removed for service independence.';
```

---

## Next Steps

1. ✅ Create migration files for each service
2. ✅ Document dependencies and execution order
3. 📋 Update service DATABASE.md files with migration references
4. 📋 Create migration execution scripts
5. 📋 Add migration tests
6. 📋 Update docker-compose to run migrations in order

---

**Last Updated:** November 12, 2025  
**Maintained By:** Development Team  
**Related:** See `docs/architecture/MIGRATIONS-MASTER.md` for overview
