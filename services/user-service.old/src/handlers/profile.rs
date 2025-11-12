use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::profile::UpdateProfileRequest;
use crate::services::UserService;

/// Path parameter for user ID
#[derive(Deserialize)]
pub struct UserIdPath {
    user_id: Uuid,
}

/// Query parameters for profile viewing
#[derive(Deserialize)]
pub struct ProfileQuery {
    viewer_id: Option<Uuid>,
}

/// Response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// GET /api/profiles/{user_id}
/// Get user profile (applies privacy rules based on viewer)
pub async fn get_profile(
    path: web::Path<UserIdPath>,
    query: web::Query<ProfileQuery>,
    service: web::Data<UserService>,
) -> Result<HttpResponse> {
    let user_id = path.user_id;
    let viewer_id = query.viewer_id;

    match service.get_public_profile(user_id, viewer_id).await {
        Ok(Some(profile)) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(profile),
            error: None,
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Profile not found or private".to_string()),
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Internal server error".to_string()),
            }))
        }
    }
}

/// GET /api/profiles/{user_id}/full
/// Get full profile (owner only)
pub async fn get_full_profile(
    path: web::Path<UserIdPath>,
    service: web::Data<UserService>,
    // TODO: Add auth middleware to extract authenticated user_id
) -> Result<HttpResponse> {
    let user_id = path.user_id;

    match service.get_profile(user_id).await {
        Ok(Some(profile)) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(profile),
            error: None,
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Profile not found".to_string()),
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Internal server error".to_string()),
            }))
        }
    }
}

/// PUT /api/profiles/{user_id}
/// Update user profile (owner only)
pub async fn update_profile(
    path: web::Path<UserIdPath>,
    body: web::Json<UpdateProfileRequest>,
    service: web::Data<UserService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify ownership
) -> Result<HttpResponse> {
    let user_id = path.user_id;
    let request = body.into_inner();

    // Validate request
    if let Err(e) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(format!("Validation error: {}", e)),
        }));
    }

    match service.update_profile(user_id, request).await {
        Ok(profile) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(profile),
            error: None,
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to update profile".to_string()),
            }))
        }
    }
}

/// DELETE /api/profiles/{user_id}
/// Delete user profile (sets all fields to NULL, owner only)
pub async fn delete_profile(
    path: web::Path<UserIdPath>,
    service: web::Data<UserService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify ownership
) -> Result<HttpResponse> {
    let user_id = path.user_id;

    // Create empty update request to clear all profile fields
    let empty_request = UpdateProfileRequest {
        about: Some(None),
        interests: Some(vec![]),
        skills: Some(vec![]),
        languages: Some(vec![]),
        location: Some(None),
        website_url: Some(None),
        github_url: Some(None),
        linkedin_url: Some(None),
        twitter_handle: Some(None),
        theme: Some("light".to_string()),
        metadata: Some(serde_json::json!({})),
        profile_visibility: Some("public".to_string()),
        show_email: Some(false),
        show_real_name: Some(true),
        allow_messages_from: Some("everyone".to_string()),
    };

    match service.update_profile(user_id, empty_request).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some("Profile cleared successfully"),
            error: None,
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to delete profile".to_string()),
            }))
        }
    }
}

/// Configure profile routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profiles")
            .route("/{user_id}", web::get().to(get_profile))
            .route("/{user_id}/full", web::get().to(get_full_profile))
            .route("/{user_id}", web::put().to(update_profile))
            .route("/{user_id}", web::delete().to(delete_profile)),
    );
}
