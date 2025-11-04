-- PostgreSQL initialization script for UnityPlan
-- This script runs when the database is first created

-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enable pgcrypto for password hashing (if needed)
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create global schema for cross-territory data
CREATE SCHEMA IF NOT EXISTS global;

-- Set default search path
ALTER DATABASE unityplan_dev SET search_path TO public, global;

-- Log initialization
DO $$
BEGIN
    RAISE NOTICE 'UnityPlan database initialized successfully';
    RAISE NOTICE 'TimescaleDB extension enabled';
    RAISE NOTICE 'Global schema created';
END $$;
