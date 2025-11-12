-- ============================================================================
-- Migration: 20251111000001_mvp_core_schema.sql
-- Description: MVP Phase 1 - Complete core schema (30 tables)
-- Created: 2025-11-11
-- Database: PostgreSQL 15+ with UUID extension
-- Territory: template for dk (Denmark single-pod example)
-- ============================================================================
--
-- This migration creates the complete MVP Phase 1 database schema including:
--   - Global schema (4 tables): territories, registries
--   - Territory schema (30 tables): users, communities, badges, groups, social
--
-- Schema Design: docs/architecture/database-schema-design.md
-- API Specification: docs/guides/development/backend-api-implementation.md
--
-- ============================================================================

-- ============================================================================
-- EXTENSIONS
-- ============================================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================================
-- GLOBAL SCHEMA (4 tables)
-- ============================================================================

CREATE SCHEMA IF NOT EXISTS global;

-- ----------------------------------------------------------------------------
-- 1. Global Territories Registry
-- ----------------------------------------------------------------------------

CREATE TABLE global.territories (
    code VARCHAR(10) PRIMARY KEY,              -- 'dk', 'no', 'se', 'eu'
    name VARCHAR(100) NOT NULL,                -- 'Denmark', 'Norway', etc.
    display_name VARCHAR(100) NOT NULL,        -- 'Denmark Territory'
    description TEXT,
    
    -- Pod Configuration
    pod_url VARCHAR(255) NOT NULL,             -- https://denmark.unityplan.org
    api_url VARCHAR(255) NOT NULL,             -- https://api.denmark.unityplan.org
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- 'active', 'maintenance', 'inactive'
    
    -- Metadata
    language_code VARCHAR(10) NOT NULL,        -- 'da', 'no', 'sv', 'en'
    timezone VARCHAR(50) NOT NULL,             -- 'Europe/Copenhagen'
    currency_code VARCHAR(3),                  -- 'DKK', 'NOK', 'SEK'
    
    -- Holochain DNA Info (Future)
    dna_hash VARCHAR(128),                     -- Holochain DNA identifier
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_territories_status ON global.territories(status);

-- ----------------------------------------------------------------------------
-- 2. Global Username Registry
-- ----------------------------------------------------------------------------

CREATE TABLE global.username_registry (
    username VARCHAR(50) PRIMARY KEY,          -- Globally unique username
    user_id UUID NOT NULL,                     -- FK to territory_X.users.id
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_username UNIQUE (username)
);

CREATE INDEX idx_username_registry_territory ON global.username_registry(territory_code);
CREATE INDEX idx_username_registry_user ON global.username_registry(user_id);

-- ----------------------------------------------------------------------------
-- 3. Global Email Registry
-- ----------------------------------------------------------------------------

CREATE TABLE global.email_registry (
    email VARCHAR(255) PRIMARY KEY,            -- Globally unique email (lowercased)
    user_id UUID NOT NULL,                     -- FK to territory_X.users.id
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code),
    
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_email UNIQUE (email)
);

CREATE INDEX idx_email_registry_territory ON global.email_registry(territory_code);
CREATE INDEX idx_email_registry_verified ON global.email_registry(is_verified);

-- ----------------------------------------------------------------------------
-- 4. Global Invitation Token Registry
-- ----------------------------------------------------------------------------

CREATE TABLE global.invitation_token_registry (
    token VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    territory_token_id UUID NOT NULL,          -- FK to territory_X.invitation_tokens.id
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_invitation_token UNIQUE (token)
);

CREATE INDEX idx_global_invitation_registry_token ON global.invitation_token_registry(token);
CREATE INDEX idx_global_invitation_registry_territory ON global.invitation_token_registry(territory_code);

-- ============================================================================
-- TERRITORY SCHEMA - DENMARK (territory_dk)
-- ============================================================================
-- Note: For multi-territory pods, repeat this section for each territory
-- with different schema names (territory_no, territory_se, etc.)
-- ============================================================================

CREATE SCHEMA IF NOT EXISTS territory_dk;

-- ============================================================================
-- USERS & PROFILES (7 tables)
-- ============================================================================

-- ----------------------------------------------------------------------------
-- 1. Users (Authentication & Identity)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Authentication (Username is primary identifier)
    username VARCHAR(50) NOT NULL UNIQUE,      -- Territory-local unique (matches global)
    email VARCHAR(255) UNIQUE,                 -- OPTIONAL - for notifications only
    password_hash VARCHAR(255) NOT NULL,       -- bcrypt/argon2
    
    -- Profile
    full_name VARCHAR(255),
    
    -- Territory Binding
    territory_code VARCHAR(10) NOT NULL DEFAULT 'dk', -- Denormalized for queries
    
    -- Account Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    
    -- Two-Factor Auth
    totp_secret VARCHAR(255),                  -- Encrypted TOTP secret
    totp_enabled BOOLEAN NOT NULL DEFAULT false,
    
    -- Soft Delete
    deleted_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (email IS NULL OR email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$'),
    CHECK (char_length(username) >= 3 AND char_length(username) <= 50),
    CHECK (deleted_at IS NULL OR is_active = false)
);

CREATE INDEX idx_users_username ON territory_dk.users(username);
CREATE INDEX idx_users_email ON territory_dk.users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_users_active ON territory_dk.users(is_active) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_verified ON territory_dk.users(is_verified);

