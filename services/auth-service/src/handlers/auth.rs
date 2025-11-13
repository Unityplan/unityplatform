use crate::models::{
    AuthResponse, HealthResponse, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest,
    ValidateResponse,
};
use crate::services::{PasswordService, TokenService};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use shared_lib::{AppError, Database, Result, ValidatedJson};
use sqlx::Row;
use std::env;
use utoipa;
use uuid::Uuid;

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Username or email already exists"),
    ),
    tag = "Authentication"
)]
#[post("/register")]
pub async fn register(
    body: ValidatedJson<RegisterRequest>,
    db: web::Data<Database>,
    token_service: web::Data<TokenService>,
) -> Result<HttpResponse> {
    let req = body.into_inner();

    // Feature flag: Check if invitation validation is enabled
    let enable_invitation_check = env::var("ENABLE_INVITATION_VALIDATION")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    if enable_invitation_check {
        if let Some(invitation_token) = &req.invitation_token {
            // TODO: Call invitation-service to validate token when service is available
            // For now, just log that we would validate
            tracing::info!(
                "Would validate invitation token: {} (invitation-service not yet implemented)",
                invitation_token
            );
        } else {
            return Err(AppError::Validation(
                "Invitation token required when invitation validation is enabled".to_string(),
            ));
        }
    } else {
        tracing::warn!("Invitation validation is disabled - registration without invitation check");
    }

    let pool = db.pool();

    // Check if username already exists in global registry
    let username_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM global.username_registry WHERE username = $1)",
    )
    .bind(&req.username)
    .fetch_one(pool)
    .await?;

    if username_exists {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    // Check if email already exists in global registry (only if email provided)
    if let Some(ref email) = req.email {
        let email_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM global.email_registry WHERE email = $1)",
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        if email_exists {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }
    }

    // Hash password
    let password_hash = PasswordService::hash(&req.password)?;

    // Start transaction
    let mut tx = pool.begin().await?;

    // Create user in territory-specific users table
    let user_id = Uuid::new_v4();
    let territory_table = format!("territory_{}.users", req.territory);

    sqlx::query(&format!(
        "INSERT INTO {} (id, username, email, password_hash, territory_code, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
        territory_table
    ))
    .bind(user_id)
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.territory)
    .execute(&mut *tx)
    .await?;

    // Register username globally
    sqlx::query(
        "INSERT INTO global.username_registry (username, user_id, territory_code, registered_at)
         VALUES ($1, $2, $3, NOW())",
    )
    .bind(&req.username)
    .bind(user_id)
    .bind(&req.territory)
    .execute(&mut *tx)
    .await?;

    // Register email globally (only if email provided)
    if let Some(ref email) = req.email {
        sqlx::query(
            "INSERT INTO global.email_registry (email, user_id, territory_code, registered_at)
             VALUES ($1, $2, $3, NOW())",
        )
        .bind(email)
        .bind(user_id)
        .bind(&req.territory)
        .execute(&mut *tx)
        .await?;
    }

    // Commit transaction
    tx.commit().await?;

    // Generate tokens
    let access_token = token_service.generate_access_token(user_id, &req.territory)?;
    let refresh_token = TokenService::generate_refresh_token();

    // Hash the refresh token for storage (never store plain tokens)
    let token_hash = PasswordService::hash(&refresh_token)?;

    // Store refresh token
    let refresh_token_table = format!("territory_{}.refresh_tokens", req.territory);
    let expires_at = Utc::now() + Duration::days(7);

    sqlx::query(&format!(
        "INSERT INTO {} (token_hash, user_id, expires_at, created_at)
         VALUES ($1, $2, $3, NOW())",
        refresh_token_table
    ))
    .bind(&token_hash)
    .bind(user_id)
    .bind(expires_at)
    .execute(pool)
    .await?;

    let response = AuthResponse::new(access_token, refresh_token);

    Ok(HttpResponse::Created().json(response))
}

/// Login an existing user
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
    ),
    tag = "Authentication"
)]
#[post("/login")]
pub async fn login(
    body: ValidatedJson<LoginRequest>,
    db: web::Data<Database>,
    token_service: web::Data<TokenService>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let pool = db.pool();

    // Get user from territory-specific table
    let territory_table = format!("territory_{}.users", req.territory);

    let user = sqlx::query(&format!(
        "SELECT id, password_hash FROM {} WHERE username = $1 AND deleted_at IS NULL",
        territory_table
    ))
    .bind(&req.username)
    .fetch_optional(pool)
    .await?;

    let user = user.ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let user_id: Uuid = user.get("id");
    let password_hash: String = user.get("password_hash");

    // Verify password
    let password_valid = PasswordService::verify(&req.password, &password_hash)?;

    if !password_valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Generate tokens
    let access_token = token_service.generate_access_token(user_id, &req.territory)?;
    let refresh_token = TokenService::generate_refresh_token();

    // Hash the refresh token for storage (never store plain tokens)
    let token_hash = PasswordService::hash(&refresh_token)?;

    // Store refresh token
    let refresh_token_table = format!("territory_{}.refresh_tokens", req.territory);
    let expires_at = Utc::now() + Duration::days(7);

    sqlx::query(&format!(
        "INSERT INTO {} (token_hash, user_id, expires_at, created_at)
         VALUES ($1, $2, $3, NOW())",
        refresh_token_table
    ))
    .bind(&token_hash)
    .bind(user_id)
    .bind(expires_at)
    .execute(pool)
    .await?;

    let response = AuthResponse::new(access_token, refresh_token);

    Ok(HttpResponse::Ok().json(response))
}

