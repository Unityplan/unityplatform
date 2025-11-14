# Unity Platform Scripts

Collection of utility scripts for managing the Unity Platform infrastructure and services.

## � Overview

**Platform Name**: Unity Platform (formerly "unityplan")  
**Database Naming**: `unityplatform_${TERRITORY_CODE}` (e.g., `unityplatform_dk`)  
**Network Naming**: `unityplatform-*` (e.g., `unityplatform-mesh-network`)  
**Container Naming**: `service-postgres-${TERRITORY_CODE}`, `monitoring-*`, `dev-*`

**Important**: All references to "unityplan" (without "form") are legacy and being phased out. Use `unityplatform` consistently.

---

## �🚀 Quick Start Development Scripts (RECOMMENDED)

### Infrastructure Management

**`rebuild-docker-infrastructure.sh`** - Complete infrastructure rebuild

- **NEW**: Clean rebuild of all Docker infrastructure with consistent naming
- Stops all containers, archives old data, recreates networks/volumes
- Preserves: Forgejo git repository data
- Usage: `./scripts/rebuild-docker-infrastructure.sh`
- **Run this first** if you encounter naming inconsistencies

### Start All Development Services

```bash
./scripts/start-dev-services.sh
```

**Starts**: Docker infrastructure + auth-service + frontend  
**One command** to start everything you need for development!

### Stop All Services

```bash
./scripts/stop-dev-services.sh
```

### Restart All Services

```bash
./scripts/restart-dev-services.sh
```

### Check Service Status

```bash
./scripts/dev-status.sh
```

### Service URLs

- Auth Service Swagger: <http://localhost:8001/swagger-ui/>
- Adminer (DB UI): <http://localhost:8080>
- Grafana (Monitoring): <http://localhost:3001> (admin/admin)
- Prometheus: <http://localhost:9090>
- Dev Dashboard: <http://localhost:8888>

---

## 📦 Infrastructure Scripts

### Development

**`start-dev.sh`** - Start Phase 1 development environment

- Starts: Forgejo + Docker Registry
- Purpose: Minimal setup for MVP development
- Usage: `./scripts/start-dev.sh`

**`start-architecture.sh`** - Flexible startup with options

- Starts: Specific components or full stack
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
- Usage: `./scripts/stop-architecture.sh [OPTIONS]`
- Examples:
  - `./scripts/stop-architecture.sh --all`
  - `./scripts/stop-architecture.sh --all --clean` (⚠️ deletes data!)

### Multi-Pod Deployment

**`deploy-multi-pod.sh`** - Deploy all production pods

- Deploys: Denmark (DK), Norway (NO), Sweden (SE), Europe (EU) pods
- Purpose: Multi-pod production deployment
- Usage: `./scripts/deploy-multi-pod.sh [--clean]`

**`verify-multi-pod.sh`** - Verify multi-pod deployment

- Checks: All pod services, health, connectivity
- Usage: `./scripts/verify-multi-pod.sh`

---

## 🗄️ Database Scripts

**`setup-database.sh`** - Reset and run migrations

- **Database**: `unityplatform_dk` (not "unityplan")
- Drops and recreates database
- Runs all migrations from `services/shared-lib/migrations/`
- Usage: `./scripts/setup-database.sh`

**Note**: Database migrations are now managed by `sqlx migrate` command:

```bash
cd services/shared-lib
DATABASE_URL="postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk" \
  sqlx migrate run
```

---

## 👥 User & Invitation Scripts

**`create-test-user.sh`** - Create test user

- Creates user directly in database (bypasses invitation system)
- **Development only** - DO NOT use in production!
- Usage: `./scripts/create-test-user.sh <territory> [email] [username] [password] [full_name]`
- Example: `./scripts/create-test-user.sh dk test@example.com testuser TestPass123!`
- Database: `unityplatform_${TERRITORY_CODE}`
- Schema: `territory_${TERRITORY_CODE}`

**`create-bootstrap-invitation.sh`** - Create bootstrap invitation

- Creates initial invitation token for territory managers
- Usage: `./scripts/create-bootstrap-invitation.sh <territory> <email> [days]`
- Example: `./scripts/create-bootstrap-invitation.sh dk admin@unityplatform.dk 365`
- Database: `unityplatform_${TERRITORY_CODE}`
- Schema: `territory_${TERRITORY_CODE}`

---

## 🛠️ Development Tools

**`scaffold-service.sh`** - Create new Rust microservice

- Scaffolds a new service with standard structure
- Usage: `./scripts/scaffold-service.sh <service-name>`
- Creates: Cargo.toml, src/ structure, handlers, models, services

**`install-dev-tools.sh`** - Install development dependencies

- Installs: sqlx-cli, cargo-watch, etc.
- Usage: `./scripts/install-dev-tools.sh`

---

## 📦 Docker Compose Reference

### Naming Convention

**Project Names**: Defined in compose files as `name: unityplatform-${SUFFIX}`

- `unityplatform-dev` - Development tools
- `unityplatform-monitoring` - Monitoring stack
- `unityplatform-pod-dk` - Denmark pod
- `unityplatform-pod-no` - Norway pod
- `unityplatform-pod-se` - Sweden pod
- `unityplatform-pod-eu` - Europe multi-territory pod

