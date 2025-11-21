#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_CRITICAL_ID=0
PRIORITY_HIGH_ID=1
PRIORITY_MEDIUM_ID=2
TYPE_FEATURE_ID=5
TYPE_INFRASTRUCTURE_ID=8
AREA_INFRASTRUCTURE_ID=14
AREA_AUTH_ID=9
AREA_USER_ID=10

# Milestone ID
MILESTONE_ID=1  # v0.1.0-alpha.2

# Function to create closed issue
create_closed_issue() {
    local title="$1"
    local body="$2"
    local label_ids="$3"
    local completed_date="$4"
    
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
        
        # Close the issue
        curl -s -X PATCH \
            -H "Authorization: token ${FORGEJO_TOKEN}" \
            -H "Content-Type: application/json" \
            "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}" \
            -d '{"state": "closed"}' > /dev/null
        
        echo "   🔒 Closed issue #${issue_number}"
        
        # Add completion comment
        curl -s -X POST \
            -H "Authorization: token ${FORGEJO_TOKEN}" \
            -H "Content-Type: application/json" \
            "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}/comments" \
            -d "{\"body\": \"✅ **Completed on ${completed_date}**\\n\\nThis issue represents historical work completed during Phase 1 MVP development. Created for tracking and documentation purposes.\"}" > /dev/null
        
        echo "${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}"
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

echo "=========================================="
echo "Creating Stage 1 closed issues..."
echo "=========================================="
echo ""

# Stage 1: Foundation & Infrastructure (33 tasks, completed Nov 4-13, 2025)

create_closed_issue \
    "Stage 1.1: Repository and project structure setup" \
    "## Description
Initialize Git repository and create project structure.

## Tasks
- Initialize Git repository
- Create .gitignore for Rust, Node, Docker
- Create README.md with project overview
- Create workspace directory structure

## Acceptance Criteria
- [ ] Git repository initialized with main branch
- [ ] Comprehensive .gitignore covering all technologies
- [ ] README with project description and setup instructions
- [ ] Proper directory structure (services/, docs/, docker/, etc.)

## Completed
✅ Completed November 4-5, 2025" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 5, 2025"

create_closed_issue \
    "Stage 1.2: Docker infrastructure setup" \
    "## Description
Set up core Docker services for development environment.

## Services
1. PostgreSQL 16 with TimescaleDB
2. NATS with JetStream
3. Redis with persistence
4. Adminer database UI

## Acceptance Criteria
- [ ] docker-compose.yml created for development
- [ ] All services configured and tested
- [ ] Data persistence configured
- [ ] Service connectivity verified

## Completed
✅ Completed November 5, 2025
✅ All services operational" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 5, 2025"

create_closed_issue \
    "Stage 1.3: Rust backend foundation and shared library" \
    "## Description
Create Rust workspace with shared library for all services.

## Components
1. Rust workspace (services/Cargo.toml)
2. Shared library crate (v0.1.0-alpha.1)
3. AppConfig (environment-based configuration)
4. Database module (SQLx integration)
5. NATS client module
6. Error types (AppError)
7. Logging and tracing
8. Infrastructure connectivity tests

## Acceptance Criteria
- [ ] Workspace compiles successfully
- [ ] All modules functional and tested
- [ ] Configuration system working
- [ ] Database connections verified
- [ ] NATS connectivity confirmed

## Completed
✅ Completed November 6-8, 2025
✅ Shared-lib v0.1.0-alpha.1 released" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 8, 2025"

create_closed_issue \
    "Stage 1.4: Multi-pod infrastructure deployment" \
    "## Description
Deploy complete multi-pod architecture with monitoring.

## Components
- Multi-pod Docker Compose configurations
- NATS clustering for cross-pod communication
- Prometheus monitoring for all pods
- Grafana dashboards (Pod Overview, Multi-Pod Overview)
- Jaeger distributed tracing
- Traefik reverse proxy with SSL
- Denmark pod fully operational
- Network architecture (global-net, mesh-network, pod-net)
- Pod exporters (PostgreSQL, Redis, NATS, cAdvisor, Node)
- Deployment scripts and verification tools

## Acceptance Criteria
- [ ] All monitoring targets UP
- [ ] Denmark pod operational
- [ ] Grafana dashboards working
- [ ] Distributed tracing functional
- [ ] Documentation complete

## Completed
✅ Completed November 10-13, 2025
✅ 6/7 Denmark targets UP
✅ Full observability stack" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 1.5: Middleware infrastructure (Priority 1-3)" \
    "## Description
Implement all middleware patterns for production-ready services.

## Middleware
**Priority 1:** request_id, logging, error_handler
**Priority 2:** security_headers, cors, rate_limit, validation
**Priority 3:** graceful_shutdown, circuit_breaker

## Acceptance Criteria
- [ ] All 9 middleware patterns implemented
- [ ] Documentation (MIDDLEWARE.md, ERROR-HANDLING.md)
- [ ] Integration tested with services
- [ ] Performance benchmarked

## Completed
✅ Completed November 12-13, 2025
✅ All middleware in shared-lib
✅ Comprehensive documentation" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 1.6: Development tools setup" \
    "## Description
Configure development and debugging tools.

## Tools
- Forgejo git server with MCP integration
- Docker Registry for container images
- MailHog for email testing
- Redis Commander for Redis management
- Development dashboard
- SQLTools VS Code extension
- PostgreSQL database connection
- Documentation reorganization

## Acceptance Criteria
- [ ] All tools operational
- [ ] MCP integration working
- [ ] Database connections verified
- [ ] Documentation consolidated

## Completed
✅ Completed November 8-13, 2025
✅ Full development stack operational" \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 13, 2025"

echo ""
echo "=========================================="
echo "Creating Stage 2 closed issues..."
echo "=========================================="
echo ""

# Stage 2: Database Schema & Migrations (6 tasks, completed Nov 5-8, 2025)

create_closed_issue \
    "Stage 2.1: SQLx migrations setup" \
    "## Description
Install SQLx CLI and create migration directory structure.

## Tasks
- Install SQLx CLI tool
- Create migration directory in shared-lib
- Configure migration paths
- Test migration workflow

## Acceptance Criteria
- [ ] SQLx CLI installed and working
- [ ] Migration directory created
- [ ] Can create and apply migrations

## Completed
✅ Completed November 5, 2025" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 5, 2025"

create_closed_issue \
    "Stage 2.2: Global schema migration" \
    "## Description
Create global schema for cross-territory identity and federation.

## Schema Components
- Global registries (username, email uniqueness)
- Territory registry
- Session management
- Audit logging

## Acceptance Criteria
- [ ] Migration 20251108000001_global_schema.sql created
- [ ] Migration applied successfully
- [ ] Schema verified in database

## Completed
✅ Completed November 8, 2025
✅ Global identity layer established" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 8, 2025"

create_closed_issue \
    "Stage 2.3: Territory schema template and Denmark seed data" \
    "## Description
Create reusable territory schema template and Denmark initialization.

## Components
- Territory schema template (users, profiles, communities)
- Denmark seed data (territory_dk initialization)
- Schema-based isolation architecture
- ISO 3166-1 Alpha-2 territory codes

## Acceptance Criteria
- [ ] Migration 20251108000002_territory_schema.sql created
- [ ] Migration 20251108000003_seed_data_dk.sql created
- [ ] Territory schema reusable for other countries
- [ ] Multi-territory architecture validated

## Completed
✅ Completed November 8, 2025
✅ Schema separation complete
✅ Future-ready for multi-territory deployment" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}" \
    "November 8, 2025"

echo ""
echo "=========================================="
echo "Creating Stage 3 closed issues..."
echo "=========================================="
echo ""

# Stage 3: Authentication Service (27 tasks, completed Nov 12-14, 2025)

create_closed_issue \
    "Stage 3.1: Auth-service scaffolding and structure" \
    "## Description
Create auth-service crate with standard structure.

## Components
- Auth-service crate in services/ directory
- Handlers, models, services structure
- Dependencies (actix-web, sqlx, jsonwebtoken)
- All middleware integrated

## Acceptance Criteria
- [ ] Crate created and compiling
- [ ] Standard structure in place
- [ ] All 5 middleware components integrated

## Completed
✅ Completed November 12, 2025" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_AUTH_ID}" \
    "November 12, 2025"

create_closed_issue \
    "Stage 3.2: Auth database schema migration" \
    "## Description
Add authentication tables to territory schema.

## Tables
- Sessions (refresh tokens, expiration)
- Invitation tokens
- Password reset tokens

## Acceptance Criteria
- [ ] Auth tables added to territory schema
- [ ] Migration applied successfully
- [ ] Territory creation function updated

## Completed
✅ Completed November 12, 2025" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_AUTH_ID}" \
    "November 12, 2025"

create_closed_issue \
    "Stage 3.3: JWT token service implementation" \
    "## Description
Implement JWT token generation and verification.

## Components
- TokenService struct
- generate_access_token (15 min expiry)
- generate_refresh_token (7 day expiry)
- verify_access_token
- verify_refresh_token

## Acceptance Criteria
- [ ] All token functions implemented
- [ ] Token expiration working
- [ ] Signature verification secure
- [ ] Error handling comprehensive

## Completed
✅ Completed November 12, 2025
✅ Secure JWT implementation with RS256" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_AUTH_ID}" \
    "November 12, 2025"

create_closed_issue \
    "Stage 3.4: Auth endpoint implementation (6 endpoints)" \
    "## Description
Implement all authentication REST endpoints.

## Endpoints
1. POST /auth/register - User registration
2. POST /auth/login - User login
3. POST /auth/refresh - Refresh access token
4. POST /auth/logout - Logout user
5. GET /auth/verify - Verify JWT token
6. POST /auth/check-username - Check username availability

## Acceptance Criteria
- [ ] All endpoints implemented with validation
- [ ] Password hashing with Argon2
- [ ] Global registry uniqueness enforced
- [ ] Territory-aware operations
- [ ] Optional email support

## Completed
✅ Completed November 12-13, 2025
✅ All endpoints tested and working" \
    "${PRIORITY_CRITICAL_ID},${TYPE_FEATURE_ID},${AREA_AUTH_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 3.5: JWT authentication middleware" \
    "## Description
Create middleware for JWT authentication and authorization.

## Components
- JWT authentication middleware
- require_auth() wrapper (JwtAuth Transform)
- Token extraction from headers
- User context injection

## Acceptance Criteria
- [ ] Middleware functional
- [ ] Protected routes working
- [ ] User ID available in handlers
- [ ] Error handling for invalid/expired tokens

## Completed
✅ Completed November 13, 2025
✅ Invitation-only platform security model confirmed" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_AUTH_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 3.6: Auth-service architecture compliance" \
    "## Description
Ensure auth-service follows all architecture standards.

## Requirements
- camelCase JSON serialization
- NATS event publishing (user.registered)
- AppConfig-based configuration
- Health/ready/metrics endpoints
- All middleware integration

## Acceptance Criteria
- [ ] camelCase responses verified
- [ ] NATS events publishing
- [ ] AppConfig migration complete
- [ ] Observability endpoints working

## Completed
✅ Completed November 14, 2025
✅ Full architecture compliance verified" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_AUTH_ID}" \
    "November 14, 2025"

create_closed_issue \
    "Stage 3.7: Auth-service testing" \
    "## Description
Comprehensive testing of authentication service.

## Testing
- TestContext pattern
- Parallel test execution
- Clean test isolation
- Manual endpoint verification (curl)

## Acceptance Criteria
- [ ] All unit tests passing
- [ ] Integration tests complete
- [ ] Manual testing successful
- [ ] Test coverage documented

## Completed
✅ Completed November 13, 2025
✅ All tests passing" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_AUTH_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 3.8: Production-ready features (graceful shutdown, circuit breakers)" \
    "## Description
Implement production-ready operational features.

## Features
- Graceful shutdown (SIGTERM/Ctrl+C)
- Circuit breaker patterns (Closed/Open/HalfOpen)
- All Priority 1-3 middleware

## Acceptance Criteria
- [ ] Graceful shutdown working
- [ ] Circuit breakers tested
- [ ] All middleware integrated
- [ ] Production deployment ready

## Completed
✅ Completed November 13, 2025
✅ 8/8 circuit breaker tests passing" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_AUTH_ID}" \
    "November 13, 2025"

echo ""
echo "=========================================="
echo "Creating Stage 4 closed issues..."
echo "=========================================="
echo ""

# Stage 4: User Service (24 endpoints, completed Nov 13-14, 2025)

create_closed_issue \
    "Stage 4.1: User-service scaffolding and structure" \
    "## Description
Create user-service crate with all middleware.

## Components
- User-service crate in services/
- Handlers, models, services structure
- All 5 middleware components
- Dependencies configured

## Acceptance Criteria
- [ ] Crate created and compiling
- [ ] Standard structure in place
- [ ] All middleware integrated

## Completed
✅ Completed November 13, 2025" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_USER_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 4.2: User database schema migrations" \
    "## Description
Create comprehensive user data schema.

## Migrations
**20251113000004:** 6 tables, 26 indexes, 2 triggers
- users_profiles
- users_profile_links
- users_language_proficiency
- user_connections
- data_exports
- account_deletion_requests

**20251113000007:** users_settings table
- App preferences (theme, language, timezone)
- Privacy settings
- Notification preferences
- Activity settings

## Acceptance Criteria
- [ ] All tables created
- [ ] Indexes optimized
- [ ] Triggers working
- [ ] Migrations applied successfully

## Completed
✅ Completed November 13, 2025
✅ 7 tables total with full settings support" \
    "${PRIORITY_CRITICAL_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_USER_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 4.3: Profile management endpoints (3 endpoints)" \
    "## Description
Implement user profile CRUD operations.

## Endpoints
1. GET /api/v1/user/profile - Get own profile (auto-created)
2. PUT /api/v1/user/profile - Update profile
3. GET /api/v1/user/profile/{id} - View other profiles

## Acceptance Criteria
- [ ] Auto-profile creation on first access
- [ ] Validation on all fields
- [ ] Privacy settings respected
- [ ] Authorization checks

## Completed
✅ Completed November 13, 2025" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_USER_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 4.4: Profile links endpoints (4 endpoints)" \
    "## Description
Manage user profile links (websites, social media).

## Endpoints
1. GET /api/v1/user/profile/links - List links
2. POST /api/v1/user/profile/links - Create link (max 10)
3. PUT /api/v1/user/profile/links/{id} - Update link
4. DELETE /api/v1/user/profile/links/{id} - Delete link

## Acceptance Criteria
- [ ] Maximum 10 links enforced
- [ ] Display ordering supported
- [ ] URL validation
- [ ] Authorization on all operations

## Completed
✅ Completed November 13, 2025" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_USER_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 4.5: Language proficiency endpoints (4 endpoints)" \
    "## Description
Track user language skills with 4-dimensional proficiency.

## Endpoints
1. GET /api/v1/user/profile/languages - List languages
2. POST /api/v1/user/profile/languages - Add language
3. PUT /api/v1/user/profile/languages/{id} - Update language
4. DELETE /api/v1/user/profile/languages/{id} - Delete language

## Proficiency Dimensions
- Spoken proficiency (6 levels)
- Written proficiency (6 levels)
- Reading proficiency (6 levels)
- Listening proficiency (6 levels)

## Acceptance Criteria
- [ ] 4D skill tracking implemented
- [ ] 6-level proficiency scale
- [ ] Validation on all operations
- [ ] Multiple languages per user

## Completed
✅ Completed November 13, 2025
✅ Full 4D language proficiency tracking" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_USER_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 4.6: User connections endpoints (7 endpoints)" \
    "## Description
Social features: follow/unfollow and block/unblock.

## Endpoints
1. POST /api/v1/user/{id}/follow - Follow user
2. DELETE /api/v1/user/{id}/follow - Unfollow user
3. POST /api/v1/user/{id}/block - Block user
4. DELETE /api/v1/user/{id}/block - Unblock user
5. GET /api/v1/user/{id}/followers - List followers
6. GET /api/v1/user/{id}/following - List following
7. GET /api/v1/user/search - Search users

## Acceptance Criteria
- [ ] Mutual follow relationship tracking
- [ ] Block removes follows automatically
- [ ] Pagination on lists
- [ ] Search with connection status
- [ ] Privacy settings respected

## Completed
✅ Completed November 13, 2025
✅ Full social connection system" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_USER_ID}" \
    "November 13, 2025"

create_closed_issue \
    "Stage 4.7: User settings endpoints (6 endpoints)" \
    "## Description
Complete user preferences and settings management.

## Endpoints
1. GET /api/v1/user/settings - Get all settings
2. PATCH /api/v1/user/settings - Update all settings
3. GET /api/v1/user/settings/privacy - Get privacy
4. PATCH /api/v1/user/settings/privacy - Update privacy
5. GET /api/v1/user/settings/notifications - Get notifications
6. PATCH /api/v1/user/settings/notifications - Update notifications

## Settings Categories
- App preferences (theme, language, timezone)
- Privacy (profile visibility, contact permissions)
- Notifications (email, badge, course, forum)
- Activity (show activity, online status)

## Acceptance Criteria
- [ ] Auto-create defaults on first access
- [ ] PATCH for partial updates
- [ ] Validation on all settings
- [ ] Settings merged into user-service (not separate service)

## Completed
✅ Completed November 14, 2025
✅ Settings-service merged for MVP simplicity" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_USER_ID}" \
    "November 14, 2025"

create_closed_issue \
    "Stage 4.8: User-service security and testing" \
    "## Description
Comprehensive security verification and testing.

## Security
- Defense-in-depth authorization (JWT + handler + service + DB)
- Users cannot modify other users' data
- Privacy settings framework

## Testing
- Endpoint testing (profiles, links, languages, connections, settings)
- Authorization verification
- OpenAPI/Swagger documentation

## Acceptance Criteria
- [ ] All security checks passing
- [ ] Users isolated from each other's data
- [ ] All endpoints tested
- [ ] Swagger docs generated

## Completed
✅ Completed November 13-14, 2025
✅ 24 endpoints fully secured and tested" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_USER_ID}" \
    "November 14, 2025"

create_closed_issue \
    "Stage 4.9: User-service architecture compliance" \
    "## Description
Ensure user-service follows all architecture standards.

## Requirements
- camelCase JSON serialization on all models
- AppConfig migration
- NATS client initialized
- Health/ready/metrics endpoints
- MetricsCollector integration

## Acceptance Criteria
- [ ] All models camelCase verified
- [ ] AppConfig integration complete
- [ ] NATS connected
- [ ] Full observability stack

## Completed
✅ Completed November 14, 2025
✅ Full architecture compliance
✅ Prometheus metrics integrated" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_USER_ID}" \
    "November 14, 2025"

echo ""
echo "=========================================="
echo "✅ All Stages 1-4 issues created!"
echo "=========================================="
echo ""
echo "Summary:"
echo "- Stage 1: 6 closed issues (Foundation & Infrastructure)"
echo "- Stage 2: 3 closed issues (Database Schema & Migrations)"
echo "- Stage 3: 8 closed issues (Authentication Service)"
echo "- Stage 4: 9 closed issues (User Service)"
echo "- Total: 26 closed issues representing work from Nov 4-14, 2025"
echo ""
echo "All issues available at: ${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues?state=closed"
