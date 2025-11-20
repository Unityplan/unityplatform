use actix_web::{test, web, App};
use invitation_service_service::handlers;
use invitation_service_service::models::{
    UseInvitationRequest, UseInvitationResponse, ValidateInvitationRequest,
    ValidateInvitationResponse,
};
use invitation_service_service::services::invitation::generate_token; // Use real token generator!
use shared_lib::{AppConfig, Database, NatsClient};
use sqlx::PgPool;
use uuid::Uuid;

/// Helper function to create test invitation in database with custom parameters
/// Uses real token generation to ensure tests match production behavior
async fn create_test_invitation_custom(
    pool: &PgPool,
    max_uses: i32,
    uses_count: i32,
    is_active: bool,
    expires_at_sql: &str,
) -> (Uuid, String, Uuid) {
    let invitation_id = Uuid::new_v4();
    let created_by = Uuid::new_v4();
    let token = generate_token(); // Use REAL token generator!

    let query = format!(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6, {})
        "#,
        expires_at_sql
    );

    sqlx::query(&query)
        .bind(invitation_id)
        .bind(&token)
        .bind(created_by)
        .bind(max_uses)
        .bind(uses_count)
        .bind(is_active)
        .execute(pool)
        .await
        .expect("Failed to create test invitation");

    // Insert into global.registry_invitation
    sqlx::query(
        r#"
        INSERT INTO global.registry_invitation (token, territory_code, territory_token_id)
        VALUES ($1, 'dk', $2)
        "#,
    )
    .bind(&token)
    .bind(invitation_id)
    .execute(pool)
    .await
    .expect("Failed to create global registry entry");

    // Insert into global.registry_username to satisfy "zombie" check
    // We need a unique username, so we'll use the UUID
    let username = format!("user_{}", created_by.simple());
    sqlx::query(
        r#"
        INSERT INTO global.registry_username (username, territory_code, user_id)
        VALUES ($1, 'dk', $2)
        ON CONFLICT (user_id) DO NOTHING
        "#,
    )
    .bind(&username)
    .bind(created_by)
    .execute(pool)
    .await
    .expect("Failed to create global username registry entry");

    // Insert into auth_users_core (required for badge FK)
    let email = format!("{}@example.com", username);
    sqlx::query(
        r#"
        INSERT INTO territory_dk.auth_users_core (id, username, email, password_hash, active)
        VALUES ($1, $2, $3, 'hash', true)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(created_by)
    .bind(&username)
    .bind(email)
    .execute(pool)
    .await
    .expect("Failed to create user in auth_users_core");

    // Get badge ID for 'territory-manager'
    let badge_id: Uuid = sqlx::query_scalar(
        "SELECT id FROM global.registry_badge WHERE slug = 'territory-manager'"
    )
    .fetch_one(pool)
    .await
    .expect("Failed to get badge id");

    // Assign badge
    sqlx::query(
        "INSERT INTO territory_dk.badge_users_badges (user_id, badge_id, awarded_at) VALUES ($1, $2, NOW()) ON CONFLICT (user_id, badge_id) DO NOTHING",
    )
    .bind(created_by)
    .bind(badge_id)
    .execute(pool)
    .await
    .expect("Failed to assign badge");

    // Assign territory manager role
    sqlx::query(
        "INSERT INTO territory_dk.territory_territories_managers (territory_code, user_id, assigned_at) VALUES ('dk', $1, NOW())",
    )
    .bind(created_by)
    .execute(pool)
    .await
    .expect("Failed to assign manager role");

    (invitation_id, token, created_by)
}

/// Helper function to create test invitation in database (default: single-use, active, expires in 7 days)
/// Uses real token generation to ensure tests match production behavior
async fn create_test_invitation(pool: &PgPool) -> (Uuid, String, Uuid) {
    create_test_invitation_custom(
        pool,
        1,                           // max_uses
        0,                           // uses_count
        true,                        // is_active
        "NOW() + INTERVAL '7 days'", // expires_at
    )
    .await
}