/// Refresh access token using refresh token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed successfully"),
        (status = 401, description = "Invalid or expired refresh token"),
    ),
    tag = "Authentication"
)]
#[post("/refresh")]
pub async fn refresh(
    body: ValidatedJson<RefreshRequest>,
    db: web::Data<Database>,
    token_service: web::Data<TokenService>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let pool = db.pool();

    // Query all territory refresh token tables to find the token
    // In production, you might want to pass territory or store it differently
    let territories = vec!["dk", "no", "se", "eu"]; // TODO: Get from config

    for territory in territories {
        let refresh_token_table = format!("territory_{}.refresh_tokens", territory);

        // Get all non-revoked, non-expired tokens for this territory
        let tokens = sqlx::query(&format!(
            "SELECT id, token_hash, user_id, expires_at FROM {} WHERE revoked = FALSE AND expires_at > NOW()",
            refresh_token_table
        ))
        .fetch_all(pool)
        .await?;

        // Check each token hash to find a match
        for token_row in tokens {
            let token_hash: String = token_row.get("token_hash");

            // Verify the provided token against the stored hash
            if PasswordService::verify(&req.refresh_token, &token_hash).unwrap_or(false) {
                let user_id: Uuid = token_row.get("user_id");
                let expires_at: chrono::DateTime<Utc> = token_row.get("expires_at");

                // Double-check expiration (already filtered in query, but be explicit)
                if expires_at < Utc::now() {
                    return Err(AppError::Unauthorized("Refresh token expired".to_string()));
                }

                // Generate new access token
                let access_token = token_service.generate_access_token(user_id, territory)?;

                let response = serde_json::json!({
                    "access_token": access_token,
                    "token_type": "Bearer",
                    "expires_in": 900,
                });

                return Ok(HttpResponse::Ok().json(response));
            }
        }
    }

    Err(AppError::Unauthorized("Invalid refresh token".to_string()))
}

/// Logout (invalidate refresh token)
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logged out successfully"),
    ),
    tag = "Authentication"
)]
#[post("/logout")]
pub async fn logout(
    body: ValidatedJson<LogoutRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let pool = db.pool();

    // Revoke refresh token from all territory tables
    let territories = vec!["dk", "no", "se", "eu"]; // TODO: Get from config

    for territory in territories {
        let refresh_token_table = format!("territory_{}.refresh_tokens", territory);

        // Get all non-revoked tokens and check hash
        let tokens = sqlx::query(&format!(
            "SELECT id, token_hash FROM {} WHERE revoked = FALSE",
            refresh_token_table
        ))
        .fetch_all(pool)
        .await?;

        for token_row in tokens {
            let token_id: Uuid = token_row.get("id");
            let token_hash: String = token_row.get("token_hash");

            // If this token matches, revoke it
            if PasswordService::verify(&req.refresh_token, &token_hash).unwrap_or(false) {
                sqlx::query(&format!(
                    "UPDATE {} SET revoked = TRUE, revoked_at = NOW() WHERE id = $1",
                    refresh_token_table
                ))
                .bind(token_id)
                .execute(pool)
                .await?;

                // Found and revoked the token, return success
                return Ok(HttpResponse::Ok().json(serde_json::json!({
                    "message": "Logged out successfully"
                })));
            }
        }
    }

    // Token not found, but still return success (idempotent operation)
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

/// Validate JWT token and return claims
#[utoipa::path(
    get,
    path = "/api/v1/auth/validate",
    responses(
        (status = 200, description = "Token is valid", body = ValidateResponse),
        (status = 401, description = "Invalid or expired token"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Authentication"
)]
#[get("/validate")]
pub async fn validate(
    req: HttpRequest,
    token_service: web::Data<TokenService>,
) -> Result<HttpResponse> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

    let token = TokenService::extract_token(auth_header)?;
    let claims = token_service.validate_token(&token)?;

    let response = ValidateResponse {
        valid: true,
        user_id: Some(claims.sub),
        territory: Some(claims.territory),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/auth/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
    ),
    tag = "Health"
)]
#[get("/health")]
pub async fn health(db: web::Data<Database>) -> Result<HttpResponse> {
    let pool = db.pool();

    // Test database connection
    let db_status = match sqlx::query("SELECT 1").fetch_one(pool).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let response = HealthResponse {
        status: "ok".to_string(),
        database: db_status.to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}
