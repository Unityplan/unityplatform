# Architecture Documentation

**Last Updated:** November 14, 2025  
**Version:** 0.1.0-alpha.1  
**Status:** MVP Phase 1 in development

---

## ⚠️ CRITICAL: Service Development Standards

**ALL services MUST be consistent. No exceptions.**

### 🎯 Non-Negotiable Requirements

1. **Shared Library Integration** - Use ALL features from [shared-lib](services/shared-lib/)
   - See [services/shared-lib/MIDDLEWARE.md](services/shared-lib/MIDDLEWARE.md) for complete middleware stack
   - All 5 middleware components are REQUIRED: Logging, RequestId, SecurityHeaders, CORS, RateLimit

2. **Code & Naming Conventions** - Follow Rust best practices
   - Rust code: `snake_case` (files, functions, variables)
   - JSON API: `camelCase` (via serde `rename_all`)
   - Database: `snake_case` (PostgreSQL standard)
   - URLs: `kebab-case` with plural resources

3. **API Endpoint Structure** - Consistent URL patterns
   - Health: `GET /api/v1/health`
   - Ready: `GET /api/v1/ready`
   - Metrics: `GET /api/v1/metrics`
   - Resources: `/api/v1/{resource-plural}/{id?}/{action?}`
   - Auth: `Authorization: Bearer {token}` → `AuthUser` extractor

4. **Error Handling** - Unified error responses
   - Use `shared_lib::AppError` and `Result<T>`
   - Automatic JSON error formatting
   - No custom error middleware needed

5. **Validation** - Automatic request validation
   - Use `ValidatedJson<T>`, `ValidatedPath<T>`, `ValidatedQuery<T>`
   - Leverage `validator` crate derive macros
   - Validation errors auto-formatted as 400 Bad Request

**Reference Services:** [user-service](services/user-service/) and [badge-service](services/badge-service/) are fully compliant.

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
| **user-service** | 8002 | ✅ Complete (incl. settings) | [README](services/user-service/README.md) · [API](services/user-service/API.md) · [DB](services/user-service/DATABASE.md) |
| ~~**settings-service**~~ | ~~8003~~ | ❌ Merged into user-service | Settings endpoints moved to user-service for MVP simplicity |
| **invitation-service** | 8004 | ⏳ Scaffolded | [README](services/invitation-service/README.md) · [API](services/invitation-service/API.md) · [DB](services/invitation-service/DATABASE.md) |
| **notification-service** | 8005 | ⏳ Scaffolded | [README](services/notification-service/README.md) · [API](services/notification-service/API.md) · [DB](services/notification-service/DATABASE.md) |

### Phase 2 Services (Planned)

| Service | Port | Status | Docs |
|---------|------|--------|------|
| **community-service** | 8006 | ⏳ Planned | [README](services/community-service/README.md) · [API](services/community-service/API.md) · [DB](services/community-service/DATABASE.md) |
| **badge-service** | 8007 | ✅ Complete (7/7 endpoints) | [README](services/badge-service/README.md) · [API](services/badge-service/API.md) · [DB](services/badge-service/DATABASE.md) |
| **territory-service** | 8008 | ✅ Complete (6/6 endpoints) | [README](services/territory-service/README.md) · [API](services/territory-service/API.md) · [DB](services/territory-service/DATABASE.md) |
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

- **Version:** 20251113000005
- **Database:** PostgreSQL 15+ with TimescaleDB
- **Migrations Applied:** 5 core migrations (33 tables)
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

### Service Consistency & Best Practices

**ALL services MUST follow these standards:**

#### Shared Library Integration (REQUIRED)

**All services MUST use these shared-lib components:**

**Core Middleware (5 Required):**

1. ✅ `LoggingMiddleware::development()` - Request/response logging with timing
2. ✅ `RequestIdMiddleware` - Unique request ID tracking and propagation
3. ✅ `SecurityHeadersMiddleware::development()` - Security headers (X-Frame-Options, CSP, etc.)
4. ✅ `cors::development()` - CORS configuration (permissive in dev, strict in prod)
5. ✅ `RateLimitMiddleware::development(redis)` - Redis-based rate limiting

**Validation & Error Handling:**
6. ✅ `ValidatedJson`, `ValidatedPath`, `ValidatedQuery` - Automatic request validation
7. ✅ `error_response_handler` - Unified error response formatting
8. ✅ `AppError`, `Result<T>` - Consistent error types

**Authentication & Authorization:**

9. ✅ `AuthUser` - JWT authentication via FromRequest (no separate middleware)
10. ✅ **`PermissionChecker`** - Badge-based permission verification ✨ NEW
11. ✅ **`RequirePermission`** - Permission enforcement middleware ✨ NEW
12. ✅ **`RequireAnyPermission`** - Multi-permission middleware ✨ NEW

