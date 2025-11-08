-- ============================================================================
-- UnityPlan Initial Database Schema (Consolidated)
-- Version: 0.1.0-alpha.1
-- Date: 2025-11-08
-- 
-- This is the consolidated alpha schema including:
-- - Multi-territory architecture with schema-based isolation
-- - User data sovereignty (personal data in territory schemas)
-- - Identity system with global username uniqueness
-- - Privacy-first design (email optional)
-- ============================================================================

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";  -- For gen_random_uuid() and digest()

--------------------------------------------------------------------------------
-- GLOBAL SCHEMA - Cross-territory shared data
--------------------------------------------------------------------------------
CREATE SCHEMA IF NOT EXISTS global;

-- Territories registry - ISO 3166-1 Alpha-2 for countries, {NAME}-FN-{COUNTRY} for First Nations
CREATE TABLE global.territories (
    code VARCHAR(100) PRIMARY KEY, -- 'DK', 'NO', 'SE', 'HAIDA-FN-CA', etc.
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL CHECK (type IN ('country', 'first_nation', 'community')),
    parent_territory_code VARCHAR(100) REFERENCES global.territories(code),
    pod_id VARCHAR(50), -- Which pod serves this territory
    timezone VARCHAR(100), -- IANA timezone
    locale VARCHAR(10), -- e.g., 'da_DK'
    default_language VARCHAR(10), -- ISO 639-1
    metadata JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Sovereignty constraint: countries and First Nations have no parent
    CHECK (
        (type IN ('country', 'first_nation') AND parent_territory_code IS NULL)
        OR
        (type = 'community' AND parent_territory_code IS NOT NULL)
    )
);

CREATE INDEX idx_global_territories_type ON global.territories(type);
CREATE INDEX idx_global_territories_parent ON global.territories(parent_territory_code);
CREATE INDEX idx_global_territories_pod ON global.territories(pod_id);

-- Global user identities - Cryptographic hashes ONLY (no personal data)
-- This table links territory users to global identity for SSO and cross-territory operations
CREATE TABLE global.user_identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL,  -- Globally unique username
    public_key_hash VARCHAR(64) UNIQUE NOT NULL, -- SHA256 hash of user's public key
    territory_code VARCHAR(100) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    territory_user_id UUID NOT NULL, -- ID of user in their territory schema
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (territory_code, territory_user_id)
);

CREATE INDEX idx_global_user_identities_public_key ON global.user_identities(public_key_hash);
CREATE INDEX idx_global_user_identities_territory ON global.user_identities(territory_code);
CREATE INDEX idx_global_user_identities_created_at ON global.user_identities(created_at);
CREATE UNIQUE INDEX idx_global_user_identities_username_lower ON global.user_identities(LOWER(username));
CREATE INDEX idx_global_user_identities_username_territory ON global.user_identities(username, territory_code);

COMMENT ON TABLE global.user_identities IS 'Cryptographic user identities for cross-territory coordination. NO personal data stored here.';
COMMENT ON COLUMN global.user_identities.username IS 'Globally unique username across all pods/territories. Used for federation (username@territory), Matrix ID (@username:unityplan.{territory}), and human-readable lookup. Never changes even during territory migration.';
COMMENT ON COLUMN global.user_identities.public_key_hash IS 'SHA-256 hash generated from username + territory + UUID. Future: Holochain agent ID or WebAuthn public key.';
COMMENT ON COLUMN global.user_identities.territory_code IS 'Which territory (pod) owns this user''s data. User data NEVER leaves this territory.';

-- Territory managers - Users who manage specific territories
CREATE TABLE global.territory_managers (
    user_id UUID REFERENCES global.user_identities(id) ON DELETE CASCADE,
    territory_code VARCHAR(100) REFERENCES global.territories(code) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL, -- 'territory_admin', 'moderator', etc.
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    granted_by UUID REFERENCES global.user_identities(id) ON DELETE SET NULL,
    PRIMARY KEY (user_id, territory_code, role)
);

CREATE INDEX idx_territory_managers_user ON global.territory_managers(user_id);
CREATE INDEX idx_territory_managers_territory ON global.territory_managers(territory_code);

-- Global role assignments (Platform Admin, DevOps, etc.)
CREATE TABLE global.role_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES global.user_identities(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    permissions JSONB DEFAULT '{}'::jsonb,
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    granted_by UUID REFERENCES global.user_identities(id) ON DELETE SET NULL
);

