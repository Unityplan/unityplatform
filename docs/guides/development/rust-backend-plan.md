# Rust Backend Development Plan

## Overview

This document outlines the step-by-step plan for building the UnityPlan backend microservices in Rust, following a test-driven, incremental approach.

## ⚠️ Critical Guideline: Database Queries

**ALWAYS use runtime queries for all services due to multi-pod architecture.**

See: [Database Query Patterns](./database-query-patterns.md) for detailed explanation and examples.

**TL;DR:** Use `sqlx::query()` and `sqlx::query_as::<_, Type>()`, NOT `sqlx::query!()` or `sqlx::query_as!()` macros.

---

## Phase 2: Database Schema & Migrations (Week 1-2)

### 2.1 Database Setup

**Goal:** Set up SQLx migrations and core database schema

**Tasks:**

1. **Initialize SQLx CLI**

   ```bash
   cargo install sqlx-cli --no-default-features --features postgres
   ```

2. **Create Migrations Directory**

   ```bash
   cd services/shared-lib
   sqlx migrate add initial_schema
   ```

3. **Design Core Tables** (Migration 001)
   - `users` - User accounts with multi-territory support
   - `territories` - Territory definitions (Denmark, Norway, etc.)
   - `user_territories` - Many-to-many relationship
   - `roles` - System roles (Admin, Moderator, User, etc.)
   - `user_roles` - User role assignments per territory
   - `sessions` - Active user sessions

4. **User Table Schema**

   ```sql
   CREATE TABLE users (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       email VARCHAR(255) UNIQUE NOT NULL,
       username VARCHAR(50) UNIQUE NOT NULL,
       password_hash VARCHAR(255) NOT NULL,
       display_name VARCHAR(100),
       avatar_url TEXT,
       bio TEXT,
       is_verified BOOLEAN DEFAULT FALSE,
       is_active BOOLEAN DEFAULT TRUE,
       created_at TIMESTAMPTZ DEFAULT NOW(),
       updated_at TIMESTAMPTZ DEFAULT NOW()
   );
   
   CREATE INDEX idx_users_email ON users(email);
   CREATE INDEX idx_users_username ON users(username);
   ```

5. **Territory & Role Schema**

   ```sql
   CREATE TABLE territories (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       code VARCHAR(10) UNIQUE NOT NULL, -- 'dk', 'no', 'se'
       name VARCHAR(100) NOT NULL,
       description TEXT,
       is_active BOOLEAN DEFAULT TRUE,
       created_at TIMESTAMPTZ DEFAULT NOW()
   );
   
   CREATE TABLE roles (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       name VARCHAR(50) UNIQUE NOT NULL,
       description TEXT,
       permissions JSONB DEFAULT '{}'::jsonb,
       created_at TIMESTAMPTZ DEFAULT NOW()
   );
   
   CREATE TABLE user_territories (
       user_id UUID REFERENCES users(id) ON DELETE CASCADE,
       territory_id UUID REFERENCES territories(id) ON DELETE CASCADE,
       joined_at TIMESTAMPTZ DEFAULT NOW(),
       PRIMARY KEY (user_id, territory_id)
   );
   
   CREATE TABLE user_roles (
       user_id UUID REFERENCES users(id) ON DELETE CASCADE,
       role_id UUID REFERENCES roles(id) ON DELETE CASCADE,
       territory_id UUID REFERENCES territories(id) ON DELETE CASCADE,
       granted_at TIMESTAMPTZ DEFAULT NOW(),
       PRIMARY KEY (user_id, role_id, territory_id)
   );
   ```

6. **Session Management**

   ```sql
   CREATE TABLE sessions (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       user_id UUID REFERENCES users(id) ON DELETE CASCADE,
       token_hash VARCHAR(255) UNIQUE NOT NULL,
       ip_address INET,
       user_agent TEXT,
       expires_at TIMESTAMPTZ NOT NULL,
       created_at TIMESTAMPTZ DEFAULT NOW()
   );
   
   CREATE INDEX idx_sessions_user_id ON sessions(user_id);
   CREATE INDEX idx_sessions_token_hash ON sessions(token_hash);
   CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
   ```

