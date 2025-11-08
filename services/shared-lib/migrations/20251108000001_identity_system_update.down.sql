-- ============================================================================
-- Rollback Identity System Update Migration
-- Purpose: Revert to email-based identity system
-- Date: 2025-11-08
-- ============================================================================

-- Step 1: Remove username-based indexes
DROP INDEX IF EXISTS global.idx_global_user_identities_username_lower;
DROP INDEX IF EXISTS global.idx_global_user_identities_username_territory;
DROP INDEX IF EXISTS territory_dk.idx_territory_dk_invitation_tokens_username;

-- Step 2: Remove invited_username from invitation_tokens
ALTER TABLE territory_dk.invitation_tokens 
DROP COLUMN IF EXISTS invited_username;

-- Step 3: Restore old trigger for public_key_hash (email-based)
DROP TRIGGER IF EXISTS trg_create_global_identity ON territory_dk.users;
DROP FUNCTION IF EXISTS create_global_user_identity();

-- Recreate old email-based trigger (from original migration)
CREATE OR REPLACE FUNCTION create_global_user_identity()
RETURNS TRIGGER AS $$
DECLARE
    v_territory_code VARCHAR(100);
    v_public_key_hash VARCHAR(64);
BEGIN
    v_territory_code := 'dk';
    
    v_public_key_hash := encode(
        digest(
            COALESCE(NEW.email, 'no-email') || '::' || NEW.username || '::unityplan',
            'sha256'
        ),
        'hex'
    );
    
    INSERT INTO global.user_identities (
        public_key_hash,
        territory_code,
        territory_user_id,
        created_at,
        updated_at
    ) VALUES (
        v_public_key_hash,
        v_territory_code,
        NEW.id,
        NOW(),
        NOW()
    );
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_create_global_identity
AFTER INSERT ON territory_dk.users
FOR EACH ROW
EXECUTE FUNCTION create_global_user_identity();

-- Step 4: Restore email unique index
DROP INDEX IF EXISTS territory_dk.idx_territory_dk_users_email_unique;
CREATE UNIQUE INDEX idx_territory_dk_users_email 
ON territory_dk.users(email);

-- Step 5: Make email required again (if all users have email)
-- WARNING: This will fail if any users have NULL email
-- Commented out to prevent data loss - uncomment only if safe
-- ALTER TABLE territory_dk.users ALTER COLUMN email SET NOT NULL;

-- Step 6: Remove username from global.user_identities
ALTER TABLE global.user_identities 
DROP COLUMN IF EXISTS username;
