# Shared Authentication Strategy

**Last Updated:** November 12, 2025  
**Version:** 0.1.0-alpha.1  
**Status:** Architecture Defined (Implementation Pending)

---

## Overview

Unity Platform uses **JWT-based authentication** with a shared middleware approach across all microservices. This document defines the authentication strategy, validation flow, and implementation patterns for all services within a pod.

---

## Architecture Principles

### 1. Stateless Authentication

**All services validate JWTs locally without database queries (99% of requests).**

```
Request → JWT Middleware → Validate Signature → Extract Claims → Handler
                               (0.01ms)           (no DB call)
```

**Benefits:**

- ✅ **Blazing fast:** 0.01-0.1ms per request
- ✅ **Zero database load:** No auth queries for standard operations
- ✅ **Scales infinitely:** Each service validates independently
- ✅ **Stateless:** No session storage required

---

### 2. Shared Database Context

**All services within a pod share ONE PostgreSQL database.**

```
Denmark Pod:
┌─────────────────────────────────────┐
│ PostgreSQL: unityplatform_dk            │
├─────────────────────────────────────┤
│ territory_dk.users          ← auth  │
│ territory_dk.users_profiles ← user  │
│ territory_dk.communities    ← comm  │
└─────────────────────────────────────┘
     ↑         ↑         ↑
     │         │         │
  auth-svc  user-svc  comm-svc
```

**Implication:** Services CAN query `territory_dk.users` directly when needed (critical operations only).

---

### 3. Two-Tier Validation

**Standard Requests (99%):** JWT validation only  
**Critical Operations (1%):** JWT + database check

---

## JWT Structure

### Token Format

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Claims Structure

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,              // user_id (subject)
    pub territory: String,      // territory_code (e.g., "dk", "no", "se")
    pub exp: i64,               // expiration timestamp (Unix time)
    pub iat: i64,               // issued at timestamp (Unix time)
    pub jti: Option<String>,    // JWT ID (for revocation tracking)
}
```

**Example JWT payload:**

```json
{
  "sub": "550e8400-e29b-41d4-a716-446655440000",
  "territory": "dk",
  "exp": 1731427200,
  "iat": 1731426300,
  "jti": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
}
```

### Token Lifetime

- **Access Token (JWT):** 15 minutes
- **Refresh Token (UUID):** 30 days (stored in database)

**Rationale:**

- Short-lived access tokens minimize security risk
- Refresh tokens enable session management without re-authentication
- 15-minute window acceptable for logout/deletion delays

---

## Validation Approaches

### Approach 1: JWT Signature Validation (Standard)

**Used for:** 99% of requests (reads, standard writes)

```rust
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

pub async fn jwt_auth_middleware(
    req: ServiceRequest,
) -> Result<ServiceRequest, Error> {
    // Extract token from Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| ErrorUnauthorized("Missing or invalid Authorization header"))?;
    
    // Validate JWT signature (cryptographic check only)
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;  // Check expiration
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &validation
    )
    .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;
    
    // Attach claims to request
    req.extensions_mut().insert(AuthUser {
        id: token_data.claims.sub,
        territory: token_data.claims.territory,
    });
    
    Ok(req)
}
```

**Performance:** ~0.01ms (cryptographic signature validation only)

**No database query!** User existence checked at login, JWT signature proves authenticity.

---

### Approach 2: JWT + Database Validation (Critical)

**Used for:** Security-critical operations (account deletion, password change, role changes)

```rust
pub async fn jwt_with_db_validation(
    req: ServiceRequest,
    pool: Data<PgPool>,
) -> Result<ServiceRequest, Error> {
    // Step 1: Validate JWT signature (same as Approach 1)
    let token = extract_bearer_token(&req)?;
    let claims = validate_jwt_signature(token)?;
    
    // Step 2: Check database for user status
    let user = sqlx::query!(
        r#"
        SELECT 
            id,
            username,
            deleted_at,
            is_active
        FROM territory_dk.users
        WHERE id = $1
        "#,
        claims.sub
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| ErrorInternalServerError("Database error"))?;
    
    // Validate user exists and is active
    match user {
        Some(u) if u.deleted_at.is_none() && u.is_active => {
            req.extensions_mut().insert(AuthUser {
                id: u.id,
                territory: claims.territory,
                username: u.username,
            });
            Ok(req)
        },
        Some(_) => Err(ErrorForbidden("Account inactive or deleted")),
        None => Err(ErrorUnauthorized("User not found")),
    }
}
```

**Performance:** ~0.5-2ms (JWT validation + database query)

**Use when:**

- User requests account deletion
- User changes password (invalidate other sessions)
- Admin modifies user roles/permissions
- Security-sensitive operations requiring real-time user status

---

### Approach 3: JWT + Cached Validation (Optimized)

**Used for:** Frequent operations needing recent user status (optional optimization)

```rust
use cached::proc_macro::cached;

