# Archived Architecture Documentation

**Purpose:** Historical documentation that has been superseded by the current microservices architecture.

**Archived Date:** November 12, 2025  
**Reason:** Transition from consolidated API documentation to service-specific documentation structure

---

## What's Here

These documents represent the initial backend API planning phase (November 11, 2025) before the microservices structure was fully established. They are kept for historical reference.

### Archived Files

| File | Original Purpose | Superseded By |
|------|------------------|---------------|
| `20251111-BACKEND-API-CORRECTIONS.md` | Email/username identity corrections | `overview/identity-system.md` |
| `20251111-BACKEND-API-SUMMARY.md` | Initial API planning summary | Service-specific README.md files |
| `20251111-backend-api-index.md` | API documentation index | Service-specific API.md files |
| `20251111-backend-api-requirements.md` | Consolidated API requirements | Service-specific API.md files |
| `20251111-backend-requirements-summary.md` | Requirements overview | Service-specific README.md files |
| `20251111-database-schema-design.md` | Monolithic database schema | Service-specific DATABASE.md files |
| `20251111-invitation-system-database.md` | Invitation system SQL | `services/invitation-service/DATABASE.md` |

---

## Current Documentation Structure

For current architecture documentation, see:

- **Architectural Overviews:** `docs/architecture/overview/`
- **Service-Specific Docs:** `docs/architecture/services/{service-name}/`
  - README.md - Service overview and responsibilities
  - API.md - Endpoint specifications
  - DATABASE.md - Schema design and migrations
  - MIGRATIONS.md - Migration plans (select services)

---

## Why These Were Archived

**Before (Consolidated):**

- Single large database schema document
- Monolithic API requirements file
- Unclear service boundaries

**After (Microservices):**

- Each service owns its documentation
- Clear separation of concerns
- Service-specific schema, API, and migration docs
- Architectural overviews separate from implementation details

---

**Note:** These documents may still contain useful historical context or design rationale. They are not deleted, just archived for reference purposes.
