#!/bin/bash
# ============================================================================
# Database Setup Script - Reset and Run Migrations
# ============================================================================
# This script:
# 1. Drops and recreates the database
# 2. Enables required extensions
# 3. Runs all migrations in order
# 4. Verifies the database structure
# ============================================================================

set -e  # Exit on error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
WORKSPACE_DIR="$(dirname "$(dirname "$SCRIPT_DIR")")"
MIGRATIONS_DIR="$WORKSPACE_DIR/services/shared-lib/migrations"
SHARED_LIB_DIR="$WORKSPACE_DIR/services/shared-lib"
CONTAINER_NAME="service-postgres-dk"
DB_USER="unityplatform"
DB_NAME="unityplatform_dk"
DB_PASSWORD="unityplatform_dev_password_dk"
DB_URL="postgresql://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}============================================================================${NC}"
echo -e "${BLUE}🔄 Database Setup - Reset and Run Migrations${NC}"
echo -e "${BLUE}============================================================================${NC}"
echo ""

# ============================================================================
# Step 1: Check Docker Container
# ============================================================================

echo -e "${YELLOW}📦 Checking PostgreSQL container...${NC}"
if ! docker ps | grep -q "$CONTAINER_NAME"; then
    echo -e "${RED}❌ PostgreSQL container '$CONTAINER_NAME' not running${NC}"
    echo ""
    echo "Start it with:"
    echo "  cd $WORKSPACE_DIR"
    echo "  docker compose -f docker-compose.pod.yml up -d postgres-dk"
    exit 1
fi
echo -e "${GREEN}✅ PostgreSQL container is running${NC}"
echo ""

# ============================================================================
# Step 2: Reset Database
# ============================================================================

echo -e "${YELLOW}🗑️  Resetting database...${NC}"

# Terminate existing connections
echo "   Terminating existing connections..."
docker exec $CONTAINER_NAME psql -U $DB_USER -d postgres -c \
    "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$DB_NAME' AND pid <> pg_backend_pid();" \
    > /dev/null 2>&1 || true

# Drop database
echo "   Dropping database '$DB_NAME'..."
docker exec $CONTAINER_NAME psql -U $DB_USER -d postgres -c \
    "DROP DATABASE IF EXISTS $DB_NAME;" > /dev/null

# Create database
echo "   Creating fresh database '$DB_NAME'..."
docker exec $CONTAINER_NAME psql -U $DB_USER -d postgres -c \
    "CREATE DATABASE $DB_NAME;" > /dev/null

echo -e "${GREEN}✅ Database reset complete${NC}"
echo ""

# ============================================================================
# Step 3: Run Migrations with sqlx
# ============================================================================

echo -e "${YELLOW}📋 Running migrations with sqlx...${NC}"
echo ""

# Check if sqlx is installed
if ! command -v sqlx &> /dev/null; then
    echo -e "${RED}❌ sqlx not found${NC}"
    echo ""
    echo "Install it with:"
    echo "  cargo install sqlx-cli --no-default-features --features postgres"
    exit 1
fi

# Run migrations
cd "$SHARED_LIB_DIR"

echo "   Running: DATABASE_URL=\"$DB_URL\" sqlx migrate run"
echo ""

if DATABASE_URL="$DB_URL" sqlx migrate run; then
    echo ""
    echo -e "${GREEN}✅ All migrations completed successfully${NC}"
else
    echo ""
    echo -e "${RED}❌ Migration failed${NC}"
    exit 1
fi

cd "$WORKSPACE_DIR"
echo ""

# ============================================================================
# Step 4: Verify Database Structure
# ============================================================================

echo -e "${YELLOW}🔍 Verifying database structure...${NC}"
echo ""

# Check schemas
echo "   Schemas:"
docker exec $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME -c \
    "SELECT schema_name FROM information_schema.schemata WHERE schema_name IN ('global', 'territory_dk') ORDER BY schema_name;" \
    -t -A | while read schema; do
    if [ ! -z "$schema" ]; then
        echo -e "${GREEN}      ✓ $schema${NC}"
    fi
done

