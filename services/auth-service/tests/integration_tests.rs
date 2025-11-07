use actix_web::{test, web, App};
use sqlx::PgPool;
use std::sync::Arc;

async fn get_test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL required");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect")
}

#[actix_web::test]
async fn test_health() {
    let pool = get_test_pool().await;
    
    let token_service = Arc::new(auth_service::services::TokenService::new(
        "test_secret_key",
        900,
        604800,
    ));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .route("/health", web::get().to(auth_service::handlers::health))
    ).await;
    
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
