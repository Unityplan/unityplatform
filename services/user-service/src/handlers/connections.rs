use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::connection::BlockUserRequest;
use crate::services::UserService;

/// Path parameter for user ID
#[derive(Deserialize)]
pub struct UserIdPath {
    user_id: Uuid,
}

/// Path parameters for connection operations
#[derive(Deserialize)]
pub struct ConnectionPath {
    user_id: Uuid,
    target_id: Uuid,
}

/// Response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// POST /api/connections/{user_id}/follow/{target_id}
/// Follow a user
pub async fn follow_user(
    path: web::Path<ConnectionPath>,
    service: web::Data<UserService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify it matches path.user_id
) -> Result<HttpResponse> {
    let follower_id = path.user_id;
    let following_id = path.target_id;

    // Prevent self-follow
    if follower_id == following_id {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Cannot follow yourself".to_string()),
        }));
    }

    // Check if target user is blocked
    match service.is_blocked(follower_id, following_id).await {
        Ok(true) => {
            return Ok(HttpResponse::Forbidden().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Cannot follow blocked user".to_string()),
            }));
        }
        Err(e) => {
            eprintln!("Database error checking block status: {}", e);
        }
        _ => {}
    }

    match service.follow_user(follower_id, following_id).await {
        Ok(connection) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(connection),
            error: None,
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to follow user".to_string()),
            }))
        }
    }
}

/// DELETE /api/connections/{user_id}/follow/{target_id}
/// Unfollow a user
pub async fn unfollow_user(
    path: web::Path<ConnectionPath>,
    service: web::Data<UserService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify it matches path.user_id
) -> Result<HttpResponse> {
    let follower_id = path.user_id;
    let following_id = path.target_id;

    match service.unfollow_user(follower_id, following_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some("Successfully unfollowed user"),
            error: None,
        })),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Connection not found".to_string()),
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to unfollow user".to_string()),
            }))
        }
    }
}

/// GET /api/connections/{user_id}/followers
/// Get followers of a user
pub async fn get_followers(
    path: web::Path<UserIdPath>,
    service: web::Data<UserService>,
) -> Result<HttpResponse> {
    let user_id = path.user_id;

    match service.get_followers(user_id).await {
        Ok(followers) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(followers),
            error: None,
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to get followers".to_string()),
            }))
        }
    }
}

/// GET /api/connections/{user_id}/following
/// Get users that a user is following
pub async fn get_following(
    path: web::Path<UserIdPath>,
    service: web::Data<UserService>,
) -> Result<HttpResponse> {
    let user_id = path.user_id;

    match service.get_following(user_id).await {
        Ok(following) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(following),
            error: None,
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to get following list".to_string()),
            }))
        }
    }
}

/// POST /api/connections/{user_id}/block/{target_id}
/// Block a user
pub async fn block_user(
    path: web::Path<ConnectionPath>,
    body: web::Json<BlockUserRequest>,
    service: web::Data<UserService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify it matches path.user_id
) -> Result<HttpResponse> {
    let blocker_id = path.user_id;
    let blocked_id = path.target_id;

    // Prevent self-block
    if blocker_id == blocked_id {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Cannot block yourself".to_string()),
        }));
    }

    match service
        .block_user(blocker_id, blocked_id, body.reason.clone())
        .await
    {
        Ok(block) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(block),
            error: None,
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to block user".to_string()),
            }))
        }
    }
}

/// DELETE /api/connections/{user_id}/block/{target_id}
/// Unblock a user
pub async fn unblock_user(
    path: web::Path<ConnectionPath>,
    service: web::Data<UserService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify it matches path.user_id
) -> Result<HttpResponse> {
    let blocker_id = path.user_id;
    let blocked_id = path.target_id;

    match service.unblock_user(blocker_id, blocked_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some("Successfully unblocked user"),
            error: None,
        })),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Block not found".to_string()),
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to unblock user".to_string()),
            }))
        }
    }
}

/// GET /api/connections/{user_id}/blocked
/// Get blocked users list (owner only)
pub async fn get_blocked_users(
    path: web::Path<UserIdPath>,
    service: web::Data<UserService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify it matches path.user_id
) -> Result<HttpResponse> {
    let user_id = path.user_id;

    match service.get_blocked_users(user_id).await {
        Ok(blocks) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(blocks),
            error: None,
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to get blocked users".to_string()),
            }))
        }
    }
}

/// Configure connection routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/connections/{user_id}")
            .route("/follow/{target_id}", web::post().to(follow_user))
            .route("/follow/{target_id}", web::delete().to(unfollow_user))
            .route("/followers", web::get().to(get_followers))
            .route("/following", web::get().to(get_following))
            .route("/block/{target_id}", web::post().to(block_user))
            .route("/block/{target_id}", web::delete().to(unblock_user))
            .route("/blocked", web::get().to(get_blocked_users)),
    );
}
