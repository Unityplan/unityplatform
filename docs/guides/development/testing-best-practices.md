# Testing Best Practices - Auth Service

## Overview

This document outlines critical testing principles discovered during auth-service development, particularly regarding integration testing with middleware.

**Last Updated:** 2025-11-08  
**Status:** Active - Based on auth-service integration test implementation

---

## Critical Principle: Test Production Configuration

### The Middleware Problem

**❌ WRONG - Testing Without Middleware:**

```rust
// This tests a configuration that doesn't exist in production
let app = test::init_service(
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::from(token_service.clone()))
        .route("/api/auth/invitations", 
               web::post().to(create_invitation))
)
.await;
```

**Issues:**

- No JWT middleware runs
- Authentication flow is bypassed
- `get_authenticated_user(&req)` fails with 401
- Tests pass scenarios that would fail in production
- Tests don't validate security model

**✅ CORRECT - Testing With Middleware:**

```rust
// This mirrors the exact production configuration
let app = test::init_service(
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::from(token_service.clone()))
        .service(
            web::scope("/api/auth")
                .route("/login", web::post().to(login))
                .service(
                    web::scope("/invitations")
                        .wrap(auth_service::middleware::JwtAuth)  // ✅ Production middleware
                        .route("", web::post().to(create_invitation))
                )
        ),
)
.await;
```

**Benefits:**

- JWT validation runs like production
- Invalid/expired tokens are rejected
- Request extensions properly populated
- Handlers receive authenticated user context
- Security model is fully tested
- Catches validation errors (e.g., wrong enum values)

---

## How JWT Authentication Works

### Production Flow

1. **Request arrives** with `Authorization: Bearer <token>` header
2. **JwtAuth middleware intercepts** (`src/middleware/auth.rs`)
   - Extracts and validates JWT signature
   - Checks token expiration
   - Loads user from database
   - Fetches public_key_hash from global.user_identities
3. **Middleware populates request extensions:**

   ```rust
   req.extensions_mut().insert(AuthenticatedUser {
       user_id: user.id,
       username: user.username,
       territory_code: claims.territory_code,
       public_key_hash,
   });
   ```

4. **Handler executes:**

   ```rust
   pub async fn create_invitation(req: HttpRequest, ...) -> Result<HttpResponse> {
       let auth_user = get_authenticated_user(&req)?;  // ✅ Data is there
       // ... handler logic
   }
   ```

### Test Flow (With Middleware)

Same as production! The test:

1. Logs in to get a real JWT token
2. Sends request with `Authorization: Bearer <token>` header
3. Middleware validates token and populates request
4. Handler receives authenticated user context
5. **Validates the complete end-to-end flow**

### Test Flow (Without Middleware)

**Broken flow:**

1. Test sends request with bearer token
2. No middleware runs - request goes directly to handler
3. Handler calls `get_authenticated_user(&req)`
4. Request extensions are empty (middleware never populated them)
5. Returns 401 Unauthorized ❌

---

## Production vs Test Configuration

### Auth Service (main.rs)

```rust
HttpServer::new(move || {
    App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::from(token_service.clone()))
        .service(
            web::scope("/api/auth")
                // Public endpoints
                .route("/register", web::post().to(handlers::register))
                .route("/login", web::post().to(handlers::login))
                .route("/refresh", web::post().to(handlers::refresh))
                .route("/logout", web::post().to(handlers::logout))
                .route("/invitations/validate/{token}", 
                       web::get().to(handlers::validate_invitation))
                
                // Protected endpoints
                .service(
                    web::scope("")
                        .wrap(middleware::JwtAuth)  // 🔐 JWT required
                        .route("/me", web::get().to(handlers::me)),
                )
                .service(
                    web::scope("/invitations")
                        .wrap(middleware::JwtAuth)  // 🔐 JWT required
                        .route("", web::post().to(handlers::create_invitation))
                        .route("", web::get().to(handlers::list_invitations))
                        .route("/{id}", web::delete().to(handlers::revoke_invitation))
                        .route("/{id}/uses", web::get().to(handlers::get_invitation_usage))
                )
        )
        .route("/health", web::get().to(handlers::health))
})
```

### Integration Tests (Must Match!)

```rust
let app = test::init_service(
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::from(token_service.clone()))
        .service(
            web::scope("/api/auth")
                // Public endpoints
                .route("/login", web::post().to(auth_service::handlers::auth::login))
                .route("/register", web::post().to(auth_service::handlers::auth::register))
                
                // Protected endpoints - MUST use middleware
                .service(
                    web::scope("/invitations")
                        .wrap(auth_service::middleware::JwtAuth)  // ✅ Same as production
                        .route("", web::post().to(auth_service::handlers::invitation::create_invitation))
                        .route("", web::get().to(auth_service::handlers::invitation::list_invitations))
                )
        ),
)
.await;
```

