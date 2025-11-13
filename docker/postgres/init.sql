-- PostgreSQL initialization script for Unity Platform
-- This script runs when the database is first created

-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enable pgcrypto for password hashing (if needed)
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create global schema for cross-territory data
CREATE SCHEMA IF NOT EXISTS global;

-- NOTE: Cannot set search_path for database from init script
-- because database name varies by pod (unityplatform_dk, unityplatform_no, etc.)
-- Search path will be set by application migrations

-- Log initialization
DO $$
BEGIN
    RAISE NOTICE 'Unity Platform database initialized successfully';
    RAISE NOTICE 'TimescaleDB extension enabled';
    RAISE NOTICE 'Global schema created';
END $$;
