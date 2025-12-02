#!/bin/bash

# Start all development services for Unity Platform
# This script starts infrastructure, backend services, and frontend
# Usage: ./start-dev-services.sh [--build] [--release]
#   --build    Rebuild Rust services before starting (uses debug mode for faster builds)
#   --release  Use release mode build (slower build, faster runtime)

set -e

# Parse arguments
BUILD_SERVICES=false
RELEASE_MODE=false
for arg in "$@"; do
    case $arg in
        --build)
            BUILD_SERVICES=true
            ;;
        --release)
            RELEASE_MODE=true
            ;;
    esac
done

echo "🚀 Starting Unity Platform Development Environment"
echo "=============================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get workspace root (scripts/dev/../.. = workspace root)
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$WORKSPACE_ROOT"

# Function to check if port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 0  # Port is in use
    else
        return 1  # Port is free
    fi
}

# Function to wait for service to be ready
wait_for_service() {
    local service_name=$1
    local port=$2
    local max_wait=30
    local waited=0
    
    echo -n "  Waiting for $service_name on port $port..."
    while ! check_port $port; do
        if [ $waited -ge $max_wait ]; then
            echo " ❌ TIMEOUT"
            return 1
        fi
        sleep 1
        waited=$((waited + 1))
        echo -n "."
    done
    echo " ✅"
    return 0
}

# 1. Start Infrastructure (Docker)
echo -e "${BLUE}📦 Step 1: Starting Infrastructure (Docker)${NC}"
echo "Starting Denmark pod (PostgreSQL, NATS, Redis, IPFS, Matrix)..."
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env up -d

echo "Waiting for infrastructure to be ready..."
# Use docker inspect to check health instead of port checking
echo "  Checking PostgreSQL health..."
for i in {1..30}; do
    if docker inspect service-postgres-dk --format='{{.State.Health.Status}}' 2>/dev/null | grep -q "healthy"; then
        echo -e "  ✅ PostgreSQL ready"
        break
    fi
    if [ $i -eq 30 ]; then
        echo -e "  ${YELLOW}⚠ PostgreSQL taking longer, but continuing...${NC}"
    fi
    sleep 1
done

if check_port 4222; then
    echo -e "  ✅ NATS ready on port 4222"
fi

if check_port 6379; then
    echo -e "  ✅ Redis ready on port 6379"
fi

echo ""

# 2. Start Backend Services
echo -e "${BLUE}🦀 Step 2: Starting Rust Backend Services${NC}"

# Determine build profile and binary path
if [ "$RELEASE_MODE" = true ]; then
    BUILD_PROFILE="release"
    BINARY_DIR="release"
    BUILD_FLAGS="--release"
else
    BUILD_PROFILE="debug"
    BINARY_DIR="debug"
    BUILD_FLAGS=""
fi

# Build services if --build flag provided
if [ "$BUILD_SERVICES" = true ]; then
    echo -e "${YELLOW}🔨 Building Rust services in ${BUILD_PROFILE} mode...${NC}"
    cd "$WORKSPACE_ROOT/services"
    
    # Use cargo build with optional release flag
    # For debug builds, we can also use incremental compilation (default)
    if [ "$RELEASE_MODE" = true ]; then
        cargo build --release
    else
        # Debug build is much faster (incremental by default)
        cargo build
    fi
    
    echo -e "${GREEN}✅ Build complete (${BUILD_PROFILE} mode)${NC}"
    cd "$WORKSPACE_ROOT"
    echo ""
fi

# Check if services are already running
if check_port 8001; then
    echo -e "  ${YELLOW}⚠ auth-service already running on port 8001${NC}"
else
    echo "Starting auth-service on port 8001..."
    cd "$WORKSPACE_ROOT/services/auth-service"
    set -a
    source .env
    set +a
    cd "$WORKSPACE_ROOT"
    ./services/target/${BINARY_DIR}/auth-service > "$WORKSPACE_ROOT/logs/auth-service.log" 2>&1 &
    
    wait_for_service "auth-service" 8001 || exit 1
fi

if check_port 8084; then
    echo -e "  ${YELLOW}⚠ user-service on old port 8084, stopping...${NC}"
    pkill -f "user-service" || true
    sleep 2
fi

if check_port 8081; then
    echo -e "  ${YELLOW}⚠ user-service on old port 8081, stopping...${NC}"
    pkill -f "user-service" || true
    sleep 2
fi

if check_port 8002; then
    echo -e "  ${YELLOW}⚠ user-service already running on port 8002${NC}"
else
    echo "Starting user-service on port 8002..."
    cd "$WORKSPACE_ROOT/services/user-service"
    set -a
    source .env
    set +a
    cd "$WORKSPACE_ROOT"
    ./services/target/${BINARY_DIR}/user-service > "$WORKSPACE_ROOT/logs/user-service.log" 2>&1 &
    
    wait_for_service "user-service" 8002 || exit 1
fi

if check_port 8004; then
    echo -e "  ${YELLOW}⚠ invitation-service already running on port 8004${NC}"
