use actix_web::{get, put, web, HttpResponse};
use shared_lib::{AuthUser, Database, Result, ValidatedJson};
use uuid::Uuid;

use crate::models::ProfileResponse;
use crate::services::ProfileService;

/// Configure profile routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile")
            .service(get_own_profile)
            .service(update_own_profile)
            .service(get_profile_by_id),
    );
}

/// GET /api/v1/user/profile - Get authenticated user's profile
#[utoipa::path(
    get,
    path = "/api/v1/user/profile",
    tag = "profile",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = ProfileResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Profile not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("")]
async fn get_own_profile(auth: AuthUser, db: web::Data<Database>) -> Result<HttpResponse> {
    let profile = ProfileService::get_profile(auth.id, &auth.territory, db.pool()).await?;
    Ok(HttpResponse::Ok().json(profile))
}

/// PUT /api/v1/user/profile - Update authenticated user's profile
#[utoipa::path(
    put,
    path = "/api/v1/user/profile",
    tag = "profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = ProfileResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("")]
async fn update_own_profile(
    auth: AuthUser,
    body: ValidatedJson<UpdateProfileRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let req = body.into_inner();

    let profile = ProfileService::update_profile(
        auth.id,
        &auth.territory,
        req.display_name,
        req.avatar_url,
        req.bio,
        req.about,
        req.location,
        req.website,
        req.interests,
        req.skills,
        db.pool(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(profile))
}

/// GET /api/v1/user/profile/{id} - Get user profile by ID
#[utoipa::path(
    get,
    path = "/api/v1/user/profile/{id}",
    tag = "profile",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User profile retrieved successfully", body = ProfileResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/{id}")]
async fn get_profile_by_id(
    auth: AuthUser,
    path: web::Path<Uuid>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    // Use authenticated user's territory for now (cross-territory lookup future feature)
    let profile = ProfileService::get_profile(user_id, &auth.territory, db.pool()).await?;

    Ok(HttpResponse::Ok().json(profile))
}

// Request/Response models

#[derive(serde::Deserialize, validator::Validate, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "John Doe")]
    pub display_name: Option<String>,

    #[validate(url)]
    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,

    #[validate(length(max = 280))]
    #[schema(example = "Software developer and open source enthusiast")]
    pub bio: Option<String>,

    #[schema(example = "I'm passionate about building user-centric applications...")]
    pub about: Option<String>,

    #[validate(length(max = 100))]
    #[schema(example = "Copenhagen, Denmark")]
    pub location: Option<String>,

    #[validate(url)]
    #[schema(example = "https://example.com")]
    pub website: Option<String>,

    #[schema(example = json!(["rust", "web development", "open source"]))]
    pub interests: Option<Vec<String>>,

    #[schema(example = json!(["Rust", "TypeScript", "PostgreSQL"]))]
    pub skills: Option<Vec<String>>,
}
