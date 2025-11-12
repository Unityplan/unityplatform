use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::ServiceError,
    models::{
        CompleteProfile, LanguageProficiency, ProfileLink, UpdateProfileRequest, User, UserProfile,
    },
    response::ApiResponse,
};

/// Get user profile by ID
///
/// Returns complete profile including user data, profile, links, and languages
#[utoipa::path(
    get,
    path = "/v1/profiles/{id}",
    tag = "profiles",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Profile retrieved successfully", body = CompleteProfile),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_profile(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Fetch user
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, full_name, territory_code, is_active, is_verified, 
                verified_at, totp_enabled, deleted_at, created_at, updated_at
         FROM territory_dk.users 
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(ServiceError::NotFound("User not found".to_string()))?;

    // Fetch profile (create if doesn't exist)
    let profile = sqlx::query_as::<_, UserProfile>(
        "SELECT user_id, display_name, avatar_url, bio, about, 
                COALESCE(interests, ARRAY[]::text[]) as interests,
                COALESCE(skills, ARRAY[]::text[]) as skills,
                COALESCE(languages, ARRAY[]::varchar(10)[]) as languages,
                location, created_at, updated_at
         FROM territory_dk.users_profiles 
         WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let profile = match profile {
        Some(p) => p,
        None => {
            // Create default profile
            sqlx::query_as::<_, UserProfile>(
                "INSERT INTO territory_dk.users_profiles (user_id)
                 VALUES ($1)
                 RETURNING user_id, display_name, avatar_url, bio, about,
                           COALESCE(interests, ARRAY[]::text[]) as interests,
                           COALESCE(skills, ARRAY[]::text[]) as skills,
                           COALESCE(languages, ARRAY[]::varchar(10)[]) as languages,
                           location, created_at, updated_at",
            )
            .bind(user_id)
            .fetch_one(pool.get_ref())
            .await?
        }
    };

    // Fetch profile links
    let links = sqlx::query_as::<_, ProfileLink>(
        "SELECT id, user_id, label, url, icon, display_order, is_visible, created_at, updated_at
         FROM territory_dk.users_profile_links 
         WHERE user_id = $1 
         ORDER BY display_order ASC",
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Fetch language proficiencies
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

    let complete_profile = CompleteProfile {
        user,
        profile,
        links,
        languages,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(complete_profile)))
}

/// Update user profile
///
/// Updates profile information for the authenticated user
#[utoipa::path(
    put,
    path = "/v1/profiles/{id}",
    tag = "profiles",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UserProfile),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_profile(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    req: web::Json<UpdateProfileRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Validate request
    req.validate()?;

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

    // Build dynamic update query
    let mut query = String::from("UPDATE territory_dk.users_profiles SET updated_at = NOW()");
    let mut param_count = 1;

    if req.display_name.is_some() {
        param_count += 1;
        query.push_str(&format!(", display_name = ${}", param_count));
    }
    if req.avatar_url.is_some() {
        param_count += 1;
        query.push_str(&format!(", avatar_url = ${}", param_count));
    }
    if req.bio.is_some() {
        param_count += 1;
        query.push_str(&format!(", bio = ${}", param_count));
    }
    if req.about.is_some() {
        param_count += 1;
        query.push_str(&format!(", about = ${}", param_count));
    }
    if req.interests.is_some() {
        param_count += 1;
        query.push_str(&format!(", interests = ${}", param_count));
    }
    if req.skills.is_some() {
        param_count += 1;
        query.push_str(&format!(", skills = ${}", param_count));
    }
    if req.languages.is_some() {
        param_count += 1;
        query.push_str(&format!(", languages = ${}", param_count));
    }
    if req.location.is_some() {
        param_count += 1;
        query.push_str(&format!(", location = ${}", param_count));
    }

    query.push_str(" WHERE user_id = $1 RETURNING user_id, display_name, avatar_url, bio, about, ");
    query.push_str("COALESCE(interests, ARRAY[]::text[]) as interests, ");
    query.push_str("COALESCE(skills, ARRAY[]::text[]) as skills, ");
    query.push_str("COALESCE(languages, ARRAY[]::varchar(10)[]) as languages, ");
    query.push_str("location, created_at, updated_at");

    // Execute query with parameters
    let mut query_builder = sqlx::query_as::<_, UserProfile>(&query).bind(user_id);

    if let Some(ref display_name) = req.display_name {
        query_builder = query_builder.bind(display_name);
    }
    if let Some(ref avatar_url) = req.avatar_url {
        query_builder = query_builder.bind(avatar_url);
    }
    if let Some(ref bio) = req.bio {
        query_builder = query_builder.bind(bio);
    }
    if let Some(ref about) = req.about {
        query_builder = query_builder.bind(about);
    }
    if let Some(ref interests) = req.interests {
        query_builder = query_builder.bind(interests);
    }
    if let Some(ref skills) = req.skills {
        query_builder = query_builder.bind(skills);
    }
    if let Some(ref languages) = req.languages {
        query_builder = query_builder.bind(languages);
    }
    if let Some(ref location) = req.location {
        query_builder = query_builder.bind(location);
    }

    let updated_profile = query_builder.fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(updated_profile)))
}
