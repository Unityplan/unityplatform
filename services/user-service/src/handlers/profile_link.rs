use actix_web::{delete, get, post, put, web, HttpResponse};
use shared_lib::{AuthUser, Database, Result, ValidatedJson};
use uuid::Uuid;

use crate::models::{CreateProfileLinkRequest, ProfileLinkResponse, UpdateProfileLinkRequest};
use crate::services::ProfileLinkService;

/// GET /api/v1/profiles/me/links - List all profile links for authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/user/profile/links",
    tag = "profile-links",
    responses(
        (status = 200, description = "Profile links retrieved successfully", body = Vec<ProfileLinkResponse>),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("")]
async fn list_links(auth: AuthUser, db: web::Data<Database>) -> Result<HttpResponse> {
    let links = ProfileLinkService::get_user_links(auth.id, &auth.territory, db.pool()).await?;
    let response: Vec<ProfileLinkResponse> = links.into_iter().map(Into::into).collect();
    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/profiles/me/links - Create a new profile link
#[utoipa::path(
    post,
    path = "/api/v1/user/profile/links",
    tag = "profile-links",
    request_body = CreateProfileLinkRequest,
    responses(
        (status = 201, description = "Profile link created successfully", body = ProfileLinkResponse),
        (status = 400, description = "Invalid request or max links exceeded (10 max)"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("")]
async fn create_link(
    auth: AuthUser,
    body: ValidatedJson<CreateProfileLinkRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let link = ProfileLinkService::create_link(auth.id, &auth.territory, req, db.pool()).await?;
    let response: ProfileLinkResponse = link.into();
    Ok(HttpResponse::Created().json(response))
}

/// PUT /api/v1/profiles/me/links/:id - Update a profile link
#[utoipa::path(
    put,
    path = "/api/v1/user/profile/links/{id}",
    tag = "profile-links",
    params(
        ("id" = Uuid, Path, description = "Profile link ID")
    ),
    request_body = UpdateProfileLinkRequest,
    responses(
        (status = 200, description = "Profile link updated successfully", body = ProfileLinkResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Profile link not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/{id}")]
async fn update_link(
    auth: AuthUser,
    path: web::Path<Uuid>,
    body: ValidatedJson<UpdateProfileLinkRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let link_id = path.into_inner();
    let req = body.into_inner();
    let link =
        ProfileLinkService::update_link(link_id, auth.id, &auth.territory, req, db.pool()).await?;
    let response: ProfileLinkResponse = link.into();
    Ok(HttpResponse::Ok().json(response))
}

/// DELETE /api/v1/profiles/me/links/:id - Delete a profile link
#[utoipa::path(
    delete,
    path = "/api/v1/user/profile/links/{id}",
    tag = "profile-links",
    params(
        ("id" = Uuid, Path, description = "Profile link ID")
    ),
    responses(
        (status = 204, description = "Profile link deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Profile link not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[delete("/{id}")]
async fn delete_link(
    auth: AuthUser,
    path: web::Path<Uuid>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let link_id = path.into_inner();
    ProfileLinkService::delete_link(link_id, auth.id, &auth.territory, db.pool()).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// Configure profile link routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile/links")
            .service(list_links)
            .service(create_link)
            .service(update_link)
            .service(delete_link),
    );
}
