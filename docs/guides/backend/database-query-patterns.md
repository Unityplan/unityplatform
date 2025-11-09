# Database Query Patterns for Multi-Pod Architecture

**Version:** 1.0  
**Last Updated:** November 8, 2025  
**Applies To:** All Rust microservices in UnityPlan platform

---

## Critical Rule: Runtime Queries Only

### ⚠️ ALWAYS Use Runtime Queries

**DO NOT use compile-time macro queries (`query!()`, `query_as!()`) in any service.**

```rust
// ✅ CORRECT - Runtime verification
let user = sqlx::query_as::<_, User>(
    "SELECT id, username, email FROM territory.users WHERE id = $1"
)
.bind(user_id)
.fetch_one(&pool)
.await?;

// ❌ WRONG - Compile-time verification
let user = sqlx::query_as!(
    User,
    "SELECT id, username, email FROM territory.users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;
```

---

## Why This Matters

### Multi-Pod Architecture

UnityPlan uses a **distributed multi-pod architecture** where:

1. **Each territory has its own pod** (Denmark, Norway, Sweden, Europe, etc.)
2. **Each pod has its own database instance**
3. **Same service binary deploys to all pods**

### The Problem with Compile-Time Queries

Compile-time macros (`query!()`, `query_as!()`) work by:

1. Connecting to database **during compilation**
2. Validating SQL against actual schema
3. **Baking schema information into the binary**

This breaks our architecture because:

- Binary is compiled **once** against ONE database (e.g., Denmark pod)
- Schema details are **hardcoded** into the binary
- When deployed to Norway pod, it still expects Denmark's exact schema
- Different seed data, territory codes, or schema variations cause runtime failures

### Runtime Queries Are Pod-Agnostic

Runtime queries (`query()`, `query_as::<_, Type>()`) work by:

1. **NO** database connection during compilation
2. Schema validation happens **at runtime** when service starts
3. Binary adapts to **whichever database** the `DATABASE_URL` points to

**Result:** Same binary works across all pods! 🎉

---

## Implementation Patterns

### 1. Simple Query

```rust
let username: String = sqlx::query_scalar(
    "SELECT username FROM territory.users WHERE id = $1"
)
.bind(user_id)
.fetch_one(&pool)
.await?;
```

### 2. Query with Struct Mapping

```rust
#[derive(sqlx::FromRow)]
struct User {
    id: Uuid,
    username: String,
    email: Option<String>,
}

let user = sqlx::query_as::<_, User>(
    "SELECT id, username, email FROM territory.users WHERE id = $1"
)
.bind(user_id)
.fetch_one(&pool)
.await?;
```

### 3. Insert/Update with RETURNING

```rust
let user = sqlx::query_as::<_, User>(
    r#"
    INSERT INTO territory.users (id, username, email, password_hash)
    VALUES ($1, $2, $3, $4)
    RETURNING id, username, email
    "#
)
.bind(Uuid::new_v4())
.bind(&username)
.bind(&email)
.bind(&password_hash)
.fetch_one(&pool)
.await?;
```

### 4. Dynamic Queries (when needed)

```rust
let query = format!(
    "SELECT * FROM {}.users WHERE username = $1",
    schema_name
);

let user = sqlx::query_as::<_, User>(&query)
    .bind(&username)
    .fetch_one(&pool)
    .await?;
```

**⚠️ Warning:** Only use dynamic queries when necessary. Always validate/sanitize schema names to prevent SQL injection.

---

## Type Safety

Runtime queries are still type-safe! The compiler verifies:

- ✅ Bind parameter types match
- ✅ Return types match struct fields
- ✅ Row mapping is valid

What it **doesn't** verify (until runtime):

- ❌ Column names exist in database
- ❌ Table names are correct
- ❌ SQL syntax is valid

**Mitigation:** Comprehensive integration tests against real database.

---

## Testing Strategy

### Unit Tests

Mock database interactions for business logic tests.

### Integration Tests

**MUST** test against actual PostgreSQL database:

```rust
#[sqlx::test]
async fn test_create_user(pool: PgPool) -> sqlx::Result<()> {
    let user = create_user(&pool, "testuser", "test@example.com").await?;
    assert_eq!(user.username, "testuser");
    Ok(())
}
```

Use `sqlx::test` macro for automatic database setup/teardown.

---

## Migration Strategy

If you have existing services with compile-time queries:

### Step 1: Identify All Queries

```bash
grep -r "query!" services/your-service/src/
grep -r "query_as!" services/your-service/src/
```

### Step 2: Convert One-by-One

**Before:**

```rust
let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    .fetch_one(&pool)
    .await?;
```

**After:**

```rust
let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
    .bind(id)
    .fetch_one(&pool)
    .await?;
```

### Step 3: Test Thoroughly

Run integration tests against multiple pod databases to ensure compatibility.

---

## Checklist for New Services

When creating a new service:

- [ ] Use `sqlx::query()` and `sqlx::query_as::<_, T>()` (runtime)
- [ ] **Never** use `sqlx::query!()` or `sqlx::query_as!()` (compile-time macros)
- [ ] Define structs with `#[derive(sqlx::FromRow)]` for type safety
- [ ] Write integration tests with `#[sqlx::test]`
- [ ] Test deployment to multiple pods
- [ ] Document query patterns in service README

---

## References

- [SQLx Documentation - Runtime Queries](https://docs.rs/sqlx/latest/sqlx/macro.query.html)
- [UnityPlan Multi-Pod Architecture](../../architecture/multi-pod-architecture.md)
- [Territory Management Standard](../../architecture/territory-management-standard.md)

---

## Questions?

If unsure about query patterns, check existing services:

- ✅ **auth-service**: Uses runtime queries (correct pattern)
- ✅ **user-service**: Uses runtime queries (correct pattern)
- See `services/auth-service/src/handlers/auth.rs` for examples