7. **Run Migrations**

   ```bash
   export DATABASE_URL=postgres://unityplan:unityplan_dev_password_dk@localhost:5432/unityplan_dk
   sqlx migrate run
   ```

**Deliverables:**

- ✅ SQLx migrations in `services/shared-lib/migrations/`
- ✅ All core tables created
- ✅ Indexes and foreign keys configured
- ✅ Migration tested on Denmark pod database

---

## Phase 3: Authentication Service (Week 2-3)

### 3.1 Auth Service Setup

**Goal:** Create standalone authentication microservice

**Tasks:**

1. **Create auth-service Crate**

   ```bash
   cd services
   cargo new --bin auth-service
   ```

2. **Add to Workspace**

   ```toml
   # services/Cargo.toml
   members = ["shared-lib", "auth-service"]
   ```

3. **Configure Dependencies**

   ```toml
   # services/auth-service/Cargo.toml
   [dependencies]
   shared-lib = { path = "../shared-lib" }
   actix-web = { workspace = true }
   tokio = { workspace = true }
   sqlx = { workspace = true }
   serde = { workspace = true }
   serde_json = { workspace = true }
   jsonwebtoken = { workspace = true }
   argon2 = { workspace = true }
   validator = { workspace = true }
   tracing = { workspace = true }
   tracing-subscriber = { workspace = true }
   ```

### 3.2 Domain Models

**Goal:** Define authentication domain types

**Files to Create:**

- `auth-service/src/models/user.rs`
- `auth-service/src/models/session.rs`
- `auth-service/src/models/auth.rs`

**User Model:**

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}
```

### 3.3 Password Hashing Service

**Goal:** Secure password handling with Argon2

**File:** `auth-service/src/services/password.rs`

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use shared_lib::error::{AppError, Result};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?
            .to_string();
        
        Ok(password_hash)
    }
    
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;
        
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
```

### 3.4 JWT Token Service

**Goal:** JWT generation and validation

**File:** `auth-service/src/services/jwt.rs`

```rust
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use shared_lib::error::{AppError, Result};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_hours: i64,
}

impl JwtService {
    pub fn new(secret: &str, expiration_hours: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiration_hours,
        }
    }
    
    pub fn generate_token(&self, user_id: Uuid, email: &str, username: &str) -> Result<String> {
        let now = chrono::Utc::now().timestamp();
        let exp = now + (self.expiration_hours * 3600);
        
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            username: username.to_string(),
            iat: now,
            exp,
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Auth(format!("Token generation failed: {}", e)))
    }
    
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
    }
}
```

### 3.5 User Repository

**Goal:** Database operations for users

**File:** `auth-service/src/repositories/user_repository.rs`

```rust
use shared_lib::{Database, error::Result};
use crate::models::user::User;
use uuid::Uuid;

pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    pub async fn create_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
        display_name: Option<String>,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, username, password_hash, display_name)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(username)
        .bind(password_hash)
        .bind(display_name)
        .fetch_one(self.db.pool())
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(self.db.pool())
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.db.pool())
        .await?;
        
        Ok(user)
    }
    
    pub async fn update_last_login(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE users SET updated_at = NOW() WHERE id = $1"
        )
        .bind(user_id)
        .execute(self.db.pool())
        .await?;
        
        Ok(())
    }
}
```

### 3.6 HTTP Handlers

**Goal:** REST API endpoints

**File:** `auth-service/src/handlers/auth_handlers.rs`

