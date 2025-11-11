# Backend Development - Requirements Summary

**Status:** Requirements Complete - Ready for Implementation  
**Created:** November 11, 2025  
**Next Phase:** Database Schema Implementation  
**Target:** PostgreSQL → Holochain Migration Path

---

## Documents Created

### 1. [Backend API Requirements](backend-api-requirements.md) ⭐ MAIN REFERENCE

**What it covers:**

- Complete API endpoint specifications for all frontend features
- Data models for User, Profile, Settings, Invitations
- Authentication flow (JWT tokens, refresh, login, register)
- Profile management (CRUD, privacy, avatar, location)
- Settings management (appearance, privacy, notifications, account)
- Invitation system (token types, validation, creation)
- GDPR compliance (data export, account deletion)
- Holochain migration considerations

**Use this for:**

- Understanding what APIs need to be built
- Frontend-backend contract definitions
- Feature requirements from user perspective

### 2. [Database Schema Design](database-schema-design.md) 🗄️ DATABASE ARCHITECTURE

**What it covers:**

- Complete PostgreSQL schema for multi-tenant architecture
- Global schema (territories, username registry, email registry, token registry)
- Territory schema (users, profiles, settings, invitations, communities)
- Indexes, constraints, triggers, and functions
- Holochain Entry Type mappings
- Migration strategy (5 phases: PostgreSQL → Dual-Write → Dual-Read → Holochain Primary → Full Holochain)

**Use this for:**

- Writing SQL migration scripts
- Understanding data relationships
- Planning Holochain DNA structure
- Database optimization

### 3. [Invitation System Database](invitation-system-database.md) 🎫 IMPLEMENTATION GUIDE

**What it covers:**

- Detailed SQL scripts for invitation tables
- Triggers and stored procedures
- Token generation and validation
- Usage tracking and audit logs
- Security considerations (rate limiting, email validation)
- Testing strategies
- Performance optimization (partitioning, archiving)

**Use this for:**

- Implementing invitation system database layer
- Writing backend API endpoints for invitations
- Testing invitation workflows
- Security hardening

---

## Frontend Features Analyzed

The requirements were extracted from these implemented frontend components:

### Authentication & User Management

- ✅ Login with username (privacy-first)
- ✅ Register with invitation token (territory-bound)
- ✅ User profile entity (id, username, email, full_name, territory_code)
- ✅ JWT authentication flow

### User Profile

- ✅ Rich profile (bio, about, avatar, interests, skills, languages)
- ✅ Location picker with geocoding (encoded format: `[lat,lng]Display Name`)
- ✅ Profile links (flexible external links system)
- ✅ Privacy settings (profile visibility, field visibility, message permissions)

### User Settings

- ✅ Appearance settings (theme, reduced motion, wide content, compact mode)
- ✅ Privacy settings (profile visibility, show email/name/location/connections, message permissions)
- ✅ Notification settings (email, in-app, push - granular controls)
- ✅ Account settings (email change, password change, 2FA/TOTP)
- ✅ Data management (GDPR export, account deletion)

### Invitation System

- ✅ Single-use tokens (personal invitations with email binding)
- ✅ Group tokens (multi-use for courses/workshops/communities)
- ✅ Territory binding (database-enforced, client cannot manipulate)
- ✅ Community auto-assignment (optional)
- ✅ Usage tracking and audit logs

---

## Data Models Summary

### Core Entities

```typescript
// Authentication
User {
  id: UUID
  username: string (unique globally - primary identifier)
  email?: string | null (optional - for external notifications only)
  password_hash: string
  territory_code: string
  is_active: boolean
  created_at: timestamp
}

// Profile
UserProfile {
  user_id: UUID (FK)
  display_name: string
  avatar_url: string
  bio: string (280 chars)
  about: string (long-form Markdown)
  interests: string[]
  skills: string[]
  languages: string[] (ISO 639-1)
  location: string (encoded: "[lat,lng]Name")
}

ProfileLink {
  id: UUID
  user_id: UUID (FK)
  label: string
  url: string
  icon: string
  display_order: number
  is_visible: boolean
}

// Settings
PrivacySettings {
  user_id: UUID (FK)
  profile_visibility: 'public' | 'connections_only' | 'private'
  show_email: boolean
  show_full_name: boolean
  show_location: boolean
  show_connections: boolean
  allow_messages_from: 'everyone' | 'connections_only' | 'nobody'
}

AppearanceSettings {
  user_id: UUID (FK)
  theme: 'light' | 'dark' | 'system'
  reduced_motion: boolean
  wide_content_view: boolean
  compact_mode: boolean
}

NotificationSettings {
  user_id: UUID (FK)
  email_digest: boolean
  email_messages: boolean
  // ... (12 more notification toggles)
}

// Invitations
InvitationToken {
  id: UUID
  token: string (unique, cryptographic)
  token_type: 'single_use' | 'group'
  email: string (for single_use)
  max_uses: number
  used_count: number
  created_by_user_id: UUID (FK)
  community_id: UUID (FK, optional)
  expires_at: timestamp
  is_active: boolean
}
```

