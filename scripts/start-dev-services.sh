#!/bin/bash

# Start all development services for UnityPlan Platform
# This script starts infrastructure, backend services, and frontend

set -e

echo "🚀 Starting UnityPlan Development Environment"
echo "=============================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get workspace root
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
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

# Check if services are already running
if check_port 8001; then
    echo -e "  ${YELLOW}⚠ auth-service already running on port 8001${NC}"
else
    echo "Starting auth-service on port 8001..."
    cd "$WORKSPACE_ROOT"
    RUST_LOG=info,auth_service=debug \
    DATABASE_URL="postgresql://unityplan:unityplan_dev_password_dk@localhost:5432/unityplan_dk" \
    SERVER_PORT=8001 \
    CORS_ALLOWED_ORIGINS="http://localhost:5173,http://localhost:3000" \
    ./services/target/release/auth-service > "$WORKSPACE_ROOT/logs/auth-service.log" 2>&1 &
    
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
    cd "$WORKSPACE_ROOT"
    RUST_LOG=info,user_service=debug \
    DATABASE_URL="postgresql://unityplan:unityplan_dev_password_dk@localhost:5432/unityplan_dk" \
    PORT=8002 \
    CORS_ALLOWED_ORIGINS="http://localhost:5173,http://localhost:3000" \
    ./services/target/release/user-service > "$WORKSPACE_ROOT/logs/user-service.log" 2>&1 &
    
    wait_for_service "user-service" 8002 || exit 1
fi

echo ""

# 3. Start Frontend
echo -e "${BLUE}⚛️  Step 3: Starting Frontend (Vite)${NC}"

if check_port 5173; then
    echo -e "  ${YELLOW}⚠ Vite dev server already running on port 5173${NC}"
else
    echo "Starting Vite dev server on port 5173..."
    cd "$WORKSPACE_ROOT/frontend"
    npm run dev > "$WORKSPACE_ROOT/logs/frontend.log" 2>&1 &
    
    wait_for_service "Vite" 5173 || exit 1
fi

echo ""
echo -e "${GREEN}✅ All services started successfully!${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}🎉 UnityPlan Development Environment Ready${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📱 Frontend:          http://localhost:5173"
echo "🔐 Auth Service:      http://localhost:8001"
echo "👤 User Service:      http://localhost:8002"
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
echo "   Frontend:          tail -f logs/frontend.log"
echo ""
echo "🛑 To stop all services: ./scripts/stop-dev-services.sh"
echo ""