---

## Database Schema Testing

### Multi-Schema Architecture

Auth service uses a dual-schema pattern:

**Global Schema (`global`):**

- `territories` - Territory definitions
- `user_identities` - Cross-territory user identity
  - `public_key_hash` - User's cryptographic identity
  - `territory_code` - Home territory
  - `territory_user_id` - References territory user table
- `sessions` - Session tokens (global, not per-territory)
  - `user_id` FK → `global.user_identities.id`

**Territory Schemas (`territory_dk`, `territory_no`, etc.):**

- `users` - Territory-specific user data
  - Profile information
  - Authentication credentials
  - Preferences
- `invitation_tokens` - Territory invitation system
- `invitation_uses` - Invitation usage tracking

### Handler Pattern

**Registration Flow:**

1. Create user in territory schema
2. Create identity in global schema with `public_key_hash`
3. Create session in global.sessions

**Login Flow:**

1. Validate credentials against territory user
2. Fetch `public_key_hash` from global.user_identities
3. Generate JWT with both territory and global data
4. Store session in global.sessions

**Token Refresh:**

1. Validate refresh token from global.sessions
2. Fetch user from global.user_identities
3. Load territory user data
4. Rotate session token in global.sessions

---

## Test Data Management

### Helper Functions

Tests should use helper functions that create data in BOTH schemas:

```rust
async fn create_test_user(pool: &PgPool, schema: &str) -> (String, String) {
    let email = format!("test_{}@test.dk", Uuid::new_v4());
    let username = format!("user_{}", Uuid::new_v4());
    let password = "TestPass123!";
    let password_hash = hash_password(password);
    
    // 1. Create territory user
    let territory_user_id: Uuid = sqlx::query_scalar(&format!(
        "INSERT INTO {}.users (email, username, password_hash, ...) 
         VALUES ($1, $2, $3, ...) RETURNING id",
        schema
    ))
    .bind(&email)
    .bind(&username)
    .bind(&password_hash)
    .fetch_one(pool)
    .await
    .unwrap();
    
    // 2. Create global identity
    let public_key_hash = format!("{:x}", sha2::Sha256::digest(email.as_bytes()));
    
    sqlx::query(
        "INSERT INTO global.user_identities 
         (public_key_hash, territory_code, territory_user_id) 
         VALUES ($1, $2, $3)"
    )
    .bind(&public_key_hash)
    .bind("dk")
    .bind(territory_user_id)
    .execute(pool)
    .await
    .unwrap();
    
    (email, password.to_string())
}
```

### Cleanup

Clean both global and territory schemas:

```rust
async fn cleanup_test_data(pool: &PgPool) {
    // Clean territory data
    sqlx::query("DELETE FROM territory_dk.invitation_uses").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM territory_dk.invitation_tokens").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM territory_dk.users").execute(pool).await.unwrap();
    
    // Clean global data
    sqlx::query("DELETE FROM global.sessions").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM global.user_identities 
                 WHERE LOWER(territory_code) = 'dk'")
        .execute(pool).await.unwrap();
}
```

---

## Key Lessons

1. **Integration tests MUST mirror production configuration**
   - Use the same middleware wrapping
   - Use the same route structure
   - Test the complete request flow

2. **Middleware is part of the security model**
   - Tests without middleware don't validate security
   - Middleware sets up authentication context
   - Handlers depend on middleware-provided data

3. **Multi-schema architecture requires careful testing**
   - Create test data in all relevant schemas
   - Clean up both global and territory schemas
   - Validate foreign key relationships

4. **Test failures reveal configuration issues**
   - 401 errors = missing middleware
   - 400 errors = validation working correctly
   - 404 errors = wrong endpoint path/method
   - 500 errors = database constraint violations

5. **Real JWTs in tests validate the full flow**
   - Login to get real tokens
   - Middleware validates real tokens
   - Catches expiration, signature, and claim issues

---

## Testing Checklist

- [ ] Integration tests use `.wrap(middleware::JwtAuth)` for protected routes
- [ ] Test app configuration matches `main.rs` route structure
- [ ] Helper functions create data in both global and territory schemas
- [ ] Cleanup removes data from both schemas
- [ ] Tests use real JWTs from login endpoint
- [ ] Public endpoints don't require middleware
- [ ] Protected endpoints require valid JWT bearer tokens
- [ ] Test data respects foreign key constraints
- [ ] Tests run independently (no shared state)
- [ ] Error messages are logged for debugging

---

## 🚨 CRITICAL: Test Data Tracking and Cleanup

**Updated:** 2025-11-08 - **MANDATORY for all new tests**

### The ONE RULE: Never Use Wildcard Cleanup

❌ **FORBIDDEN - Causes Race Conditions:**

