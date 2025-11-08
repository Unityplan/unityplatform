-- Rollback global schema
DROP TRIGGER IF EXISTS update_global_user_identities_updated_at ON global.user_identities;
DROP FUNCTION IF EXISTS update_updated_at_column();

DROP TABLE IF EXISTS global.audit_log;
DROP TABLE IF EXISTS global.sessions;
DROP TABLE IF EXISTS global.role_assignments;
DROP TABLE IF EXISTS global.territory_managers;
DROP TABLE IF EXISTS global.user_identities;
DROP TABLE IF EXISTS global.territories;

DROP SCHEMA IF EXISTS global;

-- Drop extensions (only if not used by other schemas)
-- DROP EXTENSION IF EXISTS "pgcrypto";
-- DROP EXTENSION IF EXISTS "uuid-ossp";
