-- Migration: 20251122000013_split_physical_community_type.sql
-- Service: community-service
-- Description: Split 'physical' community type into 'zone' (structure) and 'neighborhood' (people)
-- Author: AI Assistant
-- Date: 2025-11-22

-- Drop the existing check constraint
ALTER TABLE territory_dk.communities
    DROP CONSTRAINT IF EXISTS communities_type_check;

-- Update existing 'physical' communities to 'neighborhood' (assuming they have people)
UPDATE territory_dk.communities
SET type = 'neighborhood'
WHERE type = 'physical';

-- Add the new check constraint
ALTER TABLE territory_dk.communities
    ADD CONSTRAINT communities_type_check 
    CHECK (type::text = ANY (ARRAY['zone'::character varying, 'neighborhood'::character varying, 'guild'::character varying, 'study_group'::character varying]::text[]));
