use actix_web::{web, App, HttpServer};
use futures_util::StreamExt;
use shared_lib::{
    cors, shutdown_grace_period, shutdown_signal, AppConfig, Database, LoggingMiddleware,
    MetricsCollector, RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Task-scheduler Service API",
        version = "0.1.0-alpha.1",
        description = "Background task scheduler for cleanup jobs and cron operations",
        contact(
            name = "Unity Platform Team",
            email = "dev@unityplan.org"
        )
    ),
    paths(
        // Health endpoints
        health_check,
        ready_check,
        metrics,
        // Cleanup endpoints
        task_scheduler_service_service::handlers::trigger_cleanup,
        task_scheduler_service_service::handlers::get_cleanup_stats,
    ),
    components(
        schemas(
            task_scheduler_service_service::models::TriggerCleanupRequest,
            task_scheduler_service_service::models::ManualCleanupResponse,
            task_scheduler_service_service::models::UserCleanupStats,
        )
    ),
    tags(
        (name = "health", description = "Service health and monitoring"),
        (name = "cleanup", description = "User cleanup and GDPR deletion jobs"),
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

    tracing::info!("🚀 Starting Task-scheduler Service v{}", env!("CARGO_PKG_VERSION"));

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

    // Initialize Redis client for rate limiting
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

    // Initialize cleanup service
    let cleanup_service = task_scheduler_service_service::services::CleanupService::new(database.pool().clone());
    tracing::info!("✅ Cleanup service initialized");

    // Start cron scheduler for automated cleanup jobs
    let cleanup_service_arc = std::sync::Arc::new(cleanup_service.clone());
    let _scheduler = task_scheduler_service_service::services::start_scheduler(cleanup_service_arc)
        .await
        .expect("Failed to start scheduler");
    tracing::info!("✅ Cron scheduler started");

    // Subscribe to NATS user.deleted events (currently just logs, services handle their own soft-delete)
    let nats_clone = nats_client.clone();
    let pool_clone = database.pool().clone();
    tokio::spawn(async move {
        if let Ok(mut subscription) = nats_clone.subscribe("user.deleted").await {
            tracing::info!("✅ Subscribed to NATS user.deleted events");
            
            loop {
                match subscription.next().await {
                    Some(msg) => {
                        match serde_json::from_slice::<shared_lib::UserDeletedEvent>(&msg.payload) {
                            Ok(event) => {
                                tracing::info!(
                                    user_id = %event.user_id,
                                    territory = %event.territory,
                                    deleted_at = %event.deleted_at,
                                    "📨 Received user.deleted event (hard deletion will occur in 30 days)"
                                );
                                // Note: Services handle their own soft-delete logic
                                // This service only performs hard deletion via cron job
                            }
                            Err(e) => {
                                tracing::error!(
                                    error = %e,
                                    "Failed to deserialize user.deleted event"
                                );
                            }
                        }
                    }
                    None => {
                        tracing::warn!("NATS subscription ended, reconnecting...");
                        break;
                    }
                }
            }
        } else {
            tracing::error!("Failed to subscribe to user.deleted events");
        }
    });

    // Initialize metrics collector
    let metrics_collector =
        shared_lib::MetricsCollector::new("task_scheduler_service_service", env!("CARGO_PKG_VERSION"));

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
            .app_data(web::Data::new(cleanup_service.clone()))
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
                    // Cleanup routes
                    .configure(task_scheduler_service_service::handlers::configure)
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

    tracing::info!("👋 Task-scheduler service stopped gracefully");
    Ok(())
}

/// Health check endpoint (no authentication required)
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = serde_json::Value,
            example = json!({
                "status": "ok",
                "service": "task-scheduler-service",
                "version": "0.1.0-alpha.1"
            })
        )
    )
)]
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "task-scheduler-service",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Ready check endpoint (database connectivity)
#[utoipa::path(
    get,
    path = "/api/v1/ready",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready", body = serde_json::Value,
            example = json!({
                "status": "ready",
                "service": "task-scheduler-service",
                "version": "0.1.0-alpha.1"
            })
        ),
        (status = 503, description = "Service not ready", body = serde_json::Value,
            example = json!({
                "status": "not_ready",
                "service": "task-scheduler-service",
                "version": "0.1.0-alpha.1",
                "reason": "database_unavailable"
            })
        )
    )
)]
async fn ready_check(db: web::Data<Database>) -> actix_web::HttpResponse {
    // Check database connectivity
    match sqlx::query("SELECT 1").fetch_one(db.pool()).await {
        Ok(_) => actix_web::HttpResponse::Ok().json(serde_json::json!({
            "status": "ready",
            "service": "task-scheduler-service",
            "version": env!("CARGO_PKG_VERSION"),
        })),
        Err(_) => actix_web::HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "service": "task-scheduler-service",
            "version": env!("CARGO_PKG_VERSION"),
            "reason": "database_unavailable"
        })),
    }
}

/// Metrics endpoint (Prometheus format)
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    tag = "health",
    responses(
        (status = 200, description = "Prometheus metrics", 
            content_type = "text/plain",
            body = String,
            example = "# HELP task_scheduler_service_service_http_requests_total Total HTTP requests"
        )
    )
)]
async fn metrics(
    db: web::Data<Database>,
    collector: web::Data<MetricsCollector>,
) -> actix_web::HttpResponse {
    // Get database pool stats
    let pool_size = db.pool().size();
    let pool_idle = db.pool().num_idle();

    let metrics_text = collector.generate_prometheus_metrics(Some(pool_size), Some(pool_idle));

    actix_web::HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_text)
}
