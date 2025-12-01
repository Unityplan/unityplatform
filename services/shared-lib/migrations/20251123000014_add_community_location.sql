-- Add location and coverage fields to communities table
ALTER TABLE territory_dk.communities
ADD COLUMN location_lat DOUBLE PRECISION,
ADD COLUMN location_lng DOUBLE PRECISION,
ADD COLUMN coverage_area JSONB;

COMMENT ON COLUMN territory_dk.communities.location_lat IS 'Latitude of the community center point';
COMMENT ON COLUMN territory_dk.communities.location_lng IS 'Longitude of the community center point';
COMMENT ON COLUMN territory_dk.communities.coverage_area IS 'JSON defining the coverage area (circle or polygon)';
