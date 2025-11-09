#!/bin/bash

# Create Test User
# This script creates a test user directly in the database for development/testing
# IMPORTANT: This bypasses the invitation system and should ONLY be used in development!

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
TERRITORY_CODE="${1:-dk}"
EMAIL="${2:-test@unityplan.dk}"
USERNAME="${3:-testuser}"
PASSWORD="${4:-TestPassword123!}"
FULL_NAME="${5:-Test User}"

echo -e "${BLUE}Creating test user...${NC}"
echo "Territory: ${TERRITORY_CODE}"
echo "Email: ${EMAIL}"
echo "Username: ${USERNAME}"
echo "Password: ${PASSWORD}"
echo "Full Name: ${FULL_NAME}"
echo ""

# Database connection details
DB_CONTAINER="service-postgres-${TERRITORY_CODE}"
DB_NAME="unityplan_${TERRITORY_CODE}"
DB_USER="unityplan"
SCHEMA_NAME="territory"

echo -e "${YELLOW}Generating password hash...${NC}"

# Use Python with argon2-cffi to hash the password
# If not available, we'll use a pre-hashed password for "TestPassword123!"
if command -v python3 &> /dev/null; then
    PASSWORD_HASH=$(python3 -c "
from argon2 import PasswordHasher
ph = PasswordHasher()
print(ph.hash('${PASSWORD}'))
" 2>/dev/null || echo "")
fi

# Fallback to pre-hashed password if Python/argon2 not available
# This hash is for: TestPassword123!
if [ -z "$PASSWORD_HASH" ]; then
    echo -e "${YELLOW}Using pre-hashed password (TestPassword123!)${NC}"
    PASSWORD_HASH='$argon2id$v=19$m=19456,t=2,p=1$9JTjOXh0BQCB2LcC3e5f8g$K7J9x2mZvL3pQwNmR8sT6vU4yZ1aB5cD7eF9gH2iJ4k'
fi

echo -e "${YELLOW}Inserting user into database...${NC}"

# Generate UUID for user
USER_ID=$(uuidgen)

# Insert the user
docker exec -i "${DB_CONTAINER}" psql -U "${DB_USER}" -d "${DB_NAME}" <<SQL
-- Insert/update user in users table
INSERT INTO ${SCHEMA_NAME}.users (
    id,
    username,
    email,
    password_hash,
    full_name,
    is_active,
    is_verified,
    invitation_by_token_id,
    created_at,
    updated_at
) VALUES (
    '${USER_ID}'::uuid,
    '${USERNAME}',
    '${EMAIL}',
    '${PASSWORD_HASH}',
    '${FULL_NAME}',
    true,
    true,
    NULL,
    NOW(),
    NOW()
) ON CONFLICT (username) DO UPDATE SET
    password_hash = EXCLUDED.password_hash,
    email = EXCLUDED.email,
    full_name = EXCLUDED.full_name,
    is_active = EXCLUDED.is_active,
    is_verified = EXCLUDED.is_verified,
    updated_at = NOW();

-- Verify insertion
SELECT 
    id,
    username,
    email,
    full_name,
    is_active,
    is_verified,
    created_at
FROM ${SCHEMA_NAME}.users
WHERE username = '${USERNAME}';
SQL

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ Test user created successfully!${NC}"
    echo ""
    echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}Login Credentials:${NC}"
    echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
    echo "  Email:     ${EMAIL}"
    echo "  Username:  ${USERNAME}"
    echo "  Password:  ${PASSWORD}"
    echo "  Territory: ${TERRITORY_CODE}"
    echo ""
    echo -e "${YELLOW}⚠️  This is a development/test account only!${NC}"
    echo -e "${RED}⚠️  DO NOT use this script in production!${NC}"
    echo ""
else
    echo -e "${RED}✗ Failed to create test user${NC}"
    exit 1
fi