---

## Database Structure

### Global Schema (Cross-Territory)

```
global
├── territories                    -- Pod registry
├── username_registry              -- Ensure global uniqueness
├── email_registry                 -- Ensure global uniqueness
└── invitation_token_registry      -- Territory binding for tokens
```

### Territory Schema (Per Pod)

```
territory_dk (Denmark)
├── users                          -- Authentication
├── profiles                       -- User profiles
├── profile_links                  -- External links
├── privacy_settings               -- Privacy preferences
├── user_settings                  -- App preferences
├── notification_settings          -- Notification preferences
├── invitation_tokens              -- Territory-local tokens
├── invitation_uses                -- Audit trail
├── communities                    -- (Future)
├── posts                          -- (Future)
└── messages                       -- (Future)
```

**Replicate for:** `territory_no`, `territory_se`, `territory_eu`

---

## API Endpoints Summary

### Authentication

```
POST   /api/v1/auth/login               -- Login with username/password
POST   /api/v1/auth/register            -- Register with invitation token
POST   /api/v1/auth/logout              -- Invalidate tokens
POST   /api/v1/auth/refresh             -- Refresh access token
GET    /api/v1/auth/me                  -- Get current user
POST   /api/v1/auth/verify-email        -- Verify email
GET    /api/v1/auth/check-username      -- Check availability
GET    /api/v1/auth/check-email         -- Check availability
```

### Profile Management

```
GET    /api/v1/profiles/{userId}        -- Get user profile
GET    /api/v1/profiles/{userId}/full   -- Get full profile (own)
PUT    /api/v1/profiles/{userId}        -- Update profile
POST   /api/v1/avatars/{userId}         -- Upload avatar
DELETE /api/v1/avatars/{userId}         -- Delete avatar
```

### Profile Links

```
GET    /api/v1/profiles/{userId}/links           -- List links
POST   /api/v1/profiles/{userId}/links           -- Create link
PUT    /api/v1/profiles/{userId}/links/{linkId}  -- Update link
DELETE /api/v1/profiles/{userId}/links/{linkId}  -- Delete link
PATCH  /api/v1/profiles/{userId}/links/reorder   -- Reorder links
```

### Settings

```
GET    /api/v1/settings/privacy          -- Get privacy settings
PATCH  /api/v1/settings/privacy          -- Update privacy
GET    /api/v1/settings/notifications    -- Get notification settings
PATCH  /api/v1/settings/notifications    -- Update notifications
POST   /api/v1/account/email/change      -- Change email
POST   /api/v1/account/password/change   -- Change password
POST   /api/v1/account/totp/enable       -- Enable 2FA
```

### Invitations

```
GET    /api/v1/invitations/validate/{token}  -- Validate & get territory
POST   /api/v1/invitations/create            -- Create invitation
GET    /api/v1/invitations/my-invitations    -- List created invitations
DELETE /api/v1/invitations/{tokenId}         -- Revoke invitation
GET    /api/v1/invitations/{tokenId}/uses    -- View usage
```

### Data Management (GDPR)

```
POST   /api/v1/data/export                -- Request data export
GET    /api/v1/data/export/status         -- Check status
GET    /api/v1/data/export/download       -- Download export
POST   /api/v1/account/delete             -- Delete account
```

---

## Holochain Migration Path

### Phase 1: PostgreSQL (Current) ✅ FOCUS NOW

- Implement complete SQL schema
- Build REST APIs in Rust
- Test with frontend
- Optimize performance

### Phase 2: Dual-Write (3-6 months)

- Set up Holochain DNAs (one per territory)
- Write to both PostgreSQL + Holochain
- Validate data consistency

### Phase 3: Dual-Read (6-9 months)

- Read from Holochain with PostgreSQL fallback
- Gradual traffic shift
- Performance tuning

### Phase 4: Holochain Primary (9-12 months)

- Holochain as primary data source
- PostgreSQL as archive/analytics

### Phase 5: Full Holochain (12+ months)

- Complete migration
- PostgreSQL archive only
- User data sovereignty achieved! 🎉

---

## Implementation Priority

### Priority 1: Authentication & Core Infrastructure (Week 1-2)

- ✅ Global schema (territories, registries)
- ✅ Territory schema (users table)
- ✅ JWT authentication
- ✅ User registration with invitation validation
- ✅ Login/logout/refresh endpoints