-- ----------------------------------------------------------------------------
-- 2. Refresh Tokens (JWT Session Management)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Token Data (hashed for security)
    token VARCHAR(255) NOT NULL UNIQUE,        -- Hashed refresh token (SHA-256)
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Session Info
    device_name VARCHAR(255),                  -- User-agent or device identifier
    ip_address INET,                           -- IP address for security
    
    -- Lifecycle
    expires_at TIMESTAMPTZ NOT NULL,           -- Token expiration
    revoked_at TIMESTAMPTZ,                    -- Manual revocation timestamp
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (expires_at > created_at),
    CHECK (revoked_at IS NULL OR revoked_at >= created_at)
);

CREATE INDEX idx_refresh_tokens_token ON territory_dk.refresh_tokens(token);
CREATE INDEX idx_refresh_tokens_user ON territory_dk.refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_expires ON territory_dk.refresh_tokens(expires_at) WHERE revoked_at IS NULL;
CREATE INDEX idx_refresh_tokens_active ON territory_dk.refresh_tokens(user_id, expires_at) WHERE revoked_at IS NULL;

-- ----------------------------------------------------------------------------
-- 3. User Profiles
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.users_profiles (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Display Info
    display_name VARCHAR(100),
    avatar_url VARCHAR(500),                   -- S3/IPFS URL
    
    -- Rich Profile
    bio VARCHAR(280),                          -- Short tagline
    about TEXT,                                -- Long-form (Markdown)
    
    -- Arrays (PostgreSQL native, JSON in Holochain)
    interests TEXT[],                          -- Array of tags
    skills TEXT[],                             -- Array of tags
    languages VARCHAR(10)[],                   -- Array of ISO 639-1 codes
    
    -- Location (privacy-aware encoded format)
    location VARCHAR(500),                     -- "[lat,lng]Display Name"
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_profiles_display_name ON territory_dk.users_profiles(display_name);
CREATE INDEX idx_users_profiles_interests ON territory_dk.users_profiles USING GIN(interests);
CREATE INDEX idx_users_profiles_skills ON territory_dk.users_profiles USING GIN(skills);

-- Full-text search
CREATE INDEX idx_users_profiles_search ON territory_dk.users_profiles USING GIN(
    to_tsvector('english', 
        COALESCE(display_name, '') || ' ' || 
        COALESCE(bio, '') || ' ' || 
        COALESCE(about, '')
    )
);

-- ----------------------------------------------------------------------------
-- 4. Profile Links (External URLs)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.users_profile_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Link Data
    label VARCHAR(100) NOT NULL,               -- "My Portfolio", "GitHub"
    url VARCHAR(500) NOT NULL,                 -- Full URL
    icon VARCHAR(50),                          -- Icon name or emoji
    
    -- Display Control
    display_order INT NOT NULL DEFAULT 0,      -- Sort order
    is_visible BOOLEAN NOT NULL DEFAULT true,  -- Show/hide
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (url ~* '^https?://'),               -- Must be valid URL
    UNIQUE (user_id, display_order)            -- Unique order per user
);

CREATE INDEX idx_users_profile_links_user ON territory_dk.users_profile_links(user_id);
CREATE INDEX idx_users_profile_links_order ON territory_dk.users_profile_links(user_id, display_order);

-- ----------------------------------------------------------------------------
-- 5. Language Proficiency
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.users_language_proficiency (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Language Info
    language_code VARCHAR(10) NOT NULL,        -- ISO 639-1 code ('en', 'da', 'es')
    proficiency_level VARCHAR(20) NOT NULL,    -- 'native', 'fluent', 'intermediate', 'beginner'
    
    -- Display
    display_order INT NOT NULL DEFAULT 0,
    is_preferred BOOLEAN NOT NULL DEFAULT false, -- Preferred content language
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE (user_id, language_code),
    CHECK (proficiency_level IN ('native', 'fluent', 'intermediate', 'beginner'))
);

CREATE INDEX idx_users_language_proficiency_user ON territory_dk.users_language_proficiency(user_id);
CREATE INDEX idx_users_language_proficiency_order ON territory_dk.users_language_proficiency(user_id, display_order);
CREATE INDEX idx_users_language_proficiency_preferred ON territory_dk.users_language_proficiency(user_id, is_preferred) WHERE is_preferred = true;

-- ----------------------------------------------------------------------------
-- 6. User Settings (Privacy, Preferences)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.users_settings (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Privacy Settings
    profile_visibility VARCHAR(20) NOT NULL DEFAULT 'public', -- 'public', 'connections_only', 'private'
    email_visibility VARCHAR(20) NOT NULL DEFAULT 'private',  -- 'public', 'connections_only', 'private'
    badge_visibility VARCHAR(20) NOT NULL DEFAULT 'public',   -- 'public', 'connections_only', 'private'
    activity_visibility VARCHAR(20) NOT NULL DEFAULT 'public', -- 'public', 'connections_only', 'private'
    
    -- Communication Preferences
    allow_messages_from VARCHAR(20) NOT NULL DEFAULT 'connections_only', -- 'everyone', 'connections_only', 'none'
    allow_connection_requests BOOLEAN NOT NULL DEFAULT true,
    
    -- Content Preferences
    mature_content_filter BOOLEAN NOT NULL DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (profile_visibility IN ('public', 'connections_only', 'private')),
    CHECK (email_visibility IN ('public', 'connections_only', 'private')),
    CHECK (badge_visibility IN ('public', 'connections_only', 'private')),
    CHECK (activity_visibility IN ('public', 'connections_only', 'private')),
    CHECK (allow_messages_from IN ('everyone', 'connections_only', 'none'))
);

