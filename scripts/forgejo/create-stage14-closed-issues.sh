#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=1
PRIORITY_MEDIUM_ID=2
TYPE_FEATURE_ID=5
TYPE_INFRASTRUCTURE_ID=8
AREA_UTILITY_SERVICE_ID=13
AREA_TERRITORY_SERVICE_ID=12
AREA_FRONTEND_APP_ID=14

# Milestone ID
MILESTONE_ID=1  # v0.1.0-alpha.2

# Completion date
COMPLETED_DATE="2025-11-16T23:59:59Z"

# Function to create closed issue
create_closed_issue() {
    local title="$1"
    local body="$2"
    local label_ids="$3"
    
    # Convert label IDs to JSON array of numbers
    labels_json=$(printf '%s\n' ${label_ids//,/ } | jq -R 'tonumber' | jq -s .)
    
    # Create the issue as closed
    response=$(curl -s -X POST \
        -H "Authorization: token ${FORGEJO_TOKEN}" \
        -H "Content-Type: application/json" \
        "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues" \
        -d "{
            \"title\": \"${title}\",
            \"body\": \"${body}\",
            \"labels\": ${labels_json},
            \"milestone\": ${MILESTONE_ID},
            \"closed\": true
        }")
    
    issue_number=$(echo "$response" | jq -r '.number')
    
    if [ "$issue_number" != "null" ]; then
        echo "✅ Created closed issue #${issue_number}: ${title}"
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

echo "=========================================="
echo "Creating Stage 14 closed issues (Utility Service & Language Registry)..."
echo "Stage 14 completed: November 16, 2025"
echo "=========================================="
echo ""

# Stage 14: Utility Service & Language Registry - 18 tasks total

# Step 14.1: Utility Service Scaffolding (2 tasks)

create_closed_issue \
    "Stage 14.1: Create utility-service crate and structure" \
    "## Description
Create utility-service Rust crate with port 8014 and standard service structure.

## Tasks Completed
- ✅ Created utility-service crate in services/ workspace
- ✅ Set up Cargo.toml with dependencies
- ✅ Created service structure (handlers/, models/)
- ✅ Configured port 8014
- ✅ Added shared-lib middleware stack

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_UTILITY_SERVICE_ID}"

# Step 14.2: Favicon Fetching (4 tasks)

create_closed_issue \
    "Stage 14.2.1: Implement favicon endpoint with URL validation" \
    "## Description
Implement GET /utilities/favicon endpoint with URL validation.

## Tasks Completed
- ✅ Created GET /utilities/favicon?url= endpoint
- ✅ Implemented URL validation and sanitization
- ✅ Added SSRF protection (no localhost/private IPs)
- ✅ Implemented favicon fetching logic

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_UTILITY_SERVICE_ID}"

create_closed_issue \
    "Stage 14.2.2: Add SSRF protection and size limits" \
    "## Description
Add security measures for favicon fetching.

## Tasks Completed
- ✅ SSRF protection blocking private IPs
- ✅ 1MB file size limit enforced
- ✅ Timeout handling (5 second max)
- ✅ Content-type validation

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_UTILITY_SERVICE_ID}"

create_closed_issue \
    "Stage 14.2.3: Integrate Redis caching with 7-day TTL" \
    "## Description
Add Redis caching to favicon endpoint.

## Tasks Completed
- ✅ Redis client integration
- ✅ 7-day TTL for cached favicons
- ✅ Cache key: URL hash
- ✅ Binary data caching

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_UTILITY_SERVICE_ID}"

create_closed_issue \
    "Stage 14.2.4: Add cache HIT/MISS tracking" \
    "## Description
Add cache performance tracking for favicon endpoint.

## Tasks Completed
- ✅ HIT/MISS headers in response
- ✅ Cache statistics logging
- ✅ Performance monitoring

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_UTILITY_SERVICE_ID}"

# Step 14.3: Language Registry (3 tasks)

create_closed_issue \
    "Stage 14.3.1: Create language registry database migration" \
    "## Description
Create migration 20251113000008 for global.registry_languages table.

## Tasks Completed
- ✅ Created migration file
- ✅ registry_languages table in global schema
- ✅ Columns: code, name_english, name_native, iso_639_3

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TERRITORY_SERVICE_ID}"

