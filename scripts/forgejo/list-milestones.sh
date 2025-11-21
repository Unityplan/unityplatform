#!/bin/bash

# list-milestones.sh
# Lists all milestones in the Forgejo repository
#
# Usage: ./list-milestones.sh [--json]

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
SHOW_JSON=false

# Parse args
if [[ "${1:-}" == "--json" ]]; then
    SHOW_JSON=true
fi

# Fetch milestones
response=$(curl -s \
    -H "Authorization: token $FORGEJO_TOKEN" \
    "${API_BASE}/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/milestones?state=all")

if [[ "$SHOW_JSON" == true ]]; then
    echo "$response" | jq '.'
else
    echo "🎯 Milestones in ${FORGEJO_OWNER}/${FORGEJO_REPO}:"
    echo ""
    echo "$response" | jq -r '.[] | "  \(.id | tostring | "\u001b[33m" + . + "\u001b[0m") - \(.title) [\(.state)] (\(.open_issues)/\(.closed_issues) issues)"'
    echo ""
    echo "Total: $(echo "$response" | jq 'length')"
fi
