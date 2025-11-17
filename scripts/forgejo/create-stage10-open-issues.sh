#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=1
TYPE_FEATURE_ID=5
TYPE_INFRASTRUCTURE_ID=8
AREA_FORUM_SERVICE_ID=22

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
echo "Creating Stage 10 open issues (Forum Service)..."
echo "=========================================="
echo ""

# Stage 10: Forum Service - 8 tasks total

create_open_issue \
    "Stage 10.1: Create forum-service scaffolding" \
    "## Description
Create forum-service Rust crate with basic structure and Matrix integration.

## Tasks
- Create forum-service crate in services/ workspace
- Set up Cargo.toml with dependencies (actix-web, matrix-sdk, sqlx)
- Create main.rs with actix-web server
- Initialize Matrix client connection
- Add shared-lib middleware stack
- Create basic project structure (handlers/, models/, services/)
- Add health check endpoint

## Acceptance Criteria
- [ ] Service compiles without errors
- [ ] Matrix client connects successfully
- [ ] Health check endpoint returns OK
- [ ] Middleware stack applied (logging, CORS, rate limiting)
- [ ] Port 8018 configured

## Technical Notes
- Use shared_lib for middleware and config
- Use matrix-sdk for Matrix protocol integration
- Follow service creation pattern from other services
- Forums are backed by Matrix rooms

## Dependencies
- Stage 9: IPFS Service
- matrix-bridge service (Stage 8)
- shared-lib v0.1.0-alpha.1" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FORUM_SERVICE_ID}"

create_open_issue \
    "Stage 10.2: Create forum database schema" \
    "## Description
Create database schema for forum categories, topics, and posts with Matrix room references.

## Tasks
- Create migration 20251117000012_forum_schema.sql
- forum_categories table (id, territory_code, name, slug, description)
- forum_topics table (id, category_id, title, slug, matrix_room_id, created_by, locked)
- forum_posts table (id, topic_id, user_id, content, matrix_event_id, edited_at)
- forum_reactions table (id, post_id, user_id, reaction_type)
- forum_moderation table (id, user_id, post_id, action, reason, moderator_id)
- Add indexes for performance
- Add foreign key constraints

## Acceptance Criteria
- [ ] Migration runs successfully
- [ ] All tables created in territory schema
- [ ] Foreign keys properly configured
- [ ] Indexes added for common queries

## Technical Notes
- Store Matrix room_id for topic synchronization
- Store Matrix event_id for post synchronization
- Use territory_{code}.forum_* tables
- Slug fields for SEO-friendly URLs

## Dependencies
- Stage 10.1: forum-service scaffolding
- Database infrastructure (Stage 2)" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_FORUM_SERVICE_ID}"

create_open_issue \
    "Stage 10.3: Implement Matrix room integration" \
    "## Description
Integrate Matrix protocol for federated forum functionality.

## Tasks
- Create Matrix room when forum topic is created
- Set room name, topic, and permissions
- Store room_id in forum_topics table
- Sync forum posts to Matrix messages
- Listen for Matrix messages and sync to forum
- Handle Matrix events (edits, deletions, reactions)
- Implement bidirectional synchronization

## Acceptance Criteria
- [ ] Matrix room created automatically for new topics
- [ ] Posts synced to Matrix in real-time
- [ ] Matrix messages appear as forum posts
- [ ] Edits and deletions synced both ways
- [ ] Reactions synced between forum and Matrix

## Technical Notes
- Use matrix-sdk for room creation and messaging
- Store event_id for message correlation
- Handle sync conflicts gracefully
- Use Matrix webhooks/sync API for real-time updates
- Room naming: #topic-slug:territory.domain

## Dependencies
- Stage 10.2: forum database schema
- matrix-bridge service (Stage 8)" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FORUM_SERVICE_ID}"

create_open_issue \
    "Stage 10.4: Implement forum category endpoints" \
    "## Description
Implement endpoints for listing and managing forum categories.

## Tasks
- Create GET /forum/categories endpoint
- List all categories for territory
- Include category metadata (topic count, post count)
- Add pagination support
- Create handlers/categories.rs
- Add validation and error handling

## Acceptance Criteria
- [ ] Endpoint returns list of categories
- [ ] Categories filtered by territory
- [ ] Metadata includes counts and timestamps
- [ ] Proper error handling for invalid requests
- [ ] Pagination working

## Technical Notes
- Query territory_{code}.forum_categories
- Join with topics and posts for counts
- Cache category list (Redis, 5 min TTL)
- Return JSON with camelCase

## Dependencies
- Stage 10.3: Matrix integration" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FORUM_SERVICE_ID}"

