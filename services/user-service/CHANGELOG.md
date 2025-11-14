# user-service Changelog

All notable changes to the user-service will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned

- Avatar upload/storage (IPFS integration in Stage 9)
- Privacy settings enforcement in frontend
- Data export automation (scheduled jobs)
- Account deletion workflow automation
- User search with advanced filters
- Follow/unfollow implementation
- Block/unblock implementation
- Followers/following list pagination
- Activity feed
- Profile visit tracking

---

## [0.1.0-alpha.1] - 2025-11-14

**Release Stage:** Alpha (MVP Phase 1 - Production Ready)

### Added - Profile Management (3 endpoints)

- GET /api/v1/user/profile - Get own profile (auto-created on first access)
- PUT /api/v1/user/profile - Update profile (display name, bio, location, website)
- GET /api/v1/user/profile/{id} - View other user profiles

### Added - Profile Links (4 endpoints)

- GET /api/v1/user/profile/links - List profile links with display ordering
- POST /api/v1/user/profile/links - Create link (max 10 per user)
- PUT /api/v1/user/profile/links/{id} - Update link
- DELETE /api/v1/user/profile/links/{id} - Delete link

### Added - Language Proficiency (4 endpoints)

- GET /api/v1/user/profile/languages - List languages
- POST /api/v1/user/profile/languages - Add language with 4-dimensional skills
  - Spoken comprehension (0-6 scale)
  - Written comprehension (0-6 scale)
  - Reading comprehension (0-6 scale)
  - Listening comprehension (0-6 scale)
- PUT /api/v1/user/profile/languages/{id} - Update language proficiency
- DELETE /api/v1/user/profile/languages/{id} - Delete language

### Added - User Connections (7 endpoints - Placeholders)

- POST /api/v1/user/{id}/follow - Follow user (TODO)
- DELETE /api/v1/user/{id}/follow - Unfollow user (TODO)
- POST /api/v1/user/{id}/block - Block user (TODO)
- DELETE /api/v1/user/{id}/block - Unblock user (TODO)
- GET /api/v1/user/{id}/followers - List followers (TODO)
- GET /api/v1/user/{id}/following - List following (TODO)
- GET /api/v1/user/search - Search users (TODO)

### Added - User Settings (6 endpoints)

- GET /api/v1/user/settings - Get all settings (auto-creates defaults)
- PATCH /api/v1/user/settings - Update any combination of settings
- GET /api/v1/user/settings/privacy - Get privacy settings subset
- PATCH /api/v1/user/settings/privacy - Update privacy settings
- GET /api/v1/user/settings/notifications - Get notification settings subset
- PATCH /api/v1/user/settings/notifications - Update notification settings

### Added - Health Endpoints

- GET /api/v1/health - Service health status
- GET /api/v1/ready - Database connectivity check
- GET /api/v1/metrics - Prometheus metrics export

### Settings Categories

- **App Preferences:** theme (light/dark/system), language (ISO 639-1), timezone (IANA)
- **Privacy:** profile visibility (public/territory/private), show email, show location, allow messages (everyone/connections/none)
- **Notifications:** email, badge, course, forum, marketing emails
- **Activity:** show activity, show online status

### Security

- Defense-in-depth authorization (JWT + handler + service + database WHERE)
- Users cannot modify other users' data
- Profile visibility framework (enforcement in frontend)
- Privacy settings control data exposure

### Architecture

- AppConfig-based configuration
- NATS client initialized and ready
- camelCase JSON serialization
- Comprehensive error handling with AppError
- OpenAPI/Swagger documentation
- Auto-create defaults pattern (profiles, settings)
- COALESCE update pattern for partial updates

### Database

- Migration 20251113000004: 6 tables (profiles, links, languages, connections, data_exports, deletion_requests)
- Migration 20251113000007: users_settings table
- 26 indexes for performance
- 2 triggers for automatic timestamp updates
- Territory-based user data isolation

### Design Decisions

- Settings-service merged into user-service for MVP simplicity
- Settings will migrate to Holochain user source chain in Phase 3
- All user data consolidated in one service
- Clean separation for future extraction if needed

### Infrastructure

- Multi-territory support (DK pod operational)
- PostgreSQL 16 with TimescaleDB
- NATS messaging integration (unityplatform-global cluster)
- Redis rate limiting support
- Prometheus metrics
- Actix-web 4.9 HTTP server
- Port 8002 (default)

### Dependencies

- shared-lib v0.1.0-alpha.1
- actix-web 4.9
- sqlx 0.8 (PostgreSQL)
- validator 0.19
- utoipa 5.3 (OpenAPI)

### Testing

- All 24 endpoints tested (18 working, 6 TODO placeholders)
- Profile auto-creation verified
- Settings auto-creation verified
- Database persistence confirmed
- Security verified (users cannot modify others' data)

---