#[cached(time = 60, result = true, key = "Uuid", convert = r#"{ user_id }"#)]
async fn is_user_active_cached(user_id: Uuid, pool: &PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 
            FROM territory_dk.users 
            WHERE id = $1 
              AND deleted_at IS NULL 
              AND is_active = true
        ) as "exists!"
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn jwt_cached_validation(
    req: ServiceRequest,
    pool: Data<PgPool>,
    nats: Data<NatsClient>,
) -> Result<ServiceRequest, Error> {
    let token = extract_bearer_token(&req)?;
    let claims = validate_jwt_signature(token)?;
    
    // Check cache (hits 99% of time)
    if !is_user_active_cached(claims.sub, pool.get_ref()).await? {
        return Err(ErrorForbidden("User not active"));
    }
    
    req.extensions_mut().insert(AuthUser {
        id: claims.sub,
        territory: claims.territory,
    });
    
    Ok(req)
}

// Cache invalidation via NATS events
async fn start_cache_invalidation_listener(nats: NatsClient) {
    nats.subscribe("user.deleted", |event: UserDeletedEvent| {
        cached::CACHED.lock().remove(&event.user_id);
        tracing::info!("Cache invalidated for deleted user: {}", event.user_id);
    }).await;
    
    nats.subscribe("user.deactivated", |event: UserDeactivatedEvent| {
        cached::CACHED.lock().remove(&event.user_id);
        tracing::info!("Cache invalidated for deactivated user: {}", event.user_id);
    }).await;
}
```

**Performance:**

- Cache hit (99%): ~0.1ms
- Cache miss (1%): ~2ms
- NATS event invalidation: Immediate

**Trade-off:** Adds complexity (caching + NATS events) for minimal gain over Approach 1.

---

## Implementation Patterns

### Service Middleware Setup

**All services use the shared JWT middleware:**

```rust
// user-service/src/main.rs
use shared_lib::middleware::jwt_auth_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect(&config.database_url).await?;
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(jwt_auth_middleware)  // ← JWT validation for ALL routes
            .service(get_profile)       // Protected
            .service(update_profile)    // Protected
            .service(delete_account)    // Protected (+ DB check in handler)
    })
    .bind("0.0.0.0:8002")?
    .run()
    .await
}
```

---

### Handler Pattern: Standard Request

**No database check needed - trust the JWT:**

```rust
// GET /v1/profiles/{username}
async fn get_profile(
    auth: AuthUser,  // Extracted by middleware
    path: Path<String>,
    pool: Data<PgPool>
) -> Result<HttpResponse> {
    let username = path.into_inner();
    
    // NO auth validation needed - JWT already validated by middleware
    let profile = sqlx::query_as!(
        ProfileResponse,
        r#"
        SELECT 
            user_id,
            display_name,
            bio,
            avatar_url
        FROM territory_dk.users_profiles
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ErrorNotFound("Profile not found"))?;
    
    Ok(HttpResponse::Ok().json(profile))
}
```

**Performance:** Database query for profile data only, no auth check overhead.

---

### Handler Pattern: Critical Operation

**Database check required for security:**

```rust
// DELETE /v1/account
async fn delete_account(
    auth: AuthUser,  // Extracted by middleware (JWT validated)
    pool: Data<PgPool>,
    nats: Data<NatsClient>
) -> Result<HttpResponse> {
    // Additional database check for critical operation
    let user = sqlx::query!(
        r#"
        SELECT 
            id,
            username,
            deleted_at,
            is_active
        FROM territory_dk.users
        WHERE id = $1
        "#,
        auth.id
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ErrorNotFound("User not found"))?;
    
    // Check if already deleted
    if user.deleted_at.is_some() {
        return Err(ErrorBadRequest("Account already deleted"));
    }
    
    // Check if active
    if !user.is_active {
        return Err(ErrorForbidden("Account is inactive"));
    }
    
    // Soft delete user
    sqlx::query!(
        "UPDATE territory_dk.users SET deleted_at = NOW() WHERE id = $1",
        auth.id
    )
    .execute(pool.get_ref())
    .await?;
    
    // Publish NATS event for cleanup
    nats.publish("user.deleted", UserDeletedEvent {
        user_id: auth.id,
        username: user.username,
        deleted_at: Utc::now(),
    }).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "message": "Account deletion initiated"
    })))
}
```

**Performance:** +2ms for database check, acceptable for critical operation.

---

## Cross-Service Authentication

### Same Pod (Shared Database)

**Services query database directly - NO API calls:**

```rust
// user-service validates user exists (same pod)
let user_exists = sqlx::query_scalar!(
    "SELECT EXISTS(SELECT 1 FROM territory_dk.users WHERE id = $1)",
    user_id
)
.fetch_one(&pool)
.await?;
```

**Performance:** ~0.5ms (local database query)

---

### Cross-Pod (Different Territories)

**Services call other pod's API:**

```rust
// Denmark pod validating Norway user
async fn validate_cross_pod_user(user_id: Uuid, territory: &str) -> Result<bool> {
    let url = format!("https://{}.unityplatform.org/api/v1/users/{}/exists", territory, user_id);
    
    let response = reqwest::get(&url).await?;
    let exists = response.json::<bool>().await?;
    
    Ok(exists)
}
```

**Performance:** ~10-50ms (HTTPS API call to other pod)

**Use when:** Cross-territory features (following users in other countries)

---

## Error Responses

### Missing Token

```http
HTTP/1.1 401 Unauthorized
Content-Type: application/json

