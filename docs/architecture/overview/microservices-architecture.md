# Microservices Architecture

**Version:** 0.1.0-alpha.1  
**Last Updated:** November 12, 2025  
**Status:** Authoritative Service Boundaries Definition  
**Purpose:** Define clear service boundaries for multi-pod scaling and Holochain migration

---

## 🎯 Architecture Principles

### 1. **Service Independence**

- Each service owns its data (bounded context)
- No direct database access across services
- Communication via APIs or message bus (NATS)

### 2. **Multi-Pod Ready**

- Each service can scale independently per territory
- Services communicate via territory-aware routing
- Shared-nothing architecture (except global registry)

### 3. **Holochain Migration Path**

- Service boundaries map to future Holochain DNAs
- Data models compatible with Holochain entry types
- Event sourcing patterns where applicable

### 4. **Clear Ownership**

- Each table belongs to exactly ONE service
- Cross-service queries use API calls, not direct SQL
- No overlapping functionality between services

---

## 🏗️ Service Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         FRONTEND (React)                         │
│                     TanStack Query + Router                      │
└────────────┬───────────────────────────────────────┬────────────┘
             │                                       │
             │ REST APIs                             │ WebSocket
             │                                       │
┌────────────▼───────────────────────────────────────▼────────────┐
│                      API Gateway (Traefik)                       │
│              Routing, Load Balancing, TLS, CORS                  │
└──┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬──────┘
   │    │    │    │    │    │    │    │    │    │    │    │
   ▼    ▼    ▼    ▼    ▼    ▼    ▼    ▼    ▼    ▼    ▼    ▼
┌────┐┌────┐┌────┐┌────┐┌────┐┌────┐┌────┐┌────┐┌────┐┌────┐┌────┐
│Auth││User││Sett││Invt││Ntfy││Cmty││Bdge││Terr││Evnt││Crs ││Frum│
│8001││8002││8003││8004││8005││8006││8007││8008││8009││8010││8011│
└─┬──┘└─┬──┘└─┬──┘└─┬──┘└─┬──┘└─┬──┘└─┬──┘└─┬──┘└─┬──┘└─┬──┘└─┬──┘
  │     │     │     │     │     │     │     │     │     │     │
  └─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
                              │
                     ┌────────▼────────┐
                     │  Message Bus    │
                     │   NATS/JetStream│
                     └────────┬────────┘
                              │
  ┌───────────────────────────┴───────────────────────────┐
  │                                                         │
┌─▼──────────────────┐                    ┌────────────────▼──┐
│ PostgreSQL         │                    │ Storage           │
│ - global schema    │                    │ - IPFS (future)   │
│ - territory_dk     │                    │ - S3/MinIO        │
│ - territory_no     │                    │ - Local files     │
└────────────────────┘                    └───────────────────┘
```

---

## 📋 Service Definitions

### **1. auth-service (Port 8001)** 🔐

**Bounded Context:** Authentication & Authorization

**Responsibilities:**

- User authentication (login/logout)
- JWT token generation and validation
- Password management (hashing, reset)
- Session management
- 2FA/TOTP (future)

**Database Tables (Owns):**

```sql
territory_{code}.users
  - id, username, email, password_hash, full_name
  - is_active, is_verified, verified_at
  - totp_enabled, totp_secret
  - deleted_at, created_at, updated_at

territory_{code}.user_sessions (future)
  - session_id, user_id, refresh_token
  - expires_at, ip_address, user_agent

territory_{code}.password_reset_tokens (future)
  - token, user_id, expires_at, used_at

global.username_registry
  - username, territory_code, user_id (uniqueness)

global.email_registry (if email provided)
  - email, territory_code, user_id (uniqueness)
