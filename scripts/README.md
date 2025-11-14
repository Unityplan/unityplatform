# Unity Platform Scripts# Unity Platform Scripts

Collection of utility scripts for managing the Unity Platform infrastructure and services.Collection of utility scripts for managing the Unity Platform infrastructure and services.

## 📋 Overview## � Overview

**Platform Name**: Unity Platform (formerly "unityplan")  **Platform Name**: Unity Platform (formerly "unityplan")  

**Database Naming**: `unityplatform_${TERRITORY_CODE}` (e.g., `unityplatform_dk`)  **Database Naming**: `unityplatform_${TERRITORY_CODE}` (e.g., `unityplatform_dk`)  

**Network Naming**: `unityplatform-*` (e.g., `unityplatform-mesh-network`)  **Network Naming**: `unityplatform-*` (e.g., `unityplatform-mesh-network`)  

**Container Naming**: `service-postgres-${TERRITORY_CODE}`, `monitoring-*`, `dev-*`**Container Naming**: `service-postgres-${TERRITORY_CODE}`, `monitoring-*`, `dev-*`

**Important**: All references to "unityplan" (without "form") are legacy and being phased out. Use `unityplatform` consistently.**Important**: All references to "unityplan" (without "form") are legacy and being phased out. Use `unityplatform` consistently.

------

## 📁 Directory Structure## �🚀 Quick Start Development Scripts (RECOMMENDED)

Scripts are organized by category for easier navigation:### Infrastructure Management

```**`rebuild-docker-infrastructure.sh`** - Complete infrastructure rebuild

scripts/

├── README.md (this file)- **NEW**: Clean rebuild of all Docker infrastructure with consistent naming

├── start.sh          → dev/start-dev-services.sh (convenience symlink)- Stops all containers, archives old data, recreates networks/volumes

├── stop.sh           → dev/stop-dev-services.sh- Preserves: Forgejo git repository data

├── restart.sh        → dev/restart-dev-services.sh- Usage: `./scripts/rebuild-docker-infrastructure.sh`

├── status.sh         → dev/dev-status.sh- **Run this first** if you encounter naming inconsistencies

│

├── dev/              Development workflow scripts### Start All Development Services

├── infra/            Infrastructure management

├── db/               Database operations```bash

├── deploy/           Deployment scripts./scripts/start-dev-services.sh

├── tools/            Development tools```

└── test/             Testing & data seeding

```**Starts**: Docker infrastructure + auth-service + frontend  

**One command** to start everything you need for development!

Each category has its own `README.md` with detailed documentation.

### Stop All Services

---

```bash

## 🚀 Quick Start./scripts/stop-dev-services.sh

```

### Most Common Commands

### Restart All Services

```bash

# Start all development services```bash

./scripts/start.sh./scripts/restart-dev-services.sh

```

# Stop all services

./scripts/stop.sh### Check Service Status

# Restart all services```bash

./scripts/restart.sh./scripts/dev-status.sh

```

# Check service status

./scripts/status.sh### Service URLs



# Rebuild infrastructure (if you encounter naming issues)- Auth Service Swagger: <http://localhost:8001/swagger-ui/>

./scripts/infra/rebuild-docker-infrastructure.sh- Adminer (DB UI): <http://localhost:8080>

```- Grafana (Monitoring): <http://localhost:3001> (admin/admin)

- Prometheus: <http://localhost:9090>

---- Dev Dashboard: <http://localhost:8888>



## 📂 Script Categories---



### [dev/](dev/) - Development Workflow## 📦 Infrastructure Scripts



**Quick Access:** Use root-level symlinks (`start.sh`, `stop.sh`, `restart.sh`, `status.sh`)### Development



- `start-dev-services.sh` - Start all development services**`start-dev.sh`** - Start Phase 1 development environment

- `stop-dev-services.sh` - Stop all services

- `restart-dev-services.sh` - Restart all services- Starts: Forgejo + Docker Registry

- `dev-status.sh` - Check service health and status- Purpose: Minimal setup for MVP development

- `start-dev.sh` - Minimal Phase 1 environment (Forgejo, Registry)- Usage: `./scripts/start-dev.sh`



**See:** [dev/README.md](dev/README.md) for details**`start-architecture.sh`** - Flexible startup with options



---- Starts: Specific components or full stack

- Usage: `./scripts/start-architecture.sh [OPTIONS]`

### [infra/](infra/) - Infrastructure Management- Help: `./scripts/start-architecture.sh --help`

- Examples:

- `rebuild-docker-infrastructure.sh` - Complete infrastructure rebuild  - `./scripts/start-architecture.sh --phase1` (same as start-dev.sh)

