# Auth Service Rebuild Plan

**Date:** November 11, 2025  
**Status:** ✅ COMPLETED - All endpoints tested and working  
**Completion Date:** November 11, 2025  
**Old Service:** Backed up to `auth-service.old/`

---

## Architectural Principles (From Lessons Learned)

### Database

- ✅ Database: `unityplan` (NOT `unityplan_dk`)
- ✅ Schemas: `global` + `territory_dk` (schema-based multi-tenancy)
- ✅ Email: OPTIONAL (privacy-first, not used for authentication)
- ✅ Username: PRIMARY identifier (login credential, globally unique, never changes)
- ✅ Matrix ID: DERIVED from `@username:territory` (changes with migration)

### API Structure

- ✅ Base route: `/v1/auth` (NOT `/api/v1/auth`)
- ✅ Response envelope: `{ success, data, error, meta }`
- ✅ OpenAPI/Swagger: Individual service at `/swagger-ui` and `/api-docs`
- ✅ Health endpoint: `/health` with dependency checks

### Authentication Flow

- ✅ Login: username + password (NOT email)
- ✅ Tokens: JWT access token + refresh token
- ✅ Invitation: Territory-specific tokens validated against `global.invitation_token_registry`
- ✅ Registration: Requires valid invitation token

---

## New Service Structure

```
auth-service/
├── Cargo.toml               ✅ Created with all dependencies
├── src/
│   ├── main.rs              ✅ Main server entry point
│   ├── lib.rs               ✅ Library exports
│   ├── config.rs            ✅ Configuration from env
│   ├── error.rs             ✅ Custom error types
│   ├── response.rs          ✅ API response envelope
│   ├── models/              ✅ Request/response models
│   │   ├── mod.rs
│   │   ├── auth.rs          - Login, register, token models
│   │   └── health.rs        - Health check response
│   ├── handlers/            ✅ HTTP request handlers
│   │   ├── mod.rs
│   │   ├── auth.rs          - register, login, logout, refresh, validate
│   │   └── health.rs        - Health endpoint
│   ├── services/            ✅ Business logic
│   │   ├── mod.rs
│   │   ├── token.rs         - JWT generation/validation
│   │   ├── password.rs      - Argon2 hashing/verification
│   │   └── invitation.rs    - Token validation
│   ├── middleware/          ✅ HTTP middleware
│   │   ├── mod.rs
│   │   └── auth.rs          - JWT authentication middleware
│   └── openapi.rs           ✅ OpenAPI/Swagger configuration
└── tests/                   🔲 Integration tests (future)
    └── api_tests.rs
```

---

## Endpoints to Implement (From API Guide)

### Public Endpoints - ✅ ALL COMPLETED

1. **POST /v1/auth/register** ✅
   - Request: username, password, invitation_token, email (optional), display_name, accept_terms
   - Validates invitation against `global.invitation_token_registry`
   - Creates user in `territory_dk.users`
   - Registers username in `global.username_registry`
   - Registers email (if provided) in `global.email_registry`
   - **Tested:** alice user created successfully

2. **POST /v1/auth/login** ✅
   - Request: username, password
   - Response: access_token, refresh_token, user info
   - **Tested:** Returns JWT tokens successfully

3. **POST /v1/auth/refresh** ✅
   - Request: refresh_token
   - Response: new access_token, new refresh_token
   - **Tested:** Successfully generates new tokens

4. **POST /v1/auth/logout** ✅
   - Requires: Authorization header
   - Invalidates refresh token
   - **Tested:** Sets revoked_at timestamp correctly

5. **GET /v1/auth/validate** ✅
   - Requires: Authorization header
   - Response: token validity status
   - **Tested:** Correctly validates JWT claims

### System Endpoints - ✅ COMPLETED

6. **GET /health** ✅
   - Response: service status + dependencies (database, nats, redis)
   - **Tested:** Returns healthy status

7. **GET /swagger-ui** ✅
   - OpenAPI UI
   - **Tested:** All schemas properly defined (UserInfo added)

8. **GET /api-docs/openapi.json** ✅
   - OpenAPI 3.0 spec

---

## Database Tables Used

### Global Schema

```sql
-- Token-to-territory mapping
global.invitation_token_registry (
    token VARCHAR PRIMARY KEY,
    territory_code VARCHAR,
    territory_token_id UUID
)

-- Global username uniqueness
global.username_registry (
    username VARCHAR PRIMARY KEY,
    user_id UUID,
    territory_code VARCHAR
)

-- Global email uniqueness (if provided)
global.email_registry (
    email VARCHAR PRIMARY KEY,
    user_id UUID,
    territory_code VARCHAR,
    is_verified BOOLEAN
)

-- Territory registry
global.territories (
    code VARCHAR PRIMARY KEY,
    name VARCHAR,
    is_active BOOLEAN
)
```

### Territory Schema (territory_dk)

