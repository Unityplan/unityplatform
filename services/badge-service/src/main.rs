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
        badge_service::handlers::health::health_check,
        badge_service::handlers::health::ready_check,
        badge_service::handlers::health::metrics,
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
        (name = "health", description = "Service health and monitoring"),
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
                    .route("/health", web::get().to(badge_service::handlers::health::health_check))
                    .route("/ready", web::get().to(badge_service::handlers::health::ready_check))
                    .route("/metrics", web::get().to(badge_service::handlers::health::metrics))
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

