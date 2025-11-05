# Migration Guide: Old to New Architecture

**Date:** November 5, 2025  
**Status:** Completed

---

## What Changed

### Old Architecture (Monolithic)

**Single file:** `docker-compose.yml` (now archived as `docker-compose.monolith.yml.old`)

All services in one file:
- PostgreSQL, Redis, NATS
- IPFS, Matrix
- Prometheus, Grafana, Jaeger
- Adminer, MailHog, Redis Commander
- Traefik, landing page, error pages
- All exporters

**Problems:**
- ❌ Everything starts together (slow, resource-heavy)
- ❌ Can't scale to multiple territories
- ❌ No separation between dev/prod
- ❌ One database for all territories (not isolated)

### New Architecture (Modular)

**Split into 4 compose files:**

1. **`docker-compose.dev.yml`** - Development tools only
   - Adminer, MailHog, Redis Commander, Dev Dashboard
   - **Forgejo** (new: version control + MCP)
   - **Docker Registry** (new: local image storage)

2. **`docker-compose.monitoring.yml`** - Monitoring stack
   - Prometheus, Grafana, Jaeger
   - Global monitoring for all pods

3. **`docker-compose.pod.yml`** - Single-territory pod template
   - PostgreSQL, Redis, NATS, IPFS, Matrix
   - All exporters
   - Used for: Denmark (DK), Norway (NO), Sweden (SE)

4. **`docker-compose.multi-territory-pod.yml`** - Multi-territory pod
   - Same as pod.yml but supports multiple databases
   - Used for: Europe (EU) - Germany, France, Spain

**Benefits:**
- ✅ Start only what you need (Phase 1: just Forgejo + Registry)
- ✅ Per-territory isolation (DK, NO, SE, EU have separate pods)
- ✅ Clear separation (dev tools vs monitoring vs production)
- ✅ Easy to scale (add new pod = add new territory)

---

## Migration Steps

### 1. Stop Old Architecture

```bash
# Stop all containers from old docker-compose.yml
docker compose down

# Containers stopped:
# - service-postgres, service-redis, service-nats
# - service-ipfs, service-matrix
# - monitoring-prometheus, monitoring-grafana, monitoring-jaeger
# - dev-adminer, dev-mailhog, dev-redis-commander
# - reverse-proxy-traefik, reverse-proxy-landing, reverse-proxy-error-pages
# - All exporters
```

**Status:** ✅ Completed (November 5, 2025)

### 2. Archive Old Files

```bash
# Old docker-compose.yml renamed to:
docker-compose.monolith.yml.old

# Old scripts moved to scripts/old/:
scripts/old/start-dev.sh
scripts/old/stop-dev.sh
scripts/old/setup-dev.sh
```

**Status:** ✅ Completed

### 3. Data Migration

**Good news:** Data volumes are preserved!

Old volumes:
```
./docker/postgres-data     → Preserved
./docker/redis-data        → Preserved
./docker/nats-data         → Preserved
./docker/ipfs-data         → Preserved
./docker/matrix-data       → Preserved
./docker/prometheus-data   → Preserved
./docker/grafana-data      → Preserved
```

**For Phase 1:** You don't need this old data (starting fresh with Forgejo).

**For multi-pod deployment:** We'll create new per-pod volumes:
- `pod-dk-postgres-data`
- `pod-dk-redis-data`
- `pod-dk-nats-data`
- etc.

### 4. Start New Architecture

#### Phase 1 (Minimal Development)

```bash
# Start Forgejo + Registry only
./scripts/start-new-architecture.sh --phase1

# Or use dedicated script:
./scripts/start-phase1-dev.sh

# What runs:
# - dev-forgejo (port 3000)
# - dev-registry (port 5000)
```

#### Phase 1 + Development Tools

```bash
# Start all dev tools (optional)
./scripts/start-new-architecture.sh --dev-tools

# What runs:
# - Forgejo, Registry (as above)
# - Adminer, MailHog, Redis Commander, Dev Dashboard
```

#### Phase 2 (Multi-Pod Production)

```bash
# Start a specific pod
./scripts/start-new-architecture.sh --pod dk

# Start all pods
./scripts/start-new-architecture.sh --all-pods

# Start everything
./scripts/start-new-architecture.sh --full
```

---

## Port Changes

### Old Architecture (Monolithic)

