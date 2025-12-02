-- ============================================================================
-- Migration: 20251202000003_fix_community_service_fk_names
-- Level: Phase 1 - Service Independence (Fix)
-- Service: community-service  
-- Description: Remove FKs with correct constraint names from old table structure
-- Dependencies: 20251202000002_remove_community_service_fks.sql
-- ============================================================================

-- The rename migration (20251124171245) renamed tables but kept old constraint names
-- We need to drop FKs using the OLD constraint names

-- Remove FK from community_communities (old name: communities_created_by_fkey)
ALTER TABLE territory_dk.community_communities 
    DROP CONSTRAINT IF EXISTS communities_created_by_fkey;

-- Remove FK from community_communities_members (old name: community_members_user_id_fkey)
ALTER TABLE territory_dk.community_communities_members 
    DROP CONSTRAINT IF EXISTS community_members_user_id_fkey;

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251202000003 complete: Fixed community-service FK removal';
    RAISE NOTICE '    - Removed FK: communities_created_by_fkey (old constraint name)';
    RAISE NOTICE '    - Removed FK: community_members_user_id_fkey (old constraint name)';
    RAISE NOTICE '    - All 4 community-service FKs now removed';
    RAISE NOTICE '    - Progress: 7/15 FKs removed (47%% complete)';
END $$;
