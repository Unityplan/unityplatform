-- Drop trigger
DROP TRIGGER IF EXISTS update_user_profiles_timestamp ON territory.user_profiles;

-- Drop function
DROP FUNCTION IF EXISTS territory.update_user_profile_timestamp();

-- Drop tables (in reverse order due to dependencies)
DROP TABLE IF EXISTS territory.user_blocks;
DROP TABLE IF EXISTS territory.user_connections;
DROP TABLE IF EXISTS territory.user_profiles;