create_open_issue \
    "Stage 10.5: Implement forum topic endpoints" \
    "## Description
Implement endpoints for creating, listing, and managing forum topics.

## Tasks
- Create GET /forum/categories/{slug}/topics endpoint
- Create POST /forum/topics endpoint (creates Matrix room)
- Create GET /forum/topics/{slug} endpoint
- Create POST /forum/topics/{topic_id}/lock endpoint
- Add pagination for topic lists
- Create handlers/topics.rs
- Add validation and authorization

## Acceptance Criteria
- [ ] List topics by category with pagination
- [ ] Create topic with Matrix room integration
- [ ] Get topic with posts and metadata
- [ ] Lock/unlock topics (moderators only)
- [ ] Proper authorization checks

## Technical Notes
- Call matrix-bridge to create room on topic creation
- Store matrix_room_id in database
- Only moderators can lock topics
- Include post count and last activity
- Locked topics prevent new posts

## Dependencies
- Stage 10.4: category endpoints" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FORUM_SERVICE_ID}"

create_open_issue \
    "Stage 10.6: Implement forum post endpoints" \
    "## Description
Implement endpoints for creating, editing, and managing forum posts.

## Tasks
- Create POST /forum/topics/{topic_id}/posts endpoint
- Create PUT /forum/posts/{post_id} endpoint
- Create DELETE /forum/posts/{post_id} endpoint
- Create POST /forum/posts/{post_id}/reactions endpoint
- Sync posts with Matrix messages
- Create handlers/posts.rs
- Add validation and authorization

## Acceptance Criteria
- [ ] Create posts and sync to Matrix
- [ ] Edit posts (author only, within time limit)
- [ ] Delete posts (author or moderator)
- [ ] Add reactions (emoji) to posts
- [ ] Proper authorization for all actions

## Technical Notes
- Send Matrix message when post created
- Edit Matrix message when post edited
- Redact Matrix message when post deleted
- Store matrix_event_id for correlation
- Time limit for edits (30 minutes)
- Support Markdown formatting

## Dependencies
- Stage 10.5: topic endpoints" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FORUM_SERVICE_ID}"

create_open_issue \
    "Stage 10.7: Implement forum moderation system" \
    "## Description
Implement moderation tools for managing forum content and users.

## Tasks
- Create POST /forum/posts/{post_id}/flag endpoint
- Create GET /forum/moderation/queue endpoint
- Create POST /forum/moderation/strike endpoint
- Create POST /forum/moderation/ban endpoint
- Create handlers/moderation.rs
- Implement strike tracking system
- Add moderator role checks

## Acceptance Criteria
- [ ] Users can flag inappropriate posts
- [ ] Moderators see flagged content in queue
- [ ] Moderators can issue strikes to users
- [ ] 3-strike system leads to temporary ban
- [ ] Only moderators can access moderation endpoints

## Technical Notes
- Store strikes in forum_moderation table
- Track strike count per user
- Automatic ban after 3 strikes (7 days)
- Notify user via notification system
- Moderators determined by community role
- Ban prevents posting but allows reading

## Dependencies
- Stage 10.6: post endpoints" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FORUM_SERVICE_ID}"

create_open_issue \
    "Stage 10.8: Forum service testing and compliance" \
    "## Description
Ensure forum-service has comprehensive testing and follows platform standards.

## Architecture Compliance
- [ ] camelCase JSON serialization
- [ ] AppConfig integration
- [ ] Health/ready/metrics endpoints
- [ ] Logging with tracing_subscriber
- [ ] Service added to dev scripts
- [ ] Error handling follows patterns

## Testing
- [ ] Unit tests for all handlers
- [ ] Integration tests with Matrix
- [ ] Bidirectional sync tests
- [ ] Moderation workflow tests
- [ ] Authorization tests
- [ ] Test coverage >80%

## Documentation
- [ ] README.md with service overview
- [ ] API.md with all endpoints
- [ ] MATRIX-INTEGRATION.md
- [ ] MODERATION.md with policies
- [ ] Update CHANGELOG.md

## Acceptance Criteria
- [ ] All compliance checks passing
- [ ] Matrix sync tested in both directions
- [ ] Moderation system fully tested
- [ ] All endpoints documented
- [ ] Service follows platform patterns

## Technical Notes
- Use testcontainers for Matrix integration tests
- Mock Matrix SDK where appropriate
- Test edge cases (concurrent edits, sync conflicts)
- Verify rate limiting works
- Document Matrix room structure

## Dependencies
- Stage 10.7: moderation system" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FORUM_SERVICE_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 10 issues created!"
echo "=========================================="
echo ""
echo "Summary: Created 8 open issues for Stage 10 (Forum Service)"
echo ""
