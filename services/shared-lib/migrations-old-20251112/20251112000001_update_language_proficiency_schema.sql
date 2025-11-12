-- ============================================================================
-- Migration: 20251112000001_update_language_proficiency_schema.sql
-- Description: Update language proficiency table to match documentation
-- Created: 2025-11-12
-- Purpose: Add separate proficiency levels for spoken/written/reading/listening
--          and additional fields (language_name, show_on_profile, updated_at)
-- ============================================================================

-- Add new columns to users_language_proficiency
ALTER TABLE territory_dk.users_language_proficiency
    ADD COLUMN IF NOT EXISTS language_name VARCHAR(100),
    ADD COLUMN IF NOT EXISTS spoken_level VARCHAR(20),
    ADD COLUMN IF NOT EXISTS written_level VARCHAR(20),
    ADD COLUMN IF NOT EXISTS reading_level VARCHAR(20),
    ADD COLUMN IF NOT EXISTS listening_level VARCHAR(20),
    ADD COLUMN IF NOT EXISTS show_on_profile BOOLEAN NOT NULL DEFAULT true,
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Migrate existing proficiency_level data to the new columns (default all to same level)
UPDATE territory_dk.users_language_proficiency
SET 
    spoken_level = proficiency_level,
    written_level = proficiency_level,
    reading_level = proficiency_level,
    listening_level = proficiency_level,
    language_name = CASE language_code
        WHEN 'en' THEN 'English'
        WHEN 'da' THEN 'Dansk'
        WHEN 'no' THEN 'Norsk'
        WHEN 'sv' THEN 'Svenska'
        WHEN 'de' THEN 'Deutsch'
        WHEN 'fr' THEN 'Français'
        WHEN 'es' THEN 'Español'
        ELSE UPPER(language_code)
    END
WHERE spoken_level IS NULL;

-- Make new columns NOT NULL after data migration
ALTER TABLE territory_dk.users_language_proficiency
    ALTER COLUMN spoken_level SET NOT NULL,
    ALTER COLUMN written_level SET NOT NULL,
    ALTER COLUMN reading_level SET NOT NULL,
    ALTER COLUMN listening_level SET NOT NULL,
    ALTER COLUMN language_name SET NOT NULL;

-- Add CHECK constraints for proficiency levels
ALTER TABLE territory_dk.users_language_proficiency
    ADD CONSTRAINT check_spoken_level CHECK (spoken_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')),
    ADD CONSTRAINT check_written_level CHECK (written_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')),
    ADD CONSTRAINT check_reading_level CHECK (reading_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')),
    ADD CONSTRAINT check_listening_level CHECK (listening_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning'));

-- Drop old proficiency_level column and its constraint
ALTER TABLE territory_dk.users_language_proficiency
    DROP CONSTRAINT IF EXISTS users_language_proficiency_proficiency_level_check,
    DROP COLUMN IF EXISTS proficiency_level;

-- Update default value for spoken_level to 'basic' for new rows
ALTER TABLE territory_dk.users_language_proficiency
    ALTER COLUMN spoken_level SET DEFAULT 'basic',
    ALTER COLUMN written_level SET DEFAULT 'basic',
    ALTER COLUMN reading_level SET DEFAULT 'basic',
    ALTER COLUMN listening_level SET DEFAULT 'basic';
