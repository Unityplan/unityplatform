#!/bin/bash

# convert-status-to-issues.sh
# Converts markdown status items from phase-1-status.md to Forgejo issues
# 
# Usage:
#   ./convert-status-to-issues.sh [--dry-run] [--stage STAGE_NUM]
#
# Options:
#   --dry-run        Preview issues without creating them
#   --stage N        Only convert tasks from Stage N (e.g., --stage 5)
#   --limit N        Limit number of issues to create (default: all)
#   --help           Show this help message

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Load environment variables
ENV_FILE="$SCRIPT_DIR/.env"
if [[ ! -f "$ENV_FILE" ]]; then
    echo "❌ Error: .env file not found!"
    echo "💡 Copy .env.example to .env and configure your Forgejo API token:"
    echo "   cp $SCRIPT_DIR/.env.example $SCRIPT_DIR/.env"
    echo "   # Then edit .env with your settings"
    exit 1
fi

# shellcheck source=/dev/null
source "$ENV_FILE"

# Validate required variables
if [[ -z "${FORGEJO_URL:-}" ]] || [[ -z "${FORGEJO_TOKEN:-}" ]] || [[ -z "${FORGEJO_OWNER:-}" ]] || [[ -z "${FORGEJO_REPO:-}" ]]; then
    echo "❌ Error: Missing required environment variables!"
    echo "Please configure .env with: FORGEJO_URL, FORGEJO_TOKEN, FORGEJO_OWNER, FORGEJO_REPO"
    exit 1
fi

# Parse command line arguments
DRY_RUN=false
STAGE_FILTER=""
LIMIT=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --stage)
            STAGE_FILTER="$2"
            shift 2
            ;;
        --limit)
            LIMIT="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [--dry-run] [--stage N] [--limit N]"
            echo ""
            echo "Options:"
            echo "  --dry-run        Preview issues without creating them"
            echo "  --stage N        Only convert tasks from Stage N"
            echo "  --limit N        Limit number of issues to create"
            echo "  --help           Show this help message"
            exit 0
            ;;
        *)
            echo "❌ Unknown option: $1"
            echo "Run with --help for usage information"
            exit 1
            ;;
    esac
done

# Status file
STATUS_FILE="$WORKSPACE_ROOT/docs/status/current/phase-1-status.md"

if [[ ! -f "$STATUS_FILE" ]]; then
    echo "❌ Error: Status file not found: $STATUS_FILE"
    exit 1
fi

# API base URL
API_BASE="${FORGEJO_URL}/api/v1"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to make Forgejo API calls
forgejo_api() {
    local method="$1"
    local endpoint="$2"
    local data="${3:-}"
    
    local curl_args=(
        -s
        -X "$method"
        -H "Authorization: token $FORGEJO_TOKEN"
        -H "Content-Type: application/json"
    )
    
    if [[ -n "$data" ]]; then
        curl_args+=(-d "$data")
    fi
    
    curl "${curl_args[@]}" "${API_BASE}${endpoint}"
}

# Function to get label ID by name
get_label_id() {
    local label_name="$1"
    local response
    
    response=$(forgejo_api GET "/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/labels")
    echo "$response" | jq -r ".[] | select(.name == \"$label_name\") | .id"
}

# Function to get milestone ID by title
get_milestone_id() {
    local milestone_title="$1"
    local response
    
    response=$(forgejo_api GET "/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/milestones")
    echo "$response" | jq -r ".[] | select(.title == \"$milestone_title\") | .id"
}

# Function to create an issue
create_issue() {
    local title="$1"
    local body="$2"
    local labels="$3"  # Comma-separated label IDs
    local milestone="${4:-}"
    
    # Build JSON payload
    local payload
    payload=$(jq -n \
        --arg title "$title" \
        --arg body "$body" \
        --argjson labels "[$labels]" \
        '{title: $title, body: $body, labels: $labels}')
    
    # Add milestone if provided
    if [[ -n "$milestone" ]]; then
        payload=$(echo "$payload" | jq --argjson milestone "$milestone" '. + {milestone: $milestone}')
    fi
    
    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${YELLOW}[DRY RUN]${NC} Would create issue:"
        echo "  Title: $title"
        echo "  Labels: $labels"
        echo "  Milestone: ${milestone:-none}"
        echo "  Body: ${body:0:100}..."
        echo ""
        return 0
    fi
    
    local response
    response=$(forgejo_api POST "/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues" "$payload")
    
    local issue_number
    issue_number=$(echo "$response" | jq -r '.number // empty')
    
    if [[ -n "$issue_number" ]]; then
        echo -e "${GREEN}✓${NC} Created issue #$issue_number: $title"
        return 0
    else
        echo -e "${RED}✗${NC} Failed to create issue: $title"
        echo "Response: $response"
        return 1
    fi
}

