use auth_service::services::TokenService;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

pub fn create_token_service() -> Arc<TokenService> {
    Arc::new(TokenService::new(
        "test_secret_key_for_jwt_tokens_12345",
        900,    // 15 minutes access token
        604800, // 7 days refresh token
    ))
}

pub async fn setup_test_data(pool: &PgPool) {
    // Ensure Denmark territory exists
    sqlx::query(
        r#"
        INSERT INTO global.territories (code, name, type, is_active)
        VALUES ('dk', 'Denmark', 'country', true)
        ON CONFLICT (code) DO NOTHING
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to insert test territory");
}

pub async fn cleanup_test_data(pool: &PgPool) {
    // Clean up test users (this will cascade to user_identities via FK)
    sqlx::query("DELETE FROM territory_dk.users WHERE email LIKE '%@test.dk'")
        .execute(pool)
        .await
        .ok();

    // Clean up orphaned global identities (case-insensitive territory match)
    sqlx::query("DELETE FROM global.user_identities WHERE LOWER(territory_code) = 'dk' AND territory_user_id NOT IN (SELECT id FROM territory_dk.users)")
        .execute(pool)
        .await
        .ok();

    // Clean up test invitations
    sqlx::query("DELETE FROM territory_dk.invitation_tokens WHERE invited_email LIKE '%@test.dk' OR token LIKE 'test_%'")
        .execute(pool)
        .await
        .ok();

    // Clean up test invitation uses
    sqlx::query("DELETE FROM territory_dk.invitation_uses WHERE used_by_user_id IN (SELECT id FROM territory_dk.users WHERE email LIKE '%@test.dk')")
        .execute(pool)
        .await
        .ok();
}

/// Create a test invitation token
pub async fn create_test_invitation(pool: &PgPool, schema: &str) -> String {
    let token = format!(
        "test_invite_{}",
        Uuid::new_v4().to_string().replace("-", "")
    );
    let expires_at = Utc::now() + Duration::days(7);

    sqlx::query(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active)
        VALUES ($1, 'single_use', 'newuser@test.dk', 1, 0, $2, true)
        "#,
        schema
    ))
    .bind(&token)
    .bind(expires_at)
    .execute(pool)
    .await
    .expect("Failed to create test invitation");

    token
}

/// Create an expired invitation token
pub async fn create_expired_invitation(pool: &PgPool, schema: &str) -> String {
    let token = format!(
        "test_expired_{}",
        Uuid::new_v4().to_string().replace("-", "")
    );
    let expires_at = Utc::now() - Duration::days(1); // Already expired

    sqlx::query(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active)
        VALUES ($1, 'single_use', 'newuser@test.dk', 1, 0, $2, true)
        "#,
        schema
    ))
    .bind(&token)
    .bind(expires_at)
    .execute(pool)
    .await
    .expect("Failed to create expired invitation");

    token
}

/// Create a test user and return (email, password)
pub async fn create_test_user(pool: &PgPool, schema: &str) -> (String, String) {
    let email = format!(
        "testuser_{}@test.dk",
        Uuid::new_v4().to_string()[..8].to_string()
    );
    let username = format!("testuser_{}", Uuid::new_v4().to_string()[..8].to_string());
    let password = "TestPassword123!";

    // Hash password using argon2
    let password_hash = auth_service::services::PasswordService::hash_password(password)
        .expect("Failed to hash password");

    // Generate public key hash
    let public_key_hash = auth_service::utils::crypto::generate_public_key_hash(&email, &username);

    // Create user in territory schema first
    let territory_user_id: Uuid = sqlx::query_scalar(&format!(
        r#"
        INSERT INTO {}.users 
            (username, email, password_hash, full_name, is_verified, is_active)
        VALUES ($1, $2, $3, 'Test User', true, true)
        RETURNING id
        "#,
        schema
    ))
    .bind(&username)
    .bind(&email)
    .bind(&password_hash)
    .fetch_one(pool)
    .await
    .expect("Failed to create territory user");

    // Create global user identity
    sqlx::query(
        r#"
        INSERT INTO global.user_identities (public_key_hash, territory_code, territory_user_id)
        VALUES ($1, 'dk', $2)
        "#,
    )
    .bind(&public_key_hash)
    .bind(territory_user_id)
    .execute(pool)
    .await
    .expect("Failed to create user identity");

    (email, password.to_string())
}

/// Create a maxed-out invitation (all uses consumed)
pub async fn create_maxed_invitation(pool: &PgPool, schema: &str) -> String {
    let token = format!("test_maxed_{}", Uuid::new_v4().to_string().replace("-", ""));
    let expires_at = Utc::now() + Duration::days(7);

    sqlx::query(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active)
        VALUES ($1, 'group', NULL, 5, 5, $2, true)
        "#,
        schema
    ))
    .bind(&token)
    .bind(expires_at)
    .execute(pool)
    .await
    .expect("Failed to create maxed invitation");

    token
}

/// Create a revoked invitation token
pub async fn create_revoked_invitation(pool: &PgPool, schema: &str) -> String {
    let token = format!(
        "test_revoked_{}",
        Uuid::new_v4().to_string().replace("-", "")
    );
    let expires_at = Utc::now() + Duration::days(7);

    sqlx::query(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active)
        VALUES ($1, 'single_use', 'newuser@test.dk', 1, 0, $2, false)
        "#,
        schema
    ))
    .bind(&token)
    .bind(expires_at)
    .execute(pool)
    .await
    .expect("Failed to create revoked invitation");

    token
}
