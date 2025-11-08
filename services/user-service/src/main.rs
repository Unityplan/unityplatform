mod handlers;
mod models;
mod services;

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
        "postgresql://unityplan:unityplan_dev_password@localhost:5432/unityplan_dk".to_string()
    });

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8084".to_string());
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

    // Start HTTP server
    let bind_address = format!("{}:{}", host, port);
    log::info!("🚀 User Service listening on http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            // Add services to app data
            .app_data(user_service.clone())
            .app_data(storage_service.clone())
            // Middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Health check
            .route("/health", web::get().to(health_check))
            // API routes
            .service(
                web::scope("/api")
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
