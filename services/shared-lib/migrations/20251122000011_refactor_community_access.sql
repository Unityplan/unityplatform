-- Migration: 20251122000011_refactor_community_access.sql
-- Service: community-service
-- Description: Refactor community access to use badge-based requirements
-- Author: AI Assistant
-- Date: 2025-11-22

-- ============================================================================
-- TERRITORY SCHEMA: Community Access Refactoring
-- ============================================================================

-- 1. Update community_settings table
ALTER TABLE territory_dk.community_settings
    DROP COLUMN IF EXISTS allow_join_requests,
    DROP COLUMN IF EXISTS require_approval,
    DROP COLUMN IF EXISTS allow_invitations,
    DROP COLUMN IF EXISTS allow_public_posts,
    ADD COLUMN IF NOT EXISTS inherit_requirements BOOLEAN NOT NULL DEFAULT true;

-- 2. Create community_badge_requirements table
CREATE TABLE IF NOT EXISTS territory_dk.community_badge_requirements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    badge_id UUID NOT NULL REFERENCES global.registry_badge(id) ON DELETE CASCADE,
    
    -- Requirement Context
    -- 'view': Required to see content (defaults to CoC if not specified, but can be overridden)
    -- 'participate': Required to post/comment/join
    -- 'admin': Required to manage (usually handled by roles, but badges can grant roles)
    context VARCHAR(50) NOT NULL DEFAULT 'participate',
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(community_id, badge_id, context),
    CHECK (context IN ('view', 'participate', 'admin'))
);

CREATE INDEX idx_community_badge_req_community ON territory_dk.community_badge_requirements(community_id);
CREATE INDEX idx_community_badge_req_badge ON territory_dk.community_badge_requirements(badge_id);

COMMENT ON TABLE territory_dk.community_badge_requirements IS 
    'Links communities to badges required for access/participation.';
