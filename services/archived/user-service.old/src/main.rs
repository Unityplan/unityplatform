mod handlers;
mod models;
mod services;

use actix_cors::Cors;
use actix_web::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::services::{StorageService, UserService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration from environment
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://unityplatform:unityplatform_dev_password@localhost:5432/unityplatform_dk".to_string()
    });

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8002".to_string());
    let avatars_path = env::var("AVATARS_PATH").unwrap_or_else(|_| "./uploads/avatars".to_string());

    log::info!("Starting User Service...");
    log::info!("Database URL: {}", database_url);
    log::info!("Avatars storage: {}", avatars_path);

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    log::info!("✅ Database connection established");

    // Note: Migrations are managed in shared-lib
    // All migrations should be run via shared-lib/migrations

    // Create services
    let user_service = web::Data::new(UserService::new(pool));
    let storage_service = web::Data::new(StorageService::new(avatars_path));

    // Create avatars directory if it doesn't exist
    std::fs::create_dir_all("./uploads/avatars").expect("Failed to create avatars directory");

    log::info!("✅ Services initialized");

    // Parse CORS allowed origins from environment
    let cors_origins: Vec<String> = env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Start HTTP server
    let bind_address = format!("{}:{}", host, port);
    log::info!("🚀 User Service listening on http://{}", bind_address);

    HttpServer::new(move || {
        // Configure CORS with origins from environment
        let mut cors = Cors::default();
        for origin in &cors_origins {
            cors = cors.allowed_origin(origin);
        }
        let cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"])
            .allowed_headers(vec![AUTHORIZATION, ACCEPT, CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        App::new()
            // Add services to app data
            .app_data(user_service.clone())
            .app_data(storage_service.clone())
            // Middleware
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Health check
            .route("/health", web::get().to(health_check))
            // API routes
            .service(
                web::scope("/api/v1")
                    .configure(handlers::profile::configure)
                    .configure(handlers::avatar::configure)
                    .configure(handlers::connections::configure),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}

/// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "user-service",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
