use actix_web::{test, App};
use shared_lib::{AppConfig, Database, UserDeletedEvent};
use sqlx::PgPool;
use uuid::Uuid;

/// Test helper: Create a soft-deleted test user in the database
async fn create_soft_deleted_user(
    pool: &PgPool,
    territory: &str,
    days_ago: i64,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    // Username must be lowercase alphanumeric with underscores only (no hyphens)
    let username = format!("test_user_{}", user_id.to_string().replace("-", "_"));
    let email = format!("test_{}@example.com", user_id.to_string().replace("-", ""));
    let deleted_at = chrono::Utc::now() - chrono::Duration::days(days_ago);
    
    let schema = format!("territory_{}", territory);
    
    // Create user in auth_users_core
    sqlx::query(&format!(
        "INSERT INTO {}.auth_users_core (id, username, email, password_hash, active, deleted_at) 
         VALUES ($1, $2, $3, $4, $5, $6)",
        schema
    ))
    .bind(user_id)
    .bind(&username)
    .bind(&email)
    .bind("test_hash")
    .bind(false)
    .bind(deleted_at)
    .execute(pool)
    .await?;
    
    // Add to global registries
    sqlx::query(
        "INSERT INTO global.registry_username (user_id, username, territory_code, deleted_at) 
         VALUES ($1, $2, $3, $4)"
    )
    .bind(user_id)
    .bind(&username)
    .bind(territory)
    .bind(deleted_at)
    .execute(pool)
    .await?;
    
    sqlx::query(
        "INSERT INTO global.registry_email (user_id, email, territory_code, deleted_at) 
         VALUES ($1, $2, $3, $4)"
    )
    .bind(user_id)
    .bind(&email)
    .bind(territory)
    .bind(deleted_at)
    .execute(pool)
    .await?;
    
    // Create some user data in various services
    sqlx::query(&format!(
        "INSERT INTO {}.user_users_profiles (user_id, display_name) 
         VALUES ($1, $2)",
        schema
    ))
    .bind(user_id)
    .bind("Test User")
    .execute(pool)
    .await?;
    
    Ok(user_id)
}

/// Test helper: Verify user is completely deleted
async fn verify_user_deleted(
    pool: &PgPool,
    user_id: Uuid,
    territory: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let schema = format!("territory_{}", territory);
    
    // Check auth_users_core
    let auth_exists: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {}.auth_users_core WHERE id = $1)",
        schema
    ))
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    
    if auth_exists {
        return Ok(false);
    }
    
    // Check global registries
    let registry_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM global.registry_username WHERE user_id = $1)"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    
    if registry_exists {
        return Ok(false);
    }
    
    // Check user_users_profiles
    let profile_exists: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {}.user_users_profiles WHERE user_id = $1)",
        schema
    ))
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    
    Ok(!profile_exists)
}

