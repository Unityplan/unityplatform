-- ============================================================================
-- Migration: 20251112000002_create_territory_schema_template.sql
-- Level: 1 (Core Identity)
-- Service: Core (shared-lib)
-- Description: Create territory schema template for Denmark (dk)
-- Dependencies: 20251112000001_create_global_schema.sql
-- ============================================================================

-- ============================================================================
-- Create Territory Schema
-- ============================================================================
-- This migration creates the territory_dk schema which contains all
-- territory-specific data following the data sovereignty principle.
-- Each pod has its own territory schema (territory_dk, territory_no, etc.)
-- ============================================================================

CREATE SCHEMA IF NOT EXISTS territory_dk;

COMMENT ON SCHEMA territory_dk IS 
    'Denmark territory schema - contains all DK-specific user data for data sovereignty';

-- ============================================================================
-- Success Message
-- ============================================================================

DO $$
BEGIN
    RAISE NOTICE '✅ Migration 20251112000002 complete: Territory schema (territory_dk) created';
END $$;

