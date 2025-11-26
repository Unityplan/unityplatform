-- Rename community tables to follow naming convention
ALTER TABLE territory_dk.communities RENAME TO community_communities;
ALTER TABLE territory_dk.community_members RENAME TO community_communities_members;
ALTER TABLE territory_dk.community_settings RENAME TO community_communities_settings;
ALTER TABLE territory_dk.community_badge_requirements RENAME TO community_communities_badge_requirements;

-- Create new table for community managers
CREATE TABLE territory_dk.community_communities_managers (
    community_id UUID NOT NULL REFERENCES territory_dk.community_communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assigned_by UUID REFERENCES territory_dk.auth_users_core(id) ON DELETE SET NULL,
    PRIMARY KEY (community_id, user_id)
);

-- Create index for managers
CREATE INDEX idx_community_managers_user ON territory_dk.community_communities_managers(user_id);

