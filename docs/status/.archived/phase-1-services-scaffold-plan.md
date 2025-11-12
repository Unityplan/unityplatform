# Phase 1 Services Scaffolding Plan

**Date:** November 11, 2025  
**Status:** 🚧 Planning  
**Based On:** auth-service (proven architecture)

---

## Services to Scaffold for Phase 1 MVP

### ✅ Completed Services

1. **auth-service** (Port 8001) - DONE ✅
   - Registration with invitation tokens
   - Login/logout (JWT)
   - Token refresh
   - Token validation
   - Health checks
   - OpenAPI/Swagger

### 🎯 Phase 1 Priority Services (MUST HAVE)

2. **user-service** (Port 8002) - PRIORITY 1
   - User profile management
   - Avatar upload/management
   - Privacy settings
   - User connections (following/followers)
   - User blocking
   - Profile visibility

3. **badge-service** (Port 8003) - PRIORITY 2
   - Badge definitions
   - Badge issuance
   - Badge verification
   - Badge display/showcase
   - Achievement tracking

4. **community-service** (Port 8004) - PRIORITY 3
   - Community creation/management
   - Community membership
   - Community roles/permissions
   - Community settings
   - Community discovery

5. **territory-service** (Port 8005) - PRIORITY 4
   - Territory management
   - Territory administrators
   - Territory settings
   - Territory statistics
   - Multi-pod coordination

6. **notification-service** (Port 8006) - PRIORITY 5
   - Push notifications
   - Email notifications
   - In-app notifications
   - Notification preferences
   - Notification history

7. **event-service** (Port 8007) - PRIORITY 6
   - Event creation/management
   - Event calendar
   - Event RSVP/attendance
   - Event reminders
   - Event discovery

### 🔮 Phase 2 Services (Scaffold Now, Implement Later)

8. **course-service** (Port 8008) - LMS Core (Phase 2)
   - Course catalog
   - Course enrollment
   - Content delivery
   - Progress tracking
   - Completion certificates

9. **forum-service** (Port 8009) - Matrix-based (Phase 2)
   - Forum topics/threads
   - Post creation/editing
   - Reactions/voting
   - Matrix room integration
   - Moderation

10. **translation-service** (Port 8010) - i18n (Phase 2)
    - Content translation
    - Language preference management
    - Translation workflow
    - Community translations

11. **ipfs-service** (Port 8011) - Storage (Phase 2)
    - File upload to IPFS
    - File retrieval
    - Content addressing
    - Pinning management

---

## Standard Service Template (Based on auth-service)

### Directory Structure

```
{service-name}/
├── Cargo.toml
├── .env.example
├── src/
│   ├── main.rs              # Server entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Environment configuration
│   ├── error.rs             # Custom error types
│   ├── response.rs          # API response envelope (shared)
│   ├── models/              # Request/response models
│   │   ├── mod.rs
│   │   ├── {domain}.rs      # Domain-specific models
│   │   └── health.rs        # Health check response
│   ├── handlers/            # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── {domain}.rs      # Domain endpoint handlers
│   │   └── health.rs        # Health endpoint
│   ├── services/            # Business logic
│   │   ├── mod.rs
│   │   └── {domain}.rs      # Domain service logic
│   ├── middleware/          # HTTP middleware (if needed)
│   │   ├── mod.rs
│   │   └── auth.rs          # JWT authentication (most services)
│   └── openapi.rs           # OpenAPI/Swagger configuration
└── tests/                   # Integration tests (future)
    └── api_tests.rs
```

### Standard Dependencies (Cargo.toml)

```toml
[package]
name = "{service-name}"
version = "0.1.0-alpha.1"
edition = "2021"

[dependencies]
# Framework
actix-web = "4"
actix-cors = "0.7"

# Async Runtime
tokio = { version = "1", features = ["full"] }

# Database
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Validation
validator = { version = "0.18", features = ["derive"] }

# Utilities
uuid = { version = "1.10", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# JWT (for auth middleware in most services)
jsonwebtoken = "9"

# Configuration
config = "0.14"
dotenvy = "0.15"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# OpenAPI
utoipa = { version = "5", features = ["actix_extras", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "8", features = ["actix-web"] }

# Shared Library
shared-lib = { path = "../shared-lib" }
```

