-- User Data Sovereignty Migration
-- Move user personal data from global schema to territory schemas
-- Align with natural ecosystem metaphor: users (flowers) belong to their pods

-- ============================================================================
-- CRITICAL ARCHITECTURAL CHANGE
-- ============================================================================
-- Users are flowers that bloom in their specific pods (territories).
-- Personal data MUST stay in territory schema for data sovereignty.
-- Global schema only stores cryptographic identifiers for cross-territory coordination.

--------------------------------------------------------------------------------
-- 1. DROP OLD GLOBAL.USERS TABLE (personal data in wrong place)
--------------------------------------------------------------------------------

-- First, we need to preserve any existing data
-- In production, this would involve data migration
-- For now (alpha), we can safely drop as no production data exists

DROP TABLE IF EXISTS global.users CASCADE;

--------------------------------------------------------------------------------
-- 2. CREATE GLOBAL.USER_IDENTITIES (cryptographic identifiers only)
--------------------------------------------------------------------------------

-- Global schema: ONLY cryptographic hashes, NO personal data
CREATE TABLE global.user_identities (
    -- Cryptographic identity (future Holochain agent public key hash)
    public_key_hash VARCHAR(64) PRIMARY KEY,
    
    -- Territory reference (where user's data actually lives)
    territory_code VARCHAR(100) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    
    -- User ID within their territory schema
    territory_user_id UUID NOT NULL,
    
    -- Cross-territory coordination timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ,
    
    -- Unique constraint: one identity per territory user
    UNIQUE(territory_code, territory_user_id)
);

CREATE INDEX idx_user_identities_territory ON global.user_identities(territory_code);
CREATE INDEX idx_user_identities_last_seen ON global.user_identities(last_seen_at);

COMMENT ON TABLE global.user_identities IS 'Cryptographic user identities for cross-territory coordination. NO personal data stored here. Maps public key hashes to territory-local user records.';
COMMENT ON COLUMN global.user_identities.public_key_hash IS 'Blake2b/Ed25519 public key hash. Future: Holochain agent ID. Current: Generated from user credentials.';
COMMENT ON COLUMN global.user_identities.territory_code IS 'Which territory (pod) owns this user''s data. User data NEVER leaves this territory.';

--------------------------------------------------------------------------------
-- 3. ADD USERS TABLE TO TERRITORY SCHEMA TEMPLATE
--------------------------------------------------------------------------------

-- Territory DK users table (personal data stays in territory)
CREATE TABLE IF NOT EXISTS territory_dk.users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Future Holochain identity (prepare for migration)
    public_key_hash VARCHAR(64) UNIQUE,  -- Blake2b hash, links to global.user_identities
    
    -- Current authentication (email/password - temporary until Holochain/WebAuthn)
    email VARCHAR(255) UNIQUE,           -- Optional in future (with WebAuthn)
    password_hash VARCHAR(255),          -- Optional with WebAuthn/Holochain
    
    -- User profile data (STAYS IN TERRITORY - data sovereignty)
    username VARCHAR(50) UNIQUE NOT NULL,
    full_name VARCHAR(255),
    display_name VARCHAR(100),
    avatar_url TEXT,
    bio TEXT,
    
    -- Privacy controls (user sovereignty)
    email_visible BOOLEAN DEFAULT FALSE,
    profile_public BOOLEAN DEFAULT TRUE,
    data_export_requested BOOLEAN DEFAULT FALSE,
    
    -- Account status
    is_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    last_login_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_territory_dk_users_email ON territory_dk.users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_territory_dk_users_username ON territory_dk.users(username);
CREATE INDEX idx_territory_dk_users_public_key_hash ON territory_dk.users(public_key_hash) WHERE public_key_hash IS NOT NULL;
CREATE INDEX idx_territory_dk_users_created_at ON territory_dk.users(created_at);

COMMENT ON TABLE territory_dk.users IS 'User accounts for Denmark territory. ALL personal data stays in this schema. Users are flowers that bloom in their pod.';
COMMENT ON COLUMN territory_dk.users.public_key_hash IS 'Cryptographic identity hash. Links to global.user_identities for cross-territory coordination.';
COMMENT ON COLUMN territory_dk.users.email IS 'Email for password reset. Optional with WebAuthn. Will be optional with Holochain.';
COMMENT ON COLUMN territory_dk.users.password_hash IS 'Argon2 password hash. Optional with WebAuthn/Holochain auth.';

--------------------------------------------------------------------------------
-- 4. CREATE REFRESH TOKENS TABLE IN TERRITORY SCHEMA
--------------------------------------------------------------------------------

-- Refresh tokens also stay in territory (session data = personal data)
CREATE TABLE IF NOT EXISTS territory_dk.refresh_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    revoked_reason VARCHAR(255)
);

CREATE INDEX idx_territory_dk_refresh_tokens_user ON territory_dk.refresh_tokens(user_id);
CREATE INDEX idx_territory_dk_refresh_tokens_expires ON territory_dk.refresh_tokens(expires_at);
CREATE INDEX idx_territory_dk_refresh_tokens_token ON territory_dk.refresh_tokens(token_hash);

