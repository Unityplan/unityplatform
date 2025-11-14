-- Migration: Create users_settings table
-- Description: Store user preferences, privacy settings, and notification preferences
-- Date: 2025-11-13

-- Create users_settings table in territory_dk schema
CREATE TABLE IF NOT EXISTS territory_dk.users_settings (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- App preferences
    theme VARCHAR(20) NOT NULL DEFAULT 'system', -- 'light', 'dark', 'system'
    language VARCHAR(10) NOT NULL DEFAULT 'da',
    timezone VARCHAR(50) NOT NULL DEFAULT 'Europe/Copenhagen',
    
    -- Privacy settings
    profile_visibility VARCHAR(20) NOT NULL DEFAULT 'public', -- 'public', 'territory', 'private'
    show_email BOOLEAN NOT NULL DEFAULT false,
    show_location BOOLEAN NOT NULL DEFAULT true,
    allow_messages VARCHAR(20) NOT NULL DEFAULT 'everyone', -- 'everyone', 'connections', 'none'
    
    -- Notification preferences
    email_notifications BOOLEAN NOT NULL DEFAULT true,
    badge_notifications BOOLEAN NOT NULL DEFAULT true,
    course_notifications BOOLEAN NOT NULL DEFAULT true,
    forum_notifications BOOLEAN NOT NULL DEFAULT true,
    marketing_emails BOOLEAN NOT NULL DEFAULT false,
    
    -- Activity tracking
    show_activity BOOLEAN NOT NULL DEFAULT true,
    show_online_status BOOLEAN NOT NULL DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT valid_theme CHECK (theme IN ('light', 'dark', 'system')),
    CONSTRAINT valid_profile_visibility CHECK (profile_visibility IN ('public', 'territory', 'private')),
    CONSTRAINT valid_allow_messages CHECK (allow_messages IN ('everyone', 'connections', 'none'))
);

-- Create index for faster lookups
CREATE INDEX idx_users_settings_user_id ON territory_dk.users_settings(user_id);

-- Add updated_at trigger
CREATE TRIGGER update_users_settings_updated_at
    BEFORE UPDATE ON territory_dk.users_settings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments
COMMENT ON TABLE territory_dk.users_settings IS 'User preferences, privacy settings, and notification preferences';
COMMENT ON COLUMN territory_dk.users_settings.theme IS 'UI theme preference: light, dark, or system';
COMMENT ON COLUMN territory_dk.users_settings.profile_visibility IS 'Who can view the user profile: public, territory, or private';
COMMENT ON COLUMN territory_dk.users_settings.allow_messages IS 'Who can send messages: everyone, connections, or none';

-- Auto-create default settings when user registers (handled by application)
-- Settings are created with default values when user first accesses /user/settings
