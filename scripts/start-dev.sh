#!/bin/bash

set -e

# Show help if --help is passed
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "🚀 UnityPlan Phase 1 Development Environment"
    echo "============================================"
    echo ""
    echo "Usage: ./scripts/start-dev.sh"
    echo ""
    echo "Starts minimal Phase 1 development environment:"
    echo "  - Forgejo (version control + MCP integration)"
    echo "  - Docker Registry (local image storage)"
    echo ""
    echo "This is the recommended setup for MVP development."
    echo ""
    echo "See also:"
    echo "  ./scripts/start-architecture.sh --help  # For more deployment options"
    echo ""
    exit 0
fi

echo "🚀 Starting UnityPlan Phase 1 Development Environment"
echo "=================================================="
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Error: Docker is not running"
    echo "Please start Docker and try again"
    exit 1
fi

echo "✅ Docker is running"
echo ""

# Create mesh network if it doesn't exist
if ! docker network inspect unityplan-mesh-network > /dev/null 2>&1; then
    echo "📡 Creating mesh network..."
    docker network create unityplan-mesh-network
    echo "✅ Mesh network created"
else
    echo "✅ Mesh network already exists"
fi
echo ""

# Start Forgejo and Docker Registry (Phase 1 essentials)
echo "📦 Starting Forgejo + Docker Registry..."
docker compose -f ../docker-compose.dev.yml up -d forgejo registry

# Wait for Forgejo to be ready
echo "⏳ Waiting for Forgejo to start..."
sleep 5

# Check if Forgejo is accessible
if curl -f http://localhost:3000 > /dev/null 2>&1; then
    echo "✅ Forgejo is ready at http://localhost:3000"
else
    echo "⚠️  Forgejo is starting (may take a minute)..."
    echo "   Check logs: docker logs dev-forgejo"
fi
echo ""

# Check Docker Registry
if curl -f http://localhost:5000/v2/ > /dev/null 2>&1; then
    echo "✅ Docker Registry is ready at http://localhost:5000"
else
    echo "⚠️  Docker Registry is starting..."
    echo "   Check logs: docker logs dev-registry"
fi
echo ""

echo "=================================================="
echo "🎉 Phase 1 Development Environment Ready!"
echo "=================================================="
echo ""
echo "📝 Next Steps:"
echo ""
echo "1. Configure Forgejo (first-time setup):"
echo "   → Open http://localhost:3000"
echo "   → Create admin account"
echo "   → Create 'unityplan_platform' repository"
echo ""
echo "2. Push code to Forgejo:"
echo "   git remote add forgejo http://localhost:3000/admin/unityplan_platform.git"
echo "   git push forgejo main"
echo ""
echo "3. Install forgejo-mcp for AI assistance:"
echo "   npm install -g @goern/forgejo-mcp"
echo "   # See docs/forgejo-mcp-setup.md for configuration"
echo ""
echo "4. Configure insecure registry for localhost:5000:"
echo "   # Add to /etc/docker/daemon.json:"
echo '   { "insecure-registries": ["localhost:5000"] }'
echo "   sudo systemctl restart docker"
echo ""
echo "5. Start building Rust backend services:"
echo "   cd services"
echo "   cargo build --release"
echo ""
echo "=================================================="
echo "📚 Documentation:"
echo "   - Forgejo MCP Setup: docs/forgejo-mcp-setup.md"
echo "   - Phase 1 Roadmap: project_status/phase-1-mvp-roadmap.md"
echo "   - Architecture: project_docs/5-multi-pod-architecture.md"
echo "=================================================="
echo ""
echo "🛑 To stop:"
echo "   cd scripts && docker compose -f ../docker-compose.dev.yml down"
echo ""