```rust
use actix_web::{web, HttpResponse};
use shared_lib::error::Result;
use validator::Validate;

use crate::{
    models::auth::{LoginRequest, RegisterRequest, AuthResponse, UserInfo},
    services::{password::PasswordService, jwt::JwtService},
    repositories::user_repository::UserRepository,
};

pub async fn register(
    req: web::Json<RegisterRequest>,
    user_repo: web::Data<UserRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    req.validate()?;
    
    // Check if user exists
    if let Some(_) = user_repo.find_by_email(&req.email).await? {
        return Err(AppError::Validation("Email already registered".to_string()));
    }
    
    // Hash password
    let password_hash = PasswordService::hash_password(&req.password)?;
    
    // Create user
    let user = user_repo.create_user(
        &req.email,
        &req.username,
        &password_hash,
        req.display_name.clone(),
    ).await?;
    
    // Generate JWT
    let token = jwt_service.generate_token(user.id, &user.email, &user.username)?;
    
    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
        },
    }))
}

pub async fn login(
    req: web::Json<LoginRequest>,
    user_repo: web::Data<UserRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    req.validate()?;
    
    // Find user
    let user = user_repo
        .find_by_email(&req.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;
    
    // Verify password
    if !PasswordService::verify_password(&req.password, &user.password_hash)? {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }
    
    // Update last login
    user_repo.update_last_login(user.id).await?;
    
    // Generate JWT
    let token = jwt_service.generate_token(user.id, &user.email, &user.username)?;
    
    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
        },
    }))
}

pub async fn get_current_user(
    user_id: web::ReqData<Uuid>,
    user_repo: web::Data<UserRepository>,
) -> Result<HttpResponse> {
    let user = user_repo
        .find_by_id(*user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    
    Ok(HttpResponse::Ok().json(UserInfo {
        id: user.id,
        email: user.email,
        username: user.username,
        display_name: user.display_name,
        avatar_url: user.avatar_url,
    }))
}
```

### 3.7 Auth Middleware

**Goal:** JWT validation middleware

**File:** `auth-service/src/middleware/auth_middleware.rs`

```rust
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct AuthMiddleware {
    jwt_service: Arc<JwtService>,
}

impl AuthMiddleware {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            jwt_service: self.jwt_service.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_service: Arc<JwtService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        if let Some(auth_value) = auth_header {
            if let Ok(auth_str) = auth_value.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    if let Ok(claims) = self.jwt_service.validate_token(token) {
                        if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                            req.extensions_mut().insert(user_id);
                        }
                    }
                }
            }
        }
        
        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}
```

### 3.8 Main Application

**Goal:** Wire everything together

**File:** `auth-service/src/main.rs`

```rust
use actix_web::{web, App, HttpServer, middleware::Logger};
use shared_lib::{AppConfig, Database};
use std::sync::Arc;

mod models;
mod repositories;
mod services;
mod handlers;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    // Load configuration
    let config = AppConfig::from_env()
        .expect("Failed to load configuration");

    // Connect to database
    let db = Database::new(
        &config.database.url,
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");

    // Initialize services
    let jwt_service = Arc::new(services::jwt::JwtService::new(
        &config.auth.jwt_secret,
        config.auth.jwt_expiration_hours,
    ));
    
    let user_repo = Arc::new(repositories::user_repository::UserRepository::new(db.clone()));

    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting auth-service on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::from(jwt_service.clone()))
            .app_data(web::Data::from(user_repo.clone()))
            .service(
                web::scope("/api/auth")
                    .route("/register", web::post().to(handlers::auth_handlers::register))
                    .route("/login", web::post().to(handlers::auth_handlers::login))
                    .route("/me", web::get().to(handlers::auth_handlers::get_current_user))
            )
            .route("/health", web::get().to(|| async { "OK" }))
    })
    .bind(bind_addr)?
    .run()
    .await
}
```

**Deliverables:**

- ✅ auth-service compiles and runs
- ✅ Register endpoint functional
- ✅ Login endpoint functional
- ✅ JWT middleware working
- ✅ Health check endpoint

---

## Phase 4: Dockerization & Deployment (Week 3)

### 4.1 Create Dockerfile

**Goal:** Containerize auth-service

**File:** `services/auth-service/Dockerfile`

```dockerfile
# Build stage
FROM rust:1.83-slim as builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY shared-lib ./shared-lib
COPY auth-service ./auth-service

# Build release binary
RUN cargo build --release --package auth-service

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/auth-service /app/auth-service

EXPOSE 8080

CMD ["/app/auth-service"]
```

### 4.2 Add to docker-compose

**Goal:** Deploy to Denmark pod

**Update:** `docker-compose.pod.yml`

