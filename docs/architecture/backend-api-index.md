# Backend API Documentation Index

**Created:** November 11, 2025  
**Status:** Requirements Complete - Ready for Implementation

---

## 📚 Document Overview

This collection of documents defines the complete backend API requirements based on the implemented frontend features, with a focus on PostgreSQL implementation and future Holochain migration.

---

## 🗂️ Main Documents

### 1. 🎯 [Backend Requirements Summary](backend-requirements-summary.md)

**Start here!** Overview of all requirements and implementation plan.

**Contents:**

- Document navigation guide
- Frontend features analysis
- Data models summary
- Database structure overview
- API endpoints catalog
- Holochain migration roadmap
- Implementation priorities (6-week plan)
- Security considerations
- Next steps and success metrics

**Use when:** You need a high-level overview or want to understand the big picture.

---

### 2. 📋 [Backend API Requirements](backend-api-requirements.md)

**Complete API specification** for all endpoints.

**Contents:**

- Authentication & Registration (Login, Register, JWT flow)
- User Profile Management (CRUD, Avatar, Privacy)
- Profile Links System (Flexible external links)
- User Settings (Appearance, Privacy, Notifications, Account)
- Invitation System (Token types, Validation, Creation)
- Data Sovereignty & GDPR (Export, Deletion)
- Holochain migration considerations

**Use when:** Building API endpoints or defining frontend-backend contracts.

---

### 3. 🗄️ [Database Schema Design](database-schema-design.md)

**Complete PostgreSQL schema** with Holochain migration path.

**Contents:**

- Design principles (PostgreSQL + Holochain readiness)
- Global schema (territories, registries)
- Territory schema (users, profiles, settings, invitations)
- Indexes, constraints, triggers
- Holochain Entry Type mappings
- 5-phase migration strategy
- Validation rules

**Use when:** Writing SQL migration scripts or planning Holochain DNA structure.

---

### 4. 🎫 [Invitation System Database](invitation-system-database.md)

**Detailed implementation guide** for invitation-based registration.

**Contents:**

- SQL table definitions with all constraints
- Triggers and stored procedures
- Token generation and validation queries
- Usage tracking and audit logs
- Security (rate limiting, email validation)
- Testing strategies with test data
- Performance optimization (partitioning, archiving)

**Use when:** Implementing the invitation system database layer.

---

## 🔗 Related Architecture Documents

These existing documents provide additional context:

- [Invitation System Architecture](invitation-system.md) - Original invitation system design
- [Multi-Pod Architecture](multi-pod-architecture.md) - Territory deployment model
- [User Data Sovereignty](user-data-sovereignty.md) - User ownership principles
- [Territory Management Standard](territory-management-standard.md) - Territory governance

---

## 📊 Quick Reference

### Data Models

```
Core Entities:
├── User (Authentication)
├── UserProfile (Public/Private data)
├── ProfileLink (External links)
├── PrivacySettings (Privacy preferences)
├── AppearanceSettings (UI preferences)
├── NotificationSettings (Notification prefs)
└── InvitationToken (Registration tokens)
```

### Database Schemas

```
PostgreSQL Structure:
├── global (Cross-territory)
│   ├── territories
│   ├── username_registry
│   ├── email_registry
│   └── invitation_token_registry
│
└── territory_{code} (Per pod)
    ├── users
    ├── users_profiles
    ├── users_profile_links
    ├── users_language_proficiency
    ├── users_privacy_settings
    ├── users_settings
    ├── users_notification_settings
    ├── users_audit_logs
    ├── invitation_tokens
    └── invitation_uses
```

### API Endpoint Groups

```
/api/v1/auth/*              - Authentication (8 endpoints)
/api/v1/profiles/*          - Profile management (5 endpoints)
/api/v1/avatars/*           - Avatar upload/delete (2 endpoints)
/api/v1/settings/*          - Settings management (4 endpoints)
/api/v1/account/*           - Account operations (5 endpoints)
/api/v1/invitations/*       - Invitation system (5 endpoints)
/api/v1/data/*              - GDPR compliance (3 endpoints)
```

