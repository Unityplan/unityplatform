use auth_service::services::TokenService;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

/// TestContext tracks all data created during a test and ensures precise cleanup.
///
/// CRITICAL TESTING RULE:
/// - NEVER use wildcard patterns for cleanup (LIKE '%@test.dk')
/// - ALWAYS track exact IDs of created data
/// - ALWAYS delete only data this specific test created
///
/// This enables safe parallel test execution without race conditions.
pub struct TestContext {
    pub pool: PgPool,
    pub token_service: Arc<TokenService>,
    created_users: Vec<Uuid>,
    created_invitations: Vec<Uuid>,
}

impl TestContext {
    /// Create new test context and set up test data
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for tests");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database");
        
        setup_test_data(&pool).await;

        Self {
            pool,
            token_service: create_token_service(),
            created_users: Vec::new(),
            created_invitations: Vec::new(),
        }
    }

    /// Create a test user and track its ID for cleanup
    pub async fn create_user(&mut self) -> (Uuid, String, String, Option<String>) {
        let (user_id, username, password, email) =
            create_test_user_with_id(&self.pool, "territory_dk").await;

        // Track this user for precise cleanup
        self.created_users.push(user_id);

        (user_id, username, password, email)
    }

    /// Create a test invitation and track its ID for cleanup (NULL email - allows any email)
    pub async fn create_invitation(&mut self) -> String {
        let (inv_id, token) = create_test_invitation_with_id_for_email(
            &self.pool,
            "territory_dk",
            None, // NULL email - allows any email to use this invitation
        )
        .await;

        // Track this invitation for precise cleanup
        self.created_invitations.push(inv_id);

        token
    }

    /// Create a test invitation with specific email and track its ID
    pub async fn create_invitation_for_email(&mut self, email: Option<String>) -> String {
        let (inv_id, token) =
            create_test_invitation_with_id_for_email(&self.pool, "territory_dk", email).await;

        // Track this invitation for precise cleanup
        self.created_invitations.push(inv_id);

        token
    }

    /// Create a test invitation with user ownership and track its ID
    pub async fn create_invitation_with_user(&mut self, user_id: Uuid) -> (Uuid, String) {
        let (inv_id, token) =
            create_test_invitation_with_id_and_user(&self.pool, "territory_dk", Some(user_id))
                .await;

        // Track this invitation for precise cleanup
        self.created_invitations.push(inv_id);

        (inv_id, token)
    }

    /// Create an expired invitation and track its ID
    pub async fn create_expired_invitation(&mut self) -> String {
        let (inv_id, token) = create_expired_invitation_with_id(&self.pool, "territory_dk").await;

        // Track this invitation for precise cleanup
        self.created_invitations.push(inv_id);

        token
    }

    /// Create a maxed-out invitation and track its ID
    pub async fn create_maxed_invitation(&mut self) -> String {
        let (inv_id, token) = create_maxed_invitation_with_id(&self.pool, "territory_dk").await;

        // Track this invitation for precise cleanup
        self.created_invitations.push(inv_id);

        token
    }

    /// Create a revoked invitation and track its ID
    pub async fn create_revoked_invitation(&mut self) -> String {
        let (inv_id, token) = create_revoked_invitation_with_id(&self.pool, "territory_dk").await;

        // Track this invitation for precise cleanup
        self.created_invitations.push(inv_id);

        token
    }

    /// Cleanup ONLY the data this test created (precise deletion by ID)
    pub async fn cleanup(self) {
        // 1. Delete invitation uses for tracked users
        for user_id in &self.created_users {
            sqlx::query("DELETE FROM territory_dk.invitation_uses WHERE used_by_user_id = $1")
                .bind(user_id)
                .execute(&self.pool)
                .await
                .ok();
        }

        // 2. Delete tracked invitations by exact ID
        for inv_id in &self.created_invitations {
            sqlx::query("DELETE FROM territory_dk.invitation_tokens WHERE id = $1")
                .bind(inv_id)
                .execute(&self.pool)
                .await
                .ok();
        }

        // 3. Delete tracked users by exact ID (cascades to user_identities)
        for user_id in &self.created_users {
            sqlx::query("DELETE FROM territory_dk.users WHERE id = $1")
                .bind(user_id)
                .execute(&self.pool)
                .await
                .ok();
        }

        // 4. Clean up any orphaned global identities
        sqlx::query("DELETE FROM global.user_identities WHERE LOWER(territory_code) = 'dk' AND territory_user_id NOT IN (SELECT id FROM territory_dk.users)")
            .execute(&self.pool)
            .await
            .ok();
    }
}

// ============================================================================
// Internal helper functions used by TestContext
// ============================================================================

fn create_token_service() -> Arc<TokenService> {
    Arc::new(TokenService::new(
        "test_secret_key_for_jwt_tokens_12345",
        900,    // 15 minutes access token
        604800, // 7 days refresh token
    ))
}