CREATE INDEX idx_global_role_assignments_user ON global.role_assignments(user_id);

-- Sessions table - JWT tracking for authentication
CREATE TABLE global.sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES global.user_identities(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) UNIQUE NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_global_sessions_user ON global.sessions(user_id);
CREATE INDEX idx_global_sessions_token ON global.sessions(token_hash);
CREATE INDEX idx_global_sessions_expires ON global.sessions(expires_at);

-- Audit log - Immutable system-wide audit trail
CREATE TABLE global.audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES global.user_identities(id) ON DELETE SET NULL,
    territory_code VARCHAR(100) REFERENCES global.territories(code) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50),
    resource_id VARCHAR(255),
    changes JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_global_audit_log_user ON global.audit_log(user_id);
CREATE INDEX idx_global_audit_log_territory ON global.audit_log(territory_code);
CREATE INDEX idx_global_audit_log_created ON global.audit_log(created_at);

--------------------------------------------------------------------------------
-- TERRITORY SCHEMA - DK (Denmark)
--------------------------------------------------------------------------------
CREATE SCHEMA IF NOT EXISTS territory_dk;

-- Territory settings
CREATE TABLE territory_dk.settings (
    key VARCHAR(100) PRIMARY KEY,
    value JSONB NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Communities within Denmark
CREATE TABLE territory_dk.communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(100) UNIQUE NOT NULL, -- e.g., 'DK-COPENHAGEN'
    name VARCHAR(255) NOT NULL,
    parent_code VARCHAR(100), -- For nested communities
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_territory_dk_communities_code ON territory_dk.communities(code);

-- Territory users - ALL PERSONAL DATA STORED HERE (sovereignty principle)
CREATE TABLE territory_dk.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,          -- Required, unique within territory
    email VARCHAR(255),                            -- Optional (for notifications only)
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    display_name VARCHAR(100),
    avatar_url TEXT,
    bio TEXT,
    date_of_birth DATE,
    phone VARCHAR(50),
    
    -- Privacy and preferences
    profile_visibility VARCHAR(20) DEFAULT 'public' CHECK (profile_visibility IN ('public', 'community', 'private')),
    email_notifications BOOLEAN DEFAULT TRUE NOT NULL,
    push_notifications BOOLEAN DEFAULT FALSE NOT NULL,
    
    -- Status flags
    is_verified BOOLEAN DEFAULT FALSE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    
    -- Invitation tracking
    invited_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    invitation_by_token_id UUID, -- Will reference territory_dk.invitation_tokens
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

-- Email is optional but must be unique if provided
CREATE UNIQUE INDEX idx_territory_dk_users_email_unique ON territory_dk.users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_territory_dk_users_username ON territory_dk.users(username);
CREATE INDEX idx_territory_dk_users_created_at ON territory_dk.users(created_at);
CREATE INDEX idx_territory_dk_users_invited_by ON territory_dk.users(invited_by_user_id);

COMMENT ON TABLE territory_dk.users IS 'User accounts for Denmark territory. ALL personal data stays in this schema (data sovereignty principle).';
COMMENT ON COLUMN territory_dk.users.username IS 'Username within territory. Synchronized to global.user_identities.username for global uniqueness.';
COMMENT ON COLUMN territory_dk.users.email IS 'Optional email address for notifications only. NOT used for identity or authentication. User can register without email using invitation codes/QR codes.';

-- Invitation tokens table - For managing user invitations
CREATE TABLE territory_dk.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(255) UNIQUE NOT NULL,
    token_type VARCHAR(20) NOT NULL CHECK (token_type IN ('single_use', 'group')),
    
    -- Metadata
    created_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    invited_email VARCHAR(255),     -- Optional email for targeted invitations
    invited_username VARCHAR(50),   -- Optional username for targeted invitations
    community_id UUID REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member',
    
    -- Usage tracking
    max_uses INTEGER, -- NULL = unlimited (for group invitations)
    current_uses INTEGER DEFAULT 0 NOT NULL,
    
    -- Validity
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_territory_dk_invitation_tokens_token ON territory_dk.invitation_tokens(token);
CREATE INDEX idx_territory_dk_invitation_tokens_created_by ON territory_dk.invitation_tokens(created_by_user_id);
CREATE INDEX idx_territory_dk_invitation_tokens_email ON territory_dk.invitation_tokens(invited_email);
CREATE INDEX idx_territory_dk_invitation_tokens_username ON territory_dk.invitation_tokens(invited_username);
CREATE INDEX idx_territory_dk_invitation_tokens_expires ON territory_dk.invitation_tokens(expires_at);

COMMENT ON COLUMN territory_dk.invitation_tokens.invited_email IS 'Optional email for targeted invitations. If NULL, invitation is a bearer token (anyone with code can use). Invitation can also be shared via QR code, messaging apps, etc.';
COMMENT ON COLUMN territory_dk.invitation_tokens.invited_username IS 'Optional username for targeted invitations. Can invite specific user by username even without email.';

-- Add foreign key constraint now that invitation_tokens table exists
ALTER TABLE territory_dk.users 
    ADD CONSTRAINT fk_users_invitation_token 
    FOREIGN KEY (invitation_by_token_id) 
    REFERENCES territory_dk.invitation_tokens(id) 
    ON DELETE SET NULL;

-- Invitation uses audit table - Track each use of an invitation token
CREATE TABLE territory_dk.invitation_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_id UUID NOT NULL REFERENCES territory_dk.invitation_tokens(id) ON DELETE CASCADE,
    used_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    ip_address INET,
    user_agent TEXT,
    used_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_territory_dk_invitation_uses_token ON territory_dk.invitation_uses(token_id);
CREATE INDEX idx_territory_dk_invitation_uses_user ON territory_dk.invitation_uses(used_by_user_id);
CREATE INDEX idx_territory_dk_invitation_uses_used_at ON territory_dk.invitation_uses(used_at);

-- Community members - Users belonging to communities in this territory
CREATE TABLE territory_dk.community_members (
    user_id UUID REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    community_id UUID REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, community_id)
);