-- ----------------------------------------------------------------------------
-- 7. Notification Settings
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.users_notification_settings (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Email Notifications
    email_badge_expiry BOOLEAN NOT NULL DEFAULT true,
    email_role_election BOOLEAN NOT NULL DEFAULT true,
    email_community_invitation BOOLEAN NOT NULL DEFAULT true,
    email_event_reminder BOOLEAN NOT NULL DEFAULT true,
    email_forum_reply BOOLEAN NOT NULL DEFAULT false,
    email_new_message BOOLEAN NOT NULL DEFAULT true,
    
    -- Push Notifications
    push_enabled BOOLEAN NOT NULL DEFAULT false,
    push_badge_expiry BOOLEAN NOT NULL DEFAULT true,
    push_new_message BOOLEAN NOT NULL DEFAULT true,
    push_event_reminder BOOLEAN NOT NULL DEFAULT true,
    
    -- In-App Notifications
    in_app_enabled BOOLEAN NOT NULL DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ----------------------------------------------------------------------------
-- 8. Communities (Territory-specific)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Identity
    code VARCHAR(100) NOT NULL UNIQUE,         -- URL-safe: 'platform_dev', 'copenhagen_beekeepers'
    name VARCHAR(255) NOT NULL,
    description TEXT,
    slug VARCHAR(100) NOT NULL UNIQUE,         -- URL slug
    
    -- Type & Access (NEW - distinguishes territory vs guild communities)
    community_type VARCHAR(20) NOT NULL DEFAULT 'territory', -- 'territory', 'guild'
    join_policy VARCHAR(30) NOT NULL DEFAULT 'invite_only',  -- 'open', 'invite_only', 'badge_required', 'approval_required'
    access_badge_id UUID,                      -- Optional: Badge required to join (for badge_required policy)
    
    -- Hierarchy
    parent_community_id UUID REFERENCES territory_dk.communities(id) ON DELETE SET NULL,
    depth INT NOT NULL DEFAULT 0,              -- Tree depth (0 = root)
    
    -- Scope
    territory_code VARCHAR(10) NOT NULL DEFAULT 'dk',
    scope VARCHAR(20) NOT NULL DEFAULT 'territory', -- 'global', 'territory', 'community'
    scope_id UUID,                             -- NULL for global/territory, parent community UUID for community
    
    -- Display
    icon_url VARCHAR(500),
    color VARCHAR(7),                          -- Hex color
    
    -- Metadata
    metadata JSONB,                            -- Flexible JSON data
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (community_type IN ('territory', 'guild')),
    CHECK (join_policy IN ('open', 'invite_only', 'badge_required', 'approval_required')),
    CHECK (scope IN ('global', 'territory', 'community')),
    CHECK ((community_type = 'territory' AND join_policy = 'invite_only') OR community_type = 'guild'),
    CHECK ((scope = 'global' AND scope_id IS NULL AND territory_code IS NULL) OR 
           (scope = 'territory' AND scope_id IS NULL AND territory_code IS NOT NULL) OR
           (scope = 'community' AND scope_id IS NOT NULL))
);

CREATE INDEX idx_communities_parent ON territory_dk.communities(parent_community_id);
CREATE INDEX idx_communities_territory ON territory_dk.communities(territory_code);
CREATE INDEX idx_communities_slug ON territory_dk.communities(slug);
CREATE INDEX idx_communities_active ON territory_dk.communities(is_active);
CREATE INDEX idx_communities_depth ON territory_dk.communities(depth);
CREATE INDEX idx_communities_type ON territory_dk.communities(community_type);
CREATE INDEX idx_communities_badge ON territory_dk.communities(access_badge_id) WHERE access_badge_id IS NOT NULL;

-- ----------------------------------------------------------------------------
-- 9. Invitation Tokens (Territory-specific)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(64) UNIQUE NOT NULL,         -- Cryptographically random
    token_type VARCHAR(20) NOT NULL CHECK (token_type IN ('single_use', 'group')),
    
    -- Restrictions
    email VARCHAR(255),                        -- Optional: for email delivery, can be NULL for QR/link sharing
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
    CHECK (
        (token_type = 'single_use' AND max_uses = 1) OR
        (token_type = 'group' AND max_uses > 1)
    ),
    CHECK (used_count <= max_uses)
);

CREATE INDEX idx_invitation_tokens_token ON territory_dk.invitation_tokens(token);
CREATE INDEX idx_invitation_tokens_email ON territory_dk.invitation_tokens(email) WHERE email IS NOT NULL;
CREATE INDEX idx_invitation_tokens_created_by ON territory_dk.invitation_tokens(created_by_user_id);
CREATE INDEX idx_invitation_tokens_active ON territory_dk.invitation_tokens(is_active, expires_at);

-- ============================================================================
-- INVITATIONS (2 tables)
-- ============================================================================

-- ----------------------------------------------------------------------------
-- 10. Invitation Uses (Audit Log)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.invitation_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invitation_token_id UUID NOT NULL REFERENCES territory_dk.invitation_tokens(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (invitation_token_id, user_id)
);

CREATE INDEX idx_invitation_uses_token ON territory_dk.invitation_uses(invitation_token_id);
CREATE INDEX idx_invitation_uses_user ON territory_dk.invitation_uses(user_id);

-- ============================================================================
-- COMMUNITIES & GOVERNANCE (6 tables)
-- ============================================================================

-- ----------------------------------------------------------------------------
-- 11. Community Members
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.community_members (
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    role VARCHAR(20) NOT NULL DEFAULT 'member', -- 'founder', 'elected', 'member'
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    left_at TIMESTAMPTZ,
    
    PRIMARY KEY (user_id, community_id),
    
    CHECK (role IN ('founder', 'elected', 'member'))
);