```

**API Endpoints:**

```
POST   /api/v1/auth/register          - Register (requires invitation token from invitation-service)
POST   /api/v1/auth/login             - Authenticate user
POST   /api/v1/auth/logout            - Invalidate session
POST   /api/v1/auth/refresh           - Refresh access token
GET    /api/v1/auth/validate          - Validate token
POST   /api/v1/auth/password/reset    - Request password reset
POST   /api/v1/auth/password/confirm  - Confirm password reset
```

**Service Dependencies:**

- **Calls:** invitation-service (validate invitation during registration)
- **Called by:** All services (JWT validation)
- **Events:** Publishes `user.registered`, `user.logged_in`, `user.logged_out`

**Holochain Mapping:**

- DNA: `authentication`
- Entry Types: `User`, `Session`, `PasswordResetToken`

---

### **2. user-service (Port 8002)** 👤

**Bounded Context:** User Profile & Identity

**Responsibilities:**

- User profile management (display name, bio, avatar)
- Profile links (external social/web links)
- Language proficiency tracking
- User connections (follow/block)
- GDPR compliance (data export, account deletion)

**Database Tables (Owns):**

```sql
territory_{code}.users_profiles
  - user_id, display_name, avatar_url, bio, about
  - interests[], skills[], languages[]
  - location, created_at, updated_at

territory_{code}.users_profile_links
  - id, user_id, label, url, icon
  - display_order, is_visible

territory_{code}.users_language_proficiency
  - id, user_id, language_code, language_name
  - spoken_level, written_level, reading_level, listening_level
  - display_order, is_preferred, show_on_profile

territory_{code}.users_connections
  - user_id, target_user_id, connection_type (follow/friend/block)
  - status (active/pending/rejected)
  - created_at, updated_at

territory_{code}.data_exports (GDPR)
  - id, user_id, status, file_path, file_size
  - requested_at, completed_at, expires_at, downloaded_at

territory_{code}.account_deletion_requests (GDPR)
  - id, user_id, status, confirmation_token
  - requested_at, confirmed_at, scheduled_deletion_at
  - cancelled_at, cancellation_reason
```

**API Endpoints:**

```
# Profile
GET    /v1/profiles/{id}                    - Get profile
PUT    /v1/profiles/{id}                    - Update profile

# Profile Links
GET    /v1/profiles/{id}/links              - List links
POST   /v1/profiles/{id}/links              - Create link
PUT    /v1/profiles/{id}/links/{link_id}    - Update link
DELETE /v1/profiles/{id}/links/{link_id}    - Delete link
PATCH  /v1/profiles/{id}/links/reorder      - Reorder links

# Language Proficiency
GET    /v1/profiles/{id}/languages          - List languages
POST   /v1/profiles/{id}/languages          - Add language
PUT    /v1/profiles/{id}/languages/{lang_id} - Update language
DELETE /v1/profiles/{id}/languages/{lang_id} - Delete language

# Connections
POST   /v1/users/{id}/connections/follow/{target_id}   - Follow user
DELETE /v1/users/{id}/connections/follow/{target_id}   - Unfollow user
GET    /v1/users/{id}/connections/followers            - Get followers
GET    /v1/users/{id}/connections/following            - Get following
POST   /v1/users/{id}/connections/block/{target_id}    - Block user
DELETE /v1/users/{id}/connections/block/{target_id}    - Unblock user
GET    /v1/users/{id}/connections/blocked              - Get blocked users

# GDPR
POST   /v1/users/{id}/data/export                      - Request data export
GET    /v1/users/{id}/data/export                      - List exports
GET    /v1/users/{id}/data/export/{export_id}          - Download export
POST   /v1/users/{id}/account/delete                   - Request deletion
POST   /v1/users/{id}/account/delete/confirm           - Confirm deletion
GET    /v1/users/{id}/account/delete                   - Get deletion status
DELETE /v1/users/{id}/account/delete                   - Cancel deletion
```

**Service Dependencies:**

- **Calls:** settings-service (include settings in GDPR export)
- **Called by:** Frontend, other services (profile lookups)
- **Events:** Publishes `profile.updated`, `user.followed`, `user.blocked`

**Holochain Mapping:**

- DNA: `profiles`
- Entry Types: `Profile`, `ProfileLink`, `LanguageProficiency`, `Connection`

---

### **3. settings-service (Port 8003)** ⚙️

**Bounded Context:** User Preferences & Settings

**Responsibilities:**

- Appearance settings (theme, colors, layout)
- Language settings (locale, translation)
- Privacy settings (profile visibility)
- Settings synchronization across devices

**Database Tables (Owns):**

```sql
territory_{code}.users_settings
  - user_id, theme_mode, color_scheme
  - reduced_motion, wide_content_view, compact_mode
  - preferred_language, timezone
  - auto_translate, translation_provider
  - contribute_translations, fallback_to_english
  - created_at, updated_at

