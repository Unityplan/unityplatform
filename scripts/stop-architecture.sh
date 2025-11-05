#!/bin/bash

set -e

echo "🛑 UnityPlan - Stop Services"
echo "=============================="
echo ""

# Show help
show_help() {
    echo "Usage: ./scripts/stop-new-architecture.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --dev-tools        Stop development tools"
    echo "  --monitoring       Stop monitoring stack"
    echo "  --pod <pod-id>     Stop a specific pod (dk, no, se, eu)"
    echo "  --all-pods         Stop all pods"
    echo "  --all              Stop everything"
    echo "  --clean            Remove volumes (WARNING: deletes data)"
    echo "  --help             Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./scripts/stop-new-architecture.sh --dev-tools"
    echo "  ./scripts/stop-new-architecture.sh --pod dk"
    echo "  ./scripts/stop-new-architecture.sh --all"
    echo "  ./scripts/stop-new-architecture.sh --all --clean  # Deletes all data!"
    exit 0
}

# Parse arguments
STOP_DEV_TOOLS=false
STOP_MONITORING=false
STOP_POD=""
STOP_ALL_PODS=false
STOP_ALL=false
CLEAN_VOLUMES=false

# Show help if no arguments or --help
if [ $# -eq 0 ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
fi

while [[ $# -gt 0 ]]; do
    case $1 in
        --dev-tools)
            STOP_DEV_TOOLS=true
            shift
            ;;
        --monitoring)
            STOP_MONITORING=true
            shift
            ;;
        --pod)
            STOP_POD="$2"
            shift 2
            ;;
        --all-pods)
            STOP_ALL_PODS=true
            shift
            ;;
        --all)
            STOP_ALL=true
            shift
            ;;
        --clean)
            CLEAN_VOLUMES=true
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

# Confirm clean
if [ "$CLEAN_VOLUMES" = true ]; then
    echo "⚠️  WARNING: This will DELETE ALL DATA!"
    echo ""
    read -p "Are you sure you want to remove all volumes? (yes/NO): " -r
    echo
    if [[ ! $REPLY =~ ^yes$ ]]; then
        echo "ℹ️  Clean cancelled. Stopping without removing data."
        CLEAN_VOLUMES=false
    fi
    echo ""
fi

# Stop development tools
stop_dev_tools() {
    echo "🛑 Stopping Development Tools..."
    if [ "$CLEAN_VOLUMES" = true ]; then
        docker compose -f docker-compose.dev.yml down -v
        echo "✅ Dev tools stopped and volumes removed"
    else
        docker compose -f docker-compose.dev.yml down
        echo "✅ Dev tools stopped"
    fi
    echo ""
}

# Stop monitoring stack
stop_monitoring() {
    echo "🛑 Stopping Monitoring Stack..."
    if [ "$CLEAN_VOLUMES" = true ]; then
        docker compose -f docker-compose.monitoring.yml down -v
        echo "✅ Monitoring stopped and volumes removed"
    else
        docker compose -f docker-compose.monitoring.yml down
        echo "✅ Monitoring stopped"
    fi
    echo ""
}

# Stop a specific pod
stop_pod() {
    local pod_id=$1
    local compose_file=""
    
    case $pod_id in
        dk|no|se)
            compose_file="docker-compose.pod.yml"
            ;;
        eu)
            compose_file="docker-compose.multi-territory-pod.yml"
            ;;
        *)
            echo "❌ Unknown pod: $pod_id"
            echo "Valid pods: dk, no, se, eu"
            exit 1
            ;;
    esac
    
    echo "🛑 Stopping Pod: $pod_id..."
    if [ "$CLEAN_VOLUMES" = true ]; then
        docker compose -f $compose_file -p pod-$pod_id down -v
        echo "✅ Pod $pod_id stopped and volumes removed"
    else
        docker compose -f $compose_file -p pod-$pod_id down
        echo "✅ Pod $pod_id stopped"
    fi
    echo ""
}

# Stop all pods
stop_all_pods() {
    echo "🛑 Stopping All Pods..."
    echo ""
    
    stop_pod "dk"
    stop_pod "no"
    stop_pod "se"
    stop_pod "eu"
    
    echo "✅ All pods stopped"
    echo ""
}

# Main execution
if [ "$STOP_ALL" = true ]; then
    stop_dev_tools
    stop_monitoring
    stop_all_pods
else
    if [ "$STOP_DEV_TOOLS" = true ]; then
        stop_dev_tools
    fi
    
    if [ "$STOP_MONITORING" = true ]; then
        stop_monitoring
    fi
    
    if [ ! -z "$STOP_POD" ]; then
        stop_pod "$STOP_POD"
    fi
    
    if [ "$STOP_ALL_PODS" = true ]; then
        stop_all_pods
    fi
fi

echo "=============================="
echo "✅ Services Stopped"
echo "=============================="
echo ""

if [ "$CLEAN_VOLUMES" = true ]; then
    echo "⚠️  All data has been removed!"
else
    echo "ℹ️  Data volumes preserved."
    echo "   To remove data, run with --clean flag"
fi
echo ""