/// Cleanup test data - delete only data we created
async fn cleanup_test_invitation(pool: &PgPool, invitation_id: Uuid, token: &str) {
    // Delete from territory table
    sqlx::query("DELETE FROM territory_dk.invitation_invitations_tokens WHERE id = $1")
        .bind(invitation_id)
        .execute(pool)
        .await
        .ok();

    // Delete from global registry
    sqlx::query("DELETE FROM global.registry_invitation WHERE token = $1")
        .bind(token)
        .execute(pool)
        .await
        .ok();
        
    // Note: We don't delete from global.registry_username here because we don't have the user_id easily available
    // and it might be used by other tests if we're not careful. 
    // Ideally we should return created_by from this function too, but that changes the signature.
    // For now, let's rely on the fact that tests use unique UUIDs.
}

#[actix_web::test]
async fn test_validate_invitation_valid_token() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Create test invitation with real token generator
    let (invitation_id, token, created_by) = create_test_invitation(database.pool()).await;

    // Initialize test app (mirrors production configuration)
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .service(
                web::scope("/api/v1").configure(handlers::configure), // Production route config
            ),
    )
    .await;

    // Create request
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/validate")
        .set_json(&ValidateInvitationRequest {
            token: token.clone(),
        })
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;

    // Verify response
    assert_eq!(resp.status(), 200, "Should return 200 OK");

    let body: ValidateInvitationResponse = test::read_body_json(resp).await;
    assert!(body.valid, "Token should be valid");
    assert_eq!(body.invitation_id, Some(invitation_id));
    assert_eq!(body.created_by, Some(created_by));
    assert_eq!(body.uses_remaining, Some(1));
    assert!(body.expires_at.is_some());

    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
}

#[actix_web::test]
async fn test_validate_invitation_invalid_token() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Create request with non-existent token
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/validate")
        .set_json(&ValidateInvitationRequest {
            token: "NONEXISTENT-TOKEN-1234".to_string(),
        })
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;

    // Verify response
    assert_eq!(resp.status(), 200, "Should return 200 OK");

    let body: ValidateInvitationResponse = test::read_body_json(resp).await;
    assert!(!body.valid, "Token should be invalid");
    assert!(body.invitation_id.is_none());
    assert!(body.created_by.is_none());
    assert!(body.uses_remaining.is_none());
}

#[actix_web::test]
async fn test_validate_invitation_expired_token() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    let invitation_id = Uuid::new_v4();
    let token = format!("EXPIRED-{}", Uuid::new_v4());
    let created_by = Uuid::new_v4();

    // Insert expired invitation
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active, expires_at)
        VALUES ($1, $2, $3, 1, 0, true, NOW() - INTERVAL '1 day')
        "#,
    )
    .bind(invitation_id)
    .bind(&token)
    .bind(created_by)
    .execute(database.pool())
    .await
    .expect("Failed to create expired invitation");

    sqlx::query(
        "INSERT INTO global.registry_invitation (token, territory_code, territory_token_id) VALUES ($1, 'dk', $2)",
    )
    .bind(&token)
    .bind(invitation_id)
    .execute(database.pool())
    .await
    .expect("Failed to create global registry entry");

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Create request
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/validate")
        .set_json(&ValidateInvitationRequest {
            token: token.clone(),
        })
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;

    // Verify response
    assert_eq!(resp.status(), 200, "Should return 200 OK");

    let body: ValidateInvitationResponse = test::read_body_json(resp).await;
    assert!(!body.valid, "Expired token should be invalid");

    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
}

#[actix_web::test]
async fn test_validate_invitation_fully_used() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    let invitation_id = Uuid::new_v4();
    let token = format!("USED-{}", Uuid::new_v4());
    let created_by = Uuid::new_v4();

    // Insert fully used invitation
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active)
        VALUES ($1, $2, $3, 1, 1, true)
        "#,
    )
    .bind(invitation_id)
    .bind(&token)
    .bind(created_by)
    .execute(database.pool())
    .await
    .expect("Failed to create fully used invitation");

    sqlx::query(
        "INSERT INTO global.registry_invitation (token, territory_code, territory_token_id) VALUES ($1, 'dk', $2)",
    )
    .bind(&token)
    .bind(invitation_id)
    .execute(database.pool())
    .await
    .expect("Failed to create global registry entry");

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Create request
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/validate")
        .set_json(&ValidateInvitationRequest {
            token: token.clone(),
        })
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;

    // Verify response
    assert_eq!(resp.status(), 200, "Should return 200 OK");

    let body: ValidateInvitationResponse = test::read_body_json(resp).await;
    assert!(!body.valid, "Fully used token should be invalid");

    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
}

