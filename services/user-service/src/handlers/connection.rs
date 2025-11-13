use actix_web::{web, HttpResponse};
use serde::Deserialize;
use shared_lib::{jwt::AuthUser, Database, Result};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::models::{ConnectionsListResponse, UserSearchResponse};
use crate::services::ConnectionService;

/// Follow a user
#[utoipa::path(
    post,
    path = "/api/v1/users/{id}/follow",
    tag = "connections",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Target user ID to follow")
    ),
    responses(
        (status = 204, description = "User followed successfully"),
        (status = 400, description = "Validation error (cannot follow yourself, user is blocked)"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    )
)]
async fn follow_user(
    auth: AuthUser,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let target_user_id = path.into_inner();

    ConnectionService::follow_user(&db, &auth.territory, auth.id, target_user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Unfollow a user
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}/follow",
    tag = "connections",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Target user ID to unfollow")
    ),
    responses(
        (status = 204, description = "User unfollowed successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Follow connection not found")
    )
)]
async fn unfollow_user(
    auth: AuthUser,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let target_user_id = path.into_inner();

    ConnectionService::unfollow_user(&db, &auth.territory, auth.id, target_user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Block a user
#[utoipa::path(
    post,
    path = "/api/v1/users/{id}/block",
    tag = "connections",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Target user ID to block")
    ),
    responses(
        (status = 204, description = "User blocked successfully (mutual follows removed)"),
        (status = 400, description = "Validation error (cannot block yourself)"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    )
)]
async fn block_user(
    auth: AuthUser,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let target_user_id = path.into_inner();

    ConnectionService::block_user(&db, &auth.territory, auth.id, target_user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Unblock a user
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}/block",
    tag = "connections",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Target user ID to unblock")
    ),
    responses(
        (status = 204, description = "User unblocked successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Block connection not found")
    )
)]
async fn unblock_user(
    auth: AuthUser,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let target_user_id = path.into_inner();

    ConnectionService::unblock_user(&db, &auth.territory, auth.id, target_user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Query parameters for listing connections
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
struct ConnectionListQuery {
    /// Maximum number of results per page (default: 20, max: 100)
    #[serde(default = "default_limit")]
    limit: i64,
    /// Offset for pagination (default: 0)
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    20
}

/// Get followers of a user
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}/followers",
    tag = "connections",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "User ID to get followers for"),
        ConnectionListQuery
    ),
    responses(
        (status = 200, description = "List of followers", body = ConnectionsListResponse),
        (status = 401, description = "Unauthorized")
    )
)]
async fn get_followers(
    auth: AuthUser,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
    query: web::Query<ConnectionListQuery>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let limit = query.limit.min(100).max(1);
    let offset = query.offset.max(0);

    let response =
        ConnectionService::get_followers(&db, &auth.territory, user_id, limit, offset).await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Get users that a user is following
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}/following",
    tag = "connections",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "User ID to get following list for"),
        ConnectionListQuery
    ),
    responses(
        (status = 200, description = "List of users being followed", body = ConnectionsListResponse),
        (status = 401, description = "Unauthorized")
    )
)]
async fn get_following(
    auth: AuthUser,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
    query: web::Query<ConnectionListQuery>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let limit = query.limit.min(100).max(1);
    let offset = query.offset.max(0);

    let response =
        ConnectionService::get_following(&db, &auth.territory, user_id, limit, offset).await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Query parameters for user search
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
struct UserSearchQuery {
    /// Search query (username or display name)
    q: String,
    /// Maximum number of results per page (default: 20, max: 100)
    #[serde(default = "default_limit")]
    limit: i64,
    /// Offset for pagination (default: 0)
    #[serde(default)]
    offset: i64,
}

/// Search for users
#[utoipa::path(
    get,
    path = "/api/v1/users/search",
    tag = "connections",
    security(("bearer_auth" = [])),
    params(
        UserSearchQuery
    ),
    responses(
        (status = 200, description = "Search results with connection status", body = UserSearchResponse),
        (status = 400, description = "Validation error (empty search query)"),
        (status = 401, description = "Unauthorized")
    )
)]
async fn search_users(
    auth: AuthUser,
    db: web::Data<Database>,
    query: web::Query<UserSearchQuery>,
) -> Result<HttpResponse> {
    let search_query = query.q.trim();

    if search_query.is_empty() {
        return Err(shared_lib::AppError::Validation(
            "Search query cannot be empty".to_string(),
        ));
    }

    let limit = query.limit.min(100).max(1);
    let offset = query.offset.max(0);

    let response =
        ConnectionService::search_users(&db, &auth.territory, auth.id, search_query, limit, offset)
            .await?;

    Ok(HttpResponse::Ok().json(response))
}

/// Configure connection routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/search", web::get().to(search_users))
        .route("/{id}/follow", web::post().to(follow_user))
        .route("/{id}/follow", web::delete().to(unfollow_user))
        .route("/{id}/block", web::post().to(block_user))
        .route("/{id}/block", web::delete().to(unblock_user))
        .route("/{id}/followers", web::get().to(get_followers))
        .route("/{id}/following", web::get().to(get_following));
}
