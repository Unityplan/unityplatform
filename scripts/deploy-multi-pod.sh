#!/bin/bash

# UnityPlan Multi-Pod Deployment Script
# Purpose: Deploy multi-pod architecture in correct order
# Usage: ./scripts/deploy-multi-pod.sh [--clean]

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Show help
show_help() {
    echo "🚀 UnityPlan Multi-Pod Deployment"
    echo "=================================="
    echo ""
    echo "Usage: ./scripts/deploy-multi-pod.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --clean        Clean start (remove existing containers/volumes)"
    echo "  --help, -h     Show this help message"
    echo ""
    echo "Deploys:"
    echo "  1. Global development stack (Grafana, Prometheus, Jaeger)"
    echo "  2. Global monitoring (central Prometheus federation)"
    echo "  3. Pod Denmark (DK)"
    echo "  4. Pod Norway (NO)"
    echo "  5. Pod Sweden (SE)"
    echo "  6. Pod Europe (EU) - Multi-territory (DE, FR, ES)"
    echo ""
    echo "After deployment, run:"
    echo "  ./scripts/verify-multi-pod.sh"
    echo ""
    exit 0
}

# Configuration
HOST_IP="192.168.60.133"
CLEAN_START=false

# Parse arguments
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
fi

if [ "$1" = "--clean" ]; then
  CLEAN_START=true
fi

echo -e "${BLUE}==================================="
echo "UnityPlan Multi-Pod Deployment"
echo -e "===================================${NC}\n"

# ====================================================================
# Phase 0: Clean Start (Optional)
# ====================================================================
if [ "$CLEAN_START" = true ]; then
  echo -e "${YELLOW}🧹 Cleaning previous deployment...${NC}"
  
  # Stop all pods (using env files to get correct project names)
  docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down 2>/dev/null || true
  docker compose -f docker-compose.pod.yml --env-file pods/norway/.env down 2>/dev/null || true
  docker compose -f docker-compose.pod.yml --env-file pods/sweden/.env down 2>/dev/null || true
  
  # Stop monitoring and dev
  docker compose -f docker-compose.monitoring.yml down 2>/dev/null || true
  docker compose -f docker-compose.dev.yml down 2>/dev/null || true
  
  # Remove mesh network
  docker network rm unityplan-mesh-network 2>/dev/null || true
  
  echo -e "${GREEN}✓ Cleanup complete${NC}\n"
  sleep 2
fi

# ====================================================================
# Phase 1: Network Setup
# ====================================================================
echo -e "${BLUE}🔌 Phase 1: Network Setup${NC}"
echo "-----------------------------------"

if docker network inspect unityplan-mesh-network > /dev/null 2>&1; then
  echo -e "${GREEN}✓${NC} unityplan-mesh-network already exists"
else
  echo "Creating unityplan-mesh-network..."
  docker network create unityplan-mesh-network
  echo -e "${GREEN}✓${NC} unityplan-mesh-network created"
fi

echo ""
sleep 1

# ====================================================================
# Phase 2: Development Stack
# ====================================================================
echo -e "${BLUE}🛠️  Phase 2: Development Stack${NC}"
echo "-----------------------------------"

echo "Starting development tools..."
docker compose -f docker-compose.dev.yml up -d

# Wait for services to stabilize
echo "Waiting for services to stabilize..."
sleep 5

# Verify
if docker compose -f docker-compose.dev.yml ps | grep -q "running"; then
  echo -e "${GREEN}✓${NC} Development stack operational"
  echo "   - Dev Dashboard: http://${HOST_IP}:8888"
  echo "   - Adminer: http://${HOST_IP}:8080"
  echo "   - MailHog: http://${HOST_IP}:8025"
  echo "   - Redis Commander: http://${HOST_IP}:8082"
else
  echo -e "${RED}✗${NC} Development stack failed to start"
  exit 1
fi

echo ""
sleep 1

# ====================================================================
# Phase 3: Monitoring Stack
# ====================================================================
echo -e "${BLUE}📊 Phase 3: Monitoring Stack${NC}"
echo "-----------------------------------"

echo "Starting monitoring services..."
docker compose -f docker-compose.monitoring.yml up -d

