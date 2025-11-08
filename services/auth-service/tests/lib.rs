// Integration tests for auth-service
//
// This file serves as the main entry point for all integration tests.
// Following Rust best practices for scalable test organization:
// - All test modules are organized under tests/integration/
// - Common test utilities are in tests/common/
// - Each module can be run independently or as part of the full suite
//
// Run all tests: cargo test
// Run specific module: cargo test --test integration auth
// Run specific test: cargo test --test integration test_login_success

mod common;
mod integration;

use actix_web::{test, web, App};

/// Health check test to verify basic service functionality
#[actix_web::test]
async fn test_health_endpoint() {
    let ctx = common::TestContext::new().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(ctx.pool.clone()))
            .app_data(web::Data::from(ctx.token_service.clone()))
            .route(
                "/health",
                web::get().to(auth_service::handlers::auth::health),
            ),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_success(),
        "Health endpoint should return success"
    );

    ctx.cleanup().await;
}