else
    echo "Starting invitation-service on port 8004..."
    cd "$WORKSPACE_ROOT/services/invitation-service"
    set -a
    source .env
    set +a
    cd "$WORKSPACE_ROOT"
    ./services/target/${BINARY_DIR}/invitation-service > "$WORKSPACE_ROOT/logs/invitation-service.log" 2>&1 &
    
    wait_for_service "invitation-service" 8004 || exit 1
fi

if check_port 8006; then
    echo -e "  ${YELLOW}⚠ community-service already running on port 8006${NC}"
else
    echo "Starting community-service on port 8006..."
    cd "$WORKSPACE_ROOT/services/community-service"
    set -a
    source .env
    set +a
    cd "$WORKSPACE_ROOT"
    ./services/target/${BINARY_DIR}/community-service > "$WORKSPACE_ROOT/logs/community-service.log" 2>&1 &
    
    wait_for_service "community-service" 8006 || exit 1
fi

if check_port 8007; then
    echo -e "  ${YELLOW}⚠ badge-service already running on port 8007${NC}"
else
    echo "Starting badge-service on port 8007..."
    cd "$WORKSPACE_ROOT/services/badge-service"
    set -a
    source .env
    set +a
    cd "$WORKSPACE_ROOT"
    ./services/target/${BINARY_DIR}/badge-service > "$WORKSPACE_ROOT/logs/badge-service.log" 2>&1 &
    
    wait_for_service "badge-service" 8007 || exit 1
fi

if check_port 8008; then
    echo -e "  ${YELLOW}⚠ territory-service already running on port 8008${NC}"
else
    echo "Starting territory-service on port 8008..."
    cd "$WORKSPACE_ROOT/services/territory-service"
    set -a
    source .env
    set +a
    cd "$WORKSPACE_ROOT"
    ./services/target/${BINARY_DIR}/territory-service > "$WORKSPACE_ROOT/logs/territory-service.log" 2>&1 &
    
    wait_for_service "territory-service" 8008 || exit 1
fi

if check_port 8014; then
    echo -e "  ${YELLOW}⚠ utility-service already running on port 8014${NC}"
else
    echo "Starting utility-service on port 8014..."
    cd "$WORKSPACE_ROOT/services/utility-service"
    set -a
    source .env
    set +a
    cd "$WORKSPACE_ROOT"
    ./services/target/${BINARY_DIR}/utility-service > "$WORKSPACE_ROOT/logs/utility-service.log" 2>&1 &
    
    wait_for_service "utility-service" 8014 || exit 1
fi

if check_port 8015; then
    echo -e "  ${YELLOW}⚠ task-scheduler-service already running on port 8015${NC}"
else
    echo "Starting task-scheduler-service on port 8015..."
    cd "$WORKSPACE_ROOT/services/task-scheduler-service"
    set -a
    source .env
    set +a
    cd "$WORKSPACE_ROOT"
    ./services/target/${BINARY_DIR}/task-scheduler-service > "$WORKSPACE_ROOT/logs/task-scheduler-service.log" 2>&1 &
    
    wait_for_service "task-scheduler-service" 8015 || exit 1
fi

echo ""

# 3. Start Frontend
echo -e "${BLUE}⚛️  Step 3: Starting Frontend (Vite)${NC}"

if check_port 5173; then
    echo -e "  ${YELLOW}⚠ Vite dev server already running on port 5173${NC}"
else
    echo "Starting Vite dev server on port 5173..."
    cd "$WORKSPACE_ROOT/app"
    npm run dev > "$WORKSPACE_ROOT/logs/frontend.log" 2>&1 &
    
    wait_for_service "Vite" 5173 || exit 1
fi

echo ""
echo -e "${GREEN}✅ All services started successfully!${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}🎉 Unity Platform Development Environment Ready${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📱 Frontend:          http://localhost:5173"
echo "🔐 Auth Service:      http://localhost:8001"
echo "👤 User Service:      http://localhost:8002"
echo "💌 Invitation Service: http://localhost:8004"
echo "🏘️  Community Service:  http://localhost:8006"
echo "🏆 Badge Service:     http://localhost:8007"
echo "🌍 Territory Service: http://localhost:8008"
echo "🛠️  Utility Service:   http://localhost:8014"
echo "⏰ Task Scheduler:    http://localhost:8015"
echo "🗄️  PostgreSQL:        localhost:5432"
echo "📨 NATS:              localhost:4222"
echo "🗃️  Redis:             localhost:6379"
echo ""
echo "📊 Monitoring:"
echo "   Adminer (DB):      http://localhost:8080"
echo "   Dev Dashboard:     http://localhost:8888"
echo ""
echo "📝 Logs:"
echo "   Auth Service:      tail -f logs/auth-service.log"
echo "   User Service:      tail -f logs/user-service.log"
echo "   Invitation Service: tail -f logs/invitation-service.log"
echo "   Community Service: tail -f logs/community-service.log"
echo "   Badge Service:     tail -f logs/badge-service.log"
echo "   Territory Service: tail -f logs/territory-service.log"
echo "   Utility Service:   tail -f logs/utility-service.log"
echo "   Frontend:          tail -f logs/frontend.log"
echo ""
echo "🛑 To stop all services: ./scripts/stop-dev-services.sh"
echo ""
