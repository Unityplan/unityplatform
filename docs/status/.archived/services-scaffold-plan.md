# Phase 1 Services Scaffolding Plan

**Date:** November 12, 2025  
**Status:** 🚧 Planning  
**Based On:**

- Completed auth-service implementation
- Backend API documentation (backend-api-index.md)
- Database schema design (database-schema-design.md)

---

## Overview

This plan outlines the scaffolding and implementation of all Phase 1 backend services, using the proven auth-service architecture as a template.

**Strategy:**

1. Scaffold all services with basic structure (health check, OpenAPI)
2. Implement Phase 1 priority services in order
3. Leave Phase 2 services scaffolded but unimplemented

---

## Services Overview

### ✅ Completed

1. **auth-service** (Port 8001) - ✅ DONE
   - Registration with invitation tokens
   - Login/logout (JWT)
   - Token refresh & validation
   - Health checks
   - OpenAPI/Swagger
   - **Status:** Production ready, all endpoints tested

### 🎯 Phase 1 Priority Services

2. **user-service** (Port 8002) - PRIORITY 1 ⭐ CRITICAL
   - User profile CRUD
   - Avatar upload/management
   - Privacy settings
   - Profile links (flexible external links)
   - User connections (follow/unfollow)
   - User blocking
   - Language proficiency management
   - Account settings (email/password change, TOTP)
   - GDPR compliance (data export, account deletion)

3. **settings-service** (Port 8003) - PRIORITY 2
   - Appearance settings
   - Language settings
   - Notification settings
   - Settings sync across devices

4. **invitation-service** (Port 8004) - PRIORITY 3
   - Invitation token creation
   - Token validation (used by auth-service)
   - Usage tracking
   - Community auto-assignment
   - Invitation audit logs

5. **notification-service** (Port 8005) - PRIORITY 4
   - In-app notifications
   - Email notifications
   - Push notifications (future)
   - Notification preferences
   - Unread count

### 🔮 Phase 2 Services (Scaffold Only)

6. **community-service** (Port 8006)
7. **badge-service** (Port 8007)
8. **territory-service** (Port 8008)
9. **event-service** (Port 8009)
10. **course-service** (Port 8010)
11. **forum-service** (Port 8011)
12. **translation-service** (Port 8012)
13. **ipfs-service** (Port 8013)

---

## auth-service Architecture (Proven Template)

### Directory Structure

```
auth-service/
├── Cargo.toml
├── .env
└── src/
    ├── main.rs              # Server entry point, route configuration
    ├── lib.rs               # Library exports
    ├── config.rs            # Environment configuration
    ├── error.rs             # Custom error types
    ├── response.rs          # API response envelope
    ├── openapi.rs           # Swagger configuration
    ├── models/
    │   ├── mod.rs
    │   ├── auth.rs          # Request/response models
    │   └── health.rs        # Health check model
    ├── handlers/
    │   ├── mod.rs
    │   ├── auth.rs          # Endpoint handlers
    │   └── health.rs        # Health endpoint
    ├── services/
    │   ├── mod.rs
    │   ├── token.rs         # JWT service
    │   ├── password.rs      # Password hashing
    │   └── invitation.rs    # Invitation validation
    └── middleware/
        ├── mod.rs
        └── auth.rs          # JWT authentication middleware
```

### Standard Dependencies (from auth-service/Cargo.toml)

```toml
[dependencies]
shared-lib = { path = "../shared-lib" }

# Web framework
actix-web = "4"
actix-cors = "0.7"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Database
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "uuid", "chrono"] }

# Authentication & Crypto (for services that need JWT validation)
jsonwebtoken = "9"
argon2 = "0.5"           # Only if hashing needed
sha2 = "0.10"            # Only if hashing needed

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Validation
validator = { version = "0.18", features = ["derive"] }

# Types
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# Configuration
dotenvy = "0.15"

# OpenAPI
utoipa = { version = "5", features = ["actix_extras", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "8", features = ["actix-web"] }
```

### Standard Configuration (.env)

```env
# Database
DATABASE_URL=postgresql://unityplatform:password@localhost:5432/unityplatform

# Territory
TERRITORY_CODE=dk

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8XXX  # Unique per service

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000

# JWT (shared secret - for validating tokens from auth-service)
JWT_SECRET=your-secret-key

# Service-specific config...
```

### Standard Response Envelope (shared)

```rust
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub meta: ResponseMeta,
}

#[derive(Serialize, ToSchema)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
pub struct ResponseMeta {
    pub timestamp: String,
    pub request_id: String,
}
```

