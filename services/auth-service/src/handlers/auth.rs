use crate::{
    models::{user::User, AuthResponse, AuthUserInfo, LoginRequest, RegisterRequest},
    services::{use_invitation_token, validate_invitation_token, PasswordService, TokenService},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sha2::Digest;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use validator::Validate;

// Helper struct for territory validation
#[derive(FromRow)]
struct TerritoryCode {
    code: String,
}

/// Get schema name for a territory
/// For single-territory pods: returns "territory"
/// For multi-territory pods: returns "territory_XX" (e.g., "territory_de")
fn get_schema_name(_territory_code: &str) -> String {
    // TODO: Make this configurable via environment variable
    // For now, use single-territory approach (generic "territory" schema)
    "territory".to_string()

    // For multi-territory pods, use:
    // format!("territory_{}", territory_code.to_lowercase())
}

/// Register a new user
pub async fn register(
    req: web::Json<RegisterRequest>,
    pool: web::Data<PgPool>,
    token_service: web::Data<TokenService>,
) -> actix_web::Result<HttpResponse> {
    eprintln!("DEBUG: Register handler called");

    // Validate request
    req.validate().map_err(|e| {
        eprintln!("DEBUG: Validation failed: {}", e);
        actix_web::error::ErrorBadRequest(format!("Validation error: {}", e))
    })?;

    eprintln!("DEBUG: Validation passed, checking territory");

    // Verify territory exists and is active
    let territory = sqlx::query_as::<_, TerritoryCode>(
        "SELECT code FROM global.territories WHERE code = $1 AND is_active = true",
    )
    .bind(&req.territory_code)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("DEBUG: Territory query failed: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?
    .ok_or_else(|| {
        eprintln!("DEBUG: Territory not found");
        actix_web::error::ErrorBadRequest("Invalid territory code")
    })?;

    eprintln!("DEBUG: Territory found: {}", territory.code);

    // Set schema context to territory (dynamic based on territory_code)
    let schema_name = get_schema_name(&territory.code);

    // Validate invitation token
    let invitation = validate_invitation_token(
        pool.get_ref(),
        &schema_name,
        &req.invitation_token,
        req.email.as_deref(), // Pass Option<&str>
    )
    .await
    .map_err(actix_web::error::ErrorBadRequest)?;

    // Check if username is globally unique (across ALL territories/pods)
    let username_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM global.user_identities WHERE LOWER(username) = LOWER($1))",
    )
    .bind(&req.username.to_lowercase())
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if username_exists {
        return Err(actix_web::error::ErrorBadRequest(
            "Username already taken globally. Please choose another username.",
        ));
    }

    // Check if email already exists in territory (only if email provided)
    if let Some(ref email) = req.email {
        let existing_email = sqlx::query(&format!(
            "SELECT id FROM {}.users WHERE email = $1",
            schema_name
        ))
        .bind(email)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        if existing_email.is_some() {
            return Err(actix_web::error::ErrorBadRequest(
                "Email already registered in this territory",
            ));
        }
    }

    // Hash password
    let password_hash = PasswordService::hash_password(&req.password)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Create user in territory schema
    // Note: Database trigger will automatically create global.user_identities entry
    let user = sqlx::query_as::<_, User>(&format!(
        r#"
        INSERT INTO {}.users (
            username, email, password_hash, full_name, invitation_by_token_id
        ) VALUES ($1, $2, $3, $4, $5)
        RETURNING 
            id, email, password_hash, username, 
            full_name, display_name, avatar_url, bio, date_of_birth, phone,
            profile_visibility, email_notifications, push_notifications,
            is_verified, is_active, last_login_at,
            invited_by_user_id, invitation_by_token_id,
            created_at, updated_at
        "#,
        schema_name
    ))
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.full_name)
    .bind(invitation.id)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {:?}", e);
        actix_web::error::ErrorInternalServerError(format!("Failed to create user: {}", e))
    })?;

    eprintln!(
        "DEBUG: User created with ID: {}, fetching global identity created by trigger",
        user.id
    );

    // Fetch global identity ID (created automatically by database trigger)
    let global_identity: (Uuid, String) = sqlx::query_as(
        r#"
        SELECT id, public_key_hash 
        FROM global.user_identities 
        WHERE territory_code = $1 AND territory_user_id = $2
        "#,
    )
    .bind(&req.territory_code)
    .bind(user.id)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("DEBUG: Failed to fetch global identity: {:?}", e);
        actix_web::error::ErrorInternalServerError(format!(
            "Failed to fetch global identity: {}",
            e
        ))
    })?;

    let (global_identity_id, public_key_hash) = global_identity;

    eprintln!(
        "DEBUG: Global identity fetched successfully with ID: {}",
        global_identity_id
    );

    // Mark invitation as used
    use_invitation_token(pool.get_ref(), &schema_name, invitation.id, user.id, None)
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

    // Store refresh token in global.sessions table (using global identity ID)
    let refresh_token_hash = format!("{:x}", sha2::Sha256::digest(refresh_token.as_bytes()));
    let expires_at = Utc::now() + chrono::Duration::days(7);

    sqlx::query(
        "INSERT INTO global.sessions (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(global_identity_id)
    .bind(&refresh_token_hash)
    .bind(expires_at)
    .execute(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return tokens and user info
    Ok(HttpResponse::Created().json(serde_json::json!({
        "user": AuthUserInfo::from(user),
        "access_token": access_token,
        "refresh_token": refresh_token,
        "expires_in": token_service.get_access_token_ttl(),
    })))
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
    let schema_name = get_schema_name(&territory.code);

    // Find user by username (not email - privacy-first)
    let user = sqlx::query_as::<_, User>(&format!(
        r#"
        SELECT 
            id, email, password_hash, username, 
            full_name, display_name, avatar_url, bio, date_of_birth, phone,
            profile_visibility, email_notifications, push_notifications,
            is_verified, is_active, last_login_at,
            invited_by_user_id, invitation_by_token_id,
            created_at, updated_at
        FROM {}.users WHERE username = $1
        "#,
        schema_name
    ))
    .bind(&req.username)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    // Check if user is active
    if !user.is_active {
        return Err(actix_web::error::ErrorUnauthorized("Account is inactive"));
    }

    // Verify password
    let password_hash = &user.password_hash;

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

    // Get public_key_hash and global identity ID from global.user_identities
    let (public_key_hash, global_identity_id): (String, Uuid) = sqlx::query_as(
        "SELECT public_key_hash, id FROM global.user_identities WHERE territory_code = $1 AND territory_user_id = $2"
    )
    .bind(&req.territory_code)
    .bind(user.id)
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

    // Store refresh token in global.sessions (using global identity ID)
    let refresh_token_hash = format!("{:x}", sha2::Sha256::digest(refresh_token.as_bytes()));
    let expires_at = Utc::now() + chrono::Duration::days(7);

    sqlx::query(
        "INSERT INTO global.sessions (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(global_identity_id)
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

/// Get current authenticated user info
pub async fn me(
    req: actix_web::HttpRequest,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    // Get authenticated user from JWT middleware
    let auth_user = crate::middleware::get_authenticated_user(&req)?;

    // Set schema context to territory
    let schema_name = get_schema_name(&auth_user.territory_code);

    // Load full user profile from database
    let user = sqlx::query_as::<_, User>(&format!(
        r#"
        SELECT 
            id, email, password_hash, username, 
            full_name, display_name, avatar_url, bio, date_of_birth, phone,
            profile_visibility, email_notifications, push_notifications,
            is_verified, is_active, last_login_at,
            invited_by_user_id, invitation_by_token_id,
            created_at, updated_at
        FROM {}.users 
        WHERE id = $1
        "#,
        schema_name
    ))
    .bind(auth_user.user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    // Return user info respecting privacy settings
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": user.id,
        "username": user.username,
        "email": user.email, // Always show own email when viewing own profile
        "full_name": user.full_name,
        "display_name": user.display_name,
        "avatar_url": user.avatar_url,
        "bio": user.bio,
        "profile_visibility": user.profile_visibility,
        "email_notifications": user.email_notifications,
        "push_notifications": user.push_notifications,
        "is_verified": user.is_verified,
        "territory_code": auth_user.territory_code,
        "created_at": user.created_at,
        "last_login_at": user.last_login_at,
    })))
}

