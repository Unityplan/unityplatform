# Europe Multi-Territory Pod

**Pod ID:** `eu`  
**Territories:** Germany (DE), France (FR), Spain (ES)  
**Model:** Multi-Territory Shared Infrastructure

---

## 📋 Overview

The Europe pod demonstrates UnityPlan's **multi-territory deployment model**, where multiple smaller territories share infrastructure while maintaining complete data isolation.

### Architecture

```
Pod Europe (Single VPS/Instance)
├── PostgreSQL Instance
│   ├── unityplan_de (Germany database)
│   │   ├── global schema (replicated)
│   │   └── territory_DE schema (isolated)
│   ├── unityplan_fr (France database)
│   │   ├── global schema (replicated)
│   │   └── territory_FR schema (isolated)
│   └── unityplan_es (Spain database)
│       ├── global schema (replicated)
│       └── territory_ES schema (isolated)
├── Redis (Shared)
│   ├── de:* (Germany keys)
│   ├── fr:* (France keys)
│   └── es:* (Spain keys)
├── NATS (Shared)
│   ├── territory.de.* (Germany topics)
│   ├── territory.fr.* (France topics)
│   └── territory.es.* (Spain topics)
├── IPFS (Shared, content-addressed)
└── Matrix Synapse (Federated)
```

---

## 🚀 Deployment

### Quick Start

```bash
# 1. Ensure mesh network exists
docker network create unityplan-mesh-network

# 2. Deploy Europe pod
docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu \
  --env-file pods/europe/.env up -d

# 3. Verify deployment
docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu ps
```

### What Gets Created

**Containers:**
- `service-postgres-eu` - PostgreSQL with 3 databases
- `service-redis-eu` - Shared Redis cache
- `service-nats-eu` - NATS cluster node
- `service-ipfs-eu` - IPFS node
- `service-matrix-eu` - Matrix homeserver
- `monitoring-postgres-exporter-eu`
- `monitoring-redis-exporter-eu`
- `monitoring-nats-exporter-eu`
- `monitoring-node-exporter-eu`
- `monitoring-cadvisor-eu`

**Volumes:**
- `eu-postgres-data` - All 3 territory databases
- `eu-redis-data`
- `eu-nats-data`
- `eu-ipfs-data`
- `eu-matrix-data`

**Ports (Host:Container):**
- PostgreSQL: `5435:5432`
- Redis: `6382:6379`
- NATS Client: `4225:4222`
- NATS Cluster: `6225:6222`
- NATS Monitor: `8225:8222`
- IPFS API: `5004:5001`
- IPFS Gateway: `8084:8080`
- Matrix: `8011:8008`

---

## 🗄️ Database Structure

### Databases Created

```sql
-- Germany
unityplan_de
├── global (replicated data)
│   └── territories (all territories info)
└── territory_DE (Germany-specific data)
    ├── users
    ├── communities
    └── posts

-- France
unityplan_fr
├── global (replicated data)
└── territory_FR (France-specific data)
    ├── users
    ├── communities
    └── posts

-- Spain
unityplan_es
├── global (replicated data)
└── territory_ES (Spain-specific data)
    ├── users
    ├── communities
    └── posts

-- Metadata (cross-territory queries)
unityplan_eu_meta
└── meta
    └── pod_info (pod metadata)
```

### Connecting to Databases

```bash
# Germany database
docker exec -it service-postgres-eu psql -U unityplan -d unityplan_de

# France database
docker exec -it service-postgres-eu psql -U unityplan -d unityplan_fr

# Spain database
docker exec -it service-postgres-eu psql -U unityplan -d unityplan_es

# Metadata database
docker exec -it service-postgres-eu psql -U unityplan -d unityplan_eu_meta

# List all databases
docker exec service-postgres-eu psql -U unityplan -c "\l"
```

### Sample Queries

```sql
-- Germany users
SELECT * FROM territory_DE.users LIMIT 10;

-- France communities
SELECT * FROM territory_FR.communities;

-- Check schema isolation
\dn  -- List schemas in current database
```

---

## 🔴 Redis Key Namespacing

Keys are prefixed by territory code to prevent collisions:

```bash
# Germany keys
de:user:123
de:session:abc
de:cache:homepage

# France keys
fr:user:456
fr:session:def
fr:cache:homepage

# Spain keys
es:user:789
es:session:ghi
es:cache:homepage
```

**Access via Redis CLI:**

```bash
# Connect to Redis
docker exec -it service-redis-eu redis-cli -a redis_dev_password

# Get Germany keys
KEYS de:*

# Get France session
GET fr:session:def

# Get all sessions across territories
KEYS *:session:*
```

---

## 📡 NATS Topic Routing

Messages are routed by territory via topic names:

```bash
# Germany topics
territory.de.users.created
territory.de.posts.published
territory.de.community.joined

# France topics
territory.fr.users.created
territory.fr.posts.published
territory.fr.community.joined

# Spain topics
territory.es.users.created
territory.es.posts.published
territory.es.community.joined

# Global topics (all territories)
global.system.announcement
```

**Publishing/Subscribing:**

```bash
# Subscribe to Germany events
nats sub --server=nats://192.168.60.133:4225 "territory.de.*"

# Publish to France
nats pub --server=nats://192.168.60.133:4225 "territory.fr.posts.published" '{"id":123}'

# Subscribe to all EU territories
nats sub --server=nats://192.168.60.133:4225 "territory.>"
```

---

## ✅ Verification

### Health Checks

