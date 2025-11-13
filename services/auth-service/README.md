# Auth Service

Authentication service for the Unity Platform. Handles user registration, login, token refresh, and JWT validation.

## Features

- ✅ User registration with validation
- ✅ Login with password verification (Argon2id)
- ✅ JWT access tokens (15 minute expiration)
- ✅ Refresh tokens (7 day expiration)
- ✅ Token validation endpoint for other services
- ✅ Global username/email registry (cross-territory uniqueness)
- ✅ Territory-aware user storage
- ✅ Graceful degradation (invitation validation optional)
- ✅ Complete middleware stack (logging, security, CORS, rate limiting)

## API Endpoints

### POST /api/v1/auth/register

Register a new user.

**Request:**

```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "secure_password_123",
  "territory": "dk",
  "invitation_token": "optional-token-uuid"
}
```

**Response (201):**

```json
{
  "access_token": "eyJhbGci...",
  "refresh_token": "uuid-v4",
  "token_type": "Bearer",
  "expires_in": 900
}
```

### POST /api/v1/auth/login

Login an existing user.

**Request:**

```json
{
  "username": "johndoe",
  "password": "secure_password_123",
  "territory": "dk"
}
```

**Response (200):**

```json
{
  "access_token": "eyJhbGci...",
  "refresh_token": "uuid-v4",
  "token_type": "Bearer",
  "expires_in": 900
}
```

### POST /api/v1/auth/refresh

Refresh an access token using a refresh token.

**Request:**

```json
{
  "refresh_token": "uuid-v4"
}
```

**Response (200):**

```json
{
  "access_token": "eyJhbGci...",
  "token_type": "Bearer",
  "expires_in": 900
}
```

### POST /api/v1/auth/logout

Logout and invalidate refresh token.

**Request:**

```json
{
  "refresh_token": "uuid-v4"
}
```

**Response (200):**

```json
{
  "message": "Logged out successfully"
}
```

### GET /api/v1/auth/validate

Validate a JWT token (for inter-service communication).

**Headers:**

```
Authorization: Bearer eyJhbGci...
```

**Response (200):**

```json
{
  "valid": true,
  "user_id": "uuid-v4",
  "territory": "dk"
}
```

### GET /health

Health check endpoint.

**Response (200):**

```json
{
  "status": "ok",
  "database": "healthy"
}
```

## Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
# Server
SERVER__HOST=0.0.0.0
AUTH_SERVICE_PORT=8001

# Database
DATABASE__URL=postgresql://unity_user:unity_pass@postgres:5432/unityplatform

# Redis (for rate limiting)
REDIS_URL=redis://redis:6379

# JWT
JWT_SECRET=your-secret-key-here

# Feature Flags
ENABLE_INVITATION_VALIDATION=false  # Set true when invitation-service exists
```

## Development

### Build

```bash
cargo build -p auth-service
```

### Run

```bash
cargo run -p auth-service
```

### Test

```bash
cargo test -p auth-service
```

## Architecture

### Directory Structure

```
src/
├── handlers/          # HTTP request handlers
│   ├── auth.rs       # Authentication endpoints
│   └── mod.rs
├── models/           # Request/Response types
│   ├── auth.rs      # Auth-related DTOs
│   └── mod.rs
├── services/         # Business logic
│   ├── password.rs  # Password hashing (Argon2id)
│   ├── token.rs     # JWT generation/validation
│   └── mod.rs
├── lib.rs           # Library exports
└── main.rs          # Server setup with middleware
```

### Middleware Stack (from shared-lib)

**Priority 1:** Request tracking

- `RequestIdMiddleware` - Generates unique request IDs
- `LoggingMiddleware::development()` - Verbose request/response logging

**Priority 2:** Security and limits

- `SecurityHeadersMiddleware::development()` - Security headers
- `cors::development()` - Permissive CORS for development
- `RateLimitMiddleware::development()` - 100 requests/minute per IP

### Database Schema

**Territory-specific tables:**

- `territory_{code}.users` - User accounts
- `territory_{code}.refresh_tokens` - Refresh token storage

**Global tables:**

- `global.username_registry` - Cross-territory username uniqueness
- `global.email_registry` - Cross-territory email uniqueness

See `services/shared-lib/migrations/20251112000003_auth_core_tables.sql` for full schema.

## Dependencies

- `actix-web` - Web framework
- `sqlx` - Async PostgreSQL driver
- `argon2` - Password hashing
- `jsonwebtoken` - JWT generation/validation
- `validator` - Request validation
- `shared-lib` - Shared middleware and utilities

## Security

- **Password Hashing:** Argon2id with random salts
- **JWT Secret:** Configure via `JWT_SECRET` environment variable
- **Token Expiration:** 15 minutes (access), 7 days (refresh)
- **Rate Limiting:** Redis-backed distributed rate limiting
- **Input Validation:** All requests validated with `validator` crate
- **SQL Injection:** Protected by `sqlx` parameterized queries

## Graceful Degradation

The service can operate without `invitation-service`:

- Set `ENABLE_INVITATION_VALIDATION=false` (default)
- Registration proceeds without invitation token validation
- Warning logged when invitation checks are disabled
- When invitation-service is available, set to `true`

## Future Enhancements

- [ ] Password reset flow
- [ ] Email verification
- [ ] Two-factor authentication (2FA)
- [ ] Session management
- [ ] Account lockout after failed attempts
- [ ] Integration with invitation-service
- [ ] OAuth2/OIDC provider support
