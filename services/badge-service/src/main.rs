use actix_web::{web, App, HttpServer};
use badge_service::models::{
    AwardBadgeRequest, BadgeResponse, RevokeBadgeRequest, ToggleFeaturedRequest,
    UpdateProgressRequest, UserBadgeResponse,
};
use shared_lib::{
    cors, shutdown_grace_period, shutdown_signal, AppConfig, Database, LoggingMiddleware,
    MetricsCollector, NatsClient, RateLimitMiddleware, RequestIdMiddleware,
    SecurityHeadersMiddleware,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Badge Service API",
        version = "0.1.0-alpha.1",
        description = "Badge and achievement management for Unity Platform",
        contact(
            name = "Unity Platform Team",
            email = "dev@unityplan.org"
        )
    ),
    paths(
        health_check,
        ready_check,
        metrics,
        badge_service::handlers::badge::list_badges,
        badge_service::handlers::badge::get_user_badges,
        badge_service::handlers::badge::award_badge,
        badge_service::handlers::badge::revoke_badge,
        badge_service::handlers::badge::update_progress,
        badge_service::handlers::badge::toggle_featured,
    ),
    components(
        schemas(
            BadgeResponse,
            UserBadgeResponse,
            AwardBadgeRequest,
            RevokeBadgeRequest,
            UpdateProgressRequest,
            ToggleFeaturedRequest,
        )
    ),
    tags(
        (name = "service", description = "Service health and metadata"),
        (name = "badges", description = "Badge and achievement management")
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

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "badge_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Starting Badge Service v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");
    tracing::info!("✅ Configuration loaded");

    // Initialize database connection
    let database = Database::new(
        &config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    tracing::info!("✅ Database connected");

    // Initialize Redis client for rate limiting
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    tracing::info!("✅ Redis connected");

    // Initialize NATS client
    let nats_client = NatsClient::new(&config.nats_url(), config.nats.cluster_name.clone())
        .await
        .expect("Failed to connect to NATS");
    tracing::info!("✅ NATS connected");

    // Initialize NATS subscriptions
    badge_service::nats_handlers::initialize_subscriptions(nats_client.clone(), database.clone())
        .await;
    tracing::info!("✅ NATS subscriptions initialized");

    // Initialize metrics collector
    let metrics_collector = MetricsCollector::new("badge_service", env!("CARGO_PKG_VERSION"));
    tracing::info!("✅ Metrics collector initialized");

    let server_addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("🌐 Server will listen on {}", server_addr);
    tracing::info!(
        "📚 Swagger UI available at http://{}/swagger-ui/",
        server_addr
    );

    // Log development mode settings
    let dev_auto_grant =
        std::env::var("DEV_AUTO_GRANT_CODE_OF_CONDUCT").unwrap_or_else(|_| "false".to_string());
    if dev_auto_grant == "true" {
        tracing::warn!("⚠️  DEV MODE: Auto-granting Code of Conduct badge to all registered users");
    }

    // Generate OpenAPI documentation
    let openapi = ApiDoc::openapi();

    // Create HTTP server
    let server = HttpServer::new(move || {
        App::new()
            // Priority 1 middleware with metrics
            .wrap(LoggingMiddleware::development_with_metrics(
                metrics_collector.clone(),
            ))
            .wrap(RequestIdMiddleware)
            // Priority 2 middleware
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
                    .route("/health", web::get().to(health_check))
                    .route("/ready", web::get().to(ready_check))
                    .route("/metrics", web::get().to(metrics))
                    // Badge routes
                    .configure(badge_service::handlers::badge::configure),
            )
    })
    .bind(&server_addr)?
    .run();

    // Get server handle for graceful shutdown
    let server_handle = server.handle();
    tracing::info!("✅ Server started successfully");

    // Spawn server in background
    tokio::spawn(server);

    // Wait for shutdown signal
    shutdown_signal().await;

    // Graceful shutdown
    let grace_period = shutdown_grace_period();
    tracing::info!(
        "⏳ Starting graceful shutdown ({}s grace period)",
        grace_period
    );

    server_handle.stop(true).await;

    tracing::info!("🔄 Finishing in-flight requests...");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    tracing::info!("✅ Server shutdown complete");
    tracing::info!("🧹 Cleaning up resources...");

    // Cleanup would happen here (database connections auto-close via Drop)

    tracing::info!("👋 Badge service stopped gracefully");
    Ok(())
}

/// Health check endpoint (no authentication required)
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "service",
    responses(
        (status = 200, description = "Service is healthy", body = serde_json::Value,
            example = json!({
                "service": "badge-service",
                "status": "healthy",
                "version": "0.1.0-alpha.1"
            })
        )
    )
)]
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "service": "badge-service",
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Readiness check endpoint - verifies database connectivity
#[utoipa::path(
    get,
    path = "/api/v1/ready",
    tag = "service",
    responses(
        (status = 200, description = "Service is ready", body = serde_json::Value,
            example = json!({
                "service": "badge-service",
                "status": "ready",
                "version": "0.1.0-alpha.1"
            })
        ),
        (status = 503, description = "Service is not ready", body = serde_json::Value,
            example = json!({
                "service": "badge-service",
                "status": "not_ready",
                "reason": "database_unavailable",
                "version": "0.1.0-alpha.1"
            })
        )
    )
)]
async fn ready_check(db: web::Data<Database>) -> actix_web::HttpResponse {
    // Check database connectivity
    let db_ok = sqlx::query("SELECT 1").fetch_one(db.pool()).await.is_ok();

    if db_ok {
        actix_web::HttpResponse::Ok().json(serde_json::json!({
            "service": "badge-service",
            "status": "ready",
            "version": env!("CARGO_PKG_VERSION"),
        }))
    } else {
        actix_web::HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "service": "badge-service",
            "status": "not_ready",
            "reason": "database_unavailable",
            "version": env!("CARGO_PKG_VERSION"),
        }))
    }
}

/// Metrics endpoint - Prometheus-compatible metrics
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    tag = "service",
    responses(
        (status = 200, description = "Prometheus-format metrics", content_type = "text/plain")
    )
)]
async fn metrics(
    db: web::Data<Database>,
    collector: web::Data<MetricsCollector>,
) -> actix_web::HttpResponse {
    // Get database pool metrics
    let pool_size = db.pool().size();
    let pool_idle = db.pool().num_idle();

    // Generate comprehensive metrics using the collector
    let metrics_text = collector.generate_prometheus_metrics(Some(pool_size), Some(pool_idle));

    actix_web::HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_text)
}
