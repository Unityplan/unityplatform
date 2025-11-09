use sqlx::PgPool;
use uuid::Uuid;

// Territory schema name - matches multi-pod architecture
const TERRITORY_SCHEMA: &str = "territory";

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
    tracked_user_ids: Vec<Uuid>,
}

impl TestContext {
    /// Create new test context and connect to database
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        setup_test_data(&pool).await;

        Self {
            pool,
            tracked_user_ids: Vec::new(),
        }
    }

    /// Create a test user and track it for cleanup
    pub async fn create_user(
        &mut self,
        username: &str,
        email: &str,
    ) -> Uuid {
        let user_id = Uuid::new_v4();
        
        // Make username unique by appending part of UUID to avoid collisions in parallel tests
        let unique_username = format!("{}_{}", username, &user_id.to_string()[..8]);
        let unique_email = if email.contains('@') {
            let parts: Vec<&str> = email.split('@').collect();
            format!("{}+{}@{}", parts[0], &user_id.to_string()[..8], parts[1])
        } else {
            email.to_string()
        };
        
        sqlx::query(&format!(
            r#"
            INSERT INTO {}.users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
            TERRITORY_SCHEMA
        ))
        .bind(user_id)
        .bind(&unique_username)
        .bind(&unique_email)
        .bind("$argon2id$v=19$m=19456,t=2,p=1$test") // dummy hash
        .execute(&self.pool)
        .await
        .expect("Failed to create test user");
        
        // Track for cleanup
        self.tracked_user_ids.push(user_id);
        
        user_id
    }

    /// Clean up ALL tracked data created by this test context
    pub async fn cleanup(self) {
        // Delete in proper order to respect foreign keys:
        // 1. user_blocks (references global.users)
        // 2. user_connections (references global.users)
        // 3. user_profiles (references global.users)
        // 4. users

        for user_id in &self.tracked_user_ids {
            // Delete blocks
            sqlx::query(&format!(
                "DELETE FROM {}.user_blocks WHERE blocker_id = $1 OR blocked_id = $1",
                TERRITORY_SCHEMA
            ))
            .bind(user_id)
            .execute(&self.pool)
            .await
            .ok();

            // Delete connections
            sqlx::query(&format!(
                "DELETE FROM {}.user_connections WHERE follower_id = $1 OR following_id = $1",
                TERRITORY_SCHEMA
            ))
            .bind(user_id)
            .execute(&self.pool)
            .await
            .ok();

            // Delete profiles
            sqlx::query(&format!(
                "DELETE FROM {}.user_profiles WHERE user_id = $1",
                TERRITORY_SCHEMA
            ))
            .bind(user_id)
            .execute(&self.pool)
            .await
            .ok();

            // Delete user
            sqlx::query(&format!(
                "DELETE FROM {}.users WHERE id = $1",
                TERRITORY_SCHEMA
            ))
            .bind(user_id)
            .execute(&self.pool)
            .await
            .ok();
        }
    }
}

// ============================================================================
// Internal helper functions
// ============================================================================

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