// ============================================================================
// Tests for use_invitation endpoint (issue #140)
// ============================================================================

/// Helper to cleanup invitation uses
async fn cleanup_test_uses(pool: &PgPool, invitation_id: Uuid) {
    sqlx::query("DELETE FROM territory_dk.invitation_invitations_uses WHERE invitation_id = $1")
        .bind(invitation_id)
        .execute(pool)
        .await
        .ok();
}

#[actix_web::test]
async fn test_use_invitation_success() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    // Create test invitation (max_uses = 1) using real token generator
    let (invitation_id, token, _created_by) = create_test_invitation(database.pool()).await;
    let used_by = Uuid::new_v4();

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Create request
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/use")
        .set_json(&UseInvitationRequest {
            token: token.clone(),
            used_by,
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Test Agent".to_string()),
        })
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;

    // Debug: Print response status and body if not successful
    if !resp.status().is_success() {
        let status = resp.status();
        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body_bytes);
        panic!(
            "Should successfully use invitation. Status: {}, Body: {}",
            status, body_str
        );
    }

    assert!(
        resp.status().is_success(),
        "Should successfully use invitation"
    );

    // Verify response
    let body: UseInvitationResponse = test::read_body_json(resp).await;
    assert_eq!(body.invitation_id, invitation_id);
    assert_eq!(body.uses_remaining, 0, "Should have 0 uses remaining");
    assert!(body.fully_used, "Should be marked as fully used");

    // Verify database state
    let (uses_count, is_active): (i32, bool) = sqlx::query_as(
        "SELECT uses_count, is_active FROM territory_dk.invitation_invitations_tokens WHERE id = $1",
    )
    .bind(invitation_id)
    .fetch_one(database.pool())
    .await
    .expect("Failed to query invitation");

    assert_eq!(uses_count, 1, "uses_count should be 1");
    assert!(!is_active, "is_active should be false after fully used");

    // Verify usage record exists
    let usage_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM territory_dk.invitation_invitations_uses WHERE invitation_id = $1 AND used_by = $2",
    )
    .bind(invitation_id)
    .bind(used_by)
    .fetch_one(database.pool())
    .await
    .expect("Failed to query usage");

    assert_eq!(usage_count, 1, "Should have 1 usage record");

    // Cleanup
    cleanup_test_uses(database.pool(), invitation_id).await;
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
}

