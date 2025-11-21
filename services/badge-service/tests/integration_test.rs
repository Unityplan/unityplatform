use actix_web::{test, web, App};
use badge_service::handlers;
use shared_lib::{AppConfig, Database};

#[actix_web::test]
async fn test_health_check() {
    // Load configuration
    dotenvy::from_filename(".env").ok();

    let app = test::init_service(App::new().route(
        "/api/v1/health",
        web::get().to(handlers::health::health_check),
    ))
    .await;

    let req = test::TestRequest::get().uri("/api/v1/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_list_badges() {
    dotenvy::from_filename(".env").ok();

    // We need a database connection for this test
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
            .app_data(web::Data::new(database))
            .service(web::scope("/api/v1").configure(handlers::badge::configure)),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/v1/badges").to_request();
    let resp = test::call_service(&app, req).await;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = test::read_body(resp).await;
        println!("Response status: {}, body: {:?}", status, body);
        panic!("Request failed");
    }
}
