use crate::models::{
    AwardBadgeRequest, BadgeResponse, RegisterBadgeRequest, RevokeBadgeRequest,
    ToggleFeaturedRequest, UpdateProgressRequest, UserBadgeResponse,
};
use crate::services::badge;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse};
use shared_lib::{AuthUser, Database, NatsClient, Result, ValidatedJson};
use uuid::Uuid;

/// Register a new badge (service-to-service)
#[utoipa::path(
    post,
    path = "/api/v1/badges/register",
    tag = "badges",
    request_body = RegisterBadgeRequest,
    responses(
        (status = 201, description = "Badge registered successfully", body = BadgeResponse),
        (status = 409, description = "Badge already exists"),
    )
)]
pub async fn register_badge(
    body: ValidatedJson<RegisterBadgeRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let badge = badge::register_badge(&db, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(badge))
}

/// List all available badges
#[utoipa::path(
    get,
    path = "/api/v1/badges",
    tag = "badges",
    responses(
        (status = 200, description = "List of badges", body = Vec<BadgeResponse>),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_badges(req: HttpRequest, db: web::Data<Database>) -> Result<HttpResponse> {
    // Try to extract auth user (optional)
    let user_id = AuthUser::from_request(&req, &mut actix_web::dev::Payload::None)
        .into_inner()
        .ok()
        .map(|auth| auth.id);
    let badges = badge::list_badges(&db, user_id).await?;
    Ok(HttpResponse::Ok().json(badges))
}

/// Get badges for a specific user
#[utoipa::path(
    get,
    path = "/api/v1/badges/users/{user_id}",
    tag = "badges",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User's badges", body = Vec<UserBadgeResponse>),
        (status = 404, description = "User not found"),
    )
)]
pub async fn get_user_badges(
    user_id: web::Path<Uuid>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let badges = badge::get_user_badges(&db, *user_id).await?;
    Ok(HttpResponse::Ok().json(badges))
}

/// Award a badge to a user (admin/system only)
#[utoipa::path(
    post,
    path = "/api/v1/badges/award",
    tag = "badges",
    request_body = AwardBadgeRequest,
    responses(
        (status = 201, description = "Badge awarded successfully"),
        (status = 400, description = "User already has badge"),
        (status = 404, description = "Badge not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn award_badge(
    _auth: AuthUser, // TODO: Check for Platform Manager badge
    body: ValidatedJson<AwardBadgeRequest>,
    db: web::Data<Database>,
    nats: web::Data<NatsClient>,
) -> Result<HttpResponse> {
    let award_id = badge::award_badge(
        &db,
        &nats,
        body.user_id,
        &body.badge_slug,
        None, // TODO: Use auth.id when Platform Manager check is implemented
        body.reason.clone(),
    )
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "award_id": award_id
    })))
}

/// Revoke a badge from a user (admin/system only)
#[utoipa::path(
    post,
    path = "/api/v1/badges/revoke",
    tag = "badges",
    request_body = RevokeBadgeRequest,
    responses(
        (status = 200, description = "Badge revoked successfully"),
        (status = 404, description = "Badge not found or user doesn't have it"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn revoke_badge(
    _auth: AuthUser, // TODO: Check for Platform Manager badge
    body: ValidatedJson<RevokeBadgeRequest>,
    db: web::Data<Database>,
    nats: web::Data<NatsClient>,
) -> Result<HttpResponse> {
    badge::revoke_badge(
        &db,
        &nats,
        body.user_id,
        &body.badge_slug,
        body.reason.clone(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Badge revoked successfully"
    })))
}

/// Update badge progress (internal/system use)
#[utoipa::path(
    post,
    path = "/api/v1/badges/progress",
    tag = "badges",
    request_body = UpdateProgressRequest,
    responses(
        (status = 200, description = "Progress updated", body = inline(Object)),
        (status = 404, description = "Badge not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_progress(
    _auth: AuthUser,
    body: ValidatedJson<UpdateProgressRequest>,
    db: web::Data<Database>,
    nats: web::Data<NatsClient>,
) -> Result<HttpResponse> {
    let auto_awarded = badge::update_badge_progress(
        &db,
        &nats,
        body.user_id,
        &body.badge_slug,
        body.current_value,
    )
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "auto_awarded": auto_awarded
    })))
}

/// Toggle featured badge
#[utoipa::path(
    patch,
    path = "/api/v1/badges/featured",
    tag = "badges",
    request_body = ToggleFeaturedRequest,
    responses(
        (status = 200, description = "Featured status updated"),
        (status = 404, description = "Badge not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn toggle_featured(
    auth: AuthUser,
    body: ValidatedJson<ToggleFeaturedRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    badge::toggle_featured_badge(&db, auth.id, body.badge_id, body.is_featured).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true
    })))
}

/// Register a new badge (for services to register their role badges)
/// Configure badge routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/badges")
            .route("", web::get().to(list_badges))
            .route("/users/{user_id}", web::get().to(get_user_badges))
            .route("/award", web::post().to(award_badge))
            .route("/revoke", web::post().to(revoke_badge))
            .route("/progress", web::post().to(update_progress))
            .route("/featured", web::patch().to(toggle_featured))
            .route("/register", web::post().to(register_badge)),
    );
}
