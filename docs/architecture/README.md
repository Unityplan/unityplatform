# Service Architecture Documentation Index

**Last Updated:** November 12, 2025  
**Purpose:** Navigation guide for microservices architecture documentation

---

## 📚 Core Architecture Documents

### **1. [Microservices Architecture](microservices-architecture.md)** ⭐ START HERE
Complete overview of the microservices architecture, service boundaries, dependencies, and Holochain migration path.

**Covers:**
- Service independence principles
- Multi-pod ready architecture
- Service dependency matrix
- Inter-service communication (REST + NATS)
- Database ownership model
- Holochain migration strategy

---

## 🔧 Per-Service Documentation

Each service has comprehensive documentation in its own folder:

### **auth-service/** (Port 8001) 🔐
- [ ] README.md - Service overview
- [ ] database-schema.md - Tables owned (users, sessions, global registries)
- [ ] api-specification.md - Authentication endpoints
- [ ] holochain-migration.md - authentication.happ DNA design

**Status:** ✅ Complete (5/5 endpoints)  
**Dependencies:** invitation-service (during registration)

---

### **user-service/** (Port 8002) 👤
- [x] [README.md](services/user-service/README.md) - Service overview ✅
- [ ] database-schema.md - Profiles, links, connections, GDPR tables
- [ ] api-specification.md - 28 endpoints documented
- [ ] holochain-migration.md - profiles.happ DNA design

**Status:** ✅ Complete (28/28 endpoints)  
**Dependencies:** settings-service (GDPR export)

---

### **settings-service/** (Port 8003) ⚙️
- [ ] README.md - Service overview
- [ ] database-schema.md - users_settings, users_privacy_settings
- [ ] api-specification.md - Settings management endpoints
- [ ] holochain-migration.md - settings.happ DNA (private chain)

**Status:** ⏳ Scaffolded (0/5 endpoints)  
**Dependencies:** None (leaf service)  
**Migration:** Week 1 - [Service Separation Plan](../guides/development/service-separation-migration.md)

---

### **invitation-service/** (Port 8004) 🎫
- [ ] README.md - Service overview
- [ ] database-schema.md - invitation_tokens, invitation_uses, global registry
- [ ] api-specification.md - Invitation management endpoints
- [ ] holochain-migration.md - invitations.happ DNA

**Status:** ⏳ Scaffolded (0/6 endpoints)  
**Dependencies:** None (leaf service)  
**Migration:** Week 2-3 - [Service Separation Plan](../guides/development/service-separation-migration.md)

---

### **notification-service/** (Port 8005) 🔔
- [ ] README.md - Service overview
- [ ] database-schema.md - notifications, notification_settings, templates
- [ ] api-specification.md - Notification endpoints
- [ ] events.md - NATS event subscriptions
- [ ] holochain-migration.md - notifications.happ DNA (ephemeral)

**Status:** ⏳ Scaffolded (0/7 endpoints)  
**Dependencies:** None (but subscribes to all service events)  
**Migration:** Week 2 - [Service Separation Plan](../guides/development/service-separation-migration.md)

---

### **community-service/** (Port 8006) 👥
- [ ] README.md - Service overview
- [ ] database-schema.md - communities, community_members, settings
- [ ] api-specification.md - Community management endpoints
- [ ] holochain-migration.md - communities.happ DNA

**Status:** ⏳ Scaffolded (0/8 endpoints)  
**Dependencies:** user-service, notification-service  
**Priority:** Phase 2

---

### **badge-service/** (Port 8007) 🏅
- [ ] README.md - Service overview
- [ ] database-schema.md - badges, user_badges, badge_progress
- [ ] api-specification.md - Badge and achievement endpoints
- [ ] holochain-migration.md - badges.happ DNA

**Status:** ⏳ Scaffolded (0/6 endpoints)  
**Dependencies:** user-service, notification-service  
**Priority:** Phase 2

---

### **territory-service/** (Port 8008) 🌍
- [ ] README.md - Service overview
- [ ] database-schema.md - global.territories, territory_settings, stats
- [ ] api-specification.md - Territory management endpoints
- [ ] federation.md - Multi-pod federation strategy
- [ ] holochain-migration.md - territories.happ DNA (meta-level)

**Status:** ⏳ Scaffolded (0/4 endpoints)  
**Dependencies:** None (top-level service)  
**Priority:** Phase 2

---

### **event-service/** (Port 8009) 📅
- [ ] README.md - Service overview
- [ ] database-schema.md - events, event_participants, rsvps
- [ ] api-specification.md - Event management endpoints
- [ ] holochain-migration.md - events.happ DNA

**Status:** ⏳ Planned  
**Priority:** Phase 2

---

### **course-service/** (Port 8010) 📚
- [ ] README.md - Service overview (LMS)
- [ ] database-schema.md - courses, lessons, enrollments, progress
- [ ] api-specification.md - Learning management endpoints
- [ ] holochain-migration.md - courses.happ DNA

