#!/bin/bash
# Rebuild Docker Infrastructure with Correct Naming
# This script stops all containers, removes old networks/volumes (except Forgejo),
# and restarts everything with consistent unityplatform-* naming

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(dirname "$SCRIPT_DIR")"

echo "🧹 Unity Platform Docker Infrastructure Rebuild"
echo "================================================"
echo ""
echo "⚠️  WARNING: This will:"
echo "   - Stop all Unity Platform containers"
echo "   - Remove all networks (unityplan-* and unityplatform-*)"
echo "   - Remove all data volumes EXCEPT Forgejo git repository"
echo "   - Recreate everything with consistent naming"
echo ""
read -p "Continue? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    echo "Cancelled."
    exit 0
fi

cd "$WORKSPACE_DIR"

echo ""
echo "📦 Step 1: Stopping all containers..."
echo "======================================"

# Stop dev stack (old naming)
echo "Stopping dev stack..."
docker compose -f docker-compose.dev.yml down 2>/dev/null || true

# Stop monitoring stack
echo "Stopping monitoring stack..."
docker compose -f docker-compose.monitoring.yml down 2>/dev/null || true

# Stop pod stacks
echo "Stopping Denmark pod..."
docker compose -f docker-compose.pod.yml -p pod-dk --env-file pods/denmark/.env down 2>/dev/null || true

echo "Stopping Norway pod..."
docker compose -f docker-compose.pod.yml -p pod-no --env-file pods/norway/.env down 2>/dev/null || true

echo "Stopping Sweden pod..."
docker compose -f docker-compose.pod.yml -p pod-se --env-file pods/sweden/.env down 2>/dev/null || true

echo "Stopping Europe pod..."
docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu --env-file pods/europe/.env down 2>/dev/null || true

# Remove any lingering containers that might conflict
echo "Removing any lingering containers..."
docker rm -f dev-adminer dev-redis-commander dev-forgejo dev-mailhog dev-registry dev-dashboard 2>/dev/null || true
docker rm -f monitoring-prometheus monitoring-grafana monitoring-loki monitoring-promtail monitoring-tempo 2>/dev/null || true

echo ""
echo "🌐 Step 2: Removing old networks..."
echo "===================================="
docker network rm unityplan-mesh-network 2>/dev/null || true
docker network rm unityplan-global-net 2>/dev/null || true
docker network rm unityplan-pod-dk-net 2>/dev/null || true
docker network rm unityplan-pod-no-net 2>/dev/null || true
docker network rm unityplan-pod-se-net 2>/dev/null || true
docker network rm unityplatform-mesh-network 2>/dev/null || true
docker network rm unityplatform-global-net 2>/dev/null || true
docker network rm unityplatform-pod-dk-net 2>/dev/null || true
docker network rm unityplatform-pod-no-net 2>/dev/null || true
docker network rm unityplatform-pod-se-net 2>/dev/null || true

echo ""
echo "🗑️  Step 3: Archiving old data and removing volumes (preserving Forgejo)..."
echo "==========================================================================="
echo "Preserving: Forgejo git repository"

# Archive existing docker folder
if [ -d "docker" ]; then
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    echo "Archiving docker/ to docker.archive.${TIMESTAMP}/"
    mv docker "docker.archive.${TIMESTAMP}"
    ARCHIVE_DIR="docker.archive.${TIMESTAMP}"
fi

# Recreate docker structure
echo "Creating fresh docker directory structure..."
mkdir -p docker/pods/dk
mkdir -p docker/pods/no
mkdir -p docker/pods/se
mkdir -p docker/pods/eu
mkdir -p docker/prometheus
mkdir -p docker/grafana/provisioning
mkdir -p docker/grafana-data
mkdir -p docker/prometheus-data
mkdir -p docker/dev-dashboard
mkdir -p docker/traefik/landing-page
mkdir -p docker/traefik/error-pages
mkdir -p docker/traefik-data
mkdir -p docker/postgres

