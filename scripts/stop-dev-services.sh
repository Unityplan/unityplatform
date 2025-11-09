#!/bin/bash

# Stop all development services for UnityPlan Platform

set -e

echo "🛑 Stopping UnityPlan Development Environment"
echo "=============================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

# Get workspace root
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WORKSPACE_ROOT"

# Stop Frontend
echo "Stopping Vite dev server..."
pkill -f "vite" || echo "  (not running)"

# Stop Backend Services
echo "Stopping auth-service..."
pkill -f "auth-service" || echo "  (not running)"

echo "Stopping user-service..."
pkill -f "user-service" || echo "  (not running)"

# Stop Docker Infrastructure
echo "Stopping Docker infrastructure..."
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down

echo ""
echo -e "${GREEN}✅ All services stopped${NC}"
