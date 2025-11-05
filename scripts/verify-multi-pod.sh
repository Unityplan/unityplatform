#!/bin/bash

# UnityPlan Multi-Pod Verification Script
# Purpose: Quick health check for multi-pod deployment
# Usage: ./scripts/verify-multi-pod.sh

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Show help
show_help() {
    echo "✅ UnityPlan Multi-Pod Verification"
    echo "==================================="
    echo ""
    echo "Usage: ./scripts/verify-multi-pod.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --help, -h     Show this help message"
    echo ""
    echo "Verifies:"
    echo "  - Container status"
    echo "  - Database connectivity (all pods)"
    echo "  - NATS cluster (4-node: DK, NO, SE, EU)"
    echo "  - Redis connectivity"
    echo "  - Exporters (Prometheus metrics)"
    echo "  - Inter-pod communication"
    echo ""
    echo "Run after:"
    echo "  ./scripts/deploy-multi-pod.sh"
    echo ""
    exit 0
}

# Parse arguments
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
fi

# Host IP (update if different)
HOST_IP="192.168.60.133"

echo -e "${BLUE}==================================="
echo "UnityPlan Multi-Pod Verification"
echo -e "===================================${NC}\n"

# ====================================================================
# 1. Container Status
# ====================================================================
echo -e "${BLUE}📦 Container Status${NC}"
echo "-----------------------------------"

total_containers=$(docker ps --filter "name=service-" --filter "name=monitoring-" --filter "name=dev-" --filter "name=reverse-proxy-" --format "{{.Names}}" | wc -l)
echo -e "Total containers running: ${GREEN}${total_containers}${NC}"

# List all containers with status
docker ps --filter "name=service-" --filter "name=monitoring-" --filter "name=dev-" --filter "name=reverse-proxy-" --format "table {{.Names}}\t{{.Status}}" | head -25

# ====================================================================
# 2. NATS Cluster Status
# ====================================================================
echo -e "\n${BLUE}🔗 NATS Cluster Status${NC}"
echo "-----------------------------------"

