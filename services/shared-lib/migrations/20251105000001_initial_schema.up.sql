-- Initial database schema for UnityPlan
-- Multi-territory architecture with schema-based isolation
-- See: project_docs/2-project-overview.md (Multi-Tenant PostgreSQL Schema Architecture)

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

--------------------------------------------------------------------------------
-- GLOBAL SCHEMA - Cross-territory shared data
--------------------------------------------------------------------------------
CREATE SCHEMA IF NOT EXISTS global;

-- Global users table - SSO and authentication
CREATE TABLE global.users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    avatar_url TEXT,
    bio TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_global_users_email ON global.users(email);
CREATE INDEX idx_global_users_username ON global.users(username);
CREATE INDEX idx_global_users_created_at ON global.users(created_at);

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

-- Territory managers - Users who manage specific territories
CREATE TABLE global.territory_managers (
    user_id UUID REFERENCES global.users(id) ON DELETE CASCADE,
    territory_code VARCHAR(100) REFERENCES global.territories(code) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL, -- 'territory_admin', 'moderator', etc.
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    granted_by UUID REFERENCES global.users(id) ON DELETE SET NULL,
    PRIMARY KEY (user_id, territory_code, role)
);

CREATE INDEX idx_territory_managers_user ON global.territory_managers(user_id);
CREATE INDEX idx_territory_managers_territory ON global.territory_managers(territory_code);

-- Global role assignments (Platform Admin, DevOps, etc.)
CREATE TABLE global.role_assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES global.users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    permissions JSONB DEFAULT '{}'::jsonb,
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    granted_by UUID REFERENCES global.users(id) ON DELETE SET NULL
);

CREATE INDEX idx_global_role_assignments_user ON global.role_assignments(user_id);

-- Sessions table - JWT tracking for authentication
CREATE TABLE global.sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES global.users(id) ON DELETE CASCADE,
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
    user_id UUID REFERENCES global.users(id) ON DELETE SET NULL,
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

-- Community members - Users belonging to communities in this territory
CREATE TABLE territory_dk.community_members (
    user_id UUID REFERENCES global.users(id) ON DELETE CASCADE,
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

-- Trigger for global.users table
CREATE TRIGGER update_global_users_updated_at
    BEFORE UPDATE ON global.users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
