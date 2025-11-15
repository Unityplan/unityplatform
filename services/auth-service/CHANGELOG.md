# auth-service Changelog

All notable changes to the auth-service will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Changed

- **Database Schema Update:** Renamed tables to follow service-based naming convention
  - `territory_{code}.users` → `territory_{code}.auth_users_core`
  - `territory_{code}.refresh_tokens` → `territory_{code}.auth_users_refresh_tokens`
  - `global.username_registry` → `global.registry_username`
  - `global.email_registry` → `global.registry_email`
  - All SQL queries updated across all handlers (registration, login, token management)
  - Verified: All endpoints tested and working correctly with new table names

### Planned

- Invitation service integration for token validation
- Email verification flow
- Password reset functionality
- OAuth2/OIDC integration
- Two-factor authentication (2FA)

---

## [0.1.0-alpha.1] - 2025-11-14

**Release Stage:** Alpha (MVP Phase 1 - Production Ready)

### Added

- Initial release of auth-service
- User registration endpoint (POST /api/v1/auth/register)
  - Username validation and uniqueness check
  - Optional email validation and uniqueness check
  - Password hashing with Argon2id
  - Territory-based user creation
  - JWT access and refresh token generation
  - NATS event publishing (global.user.registered)
- User login endpoint (POST /api/v1/auth/login)
  - Username/password authentication
  - JWT token generation
  - Session token storage with expiration
- Token refresh endpoint (POST /api/v1/auth/refresh)
  - Refresh token validation
  - New access token generation
- Logout endpoint (POST /api/v1/auth/logout)
  - Session token cleanup
  - Invalidate refresh tokens
- Token verification endpoint (GET /api/v1/auth/verify)
  - JWT validation and claims extraction
- Username availability endpoint (POST /api/v1/auth/check-username)
  - Real-time username availability check
- Health endpoints (GET /api/v1/health, /api/v1/ready, /api/v1/metrics)
  - Service health monitoring
  - Database connectivity check
  - Prometheus metrics export

### Security

- Argon2id password hashing (OWASP recommended)
- JWT-based authentication with HS256 signing
- Refresh token rotation
- Session token expiration (24 hours default)
- Access token expiration (configurable)
- Territory-based user isolation
- Global username/email uniqueness enforcement

### Architecture

- AppConfig-based configuration (APP__*__* environment variables)
- NATS event publishing for user.registered events
- camelCase JSON serialization
- Comprehensive error handling with AppError
- OpenAPI/Swagger documentation
- Graceful shutdown (SIGTERM/Ctrl+C)
- Circuit breaker patterns ready

### Database

- Global registries: username_registry, email_registry
- Territory-based users table: territory_dk.users
- Session tokens: territory_dk.session_tokens
- Foreign key constraints with cascading deletes
- Indexed username and email lookups

### Infrastructure

- Multi-territory support (DK pod operational)
- PostgreSQL 16 with TimescaleDB
- NATS messaging integration (unityplatform-global cluster)
- Redis rate limiting support
- Prometheus metrics
- Actix-web 4.9 HTTP server
- Port 8001 (default)

### Dependencies

- shared-lib v0.1.0-alpha.1
- actix-web 4.9
- sqlx 0.8 (PostgreSQL)
- argon2 0.5
- jsonwebtoken 9.3
- validator 0.19
- utoipa 5.3 (OpenAPI)

### Testing

- All 7 endpoints manually tested and verified
- Integration with other services confirmed
- NATS event flow validated

---
