#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=1
TYPE_FEATURE_ID=5
TYPE_INFRASTRUCTURE_ID=8
AREA_IPFS_SERVICE_ID=21

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
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

echo "=========================================="
echo "Creating Stage 9 open issues (IPFS Service)..."
echo "=========================================="
echo ""

# Stage 9: IPFS Service - 6 tasks total

create_open_issue \
    "Stage 9.1: Add IPFS to Docker infrastructure" \
    "## Description
Add IPFS to docker-compose infrastructure for decentralized content storage.

## Tasks
- Add IPFS container to docker-compose.yml
- Configure IPFS initialization scripts
- Set up IPFS data volumes
- Configure IPFS API and gateway ports
- Test IPFS container startup

## Acceptance Criteria
- [ ] IPFS container runs successfully
- [ ] IPFS API accessible from other containers
- [ ] IPFS data persists across container restarts
- [ ] Gateway accessible for content retrieval

## Technical Notes
- Use official IPFS Docker image (ipfs/kubo)
- Configure IPFS for private swarm (territory-specific)
- Set up proper volume mounts for data persistence
- Expose API port 5001 and gateway port 8080

## Dependencies
- Stage 8: Matrix Protocol Integration" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_IPFS_SERVICE_ID}"

create_open_issue \
    "Stage 9.2: Create ipfs-service scaffolding" \
    "## Description
Create ipfs-service Rust crate with basic structure and IPFS client integration.

## Tasks
- Create ipfs-service crate in services/ workspace
- Set up Cargo.toml with ipfs-api dependency
- Create main.rs with actix-web server
- Initialize IPFS client connection
- Add shared-lib middleware stack
- Create basic project structure (handlers/, models/, services/)
- Add health check endpoint

## Acceptance Criteria
- [ ] Service compiles without errors
- [ ] IPFS client connects successfully
- [ ] Health check endpoint returns OK
- [ ] Middleware stack applied (logging, CORS, rate limiting)
- [ ] Port 8017 configured

## Technical Notes
- Use shared_lib for middleware and config
- Use ipfs-api-backend-actix for async IPFS operations
- Follow service creation pattern from other services
- Store metadata in database for uploaded files

## Dependencies
- Stage 9.1: IPFS Docker setup
- shared-lib v0.1.0-alpha.1" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_IPFS_SERVICE_ID}"

create_open_issue \
    "Stage 9.3: Implement IPFS file upload endpoint" \
    "## Description
Implement POST /ipfs/upload endpoint for uploading files to IPFS.

## Tasks
- Create upload handler in handlers/upload.rs
- Implement multipart file parsing
- Upload file to IPFS and get CID
- Store file metadata in database (CID, filename, size, content_type)
- Return CID and metadata to client
- Add file size limits and validation
- Handle upload errors gracefully

## Acceptance Criteria
- [ ] Endpoint accepts multipart file uploads
- [ ] Files uploaded to IPFS successfully
- [ ] CID returned to client
- [ ] Metadata stored in territory database
- [ ] Proper error handling for invalid files
- [ ] File size limits enforced (50MB max)

## Technical Notes
- Use actix-multipart for file uploads
- Store metadata in territory_{code}.ipfs_content table
- Validate content types (images, PDFs, videos for courses)
- Auto-pin uploaded content

## Database Migration
- Create 20251117000011_ipfs_content.sql migration
- Add ipfs_content table (id, cid, filename, size, content_type, uploaded_by, created_at)

## Dependencies
- Stage 9.2: ipfs-service scaffolding" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_IPFS_SERVICE_ID}"

create_open_issue \
    "Stage 9.4: Implement IPFS content retrieval endpoint" \
    "## Description
Implement GET /ipfs/{cid} endpoint to retrieve file metadata and content.

## Tasks
- Create retrieval handler in handlers/retrieve.rs
- Fetch metadata from database by CID
- Retrieve content from IPFS
- Stream content to client with proper content-type
- Add caching headers
- Handle missing content gracefully

## Acceptance Criteria
- [ ] Endpoint returns file metadata for valid CID
- [ ] Content streamed efficiently from IPFS
- [ ] Proper content-type headers set
- [ ] 404 error for missing CID
- [ ] Caching headers for CDN compatibility
- [ ] Large files stream without buffering

## Technical Notes
- Use IPFS cat API for content retrieval
- Set cache headers (1 year for immutable IPFS content)
- Stream large files efficiently using actix streaming
- Query database first to verify ownership/permissions

## Dependencies
- Stage 9.3: IPFS upload endpoint" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_IPFS_SERVICE_ID}"

create_open_issue \
    "Stage 9.5: Implement IPFS pin/unpin endpoints" \
    "## Description
Implement endpoints for pinning and unpinning IPFS content.

## Tasks
- Create POST /ipfs/{cid}/pin endpoint
- Create DELETE /ipfs/{cid}/pin endpoint
- Update database with pin status
- Implement IPFS pin API calls
- Add authorization (only content owner or admin can unpin)
- Handle pinning errors

## Acceptance Criteria
- [ ] Pin endpoint pins content to local IPFS node
- [ ] Unpin endpoint removes pin (with authorization)
- [ ] Pin status tracked in database
- [ ] Proper error handling for missing content
- [ ] Reference counting prevents premature unpinning

## Technical Notes
- Use IPFS pin add/rm API
- Store pin_count in database (multiple references)
- Only unpin when pin_count reaches 0
- Require admin or content owner for unpin
- Auto-pin content on upload

## Dependencies
- Stage 9.4: IPFS content retrieval" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_IPFS_SERVICE_ID}"

create_open_issue \
    "Stage 9.6: IPFS service architecture compliance and testing" \
    "## Description
Ensure ipfs-service follows platform architecture standards and has comprehensive testing.

## Architecture Compliance
- [ ] camelCase JSON serialization
- [ ] AppConfig integration
- [ ] Health/ready/metrics endpoints
- [ ] Logging with tracing_subscriber
- [ ] Service added to dev scripts
- [ ] Error handling follows patterns

## Testing
- [ ] Unit tests for handlers
- [ ] Integration tests with IPFS
- [ ] Upload/download roundtrip tests
- [ ] Pin/unpin lifecycle tests
- [ ] Error handling tests
- [ ] Test coverage >80%

## Documentation
- [ ] README.md with service overview
- [ ] API.md with all endpoints
- [ ] IPFS-INTEGRATION.md with architecture
- [ ] Update CHANGELOG.md

## Acceptance Criteria
- [ ] All compliance checks passing
- [ ] Integration tests successful
- [ ] Documentation complete
- [ ] Ready for course and forum integration

## Technical Notes
- Use testcontainers for IPFS integration tests
- Test with various file types and sizes
- Verify metadata storage and retrieval
- Document CID format and usage

## Dependencies
- Stage 9.5: Pin management" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_IPFS_SERVICE_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 9 issues created!"
echo "=========================================="
echo ""
echo "Summary: Created 6 open issues for Stage 9 (IPFS Service)"
echo ""
