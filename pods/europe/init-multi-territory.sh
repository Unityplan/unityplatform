#!/bin/bash
# PostgreSQL Initialization Script for Europe Multi-Territory Pod
# Creates separate schemas for Germany (DE), France (FR), Spain (ES)

set -e

echo "🇪🇺 Initializing Europe Multi-Territory Pod Database..."

# Function to create territory schema
create_territory_schema() {
  local territory_code=$1
  local territory_name=$2
  local db_name=$3
  
  echo "Creating database and schema for $territory_name ($territory_code)..."
  
  # Create database
  psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    CREATE DATABASE $db_name;
    GRANT ALL PRIVILEGES ON DATABASE $db_name TO $POSTGRES_USER;
EOSQL

  # Connect to database and create schema
  psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname="$db_name" <<-EOSQL
    -- Create extensions
    CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
    CREATE EXTENSION IF NOT EXISTS "pg_trgm";
    CREATE EXTENSION IF NOT EXISTS "btree_gin";
    
    -- Create global schema (for replicated data)
    CREATE SCHEMA IF NOT EXISTS global;
    
    -- Create territory schema
    CREATE SCHEMA IF NOT EXISTS territory_${territory_code};
    
    -- Global territories table (same across all pods)
    CREATE TABLE IF NOT EXISTS global.territories (
      id VARCHAR(100) PRIMARY KEY,
      name VARCHAR(255) NOT NULL,
      type VARCHAR(50) NOT NULL,
      parent_territory VARCHAR(100),
      pod_id VARCHAR(50),
      timezone VARCHAR(100),
      locale VARCHAR(10),
      default_language VARCHAR(10),
      metadata JSONB,
      created_at TIMESTAMPTZ DEFAULT NOW(),
      updated_at TIMESTAMPTZ DEFAULT NOW(),
      
      CHECK (
        (type IN ('country', 'first_nation') AND parent_territory IS NULL)
        OR
        (type = 'community' AND parent_territory IS NOT NULL)
      )
    );
    
    -- Insert this territory
    INSERT INTO global.territories (id, name, type, parent_territory, pod_id, timezone, locale, default_language)
    VALUES ('${territory_code}', '${territory_name}', 'country', NULL, 'eu', 
            '$(eval echo \$TERRITORY_${territory_code}_TIMEZONE)', 
            '$(eval echo \$TERRITORY_${territory_code}_LOCALE)', 
            '$(eval echo \$TERRITORY_${territory_code}_LANGUAGE)')
    ON CONFLICT (id) DO NOTHING;
    
    -- Create territory-specific tables
    CREATE TABLE IF NOT EXISTS territory_${territory_code}.users (
      id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
      username VARCHAR(255) UNIQUE NOT NULL,
      email VARCHAR(255) UNIQUE NOT NULL,
      territory_id VARCHAR(100) DEFAULT '${territory_code}',
      created_at TIMESTAMPTZ DEFAULT NOW(),
      updated_at TIMESTAMPTZ DEFAULT NOW()
    );
    
    CREATE TABLE IF NOT EXISTS territory_${territory_code}.communities (
      id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
      name VARCHAR(255) NOT NULL,
      territory_id VARCHAR(100) DEFAULT '${territory_code}',
      created_at TIMESTAMPTZ DEFAULT NOW()
    );
    
    CREATE TABLE IF NOT EXISTS territory_${territory_code}.posts (
      id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
      user_id UUID REFERENCES territory_${territory_code}.users(id),
      content TEXT,
      created_at TIMESTAMPTZ DEFAULT NOW()
    );
    
    -- Create indexes
    CREATE INDEX IF NOT EXISTS idx_users_territory ON territory_${territory_code}.users(territory_id);
    CREATE INDEX IF NOT EXISTS idx_users_email ON territory_${territory_code}.users(email);
    
    -- Grant permissions
    GRANT ALL ON SCHEMA territory_${territory_code} TO ${POSTGRES_USER};
    GRANT ALL ON ALL TABLES IN SCHEMA territory_${territory_code} TO ${POSTGRES_USER};
    GRANT ALL ON SCHEMA global TO ${POSTGRES_USER};
    GRANT ALL ON ALL TABLES IN SCHEMA global TO ${POSTGRES_USER};
    
    -- Set default privileges
    ALTER DEFAULT PRIVILEGES IN SCHEMA territory_${territory_code} GRANT ALL ON TABLES TO ${POSTGRES_USER};
    ALTER DEFAULT PRIVILEGES IN SCHEMA global GRANT ALL ON TABLES TO ${POSTGRES_USER};
    
    COMMENT ON SCHEMA territory_${territory_code} IS 'Territory-specific data for ${territory_name} (${territory_code})';
EOSQL

  echo "✅ $territory_name ($territory_code) schema created successfully"
}

# Create schemas for all three territories
create_territory_schema "de" "Germany" "unityplan_de"
create_territory_schema "fr" "France" "unityplan_fr"
create_territory_schema "es" "Spain" "unityplan_es"

# Create a connection database for multi-territory queries (optional)
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
  -- Create metadata database
  CREATE DATABASE unityplan_eu_meta;
EOSQL

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname="unityplan_eu_meta" <<-EOSQL
  CREATE EXTENSION IF NOT EXISTS "postgres_fdw";
  
  -- This database can be used for cross-territory queries via FDW
  -- (Foreign Data Wrappers to connect DE, FR, ES databases)
  
  CREATE SCHEMA IF NOT EXISTS meta;
  
  CREATE TABLE meta.pod_info (
    pod_id VARCHAR(50) PRIMARY KEY,
    pod_name VARCHAR(255),
    territories VARCHAR(255)[],
    created_at TIMESTAMPTZ DEFAULT NOW()
  );
  
  INSERT INTO meta.pod_info (pod_id, pod_name, territories)
  VALUES ('eu', 'Europe', ARRAY['DE', 'FR', 'ES']);
  
  COMMENT ON DATABASE unityplan_eu_meta IS 'Metadata database for Europe multi-territory pod';
EOSQL

echo "🎉 Europe Multi-Territory Pod initialization complete!"
echo ""
echo "📊 Created databases:"
echo "  - unityplan_de (Germany)"
echo "  - unityplan_fr (France)"
echo "  - unityplan_es (Spain)"
echo "  - unityplan_eu_meta (Metadata)"
echo ""
echo "Each database has:"
echo "  - global schema (replicated data)"
echo "  - territory_XX schema (isolated data)"
echo "  - Standard tables: users, communities, posts"
