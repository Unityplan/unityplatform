# UnityPlan Platform Changelog

All notable changes to the UnityPlan platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Auth-service implementation (register and login endpoints)
- Migration 20251106000002: NOT NULL constraints on user boolean fields

### Fixed
- Database schema: User boolean fields now properly constrained as NOT NULL
  - `email_visible`, `profile_public`, `data_export_requested`, `is_verified`, `is_active`
  - All fields have defaults and are now non-nullable

### Planned
- Authentication service refresh/logout endpoints
- User service (user-service)
- Frontend application (Vite + React)

### Next Milestone: 0.1.0-alpha.2
- Complete auth-service implementation
- Working user registration and login with corrected data sovereignty architecture

---

## [0.1.0-alpha.1] - 2025-11-06

### Changed
- **CRITICAL:** User data sovereignty architecture correction
  - Moved user personal data from `global.users` to `territory_*.users`
  - Created `global.user_identities` with cryptographic hashes only
  - Aligned with natural ecosystem metaphor (users belong to their pods)
  - Prepared for future Holochain migration (agent-based identity)
  - Added comprehensive documentation: `docs/architecture/user-data-sovereignty.md`
- Updated copilot instructions with natural ecosystem metaphor

### Added
- Natural ecosystem metaphor documentation in project overview
- Migration 20251106000001: User data sovereignty
- Database triggers for automatic identity sync
- Support for future WebAuthn/Holochain authentication

### Infrastructure
- Database schema restructured for data sovereignty
- Territory schemas now contain ALL user personal data
- Global schema only coordinates via cryptographic identifiers

### Security
- Enhanced privacy: personal data never leaves territory
- GDPR compliance: data stays in user-selected territory
- Future-proof: compatible with Holochain agent identities

**Release Stage:** Alpha (Infrastructure foundation, no working services yet)

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
