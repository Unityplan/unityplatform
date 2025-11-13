use actix_web::{delete, get, post, put, web, HttpResponse};
use shared_lib::{AuthUser, Database, Result, ValidatedJson};
use uuid::Uuid;

use crate::models::{
    CreateLanguageProficiencyRequest, LanguageProficiencyResponse, UpdateLanguageProficiencyRequest,
};
use crate::services::LanguageProficiencyService;

/// GET /api/v1/profiles/me/languages - List all language proficiencies
#[utoipa::path(
    get,
    path = "/api/v1/user/profile/languages",
    tag = "language-proficiency",
    responses(
        (status = 200, description = "Language proficiencies retrieved successfully", body = Vec<LanguageProficiencyResponse>),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("")]
async fn list_languages(auth: AuthUser, db: web::Data<Database>) -> Result<HttpResponse> {
    let languages =
        LanguageProficiencyService::get_user_languages(auth.id, &auth.territory, db.pool()).await?;
    let response: Vec<LanguageProficiencyResponse> =
        languages.into_iter().map(Into::into).collect();
    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/profiles/me/languages - Create a new language proficiency
#[utoipa::path(
    post,
    path = "/api/v1/user/profile/languages",
    tag = "language-proficiency",
    request_body = CreateLanguageProficiencyRequest,
    responses(
        (status = 201, description = "Language proficiency created successfully", body = LanguageProficiencyResponse),
        (status = 400, description = "Invalid request or language already exists"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("")]
async fn create_language(
    auth: AuthUser,
    body: ValidatedJson<CreateLanguageProficiencyRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let language =
        LanguageProficiencyService::create_language(auth.id, &auth.territory, req, db.pool())
            .await?;
    let response: LanguageProficiencyResponse = language.into();
    Ok(HttpResponse::Created().json(response))
}

/// PUT /api/v1/profiles/me/languages/:id - Update a language proficiency
#[utoipa::path(
    put,
    path = "/api/v1/user/profile/languages/{id}",
    tag = "language-proficiency",
    params(
        ("id" = Uuid, Path, description = "Language proficiency ID")
    ),
    request_body = UpdateLanguageProficiencyRequest,
    responses(
        (status = 200, description = "Language proficiency updated successfully", body = LanguageProficiencyResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Language proficiency not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/{id}")]
async fn update_language(
    auth: AuthUser,
    path: web::Path<Uuid>,
    body: ValidatedJson<UpdateLanguageProficiencyRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let lang_id = path.into_inner();
    let req = body.into_inner();
    let language = LanguageProficiencyService::update_language(
        lang_id,
        auth.id,
        &auth.territory,
        req,
        db.pool(),
    )
    .await?;
    let response: LanguageProficiencyResponse = language.into();
    Ok(HttpResponse::Ok().json(response))
}

/// DELETE /api/v1/profiles/me/languages/:id - Delete a language proficiency
#[utoipa::path(
    delete,
    path = "/api/v1/user/profile/languages/{id}",
    tag = "language-proficiency",
    params(
        ("id" = Uuid, Path, description = "Language proficiency ID")
    ),
    responses(
        (status = 204, description = "Language proficiency deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Language proficiency not found"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[delete("/{id}")]
async fn delete_language(
    auth: AuthUser,
    path: web::Path<Uuid>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let lang_id = path.into_inner();
    LanguageProficiencyService::delete_language(lang_id, auth.id, &auth.territory, db.pool())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

/// Configure language proficiency routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile/languages")
            .service(list_languages)
            .service(create_language)
            .service(update_language)
            .service(delete_language),
    );
}
