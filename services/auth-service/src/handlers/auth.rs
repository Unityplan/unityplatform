use crate::models::{
    AuthResponse, HealthResponse, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest,
    ValidateResponse,
};
use crate::services::{PasswordService, TokenService};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use shared_lib::{AppConfig, AppError, Database, NatsClient, Result, ValidatedJson};
use sqlx::Row;
use std::env;
use utoipa;
use uuid::Uuid;

/// Event envelope for NATS events (from NATS-EVENTS.md specification)
#[derive(Debug, Serialize, Deserialize)]
struct PlatformEvent<T> {
    event_id: Uuid,
    event_type: String,
    timestamp: chrono::DateTime<Utc>,
    territory: Option<String>,
    payload: T,
}

impl<T> PlatformEvent<T> {
    fn new(event_type: String, territory: Option<String>, payload: T) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type,
            timestamp: Utc::now(),
            territory,
            payload,
        }
    }
}

/// Event payload for user.registered event
#[derive(Debug, Serialize, Deserialize)]
struct UserRegisteredPayload {
    user_id: Uuid,
    username: String,
    territory: String,
    // NO email - security best practice (PII minimization)
}

/// Fetch user's badge slugs from the database
///
/// This queries the badge_users_badges table (owned by badge-service) for read-only access
/// and joins with global.registry_badge to get badge slugs for JWT claims.
///
/// Returns a list of badge slugs (e.g., ["code-of-conduct", "community-manager"])
async fn fetch_user_badge_slugs(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    territory: &str,
) -> Result<Vec<String>> {
    let query = format!(
        r#"
        SELECT rb.slug
        FROM territory_{territory}.badge_users_badges bub
        INNER JOIN global.registry_badge rb ON rb.id = bub.badge_id
        WHERE bub.user_id = $1
          AND (bub.expires_at IS NULL OR bub.expires_at > NOW())
        "#,
        territory = territory
    );

    let rows = sqlx::query(&query).bind(user_id).fetch_all(pool).await;

    // If the query fails (e.g., table doesn't exist), return empty badges
    // This allows the service to work even before badge tables are created
    match rows {
        Ok(rows) => {
            let slugs: Vec<String> = rows.iter().map(|r| r.get("slug")).collect();
            Ok(slugs)
        }
        Err(e) => {
            tracing::warn!("Failed to fetch user badges (non-fatal): {}", e);
            Ok(vec![])
        }
    }
}

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
    nats: web::Data<NatsClient>,
    config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    let req = body.into_inner();

    let allow_open_registration = config.server.allow_open_registration;
    let invitation_service_url =
        env::var("INVITATION_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8004".to_string());

    // Validate invitation token based on configuration
    if !allow_open_registration {
        // Production mode: invitation required
        let invitation_token = req.invitation_token.as_ref().ok_or_else(|| {
            AppError::Validation("Invitation token required for registration".to_string())
        })?;

        // Validate invitation token
        let is_valid = crate::services::invitation_client::validate_invitation(
            &invitation_service_url,
            invitation_token,
            allow_open_registration,
        )
        .await?;

        if !is_valid {
            return Err(AppError::Validation("Invalid invitation token".to_string()));
        }

        tracing::info!("Invitation token validated successfully");
    } else if let Some(ref invitation_token) = req.invitation_token {
        // Dev mode with invitation token provided: validate it
        let is_valid = crate::services::invitation_client::validate_invitation(
            &invitation_service_url,
            invitation_token,
            allow_open_registration,
        )
        .await?;

        if is_valid {
            tracing::info!("Invitation token validated successfully (dev mode)");
        } else {
            tracing::warn!("Invalid invitation token in dev mode, continuing anyway");
        }
    } else {
        // Dev mode without invitation token
        tracing::warn!("Registration without invitation token (allow_open_registration=true)");
    }

    let pool = db.pool();

    // Check if username already exists in global registry
    let username_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM global.registry_username WHERE username = $1)",
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
            "SELECT EXISTS(SELECT 1 FROM global.registry_email WHERE email = $1)",
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
    let territory_table = format!("territory_{}.auth_users_core", req.territory);

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
        "INSERT INTO global.registry_username (username, user_id, territory_code, registered_at)
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
            "INSERT INTO global.registry_email (email, user_id, territory_code, registered_at)
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

    // Mark invitation as used (if provided)
    if let Some(ref invitation_token) = req.invitation_token {
        // Note: We do this AFTER user creation to ensure user exists even if this fails
        let _usage_result = crate::services::invitation_client::use_invitation(
            &invitation_service_url,
            invitation_token,
            user_id,
            None, // TODO: Extract IP from request
            None, // TODO: Extract user agent from request
            allow_open_registration,
        )
        .await;

        // Log result but don't fail registration if invitation service is unavailable
        match _usage_result {
            Ok(Some(usage)) => {
                tracing::info!(
                    "Invitation {} marked as used (remaining: {})",
                    usage.invitation_id,
                    usage.uses_remaining
                );
            }
            Ok(None) => {
                tracing::warn!("Failed to mark invitation as used (non-fatal)");
            }
            Err(e) => {
                tracing::error!("Error marking invitation as used: {}", e);
            }
        }
    }

    // Fetch user's badges for JWT claims
    // For newly registered users, this will typically be empty or just "code-of-conduct"
    let badges = fetch_user_badge_slugs(pool, user_id, &req.territory).await?;

    // Generate tokens
    let access_token = token_service.generate_access_token(user_id, &req.territory, badges)?;
    let refresh_token = TokenService::generate_refresh_token();

    // Hash the refresh token for storage (never store plain tokens)
    let token_hash = PasswordService::hash(&refresh_token)?;

    // Store refresh token
    let refresh_token_table = format!("territory_{}.auth_users_refresh_tokens", req.territory);
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

    // Publish user.registered event to NATS
    // Following NATS-EVENTS.md security best practices:
    // - No PII (email excluded)
    // - Use user_id reference instead of sensitive data
    let event = PlatformEvent::new(
        "user.registered".to_string(),
        Some(req.territory.clone()),
        UserRegisteredPayload {
            user_id,
            username: req.username.clone(),
            territory: req.territory.clone(),
        },
    );

    if let Err(e) = nats
        .publish("global.user.registered", serde_json::to_vec(&event)?)
        .await
    {
        // Log error but don't fail registration if NATS unavailable (graceful degradation)
        tracing::warn!(
            "Failed to publish user.registered event for user {}: {}",
            user_id,
            e
        );
    } else {
        tracing::info!(
            "Published global.user.registered event for user {}",
            user_id
        );
    }

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
    let territory_table = format!("territory_{}.auth_users_core", req.territory);

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

    // Fetch user's badges for JWT claims
    let badges = fetch_user_badge_slugs(pool, user_id, &req.territory).await?;

    // Generate tokens
    let access_token = token_service.generate_access_token(user_id, &req.territory, badges)?;
    let refresh_token = TokenService::generate_refresh_token();

    // Hash the refresh token for storage (never store plain tokens)
    let token_hash = PasswordService::hash(&refresh_token)?;

    // Store refresh token
    let refresh_token_table = format!("territory_{}.auth_users_refresh_tokens", req.territory);
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
        let refresh_token_table = format!("territory_{}.auth_users_refresh_tokens", territory);

        // Get all non-revoked, non-expired tokens for this territory
        // Skip territories that don't exist (table not found)
        let tokens = match sqlx::query(&format!(
            "SELECT id, token_hash, user_id, expires_at FROM {} WHERE revoked = FALSE AND expires_at > NOW()",
            refresh_token_table
        ))
        .fetch_all(pool)
        .await {
            Ok(tokens) => tokens,
            Err(_) => continue, // Skip this territory if table doesn't exist
        };

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

                // Fetch user's badges for JWT claims (refreshed with each token refresh)
                let badges = fetch_user_badge_slugs(pool, user_id, territory).await?;

                // Generate new access token
                let access_token =
                    token_service.generate_access_token(user_id, territory, badges)?;

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
        let refresh_token_table = format!("territory_{}.auth_users_refresh_tokens", territory);

        // Get all non-revoked tokens and check hash
        // Skip territories that don't exist (table not found)
        let tokens = match sqlx::query(&format!(
            "SELECT id, token_hash FROM {} WHERE revoked = FALSE",
            refresh_token_table
        ))
        .fetch_all(pool)
        .await
        {
            Ok(tokens) => tokens,
            Err(_) => continue, // Skip this territory if table doesn't exist
        };

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
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
    ),
    tag = "Health"
)]
#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "ok".to_string(),
        service: "auth-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Readiness check endpoint - verifies database connectivity
#[utoipa::path(
    get,
    path = "/api/v1/ready",
    responses(
        (status = 200, description = "Service is ready", body = HealthResponse),
        (status = 503, description = "Service is not ready"),
    ),
    tag = "Health"
)]
#[get("/ready")]
pub async fn ready(db: web::Data<Database>) -> Result<HttpResponse> {
    let pool = db.pool();

    // Test database connection
    match sqlx::query("SELECT 1").fetch_one(pool).await {
        Ok(_) => {
            let response = HealthResponse {
                status: "ready".to_string(),
                service: "auth-service".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            tracing::error!("Database connectivity check failed: {}", e);
            Err(AppError::Database(e))
        }
    }
}

/// Metrics endpoint - Prometheus format
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    responses(
        (status = 200, description = "Prometheus metrics", content_type = "text/plain"),
    ),
    tag = "Health"
)]
#[get("/metrics")]
pub async fn metrics(
    db: web::Data<Database>,
    collector: web::Data<shared_lib::MetricsCollector>,
) -> HttpResponse {
    let pool = db.pool();
    let pool_size = pool.size();
    let pool_idle = pool.num_idle();

    let metrics_text = collector.generate_prometheus_metrics(Some(pool_size), Some(pool_idle));

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_text)
}
