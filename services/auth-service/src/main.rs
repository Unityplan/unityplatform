use actix_web::{web, App, HttpServer};
use auth_service::handlers;
use auth_service::models::{
    AuthResponse, HealthResponse, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest,
    ValidateResponse,
};
use auth_service::services::TokenService;
use dotenvy::dotenv;
use shared_lib::{
    cors, AppConfig, Database, LoggingMiddleware, RateLimitMiddleware, RequestIdMiddleware,
    SecurityHeadersMiddleware,
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
        (name = "Health", description = "Service health check"),
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
        &config.database.url,
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to initialize database");

    // Initialize Redis for rate limiting
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis:6379".to_string());
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");

    // Initialize token service
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev_jwt_secret_please_change_in_production".to_string());
    let token_service = TokenService::new(&jwt_secret);

    // Get service port from environment or use default
    let port: u16 = env::var("AUTH_SERVICE_PORT")
        .unwrap_or_else(|_| "8001".to_string())
        .parse()
        .expect("AUTH_SERVICE_PORT must be a valid port number");

    let bind_address = format!("0.0.0.0:{}", port);

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

    // Create HTTP server
    HttpServer::new(move || {
        App::new()
            // Priority 1 middleware - Request tracking and logging
            .wrap(LoggingMiddleware::development())
            .wrap(RequestIdMiddleware)
            // Priority 2 middleware - Security and rate limiting
            .wrap(SecurityHeadersMiddleware::development())
            .wrap(cors::development())
            .wrap(RateLimitMiddleware::development(redis_client.clone()))
            // Shared state
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(token_service.clone()))
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // Routes
            .service(
                web::scope("/api/v1/auth")
                    .service(handlers::register)
                    .service(handlers::login)
                    .service(handlers::refresh)
                    .service(handlers::logout)
                    .service(handlers::validate)
                    .service(handlers::health),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
