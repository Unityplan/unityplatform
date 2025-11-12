#!/bin/bash

# Check status of all development services

#!/bin/bash

# Display current development environment status

echo "📊 Unity Platform Development Environment Status"
echo "============================================"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Function to check if port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo -e "${GREEN}✅ Running${NC}"
        return 0
    else
        echo -e "${RED}❌ Stopped${NC}"
        return 1
    fi
}

# Check services
echo "Infrastructure (Docker):"
echo -n "  PostgreSQL (5432):     "
check_port 5432

echo -n "  NATS (4222):           "
check_port 4222

echo -n "  Redis (6379):          "
check_port 6379

echo ""
echo "Backend Services (Rust):"
echo -n "  auth-service (8001):   "
check_port 8001

echo -n "  user-service (8002):   "
check_port 8002

echo ""
echo "Frontend:"
echo -n "  Vite dev server (5173):"
check_port 5173

echo ""
echo "Development Tools:"
echo -n "  Adminer (8080):        "
check_port 8080

echo ""
