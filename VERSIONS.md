# UnityPlan Version Matrix

**Last Updated:** November 5, 2025  
**Platform Version:** 0.1.0-alpha.1 (MVP Phase 1 - Early Development)

---

## 📦 Platform Overview

| Component Category | Status | Notes |
|-------------------|--------|-------|
| Infrastructure | ✅ Complete | Multi-pod architecture operational |
| Database Schema | ✅ Complete | Global + territory schemas deployed |
| Backend Services | 🟡 In Progress | Auth service in development |
| Frontend | ⬜ Not Started | Planned for Stage 5 |
| Decentralization | ⬜ Not Started | Phase 3 (Holochain integration) |

**Release Stage:** Alpha (Internal testing, infrastructure only)

---

## 🔧 Backend Services

| Service | Version | Status | Last Updated | Dependencies |
|---------|---------|--------|--------------|--------------|
| **shared-lib** | 0.1.0-alpha.1 | ✅ Active | 2025-11-05 | sqlx, async-nats, actix-web |
| **auth-service** | - | 🟡 Development | - | shared-lib@0.1.0 |
| **user-service** | - | ⬜ Not Started | - | shared-lib@0.1.0, auth-service |
| **territory-service** | - | ⬜ Not Started | - | shared-lib@0.1.0 |
| **badge-service** | - | ⬜ Not Started | - | shared-lib@0.1.0 |
| **course-service** | - | ⬜ Not Started | - | shared-lib@0.1.0 |
| **forum-service** | - | ⬜ Not Started | - | shared-lib@0.1.0 |
| **translation-service** | - | ⬜ Not Started | - | shared-lib@0.1.0 |
| **ipfs-service** | - | ⬜ Not Started | - | shared-lib@0.1.0 |

---

## 🗄️ Database Schema

| Migration | Version | Description | Applied | Rollback Available |
|-----------|---------|-------------|---------|-------------------|
| Initial Schema | 20251105000001 | Global + territory_dk schemas | ✅ 2025-11-05 | ✅ Yes |

**Current Schema Version:** `20251105000001`  
**Database:** PostgreSQL 16 with TimescaleDB  
**Schemas:**
- `global` - Cross-territory data (users, territories, sessions, audit)
- `territory_dk` - Denmark-specific data (communities, members, settings)

---

## 🌐 Frontend

| Component | Version | Status | Last Updated | Framework |
|-----------|---------|--------|--------------|-----------|
| **Web App** | - | ⬜ Not Started | - | Vite 5.x + React 18.x |
| **UI Library** | - | ⬜ Not Started | - | shadcn/ui 3.5 + TailwindCSS 4.1 |
| **Routing** | - | ⬜ Not Started | - | TanStack Router 1.134 |
| **Data Layer** | - | ⬜ Not Started | - | TanStack Query v5 |
| **State Management** | - | ⬜ Not Started | - | Zustand (auth/UI only) |
| **Forms** | - | ⬜ Not Started | - | react-hook-form + zod |
| **Testing** | - | ⬜ Not Started | - | Vitest + Testing Library |
| **Matrix Client** | - | ⬜ Not Started | - | matrix-js-sdk |

**Stack Rationale:**
- React 18 chosen over React 19 for stable ecosystem during MVP phase
- TanStack Query offloads data fetching from manual state management
- Future-proof for Tauri migration (~1 year timeline)
- All dependencies fully optimized for React 18

---

## 🐳 Infrastructure

### Denmark Pod (pod-dk)

| Component | Version | Status | Port | Notes |
|-----------|---------|--------|------|-------|
| **PostgreSQL** | 16 | ✅ Running | 5432 | TimescaleDB enabled |
| **NATS** | 2.10 | ✅ Running | 4222 | JetStream enabled |
| **Redis** | 7 | ✅ Running | 6379 | Persistence enabled |
| **IPFS** | latest | ✅ Running | 5001/8081 | Kubo implementation |
| **Matrix Synapse** | latest | ⬜ Not Started | 8008 | Planned |