- `start-architecture.sh` - Flexible component startup  - `./scripts/start-architecture.sh --dev-tools`

- `stop-architecture.sh` - Stop infrastructure components  - `./scripts/start-architecture.sh --monitoring`

  - `./scripts/start-architecture.sh --pod dk`

**See:** [infra/README.md](infra/README.md) for details  - `./scripts/start-architecture.sh --all-pods`

  - `./scripts/start-architecture.sh --full`

---

**`stop-architecture.sh`** - Stop services

### [db/](db/) - Database Operations

- Stops: Specific components or everything

- `setup-database.sh` - Reset database and run migrations- Usage: `./scripts/stop-architecture.sh [OPTIONS]`

- Examples:

**See:** [db/README.md](db/README.md) for details  - `./scripts/stop-architecture.sh --all`

  - `./scripts/stop-architecture.sh --all --clean` (⚠️ deletes data!)

---

### Multi-Pod Deployment

### [deploy/](deploy/) - Deployment

**`deploy-multi-pod.sh`** - Deploy all production pods

- `deploy-multi-pod.sh` - Deploy all production pods

- `verify-multi-pod.sh` - Verify multi-pod deployment- Deploys: Denmark (DK), Norway (NO), Sweden (SE), Europe (EU) pods

- Purpose: Multi-pod production deployment

**See:** [deploy/README.md](deploy/README.md) for details- Usage: `./scripts/deploy-multi-pod.sh [--clean]`



---**`verify-multi-pod.sh`** - Verify multi-pod deployment



### [tools/](tools/) - Development Tools- Checks: All pod services, health, connectivity

- Usage: `./scripts/verify-multi-pod.sh`

- `install-dev-tools.sh` - Install development dependencies

- `scaffold-service.sh` - Create new Rust microservice---



**See:** [tools/README.md](tools/README.md) for details## 🗄️ Database Scripts



---**`setup-database.sh`** - Reset and run migrations



### [test/](test/) - Testing & Seeding- **Database**: `unityplatform_dk` (not "unityplan")

- Drops and recreates database

- `create-test-user.sh` - Create test users (dev only)- Runs all migrations from `services/shared-lib/migrations/`

- `create-bootstrap-invitation.sh` - Create bootstrap invitations- Usage: `./scripts/setup-database.sh`

- `register-platform-badges.sh` - Register platform badges

**Note**: Database migrations are now managed by `sqlx migrate` command:

**See:** [test/README.md](test/README.md) for details

```bash

---cd services/shared-lib

DATABASE_URL="postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk" \

## 🌐 Service URLs (Development)  sqlx migrate run

```

- **Auth Service Swagger**: <http://localhost:8001/swagger-ui/>

- **Adminer (DB UI)**: <http://localhost:8080--->

- **Grafana (Monitoring)**: <http://localhost:3001> (admin/admin)

- **Prometheus**: <http://localhost:9090##> 👥 User & Invitation Scripts

- **Dev Dashboard**: <http://localhost:8888>

**`create-test-user.sh`** - Create test user

---

- Creates user directly in database (bypasses invitation system)

## 🗄️ Database Credentials (Denmark Pod)- **Development only** - DO NOT use in production

- Usage: `./scripts/create-test-user.sh <territory> [email] [username] [password] [full_name]`

**Development Environment:**- Example: `./scripts/create-test-user.sh dk test@example.com testuser TestPass123!`

- Database: `unityplatform_${TERRITORY_CODE}`

- **Host**: `localhost:5432`- Schema: `territory_${TERRITORY_CODE}`

- **Database**: `unityplatform_dk`

- **User**: `unityplatform`**`create-bootstrap-invitation.sh`** - Create bootstrap invitation

- **Password**: `unityplatform_dev_password_dk`

- **Connection String**: `postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk`- Creates initial invitation token for territory managers

- Usage: `./scripts/create-bootstrap-invitation.sh <territory> <email> [days]`

**Direct Database Access:**- Example: `./scripts/create-bootstrap-invitation.sh dk admin@unityplatform.dk 365`

- Database: `unityplatform_${TERRITORY_CODE}`

```bash- Schema: `territory_${TERRITORY_CODE}`

# PostgreSQL CLI

docker exec -it service-postgres-dk psql -U unityplatform -d unityplatform_dk---

# List databases## 🛠️ Development Tools

docker exec service-postgres-dk psql -U unityplatform -c "\l"

**`scaffold-service.sh`** - Create new Rust microservice

# Run query

docker exec service-postgres-dk psql -U unityplatform -d unityplatform_dk \- Scaffolds a new service with standard structure

  -c "SELECT * FROM territory_dk.users;"- Usage: `./scripts/scaffold-service.sh <service-name>`

```- Creates: Cargo.toml, src/ structure, handlers, models, services



