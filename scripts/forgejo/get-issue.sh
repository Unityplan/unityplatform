#!/bin/bash

# get-issue.sh
# Retrieve a Forgejo issue by number
#
# Usage: ./get-issue.sh <issue_number>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Load environment
ENV_FILE="$SCRIPT_DIR/.env"
if [[ ! -f "$ENV_FILE" ]]; then
    echo "❌ Error: .env file not found!"
    exit 1
fi

source "$ENV_FILE"

API_BASE="${FORGEJO_URL}/api/v1"

# Parse arguments
ISSUE_NUMBER="${1:-}"

if [[ -z "$ISSUE_NUMBER" ]]; then
    echo "Usage: $0 <issue_number>"
    echo ""
    echo "Example:"
    echo "  $0 147"
    exit 1
fi

# Get issue
response=$(curl -s \
    -H "Authorization: token $FORGEJO_TOKEN" \
    "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${ISSUE_NUMBER}")

# Check if issue exists
if echo "$response" | jq -e '.message' > /dev/null 2>&1; then
    echo "❌ Error: $(echo "$response" | jq -r '.message')"
    exit 1
fi

# Pretty print issue
echo "═══════════════════════════════════════════════════════════════"
echo "Issue #$(echo "$response" | jq -r '.number'): $(echo "$response" | jq -r '.title')"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "State: $(echo "$response" | jq -r '.state')"
echo "Created: $(echo "$response" | jq -r '.created_at')"
echo "Updated: $(echo "$response" | jq -r '.updated_at')"
echo ""
echo "Labels:"
echo "$response" | jq -r '.labels[]?.name' | sed 's/^/  - /'
echo ""
echo "Milestone: $(echo "$response" | jq -r '.milestone.title // "None"')"
echo ""
echo "Assignees:"
echo "$response" | jq -r '.assignees[]?.login // "None"' | sed 's/^/  - /'
echo ""
echo "───────────────────────────────────────────────────────────────"
echo "Description:"
echo "───────────────────────────────────────────────────────────────"
echo "$response" | jq -r '.body'
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "URL: $(echo "$response" | jq -r '.html_url')"
echo "═══════════════════════════════════════════════════════════════"
