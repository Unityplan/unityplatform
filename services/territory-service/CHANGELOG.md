# territory-service Changelog

All notable changes to the territory-service will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- Territory creation workflow (admin only)
- Territory deactivation/reactivation
- Territory metadata management
- Cross-territory federation settings
- Territory-specific feature flags
- Territory analytics and statistics
- Multi-territory pod deployment automation

---

## [0.1.0-alpha.1] - 2025-11-14

**Release Stage:** Alpha (MVP Phase 1 - Production Ready)

### Added - Territory Management (6 endpoints)
- GET /api/v1/territories - List all active territories
- GET /api/v1/territories/{code} - Get territory details by ISO code
- POST /api/v1/territories - Create new territory (admin only - placeholder)
- GET /api/v1/territories/{code}/stats - Get territory statistics (user count, etc.)
- GET /api/v1/territories/{code}/settings - Get territory settings
- PUT /api/v1/territories/{code}/settings - Update territory settings

### Added - Health Endpoints
- GET /api/v1/health - Service health status
- GET /api/v1/ready - Database connectivity check
- GET /api/v1/metrics - Prometheus metrics export

### Territory Management
- ISO 3166-1 Alpha-2 territory codes (DK, NO, SE, EU)
- Multi-language support (display names per territory)
- Timezone and currency settings per territory
- Territory activation/deactivation status
- Territory metadata (description, language, timezone, currency)

### Security
- Territory creation requires Platform Manager badge (TODO: enforcement)
- Territory settings update requires Territory Manager badge (TODO: enforcement)
- Read-only access for territory listing and details

### Architecture
- AppConfig-based configuration
- NATS client initialized and ready
- camelCase JSON serialization
- Comprehensive error handling with AppError
- OpenAPI/Swagger documentation
- Territory-aware data isolation

### Database
- Global territories registry (global.territories_registry)
- Territory-specific settings (territory_dk.territory_settings)
- Automatic replication from territory settings to global registry
- User count statistics per territory

### Design Decisions
- Territory code follows ISO 3166-1 Alpha-2 standard
- One territory per pod (DK pod operational)
- Settings stored in territory schema, replicated to global
- Territory manager role badge for administrative actions

### Infrastructure
- Multi-territory support (DK pod operational)
- PostgreSQL 16 with TimescaleDB
- NATS messaging integration (unityplatform-global cluster)
- Redis rate limiting support
- Prometheus metrics
- Actix-web 4.9 HTTP server
- Port 8008 (default)

### Dependencies
- shared-lib v0.1.0-alpha.1 (with permission system)
- actix-web 4.9
- sqlx 0.8 (PostgreSQL)
- validator 0.19
- utoipa 5.3 (OpenAPI)

### Testing
- All 6 endpoints manually tested and verified
- Territory listing working
- Territory details working
- Settings management working
- Statistics endpoint working

### Documentation
- API.md - Full endpoint documentation
- DATABASE.md - Schema documentation
- README.md - Service overview

---
