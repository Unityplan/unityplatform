# Multi-Pod Deployment - Quick Start

Complete multi-pod architecture setup for UnityPlan platform.

## 📋 Overview

This deployment creates 3 territory pods (Denmark, Norway, Sweden) with:

- Full service isolation per territory
- NATS clustering for cross-pod messaging
- Unified monitoring via Prometheus & Grafana
- Shared development tools

## 🚀 Quick Deploy

```bash
# Deploy everything
./scripts/deploy-multi-pod.sh

# Or clean deploy (removes existing setup)
./scripts/deploy-multi-pod.sh --clean

# Verify deployment
./scripts/verify-multi-pod.sh
```

## 📦 What Gets Deployed

### Project Naming Convention

All pods use the naming pattern: `unityplan-pod-${POD_ID}` where POD_ID is the territory code (dk, no, se, eu).

This is defined in `docker-compose.pod.yml`:

```yaml
name: unityplan-pod-${POD_ID}
```

**Example stacks:**

- `unityplan-pod-dk` (Denmark)
- `unityplan-pod-no` (Norway)
- `unityplan-pod-se` (Sweden)
- `unityplan-pod-eu` (Europe multi-territory)

```

## 📦 What Gets Deployed

### Global Services (Single Instance)
- **Development Stack** (docker-compose.dev.yml)
  - Dev Dashboard (port 8888)
  - Adminer (port 8080)
  - MailHog (port 8025)
  - Redis Commander (port 8082)

- **Monitoring Stack** (docker-compose.monitoring.yml)
  - Prometheus (port 9090)
  - Grafana (port 3001) - admin/admin
  - Jaeger (port 16686)
  - Traefik (ports 80, 443, 8083)

### Territory Pods (One Per Country)
Each pod (docker-compose.pod.yml) contains:
- PostgreSQL database
- Redis cache
- NATS messaging (clustered)
- IPFS node
- Matrix Synapse
- Exporters (postgres, redis, nats, node, cadvisor)

**Pod Denmark (DK)**
- PostgreSQL: 5432
- Redis: 6379
- NATS: 4222, 6222, 8222

**Pod Norway (NO)**
- PostgreSQL: 5433
- Redis: 6380
- NATS: 4223, 6223, 8223

**Pod Sweden (SE)**
- PostgreSQL: 5434
- Redis: 6381
- NATS: 4224, 6224, 8224

**Pod Europe (EU)** - Multi-Territory Pod
- PostgreSQL: 5435 (hosts DE, FR, ES databases)
- Redis: 6382 (shared with key prefixing)
- NATS: 4225, 6225, 8225
- Territories: Germany, France, Spain

## 🌍 Multi-Territory Pods

UnityPlan supports **multi-territory pods** where multiple small territories share infrastructure:

### Example: Europe Pod (Germany, France, Spain)

```bash
# Deploy Europe multi-territory pod
docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu \
  --env-file pods/europe/.env up -d

# This creates:
# - 3 separate PostgreSQL databases (unityplan_de, unityplan_fr, unityplan_es)
# - Shared Redis with key prefixing (de:*, fr:*, es:*)
# - Shared NATS topics (territory.de.*, territory.fr.*, territory.es.*)
# - Shared IPFS node (content-addressed, naturally deduplicated)
```

### Benefits of Multi-Territory Pods

- **Cost Efficient**: Small territories share infrastructure costs
- **Low Latency**: Geographically close countries on same pod
- **Data Isolation**: Separate databases, key namespaces, topic routing
- **Easy Split**: Can migrate to dedicated pods when traffic grows

### When to Use Multi-Territory Pods

| Use Multi-Territory | Use Single-Territory |
|---------------------|----------------------|
| < 500K users combined | 500K+ users |
| Same geographic region | Any location |
| Cost optimization priority | Performance/isolation priority |
| Starting small territories | Major countries/regions |

## 🔧 Manual Deployment

```bash
# 1. Create mesh network
docker network create unityplan-mesh-network

# 2. Start development tools
docker compose -f docker-compose.dev.yml up -d

# 3. Start monitoring
docker compose -f docker-compose.monitoring.yml up -d

# 4. Start territory pods (using env files, project name comes from compose file)
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env up -d
docker compose -f docker-compose.pod.yml --env-file pods/norway/.env up -d
docker compose -f docker-compose.pod.yml --env-file pods/sweden/.env up -d

# 5. (Optional) Start multi-territory pod
docker compose -f docker-compose.multi-territory-pod.yml --env-file pods/europe/.env up -d
```

## ✅ Verification

```bash
# Run full verification suite
./scripts/verify-multi-pod.sh

