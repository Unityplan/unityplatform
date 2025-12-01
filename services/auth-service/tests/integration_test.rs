use actix_web::{test, web, App};
use auth_service::handlers;
use auth_service::services::TokenService;
use serde::{Deserialize, Serialize};
use shared_lib::{AppConfig, Database, NatsClient};
use sqlx::PgPool;
use uuid::Uuid;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestRegisterRequest {
    username: String,
    email: Option<String>,
    password: String,
    territory: String,
    invitation_token: Option<String>,
}

async fn cleanup_test_data(pool: &PgPool, username: &str, _invitation_token: Option<&str>) {
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

    // Note: No invitation cleanup needed - we use mocked invitation service
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

    // Mock invitation-service HTTP endpoint
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/invitations/validate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "valid": true
        })))
        .mount(&mock_server)
        .await;

    // Set invitation service URL to mock server
    std::env::set_var("INVITATION_SERVICE_URL", mock_server.uri());

    let token = format!("TEST-{}", Uuid::new_v4());
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
    cleanup_test_data(database.pool(), &username, None).await;
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestLoginRequest {
    username: String,
    password: String,
    territory: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestRefreshRequest {
    refresh_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestLogoutRequest {
    refresh_token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestAuthResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestValidateResponse {
    valid: bool,
    user_id: Option<Uuid>,
}

#[derive(Deserialize)]
struct TestRefreshResponse {
    access_token: String,
}

#[actix_web::test]
async fn test_auth_flow() {
    dotenvy::dotenv().ok();
    let mut config = AppConfig::from_env().expect("Failed to load config");

    // Enable open registration for easier testing
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
            .service(handlers::register)
            .service(handlers::login)
            .service(handlers::refresh)
            .service(handlers::logout)
            .service(handlers::validate),
    )
    .await;

    let username = format!("auth_flow_{}", Uuid::new_v4().simple());
    let password = "Password123!";
    let territory = "dk";

    // 1. Register
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&TestRegisterRequest {
            username: username.clone(),
            email: Some(format!("{}@example.com", username)),
            password: password.to_string(),
            territory: territory.to_string(),
            invitation_token: None,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    // 2. Login
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&TestLoginRequest {
            username: username.clone(),
            password: password.to_string(),
            territory: territory.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let auth_resp: TestAuthResponse = test::read_body_json(resp).await;
    let access_token = auth_resp.access_token;
    let refresh_token = auth_resp.refresh_token;

    // 3. Validate Token
    let req = test::TestRequest::get()
        .uri("/validate")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let validate_resp: TestValidateResponse = test::read_body_json(resp).await;
    assert!(validate_resp.valid);
    assert!(validate_resp.user_id.is_some());

    // 4. Refresh Token
    let req = test::TestRequest::post()
        .uri("/refresh")
        .set_json(&TestRefreshRequest {
            refresh_token: refresh_token.clone(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let refresh_resp: TestRefreshResponse = test::read_body_json(resp).await;
    let new_access_token = refresh_resp.access_token;
    assert_ne!(access_token, new_access_token);

    // 5. Logout
    let req = test::TestRequest::post()
        .uri("/logout")
        .insert_header(("Authorization", format!("Bearer {}", new_access_token)))
        .set_json(&TestLogoutRequest {
            refresh_token: refresh_token.clone(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    // 6. Verify Logout (Refresh token should be invalid)
    let req = test::TestRequest::post()
        .uri("/refresh")
        .set_json(&TestRefreshRequest {
            refresh_token: refresh_token.clone(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);

    // Cleanup
    cleanup_test_data(database.pool(), &username, None).await;
}