**When to Use Permission Middleware:**

Services SHOULD use permission middleware when:

- ✅ Endpoints require role-based access (admin, manager, moderator)
- ✅ Implementing management features (settings, user management, content moderation)
- ✅ Protecting sensitive operations (delete, ban, grant permissions)

Services SHOULD NOT use permission middleware when:

- ❌ Public endpoints (no authentication required)
- ❌ Self-service endpoints (user editing own profile)
- ❌ Basic CRUD for own resources (user viewing own badges)

**Permission Middleware Pattern:**

```rust
// Initialize PermissionChecker (in main.rs)
let permission_checker = PermissionChecker::new(database.clone(), "dk".to_string());

// Add to app data
App::new()
    .app_data(web::Data::new(permission_checker))
    
// Protect routes requiring specific permission
web::scope("/admin")
    .wrap(RequirePermission::new("portal:manage"))
    
// Protect routes requiring any of multiple permissions
web::scope("/moderate")
    .wrap(RequireAnyPermission::new(vec!["content:moderate", "portal:manage"]))
```

**See [services/shared-lib/PERMISSION.md](services/shared-lib/PERMISSION.md) for complete guide.**

**Inter-Service Communication:**

13. ✅ `NatsClient` - Event publishing/subscription
14. 🔧 `CircuitBreaker` - Prevent cascading failures (REQUIRED for HTTP calls to external services)

**Infrastructure:**

15. ✅ `Database` - PostgreSQL connection pool
16. ✅ `AppConfig` - Environment-based configuration
17. ✅ `shutdown_signal`, `shutdown_grace_period` - Graceful shutdown

**Configuration Requirements:**

- ✅ **Use AppConfig for all configuration** - Load via `AppConfig::from_env()`
- ✅ **Environment variables MUST use `APP__` prefix** - e.g., `APP__NATS__URL`, `APP__DATABASE__URL`
- ✅ **Double underscore (`__`) for nesting** - `APP__NATS__URL` maps to `AppConfig.nats.url`
- ✅ **No service-specific flat env vars** - Avoid `NATS_URL`, `DATABASE_URL`; use structured `APP__*__*` format
- ✅ **Access config values** - Use `config.nats_url()`, `config.database_url()`, not `env::var()`

**Service-Level Patterns (Each Service Implements):**

- 📋 Health endpoints: `/api/v1/health`, `/api/v1/ready`
- 📋 Metrics endpoint: `/api/v1/metrics` (Prometheus format - see standard metrics below)
- 📋 Event schemas: Define in `models/` with serde serialization
- 📋 OpenAPI docs: Use `utoipa` derive macros

**Standard Prometheus Metrics (Required at `/api/v1/metrics`):**

All services MUST expose these metrics in Prometheus format:

1. **Service Info** (gauge):

   ```
   {service}_info{version="x.x.x"} 1
   ```

2. **Database Pool Metrics** (gauges, if service uses database):

   ```
   {service}_db_pool_size
   {service}_db_pool_idle
   {service}_db_pool_active
   ```

3. **HTTP Request Metrics** (counter & histogram):

   ```
   {service}_http_requests_total{method, path, status}
   {service}_http_request_duration_seconds{method, path}
   ```

4. **Error Metrics** (counter):

   ```
   {service}_errors_total{type}
   ```

**Optional Metrics (service-specific):**

- Business metrics (e.g., `badge_service_badges_awarded_total`)
- NATS metrics (e.g., `{service}_nats_messages_published_total{subject}`)
- Cache metrics (e.g., `{service}_cache_hits_total`, `{service}_cache_misses_total`)
- Circuit breaker metrics (e.g., `{service}_circuit_breaker_state{name}`)

**See [services/shared-lib/MIDDLEWARE.md](services/shared-lib/MIDDLEWARE.md) for:**

- Complete implementation details and usage examples
- Phase 1 vs Phase 2 configuration differences
- Middleware execution order requirements
- Best practices and patterns

#### Code Structure & Naming

- ✅ **Follow Rust best practices** - snake_case for files/functions, CamelCase for types
- ✅ **Service structure pattern:**

  ```
  service-name/
  ├── src/
  │   ├── main.rs          # Server setup with ALL shared-lib middleware
  │   ├── lib.rs           # Public exports
  │   ├── handlers/        # HTTP handlers (snake_case files)
  │   ├── models/          # Request/Response types (camelCase JSON via serde)
  │   └── services/        # Business logic (snake_case)
  ├── Cargo.toml
  ├── .env / .env.example
  └── build.rs             # Version info generation
  ```

- ✅ **JSON naming:** Use `#[serde(rename_all = "camelCase")]` on all request/response models
- ✅ **Database naming:** snake_case for tables/columns (PostgreSQL standard)