```sql
-- User authentication
territory_dk.users (
    id UUID PRIMARY KEY,
    username VARCHAR UNIQUE,
    email VARCHAR NULLABLE,
    password_hash VARCHAR,
    territory_code VARCHAR DEFAULT 'dk',
    is_active BOOLEAN,
    is_verified BOOLEAN
)

-- Invitation tokens
territory_dk.invitation_tokens (
    id UUID PRIMARY KEY,
    token VARCHAR UNIQUE,
    token_type VARCHAR,  -- 'single_use' or 'group'
    email VARCHAR NULLABLE,
    max_uses INT,
    current_uses INT,
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN
)
```

---

## Response Envelope Structure

```rust
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub meta: ResponseMeta,
}

#[derive(Serialize, ToSchema)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
pub struct ResponseMeta {
    pub timestamp: String,
    pub request_id: String,
}
```

---

## JWT Structure

```rust
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id (UUID)
    pub username: String,
    pub territory: String,
    pub exp: usize,
    pub iat: usize,
}
```

---

## Configuration (Environment Variables)

```env
# Database
DATABASE_URL=postgresql://unityplan:password@localhost:5432/unityplan

# Territory (for schema routing)
TERRITORY_CODE=dk

# JWT
JWT_SECRET=your-secret-key
JWT_ACCESS_TTL=900  # 15 minutes
JWT_REFRESH_TTL=604800  # 7 days

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8001

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000
```

---

## Implementation Order

### Phase 1: Core Setup ✅ COMPLETED

1. ✅ Cargo.toml - Dependencies configured
2. ✅ src/error.rs - Error types
3. ✅ src/response.rs - API envelope
4. ✅ src/config.rs - Environment config
5. ✅ src/models/ - All request/response models
6. ✅ src/openapi.rs - Swagger configuration (UserInfo schema added)

### Phase 2: Services ✅ COMPLETED

7. ✅ src/services/password.rs - Argon2 hashing
8. ✅ src/services/token.rs - JWT generation/validation
9. ✅ src/services/invitation.rs - Token validation

### Phase 3: Handlers ✅ COMPLETED

10. ✅ src/handlers/health.rs - Health check
11. ✅ src/handlers/auth.rs - All auth endpoints
12. ✅ src/middleware/auth.rs - JWT middleware

### Phase 4: Integration ✅ COMPLETED

13. ✅ src/main.rs - Wire everything together
14. ✅ Tested with Swagger UI - All endpoints working
15. 🔲 Integration tests (future enhancement)

### Phase 5: Database Schema Fixes ✅ COMPLETED

16. ✅ Added missing refresh_tokens table to documentation
17. ✅ Added refresh_tokens table to migration
18. ✅ Updated Rust code to match database schema (token_hash → token, revoked_at)
19. ✅ Applied migration successfully (31 tables in territory_dk)
20. ✅ All endpoints tested and verified working

---

## Key Differences from Old Service

### ❌ Old (Removed)

- `/api/v1/auth/*` routes → `/v1/auth/*`
- `/me` endpoint → Moved to user-service
- `global.user_identities` table → Uses `global.username_registry` + `global.email_registry`
- Database `unityplan_dk` → `unityplan`
- Email required for registration → Email optional
- Simple JSON responses → API envelope

### ✅ New (Added)

- Swagger/OpenAPI integration
- API response envelope
- Proper health checks with dependencies
- `/v1/auth/validate` endpoint
- Email optional throughout
- Consistent with API implementation guide

---

## Completion Summary

### ✅ What Was Achieved

1. **Complete Auth Service Rebuild**
   - All core files implemented (config, error, response, models, handlers, services, middleware)
   - Clean architecture following best practices
   - Full OpenAPI/Swagger documentation

2. **Database Schema Alignment**
   - Documentation updated as source of truth
   - Migration file synchronized with documentation
   - Rust code updated to match database schema
   - refresh_tokens table properly implemented with revocation support

3. **All Endpoints Working**
   - Registration with invitation tokens
   - Login with JWT generation
   - Token refresh functionality
   - Token validation
   - Logout with token revocation
   - Health checks with database connectivity
   - Swagger UI accessible without errors

4. **Testing Completed**
   - Health check: ✅ Service healthy
   - Register: ✅ User "alice" created
   - Login: ✅ Tokens returned
   - Validate: ✅ JWT decoded correctly
   - Refresh: ✅ New tokens generated
   - Logout: ✅ Token revoked (revoked_at set)
   - Revocation: ✅ Revoked tokens rejected

### 📋 Future Enhancements

- Integration test suite
- Rate limiting
- Failed login attempt tracking
- Password reset flow
- Email verification flow
- Multi-factor authentication

---

## Service Information

**Service:** auth-service  
**Version:** 0.1.0-alpha.1  
**Port:** 8001  
**Base URL:** <http://localhost:8001>  
**Swagger UI:** <http://localhost:8001/swagger-ui>  
**Database:** unityplan (schemas: global, territory_dk)  
**Status:** ✅ Production ready for MVP Phase 1
