-- ============================================================================
-- Migration: 20251112000003_auth_core_tables.sql
-- Level: 1 (Core Identity)
-- Service: auth-service
-- Description: Create authentication core tables (users, global registries)
-- Dependencies: 20251112000002_create_territory_schema_template.sql
-- ============================================================================

-- ============================================================================
-- Global: Username Registry (Global Uniqueness)
-- ============================================================================
-- Purpose: Ensure usernames are globally unique across all territories
-- Owner: auth-service
-- Strategy: Check this table before creating users in any territory
-- ============================================================================

CREATE TABLE IF NOT EXISTS global.username_registry (
    username VARCHAR(50) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL,
    user_id UUID NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Reference to territory (OK - global table)
    CONSTRAINT fk_username_territory 
        FOREIGN KEY (territory_code) 
        REFERENCES global.territories(code) 
        ON DELETE CASCADE,
    
    -- Ensure username format
    CHECK (char_length(username) >= 3 AND char_length(username) <= 50),
    CHECK (username ~ '^[a-z0-9_]+$')  -- lowercase, numbers, underscores only
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_username_registry_territory 
    ON global.username_registry(territory_code);
CREATE INDEX IF NOT EXISTS idx_username_registry_user 
    ON global.username_registry(user_id);

-- Comments
COMMENT ON TABLE global.username_registry IS 
    'Global username registry ensuring usernames are unique across all territories. Checked before user creation.';
COMMENT ON COLUMN global.username_registry.username IS 
    'Globally unique username (lowercase, 3-50 chars, alphanumeric + underscores)';

-- ============================================================================
-- Global: Email Registry (Global Uniqueness)
-- ============================================================================
-- Purpose: Ensure emails are globally unique across all territories
-- Owner: auth-service
-- Strategy: Check this table before creating users in any territory
-- ============================================================================

CREATE TABLE IF NOT EXISTS global.email_registry (
    email VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL,
    user_id UUID NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Reference to territory (OK - global table)
    CONSTRAINT fk_email_territory 
        FOREIGN KEY (territory_code) 
        REFERENCES global.territories(code) 
        ON DELETE CASCADE,
    
    -- Ensure email format (basic check)
    CHECK (email ~ '^[^@]+@[^@]+\.[^@]+$')
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_email_registry_territory 
    ON global.email_registry(territory_code);
CREATE INDEX IF NOT EXISTS idx_email_registry_user 
    ON global.email_registry(user_id);
CREATE INDEX IF NOT EXISTS idx_email_registry_verified 
    ON global.email_registry(verified);

-- Comments
COMMENT ON TABLE global.email_registry IS 
    'Global email registry ensuring emails are unique across all territories. Checked before user creation.';
COMMENT ON COLUMN global.email_registry.verified IS 
    'Email verification status. Users cannot login until verified.';

-- ============================================================================
-- Territory: Users Table (Authentication Data)
-- ============================================================================
-- Purpose: User authentication credentials and account status
-- Owner: auth-service
-- Scope: Territory-specific (data sovereignty)
-- JWT: All user info encoded in JWT to minimize database queries
-- ============================================================================

CREATE TABLE IF NOT EXISTS territory_dk.users (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Authentication credentials
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    
    -- Account status
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    deleted_at TIMESTAMPTZ,  -- Soft delete
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,
    
    -- Territory reference (denormalized for convenience)
    territory_code VARCHAR(10) NOT NULL DEFAULT 'dk',
    
    -- Constraints
    CHECK (char_length(username) >= 3 AND char_length(username) <= 50),
    CHECK (username ~ '^[a-z0-9_]+$'),
    CHECK (email ~ '^[^@]+@[^@]+\.[^@]+$'),
    CHECK (deleted_at IS NULL OR active = FALSE)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_users_email ON territory_dk.users(email) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_users_username ON territory_dk.users(username) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_users_active ON territory_dk.users(active) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_users_created_at ON territory_dk.users(created_at);
CREATE INDEX IF NOT EXISTS idx_users_deleted_at ON territory_dk.users(deleted_at) WHERE deleted_at IS NOT NULL;

-- Comments
COMMENT ON TABLE territory_dk.users IS 
    'User authentication data. JWT contains all user info to minimize database queries (99% of requests use JWT only).';
COMMENT ON COLUMN territory_dk.users.password_hash IS 
    'Argon2id password hash (recommended for 2024+)';
COMMENT ON COLUMN territory_dk.users.email_verified IS 
    'Email verification status. Users cannot login until verified = true.';
COMMENT ON COLUMN territory_dk.users.deleted_at IS 
    'Soft delete timestamp. When set, user is marked inactive and cannot login.';
COMMENT ON COLUMN territory_dk.users.territory_code IS 
    'Territory code for this user. Denormalized for JWT payload.';

-- ============================================================================
-- Territory: Refresh Tokens (JWT Token Management)
-- ============================================================================
-- Purpose: Manage JWT refresh tokens for secure token rotation
-- Owner: auth-service
-- Scope: Territory-specific
-- Strategy: Access tokens expire in 15min, refresh tokens in 7 days
-- ============================================================================

CREATE TABLE IF NOT EXISTS territory_dk.refresh_tokens (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Token data
    token_hash VARCHAR(255) NOT NULL UNIQUE,  -- Hashed token (never store plain)
    user_id UUID NOT NULL,
    
    -- Token lifecycle
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    revoked_at TIMESTAMPTZ,
    
    -- Device/session info
    user_agent TEXT,
    ip_address INET,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (revoked_at IS NULL OR revoked = TRUE),
    CHECK (expires_at > created_at)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user ON territory_dk.refresh_tokens(user_id) WHERE revoked = FALSE;
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_token_hash ON territory_dk.refresh_tokens(token_hash) WHERE revoked = FALSE;
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expires_at ON territory_dk.refresh_tokens(expires_at) WHERE revoked = FALSE;

-- Comments
COMMENT ON TABLE territory_dk.refresh_tokens IS 
    'JWT refresh tokens for secure token rotation. Access tokens expire in 15min, refresh tokens in 7 days.';
COMMENT ON COLUMN territory_dk.refresh_tokens.token_hash IS 
    'SHA-256 hash of refresh token. Never store plain tokens in database.';
COMMENT ON COLUMN territory_dk.refresh_tokens.revoked IS 
    'Token revocation status. Revoked tokens cannot be used for refresh.';

-- ============================================================================
-- Functions: Update timestamp trigger
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply trigger to users table
DROP TRIGGER IF EXISTS update_users_updated_at ON territory_dk.users;
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON territory_dk.users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251112000003 complete: Auth service core tables created';
    RAISE NOTICE 'ℹ️  Tables created:';
    RAISE NOTICE '   - global.username_registry (global uniqueness)';
    RAISE NOTICE '   - global.email_registry (global uniqueness)';
    RAISE NOTICE '   - territory_dk.users (authentication data)';
    RAISE NOTICE '   - territory_dk.refresh_tokens (JWT tokens)';
    RAISE NOTICE 'ℹ️  Next: Create user-service tables (profiles, connections)';
END $$;
