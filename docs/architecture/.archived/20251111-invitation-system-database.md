# Invitation System - Database Implementation Guide

**Status:** Implementation Ready  
**Created:** November 11, 2025  
**Purpose:** Detailed database implementation for invitation-based registration  
**Related:** [Invitation System Architecture](invitation-system.md)

---

## Overview

This document provides SQL scripts and implementation details for the invitation system that enables secure, territory-bound user registration.

---

## Database Tables

### 1. Global Invitation Token Registry

**Location:** `global.invitation_token_registry`

**Purpose:** Maps invitation tokens to territories (prevents territory manipulation)

```sql
-- Create global schema if not exists
CREATE SCHEMA IF NOT EXISTS global;

-- Global invitation token registry
CREATE TABLE global.invitation_token_registry (
    token VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    territory_token_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_invitation_token UNIQUE (token)
);

CREATE INDEX idx_global_invitation_registry_token 
    ON global.invitation_token_registry(token);
    
CREATE INDEX idx_global_invitation_registry_territory 
    ON global.invitation_token_registry(territory_code);

COMMENT ON TABLE global.invitation_token_registry IS 
    'Registry of all invitation tokens across territories for security verification';
    
COMMENT ON COLUMN global.invitation_token_registry.token IS 
    'The invitation token string (same as in territory schema)';
    
COMMENT ON COLUMN global.invitation_token_registry.territory_code IS 
    'Territory where this invitation was created and is valid';
    
COMMENT ON COLUMN global.invitation_token_registry.territory_token_id IS 
    'Foreign key to territory_X.invitation_tokens.id';
```

### 2. Territory Invitation Tokens

**Location:** `territory_{code}.invitation_tokens` (e.g., `territory_dk.invitation_tokens`)

**Purpose:** Stores invitation token details within each territory

```sql
-- Create territory schema (example: Denmark)
CREATE SCHEMA IF NOT EXISTS territory_dk;

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Invitation tokens table
CREATE TABLE territory_dk.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(64) UNIQUE NOT NULL,
    token_type VARCHAR(20) NOT NULL,
    
    -- Restrictions
    email VARCHAR(255),
    max_uses INT NOT NULL DEFAULT 1,
    used_count INT NOT NULL DEFAULT 0,
    
    -- Metadata
    created_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    community_id UUID REFERENCES territory_dk.communities(id),
    purpose TEXT,
    metadata JSONB,
    
    -- Lifecycle
    expires_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    revoked_at TIMESTAMPTZ,
    revoked_by_user_id UUID REFERENCES territory_dk.users(id),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT chk_token_type 
        CHECK (token_type IN ('single_use', 'group')),
    
    CONSTRAINT chk_single_use_email 
        CHECK (
            (token_type = 'single_use' AND email IS NOT NULL AND max_uses = 1) OR
            (token_type = 'group' AND email IS NULL AND max_uses > 1)
        ),
    
    CONSTRAINT chk_usage_limit 
        CHECK (used_count <= max_uses),
    
    CONSTRAINT chk_revoked_inactive 
        CHECK (revoked_at IS NULL OR is_active = false)
);

-- Indexes
CREATE INDEX idx_invitation_tokens_token 
    ON territory_dk.invitation_tokens(token);
    
CREATE INDEX idx_invitation_tokens_email 
    ON territory_dk.invitation_tokens(email) 
    WHERE email IS NOT NULL;
    
CREATE INDEX idx_invitation_tokens_created_by 
    ON territory_dk.invitation_tokens(created_by_user_id);
    
CREATE INDEX idx_invitation_tokens_active 
    ON territory_dk.invitation_tokens(is_active, expires_at);
    
CREATE INDEX idx_invitation_tokens_community 
    ON territory_dk.invitation_tokens(community_id) 
    WHERE community_id IS NOT NULL;

-- Comments
COMMENT ON TABLE territory_dk.invitation_tokens IS 
    'Invitation tokens for controlled user registration in Denmark territory';
    
COMMENT ON COLUMN territory_dk.invitation_tokens.token IS 
    'Unique cryptographic token string shared with invitee';
    
COMMENT ON COLUMN territory_dk.invitation_tokens.token_type IS 
    'Type: single_use (personal) or group (multi-use)';
    
COMMENT ON COLUMN territory_dk.invitation_tokens.community_id IS 
    'Optional: Auto-assign new users to this community upon registration';
```

### 3. Invitation Uses Audit Trail

**Location:** `territory_{code}.invitation_uses`

**Purpose:** Track who used which invitation (immutable audit log)