#[actix_web::test]
async fn test_cleanup_service_dry_run() {
    // Load config and connect to database
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    let database = Database::new(
        config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    let pool = database.pool();
    
    // Create a soft-deleted user (35 days ago - eligible for deletion)
    let user_id = create_soft_deleted_user(pool, "dk", 35)
        .await
        .expect("Failed to create test user");
    
    // Create cleanup service
    let cleanup_service = task_scheduler_service_service::services::CleanupService::new(pool.clone());
    
    // Run dry run
    let stats = cleanup_service
        .cleanup_deleted_users(true)
        .await
        .expect("Dry run failed");
    
    // Verify statistics
    assert!(stats.eligible_for_deletion > 0, "Should find eligible users");
    assert_eq!(stats.deleted, 0, "Dry run should not delete anything");
    assert!(stats.territories_processed.contains(&"dk".to_string()));
    
    // Verify user still exists (not deleted in dry run)
    let user_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM territory_dk.auth_users_core WHERE id = $1)"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .expect("Failed to check user existence");
    
    assert!(user_exists, "User should still exist after dry run");
    
    // Cleanup test data
    sqlx::query("DELETE FROM global.registry_username WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM global.registry_email WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM territory_dk.user_users_profiles WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM territory_dk.auth_users_core WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .ok();
}

#[actix_web::test]
async fn test_cleanup_service_actual_deletion() {
    // Initialize logging for debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .try_init();
    
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    let database = Database::new(
        config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    let pool = database.pool();
    
    // Create a soft-deleted user (35 days ago - eligible for deletion)
    let user_id = create_soft_deleted_user(pool, "dk", 35)
        .await
        .expect("Failed to create test user");
    
    println!("Created test user: {}", user_id);
    
    // Verify user exists before cleanup
    let exists_before: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM global.registry_username WHERE user_id = $1 AND deleted_at IS NOT NULL)"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .expect("Failed to check user");
    
    println!("User exists in registry with deleted_at: {}", exists_before);
    assert!(exists_before, "Test user should exist in registry with deleted_at set");
    
    // Create cleanup service
    let cleanup_service = task_scheduler_service_service::services::CleanupService::new(pool.clone());
    
    // Run actual cleanup (not dry run)
    let stats = cleanup_service
        .cleanup_deleted_users(false)
        .await
        .expect("Cleanup failed");
    
    println!("Cleanup stats: eligible={}, deleted={}, territories={:?}", 
        stats.eligible_for_deletion, stats.deleted, stats.territories_processed);
    
    // Verify statistics (at least our user should be deleted)
    assert!(stats.eligible_for_deletion >= 1, "Should have at least 1 eligible user");
    assert!(stats.deleted >= 1, "Should have deleted at least 1 user");
    assert!(stats.territories_processed.contains(&"dk".to_string()));
    
    // Verify our specific user is completely deleted
    let deleted = verify_user_deleted(pool, user_id, "dk")
        .await
        .expect("Failed to verify deletion");
    
    assert!(deleted, "Test user should be completely deleted");
}

#[actix_web::test]
async fn test_cleanup_respects_30_day_window() {
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    let database = Database::new(
        config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    let pool = database.pool();
    
    // Create a user soft-deleted only 20 days ago (NOT eligible)
    let recent_user_id = create_soft_deleted_user(pool, "dk", 20)
        .await
        .expect("Failed to create test user");
    
    // Create cleanup service
    let cleanup_service = task_scheduler_service_service::services::CleanupService::new(pool.clone());
    
    // Run cleanup
    let stats = cleanup_service
        .cleanup_deleted_users(false)
        .await
        .expect("Cleanup failed");
    
    // Recent user should NOT be in the eligible count (or very small number from other tests)
    // We can't assert exact count, but we can verify the recent user still exists
    
    // Verify recent user still exists
    let user_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM territory_dk.auth_users_core WHERE id = $1)"
    )
    .bind(recent_user_id)
    .fetch_one(pool)
    .await
    .expect("Failed to check user existence");
    
    assert!(user_exists, "User deleted <30 days ago should NOT be deleted");
    
    // Cleanup test data
    sqlx::query("DELETE FROM global.registry_username WHERE user_id = $1")
        .bind(recent_user_id)
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM global.registry_email WHERE user_id = $1")
        .bind(recent_user_id)
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM territory_dk.user_users_profiles WHERE user_id = $1")
        .bind(recent_user_id)
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM territory_dk.auth_users_core WHERE id = $1")
        .bind(recent_user_id)
        .execute(pool)
        .await
        .ok();
}

#[actix_web::test]
async fn test_user_deleted_event_deserialization() {
    // Test that UserDeletedEvent can be serialized and deserialized correctly
    let user_id = Uuid::new_v4();
    let event = UserDeletedEvent::new(user_id, "dk".to_string());
    
    // Serialize
    let json = serde_json::to_vec(&event).expect("Failed to serialize");
    
    // Deserialize
    let deserialized: UserDeletedEvent = 
        serde_json::from_slice(&json).expect("Failed to deserialize");
    
    assert_eq!(deserialized.user_id, user_id);
    assert_eq!(deserialized.territory, "dk");
    assert!(deserialized.deleted_at <= chrono::Utc::now());
}