COMMENT ON TABLE territory_dk.refresh_tokens IS 'User refresh tokens. Personal session data stays in territory schema.';

--------------------------------------------------------------------------------
-- 5. UPDATE GLOBAL.SESSIONS TO REFERENCE USER_IDENTITIES
--------------------------------------------------------------------------------

-- Drop old sessions table (referenced global.users which no longer exists)
DROP TABLE IF EXISTS global.sessions CASCADE;

-- Recreate with reference to user_identities
CREATE TABLE global.sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    public_key_hash VARCHAR(64) NOT NULL REFERENCES global.user_identities(public_key_hash) ON DELETE CASCADE,
    territory_code VARCHAR(100) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    token_hash VARCHAR(255) UNIQUE NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_global_sessions_user ON global.sessions(public_key_hash);
CREATE INDEX idx_global_sessions_territory ON global.sessions(territory_code);
CREATE INDEX idx_global_sessions_token ON global.sessions(token_hash);
CREATE INDEX idx_global_sessions_expires ON global.sessions(expires_at);

COMMENT ON TABLE global.sessions IS 'Active session tracking across territories. Uses cryptographic identity hashes only.';

--------------------------------------------------------------------------------
-- 6. UPDATE AUDIT LOG TO USE CRYPTOGRAPHIC IDENTITIES
--------------------------------------------------------------------------------

-- Update audit_log table to reference user_identities
ALTER TABLE global.audit_log 
    DROP CONSTRAINT IF EXISTS audit_log_user_id_fkey;

ALTER TABLE global.audit_log 
    RENAME COLUMN user_id TO public_key_hash;

ALTER TABLE global.audit_log 
    ALTER COLUMN public_key_hash TYPE VARCHAR(64);

ALTER TABLE global.audit_log 
    ADD CONSTRAINT audit_log_public_key_hash_fkey 
    FOREIGN KEY (public_key_hash) REFERENCES global.user_identities(public_key_hash) ON DELETE SET NULL;

CREATE INDEX idx_global_audit_log_user ON global.audit_log(public_key_hash);

COMMENT ON COLUMN global.audit_log.public_key_hash IS 'Cryptographic user identity. No personal data in audit logs.';

--------------------------------------------------------------------------------
-- 7. UPDATE TERRITORY MANAGERS TO USE CRYPTOGRAPHIC IDENTITIES
--------------------------------------------------------------------------------

DROP TABLE IF EXISTS global.territory_managers CASCADE;

CREATE TABLE global.territory_managers (
    public_key_hash VARCHAR(64) NOT NULL REFERENCES global.user_identities(public_key_hash) ON DELETE CASCADE,
    territory_code VARCHAR(100) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL, -- 'territory_admin', 'moderator', etc.
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    granted_by VARCHAR(64) REFERENCES global.user_identities(public_key_hash) ON DELETE SET NULL,
    PRIMARY KEY (public_key_hash, territory_code, role)
);

CREATE INDEX idx_territory_managers_user ON global.territory_managers(public_key_hash);
CREATE INDEX idx_territory_managers_territory ON global.territory_managers(territory_code);

COMMENT ON TABLE global.territory_managers IS 'Territory management roles. Uses cryptographic identities, not personal data.';

--------------------------------------------------------------------------------
-- 8. UPDATE ROLE ASSIGNMENTS TO USE CRYPTOGRAPHIC IDENTITIES
--------------------------------------------------------------------------------

DROP TABLE IF EXISTS global.role_assignments CASCADE;

CREATE TABLE global.role_assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    public_key_hash VARCHAR(64) NOT NULL REFERENCES global.user_identities(public_key_hash) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    permissions JSONB DEFAULT '{}'::jsonb,
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    granted_by VARCHAR(64) REFERENCES global.user_identities(public_key_hash) ON DELETE SET NULL
);

CREATE INDEX idx_global_role_assignments_user ON global.role_assignments(public_key_hash);

COMMENT ON TABLE global.role_assignments IS 'Global platform roles (Platform Admin, DevOps). Uses cryptographic identities.';

--------------------------------------------------------------------------------
-- 9. UPDATE TRIGGERS FOR TERRITORY USERS
--------------------------------------------------------------------------------

-- Trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_territory_user_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_territory_dk_users_updated_at
    BEFORE UPDATE ON territory_dk.users
    FOR EACH ROW
    EXECUTE FUNCTION update_territory_user_updated_at();

-- Trigger to sync public_key_hash to global.user_identities
CREATE OR REPLACE FUNCTION sync_user_identity()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.public_key_hash IS NOT NULL THEN
        INSERT INTO global.user_identities (public_key_hash, territory_code, territory_user_id)
        VALUES (NEW.public_key_hash, 'DK', NEW.id)
        ON CONFLICT (public_key_hash) DO NOTHING;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sync_territory_dk_user_identity
    AFTER INSERT OR UPDATE OF public_key_hash ON territory_dk.users
    FOR EACH ROW
    WHEN (NEW.public_key_hash IS NOT NULL)
    EXECUTE FUNCTION sync_user_identity();

COMMENT ON FUNCTION sync_user_identity IS 'Automatically sync territory user to global.user_identities when public_key_hash is set.';
