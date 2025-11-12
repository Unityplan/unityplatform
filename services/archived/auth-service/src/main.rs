use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = auth_service::config::Config::from_env().expect("Failed to load configuration");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    // Create token service
    let token_service = web::Data::new(auth_service::services::TokenService::new(
        &config.jwt.secret,
        config.jwt.access_ttl,
        config.jwt.refresh_ttl,
    ));

    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    let allowed_origins = config.cors.allowed_origins.clone();

    tracing::info!("Starting auth-service on {}:{}", server_host, server_port);

    // Start HTTP server
    HttpServer::new(move || {
        // Build CORS configuration
        let mut cors = actix_cors::Cors::default();
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }
        let cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
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
            .app_data(token_service.clone())
            .service(
                web::scope("/v1/auth")
                    .route(
                        "/register",
                        web::post().to(auth_service::handlers::register),
                    )
                    .route("/login", web::post().to(auth_service::handlers::login))
                    .route("/refresh", web::post().to(auth_service::handlers::refresh))
                    .route("/logout", web::post().to(auth_service::handlers::logout))
                    .route(
                        "/validate",
                        web::get().to(auth_service::handlers::validate_token),
                    ),
            )
            .route("/health", web::get().to(auth_service::handlers::health))
            .service(auth_service::openapi::swagger_ui())
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