**Why first:** Blocks all other work

### Priority 2: User Profile (Week 2-3)

- ✅ Profile CRUD
- ✅ Avatar upload/delete
- ✅ Location encoding
- ✅ Profile links system
- ✅ Privacy filtering

**Why second:** User-facing features, high visibility

### Priority 3: Invitation System (Week 3-4)

- ✅ Token creation (single_use, group)
- ✅ Token validation
- ✅ Usage tracking
- ✅ Community auto-assignment

**Why third:** Enables community growth

### Priority 4: Settings Management (Week 4-5)

- ✅ Privacy settings
- ✅ Notification settings
- ✅ Appearance settings (sync across devices)
- ✅ Account settings (email, password, 2FA)

**Why fourth:** User sovereignty features

### Priority 5: GDPR Compliance (Week 5-6)

- ✅ Data export
- ✅ Account deletion
- ✅ Audit logs

**Why fifth:** Legal compliance

---

## Security Considerations

### Token Security

- ✅ Cryptographically random tokens (gen_random_bytes)
- ✅ Rate limiting on validation (10 attempts per 15 min)
- ✅ Email verification for single-use tokens
- ✅ Territory binding in database (prevent manipulation)

### Authentication Security

- ✅ Password hashing (bcrypt/argon2)
- ✅ JWT with short expiry (15 min access, 7 day refresh)
- ✅ Token rotation on refresh
- ✅ Token blacklist for logout
- ✅ 2FA/TOTP support

### Privacy & Data Protection

- ✅ Privacy-aware profile queries (filter based on relationship)
- ✅ GDPR data export (machine-readable JSON)
- ✅ Right to deletion (soft delete + anonymization)
- ✅ Audit logs (immutable append-only)

---

## Next Steps

### Immediate Actions (This Week)

1. **Create SQL Migration Scripts** 📝
   - Global schema: territories, registries
   - Territory schema template: users, profiles, settings, invitations
   - Triggers and functions
   - Seed data for testing

2. **Set Up Database** 🗄️
   - Create databases for each territory pod
   - Run migrations
   - Create test users
   - Create test invitation tokens

3. **Start Backend Implementation** 🦀
   - Set up Rust workspace for auth-service
   - Implement JWT middleware
   - Create database connection pool (SQLx)
   - Implement /auth/login endpoint

### Week 2-3

4. **Implement Authentication APIs**
   - Registration with invitation validation
   - Login/logout/refresh
   - Email verification
   - Username/email availability checks

5. **Implement Profile APIs**
   - Profile CRUD
   - Avatar upload (S3/IPFS)
   - Privacy filtering
   - Profile links CRUD

### Week 4-6

6. **Complete Remaining APIs**
   - Invitation system
   - Settings management
   - GDPR compliance

7. **Frontend Integration**
   - Replace mock localStorage with real API calls
   - Test all flows
   - Error handling

8. **Testing & Documentation**
   - Unit tests
   - Integration tests
   - API documentation (OpenAPI/Swagger)

---

## Questions to Resolve

Before starting implementation, decide on:

1. **Password Hashing:** bcrypt (standard) or argon2 (more secure)?
2. **File Storage:** S3-compatible (MinIO) or IPFS for avatars?
3. **Email Service:** Which provider for verification emails?
4. **Rate Limiting:** Redis or database for tracking?
5. **Token Blacklist:** Redis (fast) or database (persistent)?
6. **Background Jobs:** Which cron system for expired tokens cleanup?
7. **Logging:** tracing, slog, or env_logger?

---

## Success Metrics

### Phase 1 Complete When

- ✅ All tables created with constraints and indexes
- ✅ All authentication endpoints working
- ✅ Users can register with invitation tokens
- ✅ Users can log in and access protected endpoints
- ✅ Profile CRUD works with privacy filtering
- ✅ Settings sync across devices
- ✅ Invitation system creates and validates tokens
- ✅ GDPR export/deletion implemented
- ✅ Frontend integration complete (no more localStorage mocks)
- ✅ All tests passing

---

## Related Documents

- [Backend API Requirements](backend-api-requirements.md) - Complete API specification
- [Database Schema Design](database-schema-design.md) - SQL schema with Holochain mapping
- [Invitation System Database](invitation-system-database.md) - Implementation guide
- [Invitation System Architecture](invitation-system.md) - Original architecture doc
- [Multi-Pod Architecture](multi-pod-architecture.md) - Territory deployment model
- [User Data Sovereignty](user-data-sovereignty.md) - User ownership principles

---

**Status:** ✅ Requirements documentation complete  
**Next:** Database schema implementation  
**Timeline:** 6-8 weeks to MVP backend (Phase 1)  
**Long-term:** 12+ months to full Holochain migration
