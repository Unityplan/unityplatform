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
