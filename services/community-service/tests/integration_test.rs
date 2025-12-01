use actix_web::{test, web, App};
use community_service_service::handlers;
use community_service_service::models::{
    AddRequirementRequest, CommunityBadgeRequirement, RequirementContext,
};
use community_service_service::services::CommunityService;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use shared_lib::{AppConfig, Database, NatsClient};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    territory: String,
    #[serde(default)]
    badges: Vec<String>,
    exp: usize,
    iat: usize,
}

fn generate_test_token(user_id: Uuid) -> String {
    generate_test_token_with_badges(user_id, vec![])
}

fn generate_test_token_with_badges(user_id: Uuid, badges: Vec<String>) -> String {
    let my_claims = Claims {
        sub: user_id,
        territory: "dk".to_string(),
        badges,
        exp: 10000000000,
        iat: 10000000000,
    };
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev_jwt_secret_please_change_in_production".to_string());
    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

async fn create_test_badge(pool: &PgPool) -> Uuid {
    let badge_id = Uuid::new_v4();
    let slug = format!("test-badge-{}", Uuid::new_v4());
    sqlx::query(
        "INSERT INTO global.registry_badge (id, slug, name, description, icon, category, criteria_type) VALUES ($1, $2, 'Test Badge', 'Test', 'icon', 'role', 'manual')"
    )
    .bind(badge_id)
    .bind(slug)
    .execute(pool)
    .await
    .expect("Failed to create test badge");
    badge_id
}

async fn create_test_user(pool: &PgPool, user_id: Uuid) {
    let username = format!("test_user_{}", Uuid::new_v4().simple());
    let email = format!("{}@example.com", username);

    // 1. Create in global registry
    sqlx::query(
        "INSERT INTO global.registry_username (username, territory_code, user_id) VALUES ($1, 'dk', $2)"
    )
    .bind(&username)
    .bind(user_id)
    .execute(pool)
    .await
    .expect("Failed to create global user");

    // 2. Create in auth_users_core
    sqlx::query(
        "INSERT INTO territory_dk.auth_users_core (id, username, email, password_hash, active) VALUES ($1, $2, $3, 'hash', true)"
    )
    .bind(user_id)
    .bind(&username)
    .bind(&email)
    .execute(pool)
    .await
    .expect("Failed to create auth user");
}

async fn create_test_community(pool: &PgPool, user_id: Uuid) -> Uuid {
    let community_id = Uuid::new_v4();
    let slug = format!("test-community-{}", Uuid::new_v4());

    // Create community
    sqlx::query(
        r#"
        INSERT INTO territory_dk.community_communities (
            id, slug, name, description, type, created_by
        )
        VALUES ($1, $2, 'Test Community', 'Test', 'guild', $3)
        "#,
    )
    .bind(community_id)
    .bind(slug)
    .bind(user_id)
    .execute(pool)
    .await
    .expect("Failed to create test community");

    // Ensure community-manager badge exists
    sqlx::query(
        "INSERT INTO global.registry_badge (slug, name, description, icon, category, criteria_type, is_active) 
         VALUES ('community-manager', 'Community Manager', 'Manager', 'icon', 'role', 'manual', true)
         ON CONFLICT (slug) DO UPDATE SET is_active = true"
    )
    .execute(pool)
    .await
    .expect("Failed to ensure community-manager badge");

    // Get badge id
    let badge_id: Uuid =
        sqlx::query_scalar("SELECT id FROM global.registry_badge WHERE slug = 'community-manager'")
            .fetch_one(pool)
            .await
            .expect("Failed to get badge id");

    // Assign badge to user
    sqlx::query(
        "INSERT INTO territory_dk.badge_users_badges (badge_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
    )
    .bind(badge_id)
    .bind(user_id)
    .execute(pool)
    .await
    .expect("Failed to assign community-manager badge");

    // Add user as manager (not just member)
    sqlx::query(
        "INSERT INTO territory_dk.community_communities_managers (community_id, user_id) VALUES ($1, $2)"
    )
    .bind(community_id)
    .bind(user_id)
    .execute(pool)
    .await
    .expect("Failed to add manager");

    // Also add as member
    sqlx::query(
        "INSERT INTO territory_dk.community_communities_members (community_id, user_id) VALUES ($1, $2)"
    )
    .bind(community_id)
    .bind(user_id)
    .execute(pool)
    .await
    .expect("Failed to add member");

    // Create settings
    sqlx::query("INSERT INTO territory_dk.community_communities_settings (community_id) VALUES ($1)")
        .bind(community_id)
        .execute(pool)
        .await
        .expect("Failed to create settings");

    community_id
}

#[actix_web::test]
async fn test_badge_requirements() {
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load config");

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

    let community_service = CommunityService::new(database.pool().clone());
    let community_service_data = web::Data::new(community_service.clone());
    let nats_data = web::Data::new(nats_client);

    // Setup test data
    let admin_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    create_test_user(database.pool(), admin_id).await;
    create_test_user(database.pool(), user_id).await;

    let community_id = create_test_community(database.pool(), admin_id).await;
    let badge_id = create_test_badge(database.pool()).await;

    // Admin gets token with community-manager badge (which is assigned in create_test_community)
    let admin_token = generate_test_token_with_badges(admin_id, vec!["community-manager".to_string()]);
    let user_token = generate_test_token(user_id);

    let app = test::init_service(
        App::new()
            .app_data(community_service_data.clone())
            .app_data(nats_data.clone())
            .service(
                web::scope("/api/v1")
                    .service(web::scope("/communities").configure(handlers::configure)),
            ),
    )
    .await;

    // 1. Test Add Requirement (Admin)
    let req = test::TestRequest::post()
        .uri(&format!(
            "/api/v1/communities/{}/requirements",
            community_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(AddRequirementRequest {
            badge_id,
            context: RequirementContext::Participate,
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    if !resp.status().is_success() {
        println!("Status: {}", resp.status());
        let body = test::read_body(resp).await;
        println!("Error response: {:?}", body);
        panic!("Request failed");
    }

    // 2. Test List Requirements
    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/communities/{}/requirements",
            community_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", user_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let requirements: Vec<CommunityBadgeRequirement> = test::read_body_json(resp).await;
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].badge_id, badge_id);
    assert_eq!(requirements[0].context, RequirementContext::Participate);

    // 3. Test Add Requirement (Non-Admin) - Should Fail
    let req = test::TestRequest::post()
        .uri(&format!(
            "/api/v1/communities/{}/requirements",
            community_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", user_token)))
        .set_json(AddRequirementRequest {
            badge_id,
            context: RequirementContext::View,
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);

    // 4. Test Remove Requirement (Non-Admin) - Should Fail
    let req = test::TestRequest::delete()
        .uri(&format!(
            "/api/v1/communities/{}/requirements/{}?context=participate",
            community_id, badge_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", user_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);

    // 5. Test Remove Requirement (Admin)
    let req = test::TestRequest::delete()
        .uri(&format!(
            "/api/v1/communities/{}/requirements/{}?context=participate",
            community_id, badge_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify removal
    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/v1/communities/{}/requirements",
            community_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", user_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let requirements: Vec<CommunityBadgeRequirement> = test::read_body_json(resp).await;
    assert_eq!(requirements.len(), 0);

    // Cleanup
    sqlx::query("DELETE FROM territory_dk.community_communities WHERE id = $1")
        .bind(community_id)
        .execute(database.pool())
        .await
        .ok();

    sqlx::query("DELETE FROM global.registry_badge WHERE id = $1")
        .bind(badge_id)
        .execute(database.pool())
        .await
        .ok();
}
