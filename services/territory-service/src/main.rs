use actix_web::{web, App, HttpResponse, HttpServer};
use shared_lib::{
    cors, shutdown_grace_period, shutdown_signal, AppConfig, Database, LoggingMiddleware,
    MetricsCollector, RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
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
        config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    tracing::info!("✅ Database connected");

    // Initialize Redis client for rate limiting (use AppConfig later when Redis config is added)
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    tracing::info!("✅ Redis connected");

    // Initialize NATS client using AppConfig
    let nats_client =
        shared_lib::NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
            .await
            .expect("Failed to initialize NATS client");
    tracing::info!("✅ NATS connected");

    // Register Territory Manager badge with badge-service
    register_territory_manager_badge(&config).await;

    // Initialize metrics collector
    let metrics_collector = MetricsCollector::new("territory_service", env!("CARGO_PKG_VERSION"));

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
            // Priority 1 middleware - Request tracking and logging with metrics
            .wrap(LoggingMiddleware::development_with_metrics(
                metrics_collector.clone(),
            ))
            .wrap(RequestIdMiddleware)
            // Priority 2 middleware - Security and rate limiting
            .wrap(SecurityHeadersMiddleware::development())
            .wrap(cors::development())
            .wrap(RateLimitMiddleware::development(redis_client.clone()))
            // Shared state
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(nats_client.clone()))
            .app_data(web::Data::new(metrics_collector.clone()))
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // API routes
            .service(
                web::scope("/api/v1")
                    // Health endpoints
                    .route("/health", web::get().to(health_check))
                    .route("/ready", web::get().to(ready_check))
                    .route("/metrics", web::get().to(metrics))
                    // Territory routes
                    .service(
                        web::scope("/territories")
                            .configure(territory_service::handlers::territory::configure),
                    ),
            )
    })
    .bind(&server_addr)?
    .run();

    tracing::info!("✅ HTTP server configured");

    // Get server handle for graceful shutdown
    let server_handle = server.handle();

    // Spawn server task
    let server_task = tokio::spawn(server);

    // Wait for shutdown signal (Ctrl+C or SIGTERM)
    shutdown_signal().await;

    // Get grace period from environment (default 30s production, 10s dev)
    let grace_period = shutdown_grace_period();

    tracing::info!(
        "⏳ Starting graceful shutdown ({}s grace period)",
        grace_period
    );
    tracing::info!("🔄 Finishing in-flight requests...");

    // Stop accepting new requests but finish existing ones
    server_handle.stop(true).await;

    // Wait for server to finish with timeout
    tokio::select! {
        _ = server_task => {
            tracing::info!("✅ Server shutdown complete");
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(grace_period)) => {
            tracing::warn!("⚠️  Shutdown timeout reached, forcing exit");
        }
    }

    // Cleanup resources
    tracing::info!("🧹 Cleaning up resources...");
    tracing::info!("👋 Territory service stopped gracefully");
    Ok(())
}

/// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "territory-service",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Ready check endpoint
async fn ready_check(db: web::Data<Database>) -> HttpResponse {
    // Check database connectivity
    match sqlx::query("SELECT 1").fetch_one(db.pool()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ready",
            "service": "territory-service",
            "version": env!("CARGO_PKG_VERSION"),
        })),
        Err(_) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "service": "territory-service",
            "version": env!("CARGO_PKG_VERSION"),
            "reason": "database_unavailable"
        })),
    }
}

/// Metrics endpoint (Prometheus format)
async fn metrics(db: web::Data<Database>, collector: web::Data<MetricsCollector>) -> HttpResponse {
    // Get database pool stats
    let pool_size = db.pool().size();
    let pool_idle = db.pool().num_idle();

    let metrics_text = collector.generate_prometheus_metrics(Some(pool_size), Some(pool_idle));

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_text)
}

/// Register Territory Manager badge with badge-service on startup
async fn register_territory_manager_badge(config: &AppConfig) {
    use serde_json::json;

    let badge_service_url = std::env::var("BADGE_SERVICE_URL")
        .unwrap_or_else(|_| format!("http://{}:8007", config.server.host));

    let badge_payload = json!({
        "slug": "territory-manager",
        "name": "Territory Manager",
        "description": "Grants full management access to territory administration and settings",
        "icon": "🌍",
        "category": "role",
        "criteriaType": "manual",
        "criteriaValue": null,
        "rarity": "epic",
        "isRenewable": false,
        "renewalDays": null,
        "grantsPermissions": [
            "territory:manage",
            "territory:settings:manage",
            "territory:users:manage",
            "territory:federation:manage"
        ]
    });

    let client = reqwest::Client::new();
    match client
        .post(format!("{}/api/v1/badges/register", badge_service_url))
        .json(&badge_payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                tracing::info!("✅ Territory Manager badge registered with badge-service");
            } else {
                tracing::warn!(
                    "⚠️  Failed to register Territory Manager badge: HTTP {}",
                    response.status()
                );
            }
        }
        Err(e) => {
            tracing::warn!("⚠️  Could not reach badge-service to register badge: {}", e);
            tracing::info!("   (This is expected if badge-service is not running)");
        }
    }
}
