#!/bin/bash
# Unity Platform Service Scaffolding Script
# Creates a new microservice with correct structure and boilerplate
# Based on user-service template (uses JWT auth middleware)

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() { echo -e "${GREEN}ℹ️  $1${NC}"; }
print_warn() { echo -e "${YELLOW}⚠️  $1${NC}"; }
print_error() { echo -e "${RED}❌ $1${NC}"; exit 1; }
print_success() { echo -e "${GREEN}✅ $1${NC}"; }

# Check arguments
if [ $# -lt 3 ]; then
    print_error "Usage: $0 <service-base-name> <port> <description>
    
Example: $0 invitation 8004 'Manages invitation tokens for user registration'
         Creates: invitation-service (NOT invitation-service-service)

⚠️  IMPORTANT: Use base name only (e.g., 'invitation', not 'invitation-service')
              The script automatically adds '-service' suffix!

Available ports: 8003-8020 (auth-service is 8003, user-service is 8005)
"
fi

SERVICE_NAME=$1
PORT=$2
DESCRIPTION=$3

# Check if user accidentally included -service suffix
if [[ $SERVICE_NAME == *"-service" ]]; then
    print_error "❌ Don't include '-service' suffix! 
    
You provided: $SERVICE_NAME
Use instead:   ${SERVICE_NAME%-service}

The script automatically adds '-service' suffix.
Example: './scaffold-service.sh invitation 8004 \"description\"' creates 'invitation-service'"
fi

# Validate service name (lowercase, no spaces)
if [[ ! $SERVICE_NAME =~ ^[a-z][a-z0-9-]*$ ]]; then
    print_error "Service name must be lowercase, start with letter, and contain only letters, numbers, and hyphens"
fi

# Validate port (must be number)
if [[ ! $PORT =~ ^[0-9]+$ ]]; then
    print_error "Port must be a number"
fi

# Convert names
SERVICE_NAME_KEBAB="${SERVICE_NAME}"                    # invitation
SERVICE_NAME_WITH_SUFFIX="${SERVICE_NAME_KEBAB}-service" # invitation-service
SERVICE_NAME_SNAKE="${SERVICE_NAME_WITH_SUFFIX//-/_}"    # invitation_service
SERVICE_NAME_TITLE="${SERVICE_NAME^}"                    # Invitation

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SERVICES_DIR="$WORKSPACE_ROOT/services"
SERVICE_DIR="$SERVICES_DIR/${SERVICE_NAME_KEBAB}-service"

print_info "Creating new service: ${SERVICE_NAME_KEBAB}-service"
print_info "Port: $PORT"
print_info "Description: $DESCRIPTION"

# Check if service already exists
if [ -d "$SERVICE_DIR" ]; then
    print_warn "Service directory already exists: $SERVICE_DIR"
    read -p "Delete and recreate? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_info "Removing existing service..."
        rm -rf "$SERVICE_DIR"
    else
        print_error "Aborted - service already exists"
    fi
fi

# Create directory structure
print_info "Creating directory structure..."
mkdir -p "$SERVICE_DIR/src/"{handlers,models,services}

# Create Cargo.toml
print_info "Creating Cargo.toml..."
cat > "$SERVICE_DIR/Cargo.toml" << EOF
[package]
name = "${SERVICE_NAME_KEBAB}-service"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
homepage.workspace = true
repository.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true

[lib]
name = "${SERVICE_NAME_SNAKE}_service"
path = "src/lib.rs"

[[bin]]
name = "${SERVICE_NAME_KEBAB}-service"
path = "src/main.rs"

[dependencies]
# Shared library with middleware
shared-lib = { path = "../shared-lib" }

# Web framework
actix-web = { workspace = true }
actix-cors = { workspace = true }

# Async runtime
tokio = { workspace = true }

# Database
sqlx = { workspace = true }
redis = { workspace = true }

# Serialization
serde = { workspace = true }
serde_json = { workspace = true }

# Validation
validator = { workspace = true }

# Types
uuid = { workspace = true }
chrono = { workspace = true }

# Configuration
dotenvy = { workspace = true }

# Logging
tracing = { workspace = true }
tracing-subscriber = { workspace = true }

# Error handling
thiserror = { workspace = true }
anyhow = { workspace = true }

# API Documentation
utoipa = { workspace = true }
utoipa-swagger-ui = { workspace = true }
EOF

# Create lib.rs
print_info "Creating lib.rs..."
cat > "$SERVICE_DIR/src/lib.rs" << 'EOF'
// Re-export modules for use in main.rs
pub mod handlers;
pub mod models;
pub mod services;
EOF

# Create main.rs
print_info "Creating main.rs..."
cat > "$SERVICE_DIR/src/main.rs" << EOF
use actix_web::{web, App, HttpServer};
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
        title = "${SERVICE_NAME_TITLE} Service API",
        version = "0.1.0-alpha.1",
        description = "${DESCRIPTION}",
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
        // TODO: Add service-specific endpoints here
    ),
    components(
        schemas(
            // TODO: Add service-specific schemas here
        )
    ),
    tags(
        (name = "health", description = "Service health and monitoring"),
        // TODO: Add service-specific tags here
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

    tracing::info!("🚀 Starting ${SERVICE_NAME_TITLE} Service v{}", env!("CARGO_PKG_VERSION"));

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

    // Initialize metrics collector
    let metrics_collector =
        shared_lib::MetricsCollector::new("${SERVICE_NAME_SNAKE}_service", env!("CARGO_PKG_VERSION"));

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
                    // TODO: Add service-specific routes here
                    // Example:
                    // .service(
                    //     web::scope("/${SERVICE_NAME_KEBAB}")
                    //         .configure(${SERVICE_NAME_SNAKE}_service::handlers::configure)
                    // )
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

    tracing::info!("👋 ${SERVICE_NAME_TITLE} service stopped gracefully");
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
                "service": "${SERVICE_NAME_KEBAB}-service",
                "version": "0.1.0-alpha.1"
            })
        )
    )
)]
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "${SERVICE_NAME_KEBAB}-service",
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
                "service": "${SERVICE_NAME_KEBAB}-service",
                "version": "0.1.0-alpha.1"
            })
        ),
        (status = 503, description = "Service not ready", body = serde_json::Value,
            example = json!({
                "status": "not_ready",
                "service": "${SERVICE_NAME_KEBAB}-service",
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
            "service": "${SERVICE_NAME_KEBAB}-service",
            "version": env!("CARGO_PKG_VERSION"),
        })),
        Err(_) => actix_web::HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "service": "${SERVICE_NAME_KEBAB}-service",
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
            example = "# HELP ${SERVICE_NAME_SNAKE}_service_http_requests_total Total HTTP requests"
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
EOF