### Standard Endpoints (all services)

```
GET /health              - Health check with dependency status
GET /swagger-ui          - Interactive API documentation
GET /api-doc/openapi.json - OpenAPI 3.0 specification
```

---

## Service Details

### user-service (Port 8002) - PRIORITY 1 ⭐

**Frontend Dependencies:**

- ProfileViewPage, ProfileEditPage
- Settings pages (appearance, language, notifications, account)
- GDPR data export/deletion

**Endpoints:** (32 total)

**Profile Management:**

- `GET /v1/profiles/:id` - Get profile (privacy-aware)
- `GET /v1/profiles/:id/full` - Get full profile (own only)
- `PUT /v1/profiles/:id` - Update profile
- `PATCH /v1/profiles/:id/privacy` - Update privacy

**Profile Links:**

- `GET /v1/profiles/:userId/links` - List links
- `POST /v1/profiles/:userId/links` - Create link
- `PUT /v1/profiles/:userId/links/:linkId` - Update link
- `DELETE /v1/profiles/:userId/links/:linkId` - Delete link
- `PATCH /v1/profiles/:userId/links/reorder` - Reorder links

**Avatar:**

- `POST /v1/avatars/:userId` - Upload avatar
- `DELETE /v1/avatars/:userId` - Delete avatar

**Connections:**

- `GET /v1/users/:id/connections` - Get connections
- `POST /v1/users/:id/follow` - Follow user
- `DELETE /v1/users/:id/follow` - Unfollow
- `POST /v1/users/:id/block` - Block user
- `DELETE /v1/users/:id/block` - Unblock

**Language Proficiency:**

- `GET /v1/users/:id/languages` - List proficiencies
- `POST /v1/users/:id/languages` - Add proficiency
- `PUT /v1/users/:id/languages/:langId` - Update
- `DELETE /v1/users/:id/languages/:langId` - Remove
- `PATCH /v1/users/:id/languages/reorder` - Reorder

**Account Management:**

- `POST /v1/account/email/change` - Request email change
- `POST /v1/account/email/verify` - Verify new email
- `POST /v1/account/password/change` - Change password
- `POST /v1/account/totp/enable` - Enable 2FA
- `POST /v1/account/totp/verify` - Verify 2FA
- `POST /v1/account/totp/disable` - Disable 2FA
- `GET /v1/account/totp/recovery` - Get recovery codes

**GDPR:**

- `POST /v1/data/export` - Request export
- `GET /v1/data/export/status` - Check status
- `GET /v1/data/export/download` - Download
- `POST /v1/account/delete` - Request deletion
- `POST /v1/account/delete/confirm` - Confirm
- `POST /v1/account/delete/cancel` - Cancel

**Database Tables:**

- `territory_dk.users` (exists - from auth)
- `territory_dk.profiles` (exists)
- `territory_dk.profile_links` (NEW)
- `territory_dk.user_privacy_settings` (NEW)
- `territory_dk.user_connections` (NEW)
- `territory_dk.user_blocks` (NEW)
- `territory_dk.language_proficiencies` (NEW)
- `territory_dk.user_totp` (NEW)
- `territory_dk.data_exports` (NEW)

**Special Requirements:**

- Avatar processing (resize: 256x256, 512x512, 1024x1024)
- Location encoding: `[lat,lon]Display Name`
- Privacy filtering based on viewer relationship
- S3/IPFS storage for avatars
- Max 10 profile links per user

**Time Estimate:** 5-7 days

---

### settings-service (Port 8003) - PRIORITY 2

**Frontend Dependencies:**

- Settings pages (appearance, language, notifications)

**Endpoints:** (8 total)

**Appearance:**

- `GET /v1/settings/appearance` - Get appearance settings
- `PATCH /v1/settings/appearance` - Update appearance

**Language:**

- `GET /v1/settings/language` - Get language settings
- `PATCH /v1/settings/language` - Update language

**Privacy:**

- `GET /v1/settings/privacy` - Get privacy settings
- `PATCH /v1/settings/privacy` - Update privacy

**Notifications:**

- `GET /v1/settings/notifications` - Get notification settings
- `PATCH /v1/settings/notifications` - Update notifications

**Database Tables:**

- `territory_dk.user_settings` (appearance, language)
- `territory_dk.user_privacy_settings` (shared with user-service)
- `territory_dk.user_notification_settings`

**Data Models:**

