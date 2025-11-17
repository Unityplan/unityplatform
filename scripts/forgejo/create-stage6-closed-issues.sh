#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs (from previous work)
PRIORITY_HIGH_ID=1
PRIORITY_MEDIUM_ID=2
TYPE_FEATURE_ID=5
TYPE_INFRASTRUCTURE_ID=8
AREA_TERRITORY_ID=12
AREA_BADGE_ID=13
AREA_INFRASTRUCTURE_ID=14
STATUS_DONE_ID=17  # We'll need to verify this exists

# Milestone ID
MILESTONE_ID=1  # v0.1.0-alpha.2

# Function to create closed issue
create_closed_issue() {
    local title="$1"
    local body="$2"
    local label_ids="$3"
    
    # Convert label IDs to JSON array of numbers
    labels_json=$(printf '%s\n' ${label_ids//,/ } | jq -R 'tonumber' | jq -s .)
    
    # Create the issue
    response=$(curl -s -X POST \
        -H "Authorization: token ${FORGEJO_TOKEN}" \
        -H "Content-Type: application/json" \
        "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues" \
        -d "{
            \"title\": \"${title}\",
            \"body\": \"${body}\",
            \"labels\": ${labels_json},
            \"milestone\": ${MILESTONE_ID},
            \"closed\": false
        }")
    
    issue_number=$(echo "$response" | jq -r '.number')
    
    if [ "$issue_number" != "null" ]; then
        echo "✅ Created issue #${issue_number}: ${title}"
        
        # Close the issue immediately
        curl -s -X PATCH \
            -H "Authorization: token ${FORGEJO_TOKEN}" \
            -H "Content-Type: application/json" \
            "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}" \
            -d '{"state": "closed"}' > /dev/null
        
        echo "   🔒 Closed issue #${issue_number}"
        
        # Add comment about completion
        curl -s -X POST \
            -H "Authorization: token ${FORGEJO_TOKEN}" \
            -H "Content-Type: application/json" \
            "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}/comments" \
            -d '{"body": "✅ **Completed on November 14, 2025**\n\nThis issue represents historical work that was completed during Stage 6 development. Created for tracking and documentation purposes."}' > /dev/null
        
        echo "${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}"
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5  # Rate limiting
}

echo "Creating Stage 6 closed issues (historical record)..."
echo ""

# Step 6.1: Territory Service Scaffolding
create_closed_issue \
    "Stage 6.1: Create territory-service scaffolding" \
    "## Description
Create the territory-service Rust crate with standard structure.

## Acceptance Criteria
- [ ] territory-service crate created in services/ directory
- [ ] Standard service structure (handlers, models, services)
- [ ] Cargo.toml with dependencies (actix-web, sqlx, shared-lib)
- [ ] main.rs with basic server setup
- [ ] All 5 middleware components integrated

## Completed
✅ Completed on November 14, 2025" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TERRITORY_ID}"

# Step 6.2: Territory Handlers (combined into one issue)
create_closed_issue \
    "Stage 6.2: Implement territory endpoints" \
    "## Description
Implement all territory management endpoints.

## Endpoints
1. GET /territories - List all active territories
2. GET /territories/{code} - Get territory details
3. POST /territories - Create new territory (admin only)

## Acceptance Criteria
- [ ] All 3 endpoints implemented with proper validation
- [ ] Territory code validation (ISO 3166-1 Alpha-2)
- [ ] Admin-only protection on POST endpoint
- [ ] Database integration with territory_dk schema
- [ ] Error handling for invalid codes

## Completed
✅ Completed on November 14, 2025
✅ 6 total endpoints including stats and settings" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_TERRITORY_ID}"

# Step 6.3: Territory Architecture Compliance
create_closed_issue \
    "Stage 6.3: Territory-service architecture compliance" \
    "## Description
Ensure territory-service follows all architecture standards.

## Acceptance Criteria
- [ ] camelCase JSON serialization on all models
- [ ] AppConfig integration (database_url, nats_url)
- [ ] NATS client initialized and connected
- [ ] Health endpoint at /api/v1/health
- [ ] Ready endpoint with database connectivity check
- [ ] Metrics endpoint (Prometheus format)
- [ ] MetricsCollector integrated
- [ ] All endpoints tested and verified

## Completed
✅ Completed on November 14, 2025
✅ 7 models verified with camelCase
✅ Full observability stack" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TERRITORY_ID}"

# Step 6.4: Badge Service Scaffolding
create_closed_issue \
    "Stage 6.4: Create badge-service scaffolding" \
    "## Description
Create the badge-service Rust crate with standard structure.

## Acceptance Criteria
- [ ] badge-service crate created in services/ directory
- [ ] Standard service structure (handlers, models, services)
- [ ] Cargo.toml with dependencies
- [ ] main.rs with basic server setup
- [ ] All 5 middleware components integrated

## Completed
✅ Completed on November 14, 2025" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_BADGE_ID}"

# Step 6.5: Badge Database Schema
create_closed_issue \
    "Stage 6.5: Badge database schema migration" \
    "## Description
Create database migration for badge system tables.

## Acceptance Criteria
- [ ] Migration 20251113000005 created
- [ ] Tables: badges, user_badges, badge_permissions
- [ ] Badge types: achievement, role, special
- [ ] Expiration tracking
- [ ] Featured badge support
- [ ] Migration applied successfully

## Completed
✅ Completed on November 14, 2025
✅ Migration 20251113000005_badge_system.up.sql" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_BADGE_ID}"

# Step 6.6: Seed Code of Conduct Badge
create_closed_issue \
    "Stage 6.6: Seed essential badges" \
    "## Description
Create seed script for Code of Conduct badge and badge expiration checking.

## Acceptance Criteria
- [ ] Seed script for essential badges
- [ ] Code of Conduct badge created
- [ ] Badge expiration check function
- [ ] Automatic badge cleanup job (future)

## Completed
✅ Completed on November 14, 2025" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_BADGE_ID}"

# Step 6.7: Badge Handlers Implementation
create_closed_issue \
    "Stage 6.7: Implement badge endpoints" \
    "## Description
Implement all badge management endpoints.

## Endpoints
1. GET /badges - List all available badges
2. GET /badges/{badge_id} - Get badge details
3. GET /users/{user_id}/badges - Get user's badges
4. POST /badges/award - Award badge to user
5. POST /badges/revoke - Revoke badge
6. PATCH /users/me/badges/{badge_id}/featured - Toggle featured status

## Acceptance Criteria
- [ ] All 6 endpoints implemented
- [ ] Award/revoke permissions checked
- [ ] Featured badge tracking
- [ ] NATS event publishing on award/revoke

## Completed
✅ Completed on November 14, 2025
✅ 7 total endpoints including register-publisher" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_BADGE_ID}"

# Step 6.8: Badge Architecture Compliance
create_closed_issue \
    "Stage 6.8: Badge-service architecture compliance" \
    "## Description
Ensure badge-service follows all architecture standards.

## Acceptance Criteria
- [ ] camelCase JSON serialization on all models
- [ ] AppConfig integration
- [ ] Logging configuration with tracing_subscriber
- [ ] Health/ready/metrics endpoints
- [ ] NATS client connected
- [ ] Environment variables cleaned
- [ ] All endpoints tested

## Completed
✅ Completed on November 14, 2025
✅ 6 models verified
✅ Full observability" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_BADGE_ID}"

# Step 6.9: Permission Checking System
create_closed_issue \
    "Stage 6.9: Permission checking system" \
    "## Description
Implement badge-based RBAC permission system in shared-lib.

## Components
1. PermissionChecker with LRU cache
2. RequirePermission middleware
3. RequireAnyPermission middleware
4. Badge registration endpoint

## Acceptance Criteria
- [ ] PermissionChecker with 5-minute TTL cache
- [ ] Wildcard permission support (portal:*)
- [ ] Territory-aware queries
- [ ] Middleware integration
- [ ] Documentation in shared-lib/PERMISSION.md

## Completed
✅ Completed on November 14, 2025
✅ 1000-entry LRU cache
✅ Hierarchical permissions" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}"

# Step 6.10: Badge Event Handlers (NATS)
create_closed_issue \
    "Stage 6.10: Badge NATS event handlers" \
    "## Description
Implement NATS event publishing and subscription for badge system.

## Features
1. Subscribe to user.registered events
2. Publish badge.awarded events
3. Publish badge.revoked events

## Acceptance Criteria
- [ ] Event handler for user.registered
- [ ] Auto-grant Code of Conduct in dev mode
- [ ] Event publishing on award/revoke
- [ ] Event payload includes user_id, badge_slug, reason
- [ ] Graceful error handling

## Completed
✅ Completed on November 14, 2025
✅ Hook system design (HOOK-SYSTEM.md)" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_BADGE_ID}"

# Step 6.11: Testing Badge System
create_closed_issue \
    "Stage 6.11: Badge system testing" \
    "## Description
Manual testing of all badge and territory endpoints.

## Test Coverage
1. All 7 badge endpoints verified
2. All 6 territory endpoints verified
3. Permission system verification
4. NATS event flow validation

## Acceptance Criteria
- [ ] Manual endpoint testing complete
- [ ] Permission checks working
- [ ] NATS events publishing correctly
- [ ] All responses in camelCase
- [ ] Error handling verified

## Completed
✅ Completed on November 14, 2025
✅ Full manual test coverage
⏭️  Automated tests deferred" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_BADGE_ID}"

echo ""
echo "✅ All Stage 6 issues created and closed!"
echo ""
echo "Summary: Created 11 closed issues representing Stage 6 work completed on November 14, 2025"