# Create handlers/mod.rs
print_info "Creating handlers/mod.rs..."
cat > "$SERVICE_DIR/src/handlers/mod.rs" << EOF
// TODO: Add handler modules here
// Example:
// pub mod ${SERVICE_NAME_SNAKE};

// TODO: Add configure function for route registration
// Example:
// pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
//     cfg.service(create_item)
//        .service(get_item)
//        .service(update_item)
//        .service(delete_item);
// }
EOF

# Create models/mod.rs
print_info "Creating models/mod.rs..."
cat > "$SERVICE_DIR/src/models/mod.rs" << EOF
// TODO: Add model modules here
// Example:
// pub mod request;
// pub mod response;
//
// pub use request::*;
// pub use response::*;
EOF

# Create services/mod.rs
print_info "Creating services/mod.rs..."
cat > "$SERVICE_DIR/src/services/mod.rs" << EOF
// TODO: Add service modules here (business logic)
// Example:
// pub mod ${SERVICE_NAME_SNAKE}_service;
//
// pub use ${SERVICE_NAME_SNAKE}_service::${SERVICE_NAME_TITLE}Service;
EOF

# Create .env.example
print_info "Creating .env.example..."
cat > "$SERVICE_DIR/.env.example" << EOF
# Server Configuration
APP__SERVER__HOST=0.0.0.0
APP__SERVER__PORT=${PORT}
APP__SERVER__POD_ID=dev-pod
APP__SERVER__TERRITORY=dk