CREATE INDEX idx_community_members_user ON territory_dk.community_members(user_id);
CREATE INDEX idx_community_members_community ON territory_dk.community_members(community_id);
CREATE INDEX idx_community_members_role ON territory_dk.community_members(role);
CREATE INDEX idx_community_members_active ON territory_dk.community_members(user_id, community_id) 
    WHERE left_at IS NULL;

-- ----------------------------------------------------------------------------
-- 12. Roles (Community Governance)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    code VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    scope VARCHAR(20) NOT NULL DEFAULT 'community', -- 'territory', 'community'
    is_electable BOOLEAN NOT NULL DEFAULT false,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (scope IN ('territory', 'community'))
);

CREATE INDEX idx_roles_scope ON territory_dk.roles(scope);
CREATE INDEX idx_roles_electable ON territory_dk.roles(is_electable);

-- ----------------------------------------------------------------------------
-- 13. Role Assignments
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.role_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES territory_dk.roles(id) ON DELETE CASCADE,
    community_id UUID REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    -- Assignment Details
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assigned_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    assignment_reason TEXT,
    
    -- Term Limits (for elected roles)
    term_starts_at TIMESTAMPTZ,
    term_ends_at TIMESTAMPTZ,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    revoked_at TIMESTAMPTZ,
    revoked_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    revocation_reason TEXT,
    
    -- Constraints
    UNIQUE (user_id, role_id, community_id, assigned_at),
    CHECK ((term_starts_at IS NULL AND term_ends_at IS NULL) OR 
           (term_starts_at IS NOT NULL AND term_ends_at IS NOT NULL AND term_ends_at > term_starts_at))
);

CREATE INDEX idx_role_assignments_user ON territory_dk.role_assignments(user_id);
CREATE INDEX idx_role_assignments_role ON territory_dk.role_assignments(role_id);
CREATE INDEX idx_role_assignments_community ON territory_dk.role_assignments(community_id);
CREATE INDEX idx_role_assignments_active ON territory_dk.role_assignments(user_id, role_id, community_id)
    WHERE is_active = true AND revoked_at IS NULL;

-- ----------------------------------------------------------------------------
-- 14. Community Role Elections
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.community_role_elections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES territory_dk.roles(id) ON DELETE CASCADE,
    
    -- Nominee
    nominee_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Election Period
    nomination_start TIMESTAMPTZ NOT NULL,
    nomination_end TIMESTAMPTZ NOT NULL,
    voting_start TIMESTAMPTZ NOT NULL,
    voting_end TIMESTAMPTZ NOT NULL,
    
    -- Results
    votes_for INT NOT NULL DEFAULT 0,
    votes_against INT NOT NULL DEFAULT 0,
    total_votes INT NOT NULL DEFAULT 0,
    
    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'voting', 'passed', 'failed', 'cancelled'
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    
    CHECK (status IN ('pending', 'voting', 'passed', 'failed', 'cancelled')),
    CHECK (nomination_start < nomination_end),
    CHECK (nomination_end <= voting_start),
    CHECK (voting_start < voting_end)
);

CREATE INDEX idx_elections_community ON territory_dk.community_role_elections(community_id);
CREATE INDEX idx_elections_nominee ON territory_dk.community_role_elections(nominee_user_id);
CREATE INDEX idx_elections_status ON territory_dk.community_role_elections(status);
CREATE INDEX idx_elections_active ON territory_dk.community_role_elections(community_id, status)
    WHERE status IN ('pending', 'voting');

-- ----------------------------------------------------------------------------
-- 15. Election Votes
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.community_role_election_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    election_id UUID NOT NULL REFERENCES territory_dk.community_role_elections(id) ON DELETE CASCADE,
    voter_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    vote BOOLEAN NOT NULL,                     -- true = for, false = against
    voted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- One vote per user per election
    UNIQUE (election_id, voter_user_id)
);

CREATE INDEX idx_election_votes_election ON territory_dk.community_role_election_votes(election_id);
CREATE INDEX idx_election_votes_voter ON territory_dk.community_role_election_votes(voter_user_id);

-- ============================================================================
-- BADGES & ACCESS (3 tables)
-- ============================================================================

-- ----------------------------------------------------------------------------
-- 16. Badge Definitions
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.badge_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Identity
    code VARCHAR(100) NOT NULL UNIQUE,         -- 'code_of_conduct', 'platform_management'
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Display
    icon_url VARCHAR(500),
    color VARCHAR(7),
    
    -- Scope
    scope VARCHAR(20) NOT NULL DEFAULT 'territory', -- 'global', 'territory', 'community'
    scope_id UUID,                             -- NULL for global/territory, community UUID for community
    
    -- Category
    category VARCHAR(50) NOT NULL,             -- 'governance', 'learning', 'skills', 'achievement'
    
    -- Behavior
    is_renewable BOOLEAN NOT NULL DEFAULT false,
    renewal_period_days INT,                   -- Days until expiration (NULL = never expires)
    requires_approval BOOLEAN NOT NULL DEFAULT false,
    
    -- Prerequisites
    prerequisite_badge_ids UUID[],             -- Array of badge IDs required before this one
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (scope IN ('global', 'territory', 'community')),
    CHECK ((scope = 'global' AND scope_id IS NULL) OR 
           (scope = 'territory' AND scope_id IS NULL) OR
           (scope = 'community' AND scope_id IS NOT NULL)),
    CHECK (NOT is_renewable OR renewal_period_days IS NOT NULL)
);

CREATE INDEX idx_badge_definitions_code ON territory_dk.badge_definitions(code);
CREATE INDEX idx_badge_definitions_category ON territory_dk.badge_definitions(category);
CREATE INDEX idx_badge_definitions_active ON territory_dk.badge_definitions(is_active);

