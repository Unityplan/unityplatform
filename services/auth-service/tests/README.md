# Auth Service Integration Tests

This directory contains the integration test suite for the auth-service, following Rust and actix-web best practices for scalable test organization.

## Test Structure

```
tests/
├── lib.rs                    # Main test entry point with health check
├── common/
│   └── mod.rs               # Shared test utilities and helpers
└── integration/
    ├── mod.rs               # Module declarations
    ├── auth.rs              # Authentication flow tests
    └── invitation.rs        # Invitation system tests
```

## Design Principles

This structure follows Rust best practices for integration tests:

1. **Single Test Binary**: All tests are compiled into one binary (`lib.rs`), providing faster compilation and easier test management as the project grows.

2. **Module Organization**: Tests are organized into logical modules under `integration/`:
   - `auth.rs` - User authentication (register, login, logout, tokens)
   - `invitation.rs` - Invitation management (create, validate, revoke)

3. **Shared Utilities**: Common test helpers are in `common/` (not compiled as tests):
   - Database pool setup
   - Test data creation and cleanup
   - Token service initialization
   - Helper functions for creating test users and invitations

4. **Actix-Web Pattern**: Each test creates its service inline using `test::init_service()`:

   ```rust
   let app = test::init_service(
       App::new()
           .app_data(web::Data::new(pool.clone()))
           .route("/api/auth/login", web::post().to(handler))
   ).await;
   ```

## Running Tests

### Run all tests

```bash
cargo test
```

### Run specific test module

```bash
cargo test --test lib auth          # Run all auth tests
cargo test --test lib invitation    # Run all invitation tests
```

### Run specific test

```bash
cargo test --test lib test_login_success
cargo test --test lib test_register_invalid_invitation
```

### Run with output

```bash
cargo test -- --nocapture           # Show println! output
cargo test -- --test-threads=1      # Run tests sequentially
```

## Test Coverage

### Authentication Tests (`integration/auth.rs`)

- ✅ `test_register_success` - Successful user registration
- ✅ `test_register_invalid_invitation` - Registration with invalid token
- ✅ `test_register_expired_invitation` - Registration with expired token
- ✅ `test_login_success` - Successful login with correct credentials
- ✅ `test_login_wrong_password` - Login failure with wrong password
- ✅ `test_login_nonexistent_user` - Login failure for non-existent user
- ✅ `test_refresh_token_success` - Token refresh with valid refresh token
- ✅ `test_refresh_token_invalid` - Token refresh with invalid token
- ✅ `test_logout` - User logout

### Invitation Tests (`integration/invitation.rs`)

- ✅ `test_create_invitation_authenticated` - Create invitation when authenticated
- ✅ `test_create_invitation_unauthenticated` - Reject invitation creation without auth
- ✅ `test_list_user_invitations` - List all user's invitations
- ✅ `test_revoke_invitation` - Revoke an invitation
- ✅ `test_validate_invitation_maxed_out` - Reject invitations at max uses
- ✅ `test_validate_invitation_revoked` - Reject revoked invitations
- ✅ `test_invitation_email_mismatch` - Reject when email doesn't match invitation

## Environment Setup

Tests require a PostgreSQL database. Set the connection string:

```bash
export DATABASE_URL="postgres://unityplan:unityplan_dev_password@localhost:5432/unityplan_dk"
```

Or create a `.env` file in the auth-service directory:

```
DATABASE_URL=postgres://unityplan:unityplan_dev_password@localhost:5432/unityplan_dk
```

## Test Database

Tests use the Denmark pod database (`unityplan_dk`) and:

- Create test data with `@test.dk` email addresses
- Clean up test data after each test
- Use the `territory_dk` schema for user and invitation tables

## Adding New Tests

### 1. Create a new test in existing module

Add to `integration/auth.rs` or `integration/invitation.rs`:

```rust
#[actix_web::test]
async fn test_new_feature() {
    let pool = get_test_pool().await;
    setup_test_data(&pool).await;

    let token_service = create_token_service();
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(token_service.clone()))
            .route("/api/path", web::post().to(handler))
    ).await;

    // Test logic here

    cleanup_test_data(&pool).await;
}
```

### 2. Create a new test module

1. Create `integration/new_module.rs`
2. Add `pub mod new_module;` to `integration/mod.rs`
3. Write tests following the pattern above

### 3. Add test utilities

Add helper functions to `common/mod.rs` for:

- Creating test data
- Setting up specific scenarios
- Validation helpers

## Best Practices

1. **Cleanup**: Always call `cleanup_test_data()` at the end of each test
2. **Isolation**: Each test should be independent and idempotent
3. **Descriptive Names**: Use clear test names that describe what's being tested
4. **Assertions**: Include helpful assertion messages
5. **Inline Services**: Create `App` inline in each test (don't abstract into helpers)
6. **Use JSON**: Use `json!()` macro for request bodies instead of struct instances

## Future Enhancements

As the service grows, consider adding:

- [ ] Middleware tests (authentication, rate limiting)
- [ ] Permission and role tests
- [ ] Session management tests
- [ ] Concurrent access tests
- [ ] Performance benchmarks
- [ ] End-to-end API workflow tests

## References

- [Actix-Web Testing Documentation](https://actix.rs/docs/testing)
- [Rust Integration Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests)
- [SQLx Testing](https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-do-a-select-exists-query)