```yaml
  # Auth Service
  auth-service:
    build:
      context: ../services
      dockerfile: auth-service/Dockerfile
    container_name: service-auth-${POD_ID}
    restart: unless-stopped
    ports:
      - "${AUTH_SERVICE_PORT}:8080"
    environment:
      - APP__SERVER__HOST=0.0.0.0
      - APP__SERVER__PORT=8080
      - APP__SERVER__POD_ID=${POD_ID}
      - APP__SERVER__TERRITORY=${POD_NAME}
      - APP__DATABASE__URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/${POSTGRES_DB}
      - APP__DATABASE__MAX_CONNECTIONS=20
      - APP__DATABASE__MIN_CONNECTIONS=5
      - APP__NATS__URL=nats://nats:4222
      - APP__NATS__CLUSTER_NAME=unityplan-global
      - APP__AUTH__JWT_SECRET=${JWT_SECRET}
      - APP__AUTH__JWT_EXPIRATION_HOURS=24
      - RUST_LOG=info,auth_service=debug
    networks:
      - pod-net
      - mesh-network
    depends_on:
      postgres:
        condition: service_healthy
      nats:
        condition: service_started
```

### 4.3 Update Pod .env

**File:** `pods/denmark/.env`

Add:

```bash
# Auth Service
AUTH_SERVICE_PORT=8001
JWT_SECRET=your-super-secret-jwt-key-change-in-production
```

### 4.4 Deploy & Test

```bash
cd /home/henrik/code/data/projects/unityplan_platform/workspace
docker compose -f docker-compose.pod.yml -p pod-dk --env-file pods/denmark/.env up -d --build auth-service
```

**Test Registration:**

```bash
curl -X POST http://localhost:8001/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "SecurePass123!",
    "display_name": "Test User"
  }'
```

**Test Login:**

```bash
curl -X POST http://localhost:8001/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "SecurePass123!"
  }'
```

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_hashing() {
        let password = "SecurePass123!";
        let hash = PasswordService::hash_password(password).unwrap();
        assert!(PasswordService::verify_password(password, &hash).unwrap());
    }
    
    #[test]
    fn test_jwt_generation() {
        let service = JwtService::new("secret", 24);
        let token = service.generate_token(
            Uuid::new_v4(),
            "test@example.com",
            "testuser"
        ).unwrap();
        assert!(!token.is_empty());
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use actix_web::{test, App};
    
    #[actix_web::test]
    async fn test_register_endpoint() {
        let app = test::init_service(create_app()).await;
        
        let req = test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(RegisterRequest {
                email: "test@example.com".to_string(),
                username: "testuser".to_string(),
                password: "SecurePass123!".to_string(),
                display_name: None,
            })
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

---

## Metrics & Monitoring

### Add Prometheus Metrics

**File:** `auth-service/src/metrics.rs`

```rust
use prometheus::{IntCounterVec, HistogramVec, opts, register_int_counter_vec, register_histogram_vec};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HTTP_REQUESTS: IntCounterVec = register_int_counter_vec!(
        opts!("auth_http_requests_total", "Total HTTP requests"),
        &["method", "endpoint", "status"]
    ).unwrap();
    
    pub static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "auth_http_request_duration_seconds",
        "HTTP request duration",
        &["method", "endpoint"]
    ).unwrap();
    
    pub static ref AUTH_OPERATIONS: IntCounterVec = register_int_counter_vec!(
        opts!("auth_operations_total", "Authentication operations"),
        &["operation", "status"]
    ).unwrap();
}
```

Add metrics endpoint:

```rust
use actix_web::{HttpResponse, web};
use prometheus::{Encoder, TextEncoder};

async fn metrics() -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer)
}
```

---

## Summary Timeline

**Week 1:**

- Database schema design
- SQLx migrations
- Initial table creation

**Week 2:**

- Auth service structure
- Password & JWT services
- User repository
- HTTP handlers

**Week 3:**

- Dockerization
- Deployment to Denmark pod
- Testing & validation
- Monitoring setup

**Next Steps:**

- User service (profiles, preferences)
- Territory service
- Badge service
- Frontend integration

This plan provides a complete, production-ready authentication service as the foundation for all other microservices!
