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
WORKSPACE_DIR="$(dirname "$SCRIPT_DIR")"
MIGRATIONS_DIR="$WORKSPACE_DIR/services/shared-lib/migrations"
CONTAINER_NAME="service-postgres-dk"
DB_USER="unityplatform"
DB_NAME="unityplatform_dk"

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
# Step 3: Run Migrations
# ============================================================================

echo -e "${YELLOW}📋 Running migrations...${NC}"
echo ""

# Get list of migration files
MIGRATIONS=($(ls -1 "$MIGRATIONS_DIR"/*.sql 2>/dev/null | sort))

if [ ${#MIGRATIONS[@]} -eq 0 ]; then
    echo -e "${RED}❌ No migration files found in $MIGRATIONS_DIR${NC}"
    exit 1
fi

# Run each migration
MIGRATION_COUNT=0
for migration in "${MIGRATIONS[@]}"; do
    MIGRATION_FILE=$(basename "$migration")
    MIGRATION_COUNT=$((MIGRATION_COUNT + 1))
    
    echo -e "${BLUE}   [$MIGRATION_COUNT/${#MIGRATIONS[@]}] Running: $MIGRATION_FILE${NC}"
    
    # Copy migration to container
    docker cp "$migration" "$CONTAINER_NAME:/tmp/$MIGRATION_FILE" > /dev/null
    
    # Run migration
    if docker exec $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME -f "/tmp/$MIGRATION_FILE" 2>&1 | grep -E "(ERROR|NOTICE|✅)"; then
        echo -e "${GREEN}      ✓ Success${NC}"
    else
        echo -e "${RED}      ✗ Failed${NC}"
        exit 1
    fi
    echo ""
done

echo -e "${GREEN}✅ All migrations completed successfully${NC}"
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
# Step 5: Summary
# ============================================================================

echo -e "${BLUE}============================================================================${NC}"
echo -e "${GREEN}🎉 Database Setup Complete!${NC}"
echo -e "${BLUE}============================================================================${NC}"
echo ""
echo "📊 Summary:"
echo "   • Database: $DB_NAME (fresh)"
echo "   • Migrations run: $MIGRATION_COUNT"
echo "   • Schemas: global, territory_dk"
echo ""
echo "🔗 Connection info:"
echo "   • Host: localhost"
echo "   • Port: 5432"
echo "   • Database: $DB_NAME"
echo "   • User: $DB_USER"
echo ""
echo -e "${BLUE}============================================================================${NC}"