-- ----------------------------------------------------------------------------
-- 17. Badge Awards (User Badge Ownership)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.badge_awards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    badge_id UUID NOT NULL REFERENCES territory_dk.badge_definitions(id) ON DELETE CASCADE,
    
    -- Award Details
    awarded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    awarded_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    award_reason TEXT,
    
    -- Metadata (e.g., course completion score)
    metadata JSONB,
    
    -- Expiration
    expires_at TIMESTAMPTZ,
    
    -- Revocation
    revoked_at TIMESTAMPTZ,
    revoked_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    revocation_reason TEXT,
    
    -- Renewal
    last_renewed_at TIMESTAMPTZ,
    renewal_count INT NOT NULL DEFAULT 0,
    
    -- Constraints
    CHECK (revoked_at IS NULL OR revoked_at > awarded_at),
    CHECK (expires_at IS NULL OR expires_at > awarded_at)
);

CREATE INDEX idx_badge_awards_user ON territory_dk.badge_awards(user_id);
CREATE INDEX idx_badge_awards_badge ON territory_dk.badge_awards(badge_id);
CREATE INDEX idx_badge_awards_expires ON territory_dk.badge_awards(expires_at) WHERE revoked_at IS NULL;
CREATE INDEX idx_badge_awards_active ON territory_dk.badge_awards(user_id, badge_id) 
    WHERE revoked_at IS NULL;
-- Note: Removed expires_at > NOW() from index predicate (NOW() not immutable)
-- Filter expiration in application layer or use materialized view

-- ----------------------------------------------------------------------------
-- 18. Badge Progress (Earning Progress Tracking)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.badge_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    badge_id UUID NOT NULL REFERENCES territory_dk.badge_definitions(id) ON DELETE CASCADE,
    
    -- Progress
    progress_percentage DECIMAL(5,2) NOT NULL DEFAULT 0.00,
    progress_data JSONB,                       -- Custom progress tracking (e.g., course lessons completed)
    
    -- Completion
    completed_at TIMESTAMPTZ,
    
    -- Timestamps
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (user_id, badge_id),
    CHECK (progress_percentage >= 0 AND progress_percentage <= 100)
);

CREATE INDEX idx_badge_progress_user ON territory_dk.badge_progress(user_id);
CREATE INDEX idx_badge_progress_badge ON territory_dk.badge_progress(badge_id);
CREATE INDEX idx_badge_progress_incomplete ON territory_dk.badge_progress(user_id, badge_id)
    WHERE completed_at IS NULL;

-- ============================================================================
-- GROUPS / BUBBLES SYSTEM (4 tables)
-- ============================================================================

-- ----------------------------------------------------------------------------
-- 19. Groups (Access Control Containers)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Identity
    code VARCHAR(100) NOT NULL UNIQUE,         -- 'platform_management', 'beekeepers_guild'
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Scope (who sees this group)
    scope VARCHAR(20) NOT NULL DEFAULT 'territory', -- 'global', 'territory', 'community'
    scope_id UUID,                             -- NULL for global/territory, community/territory UUID for scoped
    
    -- Access Control (THE KEY)
    access_badge_id UUID NOT NULL REFERENCES territory_dk.badge_definitions(id) ON DELETE CASCADE,
    
    -- Display
    icon_url VARCHAR(500),
    color VARCHAR(7),
    
    -- Metadata
    metadata JSONB,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (scope IN ('global', 'territory', 'community')),
    CHECK ((scope = 'global' AND scope_id IS NULL) OR 
           (scope IN ('territory', 'community') AND scope_id IS NOT NULL))
);

CREATE INDEX idx_groups_code ON territory_dk.groups(code);
CREATE INDEX idx_groups_scope ON territory_dk.groups(scope);
CREATE INDEX idx_groups_scope_id ON territory_dk.groups(scope_id);
CREATE INDEX idx_groups_access_badge ON territory_dk.groups(access_badge_id);
CREATE INDEX idx_groups_active ON territory_dk.groups(is_active);

-- ----------------------------------------------------------------------------
-- 20. Group Communities (Link groups to communities)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.group_communities (
    group_id UUID NOT NULL REFERENCES territory_dk.groups(id) ON DELETE CASCADE,
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    
    PRIMARY KEY (group_id, community_id)
);

CREATE INDEX idx_group_communities_group ON territory_dk.group_communities(group_id);
CREATE INDEX idx_group_communities_community ON territory_dk.group_communities(community_id);

-- ----------------------------------------------------------------------------
-- 21. Group Forums (Link groups to forums - future extension)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.group_forums (
    group_id UUID NOT NULL REFERENCES territory_dk.groups(id) ON DELETE CASCADE,
    forum_id UUID NOT NULL,                    -- Will reference forums table when created
    
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    
    PRIMARY KEY (group_id, forum_id)
);

CREATE INDEX idx_group_forums_group ON territory_dk.group_forums(group_id);
CREATE INDEX idx_group_forums_forum ON territory_dk.group_forums(forum_id);

-- ----------------------------------------------------------------------------
-- 22. Group Courses (Link groups to courses - future extension)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.group_courses (
    group_id UUID NOT NULL REFERENCES territory_dk.groups(id) ON DELETE CASCADE,
    course_id UUID NOT NULL,                   -- Will reference courses table when created
    
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    
    PRIMARY KEY (group_id, course_id)
);

CREATE INDEX idx_group_courses_group ON territory_dk.group_courses(group_id);
CREATE INDEX idx_group_courses_course ON territory_dk.group_courses(course_id);

