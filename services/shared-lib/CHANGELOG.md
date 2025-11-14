# shared-lib Changelog

All notable changes to the shared-lib crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned

- JWT token utilities
- Additional middleware patterns
- Enhanced metrics collectors

### Added (November 14, 2025)

- **Permission System** - Badge-based RBAC with wildcard support
  - `PermissionChecker` with LRU cache (5-minute TTL, 1000 entries)
  - `RequirePermission` middleware for single permission enforcement
  - `RequireAnyPermission` middleware for OR logic
  - Complete documentation in `PERMISSION.md`
- **Metrics Module** - Prometheus metrics collection
  - `MetricsCollector` with automatic HTTP tracking
  - Standard metrics (service info, database pool, HTTP requests, errors)
  - Integration with actix-web middleware
- **Middleware Enhancements**
  - `LoggingMiddleware::development()` and `::production()` variants
  - Deprecated `::phase1()` in favor of `::development()`
- **Database Migrations**
  - Migration 20251113000006: Badge service tables
  - Migration 20251113000007: User settings table
- **Documentation**
  - `PERMISSION.md` - Permission system documentation
  - `NATS-EVENTS.md` - NATS event standards and best practices
  - Updated `MIDDLEWARE.md` with permission middleware

---

## [0.1.0-alpha.1] - 2025-11-05

**Release Stage:** Alpha (Foundation library, not yet used by services)

### Added

- Initial release of shared-lib
- Configuration module (`config.rs`) with environment-based config loading
- Database module (`database.rs`) with SQLx PostgreSQL connection pooling
- Error handling module (`error.rs`) with unified AppError type
- NATS client module (`nats.rs`) for async NATS messaging
- Version information exposed at build time (version, git hash, build timestamp)
- Build script to inject version metadata
- Re-exports for commonly used types (AppConfig, Database, AppError, NatsClient)

### Infrastructure

- PostgreSQL 16 support with sqlx
- NATS 2.10 integration with async-nats
- Actix-web error integration
- Tokio async runtime

### Dependencies

- sqlx 0.8 (PostgreSQL, UUID, chrono, JSON support)
- async-nats 0.37
- actix-web 4.9
- tokio 1.41
- serde 1.0 / serde_json 1.0
- anyhow 1.0 / thiserror 2.0
- config 0.14 / dotenvy 0.15
- tracing 0.1 / tracing-subscriber 0.3

---

**Crate:** shared-lib  
**Location:** services/shared-lib/
