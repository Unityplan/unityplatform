# auth-service API Endpoints

**Base URL:** `http://localhost:8001`  
**Version:** v1  
**Status:** ✅ Production Ready (5/5 endpoints complete)

---

## Authentication Endpoints

### 1. Register User

**Endpoint:** `POST /api/v1/auth/register`  
**Authentication:** None (public)  
**Status:** ✅ Implemented

**Description:** Register a new user account with invitation token validation.

**Request Body:**

```json
{
  "username": "alice",
  "email": "alice@example.com",
  "password": "SecurePassword123!",
  "invitation_token": "A7K9-M2X4-P5W8-Q1Z3",
  "territory_code": "dk"
}
```

**Validation:**

- `username`: 3-30 characters, alphanumeric + underscore
- `email`: Valid email format
- `password`: Min 8 characters, must include uppercase, lowercase, number, special char
- `invitation_token`: Valid invitation code (checked against invitation-service)
- `territory_code`: Valid territory code (dk, no, se, eu)

**Response (201 Created):**

```json
{
  "success": true,
  "data": {
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "email": "alice@example.com",
    "territory_code": "dk",
    "created_at": "2025-11-12T10:00:00Z"
  }
}
```

**Errors:**

- `400 Bad Request` - Invalid input data
- `409 Conflict` - Username or email already exists
- `422 Unprocessable Entity` - Invalid invitation token

---

### 2. Login

**Endpoint:** `POST /api/v1/auth/login`  
**Authentication:** None (public)  
**Status:** ✅ Implemented

**Description:** Authenticate user and receive JWT tokens.

**Request Body:**

```json
{
  "username": "alice",
  "password": "SecurePassword123!",
  "territory_code": "dk"
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 900,
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "alice",
      "email": "alice@example.com",
      "territory_code": "dk"
    }
  }
}
```

**Token Details:**

- `access_token`: Valid for 15 minutes
- `refresh_token`: Valid for 7 days

**Errors:**

- `400 Bad Request` - Missing username/password
- `401 Unauthorized` - Invalid credentials
- `404 Not Found` - User not found in territory

---

### 3. Refresh Token

**Endpoint:** `POST /api/v1/auth/refresh`  
**Authentication:** Refresh token required  
**Status:** ✅ Implemented

**Description:** Obtain a new access token using refresh token.

**Request Body:**

```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 900
  }
}
```

**Errors:**

- `400 Bad Request` - Missing refresh token
- `401 Unauthorized` - Invalid or expired refresh token

---

### 4. Logout

**Endpoint:** `POST /api/v1/auth/logout`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Invalidate user session and revoke tokens.

**Request Headers:**

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response (200 OK):**

```json
{
  "success": true,
  "message": "Logged out successfully"
}
```

**Errors:**

- `401 Unauthorized` - Invalid or missing token

---

### 5. Validate Token

**Endpoint:** `GET /api/v1/auth/validate`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Validate JWT token and return user info (internal service use).

**Request Headers:**

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "valid": true,
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "territory_code": "dk",
    "expires_at": "2025-11-12T10:15:00Z"
  }
}
```

**Errors:**

- `401 Unauthorized` - Invalid or expired token

---

## Password Reset Endpoints (Planned)

### 6. Request Password Reset

**Endpoint:** `POST /api/v1/auth/password/reset`  
**Authentication:** None (public)  
**Status:** ⏳ Planned

**Description:** Request password reset email with token.

**Request Body:**

```json
{
  "email": "alice@example.com",
  "territory_code": "dk"
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "message": "Password reset email sent if account exists"
}
```

---

### 7. Confirm Password Reset

**Endpoint:** `POST /api/v1/auth/password/confirm`  
**Authentication:** None (public)  
**Status:** ⏳ Planned

**Description:** Reset password using token from email.

**Request Body:**

```json
{
  "token": "abc123def456",
  "new_password": "NewSecurePassword123!",
  "territory_code": "dk"
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "message": "Password reset successfully"
}
```

---

## Response Format

All responses follow this structure:

**Success:**

```json
{
  "success": true,
  "data": { ... }
}
```

**Error:**

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable error message"
  }
}
```

---

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_CREDENTIALS` | 401 | Wrong username/password |
| `USER_NOT_FOUND` | 404 | User doesn't exist in territory |
| `USERNAME_EXISTS` | 409 | Username already taken |
| `EMAIL_EXISTS` | 409 | Email already registered |
| `INVALID_TOKEN` | 401 | JWT token invalid or expired |
| `INVALID_INVITATION` | 422 | Invitation token invalid |
| `VALIDATION_ERROR` | 400 | Input validation failed |

---

## Testing

```bash
# Register new user
curl -X POST http://localhost:8001/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "SecurePassword123!",
    "invitation_token": "A7K9-M2X4-P5W8-Q1Z3",
    "territory_code": "dk"
  }'

# Login
curl -X POST http://localhost:8001/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "SecurePassword123!",
    "territory_code": "dk"
  }'

# Validate token
curl -X GET http://localhost:8001/api/v1/auth/validate \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

# Logout
curl -X POST http://localhost:8001/api/v1/auth/logout \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

---

**Last Updated:** November 12, 2025  
**Service Version:** 0.1.0-alpha.1  
**Implementation Status:** 5/7 endpoints (71%)  
**Tests:** 19/19 passing
