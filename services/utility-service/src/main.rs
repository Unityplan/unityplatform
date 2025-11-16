mod handlers;
mod models;
mod services;

use actix_web::{web, App, HttpServer};
use shared_lib::{
    cors, LoggingMiddleware, RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Utility Service API",
        version = "0.1.0-alpha.1",
        description = "Infrastructure utility service for favicon fetching, QR codes, and other utilities for Unity Platform",
        contact(
            name = "Unity Platform Team",
            email = "dev@unityplan.org"
        )
    ),
    paths(
        handlers::get_favicon,
        handlers::health,
        handlers::ready,
        handlers::metrics,
    ),
    components(
        schemas(
            models::FaviconQuery,
            models::HealthResponse,
            models::ReadyResponse,
            models::RedisHealth,
        )
    ),
    tags(
        (name = "Utilities", description = "Utility functions (favicon fetching, etc.)"),
        (name = "Health", description = "Service health and monitoring endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

use utoipa::Modify;
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("🚀 Starting utility-service v{}", env!("CARGO_PKG_VERSION"));

    // Service configuration
    let service_host = std::env::var("SERVICE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let service_port = std::env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "8014".to_string())
        .parse::<u16>()
        .expect("Invalid SERVICE_PORT");

    tracing::info!(
        "📡 Service will listen on {}:{}",
        service_host,
        service_port
    );

    // Initialize Redis client
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client =
        redis::Client::open(redis_url.clone()).expect("Failed to create Redis client");

    tracing::info!("🔄 Redis client initialized");

    // Test Redis connection
    {
        let mut conn = redis_client
            .get_multiplexed_async_connection()
            .await
            .expect("Failed to connect to Redis");

        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .expect("Redis PING failed");

        tracing::info!("✅ Redis connection verified");
    }

    // Initialize services
    let favicon_service = Arc::new(services::FaviconService::new(redis_client.clone()));
    tracing::info!("🎨 Favicon service initialized");

    // Create shared state
    let favicon_service_data = web::Data::new(favicon_service);

    tracing::info!("📝 Registering routes and middleware...");

    // Generate OpenAPI documentation
    let openapi = ApiDoc::openapi();

    let server_addr = format!("{}:{}", service_host, service_port);
    tracing::info!("🌐 Server will listen on {}", server_addr);
    tracing::info!(
        "📚 Swagger UI available at http://{}/swagger-ui/",
        server_addr
    );

    let server = HttpServer::new(move || {
        App::new()
            // Priority 1: Logging and request tracking
            .wrap(LoggingMiddleware::development())
            .wrap(RequestIdMiddleware)
            // Priority 2: Security and access control
            .wrap(SecurityHeadersMiddleware::development())
            .wrap(cors::development())
            .wrap(RateLimitMiddleware::development(redis_client.clone()))
            // Shared state
            .app_data(favicon_service_data.clone())
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // Health endpoints (public)
            .route("/api/v1/health", web::get().to(handlers::health))
            .route("/api/v1/ready", web::get().to(handlers::ready))
            .route("/api/v1/metrics", web::get().to(handlers::metrics))
            // Utility endpoints (authenticated)
            .service(
                web::scope("/api/v1/utility")
                    .route("/favicon", web::get().to(handlers::get_favicon)),
            )
    })
    .bind((service_host.as_str(), service_port))?
    .workers(4)
    .shutdown_timeout(30);

    tracing::info!(
        "✅ utility-service is ready and listening on port {}",
        service_port
    );

    server.run().await
}
