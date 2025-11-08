use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Result};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::StorageService;

/// Path parameter for user ID
#[derive(Deserialize)]
pub struct UserIdPath {
    user_id: Uuid,
}

/// Query parameters for avatar size
#[derive(Deserialize)]
pub struct AvatarQuery {
    size: Option<String>, // thumbnail, small, medium, large (default)
}

/// Response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Avatar upload response
#[derive(Serialize)]
pub struct AvatarUploadResponse {
    pub avatar_url: String,
}

/// POST /api/avatars/{user_id}
/// Upload user avatar (owner only)
pub async fn upload_avatar(
    path: web::Path<UserIdPath>,
    mut payload: Multipart,
    storage: web::Data<StorageService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify ownership
) -> Result<HttpResponse> {
    let user_id = path.user_id;

    // Extract file from multipart
    let mut file_data: Option<Vec<u8>> = None;

    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Check if this is the file field
        let content_disposition = field.content_disposition();
        if content_disposition.get_name() == Some("avatar") {
            let mut data = Vec::new();

            // Read file data
            while let Some(chunk) = field.next().await {
                let chunk = chunk?;
                data.extend_from_slice(&chunk);

                // Check file size limit
                if data.len() > StorageService::MAX_FILE_SIZE {
                    return Ok(HttpResponse::BadRequest().json(ApiResponse::<()> {
                        success: false,
                        data: None,
                        error: Some(format!(
                            "File too large. Maximum size is {} MB",
                            StorageService::MAX_FILE_SIZE / (1024 * 1024)
                        )),
                    }));
                }
            }

            file_data = Some(data);
            break;
        }
    }

    // Validate we got file data
    let data = match file_data {
        Some(d) => d,
        None => {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("No avatar file provided".to_string()),
            }));
        }
    };

    // Validate image format
    let format = match StorageService::validate_image(&data) {
        Ok(f) => f,
        Err(e) => {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some(e.to_string()),
            }));
        }
    };

    // Save avatar with multiple sizes
    match storage.save_avatar(user_id, data.into(), format).await {
        Ok(avatar_url) => {
            // TODO: Update territory.users.avatar_url in database

            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(AvatarUploadResponse { avatar_url }),
                error: None,
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(format!("Failed to save avatar: {}", e)),
        })),
    }
}

/// GET /api/avatars/{user_id}
/// Get user avatar (public)
pub async fn get_avatar(
    path: web::Path<UserIdPath>,
    query: web::Query<AvatarQuery>,
    storage: web::Data<StorageService>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    let user_id = path.user_id;
    let size = query.size.as_deref();

    let avatar_path = storage.get_avatar_path(user_id, size);

    if !avatar_path.exists() {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Avatar not found".to_string()),
        }));
    }

    // Determine content type from file extension
    let content_type = match avatar_path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    };

    // Serve the file
    Ok(actix_files::NamedFile::open(avatar_path)?
        .set_content_type(content_type.parse().unwrap())
        .into_response(&req))
}

/// DELETE /api/avatars/{user_id}
/// Delete user avatar (owner only)
pub async fn delete_avatar(
    path: web::Path<UserIdPath>,
    storage: web::Data<StorageService>,
    // TODO: Add auth middleware to extract authenticated user_id and verify ownership
) -> Result<HttpResponse> {
    let user_id = path.user_id;

    match storage.delete_avatar(user_id).await {
        Ok(_) => {
            // TODO: Update territory.users.avatar_url to NULL in database

            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some("Avatar deleted successfully"),
                error: None,
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(format!("Failed to delete avatar: {}", e)),
        })),
    }
}

/// Configure avatar routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/avatars")
            .route("/{user_id}", web::post().to(upload_avatar))
            .route("/{user_id}", web::get().to(get_avatar))
            .route("/{user_id}", web::delete().to(delete_avatar)),
    );
}
