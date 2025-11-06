-- Rollback: Remove invitation system
-- Version: 20251106000003

-- Remove column from users table
ALTER TABLE territory_dk.users DROP COLUMN IF EXISTS invited_by_token_id;

-- Drop triggers
DROP TRIGGER IF EXISTS trigger_update_invitation_token_updated_at ON territory_dk.invitation_tokens;
DROP FUNCTION IF EXISTS update_invitation_token_updated_at();

-- Drop tables (CASCADE will remove foreign key constraints)
DROP TABLE IF EXISTS territory_dk.invitation_uses CASCADE;
DROP TABLE IF EXISTS territory_dk.invitation_tokens CASCADE;