```sql
CREATE TABLE territory_dk.invitation_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invitation_token_id UUID NOT NULL REFERENCES territory_dk.invitation_tokens(id),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    
    -- Audit Data
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT,
    
    -- Additional Context
    metadata JSONB,
    
    -- Prevent duplicate records
    CONSTRAINT uq_invitation_user UNIQUE (invitation_token_id, user_id)
);

-- Indexes
CREATE INDEX idx_invitation_uses_token 
    ON territory_dk.invitation_uses(invitation_token_id);
    
CREATE INDEX idx_invitation_uses_user 
    ON territory_dk.invitation_uses(user_id);
    
CREATE INDEX idx_invitation_uses_timestamp 
    ON territory_dk.invitation_uses(used_at DESC);

COMMENT ON TABLE territory_dk.invitation_uses IS 
    'Immutable audit trail of invitation token redemptions';
```

---

## Triggers and Functions

### 1. Auto-Update Timestamp

```sql
-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to invitation_tokens table
CREATE TRIGGER update_invitation_tokens_updated_at
    BEFORE UPDATE ON territory_dk.invitation_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

### 2. Sync to Global Registry

```sql
-- Function to sync new invitation tokens to global registry
CREATE OR REPLACE FUNCTION sync_invitation_to_global_registry()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO global.invitation_token_registry (token, territory_code, territory_token_id)
    VALUES (NEW.token, 'dk', NEW.id)
    ON CONFLICT (token) DO NOTHING;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to sync on insert
CREATE TRIGGER sync_invitation_token_to_global
    AFTER INSERT ON territory_dk.invitation_tokens
    FOR EACH ROW
    EXECUTE FUNCTION sync_invitation_to_global_registry();
```

### 3. Auto-Deactivate Expired Tokens

```sql
-- Function to deactivate expired tokens
CREATE OR REPLACE FUNCTION deactivate_expired_invitations()
RETURNS void AS $$
BEGIN
    UPDATE territory_dk.invitation_tokens
    SET is_active = false
    WHERE is_active = true
      AND expires_at < NOW();
END;
$$ LANGUAGE plpgsql;

-- Run as cron job or periodic task (example: every hour)
-- Note: Requires pg_cron extension or external scheduler
```

### 4. Increment Token Usage Counter

```sql
-- Function to record invitation use and increment counter
CREATE OR REPLACE FUNCTION record_invitation_use(
    p_token VARCHAR,
    p_user_id UUID,
    p_ip_address INET DEFAULT NULL,
    p_user_agent TEXT DEFAULT NULL
)
RETURNS BOOLEAN AS $$
DECLARE
    v_token_id UUID;
    v_max_uses INT;
    v_used_count INT;
    v_token_type VARCHAR;
BEGIN
    -- Get token details
    SELECT id, max_uses, used_count, token_type
    INTO v_token_id, v_max_uses, v_used_count, v_token_type
    FROM territory_dk.invitation_tokens
    WHERE token = p_token
      AND is_active = true
      AND expires_at > NOW()
    FOR UPDATE;
    
    -- Check if token exists and is valid
    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;
    
    -- Check if usage limit reached
    IF v_used_count >= v_max_uses THEN
        RETURN FALSE;
    END IF;
    
    -- Record usage
    INSERT INTO territory_dk.invitation_uses (
        invitation_token_id, 
        user_id, 
        ip_address, 
        user_agent
    ) VALUES (
        v_token_id, 
        p_user_id, 
        p_ip_address, 
        p_user_agent
    );
    
    -- Increment counter
    UPDATE territory_dk.invitation_tokens
    SET used_count = used_count + 1
    WHERE id = v_token_id;
    
    -- Deactivate if max uses reached
    IF v_used_count + 1 >= v_max_uses THEN
        UPDATE territory_dk.invitation_tokens
        SET is_active = false
        WHERE id = v_token_id;
    END IF;
    
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION record_invitation_use IS 
    'Records invitation token usage, increments counter, and deactivates if limit reached';
```

---

## Queries for Common Operations

### 1. Validate Invitation Token

```sql
-- Get invitation details for validation
SELECT 
    it.id,
    it.token,
    it.token_type,
    it.email,
    it.max_uses,
    it.used_count,
    (it.max_uses - it.used_count) AS remaining_uses,
    it.expires_at,
    it.is_active,
    it.community_id,
    c.name AS community_name,
    u.username AS created_by_username
FROM territory_dk.invitation_tokens it
LEFT JOIN territory_dk.communities c ON c.id = it.community_id
LEFT JOIN territory_dk.users u ON u.id = it.created_by_user_id
WHERE it.token = $1
  AND it.is_active = true
  AND it.expires_at > NOW();
```

### 2. Check Token Territory

```sql
-- Lookup territory from token
SELECT 
    gir.territory_code,
    t.name AS territory_name,
    t.display_name AS territory_display_name
