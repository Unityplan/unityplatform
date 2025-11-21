use actix_web::{test, web, App};
use user_service::handlers;
use shared_lib::{AppConfig, Database};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    territory: String,
    exp: usize,
    iat: usize,
}

fn generate_test_token(user_id: Uuid) -> String {
    let my_claims = Claims {
        sub: user_id,
        territory: "dk".to_string(),
        exp: 10000000000,
        iat: 0,
    };
    
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_jwt_secret_please_change_in_production".to_string());
    encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

#[actix_web::test]
async fn test_health_check() {
    dotenvy::from_filename(".env").ok();

    let app = test::init_service(
        App::new()
            .route("/api/v1/health", web::get().to(handlers::health::health_check))
    ).await;

    let req = test::TestRequest::get().uri("/api/v1/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_own_profile() {
    dotenvy::from_filename(".env").ok();
    
    let config = AppConfig::from_env().expect("Failed to load configuration");
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(
                web::scope("/api/v1/user")
                    .configure(handlers::profile::configure)
            )
    ).await;

    // Create test user
    let user_id = Uuid::new_v4();
    let username = format!("test_user_{}", Uuid::new_v4().simple());
    let email = format!("{}@example.com", username);
    
    // Insert into auth_users_core
    sqlx::query(
        "INSERT INTO territory_dk.auth_users_core (id, username, email, password_hash, active) VALUES ($1, $2, $3, 'hash', true)"
    )
    .bind(user_id)
    .bind(&username)
    .bind(&email)
    .execute(database.pool())
    .await
    .expect("Failed to create auth user");

    // Insert into user_users_profiles
    sqlx::query(
        "INSERT INTO territory_dk.user_users_profiles (user_id, display_name, bio) VALUES ($1, $2, $3)"
    )
    .bind(user_id)
    .bind("Test User")
    .bind("I am a test user")
    .execute(database.pool())
    .await
    .expect("Failed to create user profile");

    let token = generate_test_token(user_id);
    let req = test::TestRequest::get()
        .uri("/api/v1/user/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
        
    let resp = test::call_service(&app, req).await;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = test::read_body(resp).await;
        println!("Response status: {}, body: {:?}", status, body);
        panic!("Request failed");
    }
    
    // Clean up
    sqlx::query("DELETE FROM territory_dk.auth_users_core WHERE id = $1")
        .bind(user_id)
        .execute(database.pool())
        .await
        .ok();
}
