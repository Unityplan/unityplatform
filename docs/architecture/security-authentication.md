# Security & Authentication Architecture

**Last Updated:** November 26, 2025  
**Status:** Implemented in Phase 1 MVP

---

## Overview

The Unity Platform uses a JWT-based authentication system with badge-based authorization. This document describes the security architecture across all microservices.

## Authentication Flow

```
┌─────────────┐         ┌──────────────┐         ┌─────────────────┐
│   Client    │────────▶│ Auth Service │────────▶│ Badge Service   │
│  (Frontend) │         │              │         │ (read badges)   │
└─────────────┘         └──────────────┘         └─────────────────┘
       │                       │
       │                       ▼
       │                ┌──────────────┐
       │                │  JWT Token   │
       │                │ with badges  │
       │                └──────────────┘
       │                       │
       ▼                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Other Microservices                          │
│  (Community, Invitation, Territory, Badge, User, etc.)         │
│                                                                 │
│  - Extract AuthUser from JWT                                    │
│  - Validate badges from claims (no DB lookup)                   │
│  - Check service-specific permissions                           │
└─────────────────────────────────────────────────────────────────┘
```

## JWT Claims Structure

```rust
pub struct Claims {
    pub sub: Uuid,           // User ID
    pub territory: String,   // Territory code (e.g., "dk")
    pub badges: Vec<String>, // Badge slugs (e.g., ["code-of-conduct", "platform-manager"])
    pub exp: u64,            // Expiration timestamp
    pub iat: u64,            // Issued at timestamp
}
```

### Badge Slugs in JWT

When a user logs in or refreshes their token, the auth-service:

1. Fetches all active badges for the user
2. Includes badge slugs in the JWT claims
3. Token is valid for 15 minutes
4. Refresh token updates badges automatically

**Benefits:**

- No database lookup needed for permission checks
- Low latency authorization decisions
- Badges validated locally by each service

## AuthUser Extractor

All services use the `AuthUser` extractor from `shared-lib`:

```rust
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub territory: String,
    pub badges: Vec<String>,
}

impl AuthUser {
    /// Check if the user has a specific badge
    pub fn has_badge(&self, badge_slug: &str) -> bool { ... }

    /// Check if the user has all required badges
    pub fn has_all_badges(&self, required_slugs: &[&str]) -> bool { ... }

    /// Check if the user has any of the specified badges
    pub fn has_any_badge(&self, badge_slugs: &[&str]) -> bool { ... }
}
```

## Service Authentication Matrix

### Auth Service

| Endpoint | Auth Required | Additional Checks |
|----------|--------------|-------------------|
| `POST /register` | No | Validates invitation token |
| `POST /login` | No | Validates credentials |
| `POST /refresh` | Refresh token | Validates refresh token |
| `POST /logout` | Refresh token | - |
| `POST /validate` | Bearer token | - |

### Community Service

| Endpoint | Auth Required | Additional Checks |
|----------|--------------|-------------------|
| `GET /communities` | Yes | - |
| `POST /communities` | Yes | - |
| `GET /communities/{id}` | Yes | - |
| `PUT /communities/{id}` | Yes | Must be admin |
| `POST /communities/{id}/join` | Yes | **Must have required badges** |
| `POST /communities/{id}/leave` | Yes | - |
| `GET /communities/{id}/managers` | Yes | - |
| `POST /communities/{id}/managers` | Yes | Must be admin |
| `DELETE /communities/{id}/managers/{user_id}` | Yes | Must be admin |
| `GET /communities/{id}/requirements` | Yes | - |
| `POST /communities/{id}/requirements` | Yes | Must be admin |
| `DELETE /communities/{id}/requirements` | Yes | Must be admin |

### Badge Service

| Endpoint | Auth Required | Additional Checks |
|----------|--------------|-------------------|
| `GET /badges` | Optional | - |
| `POST /badges/register` | Yes | **Must have `platform-manager` badge** |
| `GET /badges/users/{user_id}` | Yes | - |
| `POST /badges/award` | Yes | **Must have `platform-manager` badge** |
| `POST /badges/revoke` | Yes | **Must have `platform-manager` badge** |
| `POST /badges/progress` | Yes | System/service use |
| `PATCH /badges/featured` | Yes | Own badges only |

### Invitation Service

| Endpoint | Auth Required | Additional Checks |
|----------|--------------|-------------------|
| `POST /invitations/validate` | No | Service-to-service |
| `POST /invitations/use` | No | Service-to-service |
| `POST /invitations` | Yes | Must be manager |
| `GET /invitations/me` | Yes | - |
| `GET /invitations/{id}/uses` | Yes | Must be creator or manager |
| `DELETE /invitations/{id}` | Yes | Must be creator or manager |

### Territory Service

| Endpoint | Auth Required | Additional Checks |
|----------|--------------|-------------------|
| `GET /territories` | Yes | - |
| `GET /territories/{code}` | Yes | - |
| `GET /territories/{code}/manage/stats` | Yes | Must be territory manager |
| `PATCH /territories/{code}/manage/settings` | Yes | Must be territory manager |

### User Service

| Endpoint | Auth Required | Additional Checks |
|----------|--------------|-------------------|
| All endpoints | Yes | User-specific access controls |

## Badge-Based Authorization

### Key Badges

| Badge Slug | Purpose | Required For |
|------------|---------|--------------|
| `platform-manager` | Platform administration | Register/award/revoke badges |
| `territory-manager` | Territory management | Territory stats, settings |
| `community-manager` | Community management | Create invitations, manage communities |
| `code-of-conduct` | Platform access | Basic platform features |

### Community Badge Requirements

Communities can require specific badges for joining:

```rust
// In join_community handler
let requirements = service.get_effective_requirements(community_id).await?;

let required_slugs: Vec<&str> = requirements
    .iter()
    .filter(|r| r.badge_slug != "code-of-conduct")
    .map(|r| r.badge_slug.as_str())
    .collect();

if !auth_user.has_all_badges(&required_slugs) {
    return Err(AppError::Forbidden(format!(
        "Missing required badges: {}",
        missing.join(", ")
    )));
}
```

### Inherited Requirements

Child communities inherit badge requirements from parent communities:

```
Group (requires: "mycology-certified")
├── Study Group (inherits: "mycology-certified")
└── Guild (inherits: "mycology-certified")
```

## Error Responses

### 401 Unauthorized

Missing or invalid JWT token.

```json
{
  "error": "Unauthorized",
  "message": "Missing Authorization header"
}
```

### 403 Forbidden

User authenticated but lacks required permissions.

```json
{
  "error": "Forbidden",
  "message": "Missing required badges: mycology-certified"
}
```

## Security Best Practices

1. **Never store JWT in localStorage** - Use httpOnly cookies or memory
2. **Short token expiry** - 15 minutes for access tokens
3. **Badge validation from JWT** - No database lookups for permission checks
4. **Refresh updates badges** - Users get new badges on next token refresh
5. **Service isolation** - Each service validates its own permissions
6. **No cross-service FKs** - Services validate via JWT, not database joins

## Future Enhancements

1. **Service-to-service authentication** - Mutual TLS or service tokens
2. **Rate limiting** - Per-user and per-IP limits
3. **Audit logging** - Track administrative actions
4. **Token revocation** - Immediate logout capability