**Networks**:

- `unityplatform-mesh-network` - Shared mesh network (external)
- `unityplatform-global-net` - Global network
- `unityplatform-pod-${POD_ID}-net` - Pod-specific networks

**Volumes**: Prefixed with project name

- `unityplatform-dev_forgejo-data`
- `unityplatform-dev_registry-data`

### Phase 1 (Development Tools)

```bash
# Start development tools
docker compose -f docker-compose.dev.yml up -d

# Includes: Forgejo, Registry, Adminer, MailHog, Redis Commander, Dashboard
```

### Monitoring (Global)

```bash
# Start monitoring stack
docker compose -f docker-compose.monitoring.yml up -d

# Includes: Prometheus, Grafana, Jaeger, Traefik
```

### Single-Territory Pod

```bash
# Deploy Denmark pod
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env up -d

# Includes: PostgreSQL (unityplatform_dk), Redis, NATS, IPFS, Matrix, Exporters
```

### Multi-Territory Pod

```bash
# Deploy Europe pod (Germany, France, Spain)
docker compose -f docker-compose.multi-territory-pod.yml --env-file pods/europe/.env up -d

# Includes: PostgreSQL (unityplatform_de, unityplatform_fr, unityplatform_es)
```

---

## 🔧 Common Operations

### Database Access

```bash
# Connect to Denmark pod database
docker exec -it service-postgres-dk psql -U unityplatform -d unityplatform_dk

# List all databases in a pod
docker exec service-postgres-dk psql -U unityplatform -c "\l"

# Run query
docker exec service-postgres-dk psql -U unityplatform -d unityplatform_dk \
  -c "SELECT * FROM territory_dk.users;"
```

### Database Credentials (Denmark Pod)

- **Host**: `localhost:5432`
- **Database**: `unityplatform_dk`
- **User**: `unityplatform`
- **Password**: `unityplatform_dev_password_dk`
- **Connection String**: `postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk`

### View Logs

```bash
# Development tools
docker compose -f docker-compose.dev.yml logs -f

# Pod logs
docker logs -f service-postgres-dk
docker logs -f service-redis-dk
docker logs -f service-nats-dk

# Auth service (if running)
docker logs -f auth-service  # or check terminal where cargo run was executed
```

### Stop Everything

```bash
# Stop development tools
docker compose -f docker-compose.dev.yml down

# Stop monitoring
docker compose -f docker-compose.monitoring.yml down

# Stop all pods
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down
docker compose -f docker-compose.pod.yml --env-file pods/norway/.env down
docker compose -f docker-compose.pod.yml --env-file pods/sweden/.env down
docker compose -f docker-compose.multi-territory-pod.yml --env-file pods/europe/.env down

# Remove mesh network
docker network rm unityplatform-mesh-network
```

### Clean Data (Caution!)

```bash
# Remove all volumes for a pod (deletes all data!)
docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down -v

# Remove specific volume
docker volume rm unityplatform-pod-dk_postgres-data

# List all Unity Platform volumes
docker volume ls | grep unityplatform
```

---

## 🔍 Troubleshooting

### Port Conflicts

```bash
# Check what's using a port
sudo lsof -i :5432
sudo netstat -tulpn | grep 5432
```

### Network Issues

```bash
# Verify mesh network exists
docker network inspect unityplatform-mesh-network

# Recreate mesh network
docker network rm unityplatform-mesh-network
docker network create unityplatform-mesh-network
```

### Database Connection Issues

```bash
# Check PostgreSQL is running
docker ps | grep postgres

# Check PostgreSQL logs
docker logs service-postgres-dk

# Test connection
docker exec service-postgres-dk psql -U unityplatform -d unityplatform_dk -c "SELECT 1;"
```

### Container Not Starting

```bash
# Check container status
docker ps -a | grep service-postgres-dk

# View container logs
docker logs service-postgres-dk --tail 50

# Restart container
docker restart service-postgres-dk
```

---

## 📚 Related Documentation

- **Multi-Pod Setup**: `../MULTI-POD-README.md`
- **Architecture**: `../docs/architecture/`
- **Database Schema**: `../services/shared-lib/migrations/`
- **Auth Service**: `../services/auth-service/README.md`
- **Copilot Instructions**: `../.github/copilot-instructions.md`

---

## 🔄 Migration from Old Naming

If you encounter references to "unityplan" (without "platform"):

1. **Run rebuild script**: `./scripts/rebuild-docker-infrastructure.sh`
2. **Update database connection strings**: Use `unityplatform_dk` instead of `unityplan`
3. **Check environment files**: `pods/*/env` should use `unityplatform` naming
4. **Verify networks**: Should be `unityplatform-*` not `unityplan-*`

---

**Version**: 1.1  
**Last Updated**: November 13, 2025  
**Status**: Active - Naming standardized to "unityplatform"

- Easier to scale (add new territories)
- Independent monitoring stack
- Clearer development workflow

---

## See Also

- [Multi-Pod README](../MULTI-POD-README.md) - Multi-pod deployment guide
- [Forgejo MCP Setup](../docs/forgejo-mcp-setup.md) - Development environment setup
- [Phase 1 Approach](../project_status/phase-1-development-approach.md) - Development workflow
