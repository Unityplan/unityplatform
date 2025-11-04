#!/bin/bash
# Stop all development services and optionally clean data

echo "🛑 Stopping UnityPlan Development Services"
echo "========================================"
echo ""

# Parse arguments
CLEAN_DATA=false
while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN_DATA=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: ./scripts/stop-dev.sh [--clean]"
            echo "  --clean: Remove all data volumes"
            exit 1
            ;;
    esac
done

# Stop services
echo "🐳 Stopping Docker services..."
docker-compose down

if [ "$CLEAN_DATA" = true ]; then
    echo ""
    echo "⚠️  Cleaning data volumes..."
    read -p "This will delete all database data. Are you sure? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        docker-compose down -v
        rm -rf docker/postgres-data/*
        rm -rf docker/nats-data/*
        rm -rf docker/redis-data/*
        rm -rf docker/ipfs-data/*
        rm -rf docker/matrix-data/*
        echo "✅ Data cleaned"
    else
        echo "ℹ️  Data cleaning cancelled"
    fi
fi

echo ""
echo "✅ Services stopped"
