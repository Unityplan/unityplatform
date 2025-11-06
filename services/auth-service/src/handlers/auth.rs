use crate::{
    models::{user::User, AuthResponse, AuthUserInfo, LoginRequest, RegisterRequest},
    services::{PasswordService, TokenService},
    utils::crypto::generate_public_key_hash,
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sha2::Digest;
use sqlx::{FromRow, PgPool};
use validator::Validate;

// Helper struct for territory validation
#[derive(FromRow)]
struct TerritoryCode {
    code: String,
}

/// Register a new user
pub async fn register(
    req: web::Json<RegisterRequest>,
    pool: web::Data<PgPool>,
    token_service: web::Data<TokenService>,
) -> actix_web::Result<HttpResponse> {
    // Validate request
    req.validate()
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Validation error: {}", e)))?;

    // Verify territory exists and is active
    let territory = sqlx::query_as::<_, TerritoryCode>(
        "SELECT code FROM global.territories WHERE code = $1 AND is_active = true",
    )
    .bind(&req.territory_code)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid territory code"))?;

    // Set schema context to territory (dynamic based on territory_code)
    let schema_name = format!("territory_{}", territory.code.to_lowercase());

    // Check if email already exists (dynamic schema)
    let existing_email = sqlx::query(&format!(
        "SELECT id FROM {}.users WHERE email = $1",
        schema_name
    ))
    .bind(&req.email)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if existing_email.is_some() {
        return Err(actix_web::error::ErrorBadRequest(
            "Email already registered",
        ));
    }

    // Check if username already exists (dynamic schema)
    let existing_username = sqlx::query(&format!(
        "SELECT id FROM {}.users WHERE username = $1",
        schema_name
    ))
    .bind(&req.username)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if existing_username.is_some() {
        return Err(actix_web::error::ErrorBadRequest("Username already taken"));
    }

    // Hash password
    let password_hash = PasswordService::hash_password(&req.password)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Generate public_key_hash
    let public_key_hash = generate_public_key_hash(&req.email, &req.username);

    // Create user (dynamic schema)
    let user = sqlx::query_as::<_, User>(&format!(
        r#"
        INSERT INTO {}.users (
            username, email, password_hash, full_name, public_key_hash
        ) VALUES ($1, $2, $3, $4, $5)
        RETURNING 
            id, public_key_hash, email, password_hash, username, 
            full_name, display_name, avatar_url, bio,
            email_visible, profile_public, data_export_requested,
            is_verified, is_active, last_login_at,
            created_at, updated_at
        "#,
        schema_name
    ))
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.full_name)
    .bind(&public_key_hash)
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Generate tokens
    let access_token = token_service
        .generate_access_token(
            &public_key_hash,
            &req.territory_code,
            user.id,
            &user.username,
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let refresh_token = token_service.generate_refresh_token();

    // Store refresh token (dynamic schema)
    let refresh_token_hash = format!("{:x}", sha2::Sha256::digest(refresh_token.as_bytes()));
    let expires_at = Utc::now() + chrono::Duration::days(7);

    sqlx::query(&format!(
        "INSERT INTO {}.refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        schema_name
    ))
    .bind(user.id)
    .bind(&refresh_token_hash)
    .bind(expires_at)
    .execute(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return response
    Ok(HttpResponse::Ok().json(AuthResponse {
        user: AuthUserInfo::from(user),
        access_token,
        refresh_token,
        expires_in: token_service.get_access_token_ttl(),
    }))
}

/// Login user
pub async fn login(
    req: web::Json<LoginRequest>,
    pool: web::Data<PgPool>,
    token_service: web::Data<TokenService>,
) -> actix_web::Result<HttpResponse> {
    // Validate request
    req.validate()
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Validation error: {}", e)))?;

    // Verify territory exists and is active
    let territory = sqlx::query_as::<_, TerritoryCode>(
        "SELECT code FROM global.territories WHERE code = $1 AND is_active = true",
    )
    .bind(&req.territory_code)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid territory code"))?;

    // Set schema context to territory (dynamic based on territory_code)
    let schema_name = format!("territory_{}", territory.code.to_lowercase());

    // Find user by email (dynamic schema)
    let user = sqlx::query_as::<_, User>(&format!(
        r#"
        SELECT 
            id, public_key_hash, email, password_hash, username, 
            full_name, display_name, avatar_url, bio,
            email_visible, profile_public, data_export_requested,
            is_verified, is_active, last_login_at,
            created_at, updated_at
        FROM {}.users WHERE email = $1
        "#,
        schema_name
    ))
    .bind(&req.email)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    // Check if user is active
    if !user.is_active {
        return Err(actix_web::error::ErrorUnauthorized("Account is inactive"));
    }

    // Verify password
    let password_hash = user
        .password_hash
        .as_ref()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    let is_valid = PasswordService::verify_password(&req.password, password_hash)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if !is_valid {
        return Err(actix_web::error::ErrorUnauthorized("Invalid credentials"));
    }

    // Update last login (dynamic schema)
    sqlx::query(&format!(
        "UPDATE {}.users SET last_login_at = $1 WHERE id = $2",
        schema_name
    ))
    .bind(Utc::now())
    .bind(user.id)
    .execute(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Generate tokens
    let public_key_hash = user
        .public_key_hash
        .as_ref()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Missing public key hash"))?;

    let access_token = token_service
        .generate_access_token(
            public_key_hash,
            &req.territory_code,
            user.id,
            &user.username,
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let refresh_token = token_service.generate_refresh_token();

    // Store refresh token (dynamic schema)
    let refresh_token_hash = format!("{:x}", sha2::Sha256::digest(refresh_token.as_bytes()));
    let expires_at = Utc::now() + chrono::Duration::days(7);

    sqlx::query(&format!(
        "INSERT INTO {}.refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        schema_name
    ))
    .bind(user.id)
    .bind(&refresh_token_hash)
    .bind(expires_at)
    .execute(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return response
    Ok(HttpResponse::Ok().json(AuthResponse {
        user: AuthUserInfo::from(user),
        access_token,
        refresh_token,
        expires_in: token_service.get_access_token_ttl(),
    }))
}

/// Health check endpoint
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "auth-service",
        "version": shared_lib::version::VERSION,
        "timestamp": Utc::now().to_rfc3339()
    }))
}