/// Refresh access token
pub async fn refresh(
    req: web::Json<crate::models::RefreshTokenRequest>,
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

    let schema_name = get_schema_name(&territory.code);

    // Hash the provided refresh token
    let token_hash = format!("{:x}", sha2::Sha256::digest(req.refresh_token.as_bytes()));

    // Query session from global.sessions
    #[derive(FromRow)]
    struct SessionRecord {
        user_id: uuid::Uuid,
        expires_at: chrono::DateTime<Utc>,
    }

    let session_record = sqlx::query_as::<_, SessionRecord>(
        "SELECT user_id, expires_at FROM global.sessions WHERE token_hash = $1",
    )
    .bind(&token_hash)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid refresh token"))?;

    // Check if token is expired
    if session_record.expires_at < Utc::now() {
        // Delete expired session
        sqlx::query("DELETE FROM global.sessions WHERE token_hash = $1")
            .bind(&token_hash)
            .execute(pool.get_ref())
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Err(actix_web::error::ErrorUnauthorized("Refresh token expired"));
    }

    // Get territory user ID and public_key_hash from global.user_identities
    #[derive(FromRow)]
    struct GlobalIdentity {
        territory_user_id: uuid::Uuid,
        public_key_hash: String,
    }

    let identity = sqlx::query_as::<_, GlobalIdentity>(
        "SELECT territory_user_id, public_key_hash FROM global.user_identities WHERE id = $1 AND territory_code = $2"
    )
    .bind(session_record.user_id)
    .bind(&req.territory_code)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid session"))?;

    // Load user from territory database
    let user = sqlx::query_as::<_, User>(&format!(
        r#"
        SELECT 
            id, email, password_hash, username, 
            full_name, display_name, avatar_url, bio, date_of_birth, phone,
            profile_visibility, email_notifications, push_notifications,
            is_verified, is_active, last_login_at,
            invited_by_user_id, invitation_by_token_id,
            created_at, updated_at
        FROM {}.users 
        WHERE id = $1 AND is_active = true
        "#,
        schema_name
    ))
    .bind(identity.territory_user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorUnauthorized("User not found or inactive"))?;

    // Generate new tokens
    let new_access_token = token_service
        .generate_access_token(
            &identity.public_key_hash,
            &req.territory_code,
            user.id,
            &user.username,
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let new_refresh_token = token_service.generate_refresh_token();
    let new_token_hash = format!("{:x}", sha2::Sha256::digest(new_refresh_token.as_bytes()));
    let new_expires_at = Utc::now() + chrono::Duration::days(7);

    // Delete old session and insert new one (token rotation) in global.sessions
    sqlx::query("DELETE FROM global.sessions WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    sqlx::query(
        "INSERT INTO global.sessions (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(session_record.user_id) // Use the global identity ID from the session
    .bind(&new_token_hash)
    .bind(new_expires_at)
    .execute(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return new tokens
    Ok(HttpResponse::Ok().json(AuthResponse {
        user: AuthUserInfo::from(user),
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_in: token_service.get_access_token_ttl(),
    }))
}

/// Logout user
pub async fn logout(
    req: web::Json<crate::models::LogoutRequest>,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    // Hash the provided refresh token
    let token_hash = format!("{:x}", sha2::Sha256::digest(req.refresh_token.as_bytes()));

    // Delete session from global.sessions
    let result = sqlx::query("DELETE FROM global.sessions WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if result.rows_affected() == 0 {
        return Err(actix_web::error::ErrorNotFound("Refresh token not found"));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    })))
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