---**`install-dev-tools.sh`** - Install development dependencies



## 🔧 Common Operations- Installs: sqlx-cli, cargo-watch, etc.

- Usage: `./scripts/install-dev-tools.sh`

### View Logs

---

```bash

# Development tools## 📦 Docker Compose Reference

docker compose -f docker-compose.dev.yml logs -f

### Naming Convention

# Pod services

docker logs -f service-postgres-dk**Project Names**: Defined in compose files as `name: unityplatform-${SUFFIX}`

docker logs -f service-redis-dk

docker logs -f service-nats-dk- `unityplatform-dev` - Development tools

- `unityplatform-monitoring` - Monitoring stack

# Auth service- `unityplatform-pod-dk` - Denmark pod

docker logs -f auth-service- `unityplatform-pod-no` - Norway pod

```- `unityplatform-pod-se` - Sweden pod

- `unityplatform-pod-eu` - Europe multi-territory pod

### Stop Everything

**Networks**:

```bash

# Stop all via script- `unityplatform-mesh-network` - Shared mesh network (external)

./scripts/stop.sh- `unityplatform-global-net` - Global network

- `unityplatform-pod-${POD_ID}-net` - Pod-specific networks

# Manual cleanup

docker compose -f docker-compose.dev.yml down**Volumes**: Prefixed with project name

docker compose -f docker-compose.monitoring.yml down

docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down- `unityplatform-dev_forgejo-data`

```- `unityplatform-dev_registry-data`



### Clean Data (⚠️ Caution!)### Phase 1 (Development Tools)



```bash```bash

# Remove all volumes for a pod (deletes all data!)# Start development tools

docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down -vdocker compose -f docker-compose.dev.yml up -d



# List Unity Platform volumes# Includes: Forgejo, Registry, Adminer, MailHog, Redis Commander, Dashboard

docker volume ls | grep unityplatform```

```

### Monitoring (Global)

---

```bash

## 🐛 Troubleshooting# Start monitoring stack

docker compose -f docker-compose.monitoring.yml up -d

### Port Conflicts

# Includes: Prometheus, Grafana, Jaeger, Traefik

```bash```

# Check what's using a port

sudo lsof -i :5432### Single-Territory Pod

sudo netstat -tulpn | grep 5432

``````bash

# Deploy Denmark pod

### Network Issuesdocker compose -f docker-compose.pod.yml --env-file pods/denmark/.env up -d



```bash# Includes: PostgreSQL (unityplatform_dk), Redis, NATS, IPFS, Matrix, Exporters

# Verify mesh network exists```

docker network inspect unityplatform-mesh-network

### Multi-Territory Pod

# Recreate mesh network

docker network rm unityplatform-mesh-network```bash

docker network create unityplatform-mesh-network# Deploy Europe pod (Germany, France, Spain)

```docker compose -f docker-compose.multi-territory-pod.yml --env-file pods/europe/.env up -d



### Database Connection Issues# Includes: PostgreSQL (unityplatform_de, unityplatform_fr, unityplatform_es)

```

```bash

# Check PostgreSQL is running---

docker ps | grep postgres

## 🔧 Common Operations

# Check logs

docker logs service-postgres-dk### Database Access



# Test connection```bash

docker exec service-postgres-dk psql -U unityplatform -d unityplatform_dk -c "SELECT 1;"# Connect to Denmark pod database

```docker exec -it service-postgres-dk psql -U unityplatform -d unityplatform_dk



---# List all databases in a pod

docker exec service-postgres-dk psql -U unityplatform -c "\l"

## 📚 Related Documentation

# Run query

- **Multi-Pod Setup**: [../MULTI-POD-README.md](../MULTI-POD-README.md)docker exec service-postgres-dk psql -U unityplatform -d unityplatform_dk \

- **Architecture**: [../docs/architecture/](../docs/architecture/)  -c "SELECT * FROM territory_dk.users;"

- **Database Schema**: [../services/shared-lib/migrations/](../services/shared-lib/migrations/)```

- **Auth Service**: [../services/auth-service/README.md](../services/auth-service/README.md)

- **Copilot Instructions**: [../.github/copilot-instructions.md](../.github/copilot-instructions.md)### Database Credentials (Denmark Pod)



---- **Host**: `localhost:5432`

- **Database**: `unityplatform_dk`