### Monitoring Stack

| Component | Version | Status | Port | Notes |
|-----------|---------|--------|------|-------|
| **Prometheus** | latest | ✅ Running | 9090 | Central metrics collection |
| **Grafana** | latest | ✅ Running | 3001 | Dashboards operational |
| **Jaeger** | latest | ✅ Running | 16686 | Distributed tracing |
| **Traefik** | latest | ✅ Running | 80/443 | Reverse proxy + SSL |

### Exporters (Denmark Pod)

| Exporter | Status | Target |
|----------|--------|--------|
| postgres-exporter | ✅ UP | service-postgres-dk:5432 |
| redis-exporter | ✅ UP | service-redis-dk:6379 |
| nats-exporter | ✅ UP | service-nats-dk:4222 |
| node-exporter | ✅ UP | Host metrics |
| cadvisor | ✅ UP | Container metrics |
| matrix-exporter | ⬜ DOWN | Not deployed yet |

---

## 🛠️ Development Tools

| Tool | Version | Status | Port | Purpose |
|------|---------|--------|------|---------|
| **Forgejo** | latest | ✅ Running | 3000 | Git server + CI/CD |
| **Docker Registry** | 2 | ✅ Running | 5000 | Container image registry |
| **Adminer** | latest | ✅ Running | 8080 | Database management |
| **MailHog** | latest | ✅ Running | 8025/1025 | Email testing |
| **Redis Commander** | latest | ✅ Running | 8081 | Redis GUI |
| **SQLTools** | VS Code Ext | ✅ Configured | - | Database IDE integration |

---

## 📋 Deployment Versions

### Denmark Pod (pod-dk)

**Last Deployment:** November 5, 2025  
**Platform Version:** 0.1.0  
**Environment:** Development

| Component | Image | Tag | Deployed |
|-----------|-------|-----|----------|
| PostgreSQL | postgres | 16-alpine | 2025-11-04 |
| NATS | nats | 2.10-alpine | 2025-11-04 |
| Redis | redis | 7-alpine | 2025-11-04 |
| IPFS | ipfs/kubo | latest | 2025-11-04 |

**Database Schema:** 20251105000001  
**Configuration:** pods/denmark/.env  
**Pod ID:** dk

---

## 🔄 Version History

### Platform v0.1.0-alpha.1 - November 5, 2025
**Status:** Alpha (Early Development - Infrastructure Only)

**Completed:**
- ✅ Multi-pod infrastructure deployed
- ✅ Monitoring stack operational (Prometheus, Grafana, Jaeger)
- ✅ Database schema with multi-territory support
- ✅ Rust workspace and shared-lib crate
- ✅ Development tools configured (Forgejo, SQLTools)
- ✅ Documentation reorganized

**In Progress:**
- 🟡 Authentication service implementation

**Planned:**
- ⬜ User service
- ⬜ Territory service
- ⬜ Frontend application

---

## 📝 Version Management

### Semantic Versioning (SemVer)

All services follow **MAJOR.MINOR.PATCH** versioning:

- **MAJOR**: Breaking API changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Git Tags

Services are tagged independently:
```bash
git tag shared-lib-v0.1.0
git tag auth-service-v0.1.0
git tag platform-v0.1.0
```

### Database Migrations

Migrations use timestamp-based versioning:
```
YYYYMMDDHHMMSS_description.up.sql
YYYYMMDDHHMMSS_description.down.sql
```

### API Versioning

APIs are versioned in URLs:
```
/api/v1/auth/login
/api/v1/users/me
```

---

## 🔗 Related Documentation

- [Versioning Strategy](docs/guides/development/versioning-strategy.md)
- [Deployment Guide](docs/guides/deployment/multi-pod-deployment.md)
- [Development Status](docs/status/current/phase-1-status.md)
- [Architecture Overview](docs/architecture/multi-pod-architecture.md)

---

**Note:** This file is automatically updated with each deployment. Version numbers follow [Semantic Versioning 2.0.0](https://semver.org/).
