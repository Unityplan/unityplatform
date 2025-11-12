# auth-service Database Schema

**Service:** auth-service  
**Port:** 8001  
**Database Access:** Global + Territory schemas

---

## Overview

The auth-service handles user authentication and session management. It operates across both **global** and **territory** schemas to ensure username/email uniqueness while maintaining data sovereignty.

**Data Sovereignty Principle:**
- User credentials (password hashes) stored in territory schema (local sovereignty)
- Username/email registries in global schema (prevent duplicates across all pods)
- Sessions (refresh tokens) stored in territory schema (user's data stays local)

---

## Schema Distribution

### Global Schema Tables

**Purpose:** Ensure global uniqueness of usernames and emails across all territory pods

| Table | Purpose | Why Global? |
|-------|---------|-------------|
| `global.username_registry` | Username uniqueness | Prevent duplicate usernames across all territories |
| `global.email_registry` | Email uniqueness | Prevent duplicate emails across all territories |

**Cross-Pod Requirement:**  
When a user in Denmark (dk) registers, the global registry prevents a user in Norway (no) from taking the same username.

### Territory Schema Tables

**Purpose:** Store user authentication data locally (data sovereignty)

| Table | Purpose | Why Territory? |
|-------|---------|----------------|
| `territory_{code}.users` | User accounts | Personal data sovereignty - credentials stay in user's pod |
| `territory_{code}.refresh_tokens` | JWT sessions | Session data is personal - stays in user's territory |

---

## Table Specifications

### Global: `global.username_registry`

**Purpose:** Globally unique username registry

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

**Columns:**
- `username` - PRIMARY KEY, globally unique across all pods
- `user_id` - UUID of user in territory schema
- `territory_code` - Which territory this user belongs to (e.g., 'dk', 'no')
- `created_at` - Registration timestamp

**Performance Note:**  
This table is small (only username + territory mapping) and highly indexed. Username checks are fast.

---

### Global: `global.email_registry`

**Purpose:** Globally unique email registry with verification tracking

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

**Columns:**
- `email` - PRIMARY KEY, globally unique, lowercased
- `user_id` - UUID of user in territory schema
- `territory_code` - Which territory this user belongs to
- `is_verified` - Email verification status
- `verified_at` - When email was verified
- `created_at` - Registration timestamp

**Privacy Note:**  
Emails are stored globally only for uniqueness checking. The actual email is also stored in territory schema for notifications (owned by user).

---

### Territory: `territory_{code}.users`

**Purpose:** User authentication and identity (PRIMARY user data)

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

**Columns:**
- `id` - UUID primary key (generated)
- `username` - Territory-local unique (matches global registry)
- `email` - OPTIONAL (for notifications only)
- `password_hash` - Argon2id hash
- `full_name` - Optional display name
- `territory_code` - Denormalized for queries (default: schema territory)
- `is_active` - Account status
- `is_verified` - Email verification status
- `verified_at` - Verification timestamp
- `totp_secret` - Encrypted TOTP secret for 2FA
- `totp_enabled` - Whether 2FA is enabled
- `deleted_at` - Soft delete timestamp (GDPR Article 17)
- `created_at`, `updated_at` - Timestamps

**Data Sovereignty:**  
Password hashes NEVER leave the territory pod. Authentication happens locally.

---

### Territory: `territory_{code}.refresh_tokens`

**Purpose:** JWT refresh token session management

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

**Columns:**
- `id` - UUID primary key
- `token` - Hashed refresh token (SHA-256)
- `user_id` - Foreign key to users table
- `device_name` - User-agent or device identifier
- `ip_address` - IP address for security audit
- `expires_at` - Token expiration (7 days default)
- `revoked_at` - Manual revocation timestamp
- `created_at` - Token creation
- `last_used_at` - Last refresh operation

**Security:**  
Tokens are hashed before storage. IP addresses tracked for anomaly detection.

---

## Multi-Pod Considerations

### Registration Flow (Cross-Schema Transaction)

**Scenario:** User registers in Denmark pod

```sql
-- Step 1: Check global uniqueness
SELECT username FROM global.username_registry WHERE username = 'alice';
SELECT email FROM global.email_registry WHERE email = 'alice@example.com';

-- Step 2: Insert into territory schema (BEGIN TRANSACTION)
INSERT INTO territory_dk.users (username, email, password_hash, territory_code)
VALUES ('alice', 'alice@example.com', '$argon2id$...', 'dk')
RETURNING id;

-- Step 3: Register in global schema
INSERT INTO global.username_registry (username, user_id, territory_code)
VALUES ('alice', '<user_id>', 'dk');

INSERT INTO global.email_registry (email, user_id, territory_code)
VALUES ('alice@example.com', '<user_id>', 'dk');

-- COMMIT TRANSACTION
```

**Atomicity:**  
All three inserts (territory user + 2 global registries) must succeed or rollback together.

---

### Login Flow (Territory-Local)

**Scenario:** User 'alice' logs in

```sql
-- Step 1: Lookup user in territory schema
SELECT id, username, password_hash, is_active, deleted_at
FROM territory_dk.users
WHERE username = 'alice' AND deleted_at IS NULL;

-- Step 2: Verify password (application layer - Argon2id)

-- Step 3: Create refresh token
INSERT INTO territory_dk.refresh_tokens (token, user_id, device_name, ip_address, expires_at)
VALUES ('$sha256$...', '<user_id>', 'Mozilla/5.0...', '192.168.1.100', NOW() + INTERVAL '7 days');
```

**Performance:**  
Login only touches territory schema - no global lookups needed (fast!).

---

### Cross-Territory Username Lookup

**Scenario:** User in Norway wants to message user 'alice' from Denmark

```sql
-- Step 1: Find which territory 'alice' belongs to
SELECT user_id, territory_code
FROM global.username_registry
WHERE username = 'alice';

-- Step 2: Call user-service in that territory for public profile
-- (via API Gateway routing to denmark.unityplan.org)
```

**Federation Pattern:**  
Global registry provides routing information. Actual user data fetched from their home pod.

---

## Performance Optimizations

### Indexes

**Global Schema:**
- `username_registry.username` - PRIMARY KEY (B-tree)
- `username_registry.territory_code` - Index for territory-specific queries
- `email_registry.email` - PRIMARY KEY (B-tree)

**Territory Schema:**
- `users.username` - UNIQUE index (fast login lookup)
- `users.email` - Partial index (WHERE email IS NOT NULL)
- `users.is_active` - Partial index (WHERE deleted_at IS NULL)
- `refresh_tokens.token` - UNIQUE index (fast token validation)
- `refresh_tokens(user_id, expires_at)` - Composite index (list active sessions)

### Query Patterns

**Fast Queries:**
- Username availability check: `SELECT username FROM global.username_registry WHERE username = ?` (PRIMARY KEY)
- Login: `SELECT * FROM territory_dk.users WHERE username = ?` (UNIQUE index)
- Token validation: `SELECT * FROM territory_dk.refresh_tokens WHERE token = ?` (UNIQUE index)

**Acceptable Queries:**
- List user sessions: `SELECT * FROM refresh_tokens WHERE user_id = ? AND expires_at > NOW()` (composite index)

---

## Data Duplication Strategy

**Global Schema:**
- Username + territory mapping (minimal - just routing)
- Email + territory mapping (minimal - just uniqueness)

**Territory Schema:**
- Full user data (credentials, profile, sessions)

**Why This Works:**
- Global registries are tiny (just usernames/emails + territory codes)
- No sensitive data in global schema (passwords stay in territory)
- Authentication happens locally (fast, secure)
- Cross-territory routing via global lookup (rare operation)

---

## Holochain Migration Path

**Current (PostgreSQL Multi-Pod):**
```
Global Schema: username/email registries
Territory Schema: users, refresh_tokens
```

**Future (Holochain DHT):**
```
Global DHT: username/email registries (public DHT entries)
Local Source Chain: users, refresh_tokens (private source chain)
```

**Migration Strategy:**
1. Keep global registries in PostgreSQL (or global DHT)
2. Move user credentials to Holochain source chain (cryptographically signed)
3. Refresh tokens become Holochain capabilities (zome calls)
4. Username lookups via global DHT (distributed, no central server)

**Benefits:**
- User owns their source chain (full sovereignty)
- Global DHT for username discovery (decentralized)
- Cryptographic proofs instead of password hashes

---

## Security Considerations

### Password Storage

**Algorithm:** Argon2id  
**Parameters:** Memory cost 19MB, Time cost 2, Parallelism 1  
**Output:** 64-byte hash

**Why Argon2id?**
- Winner of Password Hashing Competition (PHC 2015)
- Resistant to GPU/ASIC attacks
- Configurable memory hardness
- Recommended by OWASP

### Token Security

**Access Tokens:**
- JWT with 15-minute expiration
- Signed with RS256 (RSA public key cryptography)
- No database storage (stateless)

**Refresh Tokens:**
- 7-day expiration
- SHA-256 hashed before storage
- Stored in database (can be revoked)
- IP address tracking

### GDPR Compliance

**Right to Erasure (Article 17):**
- Soft delete: `UPDATE users SET deleted_at = NOW(), is_active = false WHERE id = ?`
- Hard delete (after 30 days): `DELETE FROM users WHERE id = ? AND deleted_at < NOW() - INTERVAL '30 days'`
- Global registries: `DELETE FROM global.username_registry WHERE user_id = ?`

**Cascading Deletes:**
- `refresh_tokens` → CASCADE (delete all sessions)
- Global registries → Manual cleanup (username becomes available)

---

## Service Dependencies

### Depends On:
- **invitation-service** - Validate invitation tokens during registration
- **notification-service** - Send verification emails

### Used By:
- **All services** - JWT token validation
- **user-service** - User creation after successful registration

### NATS Events Published:
- `user.registered` - New user created (triggers settings, notification setup)
- `user.login` - User logged in (analytics)
- `user.logout` - User logged out
- `user.deleted` - Account deletion requested (GDPR)

---

**Last Updated:** November 12, 2025  
**Schema Version:** 20251111000001 (MVP Core Schema)  
**Migration Status:** Tables exist in shared-lib migrations
