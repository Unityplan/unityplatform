use actix_web::{test, web, App};
use serde_json::json;

use crate::common::*;

#[actix_web::test]
async fn test_create_invitation_authenticated() {
    let mut ctx = TestContext::new().await;

    let (_user_id, username, password, _email) = ctx.create_user().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
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
        "username": username,
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

    // Create invitation with unique email
    let unique_id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let create_req = json!({
        "token_type": "single_use",
        "email": format!("invited_{}@test.dk", unique_id),
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
        use actix_web::body::MessageBody;
        let body_bytes = resp.into_body().try_into_bytes().unwrap();
        let body_str = String::from_utf8_lossy(&body_bytes);
        eprintln!("Create invitation failed - Status: {}", status);
        eprintln!("Response body: {}", body_str);
        panic!("Invitation creation failed");
    }

    assert_eq!(status, 201, "Invitation creation should succeed");

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["token"].is_string(), "Should return invitation token");

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_create_invitation_unauthenticated() {
    let ctx = TestContext::new().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
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

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_list_user_invitations() {
    let mut ctx = TestContext::new().await;

    let (user_id, username, password, _email) = ctx.create_user().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
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

    // Create some test invitations owned by this user
    ctx.create_invitation_with_user(user_id).await;
    ctx.create_invitation_with_user(user_id).await;

    // Login
    let login_req = json!({
        "username": username,
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

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_revoke_invitation() {
    let mut ctx = TestContext::new().await;

    let (user_id, username, password, _email) = ctx.create_user().await;

    let (invitation_id, invitation_token) = ctx.create_invitation_with_user(user_id).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
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
                                "/{id}",
                                web::delete()
                                    .to(auth_service::handlers::invitation::revoke_invitation),
                            ),
                    ),
            ),
    )
    .await;

    // Login
    let login_req = json!({
        "username": username,
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
    let req = test::TestRequest::delete()
        .uri(&format!("/api/auth/invitations/{}", invitation_id))
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200, "Should revoke invitation");

    // Verify it was revoked (is_active = false means revoked)
    let is_active = sqlx::query_scalar::<_, bool>(
        "SELECT is_active FROM territory.invitation_tokens WHERE token = $1",
    )
    .bind(&invitation_token)
    .fetch_one(&ctx.pool)
    .await
    .expect("Should fetch revocation status");

    assert!(
        !is_active,
        "Invitation should be marked as inactive (revoked)"
    );

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_validate_invitation_maxed_out() {
    let mut ctx = TestContext::new().await;

    let invitation_token = ctx.create_maxed_invitation().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/invitations/validate/{token}",
                web::get().to(auth_service::handlers::invitation::validate_invitation),
            ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/auth/invitations/validate/{}?territory_code=dk",
            invitation_token
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400, "Should reject maxed out invitation");

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_validate_invitation_revoked() {
    let mut ctx = TestContext::new().await;

    let invitation_token = ctx.create_revoked_invitation().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/invitations/validate/{token}",
                web::get().to(auth_service::handlers::invitation::validate_invitation),
            ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/auth/invitations/validate/{}?territory_code=dk",
            invitation_token
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400, "Should reject revoked invitation");

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_invitation_email_mismatch() {
    let mut ctx = TestContext::new().await;

    // Create invitation for a specific email to test mismatch validation
    let unique_id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let invited_email = format!("invited_{}@test.dk", unique_id);
    let invitation_token = ctx.create_invitation_for_email(Some(invited_email)).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/register",
                web::post().to(auth_service::handlers::auth::register),
            ),
    )
    .await;

    // Try to register with different email than invitation
    let different_email = format!("different_{}@test.dk", unique_id);
    let register_req = json!({
        "email": different_email,
        "username": format!("different_user_{}", unique_id),
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

    ctx.cleanup().await;
}
