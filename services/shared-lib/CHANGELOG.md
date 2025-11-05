# shared-lib Changelog

All notable changes to the shared-lib crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- Metrics module for Prometheus integration
- Middleware helpers for common patterns
- JWT token utilities

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
