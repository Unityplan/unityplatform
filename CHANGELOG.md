# UnityPlan Platform Changelog

All notable changes to the UnityPlan platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- Authentication service (auth-service)
- User service (user-service)
- Frontend application (Vite + React)

---

## [0.1.0] - 2025-11-05

### Added
- Multi-pod infrastructure architecture
- Denmark pod (pod-dk) deployment with PostgreSQL, NATS, Redis, IPFS
- Monitoring stack (Prometheus, Grafana, Jaeger)
- Traefik reverse proxy with automatic routing
- NATS mesh network for cross-pod communication
- Database schema with multi-territory support (global + territory schemas)
- Rust workspace configuration (services/Cargo.toml)
- shared-lib crate with config, database, error, nats modules
- SQLx database migrations system
- Development tools (Forgejo, Docker Registry, Adminer, MailHog, Redis Commander)
- SQLTools VS Code integration for database management
- Comprehensive documentation structure (docs/)
- Versioning strategy and tracking (VERSIONS.md)
- Pod exporters for metrics (PostgreSQL, Redis, NATS, Node, cAdvisor)
- Grafana dashboards (Pod Overview, Multi-Pod Overview)

### Changed
- Documentation reorganized from multiple folders into consolidated docs/ structure
- Territory management follows ISO 3166-1 Alpha-2 standard (DK, NO, SE)

### Infrastructure
- PostgreSQL 16 with TimescaleDB
- NATS 2.10 with JetStream
- Redis 7 with persistence
- IPFS Kubo
- Prometheus for metrics collection
- Grafana 3001 for visualization
- Jaeger for distributed tracing

### Security
- Schema-based multi-tenant isolation
- Network segmentation (global-net, mesh-network, pod-net)

---

## Version History Legend

- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Features marked for removal
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security improvements
- **Infrastructure**: Infrastructure changes

---

**Platform Repository:** https://github.com/unityplan/platform  
**Documentation:** https://docs.unityplan.org
