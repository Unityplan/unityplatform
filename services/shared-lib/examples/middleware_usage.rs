/// # Middleware Usage Example
///
/// This example demonstrates how to use all Priority 1 and Priority 2 middleware
/// in an actix-web service, with both Phase 1 (development) and Phase 2 (production)
/// configurations.
///
/// Run with: cargo run --example middleware_usage
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use shared_lib::{
    cors, LoggingMiddleware, RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
    ValidatedJson, ValidatedQuery,
};
use validator::Validate;

/// Example request body with validation
#[derive(Debug, Deserialize, Validate, Serialize)]
struct CreateUserRequest {
    #[validate(length(min = 3, max = 30))]
    username: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 8, max = 100))]
    password: String,

    #[validate(range(min = 18, max = 120))]
    age: Option<u32>,
}

/// Example query parameters with validation
#[derive(Debug, Deserialize, Validate)]
struct SearchQuery {
    #[validate(length(min = 1, max = 100))]
    q: String,

    #[validate(range(min = 1, max = 100))]
    limit: Option<u32>,

    #[validate(range(min = 0))]
    offset: Option<u32>,
}

/// Example handler using ValidatedJson
async fn create_user(user: ValidatedJson<CreateUserRequest>) -> HttpResponse {
    // user is guaranteed to be valid at this point
    HttpResponse::Ok().json(serde_json::json!({
        "message": "User created successfully",
        "username": user.username,
        "email": user.email,
    }))
}

/// Example handler using ValidatedQuery
async fn search_users(query: ValidatedQuery<SearchQuery>) -> HttpResponse {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    HttpResponse::Ok().json(serde_json::json!({
        "query": &query.q,
        "limit": limit,
        "offset": offset,
        "results": [],
    }))
}

/// Simple health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "example-service",
    }))
}

/// Development Configuration
///
/// - Verbose logging (DEBUG level)
/// - Detailed error messages with stack traces
/// - Permissive CORS (all origins)
/// - Lenient rate limiting (100 req/min)
/// - Basic security headers
async fn development_server() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_target(true)
        .init();

    tracing::info!("Starting Development server on http://127.0.0.1:8080");

    let redis_client =
        redis::Client::open("redis://127.0.0.1:6379/").expect("Failed to connect to Redis");

    HttpServer::new(move || {
        App::new()
            // Priority 1 middleware (in order)
            .wrap(LoggingMiddleware::development()) // Verbose logging
            .wrap(RequestIdMiddleware) // Request ID tracking
            // Priority 2 middleware (in order)
            .wrap(SecurityHeadersMiddleware::development()) // Basic headers
            .wrap(cors::development()) // Permissive CORS
            .wrap(RateLimitMiddleware::development(redis_client.clone())) // 100 req/min
            // Routes
            .route("/health", web::get().to(health_check))
            .route("/users", web::post().to(create_user))
            .route("/users/search", web::get().to(search_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Production Configuration
///
/// - Minimal logging (INFO/WARN/ERROR only)
/// - Generic error messages (security)
/// - Strict CORS whitelist
/// - Strict rate limiting (30 req/min)
/// - Comprehensive security headers (CSP, HSTS, etc.)
async fn production_server() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .json()
        .init();

    tracing::info!("Starting Production server on http://127.0.0.1:8080");

    let redis_client =
        redis::Client::open("redis://127.0.0.1:6379/").expect("Failed to connect to Redis");

    // Production allowed origins
    let allowed_origins = vec![
        "https://app.unityplatform.dk".to_string(),
        "https://api.unityplatform.dk".to_string(),
    ];

    HttpServer::new(move || {
        App::new()
            // Priority 1 middleware (in order)
            .wrap(LoggingMiddleware::production()) // Minimal logging
            .wrap(RequestIdMiddleware) // Request ID tracking
            // Priority 2 middleware (in order)
            .wrap(SecurityHeadersMiddleware::production()) // Strict headers + CSP + HSTS
            .wrap(cors::production(allowed_origins.clone())) // Strict whitelist
            .wrap(RateLimitMiddleware::production(redis_client.clone())) // 30 req/min
            // Routes
            .route("/health", web::get().to(health_check))
            .route("/users", web::post().to(create_user))
            .route("/users/search", web::get().to(search_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Use 'development' or 'production' mode
    let mode = std::env::var("MODE").unwrap_or_else(|_| "development".to_string());

    match mode.as_str() {
        "development" | "dev" => development_server().await,
        "production" | "prod" => production_server().await,
        _ => {
            eprintln!("Invalid MODE value. Use MODE=development or MODE=production");
            std::process::exit(1);
        }
    }
}

// Test requests you can make:
//
// Run in development mode:
//   MODE=development cargo run --example middleware_usage
//
// Run in production mode:
//   MODE=production cargo run --example middleware_usage
//
// 1. Health check:
//    curl http://127.0.0.1:8080/health
//
// 2. Create user (valid):
//    curl -X POST http://127.0.0.1:8080/users \
//      -H "Content-Type: application/json" \
//      -d '{"username":"johndoe","email":"john@example.com","password":"secretpass123","age":25}'
//
// 3. Create user (invalid - short username):
//    curl -X POST http://127.0.0.1:8080/users \
//      -H "Content-Type: application/json" \
//      -d '{"username":"ab","email":"john@example.com","password":"secretpass123"}'
//
// 4. Create user (invalid - bad email):
//    curl -X POST http://127.0.0.1:8080/users \
//      -H "Content-Type: application/json" \
//      -d '{"username":"johndoe","email":"not-an-email","password":"secretpass123"}'
//
// 5. Search users (valid):
//    curl "http://127.0.0.1:8080/users/search?q=john&limit=20&offset=0"
//
// 6. Search users (invalid - limit too high):
//    curl "http://127.0.0.1:8080/users/search?q=john&limit=200"
//
// 7. Test rate limiting (make 101+ requests in development, or 31+ in production):
//    for i in {1..101}; do curl http://127.0.0.1:8080/health; done
//
// 8. Test CORS (from browser console):
//    fetch('http://127.0.0.1:8080/health', {
//      headers: { 'Origin': 'http://localhost:3000' }
//    })
