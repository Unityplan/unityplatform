#!/bin/bash

# create-from-roadmap.sh - Interactive issue creator from roadmap items
#
# This script helps you create issues by prompting for details
# instead of typing long command-line arguments.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Load environment
ENV_FILE="$SCRIPT_DIR/.env"
if [[ ! -f "$ENV_FILE" ]]; then
    echo "❌ Error: .env file not found!"
    echo "Run: cp .env.example .env"
    exit 1
fi

source "$ENV_FILE"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Interactive Issue Creator                                 ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Stage selection
echo "What stage is this for?"
echo "  5 - Frontend Auth & Profile"
echo "  7 - Course Service (LMS)"
echo "  8 - Matrix Protocol Integration"
echo "  9 - IPFS Service"
echo "  10 - Forum Service"
echo "  11 - Translation Service"
echo "  12 - Frontend Course & Forum UI"
echo "  13 - Testing & Documentation"
echo ""
read -rp "Stage number: " STAGE

# Title
echo ""
read -rp "Issue title: " TITLE
FULL_TITLE="Stage ${STAGE}: ${TITLE}"

# Description
echo ""
echo "Issue description (press Enter for default):"
read -rp "> " DESCRIPTION

if [[ -z "$DESCRIPTION" ]]; then
    DESCRIPTION="**Stage ${STAGE}**\n\n${TITLE}"
fi

# Priority
echo ""
echo "Priority:"
echo "  1 - critical"
echo "  2 - high"
echo "  3 - medium"
echo "  4 - low"
read -rp "Priority (default: medium): " PRIORITY_NUM

case "${PRIORITY_NUM:-3}" in
    1) PRIORITY="critical" ;;
    2) PRIORITY="high" ;;
    3) PRIORITY="medium" ;;
    4) PRIORITY="low" ;;
    *) PRIORITY="medium" ;;
esac

# Type
echo ""
echo "Type:"
echo "  1 - feature"
echo "  2 - bug"
echo "  3 - enhancement"
echo "  4 - infrastructure"
read -rp "Type (default: feature): " TYPE_NUM

case "${TYPE_NUM:-1}" in
    1) TYPE="feature" ;;
    2) TYPE="bug" ;;
    3) TYPE="enhancement" ;;
    4) TYPE="infrastructure" ;;
    *) TYPE="feature" ;;
esac

# Area (based on stage)
case "$STAGE" in
    5|12) AREA="frontend-app" ;;
    7) AREA="auth-service" ;;  # Course service - update when created
    10|11) AREA="backend" ;;  # Forum/Translation - update when created
    8|9) AREA="infrastructure" ;;
    13) AREA="infrastructure" ;;
    *) AREA="infrastructure" ;;
esac

# Build labels with slash format (priority/high not priority:high)
LABELS="priority/${PRIORITY},type/${TYPE},area/${AREA}"

# Milestone
MILESTONE="${DEFAULT_MILESTONE:-}"

# Confirmation
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Ready to create:"
echo "  Title: $FULL_TITLE"
echo "  Labels: $LABELS"
echo "  Milestone: ${MILESTONE:-none}"
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
read -rp "Create this issue? (y/N): " CONFIRM

if [[ "${CONFIRM,,}" != "y" ]]; then
    echo "❌ Cancelled"
    exit 0
fi

# Create issue using the create-issue.sh script
"$SCRIPT_DIR/create-issue.sh" "$FULL_TITLE" "$DESCRIPTION" "$LABELS" "$MILESTONE"
