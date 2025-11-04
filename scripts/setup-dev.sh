#!/bin/bash
# Setup script for UnityPlan development environment

set -e

echo "🚀 UnityPlan Development Environment Setup"
echo "=========================================="
echo ""

# Check prerequisites
echo "📋 Checking prerequisites..."

command -v docker >/dev/null 2>&1 || { echo "❌ Docker is required but not installed. Aborting."; exit 1; }
command -v docker-compose >/dev/null 2>&1 || { echo "❌ Docker Compose is required but not installed. Aborting."; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo is required but not installed. Aborting."; exit 1; }
command -v node >/dev/null 2>&1 || { echo "❌ Node.js is required but not installed. Aborting."; exit 1; }

echo "✅ All prerequisites found"
echo ""

# Copy environment file
echo "📝 Setting up environment variables..."
if [ ! -f .env ]; then
    cp .env.example .env
    echo "✅ Created .env file from .env.example"
    echo "⚠️  Please review and update .env with your settings"
else
    echo "ℹ️  .env file already exists, skipping"
fi
echo ""

# Create necessary directories
echo "📁 Creating directories..."
mkdir -p docker/postgres-data
mkdir -p docker/nats-data
mkdir -p docker/redis-data
mkdir -p docker/ipfs-data
mkdir -p docker/matrix-data
mkdir -p uploads/avatars
mkdir -p temp
echo "✅ Directories created"
echo ""

# Install SQLx CLI
echo "🔧 Checking SQLx CLI..."
if ! command -v sqlx >/dev/null 2>&1; then
    echo "Installing SQLx CLI..."
    cargo install sqlx-cli --no-default-features --features postgres
    echo "✅ SQLx CLI installed"
else
    echo "✅ SQLx CLI already installed"
fi
echo ""

# Start Docker infrastructure
echo "🐳 Starting Docker infrastructure..."
docker-compose up -d postgres nats redis
echo "⏳ Waiting for services to be ready (30 seconds)..."
sleep 30
echo "✅ Infrastructure services started"
echo ""

# Check if services are healthy
echo "🏥 Checking service health..."
docker-compose ps
echo ""

echo "✅ Setup complete!"
echo ""
echo "📚 Next steps:"
echo "  1. Review and update .env file with your configuration"
echo "  2. Run database migrations: cd services && sqlx migrate run"
echo "  3. Start backend services: cargo run --bin <service-name>"
echo "  4. Start frontend: cd frontend && pnpm install && pnpm dev"
echo ""
echo "🔗 Useful URLs:"
echo "  - Adminer (DB): http://localhost:8080"
echo "  - NATS: http://localhost:8222"
echo ""
echo "Run './scripts/start-dev.sh' to start all services"