```typescript
interface AppearanceSettings {
  theme_mode: 'light' | 'dark' | 'system';
  color_scheme: 'forest-green' | 'ocean-blue' | 'royal-purple';
  reduced_motion: boolean;
  wide_content_view: boolean;
  compact_mode: boolean;
}

interface LanguageSettings {
  preferred_language: string;    // ISO 639-1
  timezone: string;              // IANA timezone
  auto_translate: boolean;
  translation_provider: string;
  contribute_translations: boolean;
  fallback_to_english: boolean;
}

interface NotificationSettings {
  email_digest: boolean;
  email_messages: boolean;
  email_followers: boolean;
  email_community: boolean;
  inapp_messages: boolean;
  inapp_followers: boolean;
  inapp_community: boolean;
  push_enabled: boolean;
}
```

**Special Requirements:**

- Sync settings across devices
- Merge conflicts (last-write-wins)
- Default values for new users

**Time Estimate:** 2-3 days

---

### invitation-service (Port 8004) - PRIORITY 3

**Frontend Dependencies:**

- Registration page (token validation)
- Admin invitation management

**Endpoints:** (6 total)

- `GET /v1/invitations/validate/:token` - Validate & get territory
- `POST /v1/invitations/create` - Create invitation (admin)
- `GET /v1/invitations/my-invitations` - List user's invitations
- `DELETE /v1/invitations/:tokenId` - Revoke invitation
- `GET /v1/invitations/:tokenId/uses` - View usage (audit)
- `POST /v1/invitations/:token/use` - Mark as used (internal)

**Database Tables:**

- `territory_dk.invitation_tokens` (exists)
- `global.invitation_token_registry` (exists)
- `territory_dk.invitation_uses` (NEW - audit log)

**Data Models:**

```typescript
interface InvitationToken {
  id: string;
  token: string;
  token_type: 'single_use' | 'group';
  email?: string;
  max_uses: number;
  used_count: number;
  expires_at: string;
  community_id?: string;
  created_by: string;
  invitation_url: string;
}

interface CreateInvitationRequest {
  token_type: 'single_use' | 'group';
  email?: string;
  max_uses?: number;
  expires_at?: string;
  community_id?: string;
  purpose?: string;
}
```

**Special Requirements:**

- Token generation (secure random)
- Rate limiting (max 10 invitations per day per user)
- Email validation for single_use tokens
- Audit trail of all uses
- Auto-assign to community if specified

**Time Estimate:** 3-4 days

---

### notification-service (Port 8005) - PRIORITY 4

**Frontend Dependencies:**

- Notification bell (header)
- Notification dropdown/center
- Notification preferences

**Endpoints:** (7 total)

- `GET /v1/notifications` - List notifications (paginated)
- `GET /v1/notifications/unread-count` - Get count for badge
- `POST /v1/notifications/:id/read` - Mark as read
- `POST /v1/notifications/read-all` - Mark all read
- `DELETE /v1/notifications/:id` - Delete notification
- `GET /v1/notifications/preferences` - Get preferences
- `PUT /v1/notifications/preferences` - Update preferences

**Database Tables:**

- `territory_dk.notifications`
- `territory_dk.notification_preferences` (shared with settings-service)
- `territory_dk.notification_templates`

**Data Models:**

```typescript
interface Notification {
  id: string;
  user_id: string;
  type: 'message' | 'follower' | 'community' | 'mention' | 'like' | 'system';
  title: string;
  message: string;
  action_url?: string;
  is_read: boolean;
  created_at: string;
}
```

**Special Requirements:**

- Real-time notifications (WebSocket or polling)
- Email digest (background job)
- Push notifications (future)
- Notification templates
- Batch operations (mark all read)

**Time Estimate:** 3-4 days

---

## Implementation Plan

### Phase A: Scaffold All Services (1 day)

**Goal:** All services compile, have health checks, and Swagger UI

For each service:

1. Create directory structure (copy from auth-service)
2. Create Cargo.toml (adjust name, port, dependencies)
3. Copy standard files:
   - `src/config.rs` (adjust port)
   - `src/error.rs` (same as auth-service)
   - `src/response.rs` (same as auth-service)
   - `src/lib.rs` (adjust exports)
   - `src/main.rs` (adjust service name, routes)
   - `src/openapi.rs` (adjust service info)
   - `src/models/health.rs` (same)
   - `src/handlers/health.rs` (same)
