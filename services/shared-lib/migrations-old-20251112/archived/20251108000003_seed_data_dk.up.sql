-- ============================================================================
-- UnityPlan Territory Seed Data - Denmark
-- Version: 0.1.0-alpha.1
-- Date: 2025-11-08
-- 
-- Territory-specific seed data for Denmark pod.
-- Each territory pod should have its own version of this file.
-- 
-- This file should be customized per territory:
-- - Territory registration in global.territories
-- - Territory settings (timezone, locale, language)
-- ============================================================================

-- Insert Denmark territory into global registry
INSERT INTO global.territories (code, name, type, parent_territory_code, pod_id, timezone, locale, default_language) 
VALUES ('dk', 'Denmark', 'country', NULL, 'dk', 'Europe/Copenhagen', 'da_DK', 'da')
ON CONFLICT (code) DO NOTHING;

-- Insert default territory settings for Denmark
INSERT INTO territory.settings (key, value) VALUES
    ('territory_code', '"dk"'::jsonb),
    ('language', '"da"'::jsonb),
    ('timezone', '"Europe/Copenhagen"'::jsonb),
    ('locale', '"da_DK"'::jsonb)
ON CONFLICT (key) DO NOTHING;
