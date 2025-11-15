-- ============================================================================
-- Migration: 20251112000001_create_global_schema.sql
-- Level: 0 (Foundation)
-- Service: Core (shared-lib)
-- Description: Create global schema and enable extensions
-- Dependencies: None
-- ============================================================================

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create global schema for cross-pod shared data
CREATE SCHEMA IF NOT EXISTS global;

COMMENT ON SCHEMA global IS 
    'Global schema for cross-pod shared data (territories, global registries, shared catalogs)';

-- ============================================================================
-- Global: Territories Registry
-- ============================================================================
-- Purpose: Pod/territory registry - which territories exist and their configuration
-- Owner: territory-service
-- Dependencies: None
-- ============================================================================

CREATE TABLE IF NOT EXISTS global.territories_registry (
    -- Primary key
    code VARCHAR(10) PRIMARY KEY,              -- 'dk', 'no', 'se', 'eu'
    
    -- Infrastructure (Platform Manager Only)
    pod_url VARCHAR(255) NOT NULL,             -- https://denmark.unityplatform.dk
    api_url VARCHAR(255) NOT NULL,             -- https://api.denmark.unityplatform.dk
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- 'active', 'maintenance', 'inactive'
    
    -- Territory Identity (Replicated from Pod)
    name VARCHAR(100) NOT NULL,                -- 'Denmark', 'Norway', etc.
    display_name VARCHAR(100) NOT NULL,        -- 'Denmark Territory'
    description TEXT,
    
    -- Localization (Replicated from Pod)
    language_code VARCHAR(10) NOT NULL,        -- 'da', 'no', 'sv', 'en'
    timezone VARCHAR(50) NOT NULL,             -- 'Europe/Copenhagen'
    currency_code VARCHAR(3),                  -- 'DKK', 'NOK', 'SEK'
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (status IN ('active', 'maintenance', 'inactive')),
    CHECK (char_length(code) >= 2 AND char_length(code) <= 10)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_territories_registry_status ON global.territories_registry(status);

-- Comments
COMMENT ON TABLE global.territories_registry IS 
    'Global territory registry. Platform managers manage infrastructure columns, territory managers manage sovereignty columns (replicated from pod settings).';
COMMENT ON COLUMN global.territories_registry.code IS 
    'Territory code: ISO 3166-1 Alpha-2 (dk, no, se) or custom (HAIDA-FN-CA)';
COMMENT ON COLUMN global.territories_registry.status IS 
    'Territory status: active (accepting users), maintenance (read-only), inactive (disabled)';

-- ============================================================================
-- Seed Data: Denmark Territory (MVP)
-- ============================================================================

INSERT INTO global.territories_registry (
    code, name, display_name, description,
    pod_url, api_url, status,
    language_code, timezone, currency_code
) VALUES (
    'dk',
    'Denmark',
    'Denmark Territory',
    'Primary territory for Denmark-based users',
    'https://denmark.unityplatform.dk',
    'https://api.denmark.unityplatform.dk',
    'active',
    'da',
    'Europe/Copenhagen',
    'DKK'
) ON CONFLICT (code) DO NOTHING;

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251112000001 complete: Global schema and territories_registry created';
END $$;
