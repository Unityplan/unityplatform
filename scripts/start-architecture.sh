#!/bin/bash

set -e

echo "🚀 UnityPlan Development Environment - New Architecture"
echo "======================================================="
echo ""

# Show help
show_help() {
    echo "Usage: ./scripts/start-new-architecture.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --phase1           Start Phase 1 minimal setup (Forgejo + Registry)"
    echo "  --dev-tools        Start all development tools"
    echo "  --monitoring       Start monitoring stack (Prometheus, Grafana, Jaeger)"
    echo "  --pod <pod-id>     Start a specific pod (dk, no, se, eu)"
    echo "  --all-pods         Start all pods (DK, NO, SE, EU)"
    echo "  --full             Start everything (dev tools + monitoring + all pods)"
    echo "  --help             Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./scripts/start-new-architecture.sh --phase1"
    echo "  ./scripts/start-new-architecture.sh --dev-tools --monitoring"
    echo "  ./scripts/start-new-architecture.sh --pod dk"
    echo "  ./scripts/start-new-architecture.sh --full"
    exit 0
}

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Error: Docker is not running"
    echo "Please start Docker and try again"
    exit 1
fi

# Parse arguments
START_PHASE1=false
START_DEV_TOOLS=false
START_MONITORING=false
START_POD=""
START_ALL_PODS=false
START_FULL=false

# Show help if no arguments or --help
if [ $# -eq 0 ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
fi

while [[ $# -gt 0 ]]; do
    case $1 in
        --phase1)
            START_PHASE1=true
            shift
            ;;
        --dev-tools)
            START_DEV_TOOLS=true
            shift
            ;;
        --monitoring)
            START_MONITORING=true
            shift
            ;;
        --pod)
            START_POD="$2"
            shift 2
            ;;
        --all-pods)
            START_ALL_PODS=true
            shift
            ;;
        --full)
            START_FULL=true
            shift
            ;;
        --help)
            show_help
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            ;;
    esac
done

# Create mesh network if needed
create_mesh_network() {
    if ! docker network inspect unityplan-mesh-network > /dev/null 2>&1; then
        echo "📡 Creating mesh network..."
        docker network create unityplan-mesh-network
        echo "✅ Mesh network created"
    else
        echo "✅ Mesh network already exists"
    fi
    echo ""
}

# Start Phase 1 (Forgejo + Registry)
start_phase1() {
    echo "📦 Starting Phase 1: Forgejo + Docker Registry..."
    docker compose -f ../docker-compose.dev.yml up -d forgejo registry
    
    echo "⏳ Waiting for services to start..."
    sleep 3
    
    if curl -f http://localhost:3000 > /dev/null 2>&1; then
        echo "✅ Forgejo ready at http://localhost:3000"
    else
        echo "⚠️  Forgejo starting (check logs: docker logs dev-forgejo)"
    fi
    
    if curl -f http://localhost:5000/v2/ > /dev/null 2>&1; then
        echo "✅ Docker Registry ready at http://localhost:5000"
    else
        echo "⚠️  Registry starting (check logs: docker logs dev-registry)"
    fi
    echo ""
}

# Start all dev tools
start_dev_tools() {
    echo "🛠️  Starting All Development Tools..."
    docker compose -f ../docker-compose.dev.yml up -d
    
    echo "⏳ Waiting for services to start..."
    sleep 3
    
    echo "✅ Development tools started:"
    echo "   - Dev Dashboard: http://localhost:8888"
    echo "   - Adminer (DB UI): http://localhost:8080"
    echo "   - MailHog: http://localhost:8025"
    echo "   - Redis Commander: http://localhost:8082"
    echo "   - Forgejo: http://localhost:3000"
    echo "   - Docker Registry: http://localhost:5000"
    echo ""
}

# Start monitoring stack
start_monitoring() {
    echo "📊 Starting Monitoring Stack..."
    docker compose -f ../docker-compose.monitoring.yml up -d
    
    echo "⏳ Waiting for services to start..."
    sleep 5
    
    echo "✅ Monitoring stack started:"
    echo "   - Prometheus: http://192.168.60.133:9090"
    echo "   - Grafana: http://192.168.60.133:3001 (admin/admin)"
    echo "   - Jaeger: http://192.168.60.133:16686"
    echo ""
}

# Start a specific pod
start_pod() {
    local pod_id=$1
    local pod_name=""
    local env_file=""
    local compose_file=""
    
    case $pod_id in
        dk)
            pod_name="Denmark"
            env_file="pods/denmark/.env"
            compose_file="../docker-compose.pod.yml"
            ;;
        no)
            pod_name="Norway"
            env_file="pods/norway/.env"
            compose_file="../docker-compose.pod.yml"
            ;;
        se)
            pod_name="Sweden"
            env_file="pods/sweden/.env"
            compose_file="../docker-compose.pod.yml"
            ;;
        eu)
            pod_name="Europe (Multi-Territory)"
            env_file="pods/europe/.env"
            compose_file="../docker-compose.multi-territory-pod.yml"
            ;;
        *)
            echo "❌ Unknown pod: $pod_id"
            echo "Valid pods: dk, no, se, eu"
            exit 1
            ;;
    esac
    
    if [ ! -f "$env_file" ]; then
        echo "❌ Environment file not found: $env_file"
        exit 1
    fi
    
    echo "🚢 Starting Pod: $pod_name ($pod_id)..."
    docker compose -f $compose_file -p pod-$pod_id --env-file ../$env_file up -d
    
    echo "⏳ Waiting for pod to start..."
    sleep 5
    
    echo "✅ Pod $pod_name started"
    echo ""
}

# Start all pods
start_all_pods() {
    echo "🌐 Starting All Pods..."
    echo ""
    
    start_pod "dk"
    start_pod "no"
    start_pod "se"
    start_pod "eu"
    
    echo "✅ All pods started!"
    echo ""
}

# Main execution
create_mesh_network

if [ "$START_FULL" = true ]; then
    start_dev_tools
    start_monitoring
    start_all_pods
elif [ "$START_PHASE1" = true ]; then
    start_phase1
else
    if [ "$START_DEV_TOOLS" = true ]; then
        start_dev_tools
    fi
    
    if [ "$START_MONITORING" = true ]; then
        start_monitoring
    fi
    
    if [ ! -z "$START_POD" ]; then
        start_pod "$START_POD"
    fi
    
    if [ "$START_ALL_PODS" = true ]; then
        start_all_pods
    fi
fi

echo "======================================================="
echo "✅ UnityPlan Environment Started!"
echo "======================================================="
echo ""
echo "📝 Next Steps:"
echo ""
if [ "$START_PHASE1" = true ]; then
    echo "1. Configure Forgejo: http://localhost:3000"
    echo "2. Install forgejo-mcp for AI assistance"
    echo "3. Start building Rust backend services"
    echo ""
    echo "See: docs/forgejo-mcp-setup.md"
fi

if [ "$START_ALL_PODS" = true ] || [ "$START_FULL" = true ]; then
    echo "Run verification:"
    echo "  ./scripts/verify-multi-pod.sh"
    echo ""
fi

echo "View logs:"
echo "  cd scripts && docker compose -f ../docker-compose.dev.yml logs -f"
echo "  cd scripts && docker compose -f ../docker-compose.pod.yml -p pod-dk logs -f"
echo ""
echo "Stop services:"
echo "  cd scripts && docker compose -f ../docker-compose.dev.yml down"
echo "  cd scripts && docker compose -f ../docker-compose.pod.yml -p pod-dk down"
echo ""