territory_{code}.users_privacy_settings
  - user_id, profile_visibility (public/followers/private)
  - show_email, show_location, show_connections
  - show_activity, allow_messages, allow_followers
  - created_at, updated_at
```

**API Endpoints:**

```
# User Settings
GET    /v1/settings/{user_id}               - Get all settings
PUT    /v1/settings/{user_id}               - Update settings (bulk)

# Appearance
GET    /v1/settings/{user_id}/appearance    - Get appearance settings
PATCH  /v1/settings/{user_id}/appearance    - Update appearance

# Language
GET    /v1/settings/{user_id}/language      - Get language settings
PATCH  /v1/settings/{user_id}/language      - Update language

# Privacy
GET    /v1/settings/{user_id}/privacy       - Get privacy settings
PATCH  /v1/settings/{user_id}/privacy       - Update privacy
```

**Service Dependencies:**

- **Calls:** None (leaf service)
- **Called by:** user-service (GDPR export), frontend
- **Events:** Publishes `settings.updated`

**Holochain Mapping:**

- DNA: `settings`
- Entry Types: `UserSettings`, `PrivacySettings`
- Note: Settings are user-sovereign, stored in user's source chain

---

### **4. invitation-service (Port 8004)** 🎫

**Bounded Context:** Invitation & Access Control

**Responsibilities:**

- Invitation token creation and management
- Token validation (for registration)
- Usage tracking and audit logs
- Community auto-assignment
- Rate limiting (invitations per user)

**Database Tables (Owns):**

```sql
territory_{code}.invitation_tokens
  - id, token, token_type (single_use/group)
  - email, max_uses, used_count
  - expires_at, community_id
  - created_by, purpose
  - is_active, created_at, updated_at

territory_{code}.invitation_uses
  - id, token_id, used_by_user_id
  - used_at, ip_address, user_agent

global.invitation_token_registry
  - token, territory_code (uniqueness across pods)
```

**API Endpoints:**

```
GET    /v1/invitations/validate/{token}     - Validate token (public, used by auth)
POST   /v1/invitations                      - Create invitation (authenticated)
GET    /v1/invitations                      - List my invitations
GET    /v1/invitations/{id}                 - Get invitation details
DELETE /v1/invitations/{id}                 - Revoke invitation
GET    /v1/invitations/{id}/uses            - Get usage audit log
POST   /v1/invitations/{token}/use          - Mark as used (internal, called by auth)
```

**Service Dependencies:**

- **Calls:** None (leaf service)
- **Called by:** auth-service (during registration), frontend (admin panel)
- **Events:** Publishes `invitation.created`, `invitation.used`, `invitation.revoked`

**Holochain Mapping:**

- DNA: `invitations`
- Entry Types: `InvitationToken`, `InvitationUse`
- Note: Critical for territory sovereignty and access control

---

### **5. notification-service (Port 8005)** 🔔

**Bounded Context:** Notifications & Alerts

**Responsibilities:**

- In-app notification delivery
- Email notifications (via SMTP)
- Push notifications (future)
- Notification preferences management
- Notification templates
- Email digest generation

**Database Tables (Owns):**

```sql
territory_{code}.notifications
  - id, user_id, type (message/follower/community/mention/like/system)
  - title, message, action_url
  - is_read, read_at
  - metadata (JSON - service-specific data)
  - created_at, expires_at