create_closed_issue \
    "Stage 14.3.2: Seed 25 initial languages (ISO 639-3)" \
    "## Description
Seed initial language data with ISO 639-3 standard codes.

## Tasks Completed
- ✅ Seeded 25 common languages
- ✅ ISO 639-3 codes (eng, dan, nor, swe, deu, etc.)
- ✅ Native and English names

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TERRITORY_SERVICE_ID}"

create_closed_issue \
    "Stage 14.3.3: Implement language search and list endpoints" \
    "## Description
Add language registry endpoints to territory-service.

## Tasks Completed
- ✅ GET /territories/languages (list all)
- ✅ GET /territories/languages/search?q= (search)
- ✅ Pagination support
- ✅ Case-insensitive search

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_TERRITORY_SERVICE_ID}"

# Step 14.4: Frontend Integration (5 tasks)

create_closed_issue \
    "Stage 14.4.1: Create ProfileLinksManager with favicon fetching" \
    "## Description
Create ProfileLinksManager component with automatic favicon fetching.

## Tasks Completed
- ✅ ProfileLinksManager component created
- ✅ Automatic favicon fetching from utility-service
- ✅ Add/edit/delete profile links
- ✅ URL validation

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_closed_issue \
    "Stage 14.4.2: Implement favicon display with Globe fallback" \
    "## Description
Display favicons with graceful fallback to Globe icon.

## Tasks Completed
- ✅ Favicon image display
- ✅ Globe icon fallback for errors
- ✅ Loading states
- ✅ Error handling

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_closed_issue \
    "Stage 14.4.3: Create LanguageProficiencyManager component" \
    "## Description
Create LanguageProficiencyManager with table view and search.

## Tasks Completed
- ✅ LanguageProficiencyManager component
- ✅ Table view for language proficiencies
- ✅ Language search integration
- ✅ Proficiency level selection (A1-C2)
- ✅ Add/edit/delete functionality

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_closed_issue \
    "Stage 14.4.4: Create TagInput component for skills/interests" \
    "## Description
Create reusable TagInput component.

## Tasks Completed
- ✅ TagInput component created
- ✅ Add/remove tags functionality
- ✅ Keyboard support (Enter to add, Backspace to remove)
- ✅ Used for skills and interests

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_closed_issue \
    "Stage 14.4.5: Create ResponsiveDialog for desktop/mobile patterns" \
    "## Description
Create ResponsiveDialog component for consistent mobile/desktop UX.

## Tasks Completed
- ✅ ResponsiveDialog component created
- ✅ Dialog on desktop (>768px)
- ✅ Drawer on mobile (<768px)
- ✅ Consistent API across devices

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

# Step 14.5: Documentation & DevOps (4 tasks)

create_closed_issue \
    "Stage 14.5.1: Complete utility-service documentation" \
    "## Description
Document utility-service with README.md and API.md.

## Tasks Completed
- ✅ README.md with service overview
- ✅ API.md with endpoint documentation
- ✅ Configuration documentation
- ✅ Cache behavior documented

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_UTILITY_SERVICE_ID}"

create_closed_issue \
    "Stage 14.5.2: Update architecture overview with utility-service" \
    "## Description
Update platform architecture documentation.

## Tasks Completed
- ✅ Added utility-service to architecture docs
- ✅ Updated service diagram
- ✅ Documented service patterns

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_UTILITY_SERVICE_ID}"

create_closed_issue \
    "Stage 14.5.3: Add utility-service to dev scripts" \
    "## Description
Integrate utility-service into development scripts.

## Tasks Completed
- ✅ Added to start.sh
- ✅ Added to stop.sh
- ✅ Added to status.sh
- ✅ Added to restart.sh

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_UTILITY_SERVICE_ID}"

create_closed_issue \
    "Stage 14.5.4: Update frontend environment configuration" \
    "## Description
Add utility-service URL to frontend configuration.

## Tasks Completed
- ✅ Added VITE_UTILITY_SERVICE_URL to .env
- ✅ Updated environment types
- ✅ Configured API client

## Completion Date
November 16, 2025

## Historical Note
This issue represents completed work migrated to Forgejo for tracking." \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_FRONTEND_APP_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 14 issues created!"
echo "=========================================="
echo ""
echo "Summary: Created 18 closed issues for Stage 14 (Utility Service & Language Registry)"
echo "Completion date: November 16, 2025"
echo ""
