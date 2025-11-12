-- ============================================================================
-- Migration: 20251112000002_fix_users_settings_schema.sql
-- Description: Replace users_settings table with correct schema from documentation
-- Created: 2025-11-12
-- Purpose: Fix users_settings to include theme, language, and translation preferences
--          (old schema only had privacy settings which was incorrect)
-- ============================================================================

-- Drop existing users_settings table (it has wrong schema - only privacy settings)
DROP TABLE IF EXISTS territory_dk.users_settings CASCADE;

-- Recreate users_settings table matching documentation spec
CREATE TABLE territory_dk.users_settings (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Appearance Settings
    theme_mode VARCHAR(20) NOT NULL DEFAULT 'system',
    color_scheme VARCHAR(50) NOT NULL DEFAULT 'forest-green',
    reduced_motion BOOLEAN NOT NULL DEFAULT false,
    wide_content_view BOOLEAN NOT NULL DEFAULT false,
    compact_mode BOOLEAN NOT NULL DEFAULT false,
    
    -- Locale & Language Preferences (for UI/system, not public profile)
    preferred_language VARCHAR(10) NOT NULL DEFAULT 'en',  -- ISO 639-1 code (UI language)
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    
    -- Translation Settings (for consuming content in other languages)
    auto_translate BOOLEAN NOT NULL DEFAULT true,         -- Auto-translate content not in preferred language
    translation_provider VARCHAR(30) NOT NULL DEFAULT 'libre-translate',
    contribute_translations BOOLEAN NOT NULL DEFAULT true, -- Share local translations with community
    fallback_to_english BOOLEAN NOT NULL DEFAULT true,    -- Use English if preferred language unavailable
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (theme_mode IN ('light', 'dark', 'system')),
    CHECK (color_scheme IN ('forest-green', 'ocean-blue', 'royal-purple', 'custom')),
    CHECK (translation_provider IN ('libre-translate', 'deepl', 'google', 'microsoft'))
);

COMMENT ON TABLE territory_dk.users_settings IS 'User preferences for UI, appearance, and translation settings (private, not visible to other users)';
COMMENT ON COLUMN territory_dk.users_settings.preferred_language IS 'UI language preference (private) - different from language proficiency on public profile';
COMMENT ON COLUMN territory_dk.users_settings.auto_translate IS 'Whether to automatically translate content not in user''s preferred language';
