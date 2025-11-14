use actix_web::{get, patch, web, HttpResponse};
use shared_lib::{AuthUser, Database, Result, ValidatedJson};

use crate::models::{
    NotificationSettingsResponse, PrivacySettingsResponse, SettingsResponse,
    UpdateNotificationSettingsRequest, UpdatePrivacySettingsRequest, UpdateSettingsRequest,
};
use crate::services::SettingsService;

/// Configure settings routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_settings)
        .service(update_settings)
        .service(get_privacy_settings)
        .service(update_privacy_settings)
        .service(get_notification_settings)
        .service(update_notification_settings);
}

/// GET /api/v1/user/settings - Get all user settings
#[utoipa::path(
    get,
    path = "/api/v1/user/settings",
    tag = "settings",
    responses(
        (status = 200, description = "Settings retrieved successfully", body = SettingsResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/settings")]
async fn get_settings(auth: AuthUser, db: web::Data<Database>) -> Result<HttpResponse> {
    let settings = SettingsService::get_settings(auth.id, &auth.territory, db.pool()).await?;
    Ok(HttpResponse::Ok().json(settings))
}

/// PATCH /api/v1/user/settings - Update user settings
#[utoipa::path(
    patch,
    path = "/api/v1/user/settings",
    tag = "settings",
    request_body = UpdateSettingsRequest,
    responses(
        (status = 200, description = "Settings updated successfully", body = SettingsResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[patch("/settings")]
async fn update_settings(
    auth: AuthUser,
    body: ValidatedJson<UpdateSettingsRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let settings =
        SettingsService::update_settings(auth.id, &auth.territory, req, db.pool()).await?;
    Ok(HttpResponse::Ok().json(settings))
}

/// GET /api/v1/user/settings/privacy - Get privacy settings
#[utoipa::path(
    get,
    path = "/api/v1/user/settings/privacy",
    tag = "settings",
    responses(
        (status = 200, description = "Privacy settings retrieved successfully", body = PrivacySettingsResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/settings/privacy")]
async fn get_privacy_settings(auth: AuthUser, db: web::Data<Database>) -> Result<HttpResponse> {
    let settings =
        SettingsService::get_privacy_settings(auth.id, &auth.territory, db.pool()).await?;
    Ok(HttpResponse::Ok().json(settings))
}

/// PATCH /api/v1/user/settings/privacy - Update privacy settings
#[utoipa::path(
    patch,
    path = "/api/v1/user/settings/privacy",
    tag = "settings",
    request_body = UpdatePrivacySettingsRequest,
    responses(
        (status = 200, description = "Privacy settings updated successfully", body = PrivacySettingsResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[patch("/settings/privacy")]
async fn update_privacy_settings(
    auth: AuthUser,
    body: ValidatedJson<UpdatePrivacySettingsRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let settings =
        SettingsService::update_privacy_settings(auth.id, &auth.territory, req, db.pool()).await?;
    Ok(HttpResponse::Ok().json(settings))
}

/// GET /api/v1/user/settings/notifications - Get notification settings
#[utoipa::path(
    get,
    path = "/api/v1/user/settings/notifications",
    tag = "settings",
    responses(
        (status = 200, description = "Notification settings retrieved successfully", body = NotificationSettingsResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/settings/notifications")]
async fn get_notification_settings(
    auth: AuthUser,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let settings =
        SettingsService::get_notification_settings(auth.id, &auth.territory, db.pool()).await?;
    Ok(HttpResponse::Ok().json(settings))
}

/// PATCH /api/v1/user/settings/notifications - Update notification settings
#[utoipa::path(
    patch,
    path = "/api/v1/user/settings/notifications",
    tag = "settings",
    request_body = UpdateNotificationSettingsRequest,
    responses(
        (status = 200, description = "Notification settings updated successfully", body = NotificationSettingsResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[patch("/settings/notifications")]
async fn update_notification_settings(
    auth: AuthUser,
    body: ValidatedJson<UpdateNotificationSettingsRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let settings =
        SettingsService::update_notification_settings(auth.id, &auth.territory, req, db.pool())
            .await?;
    Ok(HttpResponse::Ok().json(settings))
}
