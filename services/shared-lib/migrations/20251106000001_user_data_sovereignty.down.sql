-- Rollback user data sovereignty migration
-- Reverts to old architecture (global.users with personal data)

-- Drop territory users table
DROP TRIGGER IF EXISTS sync_territory_dk_user_identity ON territory_dk.users;
DROP TRIGGER IF EXISTS update_territory_dk_users_updated_at ON territory_dk.users;
DROP FUNCTION IF EXISTS sync_user_identity();
DROP FUNCTION IF EXISTS update_territory_user_updated_at();

DROP TABLE IF EXISTS territory_dk.refresh_tokens CASCADE;
DROP TABLE IF EXISTS territory_dk.users CASCADE;

-- Drop global role assignments
DROP TABLE IF EXISTS global.role_assignments CASCADE;
DROP TABLE IF EXISTS global.territory_managers CASCADE;

-- Drop sessions
DROP TABLE IF EXISTS global.sessions CASCADE;

-- Restore audit_log structure
ALTER TABLE global.audit_log 
    DROP CONSTRAINT IF EXISTS audit_log_public_key_hash_fkey;

ALTER TABLE global.audit_log 
    RENAME COLUMN public_key_hash TO user_id;

ALTER TABLE global.audit_log 
    ALTER COLUMN user_id TYPE UUID USING NULL;

-- Drop user_identities
DROP TABLE IF EXISTS global.user_identities CASCADE;

-- Recreate old global.users table (incorrect architecture)
CREATE TABLE global.users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    avatar_url TEXT,
    bio TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_global_users_email ON global.users(email);
CREATE INDEX idx_global_users_username ON global.users(username);
CREATE INDEX idx_global_users_created_at ON global.users(created_at);

-- Note: This rollback loses data sovereignty principles
-- Only use for development/testing purposes
