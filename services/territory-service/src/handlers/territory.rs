use crate::models::territory::{
    TerritoryResponse, TerritorySettingsResponse, TerritoryStatsResponse, UpdateSettingsRequest,
};
use crate::services::territory::TerritoryService;
use actix_web::{web, HttpResponse};
use shared_lib::{AuthUser, Database, Result, ValidatedJson};

/// List all active territories
#[utoipa::path(
    get,
    path = "/api/v1/territories",
    tag = "territories",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of active territories", body = Vec<TerritoryResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
async fn list_territories(_auth: AuthUser, db: web::Data<Database>) -> Result<HttpResponse> {
    let territories = TerritoryService::list_active_territories(&db).await?;
    Ok(HttpResponse::Ok().json(territories))
}

/// Get territory details by code
#[utoipa::path(
    get,
    path = "/api/v1/territories/{code}",
    tag = "territories",
    security(("bearer_auth" = [])),
    params(
        ("code" = String, Path, description = "Territory code (e.g., 'dk')")
    ),
    responses(
        (status = 200, description = "Territory details", body = TerritoryResponse),
        (status = 404, description = "Territory not found"),
        (status = 401, description = "Unauthorized")
    )
)]
async fn get_territory(
    _auth: AuthUser,
    db: web::Data<Database>,
    code: web::Path<String>,
) -> Result<HttpResponse> {
    let territory = TerritoryService::get_territory(&db, &code).await?;
    Ok(HttpResponse::Ok().json(territory))
}

/// Get territory statistics (Territory Manager only)
#[utoipa::path(
    get,
    path = "/api/v1/territories/{code}/manage/stats",
    tag = "territory-management",
    security(("bearer_auth" = [])),
    params(
        ("code" = String, Path, description = "Territory code (e.g., 'dk')")
    ),
    responses(
        (status = 200, description = "Territory statistics", body = TerritoryStatsResponse),
        (status = 403, description = "Not a territory manager for this territory"),
        (status = 404, description = "Territory not found"),
        (status = 401, description = "Unauthorized")
    )
)]
async fn get_territory_stats(
    auth: AuthUser,
    db: web::Data<Database>,
    code: web::Path<String>,
) -> Result<HttpResponse> {
    let stats = TerritoryService::get_territory_stats(&db, &code, &auth.id).await?;
    Ok(HttpResponse::Ok().json(stats))
}

/// Update territory settings (Territory Manager only)
#[utoipa::path(
    patch,
    path = "/api/v1/territories/{code}/manage/settings",
    tag = "territory-management",
    security(("bearer_auth" = [])),
    params(
        ("code" = String, Path, description = "Territory code (e.g., 'dk')")
    ),
    request_body = UpdateSettingsRequest,
    responses(
        (status = 200, description = "Settings updated", body = TerritorySettingsResponse),
        (status = 403, description = "Not a territory manager for this territory"),
        (status = 404, description = "Territory not found"),
        (status = 401, description = "Unauthorized")
    )
)]
async fn update_territory_settings(
    auth: AuthUser,
    db: web::Data<Database>,
    code: web::Path<String>,
    body: ValidatedJson<UpdateSettingsRequest>,
) -> Result<HttpResponse> {
    let settings =
        TerritoryService::update_territory_settings(&db, &code, &auth.id, body.into_inner())
            .await?;
    Ok(HttpResponse::Ok().json(settings))
}

/// Configure territory routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(list_territories))
        .route("/{code}", web::get().to(get_territory))
        .route("/{code}/manage/stats", web::get().to(get_territory_stats))
        .route(
            "/{code}/manage/settings",
            web::patch().to(update_territory_settings),
        );
}