async fn setup_test_data(pool: &PgPool) {
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

// ============================================================================
// Helper functions for creating test data (used by TestContext methods)
// ============================================================================

/// Create a test invitation token for specific email and return its UUID
async fn create_test_invitation_with_id_for_email(
    pool: &PgPool,
    schema: &str,
    email: Option<String>,
) -> (Uuid, String) {
    let unique_id = Uuid::new_v4().to_string().replace("-", "");
    let token = format!("test_invite_{}", unique_id);
    let expires_at = Utc::now() + Duration::days(7);

    // Use 'group' token type with NULL email to allow any email to register
    let (token_type, max_uses) = if email.is_none() {
        ("group", Some(10)) // Group invitation allows multiple uses
    } else {
        ("single_use", Some(1))
    };

    let id: Uuid = sqlx::query_scalar(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active)
        VALUES ($1, $2, $3, $4, 0, $5, true)
        RETURNING id
        "#,
        schema
    ))
    .bind(&token)
    .bind(token_type)
    .bind(&email)
    .bind(max_uses)
    .bind(expires_at)
    .fetch_one(pool)
    .await
    .expect("Failed to create test invitation");

    (id, token)
}

/// Create a test invitation token with optional user_id and return its UUID
async fn create_test_invitation_with_id_and_user(
    pool: &PgPool,
    schema: &str,
    created_by_user_id: Option<Uuid>,
) -> (Uuid, String) {
    let unique_id = Uuid::new_v4().to_string().replace("-", "");
    let token = format!("test_invite_{}", unique_id);
    let invited_email = format!("invited_{}@test.dk", &unique_id[..8]);
    let expires_at = Utc::now() + Duration::days(7);

    let id: Uuid = sqlx::query_scalar(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active, created_by_user_id)
        VALUES ($1, 'single_use', $2, 1, 0, $3, true, $4)
        RETURNING id
        "#,
        schema
    ))
    .bind(&token)
    .bind(&invited_email)
    .bind(expires_at)
    .bind(created_by_user_id)
    .fetch_one(pool)
    .await
    .expect("Failed to create test invitation");

    (id, token)
}

/// Create an expired invitation token and return its UUID
async fn create_expired_invitation_with_id(pool: &PgPool, schema: &str) -> (Uuid, String) {
    let unique_id = Uuid::new_v4().to_string().replace("-", "");
    let token = format!("test_expired_{}", unique_id);
    let expires_at = Utc::now() - Duration::days(1); // Already expired

    // Use group type with NULL email to allow any email
    let id: Uuid = sqlx::query_scalar(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active)
        VALUES ($1, 'group', NULL, 10, 0, $2, true)
        RETURNING id
        "#,
        schema
    ))
    .bind(&token)
    .bind(expires_at)
    .fetch_one(pool)
    .await
    .expect("Failed to create expired invitation");

    (id, token)
}

/// Create a test user and return (user_id, username, password, optional email)
async fn create_test_user_with_id(
    pool: &PgPool,
    schema: &str,
) -> (Uuid, String, String, Option<String>) {
    // Generate unique username (must be globally unique)
    let username = format!("testuser_{}", Uuid::new_v4().to_string()[..8].to_string());

    // Email is optional (50% chance for testing both scenarios)
    let email = if rand::random::<bool>() {
        Some(format!(
            "testuser_{}@test.dk",
            Uuid::new_v4().to_string()[..8].to_string()
        ))
    } else {
        None
    };

    let password = "TestPassword123!";

    // Hash password using argon2
    let password_hash = auth_service::services::PasswordService::hash_password(password)
        .expect("Failed to hash password");

    // Create user in territory schema
    // Note: Database trigger will automatically create global.user_identities entry
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

    // Global identity is created automatically by trigger
    // No manual insertion needed

    (territory_user_id, username, password.to_string(), email)
}

/// Create a maxed-out invitation and return its UUID
async fn create_maxed_invitation_with_id(pool: &PgPool, schema: &str) -> (Uuid, String) {
    let token = format!("test_maxed_{}", Uuid::new_v4().to_string().replace("-", ""));
    let expires_at = Utc::now() + Duration::days(7);

    let id: Uuid = sqlx::query_scalar(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active)
        VALUES ($1, 'group', NULL, 5, 5, $2, true)
        RETURNING id
        "#,
        schema
    ))
    .bind(&token)
    .bind(expires_at)
    .fetch_one(pool)
    .await
    .expect("Failed to create maxed invitation");

    (id, token)
}

/// Create a revoked invitation token and return its UUID
async fn create_revoked_invitation_with_id(pool: &PgPool, schema: &str) -> (Uuid, String) {
    let unique_id = Uuid::new_v4().to_string().replace("-", "");
    let token = format!("test_revoked_{}", unique_id);
    let invited_email = format!("revoked_{}@test.dk", &unique_id[..8]);
    let expires_at = Utc::now() + Duration::days(7);

    let id: Uuid = sqlx::query_scalar(&format!(
        r#"
        INSERT INTO {}.invitation_tokens 
        (token, token_type, invited_email, max_uses, current_uses, expires_at, is_active)
        VALUES ($1, 'single_use', $2, 1, 0, $3, false)
        RETURNING id
        "#,
        schema
    ))
    .bind(&token)
    .bind(&invited_email)
    .bind(expires_at)
    .fetch_one(pool)
    .await
    .expect("Failed to create revoked invitation");

    (id, token)
}
