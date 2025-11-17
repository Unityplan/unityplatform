#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=1
PRIORITY_MEDIUM_ID=2
TYPE_FEATURE_ID=5
TYPE_INFRASTRUCTURE_ID=8
AREA_MATRIX_BRIDGE_ID=20

# Milestone ID
MILESTONE_ID=1  # v0.1.0-alpha.2

# Function to create open issue
create_open_issue() {
    local title="$1"
    local body="$2"
    local label_ids="$3"
    
    # Convert label IDs to JSON array of numbers
    labels_json=$(printf '%s\n' ${label_ids//,/ } | jq -R 'tonumber' | jq -s .)
    
    # Create the issue (open by default)
    response=$(curl -s -X POST \
        -H "Authorization: token ${FORGEJO_TOKEN}" \
        -H "Content-Type: application/json" \
        "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues" \
        -d "{
            \"title\": \"${title}\",
            \"body\": \"${body}\",
            \"labels\": ${labels_json},
            \"milestone\": ${MILESTONE_ID}
        }")
    
    issue_number=$(echo "$response" | jq -r '.number')
    
    if [ "$issue_number" != "null" ]; then
        echo "✅ Created issue #${issue_number}: ${title}"
        echo "${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}"
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

echo "=========================================="
echo "Creating Stage 8 open issues (Matrix Protocol Integration)..."
echo "=========================================="
echo ""

# Stage 8: Matrix Protocol Integration - 6 tasks total

create_open_issue \
    "Stage 8.1: Add Matrix Synapse to Docker infrastructure" \
    "## Description
Add Matrix Synapse homeserver to the Docker infrastructure for decentralized communication.

## Tasks
- Add Matrix Synapse service to docker-compose.yml
- Configure Matrix homeserver for Denmark territory
- Set up PostgreSQL database for Synapse
- Configure federation settings
- Set up HTTPS/TLS certificates (self-signed for dev)

## Acceptance Criteria
- [ ] Matrix Synapse container running in Docker
- [ ] Synapse using PostgreSQL (not SQLite)
- [ ] Homeserver accessible at matrix.unityplatform.local (dev)
- [ ] Federation enabled for cross-territory communication
- [ ] Admin user created for territory
- [ ] Health check endpoint working

## Dependencies
- Stage 7: Course Service (for sequential completion)

## Technical Notes
- Use official matrix-org/synapse Docker image
- Territory-specific homeserver: dk.unityplatform.local
- Configure server_name in homeserver.yaml
- Enable registration (will be controlled by our platform)
- Set up reverse proxy rules in Traefik" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_MATRIX_BRIDGE_ID}"

create_open_issue \
    "Stage 8.2: Configure Matrix homeserver for territory" \
    "## Description
Complete Matrix homeserver configuration for territory-based deployment.

## Configuration Requirements
- Server name: territory code domain (e.g., dk.unityplatform.local)
- Registration: Disabled (controlled by platform)
- Federation: Enabled for cross-territory rooms
- Media repository: Configured with size limits
- Rate limiting: Configured for production
- User directory: Enabled for search

## Acceptance Criteria
- [ ] homeserver.yaml fully configured
- [ ] Database schema initialized
- [ ] Admin user can login via Element/web client
- [ ] Federation test successful (if multi-territory available)
- [ ] Media upload working
- [ ] User search functional

## Technical Notes
- Use environment variables for sensitive config
- Configure max upload size (50MB recommended)
- Enable presence tracking for online status
- Configure retention policies
- Set up log rotation" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_MATRIX_BRIDGE_ID}"

create_open_issue \
    "Stage 8.3: Create matrix-bridge service scaffolding" \
    "## Description
Create the matrix-bridge Rust service to integrate platform with Matrix protocol.

## Acceptance Criteria
- [ ] matrix-bridge crate created in services/ directory
- [ ] Standard service structure (handlers, models, services)
- [ ] Dependencies: ruma (Matrix Rust SDK), actix-web, sqlx, shared-lib
- [ ] All 5 middleware components integrated
- [ ] Port 8016 configured
- [ ] Basic health/ready/metrics endpoints

## Technical Notes
- Use ruma crate for Matrix client SDK
- This service acts as bridge between platform and Matrix
- Will handle user registration sync
- Will manage room creation for forum topics
- Use AppConfig for configuration
- Store Matrix credentials in database" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_MATRIX_BRIDGE_ID}"

create_open_issue \
    "Stage 8.4: Implement Matrix user registration sync" \
    "## Description
Automatically register platform users on Matrix homeserver.

## Features
- Subscribe to NATS user.registered events
- Create Matrix account when platform user registers
- Store Matrix credentials in database
- Generate secure Matrix password
- Handle registration errors gracefully

## Acceptance Criteria
- [ ] NATS subscription to user.registered working
- [ ] Matrix user creation via Synapse Admin API
- [ ] Credentials stored in territory database
- [ ] Error handling for duplicate registrations
- [ ] Logging of all Matrix registrations
- [ ] Test with new user registration flow

## Database Schema
Add to territory schema:
- matrix_users table: user_id, matrix_user_id, access_token, created_at

## Technical Notes
- Use Synapse Admin API for user creation
- Matrix user ID format: @username:dk.unityplatform.local
- Store access token for future operations
- Handle homeserver downtime gracefully
- Publish matrix.user.created event" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_MATRIX_BRIDGE_ID}"

create_open_issue \
    "Stage 8.5: Matrix credentials storage and retrieval" \
    "## Description
Implement secure storage and retrieval of Matrix credentials.

## Features
- Database migration for matrix_users table
- API endpoint: GET /api/v1/matrix/credentials (authenticated)
- Secure credential encryption
- Token refresh mechanism (if needed)

## Acceptance Criteria
- [ ] Migration 20251117000010_matrix_users.sql created
- [ ] matrix_users table in territory schema
- [ ] GET /api/v1/matrix/credentials endpoint working
- [ ] Returns matrix_user_id and access_token
- [ ] JWT authentication required
- [ ] User can only access own credentials

## Technical Notes
- Consider encrypting access tokens at rest
- Return credentials needed for Matrix client
- Will be used by forum-service for room operations
- Cache credentials to avoid DB lookups" \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_MATRIX_BRIDGE_ID}"

create_open_issue \
    "Stage 8.6: Matrix-bridge architecture compliance and testing" \
    "## Description
Ensure matrix-bridge follows architecture standards and test integration.

## Architecture Compliance
- [ ] camelCase JSON serialization
- [ ] AppConfig integration
- [ ] NATS client initialized
- [ ] Health/ready/metrics endpoints
- [ ] Logging with tracing_subscriber
- [ ] Service added to dev scripts

## Testing
- [ ] User registration creates Matrix account
- [ ] Credentials stored correctly
- [ ] Credentials retrieval working
- [ ] Error handling tested
- [ ] NATS event flow verified
- [ ] Matrix homeserver connectivity tested

## Documentation
- [ ] README.md with service overview
- [ ] API.md with endpoints
- [ ] MATRIX-INTEGRATION.md with architecture

## Acceptance Criteria
- [ ] All compliance checks passing
- [ ] Integration tests successful
- [ ] Documentation complete
- [ ] Ready for forum-service integration

## Technical Notes
- Test with actual Matrix Synapse instance
- Verify federation capabilities
- Document Matrix user ID format
- Include troubleshooting guide" \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_MATRIX_BRIDGE_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 8 issues created!"
echo "=========================================="
echo ""
echo "Summary: Created 6 open issues for Stage 8 (Matrix Protocol Integration)"
echo "Issues: #74-#79"
echo ""
echo "View all issues: ${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues?state=open&milestone=1"
