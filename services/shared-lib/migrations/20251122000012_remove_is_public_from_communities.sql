-- Migration: 20251122000012_remove_is_public_from_communities.sql
-- Service: community-service
-- Description: Remove is_public column from communities table as it is replaced by badge requirements
-- Author: AI Assistant
-- Date: 2025-11-22

-- Remove is_public column from communities table
ALTER TABLE territory_dk.communities
    DROP COLUMN IF EXISTS is_public;
