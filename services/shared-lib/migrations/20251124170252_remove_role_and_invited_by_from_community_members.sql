-- Remove role and invited_by from community_members

ALTER TABLE territory_dk.community_members
DROP COLUMN role,
DROP COLUMN invited_by;
