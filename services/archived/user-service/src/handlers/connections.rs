use actix_web::{web, HttpResponse};
use shared_lib::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{
    ConnectionStatus, ConnectionType, UserConnection, UserConnectionWithProfile,
};
use crate::response::ApiResponse;

/// Follow a user
///
/// Creates a 'follow' connection from the requesting user to the target user.
/// Automatically unfollows if already following (idempotent).
#[utoipa::path(
    post,
    path = "/v1/users/{id}/connections/follow/{target_id}",
    tag = "user-connections",
    params(
        ("id" = Uuid, Path, description = "User ID (must be authenticated user)"),
        ("target_id" = Uuid, Path, description = "Target user ID to follow")
    ),
    responses(
        (status = 201, description = "User followed successfully", body = ApiResponse<UserConnection>),
        (status = 400, description = "Cannot follow yourself"),
        (status = 404, description = "User not found"),
        (status = 409, description = "Already following this user")
    )
)]
pub async fn follow_user(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, target_user_id) = path.into_inner();

    // Prevent self-follow
    if user_id == target_user_id {
        return Err(AppError::Validation(
            "Cannot follow yourself".to_string(),
        ));
    }

    // Verify target user exists
    let target_exists = sqlx::query!("SELECT id FROM territory_dk.users WHERE id = $1", target_user_id)
        .fetch_optional(pool.as_ref())
        .await?;

    if target_exists.is_none() {
        return Err(AppError::NotFound("Target user not found".to_string()));
    }

    // Check if already following
    let existing = sqlx::query_as!(
        UserConnection,
        r#"
        SELECT 
            user_id, 
            target_user_id, 
            connection_type as "connection_type: ConnectionType",
            status as "status: ConnectionStatus",
            created_at, 
            updated_at
        FROM territory_dk.user_connections
        WHERE user_id = $1 
        AND target_user_id = $2 
        AND connection_type = 'follow'
        AND status = 'active'
        "#,
        user_id,
        target_user_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    if existing.is_some() {
        return Err(AppError::Validation(
            "Already following this user".to_string(),
        ));
    }

    // Create follow connection
    let connection = sqlx::query_as!(
        UserConnection,
        r#"
        INSERT INTO territory_dk.user_connections 
        (user_id, target_user_id, connection_type, status)
        VALUES ($1, $2, 'follow', 'active')
        RETURNING 
            user_id, 
            target_user_id, 
            connection_type as "connection_type: ConnectionType",
            status as "status: ConnectionStatus",
            created_at, 
            updated_at
        "#,
        user_id,
        target_user_id
    )
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(connection)))
}

/// Unfollow a user
///
/// Removes the 'follow' connection from the requesting user to the target user.
/// Idempotent - returns success even if not following.
#[utoipa::path(
    delete,
    path = "/v1/users/{id}/connections/follow/{target_id}",
    tag = "user-connections",
    params(
        ("id" = Uuid, Path, description = "User ID (must be authenticated user)"),
        ("target_id" = Uuid, Path, description = "Target user ID to unfollow")
    ),
    responses(
        (status = 200, description = "User unfollowed successfully"),
        (status = 400, description = "Cannot unfollow yourself")
    )
)]
pub async fn unfollow_user(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, target_user_id) = path.into_inner();

    // Prevent self-unfollow
    if user_id == target_user_id {
        return Err(AppError::Validation(
            "Cannot unfollow yourself".to_string(),
        ));
    }

    // Delete follow connection (idempotent)
    sqlx::query!(
        r#"
        DELETE FROM territory_dk.user_connections
        WHERE user_id = $1 
        AND target_user_id = $2 
        AND connection_type = 'follow'
        "#,
        user_id,
        target_user_id
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "User unfollowed successfully".to_string(),
    )))
}

/// Get followers list
///
/// Returns all users who are following the specified user.
#[utoipa::path(
    get,
    path = "/v1/users/{id}/connections/followers",
    tag = "user-connections",
    params(
        ("id" = Uuid, Path, description = "User ID to get followers for")
    ),
    responses(
        (status = 200, description = "Followers retrieved successfully", body = ApiResponse<Vec<UserConnectionWithProfile>>),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_followers(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Verify user exists
    let user_exists = sqlx::query!("SELECT id FROM territory_dk.users WHERE id = $1", user_id)
        .fetch_optional(pool.as_ref())
        .await?;

    if user_exists.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Get all followers with their profile info
    let followers = sqlx::query_as!(
        UserConnectionWithProfile,
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            p.display_name,
            p.avatar_url,
            uc.connection_type as "connection_type: ConnectionType",
            uc.status as "status: ConnectionStatus",
            uc.created_at
        FROM territory_dk.user_connections uc
        JOIN territory_dk.users u ON u.id = uc.user_id
        LEFT JOIN territory_dk.users_profiles p ON p.user_id = u.id
        WHERE uc.target_user_id = $1
        AND uc.connection_type = 'follow'
        AND uc.status = 'active'
        ORDER BY uc.created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(followers)))
}

/// Get following list
///
/// Returns all users that the specified user is following.
#[utoipa::path(
    get,
    path = "/v1/users/{id}/connections/following",
    tag = "user-connections",
    params(
        ("id" = Uuid, Path, description = "User ID to get following list for")
    ),
    responses(
        (status = 200, description = "Following list retrieved successfully", body = ApiResponse<Vec<UserConnectionWithProfile>>),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_following(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Verify user exists
    let user_exists = sqlx::query!("SELECT id FROM territory_dk.users WHERE id = $1", user_id)
        .fetch_optional(pool.as_ref())
        .await?;

    if user_exists.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Get all users being followed with their profile info
    let following = sqlx::query_as!(
        UserConnectionWithProfile,
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            p.display_name,
            p.avatar_url,
            uc.connection_type as "connection_type: ConnectionType",
            uc.status as "status: ConnectionStatus",
            uc.created_at
        FROM territory_dk.user_connections uc
        JOIN territory_dk.users u ON u.id = uc.target_user_id
        LEFT JOIN territory_dk.users_profiles p ON p.user_id = u.id
        WHERE uc.user_id = $1
        AND uc.connection_type = 'follow'
        AND uc.status = 'active'
        ORDER BY uc.created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(following)))
}

