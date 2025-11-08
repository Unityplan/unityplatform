-- ============================================================================
-- UnityPlan Global Schema - Identity & Federation Layer
-- Version: 0.1.0-alpha.1
-- Date: 2025-11-08
-- 
-- This schema handles cross-territory identity coordination and federation.
-- NO personal data is stored here - only cryptographic identities.
-- 
-- Deployed: Global identity service (future: separate database/container)
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
-- TRIGGERS
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
