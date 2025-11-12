-- ============================================================================
-- UnityPlan Territory Schema - User Data & Local Services
-- Version: 0.1.0-alpha.1
-- Date: 2025-11-08
-- 
-- This schema contains ALL personal user data for a territory.
-- Schema follows data sovereignty principle: user data NEVER leaves territory.
-- 
-- Deployed: In each territory pod's database
-- Schema name: 'territory' (single-territory pods) or 'territory_XX' (multi-territory pods)
-- 
-- NOTE: Replace 'territory' with 'territory_XX' for multi-territory pods
--       Example: territory_de, territory_fr, territory_es in Europe pod
-- ============================================================================

-- Prerequisite: Extensions must be enabled (done in global schema migration)
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- CREATE EXTENSION IF NOT EXISTS "pgcrypto";

--------------------------------------------------------------------------------
-- TERRITORY SCHEMA - Generic structure for all territories
--------------------------------------------------------------------------------
CREATE SCHEMA IF NOT EXISTS territory;

-- Territory settings - Configuration specific to this territory
CREATE TABLE territory.settings (
    key VARCHAR(100) PRIMARY KEY,
    value JSONB NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Communities within this territory
CREATE TABLE territory.communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(100) UNIQUE NOT NULL, -- e.g., 'DK-COPENHAGEN', 'NO-OSLO'
    name VARCHAR(255) NOT NULL,
    parent_code VARCHAR(100), -- For nested communities
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_territory_communities_code ON territory.communities(code);

-- Territory users - ALL PERSONAL DATA STORED HERE (sovereignty principle)
CREATE TABLE territory.users (
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
    invited_by_user_id UUID REFERENCES territory.users(id) ON DELETE SET NULL,
    invitation_by_token_id UUID, -- Will reference territory.invitation_tokens
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

-- Email is optional but must be unique if provided
CREATE UNIQUE INDEX idx_territory_users_email_unique ON territory.users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_territory_users_username ON territory.users(username);
CREATE INDEX idx_territory_users_created_at ON territory.users(created_at);
CREATE INDEX idx_territory_users_invited_by ON territory.users(invited_by_user_id);

COMMENT ON TABLE territory.users IS 'User accounts for this territory. ALL personal data stays in this schema (data sovereignty principle).';
COMMENT ON COLUMN territory.users.username IS 'Username within territory. Synchronized to global.user_identities.username for global uniqueness.';
COMMENT ON COLUMN territory.users.email IS 'Optional email address for notifications only. NOT used for identity or authentication. User can register without email using invitation codes/QR codes.';

-- Invitation tokens table - For managing user invitations
CREATE TABLE territory.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(255) UNIQUE NOT NULL,
    token_type VARCHAR(20) NOT NULL CHECK (token_type IN ('single_use', 'group')),
    
    -- Metadata
    created_by_user_id UUID REFERENCES territory.users(id) ON DELETE CASCADE,
    invited_email VARCHAR(255),     -- Optional email for targeted invitations
    invited_username VARCHAR(50),   -- Optional username for targeted invitations
    community_id UUID REFERENCES territory.communities(id) ON DELETE CASCADE,
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

CREATE INDEX idx_territory_invitation_tokens_token ON territory.invitation_tokens(token);
CREATE INDEX idx_territory_invitation_tokens_created_by ON territory.invitation_tokens(created_by_user_id);
CREATE INDEX idx_territory_invitation_tokens_email ON territory.invitation_tokens(invited_email);
CREATE INDEX idx_territory_invitation_tokens_username ON territory.invitation_tokens(invited_username);
CREATE INDEX idx_territory_invitation_tokens_expires ON territory.invitation_tokens(expires_at);

COMMENT ON COLUMN territory.invitation_tokens.invited_email IS 'Optional email for targeted invitations. If NULL, invitation is a bearer token (anyone with code can use). Invitation can also be shared via QR code, messaging apps, etc.';
COMMENT ON COLUMN territory.invitation_tokens.invited_username IS 'Optional username for targeted invitations. Can invite specific user by username even without email.';

-- Add foreign key constraint now that invitation_tokens table exists
ALTER TABLE territory.users 
    ADD CONSTRAINT fk_users_invitation_token 
    FOREIGN KEY (invitation_by_token_id) 
    REFERENCES territory.invitation_tokens(id) 
    ON DELETE SET NULL;

-- Invitation uses audit table - Track each use of an invitation token
CREATE TABLE territory.invitation_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_id UUID NOT NULL REFERENCES territory.invitation_tokens(id) ON DELETE CASCADE,
    used_by_user_id UUID NOT NULL REFERENCES territory.users(id) ON DELETE CASCADE,
    ip_address INET,
    user_agent TEXT,
    used_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_territory_invitation_uses_token ON territory.invitation_uses(token_id);
CREATE INDEX idx_territory_invitation_uses_user ON territory.invitation_uses(used_by_user_id);
CREATE INDEX idx_territory_invitation_uses_used_at ON territory.invitation_uses(used_at);

-- Community members - Users belonging to communities in this territory
CREATE TABLE territory.community_members (
    user_id UUID REFERENCES territory.users(id) ON DELETE CASCADE,
    community_id UUID REFERENCES territory.communities(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, community_id)
);

CREATE INDEX idx_territory_community_members_user ON territory.community_members(user_id);
CREATE INDEX idx_territory_community_members_community ON territory.community_members(community_id);

--------------------------------------------------------------------------------
-- TRIGGERS
--------------------------------------------------------------------------------

-- Trigger for territory.users table
CREATE TRIGGER update_territory_users_updated_at
    BEFORE UPDATE ON territory.users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger for territory.invitation_tokens table
CREATE TRIGGER update_territory_invitation_tokens_updated_at
    BEFORE UPDATE ON territory.invitation_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Function to create global identity when territory user is created
-- Uses username + territory + UUID for public key hash generation
-- NOTE: This function is territory-specific and must be customized per pod
CREATE OR REPLACE FUNCTION create_global_user_identity()
RETURNS TRIGGER AS $$
DECLARE
    v_territory_code VARCHAR(100);
    v_public_key_hash VARCHAR(64);
    v_global_user_id UUID;
BEGIN
    -- Get territory code from configuration
    -- TODO: Make this dynamic based on pod configuration
    SELECT value::text FROM territory.settings WHERE key = 'territory_code' INTO v_territory_code;
    
    -- Fallback if not configured (for backwards compatibility)
    IF v_territory_code IS NULL THEN
        v_territory_code := 'dk';  -- Default for development
    END IF;
    
    -- Remove quotes from JSONB text value if present
    v_territory_code := TRIM(BOTH '"' FROM v_territory_code);
    
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
    AFTER INSERT ON territory.users
    FOR EACH ROW
    EXECUTE FUNCTION create_global_user_identity();
