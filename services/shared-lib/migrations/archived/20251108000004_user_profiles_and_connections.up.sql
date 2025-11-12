-- User Profiles Table (Extended user data)
CREATE TABLE IF NOT EXISTS territory.user_profiles (
    user_id UUID PRIMARY KEY REFERENCES global.users(id) ON DELETE CASCADE,
    bio TEXT,
    date_of_birth DATE,
    country VARCHAR(2),           -- ISO 3166-1 alpha-2
    city VARCHAR(100),
    timezone VARCHAR(50),          -- IANA timezone
    language_preference VARCHAR(10) DEFAULT 'en',
    notification_preferences JSONB DEFAULT '{
        "email_notifications": true,
        "push_notifications": true,
        "newsletter": false
    }'::jsonb,
    theme_preference VARCHAR(20) DEFAULT 'system',  -- system, light, dark
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_profiles_country ON territory.user_profiles(country);
CREATE INDEX idx_user_profiles_language ON territory.user_profiles(language_preference);

-- User Connections (Following/Followers)
CREATE TABLE IF NOT EXISTS territory.user_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES global.users(id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES global.users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_connection UNIQUE(follower_id, following_id),
    CONSTRAINT no_self_follow CHECK(follower_id != following_id)
);

CREATE INDEX idx_user_connections_follower ON territory.user_connections(follower_id);
CREATE INDEX idx_user_connections_following ON territory.user_connections(following_id);

-- User Blocks (Safety feature)
CREATE TABLE IF NOT EXISTS territory.user_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    blocker_id UUID NOT NULL REFERENCES global.users(id) ON DELETE CASCADE,
    blocked_id UUID NOT NULL REFERENCES global.users(id) ON DELETE CASCADE,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_block UNIQUE(blocker_id, blocked_id),
    CONSTRAINT no_self_block CHECK(blocker_id != blocked_id)
);

CREATE INDEX idx_user_blocks_blocker ON territory.user_blocks(blocker_id);
CREATE INDEX idx_user_blocks_blocked ON territory.user_blocks(blocked_id);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION territory.update_user_profile_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to auto-update updated_at
CREATE TRIGGER update_user_profiles_timestamp
    BEFORE UPDATE ON territory.user_profiles
    FOR EACH ROW
    EXECUTE FUNCTION territory.update_user_profile_timestamp();

-- Comments for documentation
COMMENT ON TABLE territory.user_profiles IS 'Extended user profile data stored in territory schema for data sovereignty';
COMMENT ON TABLE territory.user_connections IS 'User social graph - following/followers relationships';
COMMENT ON TABLE territory.user_blocks IS 'User blocking for safety - blocks prevent all interactions';