territory_{code}.users_notification_settings
  - user_id
  - email_digest, email_messages, email_followers, email_community, email_updates
  - inapp_messages, inapp_followers, inapp_community, inapp_mentions, inapp_likes
  - push_enabled, push_messages, push_followers, push_community
  - created_at, updated_at

territory_{code}.notification_templates
  - id, template_key, language_code
  - subject_template, body_template
  - notification_type, created_at, updated_at
```

**API Endpoints:**

```
# Notifications
GET    /v1/notifications                    - List notifications (paginated)
GET    /v1/notifications/unread-count       - Get unread count
POST   /v1/notifications/{id}/read          - Mark as read
POST   /v1/notifications/read-all           - Mark all as read
DELETE /v1/notifications/{id}               - Delete notification

# Settings
GET    /v1/notifications/settings/{user_id} - Get notification preferences
PUT    /v1/notifications/settings/{user_id} - Update preferences

# Internal (called by other services)
POST   /v1/notifications/send               - Create notification (internal)
```

**Service Dependencies:**

- **Calls:** None (leaf service)
- **Called by:** All services (via NATS events or direct API)
- **Events:** Subscribes to all `*.created`, `*.updated` events from other services

**Holochain Mapping:**

- DNA: `notifications`
- Entry Types: `Notification`, `NotificationSettings`
- Note: Notifications are ephemeral, may not need full Holochain persistence

---

### **6. community-service (Port 8006)** 👥

**Bounded Context:** Communities & Groups

**Responsibilities:**

- Community creation and management
- Membership management
- Community settings and permissions
- Community roles (admin/moderator/member)

**Database Tables (Owns):**

```sql
territory_{code}.communities
  - id, name, slug, description
  - avatar_url, banner_url
  - is_public, member_count
  - created_by, created_at, updated_at

territory_{code}.community_members
  - community_id, user_id
  - role (admin/moderator/member)
  - joined_at, invited_by

territory_{code}.community_settings
  - community_id
  - allow_join_requests, require_approval
  - allow_invitations, allow_public_posts
```

**API Endpoints:**

```
GET    /v1/communities                      - List communities
POST   /v1/communities                      - Create community
GET    /v1/communities/{id}                 - Get community
PUT    /v1/communities/{id}                 - Update community
DELETE /v1/communities/{id}                 - Delete community
GET    /v1/communities/{id}/members         - List members
POST   /v1/communities/{id}/members/{user_id} - Add member
DELETE /v1/communities/{id}/members/{user_id} - Remove member
```

**Service Dependencies:**

- **Calls:** user-service (profile lookups), notification-service (member notifications)
- **Called by:** Frontend, invitation-service (community auto-assignment)
- **Events:** Publishes `community.created`, `member.joined`, `member.removed`

**Holochain Mapping:**

- DNA: `communities`
- Entry Types: `Community`, `CommunityMember`, `CommunitySettings`

---

### **7. badge-service (Port 8007)** 🏅

**Bounded Context:** Gamification & Achievements

**Responsibilities:**

- Badge definitions and metadata
- Badge awarding logic
- Achievement tracking
- Progress calculation
- Badge display and verification

**Database Tables (Owns):**

```sql
territory_{code}.badges
  - id, name, description, icon_url
  - badge_type (achievement/role/participation)
  - criteria (JSON - requirements)
  - is_active, created_at

territory_{code}.user_badges
  - user_id, badge_id
  - awarded_at, awarded_by
  - expires_at, revoked_at

territory_{code}.badge_progress
  - user_id, badge_id
  - current_value, target_value
  - percentage, updated_at
