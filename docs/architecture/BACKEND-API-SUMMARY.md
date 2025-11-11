# Backend API Documentation - Summary

**Status:** ✅ Requirements Complete - Ready for Implementation  
**Created:** November 11, 2025  
**Purpose:** Define backend API requirements based on implemented frontend features  
**Next Phase:** Database schema implementation and API development

---

## 📚 What Was Created

I've created comprehensive documentation for the backend API requirements based on all the frontend features you've implemented. This documentation is designed to guide the database schema design and API development with Holochain migration in mind.

### Documents Created

1. **[Backend API Index](backend-api-index.md)** 🗂️
   - Navigation guide for all documentation
   - Quick reference for data models and endpoints
   - How-to guide for different roles

2. **[Backend Requirements Summary](backend-requirements-summary.md)** 🎯
   - High-level overview of all requirements
   - Implementation roadmap (6-week plan)
   - Success criteria and priorities

3. **[Backend API Requirements](backend-api-requirements.md)** 📋
   - Complete API endpoint specifications
   - Data models for all entities
   - Request/response examples
   - Authentication flow details

4. **[Database Schema Design](database-schema-design.md)** 🗄️
   - Complete PostgreSQL schema
   - Holochain Entry Type mappings
   - 5-phase migration strategy
   - Triggers, functions, and constraints

5. **[Invitation System Database](invitation-system-database.md)** 🎫
   - Detailed SQL implementation
   - Token generation and validation
   - Security considerations
   - Testing and optimization

---

## 🎯 Key Features Documented

### From Your Frontend Implementation

#### ✅ Authentication & User Management

- Login with username (privacy-first approach)
- Registration with invitation tokens (territory-bound)
- JWT authentication flow
- Email verification

#### ✅ User Profile System

- Rich profiles (bio, about, avatar, interests, skills, languages)
- **Interactive map location picker** with geocoding (your recent work!)
- Flexible profile links system (replaces hardcoded social links)
- Privacy-aware profile visibility

#### ✅ User Settings

- **Appearance:** Theme, reduced motion, wide content, compact mode
- **Privacy:** Profile visibility, field visibility, message permissions
- **Notifications:** Email, in-app, push (granular controls)
- **Account:** Email change, password change, 2FA/TOTP
- **Data Management:** GDPR export, account deletion

#### ✅ Invitation System

- Single-use tokens (personal invitations)
- Group tokens (multi-use for communities/courses)
- Territory binding (database-enforced security)
- Community auto-assignment
- Usage tracking and audit logs

---

## 🗄️ Database Architecture

### Multi-Tenant Structure

```
Global Schema (Cross-Territory)
├── territories              -- Pod registry
├── username_registry        -- Global uniqueness
├── email_registry          -- Global uniqueness
└── invitation_token_registry -- Territory binding

Territory Schema (Per Pod)
├── users                          -- Authentication
├── users_profiles                 -- User profiles
├── users_profile_links            -- External links
├── users_privacy_settings         -- Privacy preferences
├── users_settings                 -- App preferences
├── users_notification_settings    -- Notification prefs
├── users_audit_logs               -- User audit trail
├── invitation_tokens              -- Territory-local tokens
└── invitation_uses                -- Invitation audit trail
```

### Holochain Migration Ready

Each PostgreSQL table maps to Holochain Entry Types:

- **Users** → Agent (built-in)
- **Profiles** → Profile Entry
- **Profile Links** → ProfileLink Entry
- **Invitation Tokens** → InvitationToken Entry
- **Settings** → Private source chain entries

---

## 🚀 Implementation Plan

### Phase 1: PostgreSQL Foundation (6-8 weeks)

#### Week 1-2: Authentication & Infrastructure ⏳

- [ ] Create global schema (territories, registries)
- [ ] Create territory schema template
- [ ] Implement JWT authentication
- [ ] Build registration with invitation validation
- [ ] Build login/logout/refresh endpoints

#### Week 2-3: User Profile 👤

