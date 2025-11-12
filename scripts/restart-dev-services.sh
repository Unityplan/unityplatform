#!/bin/bash

# Restart all development services

echo "🔄 Restarting Unity Platform Development Environment"
echo "================================================"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Stop all services
"$SCRIPT_DIR/stop-dev-services.sh"

echo ""
echo "Waiting 3 seconds before restart..."
sleep 3
echo ""

# Start all services
"$SCRIPT_DIR/start-dev-services.sh"
