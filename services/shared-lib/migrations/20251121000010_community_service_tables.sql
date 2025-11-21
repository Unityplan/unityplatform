-- Migration: 20251121000010_community_service_tables.sql
-- Service: community-service
-- Description: Create communities, community_members, and community_settings tables
-- Author: AI Assistant
-- Date: 2025-11-21

-- ============================================================================
-- TERRITORY SCHEMA: Community Service Tables
-- ============================================================================

-- Communities table
CREATE TABLE IF NOT EXISTS territory_dk.communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Identity
    slug VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    
    -- Type & Hierarchy
    type VARCHAR(50) NOT NULL, -- 'physical', 'guild', 'study_group'
    territory_id VARCHAR(50), -- Link to Territory Service ID (e.g., 'DK-CPH') for physical communities
    parent_community_id UUID REFERENCES territory_dk.communities(id) ON DELETE SET NULL,
    
    -- Visuals
    avatar_url TEXT,
    banner_url TEXT,
    
    -- Stats & Settings
    is_public BOOLEAN NOT NULL DEFAULT true,
    member_count INT NOT NULL DEFAULT 0,
    
    -- Metadata
    created_by UUID REFERENCES territory_dk.auth_users_core(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(slug),
    CHECK (type IN ('physical', 'guild', 'study_group'))
);

CREATE INDEX idx_communities_slug ON territory_dk.communities(slug);
CREATE INDEX idx_communities_type ON territory_dk.communities(type);
CREATE INDEX idx_communities_parent ON territory_dk.communities(parent_community_id);
CREATE INDEX idx_communities_territory ON territory_dk.communities(territory_id);

COMMENT ON TABLE territory_dk.communities IS 
    'Communities within the territory. Can be physical (linked to geography) or interest-based (guilds/bubbles).';

-- Community Members table
CREATE TABLE IF NOT EXISTS territory_dk.community_members (
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    
    -- Role & Status
    role VARCHAR(50) NOT NULL DEFAULT 'member', -- 'admin', 'moderator', 'member'
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    invited_by UUID REFERENCES territory_dk.auth_users_core(id) ON DELETE SET NULL,
    
    PRIMARY KEY (community_id, user_id),
    CHECK (role IN ('admin', 'moderator', 'member'))
);

CREATE INDEX idx_community_members_user ON territory_dk.community_members(user_id);
CREATE INDEX idx_community_members_role ON territory_dk.community_members(community_id, role);

COMMENT ON TABLE territory_dk.community_members IS 
    'Membership records for communities. Users can have different roles per community.';

-- Community Settings table
CREATE TABLE IF NOT EXISTS territory_dk.community_settings (
    community_id UUID PRIMARY KEY REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    -- Access Control
    allow_join_requests BOOLEAN NOT NULL DEFAULT true,
    require_approval BOOLEAN NOT NULL DEFAULT false,
    allow_invitations BOOLEAN NOT NULL DEFAULT true,
    
    -- Content Control
    allow_public_posts BOOLEAN NOT NULL DEFAULT true,
    
    -- Timestamps
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE territory_dk.community_settings IS 
    'Configuration settings for communities.';

-- Trigger to update member_count
CREATE OR REPLACE FUNCTION territory_dk.update_community_member_count()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        UPDATE territory_dk.communities
        SET member_count = member_count + 1
        WHERE id = NEW.community_id;
        RETURN NEW;
    ELSIF (TG_OP = 'DELETE') THEN
        UPDATE territory_dk.communities
        SET member_count = member_count - 1
        WHERE id = OLD.community_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_update_community_member_count ON territory_dk.community_members;
CREATE TRIGGER trg_update_community_member_count
AFTER INSERT OR DELETE ON territory_dk.community_members
FOR EACH ROW
EXECUTE FUNCTION territory_dk.update_community_member_count();

-- Update updated_at timestamp on communities changes
CREATE OR REPLACE FUNCTION territory_dk.update_communities_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_communities_updated_at ON territory_dk.communities;
CREATE TRIGGER trigger_communities_updated_at
    BEFORE UPDATE ON territory_dk.communities
    FOR EACH ROW
    EXECUTE FUNCTION territory_dk.update_communities_updated_at();

-- Log migration completion
DO $$
BEGIN
    RAISE NOTICE 'Migration 20251121000010_community_service_tables.sql completed successfully';
END $$;
