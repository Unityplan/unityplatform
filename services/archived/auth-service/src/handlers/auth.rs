use crate::{
    error::AuthError,
    models::{
        LoginRequest, LoginResponse, RefreshRequest, RefreshResponse, RegisterRequest,
        RegisterResponse, UserInfo, ValidateResponse,
    },
    services::{
        get_token_territory, hash_refresh_token, use_invitation_token, validate_invitation_token,
        PasswordService, TokenService,
    },
    ApiResponse,
};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

fn get_schema_name(territory_code: &str) -> String {
    format!("territory_{}", territory_code.to_lowercase())
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register(
    req: web::Json<RegisterRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    req.validate()
        .map_err(|e| AuthError::ValidationError(e.to_string()))?;

    let territory_code = get_token_territory(pool.get_ref(), &req.invitation_token).await?;

    let territory_active = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM global.territories WHERE code = $1 AND status = 'active')",
    )
    .bind(&territory_code)
    .fetch_one(pool.get_ref())
    .await?;

    if !territory_active {
        return Err(AuthError::BadRequest("Territory not available".to_string()));
    }

    let schema_name = get_schema_name(&territory_code);

    let invitation = validate_invitation_token(
        pool.get_ref(),
        &schema_name,
        &req.invitation_token,
        req.email.as_deref(),
    )
    .await?;

    let username_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM global.username_registry WHERE LOWER(username) = LOWER($1))",
    )
    .bind(&req.username)
    .fetch_one(pool.get_ref())
    .await?;

    if username_exists {
        return Err(AuthError::UsernameExists);
    }

    if let Some(email) = &req.email {
        let email_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM global.email_registry WHERE LOWER(email) = LOWER($1))",
        )
        .bind(email)
        .fetch_one(pool.get_ref())
        .await?;

        if email_exists {
            return Err(AuthError::EmailExists);
        }
    }

    let password_hash = PasswordService::hash(&req.password)?;

    let mut tx = pool.begin().await?;

    let user_id = Uuid::new_v4();
    let insert_user_query = format!(
        r#"
        INSERT INTO {}.users (
            id, username, email, password_hash, full_name
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id as user_id, username, email, territory_code as home_territory_code, created_at
        "#,
        schema_name
    );

    let user = sqlx::query_as::<_, RegisterResponse>(&insert_user_query)
        .bind(user_id)
        .bind(&req.username)
        .bind(&req.email)
        .bind(&password_hash)
        .bind(&req.display_name)
        .fetch_one(&mut *tx)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO global.username_registry (username, user_id, territory_code)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(&req.username)
    .bind(user_id)
    .bind(&territory_code)
    .execute(&mut *tx)
    .await?;

    if let Some(email) = &req.email {
        sqlx::query(
            r#"
            INSERT INTO global.email_registry (email, user_id, territory_code)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(email)
        .bind(user_id)
        .bind(&territory_code)
        .execute(&mut *tx)
        .await?;
    }

    use_invitation_token(&mut tx, &schema_name, invitation.id, user_id).await?;

    tx.commit().await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(user)))
}

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login(
    req: web::Json<LoginRequest>,
    pool: web::Data<PgPool>,
    token_service: web::Data<TokenService>,
) -> Result<HttpResponse, AuthError> {
    req.validate()
        .map_err(|e| AuthError::ValidationError(e.to_string()))?;

    let user_info = sqlx::query_as::<_, (Uuid, String)>(
        r#"
        SELECT user_id, territory_code
        FROM global.username_registry
        WHERE LOWER(username) = LOWER($1)
        "#,
    )
    .bind(&req.username)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(AuthError::InvalidCredentials)?;

    let (user_id, territory_code) = user_info;
    let schema_name = get_schema_name(&territory_code);

    let query = format!(
        r#"
        SELECT id, username, email, password_hash, full_name, 
               is_active, is_verified
        FROM {}.users
        WHERE id = $1
        "#,
        schema_name
    );

    let user = sqlx::query_as::<_, UserRecord>(&query)
        .bind(user_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    if !user.is_active {
        return Err(AuthError::Unauthorized);
    }

    let password_valid = PasswordService::verify(&req.password, &user.password_hash)?;
    if !password_valid {
        return Err(AuthError::InvalidCredentials);
    }

    let access_token =
        token_service.generate_access_token(user.id, &user.username, &territory_code)?;
    let refresh_token = token_service.generate_refresh_token();
    let refresh_token_hash = hash_refresh_token(&refresh_token);

    let expires_at = Utc::now() + chrono::Duration::seconds(token_service.get_refresh_token_ttl());
    let insert_refresh_query = format!(
        r#"
        INSERT INTO {}.refresh_tokens (user_id, token, expires_at)
        VALUES ($1, $2, $3)
        "#,
        schema_name
    );

    sqlx::query(&insert_refresh_query)
        .bind(user.id)
        .bind(&refresh_token_hash)
        .bind(expires_at)
        .execute(pool.get_ref())
        .await?;

    let response = LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: token_service.get_access_token_ttl(),
        user: UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            is_verified: user.is_verified,
        },
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    tag = "Authentication",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = RefreshResponse),
        (status = 401, description = "Invalid refresh token"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn refresh(
    req: web::Json<RefreshRequest>,
    pool: web::Data<PgPool>,
    token_service: web::Data<TokenService>,
) -> Result<HttpResponse, AuthError> {
    let refresh_token_hash = hash_refresh_token(&req.refresh_token);

    let territories = sqlx::query_scalar::<_, String>(
        "SELECT code FROM global.territories WHERE status = 'active'",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let mut token_found = None;

    for territory_code in territories {
        let schema_name = get_schema_name(&territory_code);
        let query = format!(
            r#"
            SELECT user_id, expires_at
            FROM {}.refresh_tokens
            WHERE token = $1 AND revoked_at IS NULL
            "#,
            schema_name
        );

        if let Some(token_record) = sqlx::query_as::<_, RefreshTokenRecord>(&query)
            .bind(&refresh_token_hash)
            .fetch_optional(pool.get_ref())
            .await?
        {
            token_found = Some((token_record, territory_code));
            break;
        }
    }

    let (token_record, territory_code) = token_found
        .ok_or_else(|| AuthError::InvalidToken("Refresh token not found".to_string()))?;

    if Utc::now() > token_record.expires_at {
        return Err(AuthError::TokenExpired);
    }

    let schema_name = get_schema_name(&territory_code);

    let query = format!(
        r#"
        SELECT username
        FROM {}.users
        WHERE id = $1 AND is_active = true
        "#,
        schema_name
    );

    let username = sqlx::query_scalar::<_, String>(&query)
        .bind(token_record.user_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AuthError::InvalidToken("User not found".to_string()))?;

    let access_token =
        token_service.generate_access_token(token_record.user_id, &username, &territory_code)?;

    let response = RefreshResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: token_service.get_access_token_ttl(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    tag = "Authentication",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Logout successful"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn logout(
    req: web::Json<RefreshRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    let refresh_token_hash = hash_refresh_token(&req.refresh_token);

    let territories = sqlx::query_scalar::<_, String>(
        "SELECT code FROM global.territories WHERE status = 'active'",
    )
    .fetch_all(pool.get_ref())
    .await?;

    for territory_code in territories {
        let schema_name = get_schema_name(&territory_code);
        let query = format!(
            r#"
            UPDATE {}.refresh_tokens
            SET revoked_at = NOW()
            WHERE token = $1 AND revoked_at IS NULL
            "#,
            schema_name
        );

        sqlx::query(&query)
            .bind(&refresh_token_hash)
            .execute(pool.get_ref())
            .await?;
    }

    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(())))
}

#[utoipa::path(
    get,
    path = "/v1/auth/validate",
    tag = "Authentication",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Token is valid", body = ValidateResponse),
        (status = 401, description = "Invalid token"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn validate_token(
    req: HttpRequest,
    token_service: web::Data<TokenService>,
) -> Result<HttpResponse, AuthError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::Unauthorized)?;

    let claims = token_service.validate_token(token)?;

    let response = ValidateResponse {
        valid: true,
        user_id: Uuid::parse_str(&claims.sub)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?,
        username: claims.username,
        territory: claims.territory,
        expires_at: chrono::DateTime::from_timestamp(claims.exp as i64, 0)
            .ok_or_else(|| AuthError::InvalidToken("Invalid expiration timestamp".to_string()))?,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

#[derive(sqlx::FromRow)]
struct UserRecord {
    id: Uuid,
    username: String,
    email: Option<String>,
    password_hash: String,
    full_name: String,
    is_active: bool,
    is_verified: bool,
}

#[derive(sqlx::FromRow)]
struct RefreshTokenRecord {
    user_id: Uuid,
    expires_at: chrono::DateTime<chrono::Utc>,
}
