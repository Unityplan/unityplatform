#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=1
PRIORITY_MEDIUM_ID=2
TYPE_FEATURE_ID=5
AREA_TRANSLATION_SERVICE_ID=23

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
echo "Creating Stage 11 open issues (Translation Service)..."
echo "=========================================="
echo ""

# Stage 11: Translation Service - 3 tasks total

create_open_issue \
    "Stage 11.1: Create translation-service scaffolding" \
    "## Description
Create translation-service Rust crate with basic structure and Redis caching.

## Tasks
- Create translation-service crate in services/ workspace
- Set up Cargo.toml with dependencies (actix-web, reqwest, redis)
- Create main.rs with actix-web server
- Initialize Redis client connection
- Add shared-lib middleware stack
- Create basic project structure (handlers/, models/, services/)
- Add health check endpoint

## Acceptance Criteria
- [ ] Service compiles without errors
- [ ] Redis client connects successfully
- [ ] Health check endpoint returns OK
- [ ] Middleware stack applied (logging, CORS, rate limiting)
- [ ] Port 8019 configured

## Technical Notes
- Use shared_lib for middleware and config
- Use reqwest for HTTP calls to translation API
- Use redis crate for caching
- Follow service creation pattern from other services
- Translation API: LibreTranslate or similar

## Dependencies
- Stage 10: Forum Service
- Redis infrastructure
- shared-lib v0.1.0-alpha.1" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_TRANSLATION_SERVICE_ID}"

create_open_issue \
    "Stage 11.2: Implement translation endpoint with caching" \
    "## Description
Implement POST /translate endpoint with Redis caching for translations.

## Tasks
- Create POST /translate endpoint in handlers/translate.rs
- Request body: {text, sourceLang, targetLang}
- Check Redis cache first (key: hash of text+langs)
- If not cached, call translation API
- Store result in Redis (TTL: 7 days)
- Return translated text
- Handle rate limiting
- Add error handling

## Acceptance Criteria
- [ ] Endpoint accepts translation requests
- [ ] Cache checked before API call
- [ ] Translations cached in Redis
- [ ] Cache hit returns immediately
- [ ] Cache miss calls translation API
- [ ] Proper error handling
- [ ] Rate limiting applied

## Technical Notes
- Use LibreTranslate API (self-hosted or cloud)
- Cache key: SHA256(text + source + target)
- Cache TTL: 604800 seconds (7 days)
- Support common languages: en, da, no, sv, de, fr, es
- Return JSON: {translatedText, sourceLang, targetLang, cached}

## Translation API Configuration
- Set TRANSLATION_API_URL in environment
- Set TRANSLATION_API_KEY if needed
- Consider self-hosted LibreTranslate for privacy

## Dependencies
- Stage 11.1: translation-service scaffolding" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_TRANSLATION_SERVICE_ID}"

create_open_issue \
    "Stage 11.3: Translation service testing and compliance" \
    "## Description
Ensure translation-service has comprehensive testing and follows platform standards.

## Architecture Compliance
- [ ] camelCase JSON serialization
- [ ] AppConfig integration
- [ ] Health/ready/metrics endpoints
- [ ] Logging with tracing_subscriber
- [ ] Service added to dev scripts
- [ ] Error handling follows patterns

## Testing
- [ ] Unit tests for handlers
- [ ] Cache hit/miss tests
- [ ] Translation API integration tests
- [ ] Error handling tests
- [ ] Rate limiting tests
- [ ] Test coverage >80%

## Documentation
- [ ] README.md with service overview
- [ ] API.md with endpoints
- [ ] TRANSLATION-API.md with configuration
- [ ] Supported languages documented
- [ ] Update CHANGELOG.md

## Acceptance Criteria
- [ ] All compliance checks passing
- [ ] Cache working correctly
- [ ] Translation API integration tested
- [ ] All endpoints documented
- [ ] Service follows platform patterns

## Technical Notes
- Mock translation API for tests
- Test cache expiration
- Test with various languages
- Verify rate limiting behavior
- Document supported language codes (ISO 639-1)

## Dependencies
- Stage 11.2: translation endpoint" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_TRANSLATION_SERVICE_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 11 issues created!"
echo "=========================================="
echo ""
echo "Summary: Created 3 open issues for Stage 11 (Translation Service)"
echo ""