---

## 🚀 Implementation Roadmap

### Week 1-2: Authentication & Infrastructure

- Global schema (territories, registries)
- Territory schema (users table)
- JWT authentication
- Registration with invitation validation
- Login/logout/refresh

### Week 2-3: User Profile

- Profile CRUD
- Avatar upload/delete
- Privacy filtering
- Profile links system

### Week 3-4: Invitation System

- Token creation (single_use, group)
- Token validation
- Usage tracking
- Community auto-assignment

### Week 4-5: Settings Management

- Privacy settings
- Notification settings
- Appearance settings
- Account settings (email, password, 2FA)

### Week 5-6: GDPR Compliance

- Data export
- Account deletion
- Audit logs

---

## 🎯 Success Criteria

Phase 1 (PostgreSQL) is complete when:

- ✅ All database tables created with constraints
- ✅ All authentication endpoints working
- ✅ Users can register with invitation tokens
- ✅ Profile CRUD works with privacy filtering
- ✅ Settings sync across devices
- ✅ Invitation system functional
- ✅ GDPR export/deletion implemented
- ✅ Frontend integration complete (no localStorage mocks)
- ✅ All tests passing

---

## 📖 How to Use This Documentation

**For Backend Developers:**

1. Read [Backend Requirements Summary](backend-requirements-summary.md) for overview
2. Use [Database Schema Design](database-schema-design.md) to create SQL migrations
3. Reference [Backend API Requirements](backend-api-requirements.md) while building endpoints
4. Follow [Invitation System Database](invitation-system-database.md) for invitation implementation

**For Frontend Developers:**

1. Check [Backend API Requirements](backend-api-requirements.md) for endpoint contracts
2. Review data models to understand API responses
3. Use example requests/responses for integration

**For Project Managers:**

1. Review [Backend Requirements Summary](backend-requirements-summary.md) for timeline
2. Track progress against implementation priorities
3. Use success criteria for milestone validation

**For Future Migration:**

1. Study Holochain Entry Type mappings in [Database Schema Design](database-schema-design.md)
2. Follow 5-phase migration strategy
3. Design validation rules based on PostgreSQL constraints

---

## 🔍 Finding Information

**"How do I authenticate users?"**
→ [Backend API Requirements](backend-api-requirements.md#11-authentication-flow)

**"What's the database schema for profiles?"**
→ [Database Schema Design](database-schema-design.md#2-profiles-table)

**"How do invitation tokens work?"**
→ [Invitation System Database](invitation-system-database.md)

**"What's the migration plan to Holochain?"**
→ [Database Schema Design](database-schema-design.md#migration-strategy)

**"What APIs are needed for settings?"**
→ [Backend API Requirements](backend-api-requirements.md#3-user-settings-management)

**"How do I implement GDPR export?"**
→ [Backend API Requirements](backend-api-requirements.md#5-data-sovereignty--gdpr)

---

## 📝 Document Maintenance

### Updating Requirements

When frontend features change:

1. Update [Backend API Requirements](backend-api-requirements.md) with new endpoints
2. Add database tables to [Database Schema Design](database-schema-design.md)
3. Update [Backend Requirements Summary](backend-requirements-summary.md) timeline
4. Map new features to Holochain Entry Types

### Version Control

All documents include:

- **Status:** Current state (Planning/Implementation/Complete)
- **Created:** Date of creation
- **Last Updated:** Date of last modification

---

## 🤝 Contributing

When adding new features:

1. Define API contract in [Backend API Requirements](backend-api-requirements.md)
2. Design database schema in [Database Schema Design](database-schema-design.md)
3. Consider Holochain migration path
4. Update this index

---

**Last Updated:** November 11, 2025  
**Next Review:** When starting Phase 2 (Dual-Write to Holochain)