-- ============================================================================
-- SOCIAL & COMMUNICATION (8 tables)
-- ============================================================================

-- ----------------------------------------------------------------------------
-- 23. Notifications
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Type & Content
    notification_type VARCHAR(50) NOT NULL,    -- 'badge_expiry_warning', 'role_election', 'event_reminder', etc.
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    
    -- Action
    action_url VARCHAR(500),                   -- URL to navigate to when clicked
    
    -- Related Entity
    related_entity_type VARCHAR(50),           -- 'badge', 'community', 'event', 'election', etc.
    related_entity_id UUID,
    
    -- Data
    data JSONB,                                -- Custom notification data
    
    -- Status
    is_read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,
    
    -- Expiration
    expires_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON territory_dk.notifications(user_id);
CREATE INDEX idx_notifications_unread ON territory_dk.notifications(user_id, is_read) WHERE is_read = FALSE;
CREATE INDEX idx_notifications_type ON territory_dk.notifications(notification_type);
CREATE INDEX idx_notifications_created ON territory_dk.notifications(created_at DESC);
CREATE INDEX idx_notifications_related ON territory_dk.notifications(related_entity_type, related_entity_id);
CREATE INDEX idx_notifications_expires ON territory_dk.notifications(expires_at) WHERE expires_at IS NOT NULL;

-- ----------------------------------------------------------------------------
-- 24. User Connections (Follow/Friend/Block)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.user_connections (
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    target_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Connection Type
    connection_type VARCHAR(20) NOT NULL,      -- 'follow', 'friend', 'block'
    
    -- Status (for 'friend' type)
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- 'pending', 'active', 'rejected'
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (user_id, target_user_id),
    
    CHECK (connection_type IN ('follow', 'friend', 'block')),
    CHECK (status IN ('pending', 'active', 'rejected')),
    CHECK (user_id != target_user_id)
);

CREATE INDEX idx_user_connections_user ON territory_dk.user_connections(user_id);
CREATE INDEX idx_user_connections_target ON territory_dk.user_connections(target_user_id);
CREATE INDEX idx_user_connections_type ON territory_dk.user_connections(connection_type);
CREATE INDEX idx_user_connections_pending ON territory_dk.user_connections(target_user_id, status) 
    WHERE connection_type = 'friend' AND status = 'pending';

-- ----------------------------------------------------------------------------
-- 25. Community Events
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.community_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    -- Event Details
    title VARCHAR(255) NOT NULL,
    description TEXT,
    event_type VARCHAR(20) NOT NULL,           -- 'in_person', 'virtual', 'hybrid'
    
    -- Schedule
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    
    -- Location
    location VARCHAR(500),                     -- Physical address or venue name
    virtual_link VARCHAR(500),                 -- Video call link
    
    -- RSVP
    capacity INT,                              -- Max attendees (NULL = unlimited)
    rsvp_deadline TIMESTAMPTZ,
    
    -- Access
    require_badge_id UUID REFERENCES territory_dk.badge_definitions(id) ON DELETE SET NULL,
    
    -- Recurrence
    is_recurring BOOLEAN NOT NULL DEFAULT false,
    recurrence_rule VARCHAR(255),              -- iCal RRULE format
    
    -- Metadata
    metadata JSONB,
    
    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'upcoming', -- 'draft', 'upcoming', 'ongoing', 'completed', 'cancelled'
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (event_type IN ('in_person', 'virtual', 'hybrid')),
    CHECK (status IN ('draft', 'upcoming', 'ongoing', 'completed', 'cancelled')),
    CHECK (ends_at > starts_at),
    CHECK (capacity IS NULL OR capacity > 0)
);

CREATE INDEX idx_community_events_community ON territory_dk.community_events(community_id);
CREATE INDEX idx_community_events_starts ON territory_dk.community_events(starts_at);
CREATE INDEX idx_community_events_status ON territory_dk.community_events(status);
CREATE INDEX idx_community_events_creator ON territory_dk.community_events(created_by_user_id);
CREATE INDEX idx_community_events_upcoming ON territory_dk.community_events(starts_at) 
    WHERE status = 'upcoming';

-- ----------------------------------------------------------------------------
-- 26. Event RSVPs
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.event_rsvps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES territory_dk.community_events(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Response
    response VARCHAR(20) NOT NULL,             -- 'going', 'maybe', 'not_going'
    guest_count INT NOT NULL DEFAULT 0,        -- Additional guests
    
    -- Notes
    notes TEXT,                                -- Dietary preferences, questions, etc.
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (event_id, user_id),
    
    CHECK (response IN ('going', 'maybe', 'not_going')),
    CHECK (guest_count >= 0)
);

CREATE INDEX idx_event_rsvps_event ON territory_dk.event_rsvps(event_id);
CREATE INDEX idx_event_rsvps_user ON territory_dk.event_rsvps(user_id);
CREATE INDEX idx_event_rsvps_response ON territory_dk.event_rsvps(event_id, response);
CREATE INDEX idx_event_rsvps_going ON territory_dk.event_rsvps(event_id) 
    WHERE response = 'going';

