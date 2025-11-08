-- Rollback seed data for Denmark
DELETE FROM territory.settings WHERE key IN ('territory_code', 'language', 'timezone', 'locale');
DELETE FROM global.territories WHERE code = 'dk';
