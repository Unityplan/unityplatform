# UnityPlan Scripts

## Active Scripts (New Architecture)

### Development

**`start-dev.sh`** - Start Phase 1 development environment
- Starts: Forgejo + Docker Registry
- Purpose: Minimal setup for MVP development
- Usage: `./scripts/start-dev.sh`
- Help: `./scripts/start-dev.sh --help`

**`start-architecture.sh`** - Flexible startup with options
- Starts: Specific components or full stack
- Purpose: Development, monitoring, pods, or everything
- Usage: `./scripts/start-architecture.sh [OPTIONS]`
- Help: `./scripts/start-architecture.sh --help`
- Examples:
  - `./scripts/start-architecture.sh --phase1` (same as start-dev.sh)
  - `./scripts/start-architecture.sh --dev-tools`
  - `./scripts/start-architecture.sh --monitoring`
  - `./scripts/start-architecture.sh --pod dk`
  - `./scripts/start-architecture.sh --all-pods`
  - `./scripts/start-architecture.sh --full`

**`stop-architecture.sh`** - Stop services
- Stops: Specific components or everything
- Purpose: Clean shutdown with optional data removal
- Usage: `./scripts/stop-architecture.sh [OPTIONS]`
- Help: `./scripts/stop-architecture.sh --help`
- Examples:
  - `./scripts/stop-architecture.sh --dev-tools`
  - `./scripts/stop-architecture.sh --pod dk`
  - `./scripts/stop-architecture.sh --all`
  - `./scripts/stop-architecture.sh --all --clean` (⚠️ deletes data!)

### Multi-Pod Deployment

**`deploy-multi-pod.sh`** - Deploy all production pods
- Deploys: Denmark (DK), Norway (NO), Sweden (SE), Europe (EU) pods
- Purpose: Multi-pod production deployment
- Usage: `./scripts/deploy-multi-pod.sh [--clean]`
- Help: `./scripts/deploy-multi-pod.sh --help`

**`verify-multi-pod.sh`** - Verify multi-pod deployment
- Checks: All pod services, health, connectivity
- Purpose: Validate multi-pod setup
- Usage: `./scripts/verify-multi-pod.sh`
- Help: `./scripts/verify-multi-pod.sh --help`

---

## Docker Compose Files

### Phase 1 (Development)

```bash
# Start Forgejo + Registry (minimal Phase 1 setup)
docker compose -f docker-compose.dev.yml up -d forgejo registry

# Start all dev tools (optional)
docker compose -f docker-compose.dev.yml up -d
# Includes: Adminer, MailHog, Redis Commander, Dev Dashboard
```

### Monitoring (Global)

```bash
# Start monitoring stack
docker compose -f docker-compose.monitoring.yml up -d
# Includes: Prometheus, Grafana, Jaeger
```

### Single-Territory Pod

```bash
# Deploy Denmark pod
docker compose -f docker-compose.pod.yml -p pod-dk \
  --env-file pods/denmark/.env up -d
```

### Multi-Territory Pod

```bash
# Deploy Europe pod (Germany, France, Spain)
docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu \
  --env-file pods/europe/.env up -d
```

---

## Archived Scripts

**`old/start-dev.sh`** - Old monolithic startup (replaced by start-phase1-dev.sh)  
**`old/stop-dev.sh`** - Old monolithic stop (use `docker compose down` now)  
**`old/setup-dev.sh`** - Old setup script (not needed with new architecture)

These scripts referenced the old `docker-compose.yml` (now archived as `docker-compose.monolith.yml.old`).

---

## Quick Reference

### Stop Everything

```bash
# Stop development tools
docker compose -f docker-compose.dev.yml down

# Stop monitoring
docker compose -f docker-compose.monitoring.yml down

# Stop all pods
docker compose -f docker-compose.pod.yml -p pod-dk down
docker compose -f docker-compose.pod.yml -p pod-no down
docker compose -f docker-compose.pod.yml -p pod-se down
docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu down
```

### View Logs

```bash
# Development tools
docker compose -f docker-compose.dev.yml logs -f forgejo

# Pod logs
docker compose -f docker-compose.pod.yml -p pod-dk logs -f postgres

# Specific service
docker logs -f service-postgres-dk
```

### Clean Data (Caution!)

```bash
# Remove all volumes for a pod
docker compose -f docker-compose.pod.yml -p pod-dk down -v

# Remove specific volume
docker volume rm pod-dk-postgres-data
```

---

## Migration from Old Architecture

The old monolithic `docker-compose.yml` has been archived as `docker-compose.monolith.yml.old`.

**What changed:**
- ❌ Old: Single `docker-compose.yml` with everything
- ✅ New: Split into 4 files (dev, monitoring, pod, multi-territory-pod)

**Why:**
- Better separation of concerns
- Per-pod deployment (multi-tenant)
- Easier to scale (add new territories)
- Independent monitoring stack
- Clearer development workflow

---

## See Also

- [Multi-Pod README](../MULTI-POD-README.md) - Multi-pod deployment guide
- [Forgejo MCP Setup](../docs/forgejo-mcp-setup.md) - Development environment setup
- [Phase 1 Approach](../project_status/phase-1-development-approach.md) - Development workflow