-- ----------------------------------------------------------------------------
-- 27. File Uploads (IPFS Integration)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.file_uploads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- File Details
    original_filename VARCHAR(255) NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    
    -- Storage
    ipfs_hash VARCHAR(128),                    -- IPFS CID
    storage_url VARCHAR(500),                  -- Full URL (IPFS gateway or S3)
    storage_backend VARCHAR(20) NOT NULL,      -- 'ipfs', 's3', 'local'
    
    -- Metadata
    width INT,                                 -- For images
    height INT,                                -- For images
    duration_seconds INT,                      -- For audio/video
    
    -- Entity Association
    entity_type VARCHAR(50),                   -- 'user_avatar', 'community_logo', 'event_image', 'post_attachment'
    entity_id UUID,
    
    -- Security
    uploaded_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    is_virus_scanned BOOLEAN NOT NULL DEFAULT false,
    virus_scan_result VARCHAR(20),             -- 'clean', 'infected', 'error'
    
    -- Status
    upload_status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'completed', 'failed'
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (file_size_bytes > 0),
    CHECK (storage_backend IN ('ipfs', 's3', 'local')),
    CHECK (upload_status IN ('pending', 'completed', 'failed')),
    CHECK (virus_scan_result IS NULL OR virus_scan_result IN ('clean', 'infected', 'error'))
);

CREATE INDEX idx_file_uploads_user ON territory_dk.file_uploads(uploaded_by_user_id);
CREATE INDEX idx_file_uploads_entity ON territory_dk.file_uploads(entity_type, entity_id);
CREATE INDEX idx_file_uploads_ipfs ON territory_dk.file_uploads(ipfs_hash) WHERE ipfs_hash IS NOT NULL;
CREATE INDEX idx_file_uploads_status ON territory_dk.file_uploads(upload_status);
CREATE INDEX idx_file_uploads_created ON territory_dk.file_uploads(created_at DESC);

-- ----------------------------------------------------------------------------
-- 28. Content Reports (User-Generated Reports)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.content_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Reporter
    reported_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Reported Content
    reported_entity_type VARCHAR(50) NOT NULL, -- 'user', 'community', 'post', 'comment', etc.
    reported_entity_id UUID NOT NULL,
    reported_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL, -- If reporting a user or user's content
    
    -- Report Details
    report_reason VARCHAR(50) NOT NULL,        -- 'spam', 'harassment', 'inappropriate', 'misinformation', etc.
    description TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'medium', -- 'low', 'medium', 'high', 'critical'
    
    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'reviewing', 'resolved', 'dismissed'
    
    -- Resolution
    resolved_at TIMESTAMPTZ,
    resolved_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    resolution_notes TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (report_reason IN ('spam', 'harassment', 'inappropriate', 'misinformation', 'other')),
    CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    CHECK (status IN ('pending', 'reviewing', 'resolved', 'dismissed'))
);

CREATE INDEX idx_content_reports_reporter ON territory_dk.content_reports(reported_by_user_id);
CREATE INDEX idx_content_reports_entity ON territory_dk.content_reports(reported_entity_type, reported_entity_id);
CREATE INDEX idx_content_reports_user ON territory_dk.content_reports(reported_user_id);
CREATE INDEX idx_content_reports_status ON territory_dk.content_reports(status);
CREATE INDEX idx_content_reports_pending ON territory_dk.content_reports(status, created_at) 
    WHERE status IN ('pending', 'reviewing');

-- ----------------------------------------------------------------------------
-- 29. Moderation Actions (3-Strike System)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.moderation_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Moderator
    moderator_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Target
    target_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    content_report_id UUID REFERENCES territory_dk.content_reports(id) ON DELETE SET NULL,
    
    -- Action
    action_type VARCHAR(30) NOT NULL,          -- 'warning', 'delete_content', 'temporary_ban', 'permanent_ban'
    reason TEXT NOT NULL,
    
    -- Strike System
    is_strike BOOLEAN NOT NULL DEFAULT false,
    strike_count INT,                          -- Current strikes at time of action
    
    -- Duration (for bans)
    expires_at TIMESTAMPTZ,
    
    -- Visibility
    notify_user BOOLEAN NOT NULL DEFAULT true,
    public_note TEXT,                          -- Public explanation (visible to community)
    
    -- Metadata
    metadata JSONB,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (action_type IN ('warning', 'delete_content', 'temporary_ban', 'permanent_ban')),
    CHECK (NOT is_strike OR strike_count IS NOT NULL),
    CHECK (action_type != 'temporary_ban' OR expires_at IS NOT NULL)
);

CREATE INDEX idx_moderation_actions_moderator ON territory_dk.moderation_actions(moderator_user_id);
CREATE INDEX idx_moderation_actions_target_user ON territory_dk.moderation_actions(target_user_id);
CREATE INDEX idx_moderation_actions_report ON territory_dk.moderation_actions(content_report_id);
CREATE INDEX idx_moderation_actions_strikes ON territory_dk.moderation_actions(target_user_id, is_strike) 
    WHERE is_strike = true;
CREATE INDEX idx_moderation_actions_active ON territory_dk.moderation_actions(target_user_id, expires_at);
-- Note: Removed expires_at > NOW() from index predicate (NOW() not immutable)

-- ----------------------------------------------------------------------------
-- 30. Activities (Activity Feed / Timeline)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Actor
    actor_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Action
    activity_type VARCHAR(50) NOT NULL,        -- 'joined_community', 'posted_topic', 'earned_badge', etc.
    
    -- Object
    object_type VARCHAR(50) NOT NULL,          -- 'community', 'topic', 'badge', etc.
    object_id UUID NOT NULL,
    
    -- Context
    community_id UUID REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    -- Visibility
    visibility VARCHAR(20) NOT NULL DEFAULT 'public', -- 'public', 'connections_only', 'private'
    
    -- Metadata
    metadata JSONB,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (visibility IN ('public', 'connections_only', 'private'))
);