{
  "error": "unauthorized",
  "message": "Missing or invalid Authorization header"
}
```

---

### Invalid/Expired Token

```http
HTTP/1.1 401 Unauthorized
Content-Type: application/json

{
  "error": "unauthorized",
  "message": "Invalid or expired token"
}
```

---

### User Deleted/Inactive (Critical Operations)

```http
HTTP/1.1 403 Forbidden
Content-Type: application/json

{
  "error": "forbidden",
  "message": "Account inactive or deleted"
}
```

---

## Security Considerations

### 1. Token Revocation

**Challenge:** JWTs can't be revoked before expiration.

**Mitigations:**

1. **Short TTL:** 15-minute access tokens limit exposure window
2. **Refresh Token Rotation:** Refresh tokens stored in database, can be revoked
3. **NATS Events:** `user.deleted` / `user.deactivated` invalidate caches
4. **Database Check:** Critical operations verify user status in real-time

**Acceptable Trade-off:** 15-minute window where deleted user's JWT still works is acceptable for MVP given the security benefits of stateless auth.

---

### 2. Secret Management

**Symmetric Keys (HS256):**

```bash
# Same secret across all services (simpler)
JWT_SECRET=your-256-bit-secret-key-here
```

**Pros:** Simple, fast  
**Cons:** Compromised service can forge tokens

---

**Asymmetric Keys (RS256) - Recommended:**

```bash
# auth-service only
JWT_PRIVATE_KEY=/secrets/jwt-private.pem

