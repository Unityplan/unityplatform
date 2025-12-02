-- ============================================================================
-- Migration: 20251202000001_remove_badge_service_fks
-- Level: Phase 1 - Service Independence
-- Service: badge-service  
-- Description: Remove foreign keys to auth_users_core, use JWT validation
-- Dependencies: 20251113000006_badge_service_tables.sql
-- ============================================================================

-- Remove FK constraints from badge_users_badges
ALTER TABLE territory_dk.badge_users_badges 
    DROP CONSTRAINT IF EXISTS badge_users_badges_user_id_fkey;

ALTER TABLE territory_dk.badge_users_badges 
    DROP CONSTRAINT IF EXISTS badge_users_badges_awarded_by_fkey;

-- Remove FK constraint from badge_users_progress
ALTER TABLE territory_dk.badge_users_progress 
    DROP CONSTRAINT IF EXISTS badge_users_progress_user_id_fkey;

-- Add comments documenting validation strategy
COMMENT ON COLUMN territory_dk.badge_users_badges.user_id IS 
    'User ID validated via JWT token (primary method) or global.registry_username (admin operations). 
     No FK constraint for service independence. JWT signature ensures user exists.';

COMMENT ON COLUMN territory_dk.badge_users_badges.awarded_by IS 
    'User ID of admin who awarded the badge. Validated via JWT token. 
     NULL = auto-awarded by system. No FK constraint for service independence.';

COMMENT ON COLUMN territory_dk.badge_users_progress.user_id IS 
    'User ID validated via JWT token. No FK constraint for service independence.';

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251202000001 complete: Removed badge-service FKs';
    RAISE NOTICE '    - Removed FK: badge_users_badges.user_id -> auth_users_core.id';
    RAISE NOTICE '    - Removed FK: badge_users_badges.awarded_by -> auth_users_core.id';
    RAISE NOTICE '    - Removed FK: badge_users_progress.user_id -> auth_users_core.id';
    RAISE NOTICE '    - Validation: JWT-based (zero DB queries)';
    RAISE NOTICE '    - Service independence achieved for badge-service';
    RAISE NOTICE '    - Remaining cross-service FKs: 12 (user: 7, community: 5)';
END $$;
