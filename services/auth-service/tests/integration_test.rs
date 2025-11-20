use actix_web::{test, web, App};
use auth_service::handlers;
use auth_service::services::TokenService;
use serde::Serialize;
use shared_lib::{AppConfig, Database, NatsClient};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestRegisterRequest {
    username: String,
    email: Option<String>,
    password: String,
    territory: String,
    invitation_token: Option<String>,
}

/// Helper to create a test invitation directly in the DB
/// This mimics what invitation-service does, so we can test the integration
async fn create_test_invitation(pool: &PgPool, territory_code: &str) -> (Uuid, String) {
    let invitation_id = Uuid::new_v4();
    // Generate a random token (simple format for test)
    let token = format!("TEST-{}", Uuid::new_v4());
    let created_by = Uuid::new_v4(); // We need a user ID for created_by

    // 1. Create the creator user in global registry (so they exist)
    let username = format!("creator_{}", Uuid::new_v4().simple());
    sqlx::query(
        "INSERT INTO global.registry_username (username, territory_code, user_id) VALUES ($1, $2, $3)"
    )
    .bind(&username)
    .bind(territory_code)
    .bind(created_by)
    .execute(pool)
    .await
    .expect("Failed to create creator user");

    // 2. Insert invitation into territory table
    let table_name = format!("territory_{}.invitation_invitations_tokens", territory_code);
    let query = format!(
        r#"
        INSERT INTO {} 
        (id, token, created_by, max_uses, uses_count, is_active, expires_at)
        VALUES ($1, $2, $3, 1, 0, true, NOW() + INTERVAL '1 day')
        "#,
        table_name
    );

    sqlx::query(&query)
        .bind(invitation_id)
        .bind(&token)
        .bind(created_by)
        .execute(pool)
        .await
        .expect("Failed to insert invitation token");

    // 3. Insert into global registry
    sqlx::query(
        "INSERT INTO global.registry_invitation (token, territory_code, territory_token_id) VALUES ($1, $2, $3)"
    )
    .bind(&token)
    .bind(territory_code)
    .bind(invitation_id)
    .execute(pool)
    .await
    .expect("Failed to insert global invitation");

    // 4. Give creator manager permissions (required for validation)
    // Create auth user
    let auth_table = format!("territory_{}.auth_users_core", territory_code);
    let email = format!("{}@example.com", username);
    sqlx::query(&format!(
        "INSERT INTO {} (id, username, email, password_hash, active) VALUES ($1, $2, $3, 'hash', true)",
        auth_table
    ))
    .bind(created_by)
    .bind(&username)
    .bind(email)
    .execute(pool)
    .await
    .expect("Failed to create auth user");

    // Get badge id
    let badge_id: Uuid =
        sqlx::query_scalar("SELECT id FROM global.registry_badge WHERE slug = 'territory-manager'")
            .fetch_one(pool)
            .await
            .expect("Failed to get badge id");

    // Assign badge
    let badge_table = format!("territory_{}.badge_users_badges", territory_code);
    sqlx::query(&format!(
        "INSERT INTO {} (user_id, badge_id, awarded_at) VALUES ($1, $2, NOW())",
        badge_table
    ))
    .bind(created_by)
    .bind(badge_id)
    .execute(pool)
    .await
    .expect("Failed to assign badge");

    // Assign manager role
    let manager_table = format!(
        "territory_{}.territory_territories_managers",
        territory_code
    );
    sqlx::query(&format!(
        "INSERT INTO {} (territory_code, user_id, assigned_at) VALUES ($1, $2, NOW())",
        manager_table
    ))
    .bind(territory_code)
    .bind(created_by)
    .execute(pool)
    .await
    .expect("Failed to assign manager role");

    (invitation_id, token)
}

