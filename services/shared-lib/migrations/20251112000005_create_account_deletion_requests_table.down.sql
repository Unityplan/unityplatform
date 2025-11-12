-- Rollback Migration: Drop account_deletion_requests table
-- Created: 2025-11-12
-- Description: Removes GDPR account deletion functionality

-- Drop trigger first
DROP TRIGGER IF EXISTS trigger_set_deletion_schedule ON territory_dk.account_deletion_requests;

-- Drop function
DROP FUNCTION IF EXISTS territory_dk.set_deletion_schedule();

-- Drop indexes (will be dropped automatically with table, but explicit for clarity)
DROP INDEX IF EXISTS territory_dk.idx_account_deletion_requests_scheduled;
DROP INDEX IF EXISTS territory_dk.idx_account_deletion_requests_status;
DROP INDEX IF EXISTS territory_dk.idx_account_deletion_requests_user;

-- Drop table (CASCADE will remove foreign key constraints)
DROP TABLE IF EXISTS territory_dk.account_deletion_requests CASCADE;
