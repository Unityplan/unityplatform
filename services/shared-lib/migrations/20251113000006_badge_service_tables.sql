-- Migration: 20251113000006_badge_service_tables.sql
-- Service: badge-service
-- Description: Create badges, user_badges, and badge_progress tables
-- Author: AI Assistant
-- Date: 2025-11-13

-- ============================================================================
-- GLOBAL SCHEMA: Badge Registry (Shared catalog across all pods)
-- ============================================================================

-- Badge definitions shared across all territories
CREATE TABLE IF NOT EXISTS global.badge_registry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Badge identity
    slug VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    icon VARCHAR(100) NOT NULL, -- Emoji or icon name
    
    -- Badge type and criteria
    category VARCHAR(50) NOT NULL, -- 'role', 'achievement', 'code_of_conduct'
    criteria_type VARCHAR(50) NOT NULL, -- 'manual', 'course_completion', 'follower_count', etc.
    criteria_value INT, -- e.g., 80 (min score), 10 (followers), null (manual)
    
    -- Badge properties
    rarity VARCHAR(20) NOT NULL DEFAULT 'common', -- 'common', 'rare', 'epic', 'legendary'
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_renewable BOOLEAN NOT NULL DEFAULT false, -- For Code of Conduct badge
    renewal_days INT, -- Days until renewal required (e.g., 365 for CoC)
    
    -- Permissions granted by this badge
    grants_permissions JSONB DEFAULT '[]'::jsonb, -- Array of permission strings
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (rarity IN ('common', 'rare', 'epic', 'legendary')),
    CHECK (category IN ('role', 'achievement', 'code_of_conduct', 'special')),
    CHECK (renewal_days IS NULL OR renewal_days > 0)
);

CREATE INDEX idx_badge_registry_slug ON global.badge_registry(slug);
CREATE INDEX idx_badge_registry_category ON global.badge_registry(category);
CREATE INDEX idx_badge_registry_active ON global.badge_registry(is_active) WHERE is_active = true;

COMMENT ON TABLE global.badge_registry IS 
    'Global badge catalog shared across all territory pods. Defines badge types, criteria, and permissions.';

COMMENT ON COLUMN global.badge_registry.grants_permissions IS 
    'Array of permission strings this badge grants, e.g., ["create_post", "moderate_forum"]';

-- ============================================================================
-- TERRITORY SCHEMA: Badge Awards (User achievements per territory)
-- ============================================================================

-- User badge awards
CREATE TABLE IF NOT EXISTS territory_dk.user_badges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- References
    badge_id UUID NOT NULL, -- References global.badge_registry(id)
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Award info
    awarded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    awarded_by UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL, -- NULL = auto-awarded
    reason TEXT,
    
    -- Renewal (for renewable badges like Code of Conduct)
    expires_at TIMESTAMPTZ, -- NULL = never expires
    last_renewed_at TIMESTAMPTZ,
    renewal_count INT NOT NULL DEFAULT 0,
    
    -- Display
    is_featured BOOLEAN NOT NULL DEFAULT false, -- Show on user profile
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(user_id, badge_id) -- Can only earn each badge once
);

CREATE INDEX idx_user_badges_user ON territory_dk.user_badges(user_id);
CREATE INDEX idx_user_badges_badge ON territory_dk.user_badges(badge_id);
CREATE INDEX idx_user_badges_featured ON territory_dk.user_badges(user_id, is_featured) WHERE is_featured = true;
CREATE INDEX idx_user_badges_expires ON territory_dk.user_badges(expires_at) WHERE expires_at IS NOT NULL;

COMMENT ON TABLE territory_dk.user_badges IS 
    'Badges awarded to users. Each user can earn each badge once, with optional renewal for time-limited badges.';

COMMENT ON COLUMN territory_dk.user_badges.expires_at IS 
    'When this badge award expires (e.g., Code of Conduct badge expires after 365 days). NULL = never expires.';

-- Badge progress tracking
CREATE TABLE IF NOT EXISTS territory_dk.badge_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- References
    badge_id UUID NOT NULL, -- References global.badge_registry(id)
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Progress
    current_value INT NOT NULL DEFAULT 0,
    target_value INT NOT NULL,
    
    -- Timestamps
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(user_id, badge_id),
    CHECK (current_value >= 0),
    CHECK (target_value > 0)
);

CREATE INDEX idx_badge_progress_user ON territory_dk.badge_progress(user_id);
CREATE INDEX idx_badge_progress_badge ON territory_dk.badge_progress(badge_id);

COMMENT ON TABLE territory_dk.badge_progress IS 
    'Tracks user progress towards earning achievement badges (e.g., 7/10 followers for Social Butterfly badge).';