- [ ] Profile CRUD operations
- [ ] Avatar upload/delete (S3 or IPFS)
- [ ] Privacy filtering logic
- [ ] Profile links system

#### Week 3-4: Invitation System 🎫

- [ ] Token creation (single_use, group)
- [ ] Token validation flow
- [ ] Usage tracking
- [ ] Community auto-assignment

#### Week 4-5: Settings Management ⚙️

- [ ] Privacy settings API
- [ ] Notification settings API
- [ ] Appearance settings sync
- [ ] Account settings (email, password, 2FA)

#### Week 5-6: GDPR Compliance 📜

- [ ] Data export system
- [ ] Account deletion workflow
- [ ] Audit logging

#### Week 6-8: Integration & Testing 🧪

- [ ] Frontend integration (replace localStorage)
- [ ] End-to-end testing
- [ ] Performance optimization
- [ ] Documentation (OpenAPI/Swagger)

### Phase 2-5: Holochain Migration (12+ months)

- Phase 2: Dual-Write (PostgreSQL + Holochain)
- Phase 3: Dual-Read (Holochain primary, PostgreSQL fallback)
- Phase 4: Holochain Primary
- Phase 5: Full Holochain (PostgreSQL archive only)

---

## 📊 API Endpoint Summary

### 32 Endpoints Across 7 Categories

```http
Authentication (8 endpoints)
├── POST   /api/v1/auth/login
├── POST   /api/v1/auth/register
├── POST   /api/v1/auth/logout
├── POST   /api/v1/auth/refresh
├── GET    /api/v1/auth/me
├── POST   /api/v1/auth/verify-email
├── GET    /api/v1/auth/check-username
└── GET    /api/v1/auth/check-email

Profile Management (5 endpoints)
├── GET    /api/v1/profiles/{userId}
├── GET    /api/v1/profiles/{userId}/full
├── PUT    /api/v1/profiles/{userId}
├── POST   /api/v1/avatars/{userId}
└── DELETE /api/v1/avatars/{userId}

Profile Links (5 endpoints)
├── GET    /api/v1/profiles/{userId}/links
├── POST   /api/v1/profiles/{userId}/links
├── PUT    /api/v1/profiles/{userId}/links/{linkId}
├── DELETE /api/v1/profiles/{userId}/links/{linkId}
└── PATCH  /api/v1/profiles/{userId}/links/reorder

Settings (4 endpoints)
├── GET/PATCH  /api/v1/settings/privacy
└── GET/PATCH  /api/v1/settings/notifications

Account (5 endpoints)
├── POST   /api/v1/account/email/change
├── POST   /api/v1/account/password/change
├── POST   /api/v1/account/totp/enable
├── POST   /api/v1/account/totp/verify
└── POST   /api/v1/account/totp/disable

Invitations (5 endpoints)
├── GET    /api/v1/invitations/validate/{token}
├── POST   /api/v1/invitations/create
├── GET    /api/v1/invitations/my-invitations
├── DELETE /api/v1/invitations/{tokenId}
└── GET    /api/v1/invitations/{tokenId}/uses

Data Management (3 endpoints)
├── POST   /api/v1/data/export
├── GET    /api/v1/data/export/status
└── POST   /api/v1/account/delete
```

---

## 🔐 Security Highlights

### Territory Binding

- Invitation tokens bound to territories in database
- Client **cannot manipulate** territory selection
- Global registry enforces security

### Authentication

- JWT with short expiry (15 min access, 7 day refresh)
- Token rotation on refresh
- Password hashing (bcrypt/argon2)
- 2FA/TOTP support

### Privacy

- Privacy-aware queries (filter by relationship)
- GDPR data export (machine-readable JSON)
- Right to deletion (soft delete + anonymization)
- Audit logs (immutable)

---

## 🎓 Special Features

### Location Encoding System

Your map location picker uses a special encoding format:

**Format:** `[latitude,longitude]Display Name`

**Example:** `[55.6761,12.5683]Havndal, Randers Municipality, Denmark`

