#!/bin/bash
# Scaffold a new service based on auth-service template
# Usage: ./scaffold-service.sh service-name port

set -e

SERVICE_NAME=$1
PORT=$2
SERVICE_DIR="services/$SERVICE_NAME"

if [ -z "$SERVICE_NAME" ] || [ -z "$PORT" ]; then
    echo "Usage: ./scaffold-service.sh service-name port"
    echo "Example: ./scaffold-service.sh user-service 8002"
    exit 1
fi

echo "📦 Scaffolding $SERVICE_NAME on port $PORT..."

# Create directory structure
mkdir -p "$SERVICE_DIR/src/"{models,handlers,services,middleware}

# Create Cargo.toml
cat > "$SERVICE_DIR/Cargo.toml" << 'EOF'
[package]
name = "SERVICE_NAME"
version.workspace = true
edition.workspace = true

[lib]
name = "SERVICE_CRATE_NAME"
path = "src/lib.rs"

[[bin]]
name = "SERVICE_NAME"
path = "src/main.rs"

[dependencies]
shared-lib = { path = "../shared-lib" }

# Web framework
actix-web = "4"
actix-cors = "0.7"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Database
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "uuid", "chrono"] }

# Authentication (for JWT validation)
jsonwebtoken = "9"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error handling
anyhow = "1"
thiserror = "1"

# Validation
validator = { version = "0.18", features = ["derive"] }

# Types
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# Configuration
dotenvy = "0.15"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# OpenAPI
utoipa = { version = "5", features = ["actix_extras", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "8", features = ["actix-web"] }
EOF

# Replace placeholders
CRATE_NAME=$(echo "$SERVICE_NAME" | tr '-' '_')
sed -i "s/SERVICE_NAME/$SERVICE_NAME/g" "$SERVICE_DIR/Cargo.toml"
sed -i "s/SERVICE_CRATE_NAME/$CRATE_NAME/g" "$SERVICE_DIR/Cargo.toml"

# Create .env
    cat > .env << 'ENV_EOF'
DATABASE_URL=postgresql://unityplan:unityplan_dev_password_dk@localhost:5432/unityplan
TERRITORY_CODE=dk
SERVER_HOST=0.0.0.0
SERVER_PORT=${PORT}
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000
JWT_SECRET=dev_secret_change_in_production_please
ENV_EOF

# Create src/main.rs
cat > "$SERVICE_DIR/src/main.rs" << 'EOF'
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = SERVICE_CRATE_NAME::config::Config::from_env()
        .expect("Failed to load configuration");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    let allowed_origins = config.cors.allowed_origins.clone();

    tracing::info!("Starting SERVICE_DISPLAY_NAME on {}:{}", server_host, server_port);

    HttpServer::new(move || {
        let mut cors = actix_cors::Cors::default();
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }
        let cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(SERVICE_CRATE_NAME::handlers::health))
            .service(SERVICE_CRATE_NAME::openapi::swagger_ui())
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
EOF

DISPLAY_NAME=$(echo "$SERVICE_NAME" | sed 's/-/ /g' | awk '{for(i=1;i<=NF;i++)sub(/./,toupper(substr($i,1,1)),$i)}1')
sed -i "s/SERVICE_CRATE_NAME/$CRATE_NAME/g" "$SERVICE_DIR/src/main.rs"
sed -i "s/SERVICE_DISPLAY_NAME/$DISPLAY_NAME/g" "$SERVICE_DIR/src/main.rs"

# Create src/lib.rs
cat > "$SERVICE_DIR/src/lib.rs" << 'EOF'
pub mod config;
pub mod error;
pub mod response;
pub mod models;
pub mod handlers;
pub mod openapi;

pub use error::ServiceError;
pub use response::ApiResponse;
EOF

# Copy config.rs from auth-service (with port adjustment)
cp services/auth-service/src/config.rs "$SERVICE_DIR/src/config.rs"
sed -i "s/8001/$PORT/g" "$SERVICE_DIR/src/config.rs"

# Copy error.rs from auth-service
cp services/auth-service/src/error.rs "$SERVICE_DIR/src/error.rs"
# Rename AuthError to ServiceError
sed -i 's/AuthError/ServiceError/g' "$SERVICE_DIR/src/error.rs"
sed -i 's/auth-service/SERVICE_NAME/g' "$SERVICE_DIR/src/error.rs"
sed -i "s/SERVICE_NAME/$SERVICE_NAME/g" "$SERVICE_DIR/src/error.rs"

# Copy response.rs from auth-service
cp services/auth-service/src/response.rs "$SERVICE_DIR/src/response.rs"

# Create models/mod.rs
cat > "$SERVICE_DIR/src/models/mod.rs" << 'EOF'
mod health;

pub use health::*;
EOF

# Create models/health.rs
cat > "$SERVICE_DIR/src/models/health.rs" << 'EOF'
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub dependencies: DependencyStatus,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DependencyStatus {
    pub database: String,
}
EOF

# Create handlers/mod.rs
cat > "$SERVICE_DIR/src/handlers/mod.rs" << 'EOF'
mod health;

pub use health::*;
EOF

# Create handlers/health.rs
cat > "$SERVICE_DIR/src/handlers/health.rs" << 'EOF'
use crate::{models::*, ApiResponse};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health(pool: web::Data<PgPool>) -> HttpResponse {
    let db_status = match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "SERVICE_NAME".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        dependencies: crate::models::DependencyStatus {
            database: db_status.to_string(),
        },
    };

    HttpResponse::Ok().json(ApiResponse::success(response))
}
EOF

sed -i "s/SERVICE_NAME/$SERVICE_NAME/g" "$SERVICE_DIR/src/handlers/health.rs"

# Create openapi.rs
cat > "$SERVICE_DIR/src/openapi.rs" << 'EOF'
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health,
    ),
    components(
        schemas(
            crate::models::HealthResponse,
            crate::models::DependencyStatus,
            crate::response::ApiResponse<crate::models::HealthResponse>,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
    ),
    info(
        title = "SERVICE_DISPLAY_NAME API",
        version = "0.1.0-alpha.1",
        description = "SERVICE_DISPLAY_NAME for Unity Platform"
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
EOF

sed -i "s/SERVICE_DISPLAY_NAME/$DISPLAY_NAME/g" "$SERVICE_DIR/src/openapi.rs"

# Create empty services/mod.rs and middleware/mod.rs
touch "$SERVICE_DIR/src/services/mod.rs"
touch "$SERVICE_DIR/src/middleware/mod.rs"

echo "✅ $SERVICE_NAME scaffolded successfully!"
echo "   Directory: $SERVICE_DIR"
echo "   Port: $PORT"
echo "   Test: cd $SERVICE_DIR && cargo build"
