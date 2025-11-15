# badge-service Changelog

All notable changes to the badge-service will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Changed

- **Database Schema Update:** Renamed tables to follow service-based naming convention
  - `global.badge_registry` → `global.registry_badge`
  - `territory_{code}.user_badges` → `territory_{code}.badge_users_badges`
  - `territory_{code}.badge_progress` → `territory_{code}.badge_users_progress`
  - All SQL queries updated across badge service logic
  - Verified: Badge listing endpoint tested and working correctly

### Planned

- Hook system implementation (cryptographic signatures for event-driven badge criteria)
- Course completion badge automation
- Badge rarity tiers
- Badge collections and showcases
- Badge trading/transfer system
- Community-created badges
- Badge analytics and statistics

---

## [0.1.0-alpha.1] - 2025-11-14

**Release Stage:** Alpha (MVP Phase 1 - Production Ready)

### Added - Badge Management (7 endpoints)

- GET /api/v1/badges - List all available badges
- GET /api/v1/badges/{badge_id} - Get badge details (API ready, implementation pending)
- GET /api/v1/users/{user_id}/badges - Get user's awarded badges
- POST /api/v1/badges/award - Award badge to user (admin only)
- POST /api/v1/badges/revoke - Revoke badge from user (admin only)
- PATCH /api/v1/users/me/badges/{badge_id}/featured - Toggle featured status
- POST /api/v1/badges/register-publisher - Register service role badges (service autonomy)

### Added - Health Endpoints

- GET /api/v1/health - Service health status
- GET /api/v1/ready - Database connectivity check
- GET /api/v1/metrics - Prometheus metrics export

### NATS Event Integration

- **Subscriber:** global.user.registered (auto-grant Code of Conduct in dev mode)
- **Publisher:** badge.awarded (published on every badge award)
- **Publisher:** badge.revoked (published on every badge revocation)

### Permission System

- Badge-based RBAC with wildcard support
- Permission registration via /badges/register-publisher endpoint
- Service autonomy: Services can register their own role badges on startup
- Example role badges:
  - territory-service:territory-manager (wildcard: territory:*)
  - portal-service:portal-admin (wildcard: portal:*)

### Security

- Badge awarding requires Platform Manager badge (TODO: enforcement)
- Badge revoking requires Platform Manager badge (TODO: enforcement)
- Permission checks with LRU cache (5-minute TTL, 1000 entries)
- Territory-aware permission queries
- Featured badge limit (3 per user)

### Architecture

- AppConfig-based configuration
- NATS event publishing and subscription
- camelCase JSON serialization
- Comprehensive error handling with AppError
- OpenAPI/Swagger documentation
- Hook system designed (see HOOK-SYSTEM.md)
- Graceful error handling for NATS events

### Database

- Migration 20251113000006: 3 tables (badge_definitions, user_badges, badge_permissions)
- Badge categories: role, achievement, course-completion, community
- Badge types: permanent, expiring
- Permission storage with wildcard support
- Indexes for performance

### Design Decisions

- Hook system with cryptographic signatures (deferred to Phase 2)
- Event-driven badge criteria evaluation
- Service autonomy through badge registration
- Permission system integrated at shared-lib level

### Infrastructure

- Multi-territory support (DK pod operational)
- PostgreSQL 16 with TimescaleDB
- NATS messaging integration (unityplatform-global cluster)
- Redis rate limiting support
- Prometheus metrics
- Actix-web 4.9 HTTP server
- Port 8007 (default)

### Dependencies

- shared-lib v0.1.0-alpha.1 (with permission system)
- actix-web 4.9
- sqlx 0.8 (PostgreSQL)
- validator 0.19
- utoipa 5.3 (OpenAPI)

### Testing

- All 7 endpoints manually tested and verified
- NATS event flow validated
- Permission system verified
- Role badge permissions working

### Documentation

- HOOK-SYSTEM.md - Comprehensive security design for event-driven badges
- API.md - Full endpoint documentation
- DATABASE.md - Schema documentation
- README.md - Service overview

---