**Status:** ⏳ Planned  
**Priority:** Phase 2

---

### **forum-service/** (Port 8011) 💬
- [ ] README.md - Service overview
- [ ] matrix-integration.md - Matrix protocol integration
- [ ] api-specification.md - Forum endpoints (wrapper around Matrix)
- [ ] holochain-migration.md - Integration with Matrix rooms

**Status:** ⏳ Planned  
**Priority:** Phase 2

---

### **translation-service/** (Port 8012) 🌐
- [ ] README.md - Service overview
- [ ] database-schema.md - translations, translation_contributions
- [ ] api-specification.md - Translation management endpoints
- [ ] providers.md - LibreTranslate, DeepL, Google integration

**Status:** ⏳ Planned  
**Priority:** Phase 2

---

### **ipfs-service/** (Port 8013) 📦
- [ ] README.md - Service overview
- [ ] ipfs-integration.md - IPFS node configuration
- [ ] api-specification.md - File storage endpoints
- [ ] holochain-migration.md - Pure IPFS (no migration needed)

**Status:** ⏳ Planned  
**Priority:** Phase 2

---

## 📖 Related Documentation

### **Implementation Guides**

- [Service Separation Migration Plan](../guides/development/service-separation-migration.md)
  - Week-by-week plan to properly separate consolidated services
  - Step-by-step instructions for moving settings, notifications, invitations
  - Testing and deployment strategy

- [Database Schema Design](database-schema-design.md)
  - Complete PostgreSQL schema (global + territory)
  - Table ownership by service
  - Migration history

- [Multi-Pod Architecture](multi-pod-architecture.md)
  - Territory-based pod deployment
  - Cross-pod federation
  - Service discovery and routing

### **Development Guides**

- [Backend API Implementation](../guides/development/backend-api-implementation.md)
  - API design patterns
  - Request/response formats
  - Error handling

- [Testing Strategy](../guides/development/testing-strategy.md)
  - Unit testing per service
  - Integration testing across services
  - Multi-pod testing

---

## 🎯 Quick Navigation

### **I want to...**

**Understand the overall architecture**  
→ Read [Microservices Architecture](microservices-architecture.md)

**Implement a new service**  
→ Follow [user-service/README.md](services/user-service/README.md) as template

**Migrate from consolidated to microservices**  
→ Follow [Service Separation Migration Plan](../guides/development/service-separation-migration.md)

**See what's implemented vs. planned**  
→ Check service status above (✅ = complete, ⏳ = planned)

**Understand service dependencies**  
→ See dependency matrix in [Microservices Architecture](microservices-architecture.md#-service-dependency-matrix)

**Plan Holochain migration**  
→ Read each service's `holochain-migration.md`

**Deploy to multi-pod**  
→ Read [Multi-Pod Architecture](multi-pod-architecture.md)

---

## ✅ Documentation Todo

### **Priority 1: Phase 1 Services** (Week 1-3)

- [x] Microservices Architecture overview ✅
- [x] user-service/README.md ✅
- [x] Service Separation Migration Plan ✅
- [ ] settings-service/README.md
- [ ] invitation-service/README.md
- [ ] notification-service/README.md
- [ ] auth-service/README.md (document current state)

### **Priority 2: Database Documentation** (Week 2)

- [ ] Per-service database-schema.md files
- [ ] Update database-schema-design.md with service ownership
- [ ] Migration file documentation

### **Priority 3: API Documentation** (Week 3)

- [ ] Per-service api-specification.md (OpenAPI format)
- [ ] Event documentation (NATS events per service)
- [ ] Inter-service communication examples

### **Priority 4: Holochain Migration** (Phase 2)

- [ ] Per-service holochain-migration.md
- [ ] DNA architecture diagrams
- [ ] Migration timeline and strategy

---

## 📊 Documentation Coverage

```
Service                    Status    Docs Coverage
──────────────────────────────────────────────────
auth-service (8001)        ✅ Done   ⏳ 20%
user-service (8002)        ✅ Done   ✅ 80%
settings-service (8003)    ⏳ Todo   ⏳ 10%
invitation-service (8004)  ⏳ Todo   ⏳ 10%
notification-service (8005)⏳ Todo   ⏳ 10%
community-service (8006)   ⏳ Plan   ⏳ 5%
badge-service (8007)       ⏳ Plan   ⏳ 5%
territory-service (8008)   ⏳ Plan   ⏳ 5%
──────────────────────────────────────────────────
Overall Documentation      ⏳ 20%
```

**Goal:** 100% documentation coverage before Phase 1 completion

---

**Last Updated:** November 12, 2025  
**Maintained By:** Core Team  
**Status:** Living Document (updated as services evolve)