#[actix_web::test]
async fn test_use_invitation_duplicate_usage() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    // Create test invitation (max_uses = 3 for multiple uses) using real token generator
    let (invitation_id, token, _created_by) = create_test_invitation_custom(
        database.pool(),
        3,    // max_uses
        0,    // uses_count
        true, // is_active
        "NOW() + INTERVAL '7 days'",
    )
    .await;
    let used_by = Uuid::new_v4();

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // First use - should succeed
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/use")
        .set_json(&UseInvitationRequest {
            token: token.clone(),
            used_by,
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Test Agent".to_string()),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "First use should succeed");

    // Second use by same user - should fail with 409 Conflict
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/use")
        .set_json(&UseInvitationRequest {
            token: token.clone(),
            used_by,
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Test Agent".to_string()),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status().as_u16(),
        409,
        "Duplicate usage should return 409 Conflict"
    );

    // Cleanup
    cleanup_test_uses(database.pool(), invitation_id).await;
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
}

#[actix_web::test]
async fn test_use_invitation_invalid_token() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Create request with non-existent token (real format, doesn't exist in DB)
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/use")
        .set_json(&UseInvitationRequest {
            token: generate_token(), // Real token that doesn't exist in DB
            used_by: Uuid::new_v4(),
            ip_address: None,
            user_agent: None,
        })
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status().as_u16(),
        404,
        "Invalid token should return 404 Not Found"
    );
}

#[actix_web::test]
async fn test_use_invitation_multi_use() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    // Create test invitation with max_uses = 3 using real token generator
    let (invitation_id, token, _created_by) = create_test_invitation_custom(
        database.pool(),
        3,    // max_uses
        0,    // uses_count
        true, // is_active
        "NOW() + INTERVAL '7 days'",
    )
    .await;

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Use 1: Should succeed, 2 uses remaining
    let user1 = Uuid::new_v4();
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/use")
        .set_json(&UseInvitationRequest {
            token: token.clone(),
            used_by: user1,
            ip_address: None,
            user_agent: None,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: UseInvitationResponse = test::read_body_json(resp).await;
    assert_eq!(body.uses_remaining, 2);
    assert!(!body.fully_used);

    // Use 2: Should succeed, 1 use remaining
    let user2 = Uuid::new_v4();
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/use")
        .set_json(&UseInvitationRequest {
            token: token.clone(),
            used_by: user2,
            ip_address: None,
            user_agent: None,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: UseInvitationResponse = test::read_body_json(resp).await;
    assert_eq!(body.uses_remaining, 1);
    assert!(!body.fully_used);

    // Use 3: Should succeed, 0 uses remaining, fully used
    let user3 = Uuid::new_v4();
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/use")
        .set_json(&UseInvitationRequest {
            token: token.clone(),
            used_by: user3,
            ip_address: None,
            user_agent: None,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: UseInvitationResponse = test::read_body_json(resp).await;
    assert_eq!(body.uses_remaining, 0);
    assert!(body.fully_used);

    // Use 4: Should fail - token is fully used
    let user4 = Uuid::new_v4();
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/use")
        .set_json(&UseInvitationRequest {
            token: token.clone(),
            used_by: user4,
            ip_address: None,
            user_agent: None,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404, "Fully used token should fail");

    // Cleanup
    cleanup_test_uses(database.pool(), invitation_id).await;
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
}

// ============================================================================
// Tests for create_invitation endpoint (issue #148)
// ============================================================================

/// Helper to create a test user and manager setup
async fn setup_manager_user(pool: &PgPool, territory_code: &str) -> (Uuid, Uuid) {
    let user_id = Uuid::new_v4();
    let username = format!(
        "testmanager_{}",
        Uuid::new_v4().to_string()[0..8].to_string()
    );

    // Insert user into auth_users_core
    sqlx::query(
        r#"
        INSERT INTO territory_dk.auth_users_core 
        (id, username, password_hash, territory_code)
        VALUES ($1, $2, 'test_hash', $3)
        "#,
    )
    .bind(user_id)
    .bind(&username)
    .bind(territory_code)
    .execute(pool)
    .await
    .expect("Failed to create test user");

    // Register in global registry
    sqlx::query(
        "INSERT INTO global.registry_username (username, user_id, territory_code) VALUES ($1, $2, $3)",
    )
    .bind(&username)
    .bind(user_id)
    .bind(territory_code)
    .execute(pool)
    .await
    .expect("Failed to register user globally");

    // Get or create territory-manager badge
    let badge_id: Uuid = match sqlx::query_scalar(
        "SELECT id FROM global.registry_badge WHERE slug = 'territory-manager'",
    )
    .fetch_optional(pool)
    .await
    .expect("Failed to query badge")
    {
        Some(id) => id,
        None => {
            // Create badge if it doesn't exist
            let badge_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO global.registry_badge 
                (id, slug, name, description, icon, category, criteria_type, rarity)
                VALUES ($1, 'territory-manager', 'Territory Manager', 'Can manage territory settings', '🔧', 'role', 'manual', 'rare')
                "#,
            )
            .bind(badge_id)
            .execute(pool)
            .await
            .expect("Failed to create badge");
            badge_id
        }
    };

    // Award manager badge to user
    sqlx::query(
        r#"
        INSERT INTO territory_dk.badge_users_badges (badge_id, user_id, awarded_at)
        VALUES ($1, $2, NOW())
        "#,
    )
    .bind(badge_id)
    .bind(user_id)
    .execute(pool)
    .await
    .expect("Failed to award badge");

    // Assign user as manager for the territory
    sqlx::query(
        r#"
        INSERT INTO territory_dk.territory_territories_managers (user_id, territory_code)
        VALUES ($1, $2)
        "#,
    )
    .bind(user_id)
    .bind(territory_code)
    .execute(pool)
    .await
    .expect("Failed to assign manager");

    (user_id, badge_id)
}

/// Helper to cleanup test manager user
async fn cleanup_manager_user(pool: &PgPool, user_id: Uuid) {
    sqlx::query("DELETE FROM territory_dk.territory_territories_managers WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .ok();

    sqlx::query("DELETE FROM territory_dk.badge_users_badges WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .ok();

    sqlx::query("DELETE FROM global.registry_username WHERE user_id = $1")
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

/// Helper to create a JWT token for testing
fn create_test_jwt(user_id: Uuid, territory: &str) -> String {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: Uuid,
        territory: String,
        exp: u64,
        iat: u64,
    }

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev_jwt_secret_please_change_in_production".to_string());

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: user_id,
        territory: territory.to_string(),
        exp: now + 3600, // 1 hour
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .expect("Failed to create JWT")
}

#[actix_web::test]
async fn test_create_invitation_success_as_manager() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    // Setup manager user
    let (user_id, _badge_id) = setup_manager_user(database.pool(), "dk").await;

    // Create JWT token
    let jwt_token = create_test_jwt(user_id, "dk");

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Create request
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations")
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .set_json(&serde_json::json!({
            "max_uses": 5,
            "expires_in_days": 30,
            "metadata": {
                "purpose": "team_invite"
            }
        }))
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;

    // Debug on failure
    if !resp.status().is_success() {
        let status = resp.status();
        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body_bytes);
        panic!(
            "Manager should be able to create invitation. Status: {}, Body: {}",
            status, body_str
        );
    }

    assert_eq!(resp.status(), 201, "Should return 201 Created");

    // Verify response structure
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("id").is_some());
    assert!(body.get("token").is_some());
    assert_eq!(
        body.get("created_by").unwrap(),
        &serde_json::json!(user_id.to_string())
    );
    assert_eq!(body.get("max_uses").unwrap(), &serde_json::json!(5));
    assert!(body.get("invite_url").is_some());

    // Verify invitation was created in database
    let token = body.get("token").unwrap().as_str().unwrap();
    let invitation_id = Uuid::parse_str(body.get("id").unwrap().as_str().unwrap()).unwrap();

    let db_invitation: (i32, Uuid) = sqlx::query_as(
        "SELECT max_uses, created_by FROM territory_dk.invitation_invitations_tokens WHERE token = $1",
    )
    .bind(token)
    .fetch_one(database.pool())
    .await
    .expect("Invitation should exist in database");

    assert_eq!(db_invitation.0, 5, "max_uses should match");
    assert_eq!(db_invitation.1, user_id, "created_by should match");

    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, token).await;
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_create_invitation_forbidden_non_manager() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    // Create regular user (no manager badge or assignment)
    let user_id = Uuid::new_v4();
    let username = format!(
        "regularuser_{}",
        Uuid::new_v4().to_string()[0..8].to_string()
    );

    sqlx::query(
        "INSERT INTO territory_dk.auth_users_core (id, username, password_hash, territory_code) VALUES ($1, $2, 'test_hash', 'dk')",
    )
    .bind(user_id)
    .bind(&username)
    .execute(database.pool())
    .await
    .expect("Failed to create test user");

    sqlx::query(
        "INSERT INTO global.registry_username (username, user_id, territory_code) VALUES ($1, $2, 'dk')",
    )
    .bind(&username)
    .bind(user_id)
    .execute(database.pool())
    .await
    .expect("Failed to register user globally");

    // Create JWT token for non-manager
    let jwt_token = create_test_jwt(user_id, "dk");

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Create request
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations")
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .set_json(&serde_json::json!({
            "max_uses": 1,
            "expires_in_days": 7
        }))
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;

    // Should return 403 Forbidden
    assert_eq!(
        resp.status().as_u16(),
        403,
        "Non-manager should receive 403 Forbidden"
    );

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body.get("error").unwrap(), "Forbidden");

    // Cleanup
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_create_invitation_unauthorized_no_jwt() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // Create request WITHOUT Authorization header
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations")
        .set_json(&serde_json::json!({
            "max_uses": 1,
            "expires_in_days": 7
        }))
        .to_request();

    // Send request
    let resp = test::call_service(&app, req).await;

    // Should return 401 Unauthorized
    assert_eq!(
        resp.status().as_u16(),
        401,
        "Request without JWT should receive 401 Unauthorized"
    );
}

// ============================================================================
// Tests for list, view uses, and revoke endpoints (issue #141)
// ============================================================================

#[actix_web::test]
async fn test_list_my_invitations_success() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    
    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");
    
    // Setup manager user
    let (user_id, _badge_id) = setup_manager_user(database.pool(), "dk").await;
    
    // Create JWT token
    let jwt_token = create_test_jwt(user_id, "dk");
    
    // Create a test invitation for this user
    let invitation_id = Uuid::new_v4();
    let token = generate_token();
    
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active)
        VALUES ($1, $2, $3, 5, 2, true)
        "#,
    )
    .bind(invitation_id)
    .bind(&token)
    .bind(user_id)
    .execute(database.pool())
    .await
    .expect("Failed to create test invitation");
    
    sqlx::query(
        "INSERT INTO global.registry_invitation (token, territory_code, territory_token_id) VALUES ($1, 'dk', $2)",
    )
    .bind(&token)
    .bind(invitation_id)
    .execute(database.pool())
    .await
    .expect("Failed to create global registry entry");
    
    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;
    
    // Create request
    let req = test::TestRequest::get()
        .uri("/api/v1/invitations/me")
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .to_request();
    
    // Send request
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 200, "Should return 200 OK");
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    let invitations = body.get("invitations").unwrap().as_array().unwrap();
    
    assert!(invitations.len() >= 1, "Should have at least 1 invitation");
    
    // Find our created invitation
    let our_invitation = invitations.iter().find(|inv| {
        inv.get("token").unwrap().as_str().unwrap() == token
    });
    
    assert!(our_invitation.is_some(), "Should find our invitation");
    let our_invitation = our_invitation.unwrap();
    assert_eq!(our_invitation.get("max_uses").unwrap(), 5);
    assert_eq!(our_invitation.get("uses_count").unwrap(), 2);
    
    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_list_my_invitations_with_filter() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    
    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");
    
    // Setup manager user
    let (user_id, _badge_id) = setup_manager_user(database.pool(), "dk").await;
    
    // Create JWT token
    let jwt_token = create_test_jwt(user_id, "dk");
    
    // Create active invitation
    let active_id = Uuid::new_v4();
    let active_token = generate_token();
    
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active, expires_at)
        VALUES ($1, $2, $3, 5, 0, true, NOW() + INTERVAL '7 days')
        "#,
    )
    .bind(active_id)
    .bind(&active_token)
    .bind(user_id)
    .execute(database.pool())
    .await
    .expect("Failed to create active invitation");
    
    sqlx::query(
        "INSERT INTO global.registry_invitation (token, territory_code, territory_token_id) VALUES ($1, 'dk', $2)",
    )
    .bind(&active_token)
    .bind(active_id)
    .execute(database.pool())
    .await
    .expect("Failed to create global registry entry");
    
    // Create revoked invitation
    let revoked_id = Uuid::new_v4();
    let revoked_token = generate_token();
    
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active, revoked_at, revoked_by)
        VALUES ($1, $2, $3, 5, 0, false, NOW(), $3)
        "#,
    )
    .bind(revoked_id)
    .bind(&revoked_token)
    .bind(user_id)
    .execute(database.pool())
    .await
    .expect("Failed to create revoked invitation");
    
    sqlx::query(
        "INSERT INTO global.registry_invitation (token, territory_code, territory_token_id) VALUES ($1, 'dk', $2)",
    )
    .bind(&revoked_token)
    .bind(revoked_id)
    .execute(database.pool())
    .await
    .expect("Failed to create global registry entry");
    
    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;
    
    // Test filter by active status
    let req = test::TestRequest::get()
        .uri("/api/v1/invitations/me?status=active")
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    let invitations = body.get("invitations").unwrap().as_array().unwrap();
    
    // Should only contain active invitations
    for inv in invitations {
        let status = inv.get("status").unwrap().as_str().unwrap();
        assert_eq!(status, "active", "Filtered by active should only return active invitations");
    }
    
    // Cleanup
    cleanup_test_invitation(database.pool(), active_id, &active_token).await;
    cleanup_test_invitation(database.pool(), revoked_id, &revoked_token).await;
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_get_invitation_uses_as_creator() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    
    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");
    
    // Setup manager user
    let (user_id, _badge_id) = setup_manager_user(database.pool(), "dk").await;
    
    // Create JWT token
    let jwt_token = create_test_jwt(user_id, "dk");
    
    // Create test invitation
    let (invitation_id, token, _) = create_test_invitation_custom(
        database.pool(),
        3,
        0,
        true,
        "NOW() + INTERVAL '7 days'",
    )
    .await;
    
    // Update created_by to our user
    sqlx::query(
        "UPDATE territory_dk.invitation_invitations_tokens SET created_by = $1 WHERE id = $2",
    )
    .bind(user_id)
    .bind(invitation_id)
    .execute(database.pool())
    .await
    .expect("Failed to update created_by");
    
    // Add some uses
    let user1_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_uses (invitation_id, used_by, used_at)
        VALUES ($1, $2, NOW())
        "#,
    )
    .bind(invitation_id)
    .bind(user1_id)
    .execute(database.pool())
    .await
    .expect("Failed to create use record");
    
    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;
    
    // Request invitation uses
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/invitations/{}/uses", invitation_id))
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Debug on failure
    if !resp.status().is_success() {
        let status = resp.status();
        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body_bytes);
        panic!(
            "Creator should be able to view uses. Status: {}, Body: {}",
            status, body_str
        );
    }
    
    assert_eq!(resp.status(), 200);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body.get("invitation_id").unwrap(), &serde_json::json!(invitation_id.to_string()));
    assert_eq!(body.get("max_uses").unwrap(), 3);
    
    // Cleanup
    cleanup_test_uses(database.pool(), invitation_id).await;
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_get_invitation_uses_forbidden_non_creator() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    
    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");
    
    // Create a regular user (not the creator)
    let user_id = Uuid::new_v4();
    let username = format!("regularuser_{}", Uuid::new_v4().to_string()[0..8].to_string());
    
    sqlx::query(
        "INSERT INTO territory_dk.auth_users_core (id, username, password_hash, territory_code) VALUES ($1, $2, 'test_hash', 'dk')",
    )
    .bind(user_id)
    .bind(&username)
    .execute(database.pool())
    .await
    .expect("Failed to create test user");
    
    sqlx::query(
        "INSERT INTO global.registry_username (username, user_id, territory_code) VALUES ($1, $2, 'dk')",
    )
    .bind(&username)
    .bind(user_id)
    .execute(database.pool())
    .await
    .expect("Failed to register user globally");
    
    // Create JWT token
    let jwt_token = create_test_jwt(user_id, "dk");
    
    // Create invitation by someone else
    let (invitation_id, token, _) = create_test_invitation(database.pool()).await;
    
    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;
    
    // Try to view uses
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/invitations/{}/uses", invitation_id))
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(
        resp.status().as_u16(),
        403,
        "Non-creator should receive 403 Forbidden"
    );
    
    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_revoke_invitation_success() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    
    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");
    
    // Setup manager user
    let (user_id, _badge_id) = setup_manager_user(database.pool(), "dk").await;
    
    // Create JWT token
    let jwt_token = create_test_jwt(user_id, "dk");
    
    // Create test invitation
    let invitation_id = Uuid::new_v4();
    let token = generate_token();
    
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active)
        VALUES ($1, $2, $3, 5, 0, true)
        "#,
    )
    .bind(invitation_id)
    .bind(&token)
    .bind(user_id)
    .execute(database.pool())
    .await
    .expect("Failed to create test invitation");
    
    sqlx::query(
        "INSERT INTO global.registry_invitation (token, territory_code, territory_token_id) VALUES ($1, 'dk', $2)",
    )
    .bind(&token)
    .bind(invitation_id)
    .execute(database.pool())
    .await
    .expect("Failed to create global registry entry");
    
    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;
    
    // Revoke invitation
    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/invitations/{}", invitation_id))
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 200, "Should successfully revoke");
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body.get("invitation_id").unwrap(), &serde_json::json!(invitation_id.to_string()));
    assert!(body.get("revoked_at").is_some());
    
    // Verify in database
    let (is_active, revoked_at, revoked_by): (bool, Option<chrono::DateTime<chrono::Utc>>, Option<Uuid>) = 
        sqlx::query_as(
            "SELECT is_active, revoked_at, revoked_by FROM territory_dk.invitation_invitations_tokens WHERE id = $1",
        )
        .bind(invitation_id)
        .fetch_one(database.pool())
        .await
        .expect("Failed to query invitation");
    
    assert!(!is_active, "Should be inactive");
    assert!(revoked_at.is_some(), "Should have revoked_at timestamp");
    assert_eq!(revoked_by, Some(user_id), "Should have revoked_by set");
    
    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_revoke_invitation_forbidden_non_creator() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    
    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");
    
    // Create a regular user
    let user_id = Uuid::new_v4();
    let username = format!("regularuser_{}", Uuid::new_v4().to_string()[0..8].to_string());
    
    sqlx::query(
        "INSERT INTO territory_dk.auth_users_core (id, username, password_hash, territory_code) VALUES ($1, $2, 'test_hash', 'dk')",
    )
    .bind(user_id)
    .bind(&username)
    .execute(database.pool())
    .await
    .expect("Failed to create test user");
    
    sqlx::query(
        "INSERT INTO global.registry_username (username, user_id, territory_code) VALUES ($1, $2, 'dk')",
    )
    .bind(&username)
    .bind(user_id)
    .execute(database.pool())
    .await
    .expect("Failed to register user globally");
    
    // Create JWT token
    let jwt_token = create_test_jwt(user_id, "dk");
    
    // Create invitation by someone else
    let (invitation_id, token, _) = create_test_invitation(database.pool()).await;
    
    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;
    
    // Try to revoke
    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/invitations/{}", invitation_id))
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(
        resp.status().as_u16(),
        403,
        "Non-creator should receive 403 Forbidden"
    );
    
    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_revoke_invitation_already_revoked() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");
    
    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    
    // Initialize NATS client
    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");
    
    // Setup manager user
    let (user_id, _badge_id) = setup_manager_user(database.pool(), "dk").await;
    
    // Create JWT token
    let jwt_token = create_test_jwt(user_id, "dk");
    
    // Create already-revoked invitation
    let invitation_id = Uuid::new_v4();
    let token = generate_token();
    
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active, revoked_at, revoked_by)
        VALUES ($1, $2, $3, 5, 0, false, NOW(), $3)
        "#,
    )
    .bind(invitation_id)
    .bind(&token)
    .bind(user_id)
    .execute(database.pool())
    .await
    .expect("Failed to create revoked invitation");
    
    sqlx::query(
        "INSERT INTO global.registry_invitation (token, territory_code, territory_token_id) VALUES ($1, 'dk', $2)",
    )
    .bind(&token)
    .bind(invitation_id)
    .execute(database.pool())
    .await
    .expect("Failed to create global registry entry");
    
    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;
    
    // Try to revoke again
    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/invitations/{}", invitation_id))
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(
        resp.status().as_u16(),
        409,
        "Already revoked should return 409 Conflict"
    );
    
    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
    cleanup_manager_user(database.pool(), user_id).await;
}