#### API Endpoint Standards

- ✅ **Consistent URL structure:**
  - Health: `GET /api/v1/health`
  - Ready: `GET /api/v1/ready`
  - Metrics: `GET /api/v1/metrics`
  - Resources: `/api/v1/{resource-plural}/{id?}/{sub-resource?}`
  - Example: `GET /api/v1/badges/user/{user_id}`
  - Example: `GET /api/v1/user/profile/{user_id}`
- ✅ **HTTP methods:** GET (read), POST (create), PATCH (update), DELETE (remove)
- ✅ **Response format:** JSON with camelCase fields
- ✅ **Error format:** JSON with `{ "error": "message" }` (handled by AppError)
- ✅ **Authentication:** `Authorization: Bearer {token}` header, validated via `AuthUser` extractor

#### Middleware Stack (Required Order)

```rust
HttpServer::new(move || {
    App::new()
        // Priority 1: Request tracking
        .wrap(LoggingMiddleware::development())
        .wrap(RequestIdMiddleware)
        // Priority 2: Security
        .wrap(SecurityHeadersMiddleware::development())
        .wrap(cors::development())
        .wrap(RateLimitMiddleware::development(redis_client.clone()))
        // Shared state (required for handlers)
        .app_data(web::Data::new(database.clone()))
        .app_data(web::Data::new(nats_client.clone()))
        // OpenAPI documentation
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", openapi.clone())
        )
        // API routes
        .service(
            web::scope("/api/v1")
                .route("/health", web::get().to(health_check))
                .route("/ready", web::get().to(ready_check))
                .route("/metrics", web::get().to(metrics))
                .configure(your_service::handlers::configure)
        )
})
.bind(&server_addr)?
.run()
```

**Required Components:**

1. ✅ **All 5 middleware** in correct order (Logging, RequestId, Security, CORS, RateLimit)
2. ✅ **Database** via `.app_data(web::Data::new(database.clone()))`
3. ✅ **NATS client** via `.app_data(web::Data::new(nats_client.clone()))`
4. ✅ **Health endpoint** at `/api/v1/health`
5. ✅ **Ready endpoint** at `/api/v1/ready` for readiness probe
6. ✅ **Metrics endpoint** at `/api/v1/metrics` (Prometheus format)
7. ✅ **OpenAPI/Swagger UI** at `/swagger-ui/`
8. ✅ **Graceful shutdown** via `shutdown_signal().await` after server spawn

**Optional Components (Use When Applicable):**

- 🔧 **Circuit Breaker** via `.app_data(web::Data::new(circuit_breaker.clone()))` - REQUIRED when making HTTP calls to external services, NOT required for database-only services

**Why this order matters:**

1. **Logging + RequestId first** → All logs have correlation IDs
2. **Security headers** → Protect all responses
3. **CORS** → Handle OPTIONS preflight before auth
4. **Rate limiting** → Prevent abuse before processing
5. **App data before routes** → Make DB/NATS available to handlers
6. **Routes last** → Actual application logic

