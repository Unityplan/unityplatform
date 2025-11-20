#!/bin/bash

# Run tests for Unity Platform services
# Usage: ./scripts/dev/run-tests.sh [service-name]
# Example: ./scripts/dev/run-tests.sh auth-service
#          ./scripts/dev/run-tests.sh (runs all)

set -e

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$WORKSPACE_ROOT"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

run_service_tests() {
    local service=$1
    local manifest_path="services/$service/Cargo.toml"

    if [ ! -f "$manifest_path" ]; then
        # Try adding -service suffix if missing
        if [ -f "services/$service-service/Cargo.toml" ]; then
            service="$service-service"
            manifest_path="services/$service/Cargo.toml"
        else
            echo -e "${RED}Error: Service '$service' not found at $manifest_path${NC}"
            return 1
        fi
    fi

    echo -e "${GREEN}Testing $service...${NC}"
    echo "Manifest: $manifest_path"
    
    # Run tests
    # --test integration_test runs the integration tests specifically
    # We also run unit tests (default cargo test)
    
    if cargo test --manifest-path "$manifest_path"; then
        echo -e "${GREEN}✅ $service tests passed${NC}"
        return 0
    else
        echo -e "${RED}❌ $service tests failed${NC}"
        return 1
    fi
}

if [ -z "$1" ]; then
    echo "Running tests for ALL services..."
    failed=0
    
    # Find all Cargo.toml files in services/ directory (depth 2)
    services=$(find services -maxdepth 2 -name "Cargo.toml" -not -path "*/shared-lib/*" | xargs dirname | xargs basename)
    
    # Always test shared-lib first
    run_service_tests "shared-lib" || failed=1
    
    for svc in $services; do
        if [ "$svc" != "shared-lib" ]; then
            run_service_tests "$svc" || failed=1
        fi
    done
    
    if [ $failed -eq 0 ]; then
        echo -e "${GREEN}All services passed!${NC}"
        exit 0
    else
        echo -e "${RED}Some services failed tests.${NC}"
        exit 1
    fi
else
    run_service_tests "$1"
fi