#[actix_web::test]
async fn test_validate_invitation_zombie_revoked_permissions() {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

    // Connect to database
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Create test invitation (which also sets up the user with permissions)
    let (invitation_id, token, created_by) = create_test_invitation(database.pool()).await;

    // Initialize test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .service(web::scope("/api/v1").configure(handlers::configure)),
    )
    .await;

    // 1. Verify it's valid initially
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/validate")
        .set_json(&ValidateInvitationRequest {
            token: token.clone(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: ValidateInvitationResponse = test::read_body_json(resp).await;
    assert!(body.valid, "Token should be valid initially");

    // 2. Revoke permissions (remove from territory managers)
    sqlx::query("DELETE FROM territory_dk.territory_territories_managers WHERE user_id = $1")
        .bind(created_by)
        .execute(database.pool())
        .await
        .expect("Failed to revoke manager role");

    // 3. Verify it's INVALID now
    let req = test::TestRequest::post()
        .uri("/api/v1/invitations/validate")
        .set_json(&ValidateInvitationRequest {
            token: token.clone(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: ValidateInvitationResponse = test::read_body_json(resp).await;
    assert!(!body.valid, "Token should be invalid after manager permissions are revoked");
    
    // Cleanup
    cleanup_test_invitation(database.pool(), invitation_id, &token).await;
}