```bash
# Check all containers running
docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu ps

# PostgreSQL health
docker exec service-postgres-eu pg_isready -U unityplan

# Redis health
docker exec service-redis-eu redis-cli -a redis_dev_password ping

# NATS health
curl http://192.168.60.133:8225/varz | jq '.server_name'

# Check NATS cluster membership
curl http://192.168.60.133:8225/varz | jq '.cluster'
```

### Database Verification

```bash
# Verify all 4 databases created
docker exec service-postgres-eu psql -U unityplan -c "\l" | grep unityplan_

# Expected output:
# unityplan_de
# unityplan_fr
# unityplan_es
# unityplan_eu_meta

# Check Germany schema
docker exec service-postgres-eu psql -U unityplan -d unityplan_de -c "\dn"

# Verify territory data
docker exec service-postgres-eu psql -U unityplan -d unityplan_de -c "SELECT * FROM global.territories WHERE id IN ('DE','FR','ES');"
```

### Monitoring

```bash
# Prometheus metrics
curl http://192.168.60.133:9190/metrics | grep postgres_  # PostgreSQL
curl http://192.168.60.133:9124/metrics | grep redis_     # Redis
curl http://192.168.60.133:7780/metrics | grep gnatsd_    # NATS

# Check metrics in Grafana
# URL: http://192.168.60.133:3001
# Look for pod=europe label in dashboards
```

---

## 🔧 Configuration

### Environment Variables

See `pods/europe/.env` for full configuration. Key settings:

```bash
# Pod identification
POD_ID=eu
TERRITORY_CODES=DE,FR,ES

# Port offsets (+3 from Denmark base)
POSTGRES_PORT=5435
REDIS_PORT=6382
NATS_CLIENT_PORT=4225

# Territory-specific configs
TERRITORY_DE_TIMEZONE=Europe/Berlin
TERRITORY_FR_TIMEZONE=Europe/Paris
TERRITORY_ES_TIMEZONE=Europe/Madrid
```

### Customization

**Add a new territory (e.g., Italy):**

1. Update `.env`:
   ```bash
   TERRITORY_CODES=DE,FR,ES,IT
   POSTGRES_DB_IT=unityplan_it
   TERRITORY_IT_TIMEZONE=Europe/Rome
   TERRITORY_IT_LOCALE=it_IT.UTF-8
   TERRITORY_IT_LANGUAGE=it
   ```

2. Update `init-multi-territory.sh`:
   ```bash
   create_territory_schema "it" "Italy" "unityplan_it"
   ```

3. Recreate pod:
   ```bash
   docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu down
   docker volume rm eu-postgres-data
   docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu up -d
   ```

---

## 🔄 Migration to Dedicated Pods

If a territory outgrows the shared pod, migrate to dedicated infrastructure:

### Example: Migrate Germany to Dedicated Pod

```bash
# 1. Dump Germany database
docker exec service-postgres-eu pg_dump -U unityplan unityplan_de > germany_backup.sql

# 2. Create dedicated Germany pod
cp pods/europe/.env pods/germany/.env
# Edit pods/germany/.env: POD_ID=de, remove FR/ES configs

# 3. Deploy dedicated pod
docker compose -f docker-compose.pod.yml -p pod-de --env-file pods/germany/.env up -d

# 4. Restore database
cat germany_backup.sql | docker exec -i service-postgres-de psql -U unityplan -d unityplan_de

# 5. Update routing (NATS, DNS, load balancer)
# Point DE users to pod-de instead of pod-eu

# 6. Remove DE from Europe pod
# Update pods/europe/.env, remove DE config
# Drop unityplan_de database from pod-eu
```

---

## 📊 Cost Analysis

**Shared Pod (Current):**
- 1 VPS: ~€30/month
- Serves 3 territories
- Cost per territory: ~€10/month

**Dedicated Pods (Alternative):**
- 3 VPS: 3 × €30 = €90/month
- Cost per territory: €30/month

**Savings:** 67% with shared infrastructure

**When to Split:**
- Combined traffic > 80% CPU/Memory
- One territory needs more resources
- Regulatory requirements demand separation
- Territory wants dedicated infrastructure

---

## 🐛 Troubleshooting

### Database Connection Issues

```bash
# Check PostgreSQL logs
docker logs service-postgres-eu

# Verify init script ran
docker exec service-postgres-eu psql -U unityplan -c "\l" | grep unityplan

# Re-run init script manually
docker exec -i service-postgres-eu bash < pods/europe/init-multi-territory.sh
```

### Redis Key Conflicts

```bash
# Check for un-prefixed keys
docker exec service-redis-eu redis-cli -a redis_dev_password KEYS '*' | grep -v ':'

# Fix: Always use territory prefix
# BAD:  user:123
# GOOD: de:user:123
```

### NATS Topic Isolation

```bash
# Verify territory topics exist
docker exec service-nats-eu nats stream ls

# Create territory streams if missing
nats stream add TERRITORY_DE --subjects="territory.de.*" --replicas=1
nats stream add TERRITORY_FR --subjects="territory.fr.*" --replicas=1
nats stream add TERRITORY_ES --subjects="territory.es.*" --replicas=1
```

---

## 📚 Related Documentation

- [Multi-Pod Architecture](../project_docs/5-multi-pod-architecture.md) - Pod deployment models
- [Multi-Pod Deployment Guide](../project_docs/6-multi-pod-deployment-guide.md) - Full deployment guide
- [Territory Management Standard](../project_docs/9-territory-management-standard.md) - Territory ID format

---

**Pod Status:** Ready for Testing  
**Last Updated:** November 5, 2025  
**Maintainer:** UnityPlan Platform Team
