use crate::models::language::{LanguageResponse, SearchLanguagesParams};
use crate::services::language::LanguageService;
use actix_web::{web, HttpResponse};
use shared_lib::{AuthUser, Database, Result};

/// Search languages in the global registry
#[utoipa::path(
    get,
    path = "/api/v1/territory/languages",
    tag = "languages",
    security(("bearer_auth" = [])),
    params(
        ("q" = Option<String>, Query, description = "Search query (language name or code)"),
        ("limit" = Option<i64>, Query, description = "Maximum number of results (default: 50, max: 500)"),
        ("active_only" = Option<bool>, Query, description = "Return only active languages (default: true)")
    ),
    responses(
        (status = 200, description = "List of matching languages", body = Vec<LanguageResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn search_languages(
    _auth: AuthUser,
    db: web::Data<Database>,
    query: web::Query<SearchLanguagesParams>,
) -> Result<HttpResponse> {
    let languages = LanguageService::search_languages(&db, query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(languages))
}

/// Get language details by code
#[utoipa::path(
    get,
    path = "/api/v1/territory/languages/{code}",
    tag = "languages",
    security(("bearer_auth" = [])),
    params(
        ("code" = String, Path, description = "ISO 639-3 language code (e.g., 'eng', 'dan')")
    ),
    responses(
        (status = 200, description = "Language details", body = LanguageResponse),
        (status = 404, description = "Language not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_language(
    _auth: AuthUser,
    db: web::Data<Database>,
    code: web::Path<String>,
) -> Result<HttpResponse> {
    let language = LanguageService::get_language(&db, &code).await?;
    Ok(HttpResponse::Ok().json(language))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/languages")
            .route("", web::get().to(search_languages))
            .route("/{code}", web::get().to(get_language)),
    );
}
