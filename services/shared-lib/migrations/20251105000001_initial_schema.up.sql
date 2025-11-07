-- Initial database schema for UnityPlan
-- Multi-territory architecture with schema-based isolation
-- See: project_docs/2-project-overview.md (Multi-Tenant PostgreSQL Schema Architecture)

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

--------------------------------------------------------------------------------
-- GLOBAL SCHEMA - Cross-territory shared data
--------------------------------------------------------------------------------
CREATE SCHEMA IF NOT EXISTS global;

-- Territories registry - ISO 3166-1 Alpha-2 for countries, {NAME}-FN-{COUNTRY} for First Nations
-- See: project_docs/9-territory-management-standard.md
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    public_key_hash VARCHAR(64) UNIQUE NOT NULL, -- SHA256 hash of user's public key
    territory_code VARCHAR(100) NOT NULL REFERENCES global.territories(code),
    territory_user_id UUID NOT NULL, -- ID of user in their territory schema
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (territory_code, territory_user_id)
);

CREATE INDEX idx_global_user_identities_public_key ON global.user_identities(public_key_hash);
CREATE INDEX idx_global_user_identities_territory ON global.user_identities(territory_code);
CREATE INDEX idx_global_user_identities_created_at ON global.user_identities(created_at);

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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES global.user_identities(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    permissions JSONB DEFAULT '{}'::jsonb,
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    granted_by UUID REFERENCES global.user_identities(id) ON DELETE SET NULL
);

CREATE INDEX idx_global_role_assignments_user ON global.role_assignments(user_id);

-- Sessions table - JWT tracking for authentication
CREATE TABLE global.sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
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

CREATE INDEX idx_territory_dk_users_email ON territory_dk.users(email);
CREATE INDEX idx_territory_dk_users_username ON territory_dk.users(username);
CREATE INDEX idx_territory_dk_users_created_at ON territory_dk.users(created_at);
CREATE INDEX idx_territory_dk_users_invited_by ON territory_dk.users(invited_by_user_id);

-- Invitation tokens table - For managing user invitations
CREATE TABLE territory_dk.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token VARCHAR(255) UNIQUE NOT NULL,
    token_type VARCHAR(20) NOT NULL CHECK (token_type IN ('single_use', 'group')),
    
    -- Metadata
    created_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    invited_email VARCHAR(255), -- For single-use invitations
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
CREATE INDEX idx_territory_dk_invitation_tokens_expires ON territory_dk.invitation_tokens(expires_at);

-- Add foreign key constraint now that invitation_tokens table exists
ALTER TABLE territory_dk.users 
    ADD CONSTRAINT fk_users_invitation_token 
    FOREIGN KEY (invitation_by_token_id) 
    REFERENCES territory_dk.invitation_tokens(id) 
    ON DELETE SET NULL;

-- Invitation uses audit table - Track each use of an invitation token
CREATE TABLE territory_dk.invitation_uses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
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
    ('DK', 'Denmark', 'country', NULL, 'dk', 'Europe/Copenhagen', 'da_DK', 'da');

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

-- Function to sync territory user to global identity
-- This creates/updates a global.user_identities record when a user is created in a territory
CREATE OR REPLACE FUNCTION sync_user_to_global_identity()
RETURNS TRIGGER AS $$
BEGIN
    -- Create a hash from user's email + username as pseudo public key hash
    -- In production, this would be an actual cryptographic public key hash
    INSERT INTO global.user_identities (public_key_hash, territory_code, territory_user_id)
    VALUES (
        encode(sha256((NEW.email || NEW.username)::bytea), 'hex'),
        'DK', -- This territory's code
        NEW.id
    )
    ON CONFLICT (territory_code, territory_user_id) DO NOTHING;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically create global identity when territory user is created
CREATE TRIGGER sync_territory_dk_user_to_global
    AFTER INSERT ON territory_dk.users
    FOR EACH ROW
    EXECUTE FUNCTION sync_user_to_global_identity();
