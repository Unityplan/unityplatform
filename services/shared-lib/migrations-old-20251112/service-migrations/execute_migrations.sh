#!/bin/bash
# ============================================================================
# Migration Execution Script - Service-Specific Migrations
# ============================================================================
# 
# Purpose: Execute database migrations for UnityPlan platform in correct order
# Usage: ./execute_migrations.sh <territory_code> [options]
#
# Examples:
#   ./execute_migrations.sh dk              # Denmark territory
#   ./execute_migrations.sh no --dry-run    # Norway territory (dry run)
#   ./execute_migrations.sh dk --level 2    # Run up to Level 2 only
#
# ============================================================================

set -e  # Exit immediately if a command exits with a non-zero status
set -u  # Treat unset variables as an error
set -o pipefail  # Catch errors in pipes

# ============================================================================
# Configuration
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TERRITORY_CODE="${1:-}"
DB_NAME="${DB_NAME:-unityplan}"
DB_USER="${DB_USER:-postgres}"
DB_HOST="${DB_HOST:-localhost}"
DB_PORT="${DB_PORT:-5432}"

DRY_RUN=false
MAX_LEVEL=99
VERBOSE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# Functions
# ============================================================================

print_header() {
    echo -e "${BLUE}============================================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}============================================================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

usage() {
    cat << EOF
Usage: $0 <territory_code> [options]

Arguments:
    territory_code    Territory code (e.g., 'dk', 'no', 'se', 'eu')

Options:
    --dry-run        Show what would be executed without running migrations
    --level <N>      Run migrations up to level N (0-4)
    --verbose        Show detailed output
    --db-name <name> Database name (default: unityplan)
    --db-user <user> Database user (default: postgres)
    --db-host <host> Database host (default: localhost)
    --db-port <port> Database port (default: 5432)
    --help           Show this help message

Migration Levels:
    Level 0: Foundation (territory-service)
    Level 1: Core Identity (auth-service)
    Level 2: User Data (user-service, settings-service, invitation-service, notification-service)
    Level 3: Communities (community-service)
    Level 4: Features (badge-service, event-service, course-service, forum-service)

Examples:
    $0 dk                    # Run all migrations for Denmark
    $0 no --dry-run          # Show what would run for Norway
    $0 dk --level 2          # Run only up to Level 2
    $0 dk --verbose          # Run with detailed output

EOF
    exit 1
}

execute_sql() {
    local sql_file="$1"
    local description="$2"
    local psql_vars="${3:-}"
    
    if [ ! -f "$sql_file" ]; then
        print_error "Migration file not found: $sql_file"
        return 1
    fi
    
    if [ "$DRY_RUN" = true ]; then
        print_info "Would execute: $description"
        print_info "  File: $sql_file"
        [ -n "$psql_vars" ] && print_info "  Variables: $psql_vars"
        return 0
    fi
    
    if [ "$VERBOSE" = true ]; then
        print_info "Executing: $description"
        print_info "  File: $sql_file"
    fi
    
    # Build psql command
    local psql_cmd="psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME"
    
    # Add variables if provided
    if [ -n "$psql_vars" ]; then
        psql_cmd="$psql_cmd $psql_vars"
    fi
    
    # Execute migration
    if $psql_cmd -f "$sql_file" > /dev/null 2>&1; then
        print_success "$description"
        return 0
    else
        print_error "Failed: $description"
        print_error "  Check the SQL file for errors: $sql_file"
        return 1
    fi
}

check_database_connection() {
    print_info "Checking database connection..."
    
    if psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "SELECT 1" > /dev/null 2>&1; then
        print_success "Database connection successful"
        return 0
    else
        print_error "Cannot connect to database"
        print_error "  Host: $DB_HOST:$DB_PORT"
        print_error "  Database: $DB_NAME"
        print_error "  User: $DB_USER"
        exit 1
    fi
}

# ============================================================================
# Parse Arguments
# ============================================================================

if [ $# -eq 0 ]; then
    print_error "Missing required argument: territory_code"
    usage
fi

# Shift past territory code
shift

while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --level)
            MAX_LEVEL="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --db-name)
            DB_NAME="$2"
            shift 2
            ;;
        --db-user)
            DB_USER="$2"
            shift 2
            ;;
        --db-host)
            DB_HOST="$2"
            shift 2
            ;;
        --db-port)
            DB_PORT="$2"
            shift 2
            ;;
        --help)
            usage
            ;;
        *)
            print_error "Unknown option: $1"
            usage
            ;;
    esac
done

# Validate territory code
if [ -z "$TERRITORY_CODE" ]; then
    print_error "Territory code cannot be empty"
    usage
fi

# ============================================================================
# Main Execution
# ============================================================================

print_header "unityplatform Database Migration Execution"

echo ""
echo "Configuration:"
echo "  Territory Code:  $TERRITORY_CODE"
echo "  Database:        $DB_NAME"
echo "  Host:            $DB_HOST:$DB_PORT"
echo "  User:            $DB_USER"
echo "  Max Level:       $MAX_LEVEL"
echo "  Dry Run:         $DRY_RUN"
echo "  Verbose:         $VERBOSE"
echo ""

