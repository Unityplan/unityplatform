use actix_web::{web, App, HttpServer};
use shared_lib::{
    cors, shutdown_grace_period, shutdown_signal, AppConfig, Database, LoggingMiddleware,
    RateLimitMiddleware, RequestIdMiddleware, SecurityHeadersMiddleware,
};
use user_service::handlers::profile::UpdateProfileRequest;
use user_service::models::{
    ConnectionResponse, ConnectionStatus, ConnectionType, ConnectionsListResponse,
    CreateLanguageProficiencyRequest, CreateProfileLinkRequest, LanguageProficiencyResponse,
    ProfileLinkResponse, ProfileResponse, UpdateLanguageProficiencyRequest,
    UpdateProfileLinkRequest, UserSearchResponse, UserSearchResult,
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
        health_check,
        user_service::handlers::profile::get_own_profile,
        user_service::handlers::profile::update_own_profile,
        user_service::handlers::profile::get_profile_by_id,
        user_service::handlers::profile_link::list_links,
        user_service::handlers::profile_link::create_link,
        user_service::handlers::profile_link::update_link,
        user_service::handlers::profile_link::delete_link,
        user_service::handlers::language_proficiency::list_languages,
        user_service::handlers::language_proficiency::create_language,
        user_service::handlers::language_proficiency::update_language,
        user_service::handlers::language_proficiency::delete_language,
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
            ProfileResponse,
            UpdateProfileRequest,
            ProfileLinkResponse,
            CreateProfileLinkRequest,
            UpdateProfileLinkRequest,
            LanguageProficiencyResponse,
            CreateLanguageProficiencyRequest,
            UpdateLanguageProficiencyRequest,
            ConnectionType,
            ConnectionStatus,
            ConnectionResponse,
            ConnectionsListResponse,
            UserSearchResult,
            UserSearchResponse
        )
    ),
    tags(
        (name = "service", description = "Service health and metadata"),
        (name = "profile", description = "User profile management"),
        (name = "profile-links", description = "External profile links (GitHub, LinkedIn, etc.)"),
        (name = "language-proficiency", description = "Language skills with 4 proficiency dimensions"),
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

    // NATS is optional for now - we'll use it later for event publishing
    // For Phase 1 single-service development, we can skip NATS
    let _nats_client: Option<shared_lib::NatsClient> = None; // TODO: Enable when we need cross-service events

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
            // NATS client will be added when we implement event publishing
            // .app_data(web::Data::new(nats_client.clone()))
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // API routes
            .service(
                web::scope("/api/v1")
                    // Service routes
                    .service(web::scope("/service").route("/health", web::get().to(health_check)))
                    // User routes (all require JWT auth)
                    .service(
                        web::scope("/user")
                            // Register more specific routes first
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

/// Health check endpoint (no authentication required)
#[utoipa::path(
    get,
    path = "/api/v1/service/health",
    tag = "service",
    responses(
        (status = 200, description = "Service is healthy", body = serde_json::Value,
            example = json!({
                "status": "healthy",
                "service": "user-service",
                "version": "0.1.0-alpha.1"
            })
        )
    )
)]
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "user-service",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
