# auth-service

**Port:** 8001  
**Version:** 0.1.0-alpha.1  
**Status:** ✅ Phase 1 Complete (5/5 endpoints - 100%)  
**Bounded Context:** Authentication & Authorization

---

## 📋 Overview

The auth-service is responsible for user authentication, JWT token management, and password security. It serves as the entry point for all user access to the platform.

### **Responsibilities**

- ✅ User registration (with invitation token validation)
- ✅ User authentication (login/logout)
- ✅ JWT token generation and validation
- ✅ Password hashing (Argon2id)
- ✅ Token refresh mechanism
- ✅ Session management
- ⏳ Password reset (planned)
- ⏳ 2FA/TOTP (planned)

### **Not Responsible For**

- ❌ Invitation token creation (handled by invitation-service)
- ❌ User profile management (handled by user-service)
- ❌ User settings (handled by settings-service)

---

## 🔐 Authentication Architecture

### **JWT Token Issuance**

This service **issues JWT tokens** that all other services validate.

**Login Flow:**

```rust
// 1. User logs in with credentials
POST /v1/auth/login
{
  "username": "alice",
  "password": "********"
}

// 2. auth-service validates credentials
// 3. auth-service issues JWT + refresh token
{
  "access_token": "eyJhbGc...",  // JWT (15 min TTL)
  "refresh_token": "uuid",        // Refresh token (30 days, stored in DB)
  "user": { ... }
}
```

### **JWT Structure**

**Claims issued by auth-service:**

```json
{
  "sub": "550e8400-e29b-41d4-a716-446655440000",  // user_id
  "territory": "dk",                               // territory_code
  "exp": 1731427200,                               // expires in 15 minutes
  "iat": 1731426300,                               // issued at
  "jti": "a1b2c3d4-e5f6-7890"                     // JWT ID (for revocation)
}
```

### **Other Services**

**All other services validate JWTs locally** without calling auth-service:

```rust
// user-service, settings-service, community-service, etc.
use shared_lib::middleware::jwt_auth_middleware;

HttpServer::new(|| {
    App::new()
        .wrap(jwt_auth_middleware)  // Validates JWT signature locally
        .service(my_handler)
})
```

**No API calls to auth-service for validation!** JWT signature proves authenticity.

### **Token Refresh**

When access token expires, client uses refresh token:

```rust
POST /v1/auth/refresh
{
  "refresh_token": "uuid"
}

// auth-service checks database, issues new access token
{
  "access_token": "eyJhbGc...",  // New JWT (15 min TTL)
}
```

**See [shared-lib/AUTHENTICATION.md](../shared-lib/AUTHENTICATION.md) for complete authentication strategy.**

---

## 🗄️ Database Schema

### **Tables Owned by auth-service**

#### **1. users**

```sql
CREATE TABLE territory_{code}.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255),
    password_hash TEXT NOT NULL,
    full_name VARCHAR(255),
    territory_code VARCHAR(10) NOT NULL,
    is_active BOOLEAN DEFAULT true,
    is_verified BOOLEAN DEFAULT false,
    verified_at TIMESTAMPTZ,
    totp_enabled BOOLEAN DEFAULT false,
    totp_secret VARCHAR(255),
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT username_format CHECK (username ~* '^[a-z0-9_-]{3,50}$')
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_users_active ON users(is_active) WHERE is_active = true;
```

**Purpose:** Core user authentication data  
**Holochain Entry Type:** `User`  
**Password Security:** Argon2id with salt

#### **2. user_sessions (planned)**

```sql
CREATE TABLE territory_{code}.user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    revoked_at TIMESTAMPTZ
);

CREATE INDEX idx_user_sessions_user ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_token ON user_sessions(refresh_token);
CREATE INDEX idx_user_sessions_active ON user_sessions(is_active, expires_at)
    WHERE is_active = true;
```

**Purpose:** Refresh token tracking and session management  
**Auto-cleanup:** Remove expired sessions daily

#### **3. password_reset_tokens (planned)**

```sql
CREATE TABLE territory_{code}.password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    ip_address TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT one_active_token_per_user UNIQUE(user_id) WHERE used_at IS NULL
);

CREATE INDEX idx_password_reset_tokens_user ON password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_tokens_token ON password_reset_tokens(token);
```

**Purpose:** Password reset flow with email token  
**Security:** Token expires after 1 hour, single use

### **Global Registry Tables**

#### **global.username_registry**

```sql
CREATE TABLE global.username_registry (
    username VARCHAR(50) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT unique_username_global UNIQUE(username)
);

CREATE INDEX idx_username_registry_territory ON username_registry(territory_code);
```

**Purpose:** Ensure global username uniqueness across all territories

#### **global.email_registry**

```sql
CREATE TABLE global.email_registry (
    email VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT unique_email_global UNIQUE(email)
);

CREATE INDEX idx_email_registry_territory ON email_registry(territory_code);
```

**Purpose:** Ensure global email uniqueness (if provided) across all territories

