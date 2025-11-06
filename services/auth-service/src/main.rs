mod handlers;
mod models;
mod services;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use services::TokenService;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
struct Config {
    database_url: String,
    jwt_secret: String,
    access_token_ttl: i64,  // seconds (default: 15 minutes)
    refresh_token_ttl: i64, // seconds (default: 7 days)
    server_host: String,
    server_port: u16,
}

impl Config {
    fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://unityplan:unityplan_dev_password_dk@localhost:5432/unityplan_dk"
                    .to_string()
            }),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev_secret_change_in_production".to_string()),
            access_token_ttl: std::env::var("ACCESS_TOKEN_TTL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(900), // 15 minutes
            refresh_token_ttl: std::env::var("REFRESH_TOKEN_TTL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(604800), // 7 days
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8001),
        })
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,auth_service=debug,sqlx=warn")),
        )
        .init();

    tracing::info!("Starting auth-service v{}", shared_lib::version::VERSION);
    tracing::info!(
        "Build: {} ({})",
        shared_lib::version::GIT_HASH,
        shared_lib::version::BUILD_TIMESTAMP
    );

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Create database connection pool
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;
    tracing::info!("Database connection established");

    // Test database connection
    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Database health check passed");

    // Create token service
    let token_service = Arc::new(TokenService::new(
        &config.jwt_secret,
        config.access_token_ttl,
        config.refresh_token_ttl,
    ));
    tracing::info!("Token service initialized");

    let bind_addr = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("Starting HTTP server on {}", bind_addr);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(token_service.clone()))
            .service(
                web::scope("/api/auth")
                    .route("/register", web::post().to(handlers::register))
                    .route("/login", web::post().to(handlers::login)),
            )
            .route("/health", web::get().to(handlers::health))
    })
    .bind(&bind_addr)?
    .run()
    .await?;

    Ok(())
}