# Check database connection
if [ "$DRY_RUN" = false ]; then
    check_database_connection
fi

# ============================================================================
# Level 0: Foundation
# ============================================================================

if [ "$MAX_LEVEL" -ge 0 ]; then
    print_header "Level 0: Foundation (territory-service)"
    
    execute_sql \
        "$SCRIPT_DIR/00-territory-service/001_create_territories.sql" \
        "Create global.territories table" \
        ""
    
    echo ""
fi

# ============================================================================
# Level 1: Core Identity
# ============================================================================

if [ "$MAX_LEVEL" -ge 1 ]; then
    print_header "Level 1: Core Identity (auth-service)"
    
    execute_sql \
        "$SCRIPT_DIR/01-auth-service/001_create_global_registries.sql" \
        "Create global username/email registries" \
        ""
    
    execute_sql \
        "$SCRIPT_DIR/01-auth-service/002_create_users_tables.sql" \
        "Create users and refresh_tokens tables (territory: $TERRITORY_CODE)" \
        "-v territory_code=$TERRITORY_CODE"
    
    echo ""
fi

# ============================================================================
# Level 2: User Data (parallel execution)
# ============================================================================

if [ "$MAX_LEVEL" -ge 2 ]; then
    print_header "Level 2: User Data (parallel services)"
    
    # user-service migrations
    print_info "Running user-service migrations..."
    execute_sql \
        "$SCRIPT_DIR/02-user-service/001_create_profiles.sql" \
        "Create users_profiles table" \
        "-v territory_code=$TERRITORY_CODE"
    
    execute_sql \
        "$SCRIPT_DIR/02-user-service/002_create_profile_links.sql" \
        "Create users_profile_links table" \
        "-v territory_code=$TERRITORY_CODE"
    
    execute_sql \
        "$SCRIPT_DIR/02-user-service/003_create_language_proficiency.sql" \
        "Create users_language_proficiency table" \
        "-v territory_code=$TERRITORY_CODE"
    
    execute_sql \
        "$SCRIPT_DIR/02-user-service/004_create_connections.sql" \
        "Create user_connections table" \
        "-v territory_code=$TERRITORY_CODE"
    
    execute_sql \
        "$SCRIPT_DIR/02-user-service/005_create_gdpr_tables.sql" \
        "Create GDPR tables (data_exports, account_deletion_requests)" \
        "-v territory_code=$TERRITORY_CODE"
    
    execute_sql \
        "$SCRIPT_DIR/02-user-service/006_create_audit_tables.sql" \
        "Create audit tables (activities, audit_log)" \
        "-v territory_code=$TERRITORY_CODE"
    
    # settings-service migrations
    print_info "Running settings-service migrations..."
    execute_sql \
        "$SCRIPT_DIR/02-settings-service/001_create_settings_tables.sql" \
        "Create users_settings and users_notification_settings tables" \
        "-v territory_code=$TERRITORY_CODE"
    
    # invitation-service migrations
    print_info "Running invitation-service migrations..."
    execute_sql \
        "$SCRIPT_DIR/02-invitation-service/001_create_global_registry.sql" \
        "Create global invitation_token_registry table" \
        ""
    
    execute_sql \
        "$SCRIPT_DIR/02-invitation-service/002_create_invitation_tables.sql" \
        "Create invitation_tokens and invitation_uses tables" \
        "-v territory_code=$TERRITORY_CODE"
    
    # notification-service migrations
    print_info "Running notification-service migrations..."
    execute_sql \
        "$SCRIPT_DIR/02-notification-service/001_create_notifications.sql" \
        "Create notifications table" \
        "-v territory_code=$TERRITORY_CODE"
    
    echo ""
fi

# ============================================================================
# Level 3: Communities
# ============================================================================

if [ "$MAX_LEVEL" -ge 3 ]; then
    print_header "Level 3: Communities (community-service)"
    
    print_warning "Level 3 migrations not yet implemented (Phase 2)"
    
    # TODO: Uncomment when migrations are created
    # execute_sql \
    #     "$SCRIPT_DIR/03-community-service/001_create_communities.sql" \
    #     "Create communities table" \
    #     "-v territory_code=$TERRITORY_CODE"
    
    echo ""
fi

# ============================================================================
# Level 4: Features
# ============================================================================

if [ "$MAX_LEVEL" -ge 4 ]; then
    print_header "Level 4: Features (badge-service, event-service, etc.)"
    
    print_warning "Level 4 migrations not yet implemented (Phase 2)"
    
    echo ""
fi

# ============================================================================
# Summary
# ============================================================================

print_header "Migration Execution Complete"

if [ "$DRY_RUN" = true ]; then
    print_warning "DRY RUN MODE - No migrations were actually executed"
    print_info "Remove --dry-run flag to execute migrations"
else
    print_success "All migrations executed successfully!"
    print_info "Database: $DB_NAME"
    print_info "Territory: $TERRITORY_CODE"
fi

echo ""
print_info "Next steps:"
echo "  1. Verify tables created: psql -d $DB_NAME -c '\dt territory_$TERRITORY_CODE.*'"
echo "  2. Check service documentation for API endpoints"
echo "  3. Run service-specific tests"

echo ""