# All other services
JWT_PUBLIC_KEY=/secrets/jwt-public.pem
```

**Pros:** Compromised service can't forge tokens (only verify)  
**Cons:** Slightly slower, more complex setup

**Recommendation:** Use RS256 for production, HS256 for MVP development.

---

### 3. JWT Best Practices

✅ **Always validate expiration (`exp` claim)**  
✅ **Use HTTPS only** (prevent token interception)  
✅ **Store tokens in httpOnly cookies** (prevent XSS)  
✅ **Implement refresh token rotation** (detect token theft)  
✅ **Add `jti` claim** (track token issuance for revocation)  
✅ **Rate limit login attempts** (prevent brute force)

---

## Performance Benchmarks

### JWT Validation Only (Approach 1)

```
1,000 requests/sec × 0.01ms = 10ms total CPU time
Database queries: 0
Memory: Negligible
```

**Use for:** 99% of requests

---

### JWT + Database Check (Approach 2)

```
1,000 requests/sec × 2ms = 2,000ms total CPU time
Database queries: 1,000/sec
Connection pool: Moderate pressure
```

**Use for:** 1% of requests (critical operations)

---

### JWT + Cached Check (Approach 3)

```
1,000 requests/sec × 0.1ms = 100ms total CPU time
Database queries: ~10/sec (1% cache miss)
Cache hits: 99%
Memory: ~100KB for 10,000 users
```

**Use for:** Optional optimization if Approach 1 insufficient

---

## Migration Path

### Phase 1: MVP (Current)

- ✅ JWT validation in all services (Approach 1)
- ✅ Database check for critical operations only
- ✅ Symmetric keys (HS256) for simplicity

---

### Phase 2: Multi-Pod

- ✅ Same JWT approach works across pods
- ✅ Cross-pod validation via HTTPS API
- ✅ Global token blacklist (Redis) for immediate revocation

---

### Phase 3: Holochain

- ✅ JWT approach compatible with Holochain
- ✅ Cryptographic signatures align with Holochain DHT
- ✅ User sovereignty maintained (users control tokens)

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jwt_validation_valid_token() {
        let token = create_test_jwt(user_id, "dk", 900);
        let claims = validate_jwt_signature(&token).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.territory, "dk");
    }
    
    #[test]
    fn test_jwt_validation_expired_token() {
        let token = create_test_jwt(user_id, "dk", -10);  // Expired
        let result = validate_jwt_signature(&token);
        assert!(result.is_err());
    }
}
```

---

### Integration Tests

```rust
#[actix_web::test]
async fn test_protected_endpoint_without_token() {
    let app = test::init_service(
        App::new()
            .wrap(jwt_auth_middleware)
            .service(get_profile)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/v1/profiles/alice")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);  // Unauthorized
}

#[actix_web::test]
async fn test_protected_endpoint_with_valid_token() {
    let token = create_test_jwt(user_id, "dk", 900);
    
    let app = test::init_service(
        App::new()
            .wrap(jwt_auth_middleware)
            .service(get_profile)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/v1/profiles/alice")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}
```

---

## Troubleshooting

### "Invalid or expired token" Error

**Causes:**

1. Token expired (check `exp` claim)
2. Invalid signature (wrong secret key)
3. Malformed token (missing Bearer prefix)

**Debug:**

```bash
# Decode JWT (https://jwt.io)
echo "eyJhbGc..." | base64 -d

# Check expiration
date -d @<exp_timestamp>
```

---

### "User not found" Error (Critical Operations)

**Causes:**

1. User deleted between JWT issuance and request
2. User soft-deleted (`deleted_at` set)
3. Database connection issue

**Debug:**

```sql
-- Check user status
SELECT id, username, deleted_at, is_active 
FROM territory_dk.users 
WHERE id = '<user_id>';
```

---

## Summary

### Authentication Strategy

| Request Type | Validation | Performance | Use Case |
|-------------|-----------|-------------|----------|
| **Standard** | JWT only | 0.01ms | 99% of requests |
| **Critical** | JWT + DB | 2ms | Account deletion, security ops |
| **Optimized** | JWT + Cache | 0.1ms | Optional enhancement |

### Key Principles

1. ✅ **Stateless by default** - JWT validation without database
2. ✅ **Shared database context** - Services can query `users` table when needed
3. ✅ **Two-tier validation** - Standard vs critical operations
4. ✅ **Multi-pod ready** - Same pattern works across territories
5. ✅ **Holochain compatible** - Cryptographic signatures align with future

### Performance Impact

- **Standard requests:** Zero database overhead (~0.01ms)
- **Critical operations:** +2ms for real-time user status check
- **Overall:** 99% of requests have near-zero auth overhead

---

## References

- [JWT.io - JWT Debugger](https://jwt.io/)
- [jsonwebtoken crate](https://docs.rs/jsonwebtoken/)
- [RFC 7519 - JSON Web Token](https://datatracker.ietf.org/doc/html/rfc7519)
- [OWASP JWT Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_for_Java_Cheat_Sheet.html)
