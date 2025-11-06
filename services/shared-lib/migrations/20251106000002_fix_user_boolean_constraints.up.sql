-- Migration: Fix user boolean field constraints
-- Version: 20251106000002
-- Description: Add NOT NULL constraints to boolean fields in territory_*.users tables
-- These fields should always have values (they have defaults)

-- Fix territory_dk.users boolean fields
ALTER TABLE territory_dk.users 
    ALTER COLUMN email_visible SET NOT NULL,
    ALTER COLUMN profile_public SET NOT NULL,
    ALTER COLUMN data_export_requested SET NOT NULL,
    ALTER COLUMN is_verified SET NOT NULL,
    ALTER COLUMN is_active SET NOT NULL;

-- Add comments explaining the constraints
COMMENT ON COLUMN territory_dk.users.email_visible IS 'Privacy: whether email is visible to other users (default: false)';
COMMENT ON COLUMN territory_dk.users.profile_public IS 'Privacy: whether profile is publicly visible (default: true)';
COMMENT ON COLUMN territory_dk.users.data_export_requested IS 'GDPR: user requested data export (default: false)';
COMMENT ON COLUMN territory_dk.users.is_verified IS 'Status: email/identity verified (default: false)';
COMMENT ON COLUMN territory_dk.users.is_active IS 'Status: account is active and can login (default: true)';