# Database Configuration
APP__DATABASE__URL=postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk
APP__DATABASE__MAX_CONNECTIONS=20
APP__DATABASE__MIN_CONNECTIONS=5

# Redis Configuration (for rate limiting)
REDIS_URL=redis://localhost:6379

# NATS Configuration
APP__NATS__URL=nats://localhost:4222
APP__NATS__CLUSTER_NAME=unityplatform-global

# JWT Configuration (for AppConfig.auth)
APP__AUTH__JWT_SECRET=dev_jwt_secret_please_change_in_production
APP__AUTH__JWT_EXPIRATION_HOURS=24

# Logging
RUST_LOG=${SERVICE_NAME_SNAKE}=debug,actix_web=info,sqlx=warn
EOF

# Create CHANGELOG.md
print_info "Creating CHANGELOG.md..."
cat > "$SERVICE_DIR/CHANGELOG.md" << EOF
# Changelog - ${SERVICE_NAME_TITLE} Service

All notable changes to ${SERVICE_NAME_KEBAB}-service will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial service scaffolding
- Health, ready, and metrics endpoints
- OpenAPI documentation structure
- Full middleware stack (logging, security, CORS, rate limiting)

## [0.1.0-alpha.1] - $(date +%Y-%m-%d)

### Added
- Project structure created with scaffold-service.sh script
EOF

# Add to workspace Cargo.toml
print_info "Adding to workspace Cargo.toml..."
if grep -q "\"${SERVICE_NAME_KEBAB}-service\"" "$SERVICES_DIR/Cargo.toml"; then
    print_warn "Service already in workspace Cargo.toml, skipping..."
else
    # Add service in alphabetical order by finding the right position
    # Simple approach: add before "shared-lib" (which is alphabetically after most services)
    if grep -q '"shared-lib"' "$SERVICES_DIR/Cargo.toml"; then
        # Add before shared-lib
        sed -i '/"shared-lib"/i\    "'"${SERVICE_NAME_KEBAB}-service"'",' "$SERVICES_DIR/Cargo.toml"
    else
        # Fallback: add after the last service entry before closing bracket
        sed -i '/members = \[/,/^\]/ {/^\]/i\    "'"${SERVICE_NAME_KEBAB}-service"'",
}' "$SERVICES_DIR/Cargo.toml"
    fi
    print_success "Added to workspace Cargo.toml"
    print_warn "Note: You may need to manually sort the members array alphabetically"
fi

# Test build
print_info "Testing build..."
cd "$SERVICES_DIR"
if cargo build -p "${SERVICE_NAME_KEBAB}-service" 2>&1 | tee /tmp/scaffold-build.log; then
    print_success "Service builds successfully!"
else
    print_error "Build failed! Check /tmp/scaffold-build.log for details"
fi

# Summary
print_success "
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Service scaffolding complete!

Service: ${SERVICE_NAME_KEBAB}-service
Port: ${PORT}
Location: $SERVICE_DIR

Next steps:
1. Review the implementation guide:
   docs/guides/development/service-implementation-guide.md

2. Implement your business logic:
   - Add models in src/models/
   - Add handlers in src/handlers/
   - Add services in src/services/

3. Update OpenAPI docs in src/main.rs:
   - Add paths to #[openapi(paths(...))]
   - Add schemas to #[openapi(components(schemas(...)))]

4. Test your service:
   cd services
   cargo test -p ${SERVICE_NAME_KEBAB}-service
   cargo run -p ${SERVICE_NAME_KEBAB}-service

5. Access Swagger UI:
   http://localhost:${PORT}/swagger-ui/

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"
