use actix_web::{test, web, App};
use serde_json::json;

use crate::common::*;

#[actix_web::test]
async fn test_register_success() {
    let mut ctx = TestContext::new().await;

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

    let invitation_token = ctx.create_invitation().await;

    // Generate unique credentials for this test run
    let unique_id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let register_req = json!({
        "email": format!("newuser_{}@test.dk", unique_id),
        "username": format!("newuser_{}", unique_id),
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
    let status = resp.status();

    if status != 201 {
        use actix_web::body::MessageBody;
        let body_bytes = resp.into_body().try_into_bytes().unwrap();
        let body_str = String::from_utf8_lossy(&body_bytes);
        eprintln!("Response status: {}", status);
        eprintln!("Response body: {}", body_str);
        panic!("Registration failed");
    }

    assert_eq!(status, 201, "Registration should succeed");

    let uses = sqlx::query_scalar::<_, i32>(
        "SELECT current_uses FROM territory.invitation_tokens WHERE token = $1",
    )
    .bind(&invitation_token)
    .fetch_one(&ctx.pool)
    .await
    .expect("Should fetch invitation use count");

    assert_eq!(uses, 1, "Invitation should be marked as used");

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_register_invalid_invitation() {
    let ctx = TestContext::new().await;

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

    // Generate unique credentials for this test run
    let unique_id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let register_req = json!({
        "email": format!("newuser_{}@test.dk", unique_id),
        "username": format!("newuser_invalid_{}", unique_id),
        "password": "StrongPassword123!",
        "full_name": "Test User",
        "territory_code": "dk",
        "invitation_token": "invalid_token_12345"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        400,
        "Registration should fail with invalid invitation"
    );

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_register_expired_invitation() {
    let mut ctx = TestContext::new().await;

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

    let invitation_token = ctx.create_expired_invitation().await;

    // Generate unique credentials for this test run
    let unique_id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let register_req = json!({
        "email": format!("newuser_{}@test.dk", unique_id),
        "username": format!("newuser_expired_{}", unique_id),
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
    assert_eq!(
        resp.status(),
        400,
        "Registration should fail with expired invitation"
    );

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_login_success() {
    let mut ctx = TestContext::new().await;

    let (_user_id, username, password, _email) = ctx.create_user().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/login",
                web::post().to(auth_service::handlers::auth::login),
            ),
    )
    .await;

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
    let status = resp.status();

    assert_eq!(status, 200, "Login should succeed");

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(
        body["access_token"].is_string(),
        "Should return access token"
    );
    assert!(
        body["refresh_token"].is_string(),
        "Should return refresh token"
    );

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_login_wrong_password() {
    let mut ctx = TestContext::new().await;

    let (_user_id, username, _, _email) = ctx.create_user().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/login",
                web::post().to(auth_service::handlers::auth::login),
            ),
    )
    .await;

    let login_req = json!({
        "username": username,
        "password": "WrongPassword123!",
        "territory_code": "dk"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401, "Login should fail with wrong password");

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_login_nonexistent_user() {
    let ctx = TestContext::new().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/login",
                web::post().to(auth_service::handlers::auth::login),
            ),
    )
    .await;

    let login_req = json!({
        "username": "nonexistent_user",
        "password": "Password123!",
        "territory_code": "dk"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401, "Login should fail for nonexistent user");

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_refresh_token_success() {
    let mut ctx = TestContext::new().await;

    let (_user_id, username, password, _email) = ctx.create_user().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/login",
                web::post().to(auth_service::handlers::auth::login),
            )
            .route(
                "/api/auth/refresh",
                web::post().to(auth_service::handlers::auth::refresh),
            ),
    )
    .await;

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
    let refresh_token = body["refresh_token"].as_str().unwrap();

    let req = test::TestRequest::post()
        .uri("/api/auth/refresh")
        .set_json(&json!({
            "refresh_token": refresh_token,
            "territory_code": "dk"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();

    assert_eq!(status, 200, "Token refresh should succeed");

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(
        body["access_token"].is_string(),
        "Should return new access token"
    );

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_refresh_token_invalid() {
    let ctx = TestContext::new().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/refresh",
                web::post().to(auth_service::handlers::auth::refresh),
            ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/auth/refresh")
        .set_json(&json!({
            "refresh_token": "invalid.token.here",
            "territory_code": "dk"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        401,
        "Token refresh should fail with invalid token"
    );

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_logout() {
    let mut ctx = TestContext::new().await;

    let (_user_id, username, password, _email) = ctx.create_user().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/login",
                web::post().to(auth_service::handlers::auth::login),
            )
            .route(
                "/api/auth/logout",
                web::post().to(auth_service::handlers::auth::logout),
            ),
    )
    .await;

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
    let refresh_token = body["refresh_token"].as_str().unwrap();

    let req = test::TestRequest::post()
        .uri("/api/auth/logout")
        .set_json(&json!({ "refresh_token": refresh_token }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200, "Logout should succeed");

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_me_endpoint() {
    let mut ctx = TestContext::new().await;

    // Create a user and log them in
    let (user_id, username, password, email) = ctx.create_user().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/api/auth/login",
                web::post().to(auth_service::handlers::auth::login),
            )
            .service(
                web::scope("")
                    .wrap(auth_service::middleware::JwtAuth)
                    .route(
                        "/api/auth/me",
                        web::get().to(auth_service::handlers::auth::me),
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
    assert_eq!(resp.status(), 200, "Login should succeed");

    let body: serde_json::Value = test::read_body_json(resp).await;
    let access_token = body["access_token"].as_str().unwrap().to_string();

    // Call /me endpoint with JWT
    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200, "Should get user info");

    let me_data: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(me_data["id"], user_id.to_string());
    assert_eq!(me_data["username"], username);

    // Email is optional - check if it exists in response
    if let Some(email_val) = email {
        assert_eq!(me_data["email"], email_val);
    } else {
        assert!(
            me_data["email"].is_null(),
            "Email should be null when not provided"
        );
    }

    assert_eq!(me_data["territory_code"], "dk");

    ctx.cleanup().await;
}

#[actix_web::test]
async fn test_me_endpoint_unauthenticated() {
    let ctx = TestContext::new().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .service(
                web::scope("")
                    .wrap(auth_service::middleware::JwtAuth)
                    .route(
                        "/api/auth/me",
                        web::get().to(auth_service::handlers::auth::me),
                    ),
            ),
    )
    .await;

    // Call /me without JWT - expect middleware to reject
    let req = test::TestRequest::get().uri("/api/auth/me").to_request();

    // The middleware returns an error response, not a service response
    // We need to use try_call_service to handle the error case
    let resp = test::try_call_service(&app, req).await;

    match resp {
        Ok(resp) => {
            assert_eq!(resp.status(), 401, "Should reject unauthenticated request");
        }
        Err(_) => {
            // If middleware returns an error, that's also expected behavior
            // The important thing is that the request is rejected
        }
    }

    ctx.cleanup().await;
}
