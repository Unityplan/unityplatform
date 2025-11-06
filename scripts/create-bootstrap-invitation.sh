#!/bin/bash

# Create Bootstrap Invitation Token
# This script creates the initial invitation token for territory managers
# to bootstrap the invitation system.

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
TERRITORY_CODE="${1:-dk}"
EMAIL="${2}"
DAYS="${3:-365}"

# Validate inputs
if [ -z "$EMAIL" ]; then
    echo -e "${RED}Error: Email is required${NC}"
    echo "Usage: $0 <territory_code> <email> [days]"
    echo "Example: $0 dk admin@unityplan.dk 365"
    exit 1
fi

# Database connection details
DB_CONTAINER="service-postgres-${TERRITORY_CODE}"
DB_NAME="unityplan_${TERRITORY_CODE}"
DB_USER="unityplan"
SCHEMA_NAME="territory_${TERRITORY_CODE}"

echo -e "${BLUE}Creating bootstrap invitation token...${NC}"
echo "Territory: ${TERRITORY_CODE}"
echo "Email: ${EMAIL}"
echo "Valid for: ${DAYS} days"
echo ""

# Generate random token (similar to Rust implementation)
TOKEN="inv_$(openssl rand -hex 16)"

# Calculate expiration date
EXPIRES_AT=$(date -u -d "+${DAYS} days" '+%Y-%m-%d %H:%M:%S+00')

echo -e "${YELLOW}Inserting token into database...${NC}"

# Insert the token (created_by_user_id is NULL for bootstrap tokens)
docker exec -i "${DB_CONTAINER}" psql -U "${DB_USER}" -d "${DB_NAME}" <<SQL
INSERT INTO ${SCHEMA_NAME}.invitation_tokens (
    id,
    token,
    token_type,
    email,
    max_uses,
    used_count,
    expires_at,
    is_active,
    created_by_user_id,
    purpose
) VALUES (
    gen_random_uuid(),
    '${TOKEN}',
    'single_use',
    '${EMAIL}',
    1,
    0,
    '${EXPIRES_AT}'::timestamptz,
    true,
    NULL,
    'Bootstrap invitation for initial territory administrator'
);

-- Verify insertion
SELECT 
    token,
    token_type,
    email,
    expires_at,
    purpose
FROM ${SCHEMA_NAME}.invitation_tokens 
WHERE token = '${TOKEN}';
SQL

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ Bootstrap invitation token created successfully!${NC}"
    echo ""
    echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}Token: ${TOKEN}${NC}"
    echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
    echo ""
    echo "This token can be used to register the initial admin account:"
    echo "  Email: ${EMAIL}"
    echo "  Territory: ${TERRITORY_CODE}"
    echo "  Expires: ${EXPIRES_AT}"
    echo ""
    echo -e "${YELLOW}⚠️  Save this token securely - it will not be shown again!${NC}"
    echo ""
    echo "Use it in the registration request:"
    echo "  POST /api/auth/register"
    echo "  {\"invitation_token\": \"${TOKEN}\", ...}"
    echo ""
else
    echo -e "${RED}✗ Failed to create bootstrap invitation token${NC}"
    exit 1
fi
