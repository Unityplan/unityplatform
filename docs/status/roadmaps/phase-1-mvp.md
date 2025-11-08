# Phase 1: MVP Implementation Roadmap

**Last Updated:** November 8, 2025

## 🎯 Phase Overview

**Timeline**: 6-9 months  
**Goal**: Launch a functional platform with core features supporting 3-5 territories  
**Team Size**: 4-6 developers + 1 DevOps + 1 Product Owner  

---

## 📋 Table of Contents

1. [Month 1-2: Foundation & Setup](#month-1-2-foundation--setup)
2. [Month 3-4: Core Services](#month-3-4-core-services)
3. [Month 5-6: Learning & Communication](#month-5-6-learning--communication)
4. [Month 7-8: Integration & Testing](#month-7-8-integration--testing)
5. [Month 9: Launch Preparation](#month-9-launch-preparation)
6. [Success Metrics](#success-metrics)

---

## Month 1-2: Foundation & Setup

### Week 1-2: Infrastructure Setup

#### DevOps & Infrastructure

```
┌─────────────────────────────────────────────────────────┐
│ Infrastructure Bootstrap                                │
├─────────────────────────────────────────────────────────┤
│ ✅ Configure Docker & Docker Compose                    │
│ ✅ Set up PostgreSQL 16 + TimescaleDB                   │
│ ✅ Configure NATS message bus clustering                │
│ ✅ Set up Traefik reverse proxy                         │
│ ✅ Create multi-pod architecture (DK, NO, SE, EU)       │
│ ✅ Set up monitoring (Prometheus + Grafana)             │
│ ✅ Set up Forgejo (version control + MCP)               │
│ ✅ Set up Docker Registry (local image storage)         │
│ ☐ Configure SSL/TLS certificates (Let's Encrypt)        │
└─────────────────────────────────────────────────────────┘

Deliverables:
✅ docker-compose.dev.yml (development tools)
✅ docker-compose.monitoring.yml (Prometheus, Grafana, Jaeger)
✅ docker-compose.pod.yml (single-territory template)
✅ docker-compose.multi-territory-pod.yml (shared infrastructure)
✅ Multi-pod deployment scripts
✅ NATS clustering guide
✅ Forgejo with MCP integration (guides/operations/forgejo-mcp-setup.md)
✅ Docker Registry for local builds
```

#### Database Schema Design

```
┌─────────────────────────────────────────────────────────┐
│ Database Schema Creation                                │
├─────────────────────────────────────────────────────────┤
│ ✅ Design territory schema architecture                 │
│   • global schema (identity/federation layer)           │
│   • territory schema (isolated per territory)           │
│   • Territory ID Format standard (countries, First      │
│     Nations, communities)                                │
│                                                          │
│ ✅ Design global schema tables                          │
│   • territories table (DK, NO, SE, DE, FR, ES, etc.)    │
│   • user_identities (cryptographic hash only)           │
│   • sessions (refresh tokens)                           │
│   • audit_log, territory_managers, role_assignments     │
│                                                          │
│ ✅ Design territory schema template                     │
│   • users (all personal data - data sovereignty)        │
│   • invitation_tokens, invitation_uses                  │
│   • communities, settings                               │
│   • Trigger: sync_global_user_identity()                │
│                                                          │
│ ✅ Create migration scripts with SQLx                   │
│   • 20251108000001_global_schema.up.sql                 │
│   • 20251108000002_territory_schema.up.sql              │
│   • 20251108000003_seed_data_dk.up.sql                  │
│                                                          │
│ ✅ Deploy to Denmark pod                                │
│   • Schema separation complete (Nov 8, 2025)            │
│   • Generic "territory" schema for single-pod           │
│   • Prepared for multi-territory deployment             │
└─────────────────────────────────────────────────────────┘

Deliverables:
✅ Territory ID Format standard (architecture/territory-management-standard.md)
✅ Multi-pod architecture with data isolation
✅ PostgreSQL with global + territory schemas
✅ services/shared-lib/migrations/ directory with SQLx migrations
✅ Database schema fully implemented and tested
```

### Week 3-4: Authentication Service

#### Auth Service Implementation

```
┌─────────────────────────────────────────────────────────┐
│ Authentication Service (Rust + actix-web)               │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ✅ User registration (with invitation system)           │
│   • Invitation token validation (required)              │
│   • Password hashing (bcrypt, cost: 12)                 │
│   • User creation in territory.users                    │
│   • Global identity sync via trigger                    │
│                                                          │
│ ✅ Login/Logout                                         │
│   • Username/password validation                        │
│   • JWT token generation (RS256)                        │
│   • Refresh token rotation                              │
│   • Session management in global.sessions               │
│                                                          │
│ ✅ Invitation System                                    │
│   • Bootstrap script for initial admin invitations      │
│   • Single-use and group invitation tokens              │
│   • Email-specific and open invitations                 │
│   • Usage tracking and audit trail                      │
│   • Revocation support                                  │
│                                                          │
│ ✅ JWT Middleware                                       │
│   • JWT validation middleware (JwtAuth)                 │
│   • Territory extraction from token                     │
│   • User authentication for protected routes            │
│                                                          │
│ ☐ OpenID Connect (OIDC) Integration (Future)            │
│   • Support for Keycloak                                │
│   • OAuth 2.0 flow                                      │
│   • Token validation                                    │
│   • User profile sync                                   │
└─────────────────────────────────────────────────────────┘

API Endpoints: ✅ ALL IMPLEMENTED
POST   /api/auth/register     ✅ With invitation validation
POST   /api/auth/login        ✅ Username/password auth
POST   /api/auth/logout       ✅ Session cleanup
POST   /api/auth/refresh      ✅ Token rotation
GET    /api/auth/me           ✅ User profile
GET    /api/auth/health       ✅ Health check

POST   /api/invitations       ✅ Create invitation (authenticated)
GET    /api/invitations       ✅ List user's invitations
POST   /api/invitations/:id/revoke  ✅ Revoke invitation
GET    /api/invitations/:token/validate  ✅ Validate token
GET    /api/invitations/:id/usage  ✅ Get usage stats

Dependencies:
✅ actix-web 4.x
✅ jsonwebtoken 9.x
✅ bcrypt
✅ sqlx 0.8
✅ uuid

Deliverables:
✅ Working auth service (services/auth-service/)
✅ Unit tests (7 tests - password, JWT, invitations)
✅ Integration tests (19 tests - 100% pass rate)
✅ All endpoints tested with zero warnings
✅ Invitation bootstrap script (scripts/create-bootstrap-invitation.sh)
```

#### Testing & Documentation

```
Tests to implement:
✅ Unit tests for password hashing
✅ Unit tests for JWT generation/validation
✅ Unit tests for invitation token generation
✅ Integration tests for registration flow (with invitations)
✅ Integration tests for login flow
✅ Integration tests for refresh/logout
✅ Integration tests for /me endpoint
✅ Integration tests for invitation CRUD
☐ Load tests (100 req/s for login)

Documentation:
✅ Database schema with separated global/territory schemas
✅ Authentication flow (invitation-based registration)
✅ Security considerations (bcrypt, JWT, token rotation)
☐ API endpoint documentation (OpenAPI/Swagger)
☐ Setup instructions for OIDC providers
```

---

## Week 5-6: User Service & Territory Management

### User Service

```
┌─────────────────────────────────────────────────────────┐
│ User Service (Rust + actix-web)                         │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ☐ User Profile Management                               │
│   • Create/read/update profile                          │
│   • Avatar upload (to IPFS)                             │
│   • Bio, location, birthdate                            │
│   • Multi-schema support (territory routing)            │
│                                                          │
│ ☐ Privacy Settings                                      │
│   • Visibility controls (7 presets)                     │
│   • Social groups (Family, Friends, etc.)               │
│   • Granular field-level privacy                        │
│                                                          │
│ ☐ Language Preferences                                  │
│   • Primary language                                    │
│   • Language proficiency tracking                       │
│   • Preferred translation language                      │
│                                                          │
│ ☐ Social Links                                          │
│   • Add/remove social media links                       │
│   • Support 30+ platforms                               │
│   • Custom link types                                   │
│                                                          │
│ ☐ Notification Preferences                              │
│   • Email notification settings                         │
│   • Push notification settings                          │
│   • Quiet hours                                         │
└─────────────────────────────────────────────────────────┘

API Endpoints:
GET    /api/users/profile
PUT    /api/users/profile
POST   /api/users/profile/avatar
GET    /api/users/privacy
PUT    /api/users/privacy
GET    /api/users/languages
POST   /api/users/languages
GET    /api/users/social-links
POST   /api/users/social-links
DELETE /api/users/social-links/:id

Deliverables:
✓ Working user service
✓ IPFS integration for avatars
✓ Territory-aware database routing
✓ Tests (unit + integration)
```

### Territory Service

```
┌─────────────────────────────────────────────────────────┐
│ Territory Service (Rust + actix-web)                    │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ☐ Territory Registry                                    │
│   • Create new territories                              │
│   • Territory settings management                       │
│   • Database server mapping                             │
│                                                          │
│ ☐ Territory Manager Assignment                          │
│   • Assign managers to territories                      │
│   • Multi-territory management support                  │
│   • Permission validation                               │
│                                                          │
│ ☐ Connection Pool Management                            │
│   • Dynamic connection pools per territory              │
│   • Health monitoring                                   │
│   • Automatic reconnection                              │
│                                                          │
│ ☐ Territory Migration Tools                             │
│   • Export territory schema                             │
│   • Import to new server                                │
│   • Update registry mapping                             │
└─────────────────────────────────────────────────────────┘

Initial Territories to Create:
• territory_global (testing)
• territory_dk (Denmark)
• territory_test1
• territory_test2

Deliverables:
✓ Territory registry service
✓ Connection pool manager
✓ Territory creation scripts
✓ Migration tooling
```

---

## Week 7-8: Badge System

### Badge Service Implementation

```
┌─────────────────────────────────────────────────────────┐
│ Badge Service (Rust + actix-web)                        │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ☐ Badge Definitions                                     │
│   • Create badge templates                              │
│   • Define permissions                                  │
│   • Set prerequisites                                   │
│   • Badge icons/images                                  │
│                                                          │
│ ☐ Badge Awards                                          │
│   • Award badge to user                                 │
│   • Automatic awards on course completion               │
│   • Manual awards by authorized users                   │
│   • Set expiration dates                                │
│                                                          │
│ ☐ Badge Validation                                      │
│   • Check if user has badge                             │
│   • Verify prerequisites                                │
│   • Check expiration                                    │
│   • Permission lookup                                   │
│                                                          │
│ ☐ Code of Conduct Badge (Special)                       │
│   • Mandatory badge logic                               │
│   • Annual renewal system                               │
│   • Expiration notifications (30, 14, 7 days)           │
│   • Automatic lockout on expiration                     │
│                                                          │
│ ☐ Event Publishing                                      │
│   • badge.awarded events                                │
│   • badge.expired events                                │
│   • badge.renewal_needed events                         │
└─────────────────────────────────────────────────────────┘

Core Badges to Create:
✓ Code of Conduct (mandatory, annual renewal)
✓ Basic Learner
✓ Forum Participant
✓ Content Contributor
✓ Community Member

API Endpoints:
GET    /api/badges/definitions
POST   /api/badges/definitions (admin)
GET    /api/badges/user/:user_id
POST   /api/badges/award
POST   /api/badges/validate
GET    /api/badges/prerequisites/:badge_id

Deliverables:
✓ Badge service implementation
✓ NATS event subscriptions
✓ Expiration notification system
✓ Permission validation middleware
```

---

## Month 3-4: Core Services

### Week 9-10: Course Service (LMS)

```
┌─────────────────────────────────────────────────────────┐
│ Course Service - Learning Management System             │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ☐ Course Management                                     │
│   • Create/edit/delete courses                          │
│   • Course versioning                                   │
│   • Multilevel courses (global/territory/community)     │
│   • Course prerequisites (badge-based)                  │
│   • Awards badge on completion                          │
│                                                          │
│ ☐ Course Content                                        │
│   • Modules and lessons                                 │
│   • Video upload to IPFS                                │
│   • PDF/document upload to IPFS                         │
│   • Rich text content                                   │
│   • Quizzes and assessments                             │
│                                                          │
│ ☐ Enrollment System                                     │
│   • Enroll in course (with badge check)                 │
│   • Track progress                                      │
│   • Mark lessons complete                               │
│   • Certificate generation                              │
│                                                          │
│ ☐ Progress Tracking                                     │
│   • Lesson completion status                            │
│   • Quiz scores                                         │
│   • Overall course progress                             │
│   • Time spent tracking                                 │
│                                                          │
│ ☐ Course Update Notifications                           │
│   • Edit vs Replace detection                           │
│   • Notification to previous participants               │
│   • Retake requirements                                 │
└─────────────────────────────────────────────────────────┘

Schema:
CREATE TABLE courses (
    id UUID PRIMARY KEY,
    title VARCHAR(255),
    description TEXT,
    level VARCHAR(20), -- global, territory, community
    level_id UUID,
    version INT,
    prerequisites JSONB, -- array of badge IDs
    awards_badge_id UUID,
    created_by UUID,
    status VARCHAR(20) -- draft, published, archived
);

CREATE TABLE course_modules (
    id UUID PRIMARY KEY,
    course_id UUID REFERENCES courses(id),
    title VARCHAR(255),
    order_index INT,
    content JSONB
);

CREATE TABLE course_enrollments (
    id UUID PRIMARY KEY,
    user_id UUID,
    course_id UUID,
    enrolled_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    progress JSONB
);

API Endpoints:
GET    /api/courses
GET    /api/courses/:id
POST   /api/courses (creator)
PUT    /api/courses/:id (creator)
POST   /api/courses/:id/enroll
GET    /api/courses/:id/progress
PUT    /api/courses/:id/progress

Deliverables:
✓ Course service with IPFS integration
✓ Enrollment and progress tracking
✓ Badge prerequisite validation
✓ Tests and documentation
```

### Week 11-12: Forum Service

```
┌─────────────────────────────────────────────────────────┐
│ Forum Service                                           │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ☐ Forum Structure Management                            │
│   • Create categories                                   │
│   • Create subcategories                                │
│   • Multi-level forums (global/territory/community)     │
│   • Badge-gated access                                  │
│   • Visibility controls                                 │
│                                                          │
│ ☐ Topic Management                                      │
│   • Create topics (with permission check)               │
│   • Edit own topics                                     │
│   • Close topics (moderators)                           │
│   • Pin topics                                          │
│   • Tags and categories                                 │
│                                                          │
│ ☐ Comments & Replies                                    │
│   • Post comments                                       │
│   • Threaded replies                                    │
│   • Edit own comments                                   │
│   • Flag inappropriate content                          │
│                                                          │
│ ☐ Moderation System                                     │
│   • Flag comments                                       │
│   • Issue warnings (3-strike system)                    │
│   • Close topics                                        │
│   • Escalate to admins                                  │
│   • Moderation log (audit trail)                        │
│                                                          │
│ ☐ Voting System                                         │
│   • Upvote/downvote comments                            │
│   • Vote tracking                                       │
│   • Sort by votes                                       │
└─────────────────────────────────────────────────────────┘

Schema:
CREATE TABLE forums (
    id UUID PRIMARY KEY,
    name VARCHAR(255),
    description TEXT,
    level VARCHAR(20),
    level_id UUID,
    required_badges JSONB,
    parent_id UUID REFERENCES forums(id),
    status VARCHAR(20) -- open, closed
);

CREATE TABLE forum_topics (
    id UUID PRIMARY KEY,
    forum_id UUID REFERENCES forums(id),
    title VARCHAR(255),
    created_by UUID,
    created_at TIMESTAMPTZ,
    status VARCHAR(20), -- open, closed, pinned
    view_count INT,
    comment_count INT
);

CREATE TABLE forum_comments (
    id UUID PRIMARY KEY,
    topic_id UUID REFERENCES forum_topics(id),
    parent_id UUID REFERENCES forum_comments(id),
    content TEXT,
    created_by UUID,
    created_at TIMESTAMPTZ,
    edited_at TIMESTAMPTZ,
    flags INT DEFAULT 0,
    votes INT DEFAULT 0
);

CREATE TABLE moderation_actions (
    id UUID PRIMARY KEY,
    moderator_id UUID,
    target_user_id UUID,
    action_type VARCHAR(50), -- warn, restrict, escalate
    reason TEXT,
    created_at TIMESTAMPTZ
);

API Endpoints:
GET    /api/forums
GET    /api/forums/:id/topics
POST   /api/forums/:id/topics (with badge check)
GET    /api/topics/:id/comments
POST   /api/topics/:id/comments
POST   /api/comments/:id/flag
POST   /api/moderation/warn

Deliverables:
✓ Forum service with badge-based access
✓ Moderation system
✓ 3-strike warning implementation
✓ Tests and documentation
```

---

## Month 5-6: Learning & Communication

### Week 13-14: IPFS Integration

```
┌─────────────────────────────────────────────────────────┐
│ IPFS Storage Service                                    │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ☐ IPFS Node Setup                                       │
│   • Deploy IPFS Kubo node                               │
│   • Configure pinning service                           │
│   • Set up garbage collection                           │
│                                                          │
│ ☐ File Upload API                                       │
│   • Upload files to IPFS                                │
│   • Return CID (Content Identifier)                     │
│   • Store metadata in PostgreSQL                        │
│   • File type validation                                │
│   • Size limits                                         │
│                                                          │
│ ☐ File Retrieval                                        │
│   • Fetch by CID                                        │
│   • Stream large files                                  │
│   • Cache frequently accessed files                     │
│                                                          │
│ ☐ Pin Management                                        │
│   • Pin important files                                 │
│   • Unpin old/unused files                              │
│   • Monitor storage usage                               │
└─────────────────────────────────────────────────────────┘

Supported File Types:
• Images: JPEG, PNG, GIF, WebP (max 10MB)
• Documents: PDF, DOCX, TXT (max 50MB)
• Videos: MP4, WebM (max 500MB)
• Archives: ZIP (max 100MB)

API Endpoints:
POST   /api/ipfs/upload
GET    /api/ipfs/:cid
GET    /api/ipfs/:cid/metadata
DELETE /api/ipfs/:cid (unpin)

Deliverables:
✓ IPFS node deployment
✓ Upload/download API
✓ Integration with User, Course, Forum services
✓ Storage monitoring dashboard
```

### Week 15-16: Translation Service (Basic)

```
┌─────────────────────────────────────────────────────────┐
│ Translation Service (MVP Version)                       │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ☐ Translation Memory                                    │
│   • Store translations in PostgreSQL                    │
│   • Translation cache (Redis)                           │
│   • Reuse previous translations                         │
│                                                          │
│ ☐ Basic Translation API                                 │
│   • Integrate with external API (DeepL/Google)          │
│   • Language detection                                  │
│   • Source text preservation                            │
│   • Quality scoring                                     │
│                                                          │
│ ☐ User Preference Support                               │
│   • Fetch user's preferred language                     │
│   • Auto-translate content                              │
│   • Show original option                                │
│                                                          │
│ ☐ Cache Management                                      │
│   • Cache translations (1 week TTL)                     │
│   • Cache key: hash(source_text + target_lang)          │
│   • Cache statistics                                    │
└─────────────────────────────────────────────────────────┘

Supported Languages (MVP):
• English (en)
• Danish (da)
• Spanish (es)
• French (fr)
• German (de)

Note: Start with external API, plan for self-hosted in Phase 2

API Endpoints:
POST   /api/translate
{
  "text": "Hello world",
  "target_language": "da",
  "source_language": "en" (optional, auto-detect)
}

Response:
{
  "translated_text": "Hej verden",
  "source_language": "en",
  "target_language": "da",
  "cached": false
}

Deliverables:
✓ Translation service with external API integration
✓ Translation memory and caching
✓ Integration with Forum and Course services
```

---

## Month 7-8: Integration & Testing

### Week 17-18: Matrix Protocol Integration

```
┌─────────────────────────────────────────────────────────┐
│ Matrix Homeserver Setup                                 │
├─────────────────────────────────────────────────────────┤
│ Core Features:                                          │
│ ☐ Matrix Synapse Deployment                             │
│   • Deploy Synapse server                               │
│   • PostgreSQL database for Matrix                      │
│   • Configure federation                                │
│   • SSL/TLS setup                                       │
│                                                          │
│ ☐ Matrix Gateway Service                                │
│   • Rust service using ruma crate                       │
│   • User registration sync                              │
│   • Room creation for forum topics                      │
│   • Message routing                                     │
│                                                          │
│ ☐ Forum-Matrix Integration                              │
│   • Create Matrix room when forum topic created         │
│   • Sync comments to Matrix messages                    │
│   • Sync Matrix messages to forum comments              │
│   • Bidirectional updates                               │
│                                                          │
│ ☐ Basic Direct Messages                                 │
│   • 1-on-1 encrypted chats                              │
│   • User presence                                       │
│   • Typing indicators                                   │
└─────────────────────────────────────────────────────────┘

Matrix Synapse Configuration:
server_name: "matrix.unityplan.org"
database:
  name: psycopg2
  args:
    database: matrix_db

Deliverables:
✓ Matrix Synapse server deployed
✓ Matrix Gateway service
✓ Forum-Matrix bidirectional sync
✓ Basic DM functionality
```

### Week 19-20: Frontend Development

```
┌─────────────────────────────────────────────────────────┐
│ React Frontend (Vite + TailwindCSS + ShadCN)            │
├─────────────────────────────────────────────────────────┤
│ Core Pages:                                             │
│ ☐ Authentication                                        │
│   • Login page                                          │
│   • Registration page                                   │
│   • OIDC callback handling                              │
│   • Password reset                                      │
│                                                          │
│ ☐ User Profile                                          │
│   • View profile                                        │
│   • Edit profile                                        │
│   • Avatar upload                                       │
│   • Privacy settings                                    │
│   • Language preferences                                │
│                                                          │
│ ☐ Course Catalog                                        │
│   • Browse courses                                      │
│   • Course details                                      │
│   • Enroll in course                                    │
│   • My courses dashboard                                │
│                                                          │
│ ☐ Course Player                                         │
│   • Video player                                        │
│   • Document viewer                                     │
│   • Progress tracking                                   │
│   • Quizzes                                             │
│   • Mark complete                                       │
│                                                          │
│ ☐ Forums                                                │
│   • Forum categories list                               │
│   • Topics list                                         │
│   • Topic view with comments                            │
│   • Create topic (if has badge)                         │
│   • Post comment                                        │
│   • Moderation tools (if moderator)                     │
│                                                          │
│ ☐ Badges & Achievements                                 │
│   • My badges                                           │
│   • Badge details                                       │
│   • Prerequisites view                                  │
│                                                          │
│ ☐ Direct Messages (Matrix)                              │
│   • Message list                                        │
│   • Chat interface                                      │
│   • Send/receive messages                               │
└─────────────────────────────────────────────────────────┘

Component Library (ShadCN):
✓ Button, Input, Textarea
✓ Card, Dialog, Dropdown
✓ Table, Tabs, Tooltip
✓ Toast notifications
✓ Avatar, Badge
✓ Form components

State Management:
• TanStack Query for server state
• Zustand for client state
• React Context for auth

Routing:
• TanStack Router with type-safe routes

Deliverables:
✓ Complete frontend application
✓ Responsive design (mobile, tablet, desktop)
✓ Dark mode support
✓ Accessibility (WCAG AA)
✓ E2E tests (Playwright)
```

### Week 21-22: End-to-End Testing

```
┌─────────────────────────────────────────────────────────┐
│ Testing & Quality Assurance                             │
├─────────────────────────────────────────────────────────┤
│ Backend Testing:                                        │
│ ☐ Unit tests for all services (>80% coverage)           │
│ ☐ Integration tests for API endpoints                   │
│ ☐ Load testing (100 concurrent users)                   │
│ ☐ Database migration tests                              │
│ ☐ NATS message flow tests                               │
│                                                          │
│ Frontend Testing:                                       │
│ ☐ Component tests (React Testing Library)               │
│ ☐ E2E tests (Playwright)                                │
│   • User registration flow                              │
│   • Login flow                                          │
│   • Course enrollment                                   │
│   • Forum participation                                 │
│   • Profile updates                                     │
│                                                          │
│ Security Testing:                                       │
│ ☐ OWASP Top 10 checks                                   │
│ ☐ SQL injection tests                                   │
│ ☐ XSS prevention tests                                  │
│ ☐ CSRF protection tests                                 │
│ ☐ JWT security audit                                    │
│ ☐ Rate limiting tests                                   │
│                                                          │
│ Performance Testing:                                    │
│ ☐ API response time (<200ms p95)                        │
│ ☐ Database query optimization                           │
│ ☐ Frontend bundle size (<500KB)                         │
│ ☐ Lighthouse score (>90)                                │
└─────────────────────────────────────────────────────────┘

Test Scenarios:
1. New user registers → completes Code of Conduct → enrolls in course
2. User completes course → earns badge → gains forum access
3. User posts forum topic → others comment → moderator flags comment
4. Territory manager creates territory-specific course
5. Cross-territory Matrix messages

Deliverables:
✓ Comprehensive test suite
✓ CI/CD pipeline with automated tests
✓ Security audit report
✓ Performance benchmarks
```

---

## Month 9: Launch Preparation

### Week 23-24: Observability & Monitoring

```
┌─────────────────────────────────────────────────────────┐
│ Observability Stack Setup                               │
├─────────────────────────────────────────────────────────┤
│ ☐ Logging (Loki + Grafana)                              │
│   • Centralized logging                                 │
│   • Log aggregation from all services                   │
│   • Log retention (30 days)                             │
│   • Search and filtering                                │
│                                                          │
│ ☐ Metrics (Prometheus + Grafana)                        │
│   • Service metrics collection                          │
│   • Database metrics                                    │
│   • NATS metrics                                        │
│   • Custom business metrics                             │
│                                                          │
│ ☐ Tracing (Jaeger)                                      │
│   • Distributed tracing setup                           │
│   • Request flow visualization                          │
│   • Performance bottleneck identification               │
│                                                          │
│ ☐ Grafana Dashboards                                    │
│   • System overview                                     │
│   • Service health                                      │
│   • Database performance                                │
│   • User activity                                       │
│   • Error rates                                         │
│                                                          │
│ ☐ Alerting (Prometheus Alertmanager)                    │
│   • High error rate alerts                              │
│   • Service down alerts                                 │
│   • Database connection pool alerts                     │
│   • Disk usage alerts                                   │
│   • Notification channels (Slack, Email)                │
└─────────────────────────────────────────────────────────┘

Key Metrics to Track:
• Request rate (req/sec)
• Error rate (%)
• Response time (p50, p95, p99)
• Active users
• Course enrollments/day
• Forum posts/day
• Badge awards/day
• Database connections
• NATS messages/sec

Deliverables:
✓ Complete observability stack
✓ Grafana dashboards
✓ Alert rules configured
✓ Runbook documentation
```

### Week 25-26: Documentation & Training

```
┌─────────────────────────────────────────────────────────┐
│ Documentation & User Guides                             │
├─────────────────────────────────────────────────────────┤
│ Technical Documentation:                                │
│ ☐ API documentation (OpenAPI/Swagger)                   │
│ ☐ Database schema documentation                         │
│ ☐ Deployment guide                                      │
│ ☐ Troubleshooting guide                                 │
│ ☐ Disaster recovery procedures                          │
│ ☐ Security best practices                               │
│                                                          │
│ User Documentation:                                     │
│ ☐ Getting started guide                                 │
│ ☐ How to create courses                                 │
│ ☐ How to moderate forums                                │
│ ☐ Badge system explanation                              │
│ ☐ Privacy settings guide                                │
│ ☐ FAQ                                                   │
│                                                          │
│ Territory Manager Documentation:                        │
│ ☐ Territory setup guide                                 │
│ ☐ User invitation process                               │
│ ☐ Content management                                    │
│ ☐ Community creation                                    │
│ ☐ Reporting and analytics                               │
└─────────────────────────────────────────────────────────┘

Training Materials:
☐ Video tutorials (5-10 minutes each)
☐ Interactive product tour
☐ Territory manager onboarding checklist

Deliverables:
✓ Complete documentation site
✓ User guides and tutorials
✓ Admin/manager training materials
```

### Week 27-28: Beta Testing & Launch

```
┌─────────────────────────────────────────────────────────┐
│ Beta Testing Phase                                      │
├─────────────────────────────────────────────────────────┤
│ Week 27: Closed Beta                                    │
│ ☐ Invite 20-30 beta testers                             │
│ ☐ Create test territories (2-3)                         │
│ ☐ Onboard territory managers                            │
│ ☐ Create sample courses and forums                      │
│ ☐ Collect feedback daily                                │
│ ☐ Fix critical bugs                                     │
│ ☐ Monitor system performance                            │
│                                                          │
│ Week 28: Launch Preparation                             │
│ ☐ Address beta feedback                                 │
│ ☐ Final security review                                 │
│ ☐ Performance optimization                              │
│ ☐ Backup systems verification                           │
│ ☐ Load testing (500 concurrent users)                   │
│ ☐ Launch checklist completion                           │
│                                                          │
│ Launch Day:                                             │
│ ☐ Deploy to production                                  │
│ ☐ Enable monitoring and alerts                          │
│ ☐ Territory managers ready                              │
│ ☐ Support team on standby                               │
│ ☐ Announce to initial territories (3-5)                 │
│ ☐ Monitor first 24 hours closely                        │
└─────────────────────────────────────────────────────────┘

Launch Checklist:
✓ All services running and healthy
✓ SSL certificates valid
✓ Backups configured and tested
✓ Monitoring and alerting active
✓ Documentation published
✓ Support channels ready
✓ Emergency rollback plan ready

Deliverables:
✓ Production deployment
✓ 3-5 territories onboarded
✓ Initial user base (50-100 users)
✓ Post-launch monitoring report
```

---

## Success Metrics

### Technical Metrics

```
Performance:
✓ API response time: <200ms (p95)
✓ Page load time: <2 seconds
✓ Database query time: <50ms (p95)
✓ Uptime: >99.5%

Scalability:
✓ Support 100 concurrent users
✓ 1000+ total users across territories
✓ 50+ courses
✓ 100+ forum topics
```

### User Metrics

```
Engagement:
✓ Code of Conduct completion: 100% of active users
✓ Course completion rate: >40%
✓ Forum participation: >30% of users
✓ Daily active users: >20% of total users

Content:
✓ 10+ global courses
✓ 5+ territory-specific courses per territory
✓ 50+ forum topics
✓ 500+ forum comments
```

### Business Metrics

```
Adoption:
✓ 3-5 territories onboarded
✓ 50-100 active users
✓ 3+ territory managers
✓ 5+ content creators
✓ User satisfaction: >4/5
```

---

## Risk Mitigation

### Technical Risks

```
Risk: Database performance issues with multi-schema
Mitigation:
• Connection pool optimization
• Query performance monitoring
• Index optimization
• Read replicas if needed

Risk: IPFS node storage exhaustion
Mitigation:
• File size limits
• Automatic unpinning of old files
• External pinning service backup

Risk: Matrix server scalability
Mitigation:
• Start with single homeserver
• Monitor performance closely
• Plan for federation in Phase 2
```

### Project Risks

```
Risk: Scope creep
Mitigation:
• Strict MVP feature list
• Monthly reviews
• Defer non-critical features to Phase 2

Risk: Resource constraints
Mitigation:
• Clear sprint planning
• Prioritize core features
• External help for specialized tasks (e.g., security audit)
```

---

## Phase 1 Completion Criteria

```
☐ All core services deployed and operational
☐ Authentication and user management working
☐ Badge system with Code of Conduct enforcement
☐ Basic LMS with course creation and enrollment
☐ Forum system with moderation
☐ Matrix integration for DMs
☐ IPFS integration for file storage
☐ 3-5 territories operational
☐ 50-100 active users
☐ Documentation complete
☐ Monitoring and alerting in place
☐ 99.5% uptime for 30 days
☐ All critical bugs resolved
☐ Security audit passed
☐ User satisfaction >4/5
```

---

**Next Steps**: Proceed to [Phase 2: Scale & Federation](#) once all completion criteria are met and system has been stable for 30 days.