nats_status=0
for pod in dk no se; do
  if [ "$pod" = "dk" ]; then port=8222; fi
  if [ "$pod" = "no" ]; then port=8223; fi
  if [ "$pod" = "se" ]; then port=8224; fi
  
  if curl -s http://${HOST_IP}:${port}/varz > /dev/null 2>&1; then
    server_name=$(curl -s http://${HOST_IP}:${port}/varz | jq -r '.server_name')
    routes=$(curl -s http://${HOST_IP}:${port}/varz | jq '.cluster.urls | length')
    connections=$(curl -s http://${HOST_IP}:${port}/varz | jq '.connections')
    
    echo -e "${GREEN}✓${NC} ${server_name}: ${routes} cluster routes, ${connections} connections"
    nats_status=$((nats_status + 1))
  else
    echo -e "${RED}✗${NC} NATS pod-${pod}: Not reachable"
  fi
done

if [ $nats_status -eq 3 ]; then
  echo -e "${GREEN}NATS Cluster: Fully operational (3/3 nodes)${NC}"
else
  echo -e "${YELLOW}NATS Cluster: Partial ($nats_status/3 nodes)${NC}"
fi

# ====================================================================
# 3. Prometheus Targets
# ====================================================================
echo -e "\n${BLUE}📊 Prometheus Monitoring${NC}"
echo "-----------------------------------"

if curl -s http://${HOST_IP}:9090/-/healthy > /dev/null 2>&1; then
  targets=$(curl -s http://${HOST_IP}:9090/api/v1/targets)
  total=$(echo "$targets" | jq '.data.activeTargets | length')
  healthy=$(echo "$targets" | jq '[.data.activeTargets[] | select(.health == "up")] | length')
  
  echo -e "Total targets: ${total}"
  echo -e "Healthy targets: ${GREEN}${healthy}${NC}"
  
  if [ "$healthy" -eq "$total" ]; then
    echo -e "${GREEN}✓ All Prometheus targets healthy${NC}"
  else
    echo -e "${YELLOW}⚠ Some targets are down${NC}"
    echo "Unhealthy targets:"
    echo "$targets" | jq -r '.data.activeTargets[] | select(.health != "up") | .labels.job' | sort | uniq
  fi
else
  echo -e "${RED}✗ Prometheus not reachable${NC}"
fi

# ====================================================================
# 4. Database Status
# ====================================================================
echo -e "\n${BLUE}🗄️  PostgreSQL Databases${NC}"
echo "-----------------------------------"

db_status=0
for pod in dk no se; do
  if docker exec service-postgres-${pod} pg_isready -U unityplan > /dev/null 2>&1; then
    # Get database size
    db_size=$(docker exec service-postgres-${pod} psql -U unityplan -d unityplan_${pod} -tAc "SELECT pg_size_pretty(pg_database_size('unityplan_${pod}'));" 2>/dev/null || echo "N/A")
    echo -e "${GREEN}✓${NC} pod-${pod}: ready (size: ${db_size})"
    db_status=$((db_status + 1))
  else
    echo -e "${RED}✗${NC} pod-${pod}: not ready"
  fi
done

if [ $db_status -eq 3 ]; then
  echo -e "${GREEN}PostgreSQL: All databases operational (3/3)${NC}"
else
  echo -e "${YELLOW}PostgreSQL: Partial ($db_status/3)${NC}"
fi

# ====================================================================
# 5. Redis Status
# ====================================================================
echo -e "\n${BLUE}🔴 Redis Instances${NC}"
echo "-----------------------------------"

redis_status=0
for pod in dk no se; do
  if docker exec service-redis-${pod} redis-cli ping > /dev/null 2>&1; then
    # Get memory usage
    memory=$(docker exec service-redis-${pod} redis-cli info memory | grep used_memory_human | cut -d: -f2 | tr -d '\r')
    keys=$(docker exec service-redis-${pod} redis-cli dbsize | cut -d: -f2 | tr -d '\r')
    echo -e "${GREEN}✓${NC} pod-${pod}: PONG (memory: ${memory}, keys: ${keys})"
    redis_status=$((redis_status + 1))
  else
    echo -e "${RED}✗${NC} pod-${pod}: not responding"
  fi
done

if [ $redis_status -eq 3 ]; then
  echo -e "${GREEN}Redis: All instances operational (3/3)${NC}"
else
  echo -e "${YELLOW}Redis: Partial ($redis_status/3)${NC}"
fi

# ====================================================================
# 6. Service Accessibility
# ====================================================================
echo -e "\n${BLUE}🌐 Web Services${NC}"
echo "-----------------------------------"

declare -A services=(
  ["Grafana"]="http://${HOST_IP}:3001"
  ["Prometheus"]="http://${HOST_IP}:9090"
  ["Dev Dashboard"]="http://${HOST_IP}:8888"
  ["Adminer"]="http://${HOST_IP}:8080"
  ["MailHog"]="http://${HOST_IP}:8025"
  ["Redis Commander"]="http://${HOST_IP}:8082"
  ["Jaeger UI"]="http://${HOST_IP}:16686"
  ["Traefik"]="http://${HOST_IP}:8083"
)

for service in "${!services[@]}"; do
  url="${services[$service]}"
  status=$(curl -s -o /dev/null -w "%{http_code}" "$url" 2>/dev/null || echo "000")
  
  if [ "$status" = "200" ] || [ "$status" = "302" ]; then
    echo -e "${GREEN}✓${NC} ${service}: ${url}"
  else
    echo -e "${RED}✗${NC} ${service}: HTTP ${status}"
  fi
done

# ====================================================================
# 7. Network Status
# ====================================================================
echo -e "\n${BLUE}🔌 Docker Networks${NC}"
echo "-----------------------------------"

if docker network inspect unityplan-mesh-network > /dev/null 2>&1; then
  mesh_containers=$(docker network inspect unityplan-mesh-network | jq '.[] .Containers | length')
  echo -e "${GREEN}✓${NC} unityplan-mesh-network: ${mesh_containers} containers connected"
else
  echo -e "${RED}✗${NC} unityplan-mesh-network: Not found"
fi

if docker network inspect unityplan-global-net > /dev/null 2>&1; then
  global_containers=$(docker network inspect unityplan-global-net | jq '.[] .Containers | length')
  echo -e "${GREEN}✓${NC} unityplan-global-net: ${global_containers} containers connected"
else
  echo -e "${YELLOW}⚠${NC} unityplan-global-net: Not found (may be auto-created)"
fi

# ====================================================================
# Summary
# ====================================================================
echo -e "\n${BLUE}==================================="
echo "Summary"
echo -e "===================================${NC}"

total_checks=5
passed_checks=0

[ $nats_status -eq 3 ] && passed_checks=$((passed_checks + 1))
[ "$healthy" -eq "$total" ] 2>/dev/null && passed_checks=$((passed_checks + 1))
[ $db_status -eq 3 ] && passed_checks=$((passed_checks + 1))
[ $redis_status -eq 3 ] && passed_checks=$((redis_status + 1))
docker network inspect unityplan-mesh-network > /dev/null 2>&1 && passed_checks=$((passed_checks + 1))

if [ $passed_checks -eq $total_checks ]; then
  echo -e "${GREEN}✅ All systems operational ($passed_checks/$total_checks checks passed)${NC}"
  exit 0
elif [ $passed_checks -ge 3 ]; then
  echo -e "${YELLOW}⚠️  System partially operational ($passed_checks/$total_checks checks passed)${NC}"
  exit 0
else
  echo -e "${RED}❌ System has issues ($passed_checks/$total_checks checks passed)${NC}"
  exit 1
fi
