use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::ServiceError,
    models::{
        NotificationSettings, UpdateNotificationSettingsRequest, UpdateUserSettingsRequest,
        UserSettings,
    },
    response::ApiResponse,
};

/// Get user settings
#[utoipa::path(
    get,
    path = "/v1/users/{id}/settings",
    tag = "settings",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Settings retrieved successfully", body = UserSettings),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_user_settings(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Fetch settings (create if doesn't exist)
    let settings = sqlx::query_as::<_, UserSettings>(
        "SELECT user_id, theme_mode, color_scheme, reduced_motion, wide_content_view, compact_mode,
                preferred_language, timezone, auto_translate, translation_provider,
                contribute_translations, fallback_to_english, created_at, updated_at
         FROM territory_dk.users_settings 
         WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let settings = match settings {
        Some(s) => s,
        None => {
            // Create default settings
            sqlx::query_as::<_, UserSettings>(
                "INSERT INTO territory_dk.users_settings (user_id)
                 VALUES ($1)
                 RETURNING user_id, theme_mode, color_scheme, reduced_motion, wide_content_view, compact_mode,
                           preferred_language, timezone, auto_translate, translation_provider,
                           contribute_translations, fallback_to_english, created_at, updated_at"
            )
            .bind(user_id)
            .fetch_one(pool.get_ref())
            .await?
        }
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(settings)))
}

/// Update user settings
#[utoipa::path(
    put,
    path = "/v1/users/{id}/settings",
    tag = "settings",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserSettingsRequest,
    responses(
        (status = 200, description = "Settings updated successfully", body = UserSettings),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_user_settings(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    req: web::Json<UpdateUserSettingsRequest>,
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
    let mut query = String::from("UPDATE territory_dk.users_settings SET updated_at = NOW()");
    let mut param_count = 1;

    if req.theme_mode.is_some() {
        param_count += 1;
        query.push_str(&format!(", theme_mode = ${}", param_count));
    }
    if req.color_scheme.is_some() {
        param_count += 1;
        query.push_str(&format!(", color_scheme = ${}", param_count));
    }
    if req.reduced_motion.is_some() {
        param_count += 1;
        query.push_str(&format!(", reduced_motion = ${}", param_count));
    }
    if req.wide_content_view.is_some() {
        param_count += 1;
        query.push_str(&format!(", wide_content_view = ${}", param_count));
    }
    if req.compact_mode.is_some() {
        param_count += 1;
        query.push_str(&format!(", compact_mode = ${}", param_count));
    }
    if req.preferred_language.is_some() {
        param_count += 1;
        query.push_str(&format!(", preferred_language = ${}", param_count));
    }
    if req.timezone.is_some() {
        param_count += 1;
        query.push_str(&format!(", timezone = ${}", param_count));
    }
    if req.auto_translate.is_some() {
        param_count += 1;
        query.push_str(&format!(", auto_translate = ${}", param_count));
    }
    if req.translation_provider.is_some() {
        param_count += 1;
        query.push_str(&format!(", translation_provider = ${}", param_count));
    }
    if req.contribute_translations.is_some() {
        param_count += 1;
        query.push_str(&format!(", contribute_translations = ${}", param_count));
    }
    if req.fallback_to_english.is_some() {
        param_count += 1;
        query.push_str(&format!(", fallback_to_english = ${}", param_count));
    }

    query.push_str(
        " WHERE user_id = $1 RETURNING user_id, theme_mode, color_scheme, reduced_motion, ",
    );
    query.push_str(
        "wide_content_view, compact_mode, preferred_language, timezone, auto_translate, ",
    );
    query.push_str("translation_provider, contribute_translations, fallback_to_english, created_at, updated_at");

    // Execute query with parameters
    let mut query_builder = sqlx::query_as::<_, UserSettings>(&query).bind(user_id);

    if let Some(theme_mode) = &req.theme_mode {
        query_builder = query_builder.bind(theme_mode);
    }
    if let Some(color_scheme) = &req.color_scheme {
        query_builder = query_builder.bind(color_scheme);
    }
    if let Some(reduced_motion) = req.reduced_motion {
        query_builder = query_builder.bind(reduced_motion);
    }
    if let Some(wide_content_view) = req.wide_content_view {
        query_builder = query_builder.bind(wide_content_view);
    }
    if let Some(compact_mode) = req.compact_mode {
        query_builder = query_builder.bind(compact_mode);
    }
    if let Some(ref preferred_language) = req.preferred_language {
        query_builder = query_builder.bind(preferred_language);
    }
    if let Some(ref timezone) = req.timezone {
        query_builder = query_builder.bind(timezone);
    }
    if let Some(auto_translate) = req.auto_translate {
        query_builder = query_builder.bind(auto_translate);
    }
    if let Some(translation_provider) = &req.translation_provider {
        query_builder = query_builder.bind(translation_provider);
    }
    if let Some(contribute_translations) = req.contribute_translations {
        query_builder = query_builder.bind(contribute_translations);
    }
    if let Some(fallback_to_english) = req.fallback_to_english {
        query_builder = query_builder.bind(fallback_to_english);
    }

    let updated_settings = query_builder.fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(updated_settings)))
}