**Backend Parsing:**

```typescript
// Extract display name for UI
function getLocationDisplayName(location: string): string {
  const match = location.match(/\](.+)$/);
  return match ? match[1] : location;
}

// Extract coordinates for geospatial queries
function parseLocationCoordinates(location: string): [number, number] | null {
  const match = location.match(/\[(-?\d+\.?\d*),(-?\d+\.?\d*)\]/);
  if (!match) return null;
  return [parseFloat(match[1]), parseFloat(match[2])];
}
```

**Database Options:**

1. Store as TEXT (simple, works now)
2. Add PostGIS POINT column (enables geospatial queries like "find users near me")

---

## 📖 How to Use This Documentation

### For You (Project Lead)

1. Review [Backend Requirements Summary](backend-requirements-summary.md) for timeline
2. Use this as a specification for backend developers
3. Track progress against the 6-week plan

### For Backend Developers (Future)

1. Start with [Backend API Index](backend-api-index.md) for navigation
2. Use [Database Schema Design](database-schema-design.md) to create migrations
3. Reference [Backend API Requirements](backend-api-requirements.md) while building
4. Follow [Invitation System Database](invitation-system-database.md) for invitations

### For Frontend Integration

1. No changes needed to your current implementation
2. When backend is ready, replace localStorage with API calls
3. API contracts match your current data models
4. Migration will be seamless

---

## ✅ What's Complete

### Requirements Phase ✅

- [x] Analyzed all frontend features
- [x] Defined data models for all entities
- [x] Specified all API endpoints
- [x] Designed complete database schema
- [x] Mapped to Holochain Entry Types
- [x] Created implementation roadmap
- [x] Documented security requirements
- [x] Planned GDPR compliance

### Next: Implementation Phase

- [ ] Create SQL migration scripts
- [ ] Set up database infrastructure
- [ ] Build authentication service
- [ ] Implement profile APIs
- [ ] Build invitation system
- [ ] Add settings management
- [ ] Implement GDPR features

---

## 🔗 Quick Links

- **Start Here:** [Backend API Index](backend-api-index.md)
- **Overview:** [Backend Requirements Summary](backend-requirements-summary.md)
- **API Spec:** [Backend API Requirements](backend-api-requirements.md)
- **Database:** [Database Schema Design](database-schema-design.md)
- **Invitations:** [Invitation System Database](invitation-system-database.md)

---

## 📝 Key Decisions Made

1. **Multi-Tenant Architecture:** Territory-based schemas for data isolation
2. **Global Registries:** Ensure username/email uniqueness across territories
3. **Territory Binding:** Database-enforced security for invitations
4. **Privacy-First:** Username login, privacy-aware queries
5. **GDPR Ready:** Export and deletion from day one
6. **Holochain Path:** Design PostgreSQL schema for easy migration
7. **Location Format:** Encoded coordinates + display name for flexibility

---

## 🎯 Success Metrics

You'll know Phase 1 is complete when:

- ✅ Users can register with invitation tokens
- ✅ Authentication works (login/logout/refresh)
- ✅ Profiles are fully functional with privacy filtering
- ✅ Map location picker data saves correctly
- ✅ All settings sync across devices
- ✅ Invitation system creates and validates tokens
- ✅ GDPR export works
- ✅ Frontend has no localStorage mocks
- ✅ All tests passing

---

## 🙏 Next Steps

**Immediate:**

1. Review the documentation to ensure it matches your vision
2. Decide on implementation questions (password hashing, file storage, email service)
3. Set up development environment for backend work

**This Week:**

1. Create SQL migration scripts
2. Set up PostgreSQL databases for each territory
3. Start implementing authentication service

**Next 6 Weeks:**

1. Follow the implementation roadmap
2. Build APIs week by week
3. Integrate with frontend
4. Test and deploy

---

**Status:** ✅ Documentation Complete  
**Timeline:** 6-8 weeks to MVP backend  
**Long-term:** 12+ months to full Holochain

Ready to start building the backend! 🚀