FROM global.invitation_token_registry gir
JOIN global.territories t ON t.code = gir.territory_code
WHERE gir.token = $1;
```

### 3. Create Single-Use Invitation

```sql
-- Generate cryptographically random token
WITH new_token AS (
    SELECT encode(gen_random_bytes(32), 'base64') AS token
)
INSERT INTO territory_dk.invitation_tokens (
    token,
    token_type,
    email,
    max_uses,
    created_by_user_id,
    expires_at,
    purpose
)
SELECT 
    token,
    'single_use',
    $1, -- email
    1,
    $2, -- creator_user_id
    NOW() + INTERVAL '7 days',
    $3  -- purpose
FROM new_token
RETURNING id, token, expires_at;
```

### 4. Create Group Invitation

```sql
-- Generate group invitation token
WITH new_token AS (
    SELECT encode(gen_random_bytes(32), 'base64') AS token
)
INSERT INTO territory_dk.invitation_tokens (
    token,
    token_type,
    max_uses,
    created_by_user_id,
    community_id,
    expires_at,
    purpose,
    metadata
)
SELECT 
    token,
    'group',
    $1, -- max_uses
    $2, -- creator_user_id
    $3, -- community_id (optional)
    $4, -- expires_at
    $5, -- purpose
    $6  -- metadata (JSONB)
FROM new_token
RETURNING id, token, max_uses, expires_at;
```

### 5. List User's Created Invitations

```sql
-- Get all invitations created by a user
SELECT 
    it.id,
    it.token,
    it.token_type,
    it.email,
    it.max_uses,
    it.used_count,
    (it.max_uses - it.used_count) AS remaining_uses,
    it.expires_at,
    it.is_active,
    it.purpose,
    c.name AS community_name,
    COUNT(iu.id) AS actual_uses
FROM territory_dk.invitation_tokens it
LEFT JOIN territory_dk.communities c ON c.id = it.community_id
LEFT JOIN territory_dk.invitation_uses iu ON iu.invitation_token_id = it.id
WHERE it.created_by_user_id = $1
GROUP BY it.id, c.name
ORDER BY it.created_at DESC;
```

### 6. Revoke Invitation

```sql
-- Revoke an invitation token
UPDATE territory_dk.invitation_tokens
SET 
    is_active = false,
    revoked_at = NOW(),
    revoked_by_user_id = $2
WHERE id = $1
  AND created_by_user_id = $2  -- Only creator can revoke
RETURNING id, token, revoked_at;
```

### 7. Get Invitation Usage History

```sql
-- View who used an invitation
SELECT 
    iu.id,
    u.username,
    u.email,
    u.full_name,
    iu.used_at,
    iu.ip_address,
    iu.user_agent
FROM territory_dk.invitation_uses iu
JOIN territory_dk.users u ON u.id = iu.user_id
WHERE iu.invitation_token_id = $1
ORDER BY iu.used_at DESC;
```

---

## Migration Scripts

### Initial Setup (all territories)

```sql
-- Run this for each territory: dk, no, se, eu

-- 1. Create schema
CREATE SCHEMA IF NOT EXISTS territory_dk;

-- 2. Create tables
\i scripts/db/territory_schema/01_users.sql
\i scripts/db/territory_schema/02_invitation_tokens.sql
\i scripts/db/territory_schema/03_invitation_uses.sql

-- 3. Create triggers
\i scripts/db/territory_schema/04_triggers.sql

-- 4. Create functions
\i scripts/db/territory_schema/05_functions.sql

-- 5. Seed data (optional)
\i scripts/db/territory_schema/06_seed_invitations.sql
```

### Rollback Script

```sql
-- Rollback invitation system (use with caution!)

-- Drop triggers
DROP TRIGGER IF EXISTS sync_invitation_token_to_global ON territory_dk.invitation_tokens;
DROP TRIGGER IF EXISTS update_invitation_tokens_updated_at ON territory_dk.invitation_tokens;

-- Drop tables (cascades to child tables)
DROP TABLE IF EXISTS territory_dk.invitation_uses CASCADE;
DROP TABLE IF EXISTS territory_dk.invitation_tokens CASCADE;

-- Drop functions
DROP FUNCTION IF EXISTS record_invitation_use(VARCHAR, UUID, INET, TEXT);
DROP FUNCTION IF EXISTS deactivate_expired_invitations();
DROP FUNCTION IF EXISTS sync_invitation_to_global_registry();

-- Drop from global registry
DELETE FROM global.invitation_token_registry 
WHERE territory_code = 'dk';
```

---

## Security Considerations

### 1. Token Generation

**Requirement:** Cryptographically secure random tokens

```sql
-- PostgreSQL: Use gen_random_bytes()
SELECT encode(gen_random_bytes(32), 'base64') AS token;

-- Rust backend alternative (recommended):
use rand::Rng;
let token: String = rand::thread_rng()
    .sample_iter(&rand::distributions::Alphanumeric)
    .take(64)
    .map(char::from)
    .collect();
