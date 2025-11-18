use actix_web::{test, web, App};
use invitation_service_service::handlers;
use invitation_service_service::models::{ValidateInvitationRequest, ValidateInvitationResponse};
use shared_lib::{AppConfig, Database};
use sqlx::PgPool;
use uuid::Uuid;

/// Helper function to create test invitation in database
async fn create_test_invitation(pool: &PgPool) -> (Uuid, String, Uuid) {
    let invitation_id = Uuid::new_v4();
    let token = "TEST-1234-5678-ABCD".to_string();
    let created_by = Uuid::new_v4();

    // Insert into territory_dk.invitation_invitations_tokens
    sqlx::query(
        r#"
        INSERT INTO territory_dk.invitation_invitations_tokens 
        (id, token, created_by, max_uses, uses_count, is_active, expires_at)
        VALUES ($1, $2, $3, 1, 0, true, NOW() + INTERVAL '7 days')
        "#,
    )
    .bind(invitation_id)
    .bind(&token)
    .bind(created_by)
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

    // Create test invitation
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
