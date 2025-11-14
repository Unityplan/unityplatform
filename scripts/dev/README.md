# Development Scripts

Scripts for managing the Unity Platform development environment.

## Scripts

### **start-dev-services.sh**
Start all development services (Docker infrastructure + auth-service + frontend).

**Usage:**
```bash
./dev/start-dev-services.sh
# OR from root: ./start.sh
```

**Starts:**
- Docker infrastructure (Postgres, Redis, NATS, etc.)
- auth-service
- frontend (app)

---

### **stop-dev-services.sh**
Stop all running development services.

**Usage:**
```bash
./dev/stop-dev-services.sh
# OR from root: ./stop.sh
```

---

### **restart-dev-services.sh**
Restart all development services.

**Usage:**
```bash
./dev/restart-dev-services.sh
# OR from root: ./restart.sh
```

---

### **dev-status.sh**
Check the status of all development services.

**Usage:**
```bash
./dev/dev-status.sh
# OR from root: ./status.sh
```

**Shows:**
- Service health
- Port availability
- Docker container status

---

### **start-dev.sh**
Start Phase 1 minimal development environment.

**Usage:**
```bash
./dev/start-dev.sh
```

**Starts:**
- Forgejo (Git server)
- Docker Registry

---

## Quick Reference

**Root-level symlinks for convenience:**
- `../start.sh` → `start-dev-services.sh`
- `../stop.sh` → `stop-dev-services.sh`
- `../restart.sh` → `restart-dev-services.sh`
- `../status.sh` → `dev-status.sh`

---

## Service URLs

- Auth Service Swagger: http://localhost:8001/swagger-ui/
- Adminer (DB UI): http://localhost:8080
- Grafana (Monitoring): http://localhost:3001 (admin/admin)
- Prometheus: http://localhost:9090
- Dev Dashboard: http://localhost:8888

---

## CHANGELOG

### [Unreleased]

### [1.0.0] - 2025-11-14

#### Added
- Organized development scripts into `dev/` subdirectory
- Created root-level symlinks for common operations

#### Changed
- **BREAKING**: Scripts moved from `scripts/` root to `scripts/dev/`
- Use symlinks at root (`start.sh`, `stop.sh`, etc.) for backwards compatibility

---

**Category**: Development Workflow  
**Maintainer**: Unity Platform Team  
**Last Updated**: 2025-11-14
