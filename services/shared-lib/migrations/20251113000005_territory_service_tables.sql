-- Migration: 20251113000005_territory_service_tables.sql
-- Service: territory-service
-- Description: Create territory_settings, territory_managers, and territory_stats tables
-- Author: AI Assistant
-- Date: 2025-11-13

-- ============================================================================
-- TERRITORY_SETTINGS: Source of truth for territory configuration
-- ============================================================================

CREATE TABLE IF NOT EXISTS territory_dk.territory_territories_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Identity (SOURCE OF TRUTH - replicates to global.registry_territories)
    name VARCHAR(100) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    
    -- Localization (SOURCE OF TRUTH - replicates to global.registry_territories)
    language_code VARCHAR(10) NOT NULL,
    timezone VARCHAR(50) NOT NULL,
    currency_code VARCHAR(3),
    
    -- Registration
    registration_enabled BOOLEAN NOT NULL DEFAULT true,
    invitation_required BOOLEAN NOT NULL DEFAULT true,
    max_users INT NOT NULL DEFAULT 0,
    
    -- Features
    features_enabled JSONB NOT NULL DEFAULT '{
        "communities": true,
        "badges": true,
        "events": false,
        "courses": false,
        "forum": false
    }'::jsonb,
    
    -- Branding
    primary_color VARCHAR(7) DEFAULT '#2E7D32',
    logo_url VARCHAR(500),
    
    -- Timestamps
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE territory_dk.territory_territories_settings IS 
    'Territory-specific settings. Source of truth for sovereignty data that replicates to global.registry_territories.';

COMMENT ON COLUMN territory_dk.territory_territories_settings.name IS 
    'Territory name - replicates to global registry';
COMMENT ON COLUMN territory_dk.territory_territories_settings.display_name IS 
    'Display name - replicates to global registry';
COMMENT ON COLUMN territory_dk.territory_territories_settings.language_code IS 
    'Default language - replicates to global registry';
COMMENT ON COLUMN territory_dk.territory_territories_settings.timezone IS 
    'Default timezone - replicates to global registry';

-- Trigger: Replicate sovereignty data to global registry
CREATE OR REPLACE FUNCTION territory_dk.replicate_settings_to_global()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE global.registry_territories
    SET 
        name = NEW.name,
        display_name = NEW.display_name,
        description = NEW.description,
        language_code = NEW.language_code,
        timezone = NEW.timezone,
        currency_code = NEW.currency_code,
        updated_at = NOW()
    WHERE code = 'dk';
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_replicate_settings
AFTER UPDATE ON territory_dk.territory_territories_settings
FOR EACH ROW
EXECUTE FUNCTION territory_dk.replicate_settings_to_global();

COMMENT ON TRIGGER trigger_replicate_settings ON territory_dk.territory_territories_settings IS 
    'Automatically replicates sovereignty data to global.registry_territories when territory managers update settings';

-- ============================================================================
-- TERRITORY_MANAGERS: Track territory manager assignments
-- ============================================================================

CREATE TABLE IF NOT EXISTS territory_dk.territory_territories_managers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Assignment
    -- Reference GLOBAL registry to allow cross-pod managers (Strict Federation)
    user_id UUID NOT NULL REFERENCES global.registry_username(user_id) ON DELETE CASCADE,
    territory_code VARCHAR(10) NOT NULL,
    
    -- Audit Trail
    assigned_by UUID,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    notes TEXT,
    
    CONSTRAINT uq_territory_managers_user_territory UNIQUE(user_id, territory_code)
);

CREATE INDEX idx_territory_managers_user 
    ON territory_dk.territory_territories_managers(user_id);
CREATE INDEX idx_territory_managers_territory 
    ON territory_dk.territory_territories_managers(territory_code);

COMMENT ON TABLE territory_dk.territory_territories_managers IS 
    'Territory manager assignments. User needs BOTH a Territory Manager badge AND an entry here to manage territory settings.';

COMMENT ON COLUMN territory_dk.territory_territories_managers.assigned_by IS 
    'Platform Manager user_id who made the assignment';

-- ============================================================================
-- TERRITORY_STATS: Aggregated statistics
-- ============================================================================

CREATE TABLE IF NOT EXISTS territory_dk.territory_territories_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- User Statistics
    total_users INT NOT NULL DEFAULT 0,
    active_users_7d INT NOT NULL DEFAULT 0,
    active_users_30d INT NOT NULL DEFAULT 0,
    
    -- Content Statistics
    total_communities INT NOT NULL DEFAULT 0,
    total_posts INT NOT NULL DEFAULT 0,
    
    -- Storage Statistics
    storage_used_mb BIGINT NOT NULL DEFAULT 0,
    
    -- Metadata
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE territory_dk.territory_territories_stats IS 
    'Aggregated territory statistics. Read-only for territory-service, calculated by background jobs aggregating data from other services.';

COMMENT ON COLUMN territory_dk.territory_territories_stats.calculated_at IS 
    'Timestamp when statistics were last calculated';

-- ============================================================================
-- SEED DATA: Initialize Denmark territory settings
-- ============================================================================

INSERT INTO territory_dk.territory_territories_settings (
    name,
    display_name,
    description,
    language_code,
    timezone,
    currency_code,
    registration_enabled,
    invitation_required,
    max_users,
    features_enabled,
    primary_color
) VALUES (
    'Denmark',
    'Denmark Territory',
    'Primary territory for Denmark-based Unity Platform users',
    'da',
    'Europe/Copenhagen',
    'DKK',
    true,
    true,
    0, -- 0 = unlimited
    '{
        "communities": true,
        "badges": true,
        "events": false,
        "courses": false,
        "forum": false
    }'::jsonb,
    '#2E7D32'
) ON CONFLICT DO NOTHING;

-- Initialize stats table with single row
INSERT INTO territory_dk.territory_territories_stats (
    total_users,
    active_users_7d,
    active_users_30d,
    total_communities,
    total_posts,
    storage_used_mb
) VALUES (0, 0, 0, 0, 0, 0)
ON CONFLICT DO NOTHING;

-- ============================================================================
-- VERIFICATION
-- ============================================================================

DO $$
BEGIN
    -- Verify tables exist
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables 
                   WHERE table_schema = 'territory_dk' 
                   AND table_name = 'territory_territories_settings') THEN
        RAISE EXCEPTION 'territory_dk.territory_territories_settings was not created';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables 
                   WHERE table_schema = 'territory_dk' 
                   AND table_name = 'territory_territories_managers') THEN
        RAISE EXCEPTION 'territory_dk.territory_territories_managers was not created';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables 
                   WHERE table_schema = 'territory_dk' 
                   AND table_name = 'territory_territories_stats') THEN
        RAISE EXCEPTION 'territory_dk.territory_territories_stats was not created';
    END IF;
    
    -- Verify trigger exists
    IF NOT EXISTS (SELECT 1 FROM information_schema.triggers 
                   WHERE trigger_schema = 'territory_dk' 
                   AND trigger_name = 'trigger_replicate_settings') THEN
        RAISE EXCEPTION 'trigger_replicate_settings was not created';
    END IF;
    
    RAISE NOTICE '✅ Migration 20251113000005 completed successfully';
    RAISE NOTICE '   - Created territory_dk.territory_territories_settings (with replication trigger)';
    RAISE NOTICE '   - Created territory_dk.territory_territories_managers';
    RAISE NOTICE '   - Created territory_dk.territory_territories_stats';
    RAISE NOTICE '   - Seeded Denmark territory settings';
END $$;
