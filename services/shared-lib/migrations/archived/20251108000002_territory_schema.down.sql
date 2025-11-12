-- Rollback territory schema
DROP TRIGGER IF EXISTS trg_create_global_identity ON territory.users;
DROP FUNCTION IF EXISTS create_global_user_identity();

DROP TRIGGER IF EXISTS update_territory_invitation_tokens_updated_at ON territory.invitation_tokens;
DROP TRIGGER IF EXISTS update_territory_users_updated_at ON territory.users;

DROP TABLE IF EXISTS territory.community_members;
DROP TABLE IF EXISTS territory.invitation_uses;
DROP TABLE IF EXISTS territory.invitation_tokens;
DROP TABLE IF EXISTS territory.users;
DROP TABLE IF EXISTS territory.communities;
DROP TABLE IF EXISTS territory.settings;

DROP SCHEMA IF EXISTS territory;