```rust
// DELETE ALL test data (affects other parallel tests!)
sqlx::query("DELETE FROM users WHERE email LIKE '%@test.dk'")
    .execute(pool)
    .await;
```

✅ **MANDATORY - Track and Delete Exact Data:**

```rust
// DELETE ONLY data this specific test created
sqlx::query("DELETE FROM users WHERE id = $1")
    .bind(user_id)  // Exact ID we tracked
    .execute(pool)
    .await;
```

### Why This Matters

**Problem**: Wildcard patterns in cleanup delete data from ALL running tests:

```
Timeline (Parallel Execution):
──────────────────────────────────────────────────────────
0.0s   Test A: Creates user_a (email: uuid-123@test.dk)
0.1s   Test B: Creates user_b (email: uuid-456@test.dk)
0.2s   Test A: Finishes, runs cleanup
       → DELETE FROM users WHERE email LIKE '%@test.dk'
       → Deletes BOTH user_a AND user_b! ❌
0.3s   Test B: Tries to use user_b → FAILS (deleted!)
```

**Solution**: Track exact IDs and delete only your test's data:

```
Timeline (With Tracking):
──────────────────────────────────────────────────────────
0.0s   Test A: Creates user_a, tracks uuid-123
0.1s   Test B: Creates user_b, tracks uuid-456
0.2s   Test A: Finishes, cleanup
       → DELETE FROM users WHERE id = 'uuid-123'
       → Deletes ONLY user_a ✅
0.3s   Test B: Uses user_b → WORKS! ✅
```

### Mandatory Pattern: TestContext

**ALL tests MUST use TestContext:**

```rust
#[actix_web::test]
async fn test_login() {
    // ✅ CORRECT: Use TestContext
    let mut ctx = TestContext::new().await;
    
    // Create data - automatically tracked
    let (_user_id, username, password, _) = ctx.create_user().await;
    
    // ... test logic ...
    
    // Cleanup ONLY this test's data
    ctx.cleanup().await;
}
```

### TestContext API

| Method | Purpose | Returns |
|--------|---------|---------|
| `TestContext::new()` | Initialize with pool and token service | `TestContext` |
| `ctx.create_user()` | Create and track user | `(user_id, username, password, email)` |
| `ctx.create_invitation()` | Create invitation (any email) | `token` |
| `ctx.create_invitation_for_email(email)` | For specific email | `token` |
| `ctx.create_invitation_with_user(user_id)` | Owned by user | `(inv_id, token)` |
| `ctx.create_expired_invitation()` | Expired invitation | `token` |
| `ctx.create_maxed_invitation()` | Maxed-out invitation | `token` |
| `ctx.create_revoked_invitation()` | Revoked invitation | `token` |
| `ctx.cleanup()` | Delete tracked data only | `()` |

### Performance Impact

| Mode | Time | Pass Rate | Usage |
|------|------|-----------|-------|
| **Sequential** (`--test-threads=1`) | 13s | 100% | Old approach with wildcards |
| **Parallel** (default) | 4s ⚡ | 100% ✅ | With TestContext |

**Result: 3x faster with TestContext + parallel execution!**

### Common Mistakes

❌ **Mistake 1**: Bypassing TestContext

```rust
// ❌ WRONG: Not tracked!
let pool = ctx.pool.clone();
let user = create_test_user(&pool, "territory_dk").await;
```

```rust
// ✅ CORRECT: Tracked!
let user = ctx.create_user().await;
```

❌ **Mistake 2**: Not mutable

```rust
// ❌ WRONG: Compile error
let ctx = TestContext::new().await;
ctx.create_user().await;
```

```rust
// ✅ CORRECT
let mut ctx = TestContext::new().await;
ctx.create_user().await;
```

❌ **Mistake 3**: No cleanup

```rust
// ❌ WRONG: Orphaned data!
let mut ctx = TestContext::new().await;
// ... test ...
// No ctx.cleanup()!
```

```rust
// ✅ CORRECT
let mut ctx = TestContext::new().await;
// ... test ...
ctx.cleanup().await;
```

### Code Review Checklist

Before merging tests:

- [ ] Uses `TestContext::new()` (not `get_test_pool()`)
- [ ] All data created via `ctx.create_*()` methods
- [ ] Calls `ctx.cleanup().await` at end
- [ ] NO direct calls to `create_test_user(&pool, ...)`
- [ ] NO wildcard patterns (`LIKE '%@test.dk'`)
- [ ] Passes `cargo test --test lib` (parallel)
- [ ] Passes 5 consecutive runs (no flakiness)

---

## References

- **Production Config:** `services/auth-service/src/main.rs`
- **Middleware Implementation:** `services/auth-service/src/middleware/auth.rs`
- **Test Examples:** `services/auth-service/tests/integration/`
- **TestContext Implementation:** `services/auth-service/tests/common/mod.rs`
- **Database Schema:** `services/shared-lib/migrations/`