# Function to parse markdown and extract tasks
parse_tasks() {
    local stage_num="$1"
    local section_title="$2"
    local priority="$3"
    local area="$4"
    
    # Extract tasks from the status file
    # Look for lines starting with "- ❌" or "- ⏸️" (incomplete tasks)
    local in_stage=false
    local in_section=false
    local task_count=0
    
    while IFS= read -r line; do
        # Check if we're entering the target stage
        if [[ "$line" =~ ^###[[:space:]]*Stage[[:space:]]*${stage_num}: ]]; then
            in_stage=true
            continue
        fi
        
        # Check if we've left the stage
        if [[ "$in_stage" == true ]] && [[ "$line" =~ ^###[[:space:]]*Stage[[:space:]]*[0-9]+: ]]; then
            in_stage=false
            break
        fi
        
        # Check if we're in the right section
        if [[ "$in_stage" == true ]] && [[ "$line" =~ ^####[[:space:]]*${section_title} ]]; then
            in_section=true
            continue
        fi
        
        # Check if we've left the section
        if [[ "$in_section" == true ]] && [[ "$line" =~ ^#### ]]; then
            in_section=false
            continue
        fi
        
        # Extract incomplete tasks
        if [[ "$in_section" == true ]] && [[ "$line" =~ ^-[[:space:]]*(❌|⏸️)[[:space:]]*(.+)$ ]]; then
            local task="${BASH_REMATCH[2]}"
            
            # Skip if limit reached
            if [[ -n "$LIMIT" ]] && [[ $task_count -ge $LIMIT ]]; then
                break
            fi
            
            # Output task info
            echo "$stage_num|$section_title|$priority|$area|$task"
            ((task_count++))
        fi
    done < "$STATUS_FILE"
}

# Main conversion logic
main() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║  Unity Platform - Markdown to Forgejo Issue Converter     ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${YELLOW}🔍 DRY RUN MODE - No issues will be created${NC}"
        echo ""
    fi
    
    # Get milestone ID if configured
    local milestone_id=""
    if [[ -n "${DEFAULT_MILESTONE:-}" ]]; then
        milestone_id=$(get_milestone_id "$DEFAULT_MILESTONE")
        if [[ -n "$milestone_id" ]]; then
            echo -e "${GREEN}✓${NC} Found milestone: $DEFAULT_MILESTONE (ID: $milestone_id)"
        else
            echo -e "${YELLOW}⚠${NC} Milestone not found: $DEFAULT_MILESTONE (issues will have no milestone)"
        fi
    fi
    
    echo ""
    echo "📋 Converting tasks from: $STATUS_FILE"
    echo ""
    
    # Define stages to convert (only incomplete ones)
    # Stage 5: Frontend Auth & Profile (0% complete)
    if [[ -z "$STAGE_FILTER" ]] || [[ "$STAGE_FILTER" == "5" ]]; then
        echo -e "${BLUE}━━━ Stage 5: Frontend Auth & Profile ━━━${NC}"
        
        # Parse tasks from Stage 5
        # Example: Parse Step 5.1 tasks
        while IFS='|' read -r stage section priority area task; do
            # Map to labels
            local label_names=("priority:$priority" "type:feature" "area:$area")
            local label_ids=()
            
            for label_name in "${label_names[@]}"; do
                label_id=$(get_label_id "$label_name")
                if [[ -n "$label_id" ]]; then
                    label_ids+=("$label_id")
                fi
            done
            
            # Join label IDs with commas
            local labels_csv=$(IFS=,; echo "${label_ids[*]}")
            
            # Create issue
            create_issue \
                "Stage $stage: $task" \
                "**Section:** $section\n\n**Task:** $task\n\n**Source:** \`docs/status/current/phase-1-status.md\`" \
                "$labels_csv" \
                "$milestone_id"
            
        done < <(parse_tasks "5" "Step 5.1" "high" "frontend")
    fi
    
    # Stage 7: Course Service (0% complete)
    if [[ -z "$STAGE_FILTER" ]] || [[ "$STAGE_FILTER" == "7" ]]; then
        echo ""
        echo -e "${BLUE}━━━ Stage 7: Course Service (LMS) ━━━${NC}"
        # Add task parsing for Stage 7 here
    fi
    
    # Stage 8: Matrix Protocol Integration (0% complete)
    if [[ -z "$STAGE_FILTER" ]] || [[ "$STAGE_FILTER" == "8" ]]; then
        echo ""
        echo -e "${BLUE}━━━ Stage 8: Matrix Protocol Integration ━━━${NC}"
        # Add task parsing for Stage 8 here
    fi
    
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${YELLOW}🔍 Dry run complete - no issues were created${NC}"
        echo -e "${YELLOW}💡 Run without --dry-run to actually create issues${NC}"
    else
        echo -e "${GREEN}✅ Conversion complete!${NC}"
        echo -e "${GREEN}🔗 View issues: ${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues${NC}"
    fi
}

# Run main function
main
