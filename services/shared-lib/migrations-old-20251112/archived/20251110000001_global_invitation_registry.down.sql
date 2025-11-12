-- Rollback global invitation token registry

DROP TABLE IF EXISTS global.invitation_token_registry CASCADE;
