-- Rollback Migration: Drop data_exports table
-- Created: 2025-11-12
-- Description: Removes GDPR data export functionality

-- Drop trigger first
DROP TRIGGER IF EXISTS trigger_set_export_expiry ON territory_dk.data_exports;

-- Drop function
DROP FUNCTION IF EXISTS territory_dk.set_export_expiry();

-- Drop indexes (will be dropped automatically with table, but explicit for clarity)
DROP INDEX IF EXISTS territory_dk.idx_data_exports_expires;
DROP INDEX IF EXISTS territory_dk.idx_data_exports_status;
DROP INDEX IF EXISTS territory_dk.idx_data_exports_user;

-- Drop table (CASCADE will remove foreign key constraints)
DROP TABLE IF EXISTS territory_dk.data_exports CASCADE;
