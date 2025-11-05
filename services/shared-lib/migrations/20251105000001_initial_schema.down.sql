-- Rollback initial schema migration
-- Drops all schemas and their objects

-- Drop triggers
DROP TRIGGER IF EXISTS update_global_users_updated_at ON global.users;

-- Drop functions
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop territory_dk schema and all objects
DROP SCHEMA IF EXISTS territory_dk CASCADE;

-- Drop global schema and all objects
DROP SCHEMA IF EXISTS global CASCADE;

-- Drop UUID extension (only if not used elsewhere)
-- DROP EXTENSION IF EXISTS "uuid-ossp";