/// Get notification settings
#[utoipa::path(
    get,
    path = "/v1/users/{id}/settings/notifications",
    tag = "settings",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Notification settings retrieved successfully", body = NotificationSettings),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_notification_settings(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Fetch settings (create if doesn't exist)
    let settings = sqlx::query_as::<_, NotificationSettings>(
        "SELECT user_id, email_digest, email_messages, email_followers, email_community, email_updates,
                inapp_messages, inapp_followers, inapp_community, inapp_mentions, inapp_likes,
                push_enabled, push_messages, push_followers, push_community, created_at, updated_at
         FROM territory_dk.users_notification_settings 
         WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let settings = match settings {
        Some(s) => s,
        None => {
            // Create default notification settings
            sqlx::query_as::<_, NotificationSettings>(
                "INSERT INTO territory_dk.users_notification_settings (user_id)
                 VALUES ($1)
                 RETURNING user_id, email_digest, email_messages, email_followers, email_community, email_updates,
                           inapp_messages, inapp_followers, inapp_community, inapp_mentions, inapp_likes,
                           push_enabled, push_messages, push_followers, push_community, created_at, updated_at"
            )
            .bind(user_id)
            .fetch_one(pool.get_ref())
            .await?
        }
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(settings)))
}

/// Update notification settings
#[utoipa::path(
    put,
    path = "/v1/users/{id}/settings/notifications",
    tag = "settings",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateNotificationSettingsRequest,
    responses(
        (status = 200, description = "Notification settings updated successfully", body = NotificationSettings),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_notification_settings(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    req: web::Json<UpdateNotificationSettingsRequest>,
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

    // Build dynamic update query
    let mut query =
        String::from("UPDATE territory_dk.users_notification_settings SET updated_at = NOW()");
    let mut param_count = 1;
    let mut has_updates = false;

    macro_rules! add_field {
        ($field:ident) => {
            if req.$field.is_some() {
                param_count += 1;
                query.push_str(&format!(", {} = ${}", stringify!($field), param_count));
                has_updates = true;
            }
        };
    }

    add_field!(email_digest);
    add_field!(email_messages);
    add_field!(email_followers);
    add_field!(email_community);
    add_field!(email_updates);
    add_field!(inapp_messages);
    add_field!(inapp_followers);
    add_field!(inapp_community);
    add_field!(inapp_mentions);
    add_field!(inapp_likes);
    add_field!(push_enabled);
    add_field!(push_messages);
    add_field!(push_followers);
    add_field!(push_community);

    query.push_str(
        " WHERE user_id = $1 RETURNING user_id, email_digest, email_messages, email_followers, ",
    );
    query.push_str(
        "email_community, email_updates, inapp_messages, inapp_followers, inapp_community, ",
    );
    query.push_str("inapp_mentions, inapp_likes, push_enabled, push_messages, push_followers, push_community, ");
    query.push_str("created_at, updated_at");

    // Execute query with parameters
    let mut query_builder = sqlx::query_as::<_, NotificationSettings>(&query).bind(user_id);

    macro_rules! bind_field {
        ($field:ident) => {
            if let Some(value) = req.$field {
                query_builder = query_builder.bind(value);
            }
        };
    }

    bind_field!(email_digest);
    bind_field!(email_messages);
    bind_field!(email_followers);
    bind_field!(email_community);
    bind_field!(email_updates);
    bind_field!(inapp_messages);
    bind_field!(inapp_followers);
    bind_field!(inapp_community);
    bind_field!(inapp_mentions);
    bind_field!(inapp_likes);
    bind_field!(push_enabled);
    bind_field!(push_messages);
    bind_field!(push_followers);
    bind_field!(push_community);

    let updated_settings = query_builder.fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(updated_settings)))
}