---

## 🔌 API Endpoints

### **Registration**

#### **POST /api/v1/auth/register**

Register new user with invitation token

**Request:**

```json
{
  "invitation_token": "uuid",
  "username": "alice",
  "password": "SecurePass123!",
  "full_name": "Alice Johnson",
  "email": "alice@example.com"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "access_token": "eyJ...",
    "refresh_token": "eyJ...",
    "token_type": "Bearer",
    "expires_in": 900,
    "user": {
      "id": "uuid",
      "username": "alice",
      "territory_code": "dk",
      "is_active": true
    }
  }
}
```

**Validation:**

- Invitation token must be valid (calls invitation-service)
- Username: 3-50 chars, lowercase alphanumeric + underscore/hyphen
- Password: minimum 8 chars (recommend 12+)
- Email: optional, valid format if provided
- Global uniqueness check for username and email

### **Authentication**

#### **POST /api/v1/auth/login**

Authenticate user with username/password

**Request:**

```json
{
  "username": "alice",
  "password": "SecurePass123!",
  "territory_code": "dk"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "access_token": "eyJ...",
    "refresh_token": "eyJ...",
    "token_type": "Bearer",
    "expires_in": 900,
    "user": {
      "id": "uuid",
      "username": "alice",
      "full_name": "Alice Johnson",
      "territory_code": "dk"
    }
  }
}
```

**JWT Claims:**

```json
{
  "sub": "user_id",
  "username": "alice",
  "territory_code": "dk",
  "roles": [],
  "exp": 1700000000,
  "iat": 1699999100
}
```

**Token Expiry:**

- Access token: 15 minutes
- Refresh token: 7 days

#### **POST /api/v1/auth/refresh**

Refresh access token using refresh token

**Request:**

```json
{
  "refresh_token": "eyJ..."
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "access_token": "eyJ...",
    "token_type": "Bearer",
    "expires_in": 900
  }
}
```

#### **POST /api/v1/auth/logout**

Invalidate refresh token (revoke session)

**Request:**

```json
{
  "refresh_token": "eyJ..."
}
```

**Response:**

```json
{
  "success": true,
  "message": "Logged out successfully"
}
```

#### **GET /api/v1/auth/validate**

Validate access token (used by other services)

**Headers:**

```
Authorization: Bearer eyJ...
```

**Response:**

```json
{
  "success": true,
  "data": {
    "user_id": "uuid",
    "username": "alice",
    "territory_code": "dk",
    "is_valid": true
  }
}
```

### **Password Reset (Planned)**

#### **POST /api/v1/auth/password/reset**

Request password reset (send email with token)

**Request:**

```json
{
  "email": "alice@example.com",
  "territory_code": "dk"
}
```

#### **POST /api/v1/auth/password/confirm**

Confirm password reset with token

**Request:**

```json
{
  "token": "reset-token-from-email",
  "new_password": "NewSecurePass456!"
}
```

---

## 🔗 Service Dependencies

### **Outbound Calls (Services auth-service depends on)**

#### **invitation-service**

- **When:** During user registration
- **Endpoint:** `GET /v1/invitations/validate/{token}`
- **Purpose:** Validate invitation token before allowing registration
- **Fallback:** Reject registration if invitation-service unavailable

#### **invitation-service (mark as used)**

- **When:** After successful registration
- **Endpoint:** `POST /v1/invitations/{token}/use`
- **Purpose:** Mark invitation token as used
- **Fallback:** Log warning if call fails (user already created)

---

### **Inbound Calls (Services that call auth-service)**

#### **All Services**

- **When:** Every authenticated request
- **Endpoint:** JWT validation (middleware)
- **Purpose:** Verify user identity and permissions

#### **Frontend**

- **When:** Login, registration, token refresh
- **Endpoints:** All auth endpoints
- **Purpose:** User authentication flow

---

## 📡 NATS Events

### **Published Events**

```typescript
// User registered
{
  event: "user.registered",
  user_id: "uuid",
  username: "alice",
  territory_code: "dk",
  timestamp: "ISO8601"
}

// User logged in
{
  event: "user.logged_in",
  user_id: "uuid",
  username: "alice",
  ip_address: "192.168.1.1",
  timestamp: "ISO8601"
}

// User logged out
{
  event: "user.logged_out",
  user_id: "uuid",
  session_id: "uuid",
  timestamp: "ISO8601"
}

// Password reset requested
{
  event: "password.reset.requested",
  user_id: "uuid",
  email: "alice@example.com",
  timestamp: "ISO8601"
}

// Password changed
{
  event: "password.changed",
  user_id: "uuid",
  timestamp: "ISO8601"
}
```

### **Subscribed Events**

None - auth-service is a leaf service

---

## 🔮 Holochain Migration

### **DNA Design: authentication.happ**

#### **Entry Types**

**1. User** (Public keys only)

