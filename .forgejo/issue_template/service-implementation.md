---
name: Service Implementation
about: Track implementation of a new microservice
title: 'Service: [SERVICE_NAME]'
labels: 'type/feature,type/infrastructure'
---

## 🎯 Service Overview

**Service Name:** `[service-name]`  
**Purpose:** <!-- What does this service do? -->  
**Stage:** <!-- Phase 1 Stage number (7-13) -->  
**Priority:** <!-- High / Medium / Low -->

## 📋 Service Specification

**Responsibilities:**
- 
- 
- 

**NOT Responsible For:**
- 
- 

## 🗄️ Database Schema

**Tables:**
```sql
-- Example schema
CREATE TABLE territory_dk.table_name (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Global Tables:**
```sql
-- If applicable
```

**Migrations:**
- [ ] Create migration file in `services/shared-lib/migrations/`
- [ ] Test migration up/down
- [ ] Document schema changes

## 🔌 API Endpoints

**Base Path:** `/api/v1/[service]`

### Endpoints

| Method | Path | Description | Auth |
|--------|------|-------------|------|
| GET | `/` | List resources | Required |
| GET | `/:id` | Get resource | Required |
| POST | `/` | Create resource | Required |
| PATCH | `/:id` | Update resource | Required |
| DELETE | `/:id` | Delete resource | Required |

## 🏗️ Implementation Checklist

### 1. Service Scaffolding
- [ ] Create `services/[service-name]/` directory
- [ ] Create `Cargo.toml` with dependencies
- [ ] Create `src/main.rs` with server setup
- [ ] Create `src/lib.rs` for public exports
- [ ] Add to workspace `Cargo.toml`

### 2. Project Structure
- [ ] Create `src/handlers/` - HTTP request handlers
- [ ] Create `src/models/` - Request/Response types
- [ ] Create `src/services/` - Business logic
- [ ] Create `tests/` - Integration tests
- [ ] Create `README.md` - Service documentation

### 3. Middleware Stack
- [ ] Add `LoggingMiddleware` (development/production)
- [ ] Add `RequestIdMiddleware`
- [ ] Add `SecurityHeadersMiddleware`
- [ ] Add `cors::development()` / `cors::production()`
- [ ] Add `RateLimitMiddleware` with Redis
- [ ] Test middleware chain

### 4. Database Integration
- [ ] Add `shared_lib::Database` connection
- [ ] Implement territory-aware queries
- [ ] Add connection pooling
- [ ] Test database operations
- [ ] Add error handling

### 5. API Implementation
- [ ] Implement all endpoints (see table above)
- [ ] Add request validation with `ValidatedJson`
- [ ] Add proper error responses
- [ ] Add OpenAPI/Swagger documentation
- [ ] Test all endpoints

### 6. Business Logic
- [ ] Implement core service logic
- [ ] Add validation rules
- [ ] Add authorization checks
- [ ] Add inter-service communication (if needed)
- [ ] Handle edge cases

### 7. Testing
- [ ] Unit tests for business logic
- [ ] Integration tests for API endpoints
- [ ] Test error handling
- [ ] Test validation rules
- [ ] Test database transactions
- [ ] Achieve >80% code coverage

### 8. Documentation
- [ ] Update service README.md
- [ ] Document API endpoints
- [ ] Document database schema
- [ ] Add code comments
- [ ] Update architecture docs

### 9. Docker Integration
- [ ] Add Dockerfile for service
- [ ] Add to `docker-compose.dev.yml`
- [ ] Add to `docker-compose.pod.yml`
- [ ] Configure environment variables
- [ ] Test container deployment

### 10. Deployment
- [ ] Add health check endpoint (`/health`)
- [ ] Add metrics endpoint (`/metrics`)
- [ ] Configure logging
- [ ] Add to monitoring
- [ ] Test in dev environment

## 🔗 Dependencies

**Depends On:**
- [ ] #<!-- Issue number: Foundation/Infrastructure -->
- [ ] #<!-- Issue number: Database schema -->
- [ ] #<!-- Issue number: Auth service -->

**Blocks:**
- [ ] #<!-- Issue number: Frontend integration -->

## 📚 Reference Documentation

**Architecture Docs:**
- `docs/architecture/services/[service-name]/README.md`
- `docs/architecture/services/[service-name]/API.md`
- `docs/architecture/services/[service-name]/DATABASE.md`

**Similar Services:**
- Check `services/auth-service/` for authentication patterns
- Check `services/user-service/` for CRUD patterns
- Check `services/badge-service/` for reference implementation

## 🚀 Environment Variables

```bash
# Service-specific config
[SERVICE]_PORT=8XXX
[SERVICE]_LOG_LEVEL=info
[SERVICE]_DATABASE_URL=postgresql://...
[SERVICE]_REDIS_URL=redis://...
```

## ✅ Acceptance Criteria

- [ ] All API endpoints implemented and documented
- [ ] Database migrations created and tested
- [ ] Tests passing with >80% coverage
- [ ] Docker container builds and runs
- [ ] Health check endpoint working
- [ ] Integrated with monitoring
- [ ] Documentation complete
- [ ] Code reviewed and approved

## 💡 Implementation Notes

<!-- Add any specific notes, decisions, or considerations -->

## ✔️ Checklist

- [ ] I have reviewed similar service implementations
- [ ] I have checked for existing issues
- [ ] I have added all required labels
- [ ] I have linked dependencies
- [ ] I have defined acceptance criteria
