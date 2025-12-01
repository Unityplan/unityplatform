-- Migration: 20251126141119_add_group_community_type.sql
-- Service: community-service
-- Description: Add 'group' community type for container communities
-- Author: AI Assistant
-- Date: 2025-11-26

-- ============================================================================
-- TERRITORY SCHEMA: Add Group Community Type
-- ============================================================================

-- The Group type is a container community that groups other communities together.
-- Unlike Zone/Neighborhood (physical) or Guild/StudyGroup (organizational),
-- a Group is purely for organization and visual grouping in the UI.
--
-- Use cases:
-- - Grouping related guilds/study groups under a topic (e.g., "Mycology" group)
-- - Creating badge-gated sections with multiple communities inside
-- - Organizing a large number of communities into manageable sections

-- Update the CHECK constraint to include 'group' type
ALTER TABLE territory_dk.community_communities 
DROP CONSTRAINT IF EXISTS communities_type_check;

ALTER TABLE territory_dk.community_communities 
ADD CONSTRAINT communities_type_check 
CHECK (type IN ('zone', 'neighborhood', 'guild', 'study_group', 'group'));

-- Add comment explaining the types
COMMENT ON COLUMN territory_dk.community_communities.type IS 
    'Community type: zone (regional), neighborhood (local physical), guild (skill-based), study_group (learning), group (container for other communities)';