# Check global tables
echo ""
echo "   Global tables:"
docker exec $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME -c \
    "\dt global.*" -t -A | grep -v "^$" | awk -F'|' '{print $2}' | while read table; do
    if [ ! -z "$table" ]; then
        echo -e "${GREEN}      ✓ global.$table${NC}"
    fi
done

# Check territory_dk tables
echo ""
echo "   Territory tables (territory_dk):"
docker exec $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME -c \
    "\dt territory_dk.*" -t -A | grep -v "^$" | awk -F'|' '{print $2}' | while read table; do
    if [ ! -z "$table" ]; then
        echo -e "${GREEN}      ✓ territory_dk.$table${NC}"
    fi
done

# Check extensions
echo ""
echo "   Extensions:"
docker exec $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME -c \
    "SELECT extname FROM pg_extension WHERE extname IN ('uuid-ossp', 'pgcrypto') ORDER BY extname;" \
    -t -A | while read ext; do
    if [ ! -z "$ext" ]; then
        echo -e "${GREEN}      ✓ $ext${NC}"
    fi
done

echo ""
echo -e "${GREEN}✅ Database verification complete${NC}"
echo ""

# ============================================================================
# Step 5: Seed Data (Optional)
# ============================================================================

# Check if --seed flag is passed
SEED_DATA=false
for arg in "$@"; do
    if [ "$arg" == "--seed" ]; then
        SEED_DATA=true
    fi
done

if [ "$SEED_DATA" == true ]; then
    echo -e "${YELLOW}🌱 Seeding database...${NC}"
    echo ""
    
    # Wait for services to be ready
    echo "   Waiting for services to be ready..."
    sleep 2
    
    # Step 5a: Run Denmark geographic seed script FIRST (creates zones/neighborhoods)
    GEO_SEED_SCRIPT="$SCRIPT_DIR/../dev/seed-denmark-geo.py"
    if [ -f "$GEO_SEED_SCRIPT" ]; then
        echo "   Running Denmark geographic seed script..."
        if python3 "$GEO_SEED_SCRIPT" 2>&1; then
            echo -e "${GREEN}   ✅ Denmark geographic data seeded${NC}"
        else
            echo -e "${YELLOW}   ⚠️  Geographic seeding failed${NC}"
        fi
    else
        echo -e "${YELLOW}   ⚠️  Denmark geo seed script not found: $GEO_SEED_SCRIPT${NC}"
    fi
    
    # Step 5b: Run community seed script AFTER geo (creates guilds, study groups, groups)
    SEED_SCRIPT="$SCRIPT_DIR/../dev/seed-communities.py"
    if [ -f "$SEED_SCRIPT" ]; then
        echo "   Running community seed script..."
        if python3 "$SEED_SCRIPT" 2>&1; then
            echo -e "${GREEN}   ✅ Communities seeded${NC}"
        else
            echo -e "${YELLOW}   ⚠️  Community seeding failed (services may not be running)${NC}"
        fi
    else
        echo -e "${YELLOW}   ⚠️  Community seed script not found: $SEED_SCRIPT${NC}"
    fi
    
    echo ""
fi

# ============================================================================
# Step 6: Summary
# ============================================================================

echo -e "${BLUE}============================================================================${NC}"
echo -e "${GREEN}🎉 Database Setup Complete!${NC}"
echo -e "${BLUE}============================================================================${NC}"
echo ""
echo "📊 Summary:"
echo "   • Database: $DB_NAME (fresh)"
echo "   • Migrations: Applied via sqlx migrate run"
echo "   • Schemas: global, territory_dk"
if [ "$SEED_DATA" == true ]; then
echo "   • Seed data: Applied"
fi
echo ""
echo "🔗 Connection info:"
echo "   • Host: localhost"
echo "   • Port: 5432"
echo "   • Database: $DB_NAME"
echo "   • User: $DB_USER"
echo ""
echo "💡 Tips:"
echo "   • Run with --seed to populate test data"
echo "   • Example: ./scripts/db/setup-database.sh --seed"
echo ""
echo -e "${BLUE}============================================================================${NC}"
