-- Rollback: Remove auto-create user profile trigger

DROP TRIGGER IF EXISTS trg_create_user_profile ON territory.users;
DROP FUNCTION IF EXISTS territory.create_user_profile();
