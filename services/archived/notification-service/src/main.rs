use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = notification_service::config::Config::from_env()
        .expect("Failed to load configuration");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    let allowed_origins = config.cors.allowed_origins.clone();

    tracing::info!("Starting Notification Service on {}:{}", server_host, server_port);

    HttpServer::new(move || {
        let mut cors = actix_cors::Cors::default();
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }
        let cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(notification_service::handlers::health))
            .service(notification_service::openapi::swagger_ui())
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
