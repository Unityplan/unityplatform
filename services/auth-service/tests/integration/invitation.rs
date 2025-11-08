use actix_web::{test, web, App};
use serde_json::json;

use crate::common::*;

#[actix_web::test]
async fn test_create_invitation_authenticated() {
    let pool = get_test_pool().await;
    setup_test_data(&pool).await;

    let (email, password) = create_test_user(&pool, "territory_dk").await;
    let token_service = create_token_service();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .service(
                web::scope("/api/auth")
                    .route(
                        "/login",
                        web::post().to(auth_service::handlers::auth::login),
                    )
                    // Protected invitation endpoints - same as production
                    .service(
                        web::scope("/invitations")
                            .wrap(auth_service::middleware::JwtAuth)
                            .route(
                                "",
                                web::post()
                                    .to(auth_service::handlers::invitation::create_invitation),
                            ),
                    ),
            ),
    )
    .await;

    // Login to get access token
    let login_req = json!({
        "email": email,
        "password": password,
        "territory_code": "dk"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let access_token = body["access_token"].as_str().unwrap();

    // Create invitation
    let create_req = json!({
        "token_type": "single_use",
        "email": "invited@test.dk",
        "max_uses": 1,
        "expires_in_days": 7,
        "purpose": "Test invitation"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/invitations")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .set_json(&create_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();

    if status != 201 {
        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body_bytes);
        eprintln!("Create invitation error - Status: {}", status);
        eprintln!("Response body: {}", body_str);
        panic!("Invitation creation failed");
    }

    assert_eq!(status, 201, "Invitation creation should succeed");

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["token"].is_string(), "Should return invitation token");

    cleanup_test_data(&pool).await;
}

#[actix_web::test]
async fn test_create_invitation_unauthenticated() {
    let pool = get_test_pool().await;
    setup_test_data(&pool).await;

    let token_service = create_token_service();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .route(
                "/api/auth/invitations",
                web::post().to(auth_service::handlers::invitation::create_invitation),
            ),
    )
    .await;

    let create_req = json!({
        "token_type": "user",
        "email": "invited@test.dk",
        "max_uses": 1,
        "expires_in_days": 7,
        "purpose": "Test invitation"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/invitations")
        .set_json(&create_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401, "Should require authentication");

    cleanup_test_data(&pool).await;
}

#[actix_web::test]
async fn test_list_user_invitations() {
    let pool = get_test_pool().await;
    setup_test_data(&pool).await;

    let (email, password) = create_test_user(&pool, "territory_dk").await;
    let token_service = create_token_service();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .service(
                web::scope("/api/auth")
                    .route(
                        "/login",
                        web::post().to(auth_service::handlers::auth::login),
                    )
                    .service(
                        web::scope("/invitations")
                            .wrap(auth_service::middleware::JwtAuth)
                            .route(
                                "",
                                web::get().to(auth_service::handlers::invitation::list_invitations),
                            ),
                    ),
            ),
    )
    .await;

    // Create some test invitations
    create_test_invitation(&pool, "territory_dk").await;
    create_test_invitation(&pool, "territory_dk").await;

    // Login
    let login_req = json!({
        "email": email,
        "password": password,
        "territory_code": "dk"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let access_token = body["access_token"].as_str().unwrap();

    // List invitations
    let req = test::TestRequest::get()
        .uri("/api/auth/invitations")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200, "Should list invitations");

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array(), "Should return array of invitations");

    cleanup_test_data(&pool).await;
}

#[actix_web::test]
async fn test_revoke_invitation() {
    let pool = get_test_pool().await;
    setup_test_data(&pool).await;

    let (email, password) = create_test_user(&pool, "territory_dk").await;
    let token_service = create_token_service();
    let invitation_token = create_test_invitation(&pool, "territory_dk").await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .service(
                web::scope("/api/auth")
                    .route(
                        "/login",
                        web::post().to(auth_service::handlers::auth::login),
                    )
                    .service(
                        web::scope("/invitations")
                            .wrap(auth_service::middleware::JwtAuth)
                            .route(
                                "/{token}/revoke",
                                web::post()
                                    .to(auth_service::handlers::invitation::revoke_invitation),
                            ),
                    ),
            ),
    )
    .await;

    // Login
    let login_req = json!({
        "email": email,
        "password": password,
        "territory_code": "dk"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let access_token = body["access_token"].as_str().unwrap();

    // Revoke invitation
    let req = test::TestRequest::post()
        .uri(&format!(
            "/api/auth/invitations/{}/revoke",
            invitation_token
        ))
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200, "Should revoke invitation");

    // Verify it was revoked (is_active = false means revoked)
    let is_active = sqlx::query_scalar::<_, bool>(
        "SELECT is_active FROM territory_dk.invitation_tokens WHERE token = $1",
    )
    .bind(&invitation_token)
    .fetch_one(&pool)
    .await
    .expect("Should fetch revocation status");

    assert!(
        !is_active,
        "Invitation should be marked as inactive (revoked)"
    );

    cleanup_test_data(&pool).await;
}

#[actix_web::test]
async fn test_validate_invitation_maxed_out() {
    let pool = get_test_pool().await;
    setup_test_data(&pool).await;

    let token_service = create_token_service();
    let invitation_token = create_maxed_invitation(&pool, "territory_dk").await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .route(
                "/api/auth/invitations/validate",
                web::post().to(auth_service::handlers::invitation::validate_invitation),
            ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/auth/invitations/validate")
        .set_json(&json!({ "token": invitation_token }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400, "Should reject maxed out invitation");

    cleanup_test_data(&pool).await;
}

#[actix_web::test]
async fn test_validate_invitation_revoked() {
    let pool = get_test_pool().await;
    setup_test_data(&pool).await;

    let token_service = create_token_service();
    let invitation_token = create_revoked_invitation(&pool, "territory_dk").await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .route(
                "/api/auth/invitations/validate",
                web::post().to(auth_service::handlers::invitation::validate_invitation),
            ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/auth/invitations/validate")
        .set_json(&json!({ "token": invitation_token }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400, "Should reject revoked invitation");

    cleanup_test_data(&pool).await;
}

#[actix_web::test]
async fn test_invitation_email_mismatch() {
    let pool = get_test_pool().await;
    setup_test_data(&pool).await;

    let token_service = create_token_service();
    let invitation_token = create_test_invitation(&pool, "territory_dk").await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .route(
                "/api/auth/register",
                web::post().to(auth_service::handlers::auth::register),
            ),
    )
    .await;

    // Try to register with different email than invitation
    let register_req = json!({
        "email": "different@test.dk",
        "username": "different_user",
        "password": "StrongPassword123!",
        "full_name": "Test User",
        "territory_code": "dk",
        "invitation_token": invitation_token
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400, "Should reject email mismatch");

    cleanup_test_data(&pool).await;
}
