-- Migration: Auto-create user profile when user is created
-- This ensures every user has a corresponding profile entry
-- Fixes issue where users could exist without profiles, causing 404 errors

-- Function to create user profile with default values
CREATE OR REPLACE FUNCTION territory.create_user_profile()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO territory.user_profiles (
        user_id,
        theme,
        profile_visibility,
        show_email,
        show_real_name,
        allow_messages_from,
        metadata
    ) VALUES (
        NEW.id,
        'light',              -- Default light theme
        'public',             -- Default public visibility
        false,                -- Don't show email by default
        true,                 -- Show real name by default
        'everyone',           -- Allow messages from everyone by default
        '{}'::jsonb          -- Empty metadata
    )
    ON CONFLICT (user_id) DO NOTHING;  -- Prevent duplicate profile creation
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to auto-create profile when user is created
CREATE TRIGGER trg_create_user_profile
    AFTER INSERT ON territory.users
    FOR EACH ROW
    EXECUTE FUNCTION territory.create_user_profile();

-- Backfill: Create profiles for existing users that don't have one
INSERT INTO territory.user_profiles (
    user_id,
    theme,
    profile_visibility,
    show_email,
    show_real_name,
    allow_messages_from,
    metadata
)
SELECT 
    u.id,
    'light',
    'public',
    false,
    true,
    'everyone',
    '{}'::jsonb
FROM territory.users u
LEFT JOIN territory.user_profiles p ON p.user_id = u.id
WHERE p.user_id IS NULL;

COMMENT ON FUNCTION territory.create_user_profile() IS 'Automatically creates a user profile entry when a new user is created';
COMMENT ON TRIGGER trg_create_user_profile ON territory.users IS 'Ensures every user has a corresponding profile';
