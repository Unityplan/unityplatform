#!/bin/bash

# update-issue.sh
# Update a Forgejo issue (title, body, state, labels, etc.)
#
# Usage: ./update-issue.sh <issue_number> [--title "New Title"] [--body "New Body"] [--state open|closed] [--labels "label1,label2"]

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
shift || true

NEW_TITLE=""
NEW_BODY=""
NEW_STATE=""
NEW_LABELS=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --title)
            NEW_TITLE="$2"
            shift 2
            ;;
        --body)
            NEW_BODY="$2"
            shift 2
            ;;
        --state)
            NEW_STATE="$2"
            shift 2
            ;;
        --labels)
            NEW_LABELS="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [[ -z "$ISSUE_NUMBER" ]]; then
    echo "Usage: $0 <issue_number> [--title \"New Title\"] [--body \"New Body\"] [--state open|closed] [--labels \"label1,label2\"]"
    echo ""
    echo "Examples:"
    echo "  $0 147 --title \"New title for issue 147\""
    echo "  $0 147 --state closed"
    echo "  $0 147 --labels \"priority:high,status:in-progress\""
    echo "  $0 147 --body \"Updated description\" --state closed"
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

# Build label IDs array
label_ids=()
if [[ -n "$NEW_LABELS" ]]; then
    IFS=',' read -ra LABEL_NAMES <<< "$NEW_LABELS"
    for label_name in "${LABEL_NAMES[@]}"; do
        label_id=$(get_label_id "$label_name")
        if [[ -n "$label_id" ]]; then
            label_ids+=("$label_id")
        else
            echo "⚠️  Label not found: $label_name"
        fi
    done
fi

# Build JSON payload for general updates
payload="{}"
has_general_updates=false

if [[ -n "$NEW_TITLE" ]]; then
    payload=$(echo "$payload" | jq --arg title "$NEW_TITLE" '. + {title: $title}')
    has_general_updates=true
fi

if [[ -n "$NEW_BODY" ]]; then
    payload=$(echo "$payload" | jq --arg body "$NEW_BODY" '. + {body: $body}')
    has_general_updates=true
fi

if [[ -n "$NEW_STATE" ]]; then
    payload=$(echo "$payload" | jq --arg state "$NEW_STATE" '. + {state: $state}')
    has_general_updates=true
fi

# 1. Perform General Updates (Title, Body, State) via PATCH
if [[ "$has_general_updates" == "true" ]]; then
    response=$(curl -s \
        -X PATCH \
        -H "Authorization: token $FORGEJO_TOKEN" \
        -H "Content-Type: application/json" \
        -d "$payload" \
        "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${ISSUE_NUMBER}")

    # Check for errors
    if echo "$response" | jq -e '.message' > /dev/null 2>&1; then
        echo "❌ Error updating issue details: $(echo "$response" | jq -r '.message')"
        exit 1
    fi
fi

# 2. Perform Label Updates via PUT (Replaces all labels)
if [[ ${#label_ids[@]} -gt 0 ]]; then
    labels_json=$(printf '%s\n' "${label_ids[@]}" | jq -R 'tonumber' | jq -s .)
    labels_payload=$(echo "{}" | jq --argjson labels "$labels_json" '. + {labels: $labels}')
    
    response=$(curl -s \
        -X PUT \
        -H "Authorization: token $FORGEJO_TOKEN" \
        -H "Content-Type: application/json" \
        -d "$labels_payload" \
        "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${ISSUE_NUMBER}/labels")

    # Check for errors
    if echo "$response" | jq -e '.message' > /dev/null 2>&1; then
        echo "❌ Error updating labels: $(echo "$response" | jq -r '.message')"
        exit 1
    fi
fi

# Fetch final state for display
final_response=$(curl -s \
    -H "Authorization: token $FORGEJO_TOKEN" \
    "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${ISSUE_NUMBER}")

echo "✅ Issue #${ISSUE_NUMBER} updated successfully"
echo ""
echo "Title: $(echo "$final_response" | jq -r '.title')"
echo "State: $(echo "$final_response" | jq -r '.state')"
echo "Labels: $(echo "$final_response" | jq -r '[.labels[].name] | join(", ")')"
echo "URL: $(echo "$final_response" | jq -r '.html_url')"
