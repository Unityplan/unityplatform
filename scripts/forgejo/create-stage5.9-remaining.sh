#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=2
PRIORITY_MEDIUM_ID=3
TYPE_INFRASTRUCTURE_ID=8
AREA_INVITATION_SERVICE_ID=26
AREA_AUTH_SERVICE_ID=9
AREA_FRONTEND_ID=14

# Milestone ID
MILESTONE_ID=1

# Function to create issue with proper JSON escaping
create_issue_with_file() {
    local title="$1"
    local body_file="$2"
    local label_ids="$3"
    
    # Convert label IDs to JSON array
    labels_json=$(printf '%s\n' ${label_ids//,/ } | jq -R 'tonumber' | jq -s .)
    
    # Read body from file and escape it properly with jq
    body=$(cat "$body_file" | jq -Rs .)
    
    # Create JSON payload
    payload=$(jq -n \
        --arg title "$title" \
        --argjson body "$body" \
        --argjson labels "$labels_json" \
        --argjson milestone "$MILESTONE_ID" \
        '{title: $title, body: $body, labels: $labels, milestone: $milestone}')
    
    # Create the issue
    response=$(curl -s -X POST \
        -H "Authorization: token ${FORGEJO_TOKEN}" \
        -H "Content-Type: application/json" \
        "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues" \
        -d "$payload")
    
    issue_number=$(echo "$response" | jq -r '.number')
    
    if [ "$issue_number" != "null" ] && [ -n "$issue_number" ]; then
        echo "✅ Created issue #${issue_number}: ${title}"
        echo "${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}"
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

# Create temp directory for issue bodies
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "==========================================  "
echo "Creating remaining Stage 5.9 issues..."
echo "=========================================="
echo ""

# Issue 5.9.2
cat > "$TEMP_DIR/5.9.2.md" << 'EOF'
## Description
Apply the invitation service migration to the Denmark pod database.

## Acceptance Criteria
- [ ] Migration applied successfully to `unityplatform_dk` database
- [ ] All tables created in global schema
- [ ] All tables created in territory_dk schema
- [ ] Migration recorded in `shared_lib_migrations` table
- [ ] Database schema validated with `\dt territory_dk.invitation*`
- [ ] Test data created for development (optional)

## Commands
```bash
# Apply migration
psql -h localhost -p 5432 -U unityplatform -d unityplatform_dk \
  -f services/shared-lib/migrations/20251118000009_invitation_service_tables.sql

# Verify tables
psql -h localhost -p 5432 -U unityplatform -d unityplatform_dk \
  -c "\dt global.registry_invitation" \
  -c "\dt territory_dk.invitation_invitations_*"
```

## Dependencies
- #137 (5.9.1: database migration must exist)
EOF

create_issue_with_file \
    "5.9.2: Apply invitation service migration to DK pod" \
    "$TEMP_DIR/5.9.2.md" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INVITATION_SERVICE_ID}"

# Issue 5.9.3
cat > "$TEMP_DIR/5.9.3.md" << 'EOF'
## Description
Document the invitation service tables in the master migrations documentation.

## Acceptance Criteria
- [ ] Add invitation tables to migration tracking
- [ ] Document naming convention rationale
- [ ] Add examples in "Service Table Naming" section
- [ ] Update "Migration History" section
- [ ] Git commit with message: `docs: add invitation service tables to MIGRATIONS-MASTER`

## Reference
`docs/architecture/MIGRATIONS-MASTER.md`

## Dependencies
- #137 (5.9.1: migration must exist to document)
EOF

create_issue_with_file \
    "5.9.3: Update MIGRATIONS-MASTER.md with invitation tables" \
    "$TEMP_DIR/5.9.3.md" \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INVITATION_SERVICE_ID}"

# Issue 5.9.4
cat > "$TEMP_DIR/5.9.4.md" << 'EOF'
## Description
Create the Rust project structure for invitation-service following the standard service pattern.

## Acceptance Criteria
- [ ] Create `services/invitation-service/` directory
- [ ] Create `Cargo.toml` with dependencies
- [ ] Create standard directory structure:
  - `src/main.rs` - Server setup with middleware
  - `src/lib.rs` - Public exports
  - `src/handlers/` - HTTP request handlers
  - `src/models/` - Request/Response types
  - `src/services/` - Business logic
- [ ] Configure shared-lib dependencies
- [ ] Add to workspace `services/Cargo.toml`
- [ ] Service builds successfully with `cargo build -p invitation-service`

## Standard Dependencies
```toml
[dependencies]
actix-web = "4.5"
serde = { version = "1.0", features = ["derive"] }
shared_lib = { path = "../shared-lib" }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
validator = { version = "0.16", features = ["derive"] }
rand = "0.8"
```

## Reference
See `services/auth-service/` for structure pattern

## Dependencies
- #137 (5.9.1: database tables must exist)
EOF

create_issue_with_file \
    "5.9.4: Create invitation-service project structure" \
    "$TEMP_DIR/5.9.4.md" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INVITATION_SERVICE_ID}"