4. Add basic middleware/auth.rs (JWT validation)
5. Add to workspace Cargo.toml
6. Verify: `cargo build` succeeds
7. Test: health endpoint works, Swagger UI loads

**Result:** 10 services running on ports 8001-8013

---

### Phase B: Implement Priority Services (3-4 weeks)

**Week 1-2: user-service**

- Day 1-2: Profile CRUD + privacy filtering
- Day 3: Avatar upload + S3/IPFS integration
- Day 4: Profile links CRUD
- Day 5: User connections (follow/block)
- Day 6: Language proficiency
- Day 7-8: Account settings + TOTP
- Day 9: GDPR export/deletion
- Day 10: Testing + OpenAPI documentation

**Week 2-3: settings-service**

- Day 1: Appearance settings
- Day 2: Language settings
- Day 3: Notification settings (shared with notification-service)
- Day 4: Settings sync + conflict resolution
- Day 5: Testing + integration

**Week 3: invitation-service**

- Day 1-2: Token creation + validation
- Day 3: Usage tracking + audit
- Day 4: Community auto-assignment
- Day 5: Testing + rate limiting

**Week 4: notification-service**

- Day 1-2: Notification CRUD + unread count
- Day 3: Notification templates
- Day 4: Email digest (background job)
- Day 5: Testing + WebSocket (future)

---

### Phase C: Database Migrations

Create migrations in order:

1. `20251112000001_user_profile_tables.sql` - Profile, links, privacy
2. `20251112000002_user_connections.sql` - Connections, blocks
3. `20251112000003_language_proficiency.sql` - Language skills
4. `20251112000004_user_settings.sql` - Appearance, language, notifications
5. `20251112000005_account_security.sql` - TOTP, audit logs
6. `20251112000006_invitation_system.sql` - Invitation uses, audit
7. `20251112000007_notifications.sql` - Notifications, templates
8. `20251112000008_gdpr.sql` - Data exports, deletion logs

---

## Workspace Configuration

Update `services/Cargo.toml`:

```toml
[workspace]
members = [
    "shared-lib",
    "auth-service",
    "user-service",
    "settings-service",
    "invitation-service",
    "notification-service",
    # Phase 2 (scaffolded only)
    "community-service",
    "badge-service",
    "territory-service",
    "event-service",
    "course-service",
    "forum-service",
    "translation-service",
    "ipfs-service",
]
resolver = "2"

[workspace.package]
version = "0.1.0-alpha.1"
edition = "2021"
```

---

## Docker Compose Integration

Add to `docker-compose.dev.yml`:

```yaml
  user-service:
    build:
      context: ./services
      dockerfile: user-service/Dockerfile
    ports:
      - "8002:8002"
    environment:
      DATABASE_URL: ${DATABASE_URL}
      TERRITORY_CODE: dk
      SERVER_PORT: 8002
      JWT_SECRET: ${JWT_SECRET}
    depends_on:
      - postgres
      - auth-service

  settings-service:
    # ... port 8003

  invitation-service:
    # ... port 8004

  notification-service:
    # ... port 8005
```

---

## Testing Strategy

### Unit Tests

- Each service: `cargo test`
- Test business logic in `services/` modules

### Integration Tests

- Test full request/response cycle
- Use test database
- Mock external dependencies (S3, email)

### API Tests

- Use curl/Postman/Insomnia
- Test all endpoints
- Verify OpenAPI spec matches implementation

---

## Success Criteria

**Scaffolding Complete When:**

- ✅ All services compile without errors
- ✅ All health endpoints return 200
- ✅ All Swagger UIs accessible
- ✅ Can run all services simultaneously

**Phase 1 Complete When:**

- ✅ user-service: All 32 endpoints working
- ✅ settings-service: All 8 endpoints working
- ✅ invitation-service: All 6 endpoints working
- ✅ notification-service: All 7 endpoints working
- ✅ All database migrations applied
- ✅ Frontend integration complete
- ✅ All API tests passing

---

## Next Steps

1. **Review this plan** - Confirm approach and priorities
2. **Scaffold all services** - 1 day to get structure in place
3. **Start with user-service** - Most critical for frontend
4. **Iterate on remaining services** - Following priority order

**Ready to begin?** Start with Phase A: Scaffold all services.

---

**Related Documents:**

- [Backend API Index](../architecture/backend-api-index.md)
- [Backend API Requirements](../architecture/backend-api-requirements.md)
- [Database Schema Design](../architecture/database-schema-design.md)
- [Invitation System Database](../architecture/invitation-system-database.md)