| Service | Port | Notes |
|---------|------|-------|
| PostgreSQL | 5432 | Single database |
| Redis | 6379 | Single instance |
| NATS | 4222 | Single node |
| Prometheus | 9090 | Hardcoded |
| Grafana | 3001 | Hardcoded |

### New Architecture (Per-Pod)

**Denmark (DK) - Base Ports:**
| Service | Port | Container |
|---------|------|-----------|
| PostgreSQL | 5432 | service-postgres-dk |
| Redis | 6379 | service-redis-dk |
| NATS Client | 4222 | service-nats-dk |
| NATS Cluster | 6222 | (internal) |
| NATS Monitor | 8222 | service-nats-dk |

**Norway (NO) - +1 Offset:**
| Service | Port | Container |
|---------|------|-----------|
| PostgreSQL | 5433 | service-postgres-no |
| Redis | 6380 | service-redis-no |
| NATS Client | 4223 | service-nats-no |
| NATS Cluster | 6223 | (internal) |
| NATS Monitor | 8223 | service-nats-no |

**Sweden (SE) - +2 Offset:**
| Service | Port | Container |
|---------|------|-----------|
| PostgreSQL | 5434 | service-postgres-se |
| Redis | 6381 | service-redis-se |
| NATS Client | 4224 | service-nats-se |

**Europe (EU) - +3 Offset (Multi-Territory):**
| Service | Port | Container |
|---------|------|-----------|
| PostgreSQL | 5435 | service-postgres-eu |
| Redis | 6382 | service-redis-eu |
| NATS Client | 4225 | service-nats-eu |

**Global Services (Unchanged):**
| Service | Port | Container |
|---------|------|-----------|
| Prometheus | 9090 | monitoring-prometheus |
| Grafana | 3001 | monitoring-grafana |
| Jaeger | 16686 | monitoring-jaeger |
| Forgejo | 3000 | dev-forgejo |
| Registry | 5000 | dev-registry |

---

## Environment Variables

### Old Architecture

**Single `.env` file** for everything:
```bash
POSTGRES_USER=unityplan
POSTGRES_PASSWORD=unityplan_dev_password
POSTGRES_DB=unityplan_dev
```

### New Architecture

**Per-pod `.env` files:**

```bash
pods/denmark/.env          # DK configuration
pods/norway/.env           # NO configuration
pods/sweden/.env           # SE configuration
pods/europe/.env           # EU configuration (multi-territory)
pods/europe/.env.staging   # EU staging environment
```

**Example:** `pods/denmark/.env`
```bash
POD_ID=dk
TERRITORY_ID=DK
TERRITORY_NAME=Denmark

POSTGRES_PORT=5432
POSTGRES_DB=unityplan_dk
POSTGRES_USER=unityplan
POSTGRES_PASSWORD=${POSTGRES_PASSWORD}

REDIS_PORT=6379
NATS_CLIENT_PORT=4222
NATS_CLUSTER_PORT=6222
NATS_MONITOR_PORT=8222
```

---

## Script Changes

### Old Scripts (Archived)

❌ **`scripts/old/start-dev.sh`**
- Used old `docker-compose.yml`
- Started everything together
- No pod support

❌ **`scripts/old/stop-dev.sh`**
- Used old `docker-compose.yml`
- Stopped everything together

❌ **`scripts/old/setup-dev.sh`**
- Created directories for monolithic setup
- Not needed with new architecture

### New Scripts (Active)

✅ **`scripts/start-new-architecture.sh`**
- Flexible: start phase1, dev-tools, monitoring, specific pod, or all
- Options: `--phase1`, `--dev-tools`, `--monitoring`, `--pod <id>`, `--all-pods`, `--full`

✅ **`scripts/stop-new-architecture.sh`**
- Stop specific components or everything
- Options: `--dev-tools`, `--monitoring`, `--pod <id>`, `--all-pods`, `--all`, `--clean`

✅ **`scripts/start-phase1-dev.sh`**
- Phase 1 specific: starts Forgejo + Registry only
- Provides next steps and documentation links

✅ **`scripts/deploy-multi-pod.sh`**
- Deploy all production pods (DK, NO, SE, EU)
- Creates mesh network
- Verifies connectivity

✅ **`scripts/verify-multi-pod.sh`**
- Health checks for all pods
- NATS cluster verification
- Exporters check

---