# Check NATS cluster manually
curl http://192.168.60.133:8222/varz | jq '.cluster'

# Check Prometheus targets
curl http://192.168.60.133:9090/api/v1/targets | jq '.data.activeTargets[] | select(.health != "up")'

# Test database connectivity
docker exec service-postgres-dk psql -U unityplan -d unityplan_dk -c "SELECT 1;"
docker exec service-postgres-no psql -U unityplan -d unityplan_no -c "SELECT 1;"
docker exec service-postgres-se psql -U unityplan -d unityplan_se -c "SELECT 1;"

# Test multi-territory pod (if deployed)
docker exec service-postgres-eu psql -U unityplan -d unityplan_de -c "SELECT 1;"  # Germany
docker exec service-postgres-eu psql -U unityplan -d unityplan_fr -c "SELECT 1;"  # France
docker exec service-postgres-eu psql -U unityplan -d unityplan_es -c "SELECT 1;"  # Spain

# List all databases in Europe pod
docker exec service-postgres-eu psql -U unityplan -c "\l"
```

## 🧪 Testing Cross-Pod Messaging

```bash
# Install NATS CLI if not already installed
curl -sf https://binaries.nats.dev/nats-io/natscli/nats@latest | sh

# Add NATS contexts
nats context add dk --server=nats://192.168.60.133:4222
nats context add no --server=nats://192.168.60.133:4223
nats context add se --server=nats://192.168.60.133:4224

# Test messaging across pods
# Terminal 1: Subscribe on Norway
nats context select no
nats sub "test.cross-pod"

# Terminal 2: Publish from Denmark
nats context select dk
nats pub "test.cross-pod" "Hello from Denmark!"

# Message should appear in Terminal 1
```

## 🛑 Shutdown

```bash
# Stop all pods (using env files to get correct project names)
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down
docker compose -f docker-compose.pod.yml --env-file pods/norway/.env down
docker compose -f docker-compose.pod.yml --env-file pods/sweden/.env down

# Stop monitoring and dev
docker compose -f docker-compose.monitoring.yml down
docker compose -f docker-compose.dev.yml down

# Remove mesh network
docker network rm unityplan-mesh-network
```

## 📊 Access Points

| Service | URL | Credentials |
|---------|-----|-------------|
| Grafana | <http://192.168.60.133:3001> | admin / admin |
| Prometheus | <http://192.168.60.133:9090> | - |
| Jaeger | <http://192.168.60.133:16686> | - |
| Dev Dashboard | <http://192.168.60.133:8888> | - |
| Adminer | <http://192.168.60.133:8080> | - |
| MailHog | <http://192.168.60.133:8025> | - |
| Redis Commander | <http://192.168.60.133:8082> | - |
| NATS Monitor DK | <http://192.168.60.133:8222> | - |
| NATS Monitor NO | <http://192.168.60.133:8223> | - |
| NATS Monitor SE | <http://192.168.60.133:8224> | - |

## 📚 Documentation

- **Comprehensive Guide**: [project_docs/6-multi-pod-deployment-guide.md](../project_docs/6-multi-pod-deployment-guide.md)
- **Architecture**: [project_docs/5-multi-pod-architecture.md](../project_docs/5-multi-pod-architecture.md)
- **NATS Clustering**: [project_docs/7-nats-clustering-guide.md](../project_docs/7-nats-clustering-guide.md)
- **Testing Guide**: [project_docs/8-testing-verification-guide.md](../project_docs/8-testing-verification-guide.md)

## 🐛 Troubleshooting

**NATS cluster not forming:**

```bash
# Check NATS logs
docker logs service-nats-dk | grep cluster
docker logs service-nats-no | grep cluster

# Verify mesh network connectivity
docker exec service-nats-dk ping service-nats-no
```

**Prometheus targets down:**

```bash
# Check exporter logs
docker logs monitoring-postgres-exporter-dk
docker logs monitoring-redis-exporter-dk

# Verify exporter metrics endpoints
curl http://192.168.60.133:9187/metrics | head
curl http://192.168.60.133:9121/metrics | head
```

**Port conflicts:**

```bash
# Check what's using a port
sudo lsof -i :5432
sudo netstat -tulpn | grep 5432

# Verify .env port configurations
cat pods/denmark/.env | grep PORT
cat pods/norway/.env | grep PORT
```

## 🔮 Next Steps

1. Test multi-pod deployment locally
2. Implement Rust backend services (Phase 1.3)
3. Test cross-pod API calls
4. Prepare for geographic distribution (Phase 2)
5. Configure WireGuard VPN for multi-host deployment

---

**Version:** 1.0  
**Last Updated:** November 5, 2025  
**Status:** Ready for testing
