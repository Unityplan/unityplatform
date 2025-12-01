use actix_web::{web, App, HttpServer};
use shared_lib::{
    cors, shutdown_grace_period, shutdown_signal, AppConfig, Database, LoggingMiddleware,
    RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
};
use user_service::handlers::profile::UpdateProfileRequest;
use user_service::models::{
    ConnectionResponse, ConnectionStatus, ConnectionType, ConnectionsListResponse,
    CreateLanguageProficiencyRequest, CreateProfileLinkRequest, LanguageProficiencyResponse,
    NotificationSettingsResponse, PrivacySettingsResponse, ProfileLinkResponse, ProfileResponse,
    SettingsResponse, UpdateLanguageProficiencyRequest, UpdateNotificationSettingsRequest,
    UpdatePrivacySettingsRequest, UpdateProfileLinkRequest, UpdateSettingsRequest,
    UserSearchResponse, UserSearchResult,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    info(
        title = "User Service API",
        version = "0.1.0-alpha.1",
        description = "User profile management, connections, and GDPR compliance for Unity Platform",
        contact(
            name = "Unity Platform Team",
            email = "dev@unityplan.org"
        )
    ),
    paths(
        // Health endpoints
        user_service::handlers::health::health_check,
        user_service::handlers::health::ready_check,
        user_service::handlers::health::metrics,
        // Profile endpoints
        user_service::handlers::profile::get_own_profile,
        user_service::handlers::profile::update_own_profile,
        user_service::handlers::profile::get_profile_by_id,
        // Profile links endpoints
        user_service::handlers::profile_link::list_links,
        user_service::handlers::profile_link::create_link,
        user_service::handlers::profile_link::update_link,
        user_service::handlers::profile_link::delete_link,
        // Language proficiency endpoints
        user_service::handlers::language_proficiency::list_languages,
        user_service::handlers::language_proficiency::create_language,
        user_service::handlers::language_proficiency::update_language,
        user_service::handlers::language_proficiency::delete_language,
        // Settings endpoints
        user_service::handlers::settings::get_settings,
        user_service::handlers::settings::update_settings,
        user_service::handlers::settings::get_privacy_settings,
        user_service::handlers::settings::update_privacy_settings,
        user_service::handlers::settings::get_notification_settings,
        user_service::handlers::settings::update_notification_settings,
        // Connection endpoints
        user_service::handlers::connection::follow_user,
        user_service::handlers::connection::unfollow_user,
        user_service::handlers::connection::block_user,
        user_service::handlers::connection::unblock_user,
        user_service::handlers::connection::get_followers,
        user_service::handlers::connection::get_following,
        user_service::handlers::connection::search_users,
    ),
    components(
        schemas(
            // Profile schemas
            ProfileResponse,
            UpdateProfileRequest,
            // Profile links schemas
            ProfileLinkResponse,
            CreateProfileLinkRequest,
            UpdateProfileLinkRequest,
            // Language proficiency schemas
            LanguageProficiencyResponse,
            CreateLanguageProficiencyRequest,
            UpdateLanguageProficiencyRequest,
            // Settings schemas
            SettingsResponse,
            UpdateSettingsRequest,
            PrivacySettingsResponse,
            UpdatePrivacySettingsRequest,
            NotificationSettingsResponse,
            UpdateNotificationSettingsRequest,
            // Connection schemas
            ConnectionType,
            ConnectionStatus,
            ConnectionResponse,
            ConnectionsListResponse,
            UserSearchResult,
            UserSearchResponse
        )
    ),
    tags(
        (name = "health", description = "Service health and monitoring"),
        (name = "profile", description = "User profile management (basic info, links, languages)"),
        (name = "settings", description = "User preferences, privacy, and notification settings"),
        (name = "connections", description = "User connections (follow/block)")
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

    tracing::info!("🚀 Starting User Service v{}", env!("CARGO_PKG_VERSION"));

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

    // Initialize metrics collector
    let metrics_collector =
        shared_lib::MetricsCollector::new("user_service", env!("CARGO_PKG_VERSION"));

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
                    .route("/health", web::get().to(user_service::handlers::health::health_check))
                    .route("/ready", web::get().to(user_service::handlers::health::ready_check))
                    .route("/metrics", web::get().to(user_service::handlers::health::metrics))
                    // User routes (all require JWT auth)
                    .service(
                        web::scope("/user")
                            // Register more specific routes first
                            .configure(user_service::handlers::settings::configure)
                            .configure(user_service::handlers::profile_link::configure)
                            .configure(user_service::handlers::language_proficiency::configure)
                            .configure(user_service::handlers::connection::configure)
                            // Register profile with /{id} last (greedy catch-all)
                            .configure(user_service::handlers::profile::configure),
                    ),
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

    tracing::info!("👋 User service stopped gracefully");
    Ok(())
}

