#!/bin/bash
# Start all development services

set -e

echo "🚀 Starting UnityPlan Development Services"
echo "========================================"
echo ""

# Start Docker infrastructure
echo "🐳 Starting infrastructure services..."
docker-compose up -d
echo "✅ Infrastructure started"
echo ""

echo "📊 Service Status:"
docker-compose ps
echo ""

echo "✅ All services started!"
echo ""
echo "🔗 Access points:"
echo "  - Frontend: http://localhost:5173"
echo "  - API Gateway: http://localhost:8000"
echo "  - Adminer (DB): http://localhost:8080"
echo "  - NATS Monitoring: http://localhost:8222"
echo "  - IPFS Gateway: http://localhost:8080/ipfs/"
echo ""
echo "📝 To view logs: docker-compose logs -f [service-name]"
echo "🛑 To stop: docker-compose down"
