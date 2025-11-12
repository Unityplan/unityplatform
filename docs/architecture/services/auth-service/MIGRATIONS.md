# auth-service Migrations Specification

**Service:** auth-service  
**Database:** PostgreSQL 15+  
**Migration Tool:** sqlx-cli  
**Current Version:** 20251111000001

---

## Migration Files Needed

The auth-service requires tables in both **global** and **territory** schemas. These tables are already created in the shared-lib core migration.

---

## ✅ Existing Migrations (in shared-lib)

### Global Schema

**File:** `services/shared-lib/migrations/20251111000001_mvp_core_schema.sql`

#### global.username_registry

```sql
CREATE TABLE global.username_registry (
    username VARCHAR(50) PRIMARY KEY,
    user_id UUID NOT NULL,
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_username UNIQUE (username)
);

CREATE INDEX idx_username_registry_territory ON global.username_registry(territory_code);
CREATE INDEX idx_username_registry_user ON global.username_registry(user_id);
```

**Purpose:** Ensure global username uniqueness across all territory pods

---

#### global.email_registry

```sql
CREATE TABLE global.email_registry (
    email VARCHAR(255) PRIMARY KEY,
    user_id UUID NOT NULL,
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code),
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_email UNIQUE (email)
);

CREATE INDEX idx_email_registry_territory ON global.email_registry(territory_code);
CREATE INDEX idx_email_registry_verified ON global.email_registry(is_verified);
```

**Purpose:** Ensure global email uniqueness across all territory pods

---

### Territory Schema

**File:** `services/shared-lib/migrations/20251111000001_mvp_core_schema.sql`

#### territory_{code}.users

```sql
CREATE TABLE territory_{code}.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Authentication
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    
    -- Profile
    full_name VARCHAR(255),
    
    -- Territory Binding
    territory_code VARCHAR(10) NOT NULL DEFAULT '{code}',
    
    -- Account Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    
    -- Two-Factor Auth
    totp_secret VARCHAR(255),
    totp_enabled BOOLEAN NOT NULL DEFAULT false,
    
    -- Soft Delete
    deleted_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (email IS NULL OR email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$'),
    CHECK (char_length(username) >= 3 AND char_length(username) <= 50),
    CHECK (deleted_at IS NULL OR is_active = false)
);

CREATE INDEX idx_users_username ON territory_{code}.users(username);
CREATE INDEX idx_users_email ON territory_{code}.users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_users_active ON territory_{code}.users(is_active) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_verified ON territory_{code}.users(is_verified);
```

**Purpose:** Store user authentication data locally (data sovereignty)

---

#### territory_{code}.refresh_tokens

```sql
CREATE TABLE territory_{code}.refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Token Data
    token VARCHAR(255) NOT NULL UNIQUE,
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    -- Session Info
    device_name VARCHAR(255),
    ip_address INET,
    
    -- Lifecycle
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (expires_at > created_at),
    CHECK (revoked_at IS NULL OR revoked_at >= created_at)
);

CREATE INDEX idx_refresh_tokens_token ON territory_{code}.refresh_tokens(token);
CREATE INDEX idx_refresh_tokens_user ON territory_{code}.refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_expires ON territory_{code}.refresh_tokens(expires_at) WHERE revoked_at IS NULL;
CREATE INDEX idx_refresh_tokens_active ON territory_{code}.refresh_tokens(user_id, expires_at) WHERE revoked_at IS NULL;
```

**Purpose:** JWT refresh token session management

---

## 📋 Future Migrations

### Migration: Email Verification Tokens

**File:** `services/auth-service/migrations/20251120000001_create_email_verification_tokens.sql`

**Purpose:** Support email verification flow

```sql
-- Territory schema (user's personal data)
CREATE TABLE territory_{code}.email_verification_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    token VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL,
    
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (expires_at > created_at)
);

CREATE INDEX idx_email_verification_tokens_token ON territory_{code}.email_verification_tokens(token);
CREATE INDEX idx_email_verification_tokens_user ON territory_{code}.email_verification_tokens(user_id);
CREATE INDEX idx_email_verification_tokens_expires ON territory_{code}.email_verification_tokens(expires_at) WHERE used_at IS NULL;
```

**Why Territory Schema?**  
Email verification is personal to the user - stays in their territory pod.

---

### Migration: Password Reset Tokens

**File:** `services/auth-service/migrations/20251120000002_create_password_reset_tokens.sql`

**Purpose:** Support password reset flow

