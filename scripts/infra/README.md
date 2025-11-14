# Infrastructure Scripts

Scripts for managing Unity Platform Docker infrastructure and architecture.

## Scripts

### **rebuild-docker-infrastructure.sh**
Complete infrastructure rebuild with consistent naming.

**Usage:**
```bash
./infra/rebuild-docker-infrastructure.sh
```

**Actions:**
- Stops all containers
- Archives old data volumes
- Recreates networks and volumes with `unityplatform-*` naming
- Preserves Forgejo git repository data

**⚠️ IMPORTANT**: Run this first if you encounter naming inconsistencies (legacy `unityplan` references).

---

### **start-architecture.sh**
Flexible startup script with component selection.

**Usage:**
```bash
./infra/start-architecture.sh [OPTIONS]
```

**Options:**
- `--phase1` - Start Phase 1 development tools (Forgejo, Registry)
- `--dev-tools` - Start development tools
- `--monitoring` - Start monitoring stack (Prometheus, Grafana)
- `--pod <territory>` - Start specific pod (dk, no, se, eu)
- `--all-pods` - Start all pods
- `--full` - Start everything
- `--help` - Show help message

**Examples:**
```bash
./infra/start-architecture.sh --phase1
./infra/start-architecture.sh --monitoring
./infra/start-architecture.sh --pod dk
./infra/start-architecture.sh --full
```

---

### **stop-architecture.sh**
Stop Docker infrastructure components.

**Usage:**
```bash
./infra/stop-architecture.sh [OPTIONS]
```

**Options:**
- `--all` - Stop all containers
- `--all --clean` - Stop and remove volumes (⚠️ deletes data!)
- `--pod <territory>` - Stop specific pod

**Examples:**
```bash
./infra/stop-architecture.sh --all
./infra/stop-architecture.sh --pod dk
./infra/stop-architecture.sh --all --clean  # ⚠️ DANGER: Removes all data!
```

---

## Architecture Components

### Networks
- `unityplatform-mesh-network` - Shared mesh network (external)
- `unityplatform-global-net` - Global network
- `unityplatform-pod-${POD_ID}-net` - Pod-specific networks

### Docker Compose Projects
- `unityplatform-dev` - Development tools
- `unityplatform-monitoring` - Monitoring stack
- `unityplatform-pod-dk` - Denmark pod
- `unityplatform-pod-no` - Norway pod
- `unityplatform-pod-se` - Sweden pod
- `unityplatform-pod-eu` - Europe multi-territory pod

### Volumes
All volumes prefixed with project name:
- `unityplatform-dev_forgejo-data`
- `unityplatform-dev_registry-data`
- `unityplatform-pod-dk_postgres-data`
- etc.

---

## CHANGELOG

### [Unreleased]

### [1.0.0] - 2025-11-14

#### Added
- Organized infrastructure scripts into `infra/` subdirectory
- Consolidated architecture management scripts

#### Changed
- **BREAKING**: Scripts moved from `scripts/` root to `scripts/infra/`
- `rebuild-docker-infrastructure.sh` updated with `unityplatform` naming

#### Fixed
- Naming standardization from legacy `unityplan` to `unityplatform`

---

**Category**: Infrastructure Management  
**Maintainer**: Unity Platform Team  
**Last Updated**: 2025-11-14
