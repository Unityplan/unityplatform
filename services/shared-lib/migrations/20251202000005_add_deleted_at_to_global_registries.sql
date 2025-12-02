-- ============================================================================
-- Migration: 20251202000005_add_deleted_at_to_global_registries
-- Level: Phase 3 - Cleanup System
-- Description: Add deleted_at column to global registries for cleanup tracking
-- Dependencies: Previous migrations
-- ============================================================================

-- Add deleted_at to registry_username
ALTER TABLE global.registry_username 
    ADD COLUMN deleted_at TIMESTAMPTZ;

-- Add deleted_at to registry_email
ALTER TABLE global.registry_email 
    ADD COLUMN deleted_at TIMESTAMPTZ;

-- Add index for cleanup queries (find users deleted > X days ago)
CREATE INDEX idx_registry_username_deleted_at 
    ON global.registry_username(deleted_at) 
    WHERE deleted_at IS NOT NULL;

CREATE INDEX idx_registry_email_deleted_at 
    ON global.registry_email(deleted_at) 
    WHERE deleted_at IS NOT NULL;

-- Add comments
COMMENT ON COLUMN global.registry_username.deleted_at IS 
    'Timestamp when user requested account deletion. Used by task-scheduler for cleanup after 30 days.';

COMMENT ON COLUMN global.registry_email.deleted_at IS 
    'Timestamp when user requested account deletion. Synced with registry_username.deleted_at.';

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251202000005 complete: Added deleted_at to global registries';
    RAISE NOTICE '    - Added column: global.registry_username.deleted_at';
    RAISE NOTICE '    - Added column: global.registry_email.deleted_at';
    RAISE NOTICE '    - Added indexes for cleanup queries';
    RAISE NOTICE '    - Ready for task-scheduler cleanup operations';
END $$;
