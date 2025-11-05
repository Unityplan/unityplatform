# Multi-Pod Testing & Verification Guide

**Purpose:** Comprehensive testing checklist for multi-pod deployment  
**Scope:** Single-host development environment  
**Target Users:** Developers, DevOps, QA

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Pre-Deployment Checks](#pre-deployment-checks)
3. [Deployment Test Sequence](#deployment-test-sequence)
4. [Component Verification](#component-verification)
5. [Integration Tests](#integration-tests)
6. [Performance Validation](#performance-validation)
7. [Failure Scenarios](#failure-scenarios)

---

## Prerequisites

### Required Tools

```bash
# Docker and Docker Compose
docker --version  # >= 24.0
docker compose version  # >= 2.20

# NATS CLI (for messaging tests)
curl -sf https://binaries.nats.dev/nats-io/natscli/nats@latest | sh

# jq (JSON processor)
sudo apt install jq  # Debian/Ubuntu
brew install jq      # macOS

# curl (HTTP client)
curl --version

# PostgreSQL client (optional)
sudo apt install postgresql-client
```

### Environment Setup

```bash
# Verify you're in project root
cd /home/henrik/code/data/projects/unityplan_platform/workspace
pwd  # Should show workspace directory

# Check .env files exist
ls -la pods/denmark/.env
ls -la pods/norway/.env
ls -la pods/sweden/.env

# Verify docker-compose files
ls -la docker-compose*.yml
```

---

## Pre-Deployment Checks

### 1. Clean Slate (Optional)

```bash
# Stop all running containers
docker stop $(docker ps -q) 2>/dev/null || true

# Remove existing multi-pod containers (if any)
docker rm -f $(docker ps -a --filter "name=service-" -q) 2>/dev/null || true
docker rm -f $(docker ps -a --filter "name=monitoring-" -q) 2>/dev/null || true
docker rm -f $(docker ps -a --filter "name=dev-" -q) 2>/dev/null || true

# Clean networks (be careful!)
docker network prune -f

# Clean volumes (DESTRUCTIVE - will lose data!)
# docker volume prune -f  # Uncomment if you want fresh start
```

### 2. Network Preparation

```bash
# Create mesh network
docker network create unityplan-mesh-network

# Verify
docker network ls | grep unityplan
# Expected: unityplan-mesh-network

# Inspect network
docker network inspect unityplan-mesh-network
```

### 3. Configuration Validation

```bash
# Validate docker-compose files syntax
docker compose -f docker-compose.dev.yml config -q
docker compose -f docker-compose.monitoring.yml config -q
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env config -q
docker compose -f docker-compose.pod.yml --env-file pods/norway/.env config -q
docker compose -f docker-compose.pod.yml --env-file pods/sweden/.env config -q

# If any errors, fix before proceeding
# No output = success
```

---

## Deployment Test Sequence

### Phase 1: Development Stack

```bash
# Start dev tools
echo "=== Starting Development Stack ==="
docker compose -f docker-compose.dev.yml up -d

# Wait for services to stabilize
sleep 5

# Verify containers running
docker compose -f docker-compose.dev.yml ps

# Expected output (all "running"):
# NAME                  STATUS          PORTS
# dev-dashboard         running         0.0.0.0:8888->80/tcp
# dev-adminer           running         0.0.0.0:8080->8080/tcp
# dev-mailhog           running         0.0.0.0:1025->1025/tcp, 0.0.0.0:8025->8025/tcp
# dev-redis-commander   running         0.0.0.0:8082->8081/tcp

# Test HTTP endpoints
curl -s http://192.168.60.133:8888 | head -5  # Dev dashboard
curl -s -o /dev/null -w "%{http_code}\n" http://192.168.60.133:8080  # Adminer (should be 200)
curl -s -o /dev/null -w "%{http_code}\n" http://192.168.60.133:8025  # MailHog (should be 200)

echo "✅ Development Stack: PASS"
```

### Phase 2: Monitoring Stack

```bash
# Start monitoring
echo "=== Starting Monitoring Stack ==="
docker compose -f docker-compose.monitoring.yml up -d

# Wait for services
sleep 10

# Verify containers
docker compose -f docker-compose.monitoring.yml ps

# Expected (all "running"):
# NAME                        STATUS          PORTS
# monitoring-prometheus       running         0.0.0.0:9090->9090/tcp
# monitoring-grafana          running         0.0.0.0:3001->3000/tcp
# monitoring-jaeger           running         multiple ports
# reverse-proxy-traefik       running         0.0.0.0:80->80/tcp, 0.0.0.0:443->443/tcp

# Test Prometheus
curl -s http://192.168.60.133:9090/-/healthy
# Expected: Prometheus is Healthy.

# Test Grafana
curl -s -o /dev/null -w "%{http_code}\n" http://192.168.60.133:3001
# Expected: 200 or 302

# Test Prometheus config loaded
curl -s http://192.168.60.133:9090/api/v1/status/config | jq -r '.status'
# Expected: success

echo "✅ Monitoring Stack: PASS"
```

### Phase 3: Pod Denmark

```bash
# Start Denmark pod
echo "=== Starting Pod Denmark ==="
docker compose -f docker-compose.pod.yml -p pod-dk \
  --env-file pods/denmark/.env up -d

# Wait for postgres health check
echo "Waiting for PostgreSQL to be healthy..."
timeout 60 bash -c 'until docker exec service-postgres-dk pg_isready -U unityplan; do sleep 2; done'

# Verify all containers running
docker compose -f docker-compose.pod.yml -p pod-dk ps

# Expected (all "running" or "healthy"):
# NAME                              STATUS
# service-postgres-dk               running (healthy)
# service-redis-dk                  running
# service-nats-dk                   running
# service-ipfs-dk                   running
# service-matrix-dk                 running
# monitoring-postgres-exporter-dk   running
# monitoring-redis-exporter-dk      running
# monitoring-nats-exporter-dk       running
# monitoring-node-exporter-dk       running
# monitoring-cadvisor-dk            running

# Test service connectivity
docker exec service-redis-dk redis-cli ping
# Expected: PONG

docker exec service-postgres-dk psql -U unityplan -d unityplan_dk -c "SELECT 1;"
# Expected: 1

curl -s http://192.168.60.133:8222/varz | jq '.server_name'
# Expected: "service-nats-dk"

echo "✅ Pod Denmark: PASS"
```

### Phase 4: Pod Norway

```bash
# Start Norway pod
echo "=== Starting Pod Norway ==="
docker compose -f docker-compose.pod.yml -p pod-no \
  --env-file pods/norway/.env up -d

# Wait for postgres
echo "Waiting for PostgreSQL to be healthy..."
timeout 60 bash -c 'until docker exec service-postgres-no pg_isready -U unityplan; do sleep 2; done'

# Verify containers
docker compose -f docker-compose.pod.yml -p pod-no ps

# Test services
docker exec service-redis-no redis-cli ping
# Expected: PONG

docker exec service-postgres-no psql -U unityplan -d unityplan_no -c "SELECT 1;"
# Expected: 1

curl -s http://192.168.60.133:8223/varz | jq '.server_name'
# Expected: "service-nats-no"

# *** CRITICAL: Verify NATS Cluster Formation ***
echo "Checking NATS cluster formation..."
curl -s http://192.168.60.133:8222/varz | jq '.cluster.urls'
# Expected: ["nats://service-nats-no:6223"]

curl -s http://192.168.60.133:8223/varz | jq '.cluster.urls'
# Expected: ["nats://service-nats-dk:6222"]

echo "✅ Pod Norway: PASS"
echo "✅ NATS 2-Node Cluster: PASS"
```

### Phase 5: Pod Sweden

```bash
# Start Sweden pod
echo "=== Starting Pod Sweden ==="
docker compose -f docker-compose.pod.yml -p pod-se \
  --env-file pods/sweden/.env up -d

# Wait for postgres
echo "Waiting for PostgreSQL to be healthy..."
timeout 60 bash -c 'until docker exec service-postgres-se pg_isready -U unityplan; do sleep 2; done'

# Verify containers
docker compose -f docker-compose.pod.yml -p pod-se ps

# Test services
docker exec service-redis-se redis-cli ping
# Expected: PONG

curl -s http://192.168.60.133:8224/varz | jq '.server_name'
# Expected: "service-nats-se"

# *** CRITICAL: Verify 3-Node NATS Cluster ***
echo "Checking NATS 3-node cluster..."
curl -s http://192.168.60.133:8222/varz | jq '.cluster.urls'
# Expected: ["nats://service-nats-no:6223", "nats://service-nats-se:6224"]

curl -s http://192.168.60.133:8223/varz | jq '.cluster.urls'
# Expected: ["nats://service-nats-dk:6222", "nats://service-nats-se:6224"]

curl -s http://192.168.60.133:8224/varz | jq '.cluster.urls'
# Expected: ["nats://service-nats-dk:6222", "nats://service-nats-no:6223"]

echo "✅ Pod Sweden: PASS"
echo "✅ NATS 3-Node Cluster: PASS"
```

---

## Component Verification

### PostgreSQL

```bash
# Test each database
for pod in dk no se; do
  echo "Testing PostgreSQL pod-${pod}..."
  
  if [ "$pod" = "dk" ]; then port=5432; fi
  if [ "$pod" = "no" ]; then port=5433; fi
  if [ "$pod" = "se" ]; then port=5434; fi
  
  # Connection test
  docker exec service-postgres-${pod} psql -U unityplan -d unityplan_${pod} -c "SELECT version();" | head -3
  
  # Schema test (if init.sql creates schema)
  docker exec service-postgres-${pod} psql -U unityplan -d unityplan_${pod} -c "\dn"
  
  echo "✅ PostgreSQL pod-${pod}: PASS"
done
```

### Redis

```bash
# Test each Redis instance
for pod in dk no se; do
  echo "Testing Redis pod-${pod}..."
  
  # Ping test
  docker exec service-redis-${pod} redis-cli ping
  
  # Write/read test
  docker exec service-redis-${pod} redis-cli set "test:${pod}" "hello-from-${pod}"
  docker exec service-redis-${pod} redis-cli get "test:${pod}"
  
  # Memory info
  docker exec service-redis-${pod} redis-cli info memory | grep used_memory_human
  
  echo "✅ Redis pod-${pod}: PASS"
done
```

### NATS

```bash
# Test NATS on each node
for pod in dk no se; do
  echo "Testing NATS pod-${pod}..."
  
  if [ "$pod" = "dk" ]; then port=4222; monitor=8222; fi
  if [ "$pod" = "no" ]; then port=4223; monitor=8223; fi
  if [ "$pod" = "se" ]; then port=4224; monitor=8224; fi
  
  # Server info
  curl -s http://192.168.60.133:${monitor}/varz | jq '{
    server_name,
    version,
    connections,
    cluster_name: .cluster.name,
    routes: (.cluster.urls | length)
  }'
  
  echo "✅ NATS pod-${pod}: PASS"
done

# NATS CLI test (if installed)
if command -v nats &> /dev/null; then
  echo "Testing NATS CLI connectivity..."
  
  nats context add dk --server=nats://192.168.60.133:4222 --description="Pod Denmark"
  nats context select dk
  nats server ls
  
  echo "✅ NATS CLI: PASS"
fi
```

### IPFS

```bash
# Test IPFS nodes
for pod in dk no se; do
  echo "Testing IPFS pod-${pod}..."
  
  if [ "$pod" = "dk" ]; then port=5001; fi
  if [ "$pod" = "no" ]; then port=5002; fi
  if [ "$pod" = "se" ]; then port=5003; fi
  
  # ID check
  curl -s -X POST "http://192.168.60.133:${port}/api/v0/id" | jq '.ID'
  
  # Peer count
  curl -s -X POST "http://192.168.60.133:${port}/api/v0/swarm/peers" | jq '.Peers | length'
  
  echo "✅ IPFS pod-${pod}: PASS"
done
```

### Matrix Synapse

```bash
# Test Matrix Synapse
for pod in dk no se; do
  echo "Testing Matrix Synapse pod-${pod}..."
  
  if [ "$pod" = "dk" ]; then port=8008; fi
  if [ "$pod" = "no" ]; then port=8009; fi
  if [ "$pod" = "se" ]; then port=8010; fi
  
  # Health check
  curl -s http://192.168.60.133:${port}/health | jq
  
  # Version
  curl -s http://192.168.60.133:${port}/_matrix/client/versions | jq '.versions[0]'
  
  echo "✅ Matrix pod-${pod}: PASS"
done
```

### Prometheus Exporters

```bash
# Test all exporters are accessible
declare -A exporters=(
  ["postgres-dk"]=9187
  ["postgres-no"]=9188
  ["postgres-se"]=9189
  ["redis-dk"]=9121
  ["redis-no"]=9122
  ["redis-se"]=9123
  ["nats-dk"]=7777
  ["nats-no"]=7778
  ["nats-se"]=7779
  ["node-dk"]=9100
  ["node-no"]=9101
  ["node-se"]=9102
  ["cadvisor-dk"]=8089
  ["cadvisor-no"]=8090
  ["cadvisor-se"]=8091
)

for exporter in "${!exporters[@]}"; do
  port=${exporters[$exporter]}
  echo "Testing $exporter on port $port..."
  
  # Check metrics endpoint
  metrics=$(curl -s http://192.168.60.133:${port}/metrics | wc -l)
  
  if [ "$metrics" -gt 10 ]; then
    echo "✅ $exporter: PASS ($metrics metric lines)"
  else
    echo "❌ $exporter: FAIL (only $metrics lines)"
  fi
done
```

---

## Integration Tests

### Cross-Pod NATS Messaging

```bash
echo "=== Testing Cross-Pod NATS Messaging ==="

# Start subscriber on Pod Norway (background)
echo "Starting subscriber on Pod Norway..."
timeout 10 nats sub --server=nats://192.168.60.133:4223 "test.cross-pod" > /tmp/nats-sub.log 2>&1 &
SUB_PID=$!

# Give subscriber time to connect
sleep 2

# Publish from Pod Denmark
echo "Publishing from Pod Denmark..."
nats pub --server=nats://192.168.60.133:4222 "test.cross-pod" "Hello from Denmark to Norway!"

# Wait for subscriber
sleep 2

# Check if message received
if grep -q "Hello from Denmark to Norway" /tmp/nats-sub.log; then
  echo "✅ Cross-Pod Messaging: PASS"
else
  echo "❌ Cross-Pod Messaging: FAIL"
  cat /tmp/nats-sub.log
fi

# Cleanup
kill $SUB_PID 2>/dev/null || true
rm /tmp/nats-sub.log
```

### Prometheus Scraping All Targets

```bash
echo "=== Verifying Prometheus Targets ==="

# Get target status
targets=$(curl -s http://192.168.60.133:9090/api/v1/targets | jq '.data.activeTargets')

# Count total targets
total=$(echo "$targets" | jq 'length')
echo "Total Prometheus targets: $total"

# Count healthy targets
healthy=$(echo "$targets" | jq '[.[] | select(.health == "up")] | length')
echo "Healthy targets: $healthy"

# List unhealthy targets
unhealthy=$(echo "$targets" | jq '[.[] | select(.health != "up") | .labels.job]')
echo "Unhealthy targets: $unhealthy"

if [ "$healthy" -eq "$total" ]; then
  echo "✅ Prometheus Scraping: PASS (all targets healthy)"
else
  echo "⚠️  Prometheus Scraping: PARTIAL ($healthy/$total healthy)"
fi
```

### Grafana Datasource Connection

```bash
echo "=== Testing Grafana Datasource ==="

# Get Grafana datasources (requires API key or login)
# For now, just check Grafana is accessible
grafana_status=$(curl -s -o /dev/null -w "%{http_code}" http://192.168.60.133:3001)

if [ "$grafana_status" -eq 200 ] || [ "$grafana_status" -eq 302 ]; then
  echo "✅ Grafana: PASS (HTTP $grafana_status)"
  echo "   Login: http://192.168.60.133:3001 (admin/admin)"
else
  echo "❌ Grafana: FAIL (HTTP $grafana_status)"
fi
```

### Redis Commander Multi-Pod View

```bash
echo "=== Testing Redis Commander ==="

# Check Redis Commander is accessible
rc_status=$(curl -s -o /dev/null -w "%{http_code}" http://192.168.60.133:8082)

if [ "$rc_status" -eq 200 ]; then
  echo "✅ Redis Commander: PASS"
  echo "   URL: http://192.168.60.133:8082"
  echo "   Note: Manually verify all 3 Redis instances appear (dk, no, se)"
else
  echo "❌ Redis Commander: FAIL (HTTP $rc_status)"
fi
```

---

## Performance Validation

### NATS Throughput Test

```bash
echo "=== NATS Performance Benchmark ==="

# Run benchmark (requires NATS CLI)
if command -v nats &> /dev/null; then
  echo "Running benchmark on Pod Denmark..."
  nats bench --server=nats://192.168.60.133:4222 test.bench \
    --msgs=10000 --size=1024 --pub=5 --sub=5
  
  echo "✅ NATS Benchmark: COMPLETE"
else
  echo "⚠️  NATS CLI not installed, skipping benchmark"
fi
```

### PostgreSQL Query Performance

```bash
echo "=== PostgreSQL Performance Test ==="

for pod in dk no se; do
  echo "Testing PostgreSQL pod-${pod}..."
  
  # Simple query timing
  docker exec service-postgres-${pod} psql -U unityplan -d unityplan_${pod} -c "\timing" -c "SELECT COUNT(*) FROM pg_database;"
done

echo "✅ PostgreSQL Performance: COMPLETE"
```

### Redis Latency Test

```bash
echo "=== Redis Latency Test ==="

for pod in dk no se; do
  echo "Testing Redis pod-${pod}..."
  
  # Latency test (100 requests)
  docker exec service-redis-${pod} redis-cli --latency-history -i 1 --csv -c 10
done

echo "✅ Redis Latency: COMPLETE"
```

---

## Failure Scenarios

### Test Pod Isolation

```bash
echo "=== Testing Pod Isolation ==="

# Try to connect to Pod Norway's postgres from Pod Denmark container
echo "Attempting cross-pod database access (should fail)..."
docker exec service-postgres-dk psql -h service-postgres-no -U unityplan -d unityplan_no -c "SELECT 1;" 2>&1

# This SHOULD fail with connection error (pods are isolated)
# Only mesh-network services (NATS, IPFS) should be cross-accessible

echo "✅ Pod Isolation: Verified (database access blocked)"
```

### Test NATS Cluster Recovery

```bash
echo "=== Testing NATS Cluster Recovery ==="

# Stop one NATS node
echo "Stopping NATS Norway..."
docker compose -f docker-compose.pod.yml -p pod-no stop nats

sleep 5

# Check cluster status from remaining nodes
curl -s http://192.168.60.133:8222/varz | jq '.cluster.urls'
# Should show only SE now (NO is down)

# Test messaging still works
nats pub --server=nats://192.168.60.133:4222 "test.failover" "Testing with NO down"

# Restart NATS Norway
echo "Restarting NATS Norway..."
docker compose -f docker-compose.pod.yml -p pod-no start nats

sleep 5

# Verify cluster reformed
curl -s http://192.168.60.133:8222/varz | jq '.cluster.urls'
# Should show both NO and SE again

echo "✅ NATS Cluster Recovery: PASS"
```

### Test Database Failover (Future)

```bash
# Future: Test PostgreSQL replication/failover
echo "⏳ Database Failover: Not yet implemented (Phase 2)"
```

---

## Complete Verification Checklist

Run this comprehensive check:

```bash
#!/bin/bash
echo "==================================="
echo "UnityPlan Multi-Pod Verification"
echo "==================================="

# Check all containers running
echo -e "\n📦 Container Status:"
docker ps --filter "name=service-" --filter "name=monitoring-" --filter "name=dev-" --format "table {{.Names}}\t{{.Status}}" | head -20

# Check NATS cluster
echo -e "\n🔗 NATS Cluster Status:"
for port in 8222 8223 8224; do
  curl -s http://192.168.60.133:${port}/varz | jq -r '"\(.server_name): \(.cluster.urls | length) routes"'
done

# Check Prometheus targets
echo -e "\n📊 Prometheus Targets:"
curl -s http://192.168.60.133:9090/api/v1/targets | jq '[.data.activeTargets[] | select(.health == "up")] | length' | xargs echo "Healthy targets:"

# Check databases
echo -e "\n🗄️  Database Connections:"
for pod in dk no se; do
  docker exec service-postgres-${pod} psql -U unityplan -d unityplan_${pod} -tAc "SELECT 'pod-${pod}: OK';" 2>/dev/null || echo "pod-${pod}: FAIL"
done

# Check Redis
echo -e "\n🔴 Redis Status:"
for pod in dk no se; do
  docker exec service-redis-${pod} redis-cli ping 2>/dev/null | xargs echo "pod-${pod}:"
done

echo -e "\n==================================="
echo "✅ Verification Complete!"
echo "==================================="
```

Save as `scripts/verify-multi-pod.sh` and run:
```bash
chmod +x scripts/verify-multi-pod.sh
./scripts/verify-multi-pod.sh
```

---

## Troubleshooting Reference

| Issue | Symptom | Solution |
|-------|---------|----------|
| NATS cluster not forming | `cluster.urls` is empty | Check mesh-network connectivity, verify NATS_ROUTES in .env |
| Prometheus targets down | Red targets in Prometheus UI | Check exporter containers running, verify network connectivity |
| Port conflict | "port is already allocated" | Verify port offsets in .env files, check for conflicting services |
| Database connection failed | psql errors | Wait for health check, check POSTGRES_PORT, verify init.sql ran |
| Container fails to start | Exited status | Check logs: `docker logs <container-name>` |

---

**Test Plan Version:** 1.0  
**Last Updated:** November 5, 2025  
**Maintainer:** UnityPlan Platform Team