async fn cleanup_test_data(pool: &PgPool, username: &str, invitation_token: Option<&str>) {
    // Delete user (cascades to most things)
    // But we need to be careful about global registry

    // 1. Get user_id
    let user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT user_id FROM global.registry_username WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await
            .unwrap_or(None);

    if let Some(uid) = user_id {
        // Delete from territory auth table
        sqlx::query("DELETE FROM territory_dk.auth_users_core WHERE id = $1")
            .bind(uid)
            .execute(pool)
            .await
            .ok();

        // Delete from global username registry
        sqlx::query("DELETE FROM global.registry_username WHERE user_id = $1")
            .bind(uid)
            .execute(pool)
            .await
            .ok();
    }

    if let Some(token) = invitation_token {
        // Delete invitation
        sqlx::query("DELETE FROM territory_dk.invitation_invitations_tokens WHERE token = $1")
            .bind(token)
            .execute(pool)
            .await
            .ok();

        sqlx::query("DELETE FROM global.registry_invitation WHERE token = $1")
            .bind(token)
            .execute(pool)
            .await
            .ok();
    }
}

#[actix_web::test]
async fn test_register_production_mode_success() {
    dotenvy::dotenv().ok();
    let mut config = AppConfig::from_env().expect("Failed to load config");

    // FORCE PRODUCTION MODE for this test
    config.server.allow_open_registration = false;

    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    let token_service = TokenService::new(&config.auth.jwt_secret);

    // Create a valid invitation
    let (_inv_id, token) = create_test_invitation(database.pool(), "dk").await;
    let username = format!("test_user_{}", Uuid::new_v4().simple());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .app_data(web::Data::new(token_service))
            .service(handlers::register),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&TestRegisterRequest {
            username: username.clone(),
            email: Some(format!("{}@example.com", username)),
            password: "Password123!".to_string(),
            territory: "dk".to_string(),
            invitation_token: Some(token.clone()),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;

    if !resp.status().is_success() {
        let body = test::read_body(resp).await;
        println!("Response body: {:?}", body);
        panic!("Registration failed");
    }

    assert_eq!(resp.status(), 201);

    // Cleanup
    cleanup_test_data(database.pool(), &username, Some(&token)).await;
}

#[actix_web::test]
async fn test_register_production_mode_missing_token() {
    dotenvy::dotenv().ok();
    let mut config = AppConfig::from_env().expect("Failed to load config");

    // FORCE PRODUCTION MODE
    config.server.allow_open_registration = false;

    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    let token_service = TokenService::new(&config.auth.jwt_secret);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .app_data(web::Data::new(token_service))
            .service(handlers::register),
    )
    .await;

    let username = format!("test_user_{}", Uuid::new_v4().simple());
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&TestRegisterRequest {
            username: username.clone(),
            email: Some(format!("{}@example.com", username)),
            password: "Password123!".to_string(),
            territory: "dk".to_string(),
            invitation_token: None, // MISSING TOKEN
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        400,
        "Should fail without token in production mode"
    );
}

#[actix_web::test]
async fn test_register_dev_mode_no_token() {
    dotenvy::dotenv().ok();
    let mut config = AppConfig::from_env().expect("Failed to load config");

    // FORCE DEV MODE
    config.server.allow_open_registration = true;

    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    let nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");

    let token_service = TokenService::new(&config.auth.jwt_secret);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client))
            .app_data(web::Data::new(token_service))
            .service(handlers::register),
    )
    .await;

    let username = format!("test_user_{}", Uuid::new_v4().simple());
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&TestRegisterRequest {
            username: username.clone(),
            email: Some(format!("{}@example.com", username)),
            password: "Password123!".to_string(),
            territory: "dk".to_string(),
            invitation_token: None, // No token, but allowed in dev
        })
        .to_request();

    let resp = test::call_service(&app, req).await;

    if !resp.status().is_success() {
        let body = test::read_body(resp).await;
        println!("Response body: {:?}", body);
        panic!("Registration failed in dev mode");
    }

    assert_eq!(resp.status(), 201);

    // Cleanup
    cleanup_test_data(database.pool(), &username, None).await;
}
