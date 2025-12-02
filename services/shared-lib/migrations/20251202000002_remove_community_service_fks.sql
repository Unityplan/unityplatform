-- ============================================================================
-- Migration: 20251202000002_remove_community_service_fks
-- Level: Phase 1 - Service Independence
-- Service: community-service  
-- Description: Remove foreign keys to auth_users_core, use JWT validation
-- Dependencies: 20251124171245_rename_community_tables_and_add_managers.sql
-- ============================================================================

-- Remove FK constraints from community_communities
ALTER TABLE territory_dk.community_communities 
    DROP CONSTRAINT IF EXISTS community_communities_created_by_fkey;

-- Remove FK constraints from community_communities_managers
ALTER TABLE territory_dk.community_communities_managers 
    DROP CONSTRAINT IF EXISTS community_communities_managers_user_id_fkey;

ALTER TABLE territory_dk.community_communities_managers 
    DROP CONSTRAINT IF EXISTS community_communities_managers_assigned_by_fkey;

-- Remove FK constraint from community_communities_members
ALTER TABLE territory_dk.community_communities_members 
    DROP CONSTRAINT IF EXISTS community_communities_members_user_id_fkey;

-- Add comments documenting validation strategy
COMMENT ON COLUMN territory_dk.community_communities.created_by IS 
    'User ID of community creator. Validated via JWT token. 
     NULL if creator account deleted. No FK constraint for service independence.';

COMMENT ON COLUMN territory_dk.community_communities_managers.user_id IS 
    'User ID of community manager. Validated via JWT token (primary) or global.registry_username (admin operations). 
     Manager must also have community-manager badge. No FK constraint for service independence.';

COMMENT ON COLUMN territory_dk.community_communities_managers.assigned_by IS 
    'User ID of admin who assigned this manager. Validated via JWT token. 
     NULL if assigned by system. No FK constraint for service independence.';

COMMENT ON COLUMN territory_dk.community_communities_members.user_id IS 
    'User ID of community member. Validated via JWT token. 
     No FK constraint for service independence.';

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251202000002 complete: Removed community-service FKs';
    RAISE NOTICE '    - Removed FK: community_communities.created_by -> auth_users_core.id';
    RAISE NOTICE '    - Removed FK: community_communities_managers.user_id -> auth_users_core.id';
    RAISE NOTICE '    - Removed FK: community_communities_managers.assigned_by -> auth_users_core.id';
    RAISE NOTICE '    - Removed FK: community_communities_members.user_id -> auth_users_core.id';
    RAISE NOTICE '    - Validation: JWT-based (zero DB queries)';
    RAISE NOTICE '    - Service independence achieved for community-service';
    RAISE NOTICE '    - Progress: 7/15 FKs removed (47%% complete)';
    RAISE NOTICE '    - Remaining cross-service FKs: 8 (all in user-service)';
END $$;