/// Block a user
///
/// Creates a 'block' connection. Automatically unfollows the target user if following.
/// Prevents the target user from following you.
#[utoipa::path(
    post,
    path = "/v1/users/{id}/connections/block/{target_id}",
    tag = "user-connections",
    params(
        ("id" = Uuid, Path, description = "User ID (must be authenticated user)"),
        ("target_id" = Uuid, Path, description = "Target user ID to block")
    ),
    responses(
        (status = 201, description = "User blocked successfully", body = ApiResponse<UserConnection>),
        (status = 400, description = "Cannot block yourself"),
        (status = 404, description = "User not found"),
        (status = 409, description = "Already blocked this user")
    )
)]
pub async fn block_user(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, target_user_id) = path.into_inner();

    // Prevent self-block
    if user_id == target_user_id {
        return Err(AppError::Validation(
            "Cannot block yourself".to_string(),
        ));
    }

    // Verify target user exists
    let target_exists = sqlx::query!("SELECT id FROM territory_dk.users WHERE id = $1", target_user_id)
        .fetch_optional(pool.as_ref())
        .await?;

    if target_exists.is_none() {
        return Err(AppError::NotFound("Target user not found".to_string()));
    }

    // Check if already blocked
    let existing = sqlx::query_as!(
        UserConnection,
        r#"
        SELECT 
            user_id, 
            target_user_id, 
            connection_type as "connection_type: ConnectionType",
            status as "status: ConnectionStatus",
            created_at, 
            updated_at
        FROM territory_dk.user_connections
        WHERE user_id = $1 
        AND target_user_id = $2 
        AND connection_type = 'block'
        AND status = 'active'
        "#,
        user_id,
        target_user_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    if existing.is_some() {
        return Err(AppError::Validation("Already blocked this user".to_string()));
    }

    // Begin transaction to unfollow + block atomically
    let mut tx = pool.begin().await?;

    // Remove any follow connections (both directions)
    sqlx::query!(
        r#"
        DELETE FROM territory_dk.user_connections
        WHERE ((user_id = $1 AND target_user_id = $2) OR (user_id = $2 AND target_user_id = $1))
        AND connection_type = 'follow'
        "#,
        user_id,
        target_user_id
    )
    .execute(&mut *tx)
    .await?;

    // Create block connection
    let connection = sqlx::query_as!(
        UserConnection,
        r#"
        INSERT INTO territory_dk.user_connections 
        (user_id, target_user_id, connection_type, status)
        VALUES ($1, $2, 'block', 'active')
        RETURNING 
            user_id, 
            target_user_id, 
            connection_type as "connection_type: ConnectionType",
            status as "status: ConnectionStatus",
            created_at, 
            updated_at
        "#,
        user_id,
        target_user_id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(connection)))
}

/// Unblock a user
///
/// Removes the 'block' connection from the requesting user to the target user.
/// Idempotent - returns success even if not blocked.
#[utoipa::path(
    delete,
    path = "/v1/users/{id}/connections/block/{target_id}",
    tag = "user-connections",
    params(
        ("id" = Uuid, Path, description = "User ID (must be authenticated user)"),
        ("target_id" = Uuid, Path, description = "Target user ID to unblock")
    ),
    responses(
        (status = 200, description = "User unblocked successfully"),
        (status = 400, description = "Cannot unblock yourself")
    )
)]
pub async fn unblock_user(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, target_user_id) = path.into_inner();

    // Prevent self-unblock
    if user_id == target_user_id {
        return Err(AppError::Validation(
            "Cannot unblock yourself".to_string(),
        ));
    }

    // Delete block connection (idempotent)
    sqlx::query!(
        r#"
        DELETE FROM territory_dk.user_connections
        WHERE user_id = $1 
        AND target_user_id = $2 
        AND connection_type = 'block'
        "#,
        user_id,
        target_user_id
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        "User unblocked successfully".to_string(),
    )))
}

/// Get blocked users list
///
/// Returns all users that the specified user has blocked.
#[utoipa::path(
    get,
    path = "/v1/users/{id}/connections/blocked",
    tag = "user-connections",
    params(
        ("id" = Uuid, Path, description = "User ID to get blocked users for")
    ),
    responses(
        (status = 200, description = "Blocked users retrieved successfully", body = ApiResponse<Vec<UserConnectionWithProfile>>),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_blocked_users(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Verify user exists
    let user_exists = sqlx::query!("SELECT id FROM territory_dk.users WHERE id = $1", user_id)
        .fetch_optional(pool.as_ref())
        .await?;

    if user_exists.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Get all blocked users with their profile info
    let blocked = sqlx::query_as!(
        UserConnectionWithProfile,
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            p.display_name,
            p.avatar_url,
            uc.connection_type as "connection_type: ConnectionType",
            uc.status as "status: ConnectionStatus",
            uc.created_at
        FROM territory_dk.user_connections uc
        JOIN territory_dk.users u ON u.id = uc.target_user_id
        LEFT JOIN territory_dk.users_profiles p ON p.user_id = u.id
        WHERE uc.user_id = $1
        AND uc.connection_type = 'block'
        AND uc.status = 'active'
        ORDER BY uc.created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(blocked)))
}