-- ============================================================================
-- SEED DATA: Essential Badge Definitions
-- ============================================================================

-- Code of Conduct Badge (Mandatory, Renewable)
INSERT INTO global.badge_registry (
    slug, name, description, icon, category, criteria_type, criteria_value,
    rarity, is_renewable, renewal_days, grants_permissions
) VALUES (
    'code-of-conduct',
    'Code of Conduct',
    'Completed the Code of Conduct training and agreed to community guidelines. Must be renewed annually.',
    '📜',
    'code_of_conduct',
    'course_completion',
    80, -- Minimum score required
    'common',
    true, -- Renewable
    365, -- Annual renewal
    '["create_post", "create_topic", "comment", "vote", "join_community"]'::jsonb
) ON CONFLICT (slug) DO NOTHING;

-- Platform Manager Badge (Role, Manual Award)
INSERT INTO global.badge_registry (
    slug, name, description, icon, category, criteria_type,
    rarity, grants_permissions
) VALUES (
    'platform-manager',
    'Platform Manager',
    'Manages global platform infrastructure and settings. Manually awarded by system administrators.',
    '🔧',
    'role',
    'manual',
    'legendary',
    '["manage_platform", "create_territory", "manage_territories", "assign_badges", "view_all_users"]'::jsonb
) ON CONFLICT (slug) DO NOTHING;

-- Territory Manager Badge (Role, Manual Award)
INSERT INTO global.badge_registry (
    slug, name, description, icon, category, criteria_type,
    rarity, grants_permissions
) VALUES (
    'territory-manager',
    'Territory Manager',
    'Manages territory-specific settings and features. Manually awarded by Platform Managers.',
    '🏛️',
    'role',
    'manual',
    'epic',
    '["manage_territory_settings", "view_territory_stats", "manage_communities"]'::jsonb
) ON CONFLICT (slug) DO NOTHING;

-- Community Manager Badge (Role, Manual Award)
INSERT INTO global.badge_registry (
    slug, name, description, icon, category, criteria_type,
    rarity, grants_permissions
) VALUES (
    'community-manager',
    'Community Manager',
    'Manages community settings and moderation. Awarded by Territory Managers.',
    '👥',
    'role',
    'manual',
    'rare',
    '["manage_community", "moderate_content", "assign_roles"]'::jsonb
) ON CONFLICT (slug) DO NOTHING;

-- Early Adopter Badge (Achievement, Auto-Award)
INSERT INTO global.badge_registry (
    slug, name, description, icon, category, criteria_type,
    rarity
) VALUES (
    'early-adopter',
    'Early Adopter',
    'One of the first users to join the platform. Automatically awarded during MVP phase.',
    '🌟',
    'achievement',
    'manual',
    'rare'
) ON CONFLICT (slug) DO NOTHING;

-- Social Butterfly Badge (Achievement, Auto-Award)
INSERT INTO global.badge_registry (
    slug, name, description, icon, category, criteria_type, criteria_value,
    rarity
) VALUES (
    'social-butterfly',
    'Social Butterfly',
    'Earned by having 10 or more followers in the community.',
    '🦋',
    'achievement',
    'follower_count',
    10,
    'common'
) ON CONFLICT (slug) DO NOTHING;

-- Invitation Champion Badge (Achievement, Auto-Award)
INSERT INTO global.badge_registry (
    slug, name, description, icon, category, criteria_type, criteria_value,
    rarity
) VALUES (
    'invitation-champion',
    'Invitation Champion',
    'Successfully invited 5 or more users to join the platform.',
    '🎫',
    'achievement',
    'invitation_count',
    5,
    'common'
) ON CONFLICT (slug) DO NOTHING;

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Update updated_at timestamp on badge_registry changes
CREATE OR REPLACE FUNCTION update_badge_registry_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_badge_registry_updated_at
    BEFORE UPDATE ON global.badge_registry
    FOR EACH ROW
    EXECUTE FUNCTION update_badge_registry_updated_at();

-- Update updated_at timestamp on user_badges changes
CREATE OR REPLACE FUNCTION update_user_badges_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_user_badges_updated_at
    BEFORE UPDATE ON territory_dk.user_badges
    FOR EACH ROW
    EXECUTE FUNCTION update_user_badges_updated_at();

-- ============================================================================
-- MIGRATION COMPLETE
-- ============================================================================

-- Log migration completion
DO $$
BEGIN
    RAISE NOTICE 'Migration 20251113000006_badge_service_tables.sql completed successfully';
    RAISE NOTICE 'Created tables: global.badge_registry, territory_dk.user_badges, territory_dk.badge_progress';
    RAISE NOTICE 'Seeded % badge definitions', (SELECT COUNT(*) FROM global.badge_registry);
END $$;
