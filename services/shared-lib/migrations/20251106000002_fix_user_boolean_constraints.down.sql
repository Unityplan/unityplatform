-- Rollback: Remove NOT NULL constraints from boolean fields
-- Version: 20251106000002

-- Revert territory_dk.users boolean fields to nullable
ALTER TABLE territory_dk.users 
    ALTER COLUMN email_visible DROP NOT NULL,
    ALTER COLUMN profile_public DROP NOT NULL,
    ALTER COLUMN data_export_requested DROP NOT NULL,
    ALTER COLUMN is_verified DROP NOT NULL,
    ALTER COLUMN is_active DROP NOT NULL;

-- Remove comments
COMMENT ON COLUMN territory_dk.users.email_visible IS NULL;
COMMENT ON COLUMN territory_dk.users.profile_public IS NULL;
COMMENT ON COLUMN territory_dk.users.data_export_requested IS NULL;
COMMENT ON COLUMN territory_dk.users.is_verified IS NULL;
COMMENT ON COLUMN territory_dk.users.is_active IS NULL;
