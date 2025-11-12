# Architecture Documentation

**Last Updated:** November 12, 2025  
**Version:** 0.1.0-alpha.1  
**Status:** MVP Phase 1 in development

---

## 📂 Documentation Structure

```
docs/architecture/
├── README.md (this file)           # Documentation index and navigation
├── MIGRATIONS-MASTER.md            # Database migration tracking
├── overview/                       # Architectural patterns and principles
│   ├── microservices-architecture.md
│   ├── multi-pod-architecture.md
│   ├── identity-system.md
│   ├── invitation-system.md
│   ├── user-data-sovereignty.md
│   ├── infrastructure-overview.md
│   └── ...
├── services/                       # Service-specific documentation
│   ├── auth-service/
│   │   ├── README.md              # Service overview
│   │   ├── API.md                 # Endpoint specifications
│   │   ├── DATABASE.md            # Schema design
│   │   └── MIGRATIONS.md          # Migration plan
│   ├── user-service/
│   ├── invitation-service/
│   └── ...
└── .archived/                      # Historical documentation (Nov 11, 2025)
```

---

## 🎯 Quick Start

### I want to understand

**...the overall architecture**  
→ Start with [overview/microservices-architecture.md](overview/microservices-architecture.md)

**...multi-pod deployment**  
→ Read [overview/multi-pod-architecture.md](overview/multi-pod-architecture.md)

**...how identity works**  
→ See [overview/identity-system.md](overview/identity-system.md)

**...data sovereignty principles**  
→ Check [overview/user-data-sovereignty.md](overview/user-data-sovereignty.md)

**...a specific service**  
→ Browse [services/](services/) directory

**...database migrations**  
→ Review [MIGRATIONS-MASTER.md](MIGRATIONS-MASTER.md)

---

## 📚 Architectural Overview Documentation

High-level architectural patterns, principles, and cross-cutting concerns.

**Location:** [overview/](overview/)

| Document | Description |
|----------|-------------|
| [microservices-architecture.md](overview/microservices-architecture.md) | Service boundaries, dependencies, communication patterns |
| [multi-pod-architecture.md](overview/multi-pod-architecture.md) | Territory-based deployment, federation, data residency |
| [identity-system.md](overview/identity-system.md) | Username uniqueness, Matrix federation, identity migration |
| [invitation-system.md](overview/invitation-system.md) | Invitation-based registration flow and security |
| [user-data-sovereignty.md](overview/user-data-sovereignty.md) | GDPR compliance, data ownership principles |
| [infrastructure-overview.md](overview/infrastructure-overview.md) | Infrastructure stack, containers, databases |
| [territory-management-standard.md](overview/territory-management-standard.md) | Territory hierarchy, sovereignty model |
| [language-proficiency-system.md](overview/language-proficiency-system.md) | Multi-language support, translation features |
| [password-reset-flow.md](overview/password-reset-flow.md) | Password recovery mechanisms |
| [frontend-stack-rationale.md](overview/frontend-stack-rationale.md) | Frontend technology decisions |

---

## 🔧 Service-Specific Documentation

Detailed implementation documentation for each microservice.

**Location:** [services/](services/)
---

## 🔧 Service-Specific Documentation

Detailed implementation documentation for each microservice.

**Location:** [services/](services/)

**Documentation Pattern:** Each service contains:
- **README.md** - Service overview, responsibilities, dependencies
- **API.md** - Complete endpoint specifications with examples
- **DATABASE.md** - Schema design with multi-pod considerations
- **MIGRATIONS.md** - Migration specifications (select services)

### Phase 1 Services (MVP - In Development)

| Service | Port | Status | Docs |
|---------|------|--------|------|
| **auth-service** | 8001 | ✅ Complete | [README](services/auth-service/README.md) · [API](services/auth-service/API.md) · [DB](services/auth-service/DATABASE.md) · [Migrations](services/auth-service/MIGRATIONS.md) |
| **user-service** | 8002 | ✅ Complete | [README](services/user-service/README.md) · [API](services/user-service/API.md) · [DB](services/user-service/DATABASE.md) |
| **settings-service** | 8003 | ⏳ Scaffolded | [README](services/settings-service/README.md) · [API](services/settings-service/API.md) · [DB](services/settings-service/DATABASE.md) |
| **invitation-service** | 8004 | ⏳ Scaffolded | [README](services/invitation-service/README.md) · [API](services/invitation-service/API.md) · [DB](services/invitation-service/DATABASE.md) |
| **notification-service** | 8005 | ⏳ Scaffolded | [README](services/notification-service/README.md) · [API](services/notification-service/API.md) · [DB](services/notification-service/DATABASE.md) |

### Phase 2 Services (Planned)

| Service | Port | Status | Docs |
|---------|------|--------|------|
| **community-service** | 8006 | ⏳ Planned | [README](services/community-service/README.md) · [API](services/community-service/API.md) · [DB](services/community-service/DATABASE.md) |
| **badge-service** | 8007 | ⏳ Planned | [README](services/badge-service/README.md) · [API](services/badge-service/API.md) · [DB](services/badge-service/DATABASE.md) |
| **territory-service** | 8008 | ⏳ Planned | [README](services/territory-service/README.md) · [API](services/territory-service/API.md) · [DB](services/territory-service/DATABASE.md) |
| **event-service** | 8009 | ⏳ Planned | [README](services/event-service/README.md) · [API](services/event-service/API.md) · [DB](services/event-service/DATABASE.md) |
| **course-service** | 8010 | ⏳ Planned | [README](services/course-service/README.md) · [API](services/course-service/API.md) · [DB](services/course-service/DATABASE.md) |
| **forum-service** | 8011 | ⏳ Planned | [README](services/forum-service/README.md) · [API](services/forum-service/API.md) · [DB](services/forum-service/DATABASE.md) |
| **translation-service** | 8012 | ⏳ Planned | [README](services/translation-service/README.md) · [API](services/translation-service/API.md) · [DB](services/translation-service/DATABASE.md) |
| **ipfs-service** | 8013 | ⏳ Planned | [README](services/ipfs-service/README.md) · [API](services/ipfs-service/API.md) · [DB](services/ipfs-service/DATABASE.md) |

