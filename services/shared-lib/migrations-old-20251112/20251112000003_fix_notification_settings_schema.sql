-- ============================================================================
-- Migration: 20251112000003_fix_notification_settings_schema.sql
-- Description: Replace users_notification_settings table with correct schema
-- Created: 2025-11-12
-- Purpose: Fix notification settings to match documentation specification
--          (old schema had different notification types)
-- ============================================================================

-- Drop existing users_notification_settings table (it has wrong schema)
DROP TABLE IF EXISTS territory_dk.users_notification_settings CASCADE;

-- Recreate users_notification_settings table matching documentation spec
CREATE TABLE territory_dk.users_notification_settings (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Email Notifications
    email_digest BOOLEAN NOT NULL DEFAULT true,
    email_messages BOOLEAN NOT NULL DEFAULT true,
    email_followers BOOLEAN NOT NULL DEFAULT true,
    email_community BOOLEAN NOT NULL DEFAULT false,
    email_updates BOOLEAN NOT NULL DEFAULT true,
    
    -- In-App Notifications
    inapp_messages BOOLEAN NOT NULL DEFAULT true,
    inapp_followers BOOLEAN NOT NULL DEFAULT true,
    inapp_community BOOLEAN NOT NULL DEFAULT true,
    inapp_mentions BOOLEAN NOT NULL DEFAULT true,
    inapp_likes BOOLEAN NOT NULL DEFAULT false,
    
    -- Push Notifications
    push_enabled BOOLEAN NOT NULL DEFAULT false,
    push_messages BOOLEAN NOT NULL DEFAULT false,
    push_followers BOOLEAN NOT NULL DEFAULT false,
    push_community BOOLEAN NOT NULL DEFAULT false,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE territory_dk.users_notification_settings IS 'User notification preferences for email, in-app, and push notifications';
COMMENT ON COLUMN territory_dk.users_notification_settings.email_digest IS 'Receive daily/weekly email digest of activity';
COMMENT ON COLUMN territory_dk.users_notification_settings.push_enabled IS 'Master switch for push notifications';