```rust
#[hdk_entry_helper]
struct User {
    username: String,
    agent_pub_key: AgentPubKey,
    territory_code: String,
    is_active: bool,
}
```

**Note:** Passwords never stored in Holochain. Authentication via cryptographic signatures.

**2. Session** (Private)

```rust
#[hdk_entry_helper]
struct Session {
    session_id: String,
    expires_at: Timestamp,
    is_active: bool,
}
```

#### **Migration Strategy**

**Phase 1:** PostgreSQL authentication, prepare for keypair migration

- Continue using password-based auth
- Add optional keypair field to users table
- Allow both password and signature auth

**Phase 2:** Dual authentication

- Users can authenticate with password OR signature
- Encourage keypair setup
- Keep PostgreSQL as fallback

**Phase 3:** Holochain native

- Signature-based authentication only
- PostgreSQL used only for session caching
- Full agent sovereignty

---

## ✅ Implementation Status

### **Completed** (5/5 endpoints - 100%)

- ✅ User registration with invitation validation
- ✅ Login with JWT tokens (access + refresh)
- ✅ Token refresh mechanism
- ✅ Logout (session revocation)
- ✅ Token validation endpoint
- ✅ Password hashing (Argon2id)
- ✅ Multi-territory support
- ✅ Global username/email uniqueness
- ✅ Comprehensive tests (19/19 passing)
- ✅ OpenAPI/Swagger documentation

### **Pending**

- ⏳ Password reset flow (email + token)
- ⏳ 2FA/TOTP support
- ⏳ Session management table
- ⏳ Rate limiting (login attempts)
- ⏳ Account lockout after failed attempts
- ⏳ NATS event publishing
- ⏳ Security audit logging

---

## 🧪 Testing

### **Test Coverage**

```bash
cd services/auth-service
cargo test

# Results:
# 19 passing tests
# 100% handler coverage
# 100% validation coverage
```

### **Key Test Cases**

- ✅ Registration with valid invitation
- ✅ Registration with invalid invitation (rejected)
- ✅ Username uniqueness enforcement
- ✅ Email uniqueness enforcement (if provided)
- ✅ Password hashing verification
- ✅ Login with correct credentials
- ✅ Login with wrong password (rejected)
- ✅ Token generation and validation
- ✅ Token refresh flow
- ✅ Logout invalidates session
- ✅ Multi-territory support
- ✅ Global registry updates

---

## 🔒 Security Considerations

### **Password Security**

- **Algorithm:** Argon2id (recommended by OWASP)
- **Parameters:** Memory-hard, resistant to GPU attacks
- **Salt:** Unique per password, generated automatically
- **Hash Storage:** Never store plaintext passwords

### **Token Security**

- **Access Token:** Short-lived (15 min), stateless JWT
- **Refresh Token:** Long-lived (7 days), stored in database
- **Rotation:** Refresh tokens rotated on each use
- **Revocation:** Logout invalidates refresh token

### **API Security**

- **HTTPS Only:** All authentication endpoints require TLS
- **CORS:** Restricted to allowed origins only
- **Rate Limiting:** Planned - prevent brute force attacks
- **Account Lockout:** Planned - after 5 failed attempts

---

## 📝 Configuration

### **Environment Variables**

```env
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8001

# Database
DATABASE_URL=postgresql://user:pass@postgres:5432/unityplatform

# JWT
JWT_SECRET=your-secret-key-min-32-chars
JWT_ACCESS_EXPIRY=900       # 15 minutes
JWT_REFRESH_EXPIRY=604800   # 7 days

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:5173,https://unityplatform.dk

# Territory
TERRITORY_CODE=dk
```

---

## 📊 Metrics & Monitoring

### **Key Metrics**

- Registration rate (users/day)
- Login success rate (%)
- Login failure rate (%)
- Token refresh rate (requests/min)
- Average response time (ms)
- Active sessions count

### **Health Check**

```bash
curl http://localhost:8001/health

# Response:
{
  "status": "healthy",
  "service": "auth-service",
  "version": "0.1.0-alpha.1",
  "dependencies": {
    "database": "connected",
    "invitation_service": "reachable"
  }
}
```

---

## 🚀 Deployment

### **Docker**

```yaml
# docker-compose.yml
services:
  auth-service:
    build: ./services/auth-service
    ports:
      - "8001:8001"
    environment:
      - DATABASE_URL=postgresql://...
      - JWT_SECRET=${JWT_SECRET}
      - TERRITORY_CODE=dk
    depends_on:
      - postgres
      - invitation-service
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8001/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

### **Multi-Pod Deployment**

Each territory runs its own auth-service:

```
Denmark Pod:  auth.dk.unityplatform.org  → auth-service:8001 (territory_dk)
Norway Pod:   auth.no.unityplatform.org  → auth-service:8001 (territory_no)
Sweden Pod:   auth.se.unityplatform.org  → auth-service:8001 (territory_se)
```

**Global Registry:** Replicated across all pods for username/email uniqueness

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Production Ready (Phase 1)