CREATE INDEX idx_territory_dk_community_members_user ON territory_dk.community_members(user_id);
CREATE INDEX idx_territory_dk_community_members_community ON territory_dk.community_members(community_id);

--------------------------------------------------------------------------------
-- DEFAULT DATA
--------------------------------------------------------------------------------

-- Insert Denmark territory (this pod's territory)
INSERT INTO global.territories (code, name, type, parent_territory_code, pod_id, timezone, locale, default_language) VALUES
    ('dk', 'Denmark', 'country', NULL, 'dk', 'Europe/Copenhagen', 'da_DK', 'da');

-- Insert default territory settings for Denmark
INSERT INTO territory_dk.settings (key, value) VALUES
    ('language', '"da"'::jsonb),
    ('timezone', '"Europe/Copenhagen"'::jsonb),
    ('locale', '"da_DK"'::jsonb);

--------------------------------------------------------------------------------
-- FUNCTIONS AND TRIGGERS
--------------------------------------------------------------------------------

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for global.user_identities table
CREATE TRIGGER update_global_user_identities_updated_at
    BEFORE UPDATE ON global.user_identities
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger for territory_dk.users table
CREATE TRIGGER update_territory_dk_users_updated_at
    BEFORE UPDATE ON territory_dk.users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger for territory_dk.invitation_tokens table
CREATE TRIGGER update_territory_dk_invitation_tokens_updated_at
    BEFORE UPDATE ON territory_dk.invitation_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Function to create global identity when territory user is created
-- Uses username + territory + UUID for public key hash generation
CREATE OR REPLACE FUNCTION create_global_user_identity()
RETURNS TRIGGER AS $$
DECLARE
    v_territory_code VARCHAR(100);
    v_public_key_hash VARCHAR(64);
    v_global_user_id UUID;
BEGIN
    -- Get territory code (hardcoded for now, could be dynamic)
    v_territory_code := 'dk';
    
    -- Generate new global UUID
    v_global_user_id := gen_random_uuid();
    
    -- Generate public key hash: SHA-256(username::territory::uuid)
    v_public_key_hash := encode(
        digest(
            NEW.username || '::' || v_territory_code || '::' || v_global_user_id::text,
            'sha256'
        ),
        'hex'
    );
    
    -- Insert into global.user_identities
    INSERT INTO global.user_identities (
        id,
        username,
        public_key_hash,
        territory_code,
        territory_user_id,
        created_at,
        updated_at
    ) VALUES (
        v_global_user_id,
        NEW.username,
        v_public_key_hash,
        v_territory_code,
        NEW.id,
        NOW(),
        NOW()
    );
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically create global identity when territory user is created
CREATE TRIGGER trg_create_global_identity
    AFTER INSERT ON territory_dk.users
    FOR EACH ROW
    EXECUTE FUNCTION create_global_user_identity();