# Issue 5.9.8
cat > "$TEMP_DIR/5.9.8.md" << 'EOF'
## Description
Implement POST /api/v1/invitations endpoint with manager role authorization.

## Acceptance Criteria
- [ ] Implement `create_invitation()` handler
- [ ] Require JWT authentication (shared-lib middleware)
- [ ] Verify manager role from JWT claims (territory or community manager)
- [ ] Validate request body (max_uses, expires_in_days)
- [ ] Call `invitation_service::create_invitation()`
- [ ] Return invitation token and invite URL
- [ ] Return 403 if user is not a manager
- [ ] Publish NATS event: `invitation.created`
- [ ] Integration test with manager JWT
- [ ] Integration test with non-manager JWT (403 expected)

## Reference
`docs/architecture/services/invitation-service/API.md`

## Dependencies
- #138 (5.9.5: token generation service must exist)
EOF

create_issue_with_file \
    "5.9.8: Implement manager-only invitation creation endpoint" \
    "$TEMP_DIR/5.9.8.md" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INVITATION_SERVICE_ID}"

# Issue 5.9.10
cat > "$TEMP_DIR/5.9.10.md" << 'EOF'
## Description
Update auth-service registration to enforce invitation-only registration in production.

## Acceptance Criteria
- [ ] Add environment variable: `ALLOW_OPEN_REGISTRATION` (default: false)
- [ ] Update `AppConfig` in shared-lib or auth-service config
- [ ] Update registration handler:
  - If production mode: require `invitation_token` field (non-optional)
  - If dev mode: allow optional `invitation_token` for testing
- [ ] Call invitation-service `/validate` endpoint (HTTP client)
- [ ] Call invitation-service `/use` endpoint after successful registration
- [ ] Handle invitation-service errors gracefully
- [ ] Return clear error if invitation required but missing
- [ ] Update API documentation
- [ ] Integration tests for both modes

## Reference
`docs/architecture/services/auth-service/README.md`

## Dependencies
- #140 (5.9.7: invitation usage endpoint must exist)
EOF

create_issue_with_file \
    "5.9.10: Update auth-service registration with production mode" \
    "$TEMP_DIR/5.9.10.md" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_AUTH_SERVICE_ID}"

# Issue 5.9.11
cat > "$TEMP_DIR/5.9.11.md" << 'EOF'
## Description
Create HTTP client for auth-service to communicate with invitation-service.

## Acceptance Criteria
- [ ] Create `src/clients/invitation_client.rs` in auth-service
- [ ] Implement `validate()` method (POST /api/v1/invitations/validate)
- [ ] Implement `mark_used()` method (POST /api/v1/invitations/use)
- [ ] Use `reqwest` for HTTP requests
- [ ] Handle network errors gracefully
- [ ] Add request timeout (5 seconds)
- [ ] Add retry logic for transient failures (optional)
- [ ] Unit tests with mocked HTTP responses
- [ ] Configuration: `INVITATION_SERVICE_URL` environment variable

## Dependencies
- #139 (5.9.6: validation endpoint must exist)
- #140 (5.9.7: usage endpoint must exist)
EOF

create_issue_with_file \
    "5.9.11: Create invitation-service HTTP client for auth-service" \
    "$TEMP_DIR/5.9.11.md" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_AUTH_SERVICE_ID}"

# Issue 5.9.15
cat > "$TEMP_DIR/5.9.15.md" << 'EOF'
## Description
Update the registration page to handle invitation tokens.

## Acceptance Criteria
- [ ] Update `RegisterPage.tsx` to accept `?invite=TOKEN` query parameter
- [ ] Pre-fill invitation token field from query param
- [ ] Show invitation token input field
- [ ] Validate invitation format (XXXX-XXXX-XXXX-XXXX)
- [ ] Call `invitationApi.validateInvitation()` on blur (optional UX)
- [ ] Include invitation token in registration request
- [ ] Handle "invitation required" error from backend
- [ ] Show helpful error if invitation invalid/expired
- [ ] Dev mode: Make field optional with info message
- [ ] Production mode: Make field required

## Dependencies
- #145 (5.9.10: auth-service must support invitation validation)
- #142 (5.9.12: API client must exist)
EOF

create_issue_with_file \
    "5.9.15: Update registration page with invitation token input" \
    "$TEMP_DIR/5.9.15.md" \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_FRONTEND_ID}"

echo ""
echo "=========================================="
echo "✅ Remaining Stage 5.9 issues created!"
echo "=========================================="
