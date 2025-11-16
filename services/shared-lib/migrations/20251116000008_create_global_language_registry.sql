-- Migration: 20251116000008_create_global_language_registry.sql
-- Service: territory-service
-- Description: Create global.registry_languages table for ISO 639-3 living languages
-- Author: AI Assistant
-- Date: 2025-11-16

-- ============================================================================
-- GLOBAL.REGISTRY_LANGUAGES: ISO 639-3 Language Registry
-- ============================================================================

CREATE TABLE IF NOT EXISTS global.registry_languages (
    -- ISO 639-3 standard fields
    language_code VARCHAR(3) PRIMARY KEY,
    language_name VARCHAR(200) NOT NULL,
    
    -- Alternative codes for compatibility
    iso639_2b VARCHAR(3),  -- ISO 639-2/B (bibliographic)
    iso639_2t VARCHAR(3),  -- ISO 639-2/T (terminological)
    iso639_1 VARCHAR(2),   -- ISO 639-1 (two-letter code when available)
    
    -- Classification
    language_scope VARCHAR(1) NOT NULL DEFAULT 'I',  -- I=Individual, M=Macrolanguage, S=Special
    language_type VARCHAR(1) NOT NULL DEFAULT 'L',   -- L=Living, E=Extinct, A=Ancient, H=Historic, C=Constructed
    
    -- Macro language relationship (e.g., 'nor' is macrolanguage for 'nob' and 'nno')
    part_of_macro VARCHAR(3) REFERENCES global.registry_languages(language_code),
    
    -- Metadata
    native_name VARCHAR(200),
    script_code VARCHAR(4),  -- ISO 15924 script code (e.g., 'Latn', 'Cyrl', 'Arab')
    is_active BOOLEAN NOT NULL DEFAULT true,
    
    -- Statistics (optional, can be updated periodically)
    speaker_count BIGINT,
    country_codes TEXT[],  -- Array of ISO 3166-1 alpha-2 country codes
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_registry_languages_iso639_1 ON global.registry_languages(iso639_1) WHERE iso639_1 IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_registry_languages_iso639_2b ON global.registry_languages(iso639_2b) WHERE iso639_2b IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_registry_languages_iso639_2t ON global.registry_languages(iso639_2t) WHERE iso639_2t IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_registry_languages_name ON global.registry_languages USING gin(to_tsvector('simple', language_name));
CREATE INDEX IF NOT EXISTS idx_registry_languages_active ON global.registry_languages(is_active) WHERE is_active = true;
CREATE INDEX IF NOT EXISTS idx_registry_languages_type ON global.registry_languages(language_type);
CREATE INDEX IF NOT EXISTS idx_registry_languages_macro ON global.registry_languages(part_of_macro) WHERE part_of_macro IS NOT NULL;

-- Comments
COMMENT ON TABLE global.registry_languages IS 
    'Global registry of living languages based on ISO 639-3 standard. Used for language selection across all territories.';

COMMENT ON COLUMN global.registry_languages.language_code IS 
    'ISO 639-3 three-letter language code (primary key)';

COMMENT ON COLUMN global.registry_languages.language_name IS 
    'English name of the language from ISO 639-3';

COMMENT ON COLUMN global.registry_languages.iso639_1 IS 
    'Two-letter ISO 639-1 code (e.g., "en", "da", "de") when available';

COMMENT ON COLUMN global.registry_languages.iso639_2b IS 
    'Three-letter ISO 639-2/B bibliographic code';

COMMENT ON COLUMN global.registry_languages.iso639_2t IS 
    'Three-letter ISO 639-2/T terminological code';

COMMENT ON COLUMN global.registry_languages.language_scope IS 
    'I=Individual language, M=Macrolanguage, S=Special';

COMMENT ON COLUMN global.registry_languages.language_type IS 
    'L=Living (actively spoken), E=Extinct, A=Ancient, H=Historic, C=Constructed';

COMMENT ON COLUMN global.registry_languages.part_of_macro IS 
    'Reference to macrolanguage if this is a specific language variant';

COMMENT ON COLUMN global.registry_languages.native_name IS 
    'Name of the language in the language itself (optional)';

COMMENT ON COLUMN global.registry_languages.script_code IS 
    'ISO 15924 script code (e.g., Latn=Latin, Cyrl=Cyrillic, Arab=Arabic)';

COMMENT ON COLUMN global.registry_languages.is_active IS 
    'Whether this language is available for selection (can be used to hide extinct/ancient languages)';

-- ============================================================================
-- SEED DATA: Most Common Languages
-- ============================================================================
-- Initial seed data will be added via separate seed script
-- This will include all living languages from ISO 639-3

-- For now, insert some common languages manually for testing
INSERT INTO global.registry_languages (language_code, language_name, iso639_1, iso639_2b, iso639_2t, language_scope, language_type, script_code, is_active) VALUES
    ('eng', 'English', 'en', 'eng', 'eng', 'I', 'L', 'Latn', true),
    ('dan', 'Danish', 'da', 'dan', 'dan', 'I', 'L', 'Latn', true),
    ('nor', 'Norwegian', 'no', 'nor', 'nor', 'M', 'L', 'Latn', true),
    ('nob', 'Norwegian Bokmål', 'nb', 'nob', 'nob', 'I', 'L', 'Latn', true),
    ('nno', 'Norwegian Nynorsk', 'nn', 'nno', 'nno', 'I', 'L', 'Latn', true),
    ('swe', 'Swedish', 'sv', 'swe', 'swe', 'I', 'L', 'Latn', true),
    ('deu', 'German', 'de', 'ger', 'deu', 'I', 'L', 'Latn', true),
    ('fra', 'French', 'fr', 'fre', 'fra', 'I', 'L', 'Latn', true),
    ('spa', 'Spanish', 'es', 'spa', 'spa', 'I', 'L', 'Latn', true),
    ('ita', 'Italian', 'it', 'ita', 'ita', 'I', 'L', 'Latn', true),
    ('nld', 'Dutch', 'nl', 'dut', 'nld', 'I', 'L', 'Latn', true),
    ('pol', 'Polish', 'pl', 'pol', 'pol', 'I', 'L', 'Latn', true),
    ('rus', 'Russian', 'ru', 'rus', 'rus', 'I', 'L', 'Cyrl', true),
    ('por', 'Portuguese', 'pt', 'por', 'por', 'I', 'L', 'Latn', true),
    ('jpn', 'Japanese', 'ja', 'jpn', 'jpn', 'I', 'L', 'Jpan', true),
    ('cmn', 'Mandarin Chinese', null, 'chi', 'cmn', 'I', 'L', 'Hans', true),
    ('zho', 'Chinese', 'zh', 'chi', 'zho', 'M', 'L', 'Hans', true),
    ('ara', 'Arabic', 'ar', 'ara', 'ara', 'M', 'L', 'Arab', true),
    ('hin', 'Hindi', 'hi', 'hin', 'hin', 'I', 'L', 'Deva', true),
    ('kor', 'Korean', 'ko', 'kor', 'kor', 'I', 'L', 'Kore', true),
    ('fin', 'Finnish', 'fi', 'fin', 'fin', 'I', 'L', 'Latn', true),
    ('isl', 'Icelandic', 'is', 'ice', 'isl', 'I', 'L', 'Latn', true),
    ('tur', 'Turkish', 'tr', 'tur', 'tur', 'I', 'L', 'Latn', true),
    ('vie', 'Vietnamese', 'vi', 'vie', 'vie', 'I', 'L', 'Latn', true),
    ('tha', 'Thai', 'th', 'tha', 'tha', 'I', 'L', 'Thai', true)
ON CONFLICT (language_code) DO NOTHING;

-- Update Norwegian macrolanguage relationships
UPDATE global.registry_languages SET part_of_macro = 'nor' WHERE language_code IN ('nob', 'nno');

-- ============================================================================
-- TRIGGER: Update timestamp
-- ============================================================================

CREATE OR REPLACE FUNCTION update_registry_languages_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_registry_languages_timestamp
    BEFORE UPDATE ON global.registry_languages
    FOR EACH ROW
    EXECUTE FUNCTION update_registry_languages_timestamp();