```sql
-- Territory schema (user's personal data)
CREATE TABLE territory_{code}.password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    token VARCHAR(255) NOT NULL UNIQUE,
    
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (expires_at > created_at)
);

CREATE INDEX idx_password_reset_tokens_token ON territory_{code}.password_reset_tokens(token);
CREATE INDEX idx_password_reset_tokens_user ON territory_{code}.password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_tokens_expires ON territory_{code}.password_reset_tokens(expires_at) WHERE used_at IS NULL;
```

**Why Territory Schema?**  
Password reset is personal security data - stays in user's territory pod.

---

### Migration: Login Attempts (Rate Limiting)

**File:** `services/auth-service/migrations/20251120000003_create_login_attempts.sql`

**Purpose:** Track failed login attempts for rate limiting

```sql
-- Territory schema (security audit data)
CREATE TABLE territory_{code}.login_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    username VARCHAR(50) NOT NULL,
    ip_address INET NOT NULL,
    user_agent TEXT,
    
    success BOOLEAN NOT NULL,
    failure_reason VARCHAR(100),
    
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_login_attempts_username ON territory_{code}.login_attempts(username, attempted_at);
CREATE INDEX idx_login_attempts_ip ON territory_{code}.login_attempts(ip_address, attempted_at);
CREATE INDEX idx_login_attempts_time ON territory_{code}.login_attempts(attempted_at);

-- Cleanup old attempts (keep 30 days)
-- Run as cron job: DELETE FROM login_attempts WHERE attempted_at < NOW() - INTERVAL '30 days';
```

**Why Territory Schema?**  
Login attempts are security audit logs - stay in territory for sovereignty.

**Performance:**  
Use time-based partitioning for high-traffic pods (future optimization).

---

## Multi-Pod Deployment Strategy

### Single-Pod Deployment (Denmark)

**Schemas Created:**

```sql
CREATE SCHEMA IF NOT EXISTS global;
CREATE SCHEMA IF NOT EXISTS territory_dk;
```

**Migration Command:**

```bash
cd services/shared-lib
sqlx migrate run --database-url "postgresql://user:pass@localhost:5432/unityplan_dk"
```

**Result:**

- `global.*` tables created (shared across future pods)
- `territory_dk.*` tables created (Denmark-specific data)

---

### Multi-Pod Deployment (Denmark, Norway, Sweden)

**Schemas Created:**

```sql
-- Shared global schema
CREATE SCHEMA IF NOT EXISTS global;

-- Territory schemas
CREATE SCHEMA IF NOT EXISTS territory_dk;
CREATE SCHEMA IF NOT EXISTS territory_no;
CREATE SCHEMA IF NOT EXISTS territory_se;
```

**Migration Command:**

```bash
# Deploy to Denmark pod
sqlx migrate run --database-url "postgresql://user:pass@denmark-db:5432/unityplan_dk"

# Deploy to Norway pod
sqlx migrate run --database-url "postgresql://user:pass@norway-db:5432/unityplan_no"

# Deploy to Sweden pod
sqlx migrate run --database-url "postgresql://user:pass@sweden-db:5432/unityplan_se"
```

**Result:**

- Each pod has its own database
- `global.*` schema replicated (or shared via federation)
- `territory_{code}.*` tables unique per pod

---

## Cross-Schema Referential Integrity

### Problem: Foreign Keys Across Schemas

**Challenge:**  
Global registries reference territory users, but PostgreSQL FK constraints don't work across schemas in different databases.

**Solution:**  
Use application-level referential integrity:

```rust
// Registration transaction (pseudo-code)
async fn register_user(username: &str, email: &str, territory: &str) -> Result<Uuid> {
    let mut tx = db.begin().await?;
    
    // 1. Check global uniqueness
    if username_exists_globally(username).await? {
        return Err(Error::UsernameExists);
    }
    
    // 2. Create user in territory schema
    let user_id = sqlx::query!(
        "INSERT INTO territory_dk.users (username, email, password_hash, territory_code)
         VALUES ($1, $2, $3, $4) RETURNING id",
        username, email, password_hash, territory
    ).fetch_one(&mut tx).await?.id;
    
    // 3. Register in global schema
    sqlx::query!(
        "INSERT INTO global.username_registry (username, user_id, territory_code)
         VALUES ($1, $2, $3)",
        username, user_id, territory
    ).execute(&mut tx).await?;
    
    sqlx::query!(
        "INSERT INTO global.email_registry (email, user_id, territory_code)
         VALUES ($1, $2, $3)",
        email, user_id, territory
    ).execute(&mut tx).await?;
    
    // 4. Commit transaction (all or nothing)
    tx.commit().await?;
    
    Ok(user_id)
}
```

**Atomicity:**  
Database transaction ensures all inserts succeed or rollback together.

---

### Deletion Cascade

**Challenge:**  
When user is deleted, clean up global registries.

