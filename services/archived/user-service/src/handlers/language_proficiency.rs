use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::ServiceError,
    models::{
        CreateLanguageProficiencyRequest, LanguageProficiency, UpdateLanguageProficiencyRequest,
    },
    response::ApiResponse,
};

/// Get all language proficiencies for a user
///
/// Returns languages the user speaks/understands (PUBLIC profile data)
#[utoipa::path(
    get,
    path = "/v1/profiles/{id}/languages",
    tag = "language-proficiency",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Languages retrieved successfully", body = Vec<LanguageProficiency>),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_language_proficiencies(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Verify user exists
    let user_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM territory_dk.users WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !user_exists {
        return Err(ServiceError::NotFound("User not found".to_string()));
    }

    // Fetch visible language proficiencies
    let languages = sqlx::query_as::<_, LanguageProficiency>(
        "SELECT id, user_id, language_code, language_name, 
                spoken_level, written_level, reading_level, listening_level,
                display_order, is_preferred, show_on_profile, created_at, updated_at
         FROM territory_dk.users_language_proficiency 
         WHERE user_id = $1 AND show_on_profile = true
         ORDER BY is_preferred DESC, display_order ASC",
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(languages)))
}

/// Add a new language proficiency
///
/// Add a language the user speaks/understands to their PUBLIC profile
#[utoipa::path(
    post,
    path = "/v1/profiles/{id}/languages",
    tag = "language-proficiency",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = CreateLanguageProficiencyRequest,
    responses(
        (status = 201, description = "Language proficiency created successfully", body = LanguageProficiency),
        (status = 400, description = "Invalid input or language already exists"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_language_proficiency(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    req: web::Json<CreateLanguageProficiencyRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Validate request
    req.validate()?;

    // Check if language already exists for user
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM territory_dk.users_language_proficiency 
         WHERE user_id = $1 AND language_code = $2)",
    )
    .bind(user_id)
    .bind(&req.language_code)
    .fetch_one(pool.get_ref())
    .await?;

    if exists {
        return Err(ServiceError::BadRequest(format!(
            "Language '{}' already exists in profile",
            req.language_code
        )));
    }

    // Get next display_order
    let next_order = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT MAX(display_order) + 1 FROM territory_dk.users_language_proficiency WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await?
    .unwrap_or(0);

    // If this is marked as preferred, unset other preferred languages
    if req.is_preferred.unwrap_or(false) {
        sqlx::query(
            "UPDATE territory_dk.users_language_proficiency 
             SET is_preferred = false 
             WHERE user_id = $1",
        )
        .bind(user_id)
        .execute(pool.get_ref())
        .await?;
    }

    let language = sqlx::query_as::<_, LanguageProficiency>(
        "INSERT INTO territory_dk.users_language_proficiency 
         (user_id, language_code, language_name, spoken_level, written_level, 
          reading_level, listening_level, display_order, is_preferred, show_on_profile)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING id, user_id, language_code, language_name, 
                   spoken_level, written_level, reading_level, listening_level,
                   display_order, is_preferred, show_on_profile, created_at, updated_at",
    )
    .bind(user_id)
    .bind(&req.language_code)
    .bind(&req.language_name)
    .bind(&req.spoken_level)
    .bind(&req.written_level)
    .bind(&req.reading_level)
    .bind(&req.listening_level)
    .bind(next_order)
    .bind(req.is_preferred.unwrap_or(false))
    .bind(req.show_on_profile.unwrap_or(true))
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(language)))
}

/// Update a language proficiency
#[utoipa::path(
    put,
    path = "/v1/profiles/{id}/languages/{lang_id}",
    tag = "language-proficiency",
    params(
        ("id" = Uuid, Path, description = "User ID"),
        ("lang_id" = Uuid, Path, description = "Language proficiency ID")
    ),
    request_body = UpdateLanguageProficiencyRequest,
    responses(
        (status = 200, description = "Language proficiency updated successfully", body = LanguageProficiency),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Language proficiency not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_language_proficiency(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    req: web::Json<UpdateLanguageProficiencyRequest>,
) -> Result<HttpResponse, ServiceError> {
    let (user_id, lang_id) = path.into_inner();

    // Validate request
    req.validate()?;

    // If setting as preferred, unset other preferred languages
    if req.is_preferred == Some(true) {
        sqlx::query(
            "UPDATE territory_dk.users_language_proficiency 
             SET is_preferred = false 
             WHERE user_id = $1 AND id != $2",
        )
        .bind(user_id)
        .bind(lang_id)
        .execute(pool.get_ref())
        .await?;
    }

    // Build dynamic update query
    let mut query =
        String::from("UPDATE territory_dk.users_language_proficiency SET updated_at = NOW()");
    let mut param_count = 2; // user_id and lang_id are $1 and $2

    if req.spoken_level.is_some() {
        param_count += 1;
        query.push_str(&format!(", spoken_level = ${}", param_count));
    }
    if req.written_level.is_some() {
        param_count += 1;
        query.push_str(&format!(", written_level = ${}", param_count));
    }
    if req.reading_level.is_some() {
        param_count += 1;
        query.push_str(&format!(", reading_level = ${}", param_count));
    }
    if req.listening_level.is_some() {
        param_count += 1;
        query.push_str(&format!(", listening_level = ${}", param_count));
    }
    if req.is_preferred.is_some() {
        param_count += 1;
        query.push_str(&format!(", is_preferred = ${}", param_count));
    }
    if req.show_on_profile.is_some() {
        param_count += 1;
        query.push_str(&format!(", show_on_profile = ${}", param_count));
    }

    query.push_str(" WHERE user_id = $1 AND id = $2 ");
    query.push_str("RETURNING id, user_id, language_code, language_name, ");
    query.push_str("spoken_level, written_level, reading_level, listening_level, ");
    query.push_str("display_order, is_preferred, show_on_profile, created_at, updated_at");

    let mut query_builder = sqlx::query_as::<_, LanguageProficiency>(&query)
        .bind(user_id)
        .bind(lang_id);

    if let Some(ref level) = req.spoken_level {
        query_builder = query_builder.bind(level);
    }
    if let Some(ref level) = req.written_level {
        query_builder = query_builder.bind(level);
    }
    if let Some(ref level) = req.reading_level {
        query_builder = query_builder.bind(level);
    }
    if let Some(ref level) = req.listening_level {
        query_builder = query_builder.bind(level);
    }
    if let Some(is_preferred) = req.is_preferred {
        query_builder = query_builder.bind(is_preferred);
    }
    if let Some(show_on_profile) = req.show_on_profile {
        query_builder = query_builder.bind(show_on_profile);
    }

    let language =
        query_builder
            .fetch_optional(pool.get_ref())
            .await?
            .ok_or(ServiceError::NotFound(
                "Language proficiency not found".to_string(),
            ))?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(language)))
}

/// Delete a language proficiency
#[utoipa::path(
    delete,
    path = "/v1/profiles/{id}/languages/{lang_id}",
    tag = "language-proficiency",
    params(
        ("id" = Uuid, Path, description = "User ID"),
        ("lang_id" = Uuid, Path, description = "Language proficiency ID")
    ),
    responses(
        (status = 204, description = "Language proficiency deleted successfully"),
        (status = 404, description = "Language proficiency not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_language_proficiency(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ServiceError> {
    let (user_id, lang_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM territory_dk.users_language_proficiency 
         WHERE user_id = $1 AND id = $2",
    )
    .bind(user_id)
    .bind(lang_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ServiceError::NotFound(
            "Language proficiency not found".to_string(),
        ));
    }

    Ok(HttpResponse::NoContent().finish())
}
