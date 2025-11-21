#!/bin/bash

# close-issue.sh
# Close a Forgejo issue with an optional comment
#
# Usage: ./close-issue.sh <issue_number> [comment]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

ISSUE_NUMBER="${1:-}"
COMMENT="${2:-}"

if [[ -z "$ISSUE_NUMBER" ]]; then
    echo "Usage: $0 <issue_number> [comment]"
    echo ""
    echo "Examples:"
    echo "  $0 147"
    echo "  $0 147 \"Completed in commit abc123\""
    exit 1
fi

# Close the issue
"$SCRIPT_DIR/update-issue.sh" "$ISSUE_NUMBER" --state closed

# Add comment if provided
if [[ -n "$COMMENT" ]]; then
    echo ""
    echo "Adding closing comment..."
    
    # Load environment
    ENV_FILE="$SCRIPT_DIR/.env"
    if [[ ! -f "$ENV_FILE" ]]; then
        echo "❌ Error: .env file not found!"
        exit 1
    fi
    
    source "$ENV_FILE"
    API_BASE="${FORGEJO_URL}/api/v1"
    
    # Add comment
    response=$(curl -s \
        -X POST \
        -H "Authorization: token $FORGEJO_TOKEN" \
        -H "Content-Type: application/json" \
        -d "{\"body\": \"$COMMENT\"}" \
        "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${ISSUE_NUMBER}/comments")
    
    if echo "$response" | jq -e '.message' > /dev/null 2>&1; then
        echo "⚠️  Failed to add comment: $(echo "$response" | jq -r '.message')"
    else
        echo "✅ Comment added"
    fi
fi