### Standard Configuration (.env.example)

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

# JWT (for validating tokens from auth-service)
JWT_SECRET=your-secret-key

# Service-specific config...
```

### Standard API Patterns

1. **Response Envelope** (shared from auth-service)

   ```rust
   ApiResponse<T> {
       success: bool,
       data: Option<T>,
       error: Option<ApiError>,
       meta: ResponseMeta,
   }
   ```

2. **Error Handling** (consistent across services)

   ```rust
   pub enum ServiceError {
       DatabaseError(String),
       NotFound(String),
       Unauthorized(String),
       ValidationError(String),
       InternalError(String),
   }
   ```

3. **Health Check** (all services)

   ```rust
   GET /health
   {
       "status": "healthy",
       "service": "{service-name}",
       "version": "0.1.0-alpha.1",
       "dependencies": {
           "database": "healthy",
           "nats": "healthy"  // if used
       }
   }
   ```

4. **Authentication Middleware** (most services need this)

   ```rust
   // Extract JWT from Authorization header
   // Validate token
   // Extract user_id, territory_code
   // Add to request extensions
   ```

5. **OpenAPI/Swagger** (all services)
   - `/swagger-ui` - Interactive documentation
   - `/api-doc/openapi.json` - OpenAPI spec

---

## Implementation Strategy

### Phase A: Scaffold All Services (1-2 days)

For each service:

1. Create directory structure
2. Copy standard files from auth-service:
   - `Cargo.toml` (adjust name, port)
   - `src/config.rs` (adjust port default)
   - `src/error.rs` (same)
   - `src/response.rs` (same)
   - `src/models/health.rs` (same)
   - `src/handlers/health.rs` (same)
   - `src/main.rs` (adjust service name)
   - `src/lib.rs` (adjust exports)
3. Create basic domain models (empty structs for now)
4. Create basic domain handlers (stub endpoints)
5. Add to workspace `Cargo.toml`
6. Verify compilation

**Result:** All services compile, health endpoint works, Swagger UI accessible

### Phase B: Implement Core Endpoints (Service by Service)

Priority order:

1. **user-service** - Foundation for all user interactions
2. **territory-service** - Territory management
3. **badge-service** - Achievement system
4. **course-service** - LMS core
5. **forum-service** - Community discussions
6. **translation-service** - Internationalization
7. **ipfs-service** - Decentralized storage

For each service:

1. Define complete data models
2. Implement database queries
3. Implement business logic
4. Implement endpoint handlers
5. Add OpenAPI documentation
6. Test all endpoints

---

## Service-Specific Notes

### user-service

**Key Endpoints:**

- `GET /v1/users/:id` - Get user profile
- `PUT /v1/users/:id` - Update profile
- `POST /v1/users/:id/avatar` - Upload avatar
- `GET /v1/users/:id/privacy` - Get privacy settings
- `PUT /v1/users/:id/privacy` - Update privacy settings
- `GET /v1/users/:id/connections` - Get connections
- `POST /v1/users/:id/follow` - Follow user
- `DELETE /v1/users/:id/follow` - Unfollow user
- `POST /v1/users/:id/block` - Block user
- `DELETE /v1/users/:id/block` - Unblock user

**Database Tables:**

- `territory_dk.users` (already exists)
- `territory_dk.profiles` (already exists)
- `territory_dk.user_privacy_settings`
- `territory_dk.user_connections`
- `territory_dk.user_blocks`

### territory-service

**Key Endpoints:**

- `GET /v1/territories` - List territories
- `GET /v1/territories/:code` - Get territory details
- `PUT /v1/territories/:code` - Update territory settings (admin)
- `GET /v1/territories/:code/stats` - Territory statistics
- `GET /v1/territories/:code/admins` - List administrators

**Database Tables:**

- `global.territories` (already exists)
- `territory_dk.territory_administrators`
- `territory_dk.territory_settings`

### badge-service

**Key Endpoints:**

- `GET /v1/badges` - List badge definitions
- `GET /v1/badges/:id` - Get badge details
- `GET /v1/users/:user_id/badges` - Get user's badges
- `POST /v1/users/:user_id/badges/:badge_id` - Issue badge (admin)
- `GET /v1/users/:user_id/achievements` - Achievement progress

**Database Tables:**

- `territory_dk.badge_definitions`
- `territory_dk.user_badges`
- `territory_dk.badge_criteria`

### course-service

**Key Endpoints:**

- `GET /v1/courses` - List courses
- `GET /v1/courses/:id` - Get course details
- `POST /v1/courses/:id/enroll` - Enroll in course
- `GET /v1/courses/:id/content` - Get course content
- `POST /v1/courses/:id/progress` - Update progress
- `GET /v1/users/:user_id/enrollments` - User's courses

**Database Tables:**

- `territory_dk.courses`
- `territory_dk.course_content`
- `territory_dk.user_enrollments`
- `territory_dk.user_progress`

### forum-service

**Key Endpoints:**

- `GET /v1/forums` - List forums
- `GET /v1/forums/:id/topics` - List topics
- `POST /v1/forums/:id/topics` - Create topic
- `GET /v1/topics/:id/posts` - List posts
- `POST /v1/topics/:id/posts` - Create post
- `PUT /v1/posts/:id` - Edit post
- `POST /v1/posts/:id/react` - React to post

**Database Tables:**

- `territory_dk.forums`
- `territory_dk.forum_topics`
- `territory_dk.forum_posts`
- `territory_dk.post_reactions`
- Integration with Matrix rooms

### translation-service

**Key Endpoints:**

- `GET /v1/translations/:key` - Get translation
- `POST /v1/translations` - Submit translation
- `GET /v1/languages` - List supported languages
- `GET /v1/users/:user_id/language` - Get user preference
- `PUT /v1/users/:user_id/language` - Update preference

**Database Tables:**

- `territory_dk.translations`
- `territory_dk.translation_keys`
- `global.supported_languages`

### ipfs-service

**Key Endpoints:**

- `POST /v1/files` - Upload file to IPFS
- `GET /v1/files/:cid` - Retrieve file
- `POST /v1/files/:cid/pin` - Pin file
- `DELETE /v1/files/:cid/pin` - Unpin file
- `GET /v1/files/:cid/info` - Get file metadata

**External Dependencies:**

- IPFS daemon
- Pinning service

---

## Workspace Configuration

Update `services/Cargo.toml`:

```toml
[workspace]
members = [
    "shared-lib",
    "auth-service",
    "user-service",
    "territory-service",
    "badge-service",
    "course-service",
    "forum-service",
    "translation-service",
    "ipfs-service",
]
resolver = "2"
```

---

## Docker Compose Updates

Add services to `docker-compose.dev.yml`:

```yaml
  user-service:
    build:
      context: ./services
      dockerfile: user-service/Dockerfile
    ports:
      - "8002:8002"
    environment:
      - DATABASE_URL=${DATABASE_URL}
      - SERVER_PORT=8002
    depends_on:
      - postgres

  territory-service:
    # ... port 8003

  badge-service:
    # ... port 8004

  # etc.
```

---

## Next Steps

**Option 1: Quick Scaffold**

- Scaffold all 7 services with basic structure
- Get them all compiling and running
- Health checks working
- Swagger UI accessible
- Then implement endpoints one service at a time

**Option 2: Complete One at a Time**

- Fully implement user-service first
- Then territory-service
- Then badge-service
- etc.

**Recommendation:** Option 1 (Quick Scaffold)

- Sets up consistent architecture across all services
- Makes it easier to see the big picture
- Can work on services in parallel later
- Catches dependency issues early

---

## Time Estimate

- **Scaffolding (Option 1):** 4-6 hours for all 7 services
- **Per-Service Implementation:** 1-3 days each depending on complexity
- **Total for Phase 1 backend:** 2-3 weeks

Should I proceed with scaffolding all services?