## 🔄 Migration Guide- **User**: `unityplatform`

- **Password**: `unityplatform_dev_password_dk`

### From Flat Structure (pre-v2.0.0)- **Connection String**: `postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk`



**Old paths → New paths:**### View Logs



```bash```bash

# Development# Development tools

./scripts/start-dev-services.sh  → ./scripts/start.sh (or ./scripts/dev/start-dev-services.sh)docker compose -f docker-compose.dev.yml logs -f

./scripts/stop-dev-services.sh   → ./scripts/stop.sh

./scripts/restart-dev-services.sh → ./scripts/restart.sh# Pod logs

./scripts/dev-status.sh          → ./scripts/status.shdocker logs -f service-postgres-dk

docker logs -f service-redis-dk

# Infrastructuredocker logs -f service-nats-dk

./scripts/rebuild-docker-infrastructure.sh → ./scripts/infra/rebuild-docker-infrastructure.sh

./scripts/start-architecture.sh            → ./scripts/infra/start-architecture.sh# Auth service (if running)

./scripts/stop-architecture.sh             → ./scripts/infra/stop-architecture.shdocker logs -f auth-service  # or check terminal where cargo run was executed

```

# Database

./scripts/setup-database.sh → ./scripts/db/setup-database.sh### Stop Everything

# Deployment```bash

./scripts/deploy-multi-pod.sh → ./scripts/deploy/deploy-multi-pod.sh# Stop development tools

./scripts/verify-multi-pod.sh → ./scripts/deploy/verify-multi-pod.shdocker compose -f docker-compose.dev.yml down

# Tools# Stop monitoring

./scripts/install-dev-tools.sh → ./scripts/tools/install-dev-tools.shdocker compose -f docker-compose.monitoring.yml down

./scripts/scaffold-service.sh  → ./scripts/tools/scaffold-service.sh

# Stop all pods

# Testingdocker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down

./scripts/create-test-user.sh → ./scripts/test/create-test-user.shdocker compose -f docker-compose.pod.yml --env-file pods/norway/.env down

./scripts/create-bootstrap-invitation.sh → ./scripts/test/create-bootstrap-invitation.shdocker compose -f docker-compose.pod.yml --env-file pods/sweden/.env down

./scripts/register-platform-badges.sh → ./scripts/test/register-platform-badges.shdocker compose -f docker-compose.multi-territory-pod.yml --env-file pods/europe/.env down

```

# Remove mesh network

**Backwards Compatibility:** Root-level symlinks (`start.sh`, `stop.sh`, `restart.sh`, `status.sh`) provide quick access to common operations.docker network rm unityplatform-mesh-network

```

---

### Clean Data (Caution!)

## 📝 CHANGELOG

```bash

### [2.0.0] - 2025-11-14# Remove all volumes for a pod (deletes all data!)

docker compose -f docker-compose.pod.yml --env-file pods/denmark/.env down -v

#### Added

# Remove specific volume

- **Organized directory structure**: Scripts categorized into `dev/`, `infra/`, `db/`, `deploy/`, `tools/`, `test/`docker volume rm unityplatform-pod-dk_postgres-data

- **Category READMEs**: Each subdirectory has its own README.md with detailed documentation

- **Root-level symlinks**: Quick access to common operations (`start.sh`, `stop.sh`, etc.)# List all Unity Platform volumes

- **CHANGELOG**: Each category now has version trackingdocker volume ls | grep unityplatform

```

#### Changed

---

- **BREAKING**: Scripts moved from flat structure to categorized subdirectories

- **Navigation**: Easier to find scripts by category## 🔍 Troubleshooting

- **Documentation**: Improved organization and discoverability

### Port Conflicts

#### Migration

```bash

- Use root-level symlinks for backwards compatibility with common commands# Check what's using a port

- Update any CI/CD pipelines or scripts that reference old pathssudo lsof -i :5432

- See Migration Guide above for path mappingssudo netstat -tulpn | grep 5432

```

---

### Network Issues

**Version**: 2.0.0  

**Last Updated**: November 14, 2025  ```bash

**Status**: Active - Reorganized for better maintainability# Verify mesh network exists

docker network inspect unityplatform-mesh-network

---

# Recreate mesh network

## See Alsodocker network rm unityplatform-mesh-network

docker network create unityplatform-mesh-network

- [Multi-Pod README](../MULTI-POD-README.md) - Multi-pod deployment guide```

- [Development Guide](../docs/guides/development/) - Development workflow

- [Architecture Documentation](../docs/architecture/) - System architecture### Database Connection Issues

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