CREATE INDEX idx_activities_actor ON territory_dk.activities(actor_user_id);
CREATE INDEX idx_activities_community ON territory_dk.activities(community_id);
CREATE INDEX idx_activities_object ON territory_dk.activities(object_type, object_id);
CREATE INDEX idx_activities_created ON territory_dk.activities(created_at DESC);
CREATE INDEX idx_activities_public ON territory_dk.activities(created_at DESC) WHERE visibility = 'public';
CREATE INDEX idx_activities_community_feed ON territory_dk.activities(community_id, created_at DESC)
    WHERE visibility = 'public';

-- ============================================================================
-- AUDIT (1 table)
-- ============================================================================

-- ----------------------------------------------------------------------------
-- 31. Audit Log (System-Wide Event Tracking)
-- ----------------------------------------------------------------------------

CREATE TABLE territory_dk.audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Actor
    user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    
    -- Action
    action VARCHAR(100) NOT NULL,              -- 'user.created', 'badge.awarded', 'community.deleted'
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID,
    
    -- Context
    ip_address INET,
    user_agent TEXT,
    
    -- Data
    old_values JSONB,
    new_values JSONB,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_log_user ON territory_dk.audit_log(user_id);
CREATE INDEX idx_audit_log_entity ON territory_dk.audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_log_action ON territory_dk.audit_log(action);
CREATE INDEX idx_audit_log_created ON territory_dk.audit_log(created_at DESC);

-- ============================================================================
-- FUNCTIONS & TRIGGERS
-- ============================================================================

-- ----------------------------------------------------------------------------
-- Function: Update updated_at timestamp
-- ----------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION territory_dk.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to all tables with updated_at column
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON territory_dk.users
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_users_profiles_updated_at BEFORE UPDATE ON territory_dk.users_profiles
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_users_profile_links_updated_at BEFORE UPDATE ON territory_dk.users_profile_links
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_users_settings_updated_at BEFORE UPDATE ON territory_dk.users_settings
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_users_notification_settings_updated_at BEFORE UPDATE ON territory_dk.users_notification_settings
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_communities_updated_at BEFORE UPDATE ON territory_dk.communities
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_badge_definitions_updated_at BEFORE UPDATE ON territory_dk.badge_definitions
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_badge_progress_updated_at BEFORE UPDATE ON territory_dk.badge_progress
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_groups_updated_at BEFORE UPDATE ON territory_dk.groups
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_user_connections_updated_at BEFORE UPDATE ON territory_dk.user_connections
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_community_events_updated_at BEFORE UPDATE ON territory_dk.community_events
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

CREATE TRIGGER update_event_rsvps_updated_at BEFORE UPDATE ON territory_dk.event_rsvps
    FOR EACH ROW EXECUTE FUNCTION territory_dk.update_updated_at_column();

-- ============================================================================
-- INITIAL SEED DATA
-- ============================================================================

-- ----------------------------------------------------------------------------
-- Insert Denmark Territory
-- ----------------------------------------------------------------------------

INSERT INTO global.territories (code, name, display_name, description, pod_url, api_url, language_code, timezone, currency_code)
VALUES (
    'dk',
    'Denmark',
    'Denmark Territory',
    'Danish territory pod (example deployment)',
    'https://denmark.example.org',
    'https://api.denmark.example.org',
    'da',
    'Europe/Copenhagen',
    'DKK'
) ON CONFLICT (code) DO NOTHING;

-- ----------------------------------------------------------------------------
-- Insert Core Badge Definitions
-- ----------------------------------------------------------------------------

-- Code of Conduct Badge (Required for all users)
INSERT INTO territory_dk.badge_definitions (
    code,
    name,
    description,
    scope,
    category,
    is_renewable,
    renewal_period_days,
    requires_approval,
    is_active
) VALUES (
    'code_of_conduct',
    'Code of Conduct Agreement',
    'Signed and accepted the platform Code of Conduct',
    'global',
    'governance',
    true,
    365,
    false,
    true
) ON CONFLICT (code) DO NOTHING;

-- Platform Management Badge
INSERT INTO territory_dk.badge_definitions (
    code,
    name,
    description,
    scope,
    category,
    is_renewable,
    renewal_period_days,
    requires_approval,
    is_active
) VALUES (
    'platform_management',
    'Platform Management Access',
    'Access to platform development and management resources',
    'global',
    'governance',
    false,
    NULL,
    true,
    true
) ON CONFLICT (code) DO NOTHING;

-- ----------------------------------------------------------------------------
-- Insert Core Roles
-- ----------------------------------------------------------------------------

INSERT INTO territory_dk.roles (code, name, description, scope, is_electable)
VALUES 
    ('community_moderator', 'Community Moderator', 'Moderates community content and discussions', 'community', true),
    ('community_admin', 'Community Administrator', 'Manages community settings and members', 'community', true),
    ('territory_admin', 'Territory Administrator', 'Administers territory-wide settings', 'territory', false)
ON CONFLICT (code) DO NOTHING;

-- ============================================================================
-- COMPLETION
-- ============================================================================

-- Success message
DO $$
BEGIN
    RAISE NOTICE '========================================';
    RAISE NOTICE 'MVP Core Schema Migration Complete!';
    RAISE NOTICE '========================================';
    RAISE NOTICE 'Global Schema: 4 tables created';
    RAISE NOTICE 'Territory Schema (territory_dk): 30 tables created';
    RAISE NOTICE 'Triggers: updated_at automation enabled';
    RAISE NOTICE 'Seed Data: Denmark territory + core badges/roles';
    RAISE NOTICE '========================================';
    RAISE NOTICE 'Next Steps:';
    RAISE NOTICE '1. Create bootstrap invitation token';
    RAISE NOTICE '2. Register first user';
    RAISE NOTICE '3. Start backend service development';
    RAISE NOTICE '========================================';
END $$;