# Copy essential config files from archive if it exists
if [ -d "$ARCHIVE_DIR" ]; then
    echo "Restoring essential configuration files..."
    
    # Prometheus configs
    if [ -f "$ARCHIVE_DIR/prometheus/prometheus.yml" ]; then
        cp "$ARCHIVE_DIR/prometheus/prometheus.yml" docker/prometheus/
    fi
    if [ -f "$ARCHIVE_DIR/prometheus/prometheus-central.yml" ]; then
        cp "$ARCHIVE_DIR/prometheus/prometheus-central.yml" docker/prometheus/
    fi
    
    # Grafana provisioning
    if [ -d "$ARCHIVE_DIR/grafana/provisioning" ]; then
        cp -r "$ARCHIVE_DIR/grafana/provisioning"/* docker/grafana/provisioning/ 2>/dev/null || true
    fi
    
    # Traefik
    if [ -d "$ARCHIVE_DIR/traefik" ]; then
        cp -r "$ARCHIVE_DIR/traefik"/* docker/traefik/ 2>/dev/null || true
    fi
    
    # Dev dashboard
    if [ -f "$ARCHIVE_DIR/dev-dashboard/index.html" ]; then
        cp "$ARCHIVE_DIR/dev-dashboard/index.html" docker/dev-dashboard/
    fi
    
    # Placeholder pages
    if [ -d "$ARCHIVE_DIR/placeholder-pages" ]; then
        cp -r "$ARCHIVE_DIR/placeholder-pages" docker/
    fi
    
    # PostgreSQL init.sql
    if [ -f "$ARCHIVE_DIR/postgres/init.sql" ]; then
        cp "$ARCHIVE_DIR/postgres/init.sql" docker/postgres/
    fi
fi

# Remove Docker volumes (except Forgejo)
echo "Removing old Docker volumes..."
docker volume rm unityplan-dev_registry-data 2>/dev/null || true
docker volume rm dk-postgres-data 2>/dev/null || true
docker volume rm dk-redis-data 2>/dev/null || true
docker volume rm dk-nats-data 2>/dev/null || true
docker volume rm dk-ipfs-data 2>/dev/null || true
docker volume rm dk-matrix-data 2>/dev/null || true

echo "✅ Old data archived, Forgejo data preserved at: unityplan-dev_forgejo-data"

echo ""
echo "🌐 Step 4: Creating shared mesh network..."
echo "==========================================="
# The mesh-network is shared across all compose files, so create it manually
# The global-net is created by docker-compose.dev.yml
docker network rm unityplatform-mesh-network 2>/dev/null || true
docker network create unityplatform-mesh-network
echo "✅ Mesh network created (global-net will be created by docker-compose)"

echo ""
echo "🚀 Step 5: Starting services..."
echo "================================"

# Start dev stack
echo "Starting development stack..."
docker compose -f docker-compose.dev.yml up -d

# Start monitoring stack
echo "Starting monitoring stack..."
docker compose -f docker-compose.monitoring.yml up -d

# Start Denmark pod
echo "Starting Denmark pod..."
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env up -d

echo ""
echo "✅ Infrastructure rebuild complete!"
echo ""
echo "📊 Quick Status Check:"
echo "======================"
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep -E "service-|monitoring-|dev-"

echo ""
echo "🔧 Next Steps:"
echo "=============="
echo "1. Wait ~30 seconds for databases to initialize"
echo "2. Run migrations: cd services && sqlx migrate run"
echo "3. Test auth-service"
echo ""
echo "📚 Access Points:"
echo "================="
echo "  Grafana:        http://localhost:3001 (admin/admin)"
echo "  Adminer:        http://localhost:8080"
echo "  Dev Dashboard:  http://localhost:8888"
echo "  Forgejo:        http://192.168.60.133:3000"
echo ""