## Workflow Comparison

### Old Workflow (Monolithic)

```bash
# Start everything
./scripts/start-dev.sh

# Wait for all services (slow)

# Use Adminer to access database
# Build Rust backend
# Test locally

# Stop everything
./scripts/stop-dev.sh
```

### New Workflow (Phase 1)

```bash
# Start minimal environment
./scripts/start-new-architecture.sh --phase1

# Configure Forgejo
# Install forgejo-mcp

# Build Rust backend
cd services
cargo build --release

# Build Docker image
docker build -t localhost:5000/unityplan/auth-service:latest .

# Push to local registry
docker push localhost:5000/unityplan/auth-service:latest

# Test locally (without pods running)
cargo test

# Commit to Forgejo
git push forgejo main

# Stop when done
./scripts/stop-new-architecture.sh --dev-tools
```

### New Workflow (Multi-Pod)

```bash
# Start development tools + monitoring
./scripts/start-new-architecture.sh --dev-tools --monitoring

# Deploy Denmark pod
./scripts/start-new-architecture.sh --pod dk

# Build and deploy service
docker build -t localhost:5000/unityplan/auth-service:latest .
docker push localhost:5000/unityplan/auth-service:latest

# Update pod
docker compose -f docker-compose.pod.yml -p pod-dk pull auth
docker compose -f docker-compose.pod.yml -p pod-dk up -d --no-deps auth

# Verify
curl http://localhost:8080/api/auth/health

# Deploy to more pods
./scripts/start-new-architecture.sh --pod no
./scripts/start-new-architecture.sh --pod se

# Verify multi-pod setup
./scripts/verify-multi-pod.sh
```

---

## Troubleshooting

### "Port already in use"

**Old architecture containers still running?**
```bash
# List all containers
docker ps -a

# Remove old containers
docker rm -f $(docker ps -aq)

# Or stop old architecture specifically
docker compose -f docker-compose.monolith.yml.old down
```

### "Cannot connect to Forgejo"

**Service starting slowly?**
```bash
# Check logs
docker logs dev-forgejo

# Wait a bit longer (Forgejo takes 30-60 seconds)
sleep 30

# Try again
curl http://localhost:3000
```

### "Old data volumes interfering"

**Want fresh start?**
```bash
# Stop everything
./scripts/stop-new-architecture.sh --all

# Remove old volumes
docker volume prune

# Remove specific volumes
docker volume rm unityplan_postgres-data
docker volume rm unityplan_redis-data

# Start fresh
./scripts/start-new-architecture.sh --phase1
```

### "Missing .env files"

**Pod environments not created?**
```bash
# Check if pod env files exist
ls pods/*/env

# If missing, see pod README files:
cat pods/denmark/README.md
cat pods/europe/README.md
```

---

## Summary

### What Was Removed

- ❌ `docker-compose.yml` (monolithic) → archived
- ❌ `scripts/start-dev.sh` → replaced by `start-new-architecture.sh`
- ❌ `scripts/stop-dev.sh` → replaced by `stop-new-architecture.sh`
- ❌ `scripts/setup-dev.sh` → not needed

### What Was Added

- ✅ `docker-compose.dev.yml` (dev tools)
- ✅ `docker-compose.monitoring.yml` (monitoring)
- ✅ `docker-compose.pod.yml` (single-territory)
- ✅ `docker-compose.multi-territory-pod.yml` (multi-territory)
- ✅ `scripts/start-new-architecture.sh`
- ✅ `scripts/stop-new-architecture.sh`
- ✅ `scripts/start-phase1-dev.sh`
- ✅ `scripts/deploy-multi-pod.sh`
- ✅ `scripts/verify-multi-pod.sh`
- ✅ Pod configurations (DK, NO, SE, EU)
- ✅ Forgejo + MCP integration
- ✅ Docker Registry

### Benefits Achieved

✅ **Faster development** - Start only what you need  
✅ **Multi-territory support** - Per-pod isolation  
✅ **Clear separation** - Dev vs Monitoring vs Production  
✅ **Scalable** - Add territories without disruption  
✅ **AI-assisted** - Forgejo MCP integration  
✅ **Production-ready** - Same workflow dev → staging → prod

---

**Migration Status:** ✅ Complete  
**Date:** November 5, 2025  
**Next Steps:** Start building Rust backend services for Phase 1
