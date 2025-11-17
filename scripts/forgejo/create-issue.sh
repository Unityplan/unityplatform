#!/bin/bash

# create-issue.sh
# Quick helper to create a single Forgejo issue from command line
#
# Usage: ./create-issue.sh "Issue Title" "Issue Body" [label1,label2,...] [milestone]

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
TITLE="${1:-}"
BODY="${2:-}"
LABELS="${3:-}"
MILESTONE="${4:-}"

if [[ -z "$TITLE" ]]; then
    echo "Usage: $0 \"Issue Title\" \"Issue Body\" [label1,label2,...] [milestone]"
    echo ""
    echo "Example:"
    echo "  $0 \"Implement user login\" \"Add JWT-based authentication\" \"priority:high,type:feature\" \"v0.1.0-alpha.2\""
    exit 1
fi

# Function to get label ID by name
get_label_id() {
    local label_name="$1"
    local response
    
    response=$(curl -s \
        -H "Authorization: token $FORGEJO_TOKEN" \
        "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/labels")
    echo "$response" | jq -r ".[] | select(.name == \"$label_name\") | .id"
}

# Function to get milestone ID by title
get_milestone_id() {
    local milestone_title="$1"
    local response
    
    response=$(curl -s \
        -H "Authorization: token $FORGEJO_TOKEN" \
        "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/milestones")
    echo "$response" | jq -r ".[] | select(.title == \"$milestone_title\") | .id"
}

# Build label IDs array
label_ids=()
if [[ -n "$LABELS" ]]; then
    IFS=',' read -ra LABEL_NAMES <<< "$LABELS"
    for label_name in "${LABEL_NAMES[@]}"; do
        label_id=$(get_label_id "$label_name")
        if [[ -n "$label_id" ]]; then
            label_ids+=("$label_id")
        else
            echo "⚠️  Label not found: $label_name"
        fi
    done
fi

# Get milestone ID
milestone_id=""
if [[ -n "$MILESTONE" ]]; then
    milestone_id=$(get_milestone_id "$MILESTONE")
    if [[ -z "$milestone_id" ]]; then
        echo "⚠️  Milestone not found: $MILESTONE"
    fi
fi

# Build JSON payload
payload=$(jq -n \
    --arg title "$TITLE" \
    --arg body "$BODY" \
    --argjson labels "$(printf '%s\n' "${label_ids[@]}" | jq -R . | jq -s .)" \
    '{title: $title, body: $body, labels: $labels}')

if [[ -n "$milestone_id" ]]; then
    payload=$(echo "$payload" | jq --argjson milestone "$milestone_id" '. + {milestone: $milestone}')
fi

# Create issue
response=$(curl -s \
    -X POST \
    -H "Authorization: token $FORGEJO_TOKEN" \
    -H "Content-Type: application/json" \
    -d "$payload" \
    "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues")

issue_number=$(echo "$response" | jq -r '.number // empty')

if [[ -n "$issue_number" ]]; then
    echo "✅ Created issue #$issue_number: $TITLE"
    echo "🔗 ${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/$issue_number"
else
    echo "❌ Failed to create issue"
    echo "$response" | jq '.'
    exit 1
fi
