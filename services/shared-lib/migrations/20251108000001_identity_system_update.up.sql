-- ============================================================================
-- Identity System Update Migration
-- Purpose: Implement privacy-first identity with global username uniqueness
-- Date: 2025-11-08
-- Reference: docs/architecture/identity-system.md
-- ============================================================================

-- Step 1: Add username column to global.user_identities
-- This becomes the globally unique human-readable identifier
ALTER TABLE global.user_identities 
ADD COLUMN username VARCHAR(50);

-- Step 2: Populate username from territory users (one-time data migration)
-- For existing users, copy username from their territory user record
UPDATE global.user_identities ui
SET username = tu.username
FROM territory_dk.users tu
WHERE ui.territory_user_id = tu.id 
  AND ui.territory_code = 'dk';

-- Step 3: Make username required and globally unique
ALTER TABLE global.user_identities 
ALTER COLUMN username SET NOT NULL;

CREATE UNIQUE INDEX idx_global_user_identities_username_lower 
ON global.user_identities(LOWER(username));

-- Step 4: Add comment explaining the column
COMMENT ON COLUMN global.user_identities.username IS 
'Globally unique username across all pods/territories. Used for federation (username@territory), Matrix ID (@username:unityplan.{territory}), and human-readable lookup. Never changes even during territory migration.';

-- Step 5: Make email optional in territory users
-- Remove NOT NULL constraint from email
ALTER TABLE territory_dk.users 
ALTER COLUMN email DROP NOT NULL;

-- Step 6: Update email index to allow NULL values
-- Drop old unique index and create new one that handles NULL
DROP INDEX IF EXISTS territory_dk.idx_territory_dk_users_email;
CREATE UNIQUE INDEX idx_territory_dk_users_email_unique 
ON territory_dk.users(email) 
WHERE email IS NOT NULL;

-- Step 7: Add comments explaining the changes
COMMENT ON COLUMN territory_dk.users.email IS 
'Optional email address for notifications only. NOT used for identity or authentication. User can register without email using invitation codes/QR codes.';

-- Step 8: Update public_key_hash generation (via trigger)
-- Drop old trigger that used email
DROP TRIGGER IF EXISTS trg_create_global_identity ON territory_dk.users;
DROP FUNCTION IF EXISTS create_global_user_identity();

-- Create new trigger function using username + territory + UUID
CREATE OR REPLACE FUNCTION create_global_user_identity()
RETURNS TRIGGER AS $$
DECLARE
    v_territory_code VARCHAR(100);
    v_public_key_hash VARCHAR(64);
    v_global_user_id UUID;
BEGIN
    -- Get territory code (hardcoded for now, could be dynamic)
    v_territory_code := 'dk';
    
    -- Generate new global UUID
    v_global_user_id := gen_random_uuid();
    
    -- Generate public key hash: SHA-256(username::territory::uuid)
    v_public_key_hash := encode(
        digest(
            NEW.username || '::' || v_territory_code || '::' || v_global_user_id::text,
            'sha256'
        ),
        'hex'
    );
    
    -- Insert into global.user_identities
    INSERT INTO global.user_identities (
        id,
        username,
        public_key_hash,
        territory_code,
        territory_user_id,
        created_at,
        updated_at
    ) VALUES (
        v_global_user_id,
        NEW.username,
        v_public_key_hash,
        v_territory_code,
        NEW.id,
        NOW(),
        NOW()
    );
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Recreate trigger
CREATE TRIGGER trg_create_global_identity
AFTER INSERT ON territory_dk.users
FOR EACH ROW
EXECUTE FUNCTION create_global_user_identity();

-- Step 9: Add index for federated identity lookup (username@territory)
CREATE INDEX idx_global_user_identities_username_territory 
ON global.user_identities(username, territory_code);

-- Step 10: Update invitation_tokens schema for optional email
COMMENT ON COLUMN territory_dk.invitation_tokens.invited_email IS 
'Optional email for targeted invitations. If NULL, invitation is a bearer token (anyone with code can use). Invitation can also be shared via QR code, messaging apps, etc.';

ALTER TABLE territory_dk.invitation_tokens 
ADD COLUMN invited_username VARCHAR(50);

COMMENT ON COLUMN territory_dk.invitation_tokens.invited_username IS 
'Optional username for targeted invitations. Can invite specific user by username even without email.';

CREATE INDEX idx_territory_dk_invitation_tokens_username 
ON territory_dk.invitation_tokens(invited_username);
