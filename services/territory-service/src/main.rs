use actix_web::{web, App, HttpResponse, HttpServer};
use shared_lib::{
    cors, shutdown_grace_period, shutdown_signal, AppConfig, Database, LoggingMiddleware,
    RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
};
use territory_service::models::territory::{
    TerritoryResponse, TerritorySettingsResponse, TerritoryStatsResponse, UpdateSettingsRequest,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Territory Service API",
        version = "0.1.0-alpha.1",
        description = "Territory registry, settings management, and federation for Unity Platform",
        contact(
            name = "Unity Platform Team",
            email = "dev@unityplan.org"
        )
    ),
    paths(
        health_check,
        territory_service::handlers::territory::list_territories,
        territory_service::handlers::territory::get_territory,
        territory_service::handlers::territory::get_territory_stats,
        territory_service::handlers::territory::update_territory_settings,
    ),
    components(
        schemas(
            TerritoryResponse,
            TerritoryStatsResponse,
            TerritorySettingsResponse,
            UpdateSettingsRequest,
        )
    ),
    tags(
        (name = "service", description = "Service health and metadata"),
        (name = "territories", description = "Territory registry and public information"),
        (name = "territory-management", description = "Territory management endpoints (Territory Managers)"),
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

    tracing::info!(
        "🚀 Starting Territory Service v{}",
        env!("CARGO_PKG_VERSION")
    );

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");
    tracing::info!("✅ Configuration loaded");

    // Initialize database connection
    let database = Database::new(
        &config.database.url,
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    tracing::info!("✅ Database connected");

    // Initialize Redis client for rate limiting
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis:6379".to_string());
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    tracing::info!("✅ Redis connected");

    // NATS client for event publishing (optional for Phase 1)
    let _nats_client: Option<shared_lib::NatsClient> = None;

    let server_addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("🌐 Server will listen on {}", server_addr);
    tracing::info!(
        "📚 Swagger UI available at http://{}/swagger-ui/",
        server_addr
    );

    // Generate OpenAPI documentation
    let openapi = ApiDoc::openapi();

    // Create HTTP server
    let server = HttpServer::new(move || {
        App::new()
            // Priority 1 middleware
            .wrap(LoggingMiddleware::development())
            .wrap(RequestIdMiddleware)
            // Priority 2 middleware
            .wrap(SecurityHeadersMiddleware::development())
            .wrap(cors::development())
            .wrap(RateLimitMiddleware::development(redis_client.clone()))
            // Shared state
            .app_data(web::Data::new(database.clone()))
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // API routes
            .service(
                web::scope("/api/v1")
                    // Service routes
                    .service(web::scope("/service").route("/health", web::get().to(health_check)))
                    // Territory routes
                    .service(
                        web::scope("/territories")
                            .configure(territory_service::handlers::territory::configure),
                    ),
            )
    })
    .bind(&server_addr)?;

    tracing::info!("✅ HTTP server configured");

    let server_handle = server.run();
    tracing::info!("🎉 Territory service is running!");

    // Graceful shutdown handling
    let shutdown_handle = server_handle.handle();
    tokio::spawn(async move {
        shutdown_signal().await;
        tracing::info!("🛑 Shutdown signal received, starting graceful shutdown...");
        shutdown_handle.stop(true).await;
    });

    // Run server
    server_handle.await?;

    // Wait for graceful shutdown
    tokio::time::sleep(std::time::Duration::from_secs(shutdown_grace_period())).await;

    tracing::info!("✅ Server shutdown complete");
    tracing::info!("👋 Territory service stopped gracefully");
    Ok(())
}

/// Health check endpoint (no authentication required)
#[utoipa::path(
    get,
    path = "/api/v1/service/health",
    tag = "service",
    responses(
        (status = 200, description = "Service is healthy", body = serde_json::Value,
            example = json!({
                "status": "healthy",
                "service": "territory-service",
                "version": "0.1.0-alpha.1"
            })
        )
    )
)]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "territory-service",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
