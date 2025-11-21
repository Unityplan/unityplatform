use actix_web::{web, App, HttpServer};
use shared_lib::{
    cors, shutdown_grace_period, AppConfig, Database, LoggingMiddleware, MetricsCollector,
    NatsClient, RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
};
use std::sync::Arc;
use utility_service::{handlers, models, services};
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

    tracing::info!("🚀 Starting Utility Service v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");
    tracing::info!("✅ Configuration loaded");

    // Initialize database connection
    let _database = Database::new(
        config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to initialize database");
    tracing::info!("✅ Database connected");

    // Initialize Redis client
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client =
        redis::Client::open(redis_url.clone()).expect("Failed to create Redis client");
    tracing::info!("✅ Redis connected");

    // Initialize NATS client
    let _nats_client = NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to initialize NATS client");
    tracing::info!("✅ NATS connected");

    // Initialize services
    let favicon_service = Arc::new(services::FaviconService::new(redis_client.clone()));
    tracing::info!("🎨 Favicon service initialized");

    // Initialize metrics collector
    let metrics_collector = MetricsCollector::new("utility_service", env!("CARGO_PKG_VERSION"));

    // Create shared state
    let favicon_service_data = web::Data::new(favicon_service);

    // Generate OpenAPI documentation
    let openapi = ApiDoc::openapi();

    let server_addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("🌐 Server will listen on {}", server_addr);
    tracing::info!(
        "📚 Swagger UI available at http://{}/swagger-ui/",
        server_addr
    );

    let server = HttpServer::new(move || {
        App::new()
            // Priority 1: Logging and request tracking
            .wrap(LoggingMiddleware::development_with_metrics(
                metrics_collector.clone(),
            ))
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
    .bind((config.server.host.as_str(), config.server.port))?
    .workers(4)
    .shutdown_timeout(shutdown_grace_period());

    tracing::info!("✅ Server started successfully");

    server.run().await
}