```

**API Endpoints:**

```
GET    /v1/badges                           - List all badges
GET    /v1/badges/{id}                      - Get badge details
POST   /v1/badges                           - Create badge (admin)
GET    /v1/users/{user_id}/badges           - Get user's badges
POST   /v1/badges/{id}/award                - Award badge (internal)
GET    /v1/users/{user_id}/progress/{badge_id} - Get progress
```

**Service Dependencies:**

- **Calls:** user-service (user verification), notification-service (award notifications)
- **Called by:** All services (via events for automatic badge awards)
- **Events:** Publishes `badge.awarded`, subscribes to achievement triggers

**Holochain Mapping:**

- DNA: `badges`
- Entry Types: `Badge`, `UserBadge`, `BadgeProgress`

---

### **8. territory-service (Port 8008)** 🌍

**Bounded Context:** Territory & Pod Management

**Responsibilities:**

- Territory registration and configuration
- Pod deployment and management
- Territory-level settings
- Inter-pod federation
- Territory member directory

**Database Tables (Owns):**

```sql
global.territories
  - code (ISO 3166-1 Alpha-2), name, full_name
  - region, currency, timezone
  - pod_url, admin_contact
  - is_active, created_at

territory_{code}.territory_settings
  - default_language, supported_languages[]
  - registration_mode (open/invite_only/closed)
  - features_enabled (JSON)

territory_{code}.territory_stats
  - total_users, active_users
  - total_communities, total_courses
  - updated_at
```

**API Endpoints:**

```
GET    /v1/territories                      - List all territories
GET    /v1/territories/{code}               - Get territory details
PUT    /v1/territories/{code}/settings      - Update settings (admin)
GET    /v1/territories/{code}/stats         - Get statistics
```

**Service Dependencies:**

- **Calls:** None (top-level service)
- **Called by:** auth-service (territory validation), frontend
- **Events:** Publishes `territory.created`, `territory.updated`

**Holochain Mapping:**

- DNA: `territories` (meta-level)
- Entry Types: `Territory`, `TerritorySettings`
- Note: Critical for multi-pod federation

---

## 📊 Service Dependency Matrix

```
Service          → Depends On
─────────────────────────────────────────────────────
auth             → invitation
user             → settings (GDPR export)
settings         → (none - leaf)
invitation       → (none - leaf)
notification     → (none - leaf, but subscribes to events)
community        → user, notification
badge            → user, notification
territory        → (none - top-level)
event            → community, user, notification
course           → community, user, notification, badge
forum            → community, user, notification
translation      → (none - leaf)
```

**Key Principles:**

- Leaf services have no dependencies (can deploy independently)
- Higher-level services depend on lower-level services
- No circular dependencies
- Communication via REST APIs or NATS events

---

## 🔄 Inter-Service Communication

### **Synchronous (REST API)**

Used for: Queries, lookups, immediate responses

Example:

```
auth-service → invitation-service.validateToken(token)
user-service → settings-service.getSettings(user_id)
community-service → user-service.getProfile(user_id)
```

### **Asynchronous (NATS Events)**

Used for: Notifications, side effects, eventual consistency

Example:

```
user-service publishes: { event: "user.followed", user_id, target_id }
notification-service subscribes → creates notification
badge-service subscribes → checks if badge criteria met
```

---

## 🗄️ Database Architecture

### **Global Schema** (shared across territories)

```sql
global.territories               -- Territory registry
global.username_registry         -- Username uniqueness
global.email_registry           -- Email uniqueness (if provided)
global.invitation_token_registry -- Token uniqueness
```

### **Territory Schema** (per pod: territory_dk, territory_no, etc.)

Each service owns its tables within the territory schema:

```
territory_{code}.users                      -- auth-service
territory_{code}.users_profiles             -- user-service
territory_{code}.users_profile_links        -- user-service
territory_{code}.users_language_proficiency -- user-service
territory_{code}.users_connections          -- user-service
territory_{code}.data_exports               -- user-service
territory_{code}.account_deletion_requests  -- user-service
territory_{code}.users_settings             -- settings-service
territory_{code}.users_privacy_settings     -- settings-service
territory_{code}.users_notification_settings -- notification-service
territory_{code}.notifications              -- notification-service
territory_{code}.notification_templates     -- notification-service
territory_{code}.invitation_tokens          -- invitation-service
territory_{code}.invitation_uses            -- invitation-service
territory_{code}.communities                -- community-service
territory_{code}.community_members          -- community-service
territory_{code}.badges                     -- badge-service
territory_{code}.user_badges                -- badge-service
```

**Rule:** Each service ONLY writes to its own tables. Cross-service data access via APIs.

---

## 🚀 Multi-Pod Deployment

### **Per-Territory Deployment**

Each territory (Denmark, Norway, Sweden, Europe) runs:

- Full set of microservices (auth, user, settings, etc.)
- Territory-specific database schema
- Isolated message bus (NATS)
- Shared global registry (replicated)

### **Service Discovery**

```
Denmark Pod:
  auth.dk.unityplatform.org → auth-service:8001 (territory_dk)
  user.dk.unityplatform.org → user-service:8002 (territory_dk)