```

### 2. Rate Limiting

**Prevent brute force token guessing:**

```sql
-- Track failed validation attempts (add to schema)
CREATE TABLE territory_dk.invitation_validation_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_attempt VARCHAR(255) NOT NULL,
    ip_address INET NOT NULL,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_validation_attempts_ip 
    ON territory_dk.invitation_validation_attempts(ip_address, attempted_at);

-- Rate limit function
CREATE OR REPLACE FUNCTION check_validation_rate_limit(
    p_ip_address INET
)
RETURNS BOOLEAN AS $$
DECLARE
    v_attempt_count INT;
BEGIN
    SELECT COUNT(*)
    INTO v_attempt_count
    FROM territory_dk.invitation_validation_attempts
    WHERE ip_address = p_ip_address
      AND attempted_at > NOW() - INTERVAL '15 minutes';
    
    RETURN v_attempt_count < 10;  -- Max 10 attempts per 15 minutes
END;
$$ LANGUAGE plpgsql;
```

### 3. Email Validation

**For single-use tokens, verify email match:**

```sql
-- Validate email matches token
SELECT id
FROM territory_dk.invitation_tokens
WHERE token = $1
  AND (email IS NULL OR LOWER(email) = LOWER($2))
  AND is_active = true
  AND expires_at > NOW();
```

---

## Testing

### Test Data Setup

```sql
-- Create test user (invitation creator)
INSERT INTO territory_dk.users (id, username, email, password_hash, territory_code)
VALUES (
    '11111111-1111-1111-1111-111111111111',
    'test_admin',
    'admin@example.com',
    '$2b$12$hash', -- Replace with real hash
    'dk'
);

-- Create single-use invitation
INSERT INTO territory_dk.invitation_tokens (
    token,
    token_type,
    email,
    max_uses,
    created_by_user_id,
    expires_at
) VALUES (
    'test_single_use_token_abc123',
    'single_use',
    'alice@example.com',
    1,
    '11111111-1111-1111-1111-111111111111',
    NOW() + INTERVAL '7 days'
);

-- Create group invitation
INSERT INTO territory_dk.invitation_tokens (
    token,
    token_type,
    max_uses,
    created_by_user_id,
    expires_at,
    purpose
) VALUES (
    'test_group_token_xyz789',
    'group',
    50,
    '11111111-1111-1111-1111-111111111111',
    NOW() + INTERVAL '30 days',
    'Test Group Workshop'
);
```

### Unit Tests

```sql
-- Test: Token validation returns correct data
SELECT 
    token,
    token_type,
    email,
    remaining_uses
FROM territory_dk.invitation_tokens
WHERE token = 'test_single_use_token_abc123';
-- Expected: Returns 1 row with remaining_uses = 1

-- Test: Record invitation use
SELECT record_invitation_use(
    'test_single_use_token_abc123',
    '22222222-2222-2222-2222-222222222222'::UUID
);
-- Expected: Returns TRUE

-- Test: Token deactivated after use
SELECT is_active
FROM territory_dk.invitation_tokens
WHERE token = 'test_single_use_token_abc123';
-- Expected: FALSE

-- Test: Cannot reuse single-use token
SELECT record_invitation_use(
    'test_single_use_token_abc123',
    '33333333-3333-3333-3333-333333333333'::UUID
);
-- Expected: Returns FALSE
```

---

## Performance Optimization

### 1. Cleanup Old Records

```sql
-- Archive expired invitations (run periodically)
CREATE TABLE territory_dk.invitation_tokens_archive (
    LIKE territory_dk.invitation_tokens INCLUDING ALL
);

-- Move expired/used tokens to archive
WITH moved_tokens AS (
    DELETE FROM territory_dk.invitation_tokens
    WHERE is_active = false
      AND expires_at < NOW() - INTERVAL '90 days'
    RETURNING *
)
INSERT INTO territory_dk.invitation_tokens_archive
SELECT * FROM moved_tokens;
```

### 2. Partitioning (for high volume)

```sql
-- Partition invitation_uses by month
CREATE TABLE territory_dk.invitation_uses (
    id UUID DEFAULT gen_random_uuid(),
    invitation_token_id UUID NOT NULL,
    user_id UUID NOT NULL,
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT
) PARTITION BY RANGE (used_at);

-- Create partitions
CREATE TABLE territory_dk.invitation_uses_2025_11 
    PARTITION OF territory_dk.invitation_uses
    FOR VALUES FROM ('2025-11-01') TO ('2025-12-01');
    
CREATE TABLE territory_dk.invitation_uses_2025_12 
    PARTITION OF territory_dk.invitation_uses
    FOR VALUES FROM ('2025-12-01') TO ('2026-01-01');
```

---

## Related Documents

- [Backend API Requirements](backend-api-requirements.md)
- [Invitation System Architecture](invitation-system.md)
- [Database Schema Design](database-schema-design.md)
