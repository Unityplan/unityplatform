use actix_web::{web, App, HttpServer};
use auth_service::handlers;
use auth_service::models::{
    AuthResponse, HealthResponse, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest,
    ValidateResponse,
};
use auth_service::services::TokenService;
use dotenvy::dotenv;
use shared_lib::{
    cors, shutdown_grace_period, shutdown_signal, AppConfig, Database, LoggingMiddleware,
    MetricsCollector, RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::register,
        handlers::login,
        handlers::refresh,
        handlers::logout,
        handlers::validate,
        handlers::health,
        handlers::ready,
        handlers::metrics,
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            RefreshRequest,
            LogoutRequest,
            AuthResponse,
            ValidateResponse,
            HealthResponse,
        )
    ),
    tags(
        (name = "Authentication", description = "User authentication endpoints"),
        (name = "Health", description = "Service health and metrics"),
    ),
    info(
        title = "Unity Platform Auth Service",
        version = "0.1.0-alpha.1",
        description = "Authentication service for Unity Platform. Handles user registration, login, token refresh, and JWT validation.",
    ),
    servers(
        (url = "http://localhost:8001", description = "Local development server")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "auth_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    // Initialize database
    let database = Database::new(
        config.database_url(),
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to initialize database");

    // Initialize Redis for rate limiting (use AppConfig later when Redis config is added)
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");

    // Initialize token service (use AppConfig auth settings)
    let token_service = TokenService::new(&config.auth.jwt_secret);

    // Get service port from config
    let bind_address = format!("{}:{}", config.server.host, config.server.port);

    tracing::info!("🚀 Starting auth-service on {}", bind_address);
    tracing::info!(
        "📚 Swagger UI available at http://{}/swagger-ui/",
        bind_address
    );
    tracing::info!(
        "📝 Invitation validation: {}",
        env::var("ENABLE_INVITATION_VALIDATION").unwrap_or_else(|_| "false".to_string())
    );

    let openapi = ApiDoc::openapi();

    // Initialize metrics collector
    let metrics_collector = MetricsCollector::new("auth_service", env!("CARGO_PKG_VERSION"));

    // Initialize NATS client using AppConfig
    let nats_client =
        shared_lib::NatsClient::new(config.nats_url(), config.nats.cluster_name.clone())
            .await
            .expect("Failed to initialize NATS client");

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
            .app_data(web::Data::new(token_service.clone()))
            .app_data(web::Data::new(metrics_collector.clone()))
            .app_data(web::Data::new(nats_client.clone()))
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // Routes
            .service(
                web::scope("/api/v1")
                    // Health endpoints
                    .service(handlers::health)
                    .service(handlers::ready)
                    .service(handlers::metrics)
                    // Auth endpoints
                    .service(
                        web::scope("/auth")
                            .service(handlers::register)
                            .service(handlers::login)
                            .service(handlers::refresh)
                            .service(handlers::logout)
                            .service(handlers::validate),
                    ),
            )
    })
    .bind(&bind_address)?
    .run();

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
    // Database connections are automatically closed when dropped
    // Redis connections are automatically closed when dropped

    tracing::info!("👋 Auth service stopped gracefully");

    Ok(())
}
