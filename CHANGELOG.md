# UnityPlan Platform Changelog

All notable changes to the UnityPlan platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

- **Frontend Auth & Profile UI** (Sprint 5 - In Progress)
  - React 19.1.1 + Vite 7.2.2 + TypeScript 5.x project scaffolding
  - TailwindCSS v4.1.17 with @tailwindcss/vite plugin (CSS-based config)
  - shadcn/ui components (button, input, card, form, label, checkbox, select)
  - Zustand 5.0.8 state management with localStorage persistence
  - Auth store with login, register, logout, token refresh actions
  - UI store with theme management (light/dark/system)
  - Axios API client with automatic token refresh interceptor
  - Auth API functions: register, login, logout, refreshToken, getCurrentUser, validateInvitation
  - User API functions: profile, avatar, privacy, connections, blocks (11 endpoints)
  - **LoginPage**: Username/email + password + territory, show/hide password
  - **RegisterPage**: Two-step flow (validate invitation → registration form with strong password validation)
  - **PasswordResetPage**: Two-step flow (request reset email → reset with token)
  - **ProfileViewPage**: Display user profile with avatar, bio, metadata, join date, territory
  - **ProfileEditPage**: Edit profile info, upload avatar, manage privacy settings (profile visibility, show_email, show_location, show_connections, allow_messages_from)
  - Temporary routing in App.tsx for testing (will be replaced with TanStack Router)
  - Type-safe error handling with helper functions
  - Environment configuration (.env.development, .env.production)
  - Path aliases (@/ → src/) configured in tsconfig
  - 416 packages installed, 0 vulnerabilities
  - Comprehensive documentation: TAILWIND-V4-MIGRATION.md
- **Invitation System** - Complete invitation-only registration system
  - Database migration 20251106000003: `invitation_tokens` and `invitation_uses` tables
  - Two token types: `single_use` (email-specific) and `group` (multi-use)
  - Cryptographically secure tokens (inv_ + 32 hex characters)
  - Full CRUD API for invitation management:
    - POST /api/auth/invitations (create token)
    - GET /api/auth/invitations (list user's tokens)
    - DELETE /api/auth/invitations/{id} (revoke token)
    - GET /api/auth/invitations/{id}/uses (audit trail)
    - GET /api/auth/invitations/validate/{token} (public validation)
  - Audit trail tracking: user_id, timestamp, ip_address, user_agent
  - Bootstrap script for territory admins (`scripts/create-bootstrap-invitation.sh`)
  - Documentation: `docs/architecture/invitation-system.md`
- **JWT Middleware** - Actix-web Transform pattern for protected routes
  - Bearer token extraction and validation
  - User loading from territory-specific schemas
  - AuthenticatedUser stored in request extensions
- Auth-service implementation (register and login endpoints)
- Migration 20251106000002: NOT NULL constraints on user boolean fields
- Dynamic schema routing for multi-territory support

### Changed

- **Frontend Configuration Verified** (November 9, 2025)
  - TailwindCSS v4: Verified against official docs (<https://tailwindcss.com/docs/installation/using-vite>)
  - shadcn/ui: Verified against official docs (<https://ui.shadcn.com/docs/installation/vite>)
  - Path aliases configured in both tsconfig.json and tsconfig.app.json
  - OKLCH color theming with CSS variables (not JavaScript config)
  - @import "tailwindcss" syntax (not @tailwind directives)
  - TypeScript compilation: 0 errors
- **BREAKING:** User registration now requires invitation token
  - `RegisterRequest` now includes mandatory `invitation_token` field
  - All registrations must use a valid invitation token
  - Existing users grandfathered (invited_by_token_id nullable)
- User model updated with `invited_by_token_id` field
- All SQL queries updated to include `invited_by_token_id` column
- Auth-service queries: Migrated from compile-time macros (query!) to runtime queries (query())
  - Enables dynamic schema routing without DATABASE_URL at compile time
  - Trade-off: Less compile-time safety, more runtime flexibility

### Fixed

- **Frontend Path Alias Configuration** (November 9, 2025)
  - Added compilerOptions to tsconfig.json (was missing baseUrl and paths)
  - shadcn components now correctly installed in src/components/ui/ (not @/components/ui/)
  - Missing dependencies installed: clsx, tailwind-merge, class-variance-authority, lucide-react
- **CRITICAL:** Removed hardcoded territory_dk from auth-service
  - Service now works universally for all territories (DK, NO, SE, etc.)
  - Dynamic schema selection based on territory_code in requests
  - Queries use runtime schema interpolation instead of hardcoded names
- Database schema: User boolean fields now properly constrained as NOT NULL
  - `email_visible`, `profile_public`, `data_export_requested`, `is_verified`, `is_active`
  - All fields have defaults and are now non-nullable
- JWT middleware: UUID type conversion for user_id (JWT claims store as String)
- Invitation audit trail: Changed ip_address column from inet to text for flexibility

### Planned

- Authentication service refresh/logout endpoints (POST /auth/refresh, POST /auth/logout)
- Get current user endpoint (GET /auth/me)
- **Badge-based invitations** - Attach badges to invitation tokens for auto-granting:
  - Course access permissions (auto-enroll in specific courses)
  - Forum category permissions (grant posting rights)
  - Conditional activation after Code of Conduct course completion
  - Use cases: workshop materials access, student course enrollment, community onboarding
- User service (user-service)
- Frontend application (Vite + React)

### Next Milestone: 0.1.0-alpha.2

- Complete auth-service core features (refresh, logout, me endpoints)
- Integration tests for invitation system
- API documentation (OpenAPI/Swagger)

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

**Platform Repository:** <https://github.com/unityplan/platform>  
**Documentation:** <https://docs.unityplan.org>