# Wait for Prometheus and Grafana
echo "Waiting for monitoring services to initialize..."
sleep 10

# Verify Prometheus
if curl -s http://${HOST_IP}:9090/-/healthy > /dev/null 2>&1; then
  echo -e "${GREEN}✓${NC} Prometheus operational: http://${HOST_IP}:9090"
else
  echo -e "${YELLOW}⚠${NC} Prometheus not yet ready (may need more time)"
fi

# Verify Grafana
grafana_status=$(curl -s -o /dev/null -w "%{http_code}" http://${HOST_IP}:3001 2>/dev/null || echo "000")
if [ "$grafana_status" = "200" ] || [ "$grafana_status" = "302" ]; then
  echo -e "${GREEN}✓${NC} Grafana operational: http://${HOST_IP}:3001"
  echo "   Default login: admin / admin"
else
  echo -e "${YELLOW}⚠${NC} Grafana not yet ready (HTTP ${grafana_status})"
fi

echo ""
sleep 2

# ====================================================================
# Phase 4: Pod Denmark (Territory DK)
# ====================================================================
echo -e "${BLUE}🇩🇰 Phase 4: Pod Denmark${NC}"
echo "-----------------------------------"

echo "Starting Pod Denmark..."
docker compose -f docker-compose.pod.yml \
  --env-file pods/denmark/.env up -d

# Wait for PostgreSQL health check
echo "Waiting for PostgreSQL to be healthy..."
timeout 60 bash -c 'until docker exec service-postgres-dk pg_isready -U unityplan > /dev/null 2>&1; do sleep 2; done' || {
  echo -e "${RED}✗${NC} PostgreSQL failed to become healthy"
  exit 1
}

echo -e "${GREEN}✓${NC} Pod Denmark operational"
echo "   - PostgreSQL: ${HOST_IP}:5432"
echo "   - Redis: ${HOST_IP}:6379"
echo "   - NATS: ${HOST_IP}:4222"
echo "   - NATS Monitor: http://${HOST_IP}:8222"

echo ""
sleep 2

# ====================================================================
# Phase 5: Pod Norway (Territory NO)
# ====================================================================
echo -e "${BLUE}🇳🇴 Phase 5: Pod Norway${NC}"
echo "-----------------------------------"

echo "Starting Pod Norway..."
docker compose -f docker-compose.pod.yml \
  --env-file pods/norway/.env up -d

# Wait for PostgreSQL
echo "Waiting for PostgreSQL to be healthy..."
timeout 60 bash -c 'until docker exec service-postgres-no pg_isready -U unityplan > /dev/null 2>&1; do sleep 2; done' || {
  echo -e "${RED}✗${NC} PostgreSQL failed to become healthy"
  exit 1
}

echo -e "${GREEN}✓${NC} Pod Norway operational"
echo "   - PostgreSQL: ${HOST_IP}:5433"
echo "   - Redis: ${HOST_IP}:6380"
echo "   - NATS: ${HOST_IP}:4223"
echo "   - NATS Monitor: http://${HOST_IP}:8223"

# Verify NATS cluster formation (2 nodes)
echo ""
echo "Verifying NATS cluster formation..."
sleep 3

routes=$(curl -s http://${HOST_IP}:8222/varz | jq '.cluster.urls | length')
if [ "$routes" -eq 1 ]; then
  echo -e "${GREEN}✓${NC} NATS cluster formed: 2 nodes connected"
else
  echo -e "${YELLOW}⚠${NC} NATS cluster: $routes routes detected (expected 1)"
fi

echo ""
sleep 2

# ====================================================================
# Phase 6: Pod Sweden (Territory SE)
# ====================================================================
echo -e "${BLUE}🇸🇪 Phase 6: Pod Sweden${NC}"
echo "-----------------------------------"

echo "Starting Pod Sweden..."
docker compose -f docker-compose.pod.yml \
  --env-file pods/sweden/.env up -d

# Wait for PostgreSQL
echo "Waiting for PostgreSQL to be healthy..."
timeout 60 bash -c 'until docker exec service-postgres-se pg_isready -U unityplan > /dev/null 2>&1; do sleep 2; done' || {
  echo -e "${RED}✗${NC} PostgreSQL failed to become healthy"
  exit 1
}

echo -e "${GREEN}✓${NC} Pod Sweden operational"
echo "   - PostgreSQL: ${HOST_IP}:5434"
echo "   - Redis: ${HOST_IP}:6381"
echo "   - NATS: ${HOST_IP}:4224"
echo "   - NATS Monitor: http://${HOST_IP}:8224"

# Verify NATS 3-node cluster
echo ""
echo "Verifying NATS 3-node cluster..."
sleep 3

routes=$(curl -s http://${HOST_IP}:8222/varz | jq '.cluster.urls | length')
if [ "$routes" -eq 2 ]; then
  echo -e "${GREEN}✓${NC} NATS cluster complete: 3 nodes connected"
else
  echo -e "${YELLOW}⚠${NC} NATS cluster: $routes routes detected (expected 2)"
fi

echo ""
sleep 2

# ====================================================================
# Final Verification
# ====================================================================
echo -e "${BLUE}🔍 Final Verification${NC}"
echo "-----------------------------------"

# Count running containers
total_containers=$(docker ps --filter "name=service-" --filter "name=monitoring-" --filter "name=dev-" --format "{{.Names}}" | wc -l)
echo "Total containers running: ${total_containers}"

# NATS cluster status
echo ""
echo "NATS Cluster Status:"
for pod in dk no se; do
  if [ "$pod" = "dk" ]; then port=8222; fi
  if [ "$pod" = "no" ]; then port=8223; fi
  if [ "$pod" = "se" ]; then port=8224; fi
  
  server_name=$(curl -s http://${HOST_IP}:${port}/varz | jq -r '.server_name')
  routes=$(curl -s http://${HOST_IP}:${port}/varz | jq '.cluster.urls | length')
  echo "  - ${server_name}: ${routes} cluster routes"
done

# Prometheus targets
echo ""
echo "Prometheus Monitoring:"
if curl -s http://${HOST_IP}:9090/-/healthy > /dev/null 2>&1; then
  targets=$(curl -s http://${HOST_IP}:9090/api/v1/targets)
  total=$(echo "$targets" | jq '.data.activeTargets | length')
  healthy=$(echo "$targets" | jq '[.data.activeTargets[] | select(.health == "up")] | length')
  
  echo "  - Total targets: ${total}"
  echo "  - Healthy targets: ${healthy}"
else
  echo -e "${YELLOW}  ⚠ Prometheus not yet fully operational${NC}"
fi

# ====================================================================
# Summary
# ====================================================================
echo ""
echo -e "${BLUE}==================================="
echo "Deployment Complete!"
echo -e "===================================${NC}\n"

echo -e "${GREEN}✅ Multi-pod deployment successful!${NC}\n"

echo "Access Points:"
echo "  🌐 Grafana:         http://${HOST_IP}:3001 (admin/admin)"
echo "  📊 Prometheus:      http://${HOST_IP}:9090"
echo "  🔍 Jaeger:          http://${HOST_IP}:16686"
echo "  🛠️  Dev Dashboard:   http://${HOST_IP}:8888"
echo "  🗄️  Adminer:         http://${HOST_IP}:8080"
echo "  📧 MailHog:         http://${HOST_IP}:8025"
echo "  🔴 Redis Commander: http://${HOST_IP}:8082"
echo ""

echo "Pod Services:"
echo "  🇩🇰 Denmark (DK):    PostgreSQL:5432, Redis:6379, NATS:4222"
echo "  🇳🇴 Norway (NO):     PostgreSQL:5433, Redis:6380, NATS:4223"
echo "  🇸🇪 Sweden (SE):     PostgreSQL:5434, Redis:6381, NATS:4224"
echo ""

echo "Next Steps:"
echo "  1. Run verification: ./scripts/verify-multi-pod.sh"
echo "  2. Check NATS cluster: nats server ls"
echo "  3. View logs: docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env logs -f"
echo "  4. Test messaging: nats pub --server=nats://${HOST_IP}:4222 test.hello 'Hello World!'"
echo ""

echo -e "${BLUE}Documentation: project_docs/6-multi-pod-deployment-guide.md${NC}"
