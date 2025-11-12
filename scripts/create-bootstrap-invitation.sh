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
    echo "Example: $0 dk admin@unityplatform.dk 365"
    exit 1
fi

# Database connection details
DB_CONTAINER="service-postgres-${TERRITORY_CODE}"
DB_NAME="unityplatform"
DB_USER="unityplatform"
# For single-territory pods, use generic "territory" schema
# For multi-territory pods, use "territory_XX" schema
SCHEMA_NAME="territory"

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
# ⭐ DUAL-TABLE INSERT: territory.invitation_tokens + global.invitation_token_registry
docker exec -i "${DB_CONTAINER}" psql -U "${DB_USER}" -d "${DB_NAME}" <<SQL
-- Insert into territory schema and global registry in a single transaction
WITH new_token AS (
    INSERT INTO ${SCHEMA_NAME}.invitation_tokens (
        id,
        token,
        token_type,
        invited_email,
        max_uses,
        current_uses,
        expires_at,
        is_active,
        created_by_user_id
    ) VALUES (
        gen_random_uuid(),
        '${TOKEN}',
        'single_use',
        '${EMAIL}',
        1,
        0,
        '${EXPIRES_AT}'::timestamptz,
        true,
        NULL
    )
    RETURNING id, token
)
INSERT INTO global.invitation_token_registry (
    token,
    territory_code,
    territory_token_id
)
SELECT 
    token,
    '${TERRITORY_CODE}',
    id
FROM new_token;

-- Verify insertion in both tables
SELECT 
    'territory.invitation_tokens' AS source,
    t.token,
    t.token_type,
    t.invited_email,
    t.expires_at,
    t.is_active
FROM ${SCHEMA_NAME}.invitation_tokens t
WHERE t.token = '${TOKEN}'
UNION ALL
SELECT 
    'global.invitation_token_registry' AS source,
    r.token,
    NULL AS token_type,
    NULL AS invited_email,
    NULL AS expires_at,
    NULL AS is_active
FROM global.invitation_token_registry r
WHERE r.token = '${TOKEN}';
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