**Solution:**  
Application-level cascade in delete handler:

```rust
async fn delete_user(user_id: Uuid) -> Result<()> {
    let mut tx = db.begin().await?;
    
    // 1. Get username and email for registry cleanup
    let user = sqlx::query!(
        "SELECT username, email FROM territory_dk.users WHERE id = $1",
        user_id
    ).fetch_one(&mut tx).await?;
    
    // 2. Delete from territory schema (CASCADE deletes refresh_tokens)
    sqlx::query!(
        "DELETE FROM territory_dk.users WHERE id = $1",
        user_id
    ).execute(&mut tx).await?;
    
    // 3. Clean up global registries
    sqlx::query!(
        "DELETE FROM global.username_registry WHERE username = $1",
        user.username
    ).execute(&mut tx).await?;
    
    if let Some(email) = user.email {
        sqlx::query!(
            "DELETE FROM global.email_registry WHERE email = $1",
            email
        ).execute(&mut tx).await?;
    }
    
    tx.commit().await?;
    Ok(())
}
```

---

## Performance Considerations

### Global Registry Size

**Estimated Rows:**

- 10,000 users per pod × 4 pods = 40,000 rows
- 100,000 users per pod × 4 pods = 400,000 rows (mature platform)

**Query Performance:**

```sql
-- Username check (PRIMARY KEY lookup) - <1ms
SELECT username FROM global.username_registry WHERE username = 'alice';

-- Territory lookup - <1ms
SELECT user_id, territory_code FROM global.username_registry WHERE username = 'alice';
```

**Index Size:**

- 40,000 usernames × 50 bytes = 2MB (fits in memory)
- 400,000 usernames × 50 bytes = 20MB (fits in memory)

**Conclusion:**  
Global registries are tiny and fast. No performance concerns.

---

### Refresh Token Cleanup

**Problem:**  
Expired tokens accumulate over time.

**Solution:**  
Scheduled cleanup job (cron or background task):

```sql
-- Delete expired tokens older than 30 days
DELETE FROM territory_dk.refresh_tokens
WHERE expires_at < NOW() - INTERVAL '30 days';
```

**Frequency:** Daily at 3:00 AM (low traffic time)

---

## Testing Strategy

### Unit Tests

**Test Global Uniqueness:**

```rust
#[sqlx::test]
async fn test_duplicate_username_rejected() {
    // Register alice in DK
    register_user("alice", "alice@dk.com", "dk").await.unwrap();
    
    // Try to register alice in NO (should fail)
    let result = register_user("alice", "alice@no.com", "no").await;
    assert!(result.is_err());
}
```

**Test Cascading Delete:**

```rust
#[sqlx::test]
async fn test_user_deletion_cleans_global_registry() {
    let user_id = register_user("bob", "bob@dk.com", "dk").await.unwrap();
    
    delete_user(user_id).await.unwrap();
    
    // Verify global registry cleaned up
    let exists = sqlx::query!("SELECT username FROM global.username_registry WHERE username = 'bob'")
        .fetch_optional(&pool).await.unwrap();
    assert!(exists.is_none());
}
```

---

### Integration Tests

**Test Cross-Pod Registration:**

```bash
# Register alice in DK
curl -X POST http://denmark.unityplan.org/api/v1/auth/register \
  -d '{"username": "alice", "email": "alice@dk.com", "password": "Test123!@#"}'

# Try to register alice in NO (should fail with 409 Conflict)
curl -X POST http://norway.unityplan.org/api/v1/auth/register \
  -d '{"username": "alice", "email": "alice@no.com", "password": "Test123!@#"}'
```

---

## Rollback Strategy

### Down Migration for Future Changes

**Email Verification Tokens Down:**

```sql
DROP TABLE IF EXISTS territory_dk.email_verification_tokens;
```

**Password Reset Tokens Down:**

```sql
DROP TABLE IF EXISTS territory_dk.password_reset_tokens;
```

**Login Attempts Down:**

```sql
DROP TABLE IF EXISTS territory_dk.login_attempts;
```

**Note:**  
Core tables (users, refresh_tokens, global registries) should NOT be dropped - they are foundational.

---

## Migration Checklist

- [x] Global username registry (in shared-lib)
- [x] Global email registry (in shared-lib)
- [x] Territory users table (in shared-lib)
- [x] Territory refresh_tokens table (in shared-lib)
- [ ] Email verification tokens (future)
- [ ] Password reset tokens (future)
- [ ] Login attempts tracking (future)

---

**Last Updated:** November 12, 2025  
**Current Migration Version:** 20251111000001  
**Next Migration Version:** 20251120000001 (email verification)