See [MIDDLEWARE.md](services/shared-lib/MIDDLEWARE.md#12-middleware-execution-order) for detailed explanation.

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
- ✅ user-service (100% - README, API, DATABASE + settings endpoints implemented)
- ❌ settings-service (merged into user-service for MVP simplicity)
- ✅ invitation-service (75% - README, API, DATABASE)
- ✅ notification-service (75% - README, API, DATABASE)

**Phase 2:**

- ⏳ community-service (50% - README, API, DATABASE)
- ✅ badge-service (100% - README, API, DATABASE + 7/7 endpoints implemented, NATS integrated)
- ✅ territory-service (100% - README, API, DATABASE + 6/6 endpoints implemented)
- ⏳ event-service (50% - README, API, DATABASE)
- ⏳ course-service (50% - README, API, DATABASE)
- ⏳ forum-service (50% - README, API, DATABASE)
- ⏳ translation-service (50% - README, API, DATABASE)
- ⏳ ipfs-service (50% - README, API, DATABASE)

**Overall Coverage:** ~80% (all services documented, auth + user + badge + territory fully implemented)

---

## 🗃️ Archived Documentation

Historical documentation from the initial planning phase (November 11, 2025) before the microservices structure was established.

**Location:** [.archived/](.archived/)

**Contents:** Consolidated API planning documents that were superseded by the service-specific documentation structure. Kept for historical reference.

See [.archived/README.md](.archived/README.md) for details.

---

## 🧪 Test Users

For development and testing, the following test users are available in the database:

| Username | Email | Password | Full Name | Role/Badge | Purpose |
|----------|-------|----------|-----------|------------|---------|
| `alice_admin` | <alice@unityplatform.test> | `SecurePass123!` | Alice Anderson | ✅ Platform Manager | Platform-level admin testing |
| `bob_manager` | <bob@unityplatform.test> | `SecurePass123!` | Bob Builder | Territory Manager* | Territory management testing |
| `carol_user` | <carol@unityplatform.test> | `SecurePass123!` | Carol Chen | User | Standard features testing |
| `david_dev` | <david@unityplatform.test> | `SecurePass123!` | David Developer | User | Development features testing |
| `emma_explorer` | <emma@unityplatform.test> | `SecurePass123!` | Emma Explorer | User | Community features testing |
| `frank_solo` | *(none)* | `SecurePass123!` | Frank Solo | User | Email-less account testing |

**Notes:**

- ✅ = Badge assigned via badge-service
- Roles marked with * will be assigned when needed
- All users are in territory `dk` (Denmark)
- Password is the same for all test accounts: `SecurePass123!`
- `frank_solo` demonstrates optional email functionality
- `alice_admin` has Platform Manager badge (awarded 2025-11-14)

**Quick Login Example:**

```bash
curl -X POST http://localhost:8001/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice_admin", "password": "SecurePass123!", "territory": "dk"}'
```

---

## 🚀 Getting Started

**New to the project?**

1. Read [overview/microservices-architecture.md](overview/microservices-architecture.md) for architecture overview
2. Check [MIGRATIONS-MASTER.md](MIGRATIONS-MASTER.md) for database structure
3. Browse [services/](services/) to understand individual services
4. See [../status/current/phase-1-status.md](../status/current/phase-1-status.md) for current progress

**Implementing a new service?**

1. **Follow the standard structure** - Use [services/user-service/](services/user-service/) or [services/badge-service/](services/badge-service/) as templates
2. **Include ALL shared-lib middleware** - See [services/shared-lib/MIDDLEWARE.md](services/shared-lib/MIDDLEWARE.md) for complete list
3. **Use consistent naming:**
   - Rust code: snake_case (files, functions, variables)
   - JSON API: camelCase (via `#[serde(rename_all = "camelCase")]`)
   - Database: snake_case (tables, columns)
   - URLs: kebab-case with plural resources
4. **Create documentation:**
   - `README.md` - Service overview, responsibilities, dependencies
   - `API.md` - Complete endpoint specifications with examples
   - `DATABASE.md` - Schema design with multi-pod considerations
   - `CHANGELOG.md` - Version history following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format
5. **Update master tracking:**
   - Add tables to [MIGRATIONS-MASTER.md](MIGRATIONS-MASTER.md)
   - Add service to this README's service table
   - Create migration plan if needed (MIGRATIONS.md)
6. **Validation checklist:**
   - ✅ Uses ALL 5 core middleware (logging, request-id, security, cors, rate-limit)
   - ✅ Uses `ValidatedJson` for request bodies with `validator` crate
   - ✅ Uses `AuthUser` for authentication (no custom JWT middleware)
     - **Exception:** auth-service itself does NOT use `AuthUser` - it is the authentication provider that issues JWTs for other services
   - ✅ Uses `AppError` and `Result<T>` for error handling
   - ✅ Implements health endpoints: `/api/v1/health`, `/api/v1/ready`
   - ✅ Implements metrics endpoint: `/api/v1/metrics` with standard Prometheus metrics:
     - Service info (version)
     - Database pool metrics (size, idle, active) if applicable
     - HTTP request metrics (total, duration)
     - Error metrics (total by type)
   - ✅ Follows API endpoint naming: `/api/v1/{resource}/{id?}`
   - ✅ Uses NATS for event publishing/subscription (see [NATS Events Reference](services/shared-lib/NATS-EVENTS.md) for standard event types and naming)
   - ✅ Enforce NATS security best practices as defined in [NATS Events Reference](services/shared-lib/NATS-EVENTS.md), Security Considerations section and Best Practices section
   - ✅ Includes OpenAPI/Swagger documentation via `utoipa`
   - ✅ Implements graceful shutdown with `shutdown_signal()`
   - ✅ **Uses workspace dependencies:** All dependencies use `{ workspace = true }` (see [Dependency Management Guide](../guides/development/dependency-management.md))
7. **Input Validation:**
   - Validate event payloads on consumption
   - Sanitize for SQL injection and XSS
   - Use `validator` crate for request validation
   - Handle validation errors with `AppError::BadRequest`
8. **Testing:**
   - Write unit and integration tests as per [Testing Strategy](../guides/development/testing-strategy.md)

**Making changes to architecture?**

1. Update relevant overview/ documentation for architectural changes
2. Update service-specific docs for implementation changes
3. Keep documentation in sync with code
4. Update this README.md if structure changes

---

**Last Updated:** November 14, 2025  
**Maintained By:** Development Team  
**Status:** Living Documentation (updated continuously)