**Port Allocation:** See [services/port-allocation.md](services/port-allocation.md)

---

## 🗄️ Database Migrations

**Master Plan:** [MIGRATIONS-MASTER.md](MIGRATIONS-MASTER.md)

**Current Status:**
- **Version:** 20251112000005
- **Database:** PostgreSQL 15+ with TimescaleDB
- **Migrations Applied:** 6 core migrations (30 tables)
- **Service Ownership:** Defined per table

**Key Principles:**
- Data sovereignty (personal data in territory pods)
- Global uniqueness (usernames/emails across all pods)
- Service ownership (each service owns its tables)
- Migration path to Holochain (future decentralization)

**Service Migration Plans:**
- [auth-service/MIGRATIONS.md](services/auth-service/MIGRATIONS.md) - Authentication migrations
- [user-service/MIGRATIONS.md](services/user-service/MIGRATIONS.md) - User data migrations (planned)
- More service-specific migration plans coming

---

## 🏗️ Architecture Principles

### Service Independence
- Each service owns its database tables
- Clear API boundaries between services
- NATS for event-driven communication
- No direct database access across services

### Multi-Pod Ready
- Territory-based data isolation
- Global schema for coordination only
- Personal data stays in user's territory
- Cross-pod federation via NATS

### Data Sovereignty
- Users choose their territory (pod)
- All personal data in territory schema
- GDPR-compliant data export/deletion
- Migration between territories supported

### Holochain Migration Path
- Current: PostgreSQL + NATS
- Future: Holochain DNAs per service
- Agent-centric data ownership
- Distributed hash table storage

---

## 📖 Related Documentation

### Development Guides
- [Service Separation Migration](../guides/development/service-separation-migration.md) - Migration from monolith
- [Versioning Strategy](../guides/development/versioning-strategy.md) - SemVer 2.0.0 approach
- [Testing Strategy](../guides/development/testing-strategy.md) - Unit and integration testing

### Deployment Guides
- [Multi-Pod Deployment](../guides/deployment/multi-pod-setup.md) - Territory pod setup
- [Docker Architecture](../guides/deployment/docker-architecture.md) - Container orchestration

### Project Documentation
- [Project Overview](../project/overview.md) - Platform vision and goals
- [Technology Stack](../project/tech-stack.md) - Technology choices
- [Current Status](../status/current/phase-1-status.md) - Development progress

---

## 📊 Documentation Status

### Overview Documentation
- ✅ Microservices architecture
- ✅ Multi-pod architecture
- ✅ Identity system
- ✅ Invitation system
- ✅ Data sovereignty
- ✅ Infrastructure overview
- ✅ Territory management
- ✅ Language proficiency
- ✅ Password reset flow
- ✅ Frontend stack rationale

**Coverage:** 100% (10/10 architectural topics)

### Service Documentation

**Phase 1 (MVP):**
- ✅ auth-service (100% - README, API, DATABASE, MIGRATIONS)
- ✅ user-service (100% - README, API, DATABASE)
- ✅ settings-service (75% - README, API, DATABASE)
- ✅ invitation-service (75% - README, API, DATABASE)
- ✅ notification-service (75% - README, API, DATABASE)

**Phase 2:**
- ⏳ community-service (50% - README, API, DATABASE)
- ⏳ badge-service (50% - README, API, DATABASE)
- ⏳ territory-service (50% - README, API, DATABASE)
- ⏳ event-service (50% - README, API, DATABASE)
- ⏳ course-service (50% - README, API, DATABASE)
- ⏳ forum-service (50% - README, API, DATABASE)
- ⏳ translation-service (50% - README, API, DATABASE)
- ⏳ ipfs-service (50% - README, API, DATABASE)

**Overall Coverage:** ~70% (all services documented, implementation in progress)

---

## 🗃️ Archived Documentation

Historical documentation from the initial planning phase (November 11, 2025) before the microservices structure was established.

**Location:** [.archived/](.archived/)

**Contents:** Consolidated API planning documents that were superseded by the service-specific documentation structure. Kept for historical reference.

See [.archived/README.md](.archived/README.md) for details.

---

## 🚀 Getting Started

**New to the project?**

1. Read [overview/microservices-architecture.md](overview/microservices-architecture.md) for architecture overview
2. Check [MIGRATIONS-MASTER.md](MIGRATIONS-MASTER.md) for database structure
3. Browse [services/](services/) to understand individual services
4. See [../status/current/phase-1-status.md](../status/current/phase-1-status.md) for current progress

**Implementing a new service?**

1. Follow [services/user-service/](services/user-service/) as a template
2. Create README.md, API.md, DATABASE.md in your service folder
3. Update [MIGRATIONS-MASTER.md](MIGRATIONS-MASTER.md) with your tables
4. Add migration plan if needed (e.g., MIGRATIONS.md)

**Making changes to architecture?**

1. Update relevant overview/ documentation for architectural changes
2. Update service-specific docs for implementation changes
3. Keep documentation in sync with code
4. Update this README.md if structure changes

---

**Last Updated:** November 12, 2025  
**Maintained By:** Development Team  
**Status:** Living Documentation (updated continuously)