Norway Pod:
  auth.no.unityplatform.org → auth-service:8001 (territory_no)
  user.no.unityplatform.org → user-service:8002 (territory_no)
```

### **Cross-Pod Federation**

```
User in DK follows user in NO:
1. user-service (DK) → makes API call to user-service (NO)
2. Connection stored in territory_dk.users_connections
3. Both services publish events to their local NATS
4. Global event bus replicates across pods (future)
```

---

## 🔮 Holochain Migration Path

### **Phase 1: PostgreSQL (Current)**

- Microservices with PostgreSQL
- REST APIs
- NATS for async communication

### **Phase 2: Hybrid**

- Keep PostgreSQL for queries (read model)
- Add Holochain for writes (source of truth)
- Event sourcing pattern

### **Phase 3: Holochain Native**

- Each service becomes a Holochain DNA
- PostgreSQL becomes optional (caching/indexing)
- P2P communication between DNAs

### **Service → DNA Mapping**

```
auth-service        → authentication.happ
user-service        → profiles.happ
settings-service    → settings.happ (private chain)
invitation-service  → invitations.happ
notification-service → notifications.happ (ephemeral)
community-service   → communities.happ
badge-service       → badges.happ
```

---

## ✅ Migration Checklist

To move from current consolidated approach to proper microservices:

### **Phase 1: Settings Separation** (Week 1)

- [ ] Create settings-service handlers
- [ ] Move users_settings tables to settings-service ownership
- [ ] Move users_privacy_settings to settings-service
- [ ] Update user-service to call settings-service API
- [ ] Update GDPR export to call settings-service
- [ ] Update frontend to call settings-service endpoints

### **Phase 2: Notification Separation** (Week 2)

- [ ] Create notification-service handlers
- [ ] Move users_notification_settings to notification-service
- [ ] Create notifications table
- [ ] Create notification templates
- [ ] Implement NATS event subscriptions
- [ ] Update frontend notification components

### **Phase 3: Invitation Separation** (Week 2)

- [ ] Move invitation logic from auth-service to invitation-service
- [ ] Update auth-service to call invitation-service API
- [ ] Create invitation admin panel endpoints
- [ ] Implement usage tracking
- [ ] Add rate limiting

### **Phase 4: Integration & Testing** (Week 3)

- [ ] End-to-end testing of all services
- [ ] Performance testing (multi-service calls)
- [ ] Documentation updates
- [ ] Docker Compose multi-service deployment
- [ ] Monitoring and observability setup

---

## 📝 Documentation Structure

Each service should have:

```
docs/architecture/services/{service-name}/
  ├── README.md                 # Service overview
  ├── database-schema.md        # Tables owned by this service
  ├── api-specification.md      # OpenAPI/REST endpoints
  ├── data-models.md            # Rust structs, validation rules
  ├── dependencies.md           # Services this one depends on
  ├── events.md                 # NATS events published/subscribed
  └── holochain-migration.md    # Holochain DNA design
```

---

**Next Steps:**

1. Create per-service documentation folders
2. Extract current implementations into proper service boundaries
3. Update database migrations to reflect service ownership
4. Implement missing services (settings, notification, invitation)
5. Test multi-service integration
6. Deploy to multi-pod architecture

This architecture ensures:

- ✅ Clean service boundaries (no overlaps)
- ✅ Multi-pod scalability
- ✅ Holochain migration path
- ✅ Independent service deployment
- ✅ Clear ownership and responsibility
