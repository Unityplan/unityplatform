use actix_web::{web, HttpResponse};
use chrono::Utc;
use shared_lib::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{
    DataExport, DataExportResponse, ExportStatus, ExportedConnections, ExportedUserData,
    LanguageProficiency, NotificationSettings, ProfileLink, User, UserConnectionWithProfile,
    UserProfile, UserSettings,
};
use crate::response::ApiResponse;

/// Request data export
///
/// Initiates an async data export job for GDPR compliance (Article 20).
/// Returns immediately with export ID. User can check status and download when complete.
#[utoipa::path(
    post,
    path = "/v1/users/{id}/data/export",
    tag = "gdpr",
    params(
        ("id" = Uuid, Path, description = "User ID requesting export")
    ),
    responses(
        (status = 201, description = "Export request created", body = ApiResponse<DataExportResponse>),
        (status = 404, description = "User not found"),
        (status = 429, description = "Too many export requests")
    )
)]
pub async fn request_data_export(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Verify user exists
    let user_exists = sqlx::query!("SELECT id FROM territory_dk.users WHERE id = $1", user_id)
        .fetch_optional(pool.as_ref())
        .await?;

    if user_exists.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Check for recent pending/processing exports (rate limiting)
    let recent_export = sqlx::query!(
        r#"
        SELECT id FROM territory_dk.data_exports
        WHERE user_id = $1
        AND status IN ('pending', 'processing')
        AND requested_at > NOW() - INTERVAL '1 hour'
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    if recent_export.is_some() {
        return Err(AppError::Validation(
            "An export is already in progress. Please wait before requesting another.".to_string(),
        ));
    }

    // Create export request
    let export = sqlx::query_as!(
        DataExport,
        r#"
        INSERT INTO territory_dk.data_exports (user_id, status)
        VALUES ($1, 'pending')
        RETURNING 
            id, 
            user_id, 
            status as "status: ExportStatus",
            file_path, 
            file_size,
            requested_at, 
            started_at, 
            completed_at, 
            expires_at,
            downloaded_at,
            error_message
        "#,
        user_id
    )
    .fetch_one(pool.as_ref())
    .await?;

    // In a real implementation, trigger async job here
    // For now, immediately process the export synchronously
    process_export(pool.as_ref(), export.id, user_id).await?;

    // Fetch the updated export
    let updated_export = sqlx::query_as!(
        DataExport,
        r#"
        SELECT 
            id, 
            user_id, 
            status as "status: ExportStatus",
            file_path, 
            file_size,
            requested_at, 
            started_at, 
            completed_at, 
            expires_at,
            downloaded_at,
            error_message
        FROM territory_dk.data_exports
        WHERE id = $1
        "#,
        export.id
    )
    .fetch_one(pool.as_ref())
    .await?;

    let response = DataExportResponse {
        id: updated_export.id,
        status: updated_export.status,
        file_size: updated_export.file_size,
        requested_at: updated_export.requested_at,
        completed_at: updated_export.completed_at,
        expires_at: updated_export.expires_at,
        download_url: updated_export
            .completed_at
            .map(|_| format!("/v1/users/{}/data/export/{}", user_id, updated_export.id)),
    };

    Ok(HttpResponse::Created().json(ApiResponse::success(response)))
}

/// Get export status
///
/// Returns list of all export requests for the user with their current status.
#[utoipa::path(
    get,
    path = "/v1/users/{id}/data/export",
    tag = "gdpr",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Export list retrieved", body = ApiResponse<Vec<DataExportResponse>>),
        (status = 404, description = "User not found")
    )
)]
pub async fn list_data_exports(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Verify user exists
    let user_exists = sqlx::query!("SELECT id FROM territory_dk.users WHERE id = $1", user_id)
        .fetch_optional(pool.as_ref())
        .await?;

    if user_exists.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Get all exports for user
    let exports = sqlx::query_as!(
        DataExport,
        r#"
        SELECT 
            id, 
            user_id, 
            status as "status: ExportStatus",
            file_path, 
            file_size,
            requested_at, 
            started_at, 
            completed_at, 
            expires_at,
            downloaded_at,
            error_message
        FROM territory_dk.data_exports
        WHERE user_id = $1
        ORDER BY requested_at DESC
        "#,
        user_id
    )
    .fetch_all(pool.as_ref())
    .await?;

    let responses: Vec<DataExportResponse> = exports
        .into_iter()
        .map(|export| DataExportResponse {
            id: export.id,
            status: export.status,
            file_size: export.file_size,
            requested_at: export.requested_at,
            completed_at: export.completed_at,
            expires_at: export.expires_at,
            download_url: export
                .completed_at
                .map(|_| format!("/v1/users/{}/data/export/{}", user_id, export.id)),
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

/// Download export data
///
/// Downloads the generated export file as JSON. Export must be completed and not expired.
#[utoipa::path(
    get,
    path = "/v1/users/{id}/data/export/{export_id}",
    tag = "gdpr",
    params(
        ("id" = Uuid, Path, description = "User ID"),
        ("export_id" = Uuid, Path, description = "Export ID")
    ),
    responses(
        (status = 200, description = "Export data downloaded", body = ExportedUserData),
        (status = 404, description = "Export not found"),
        (status = 410, description = "Export expired")
    )
)]
pub async fn download_data_export(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, export_id) = path.into_inner();

    // Get export
    let export = sqlx::query_as!(
        DataExport,
        r#"
        SELECT 
            id, 
            user_id, 
            status as "status: ExportStatus",
            file_path, 
            file_size,
            requested_at, 
            started_at, 
            completed_at, 
            expires_at,
            downloaded_at,
            error_message
        FROM territory_dk.data_exports
        WHERE id = $1 AND user_id = $2
        "#,
        export_id,
        user_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    let export = export.ok_or_else(|| AppError::NotFound("Export not found".to_string()))?;

    // Check if expired
    if let Some(expires_at) = export.expires_at {
        if Utc::now() > expires_at {
            return Err(AppError::NotFound(
                "Export has expired. Please request a new export.".to_string(),
            ));
        }
    }

    // Check if completed
    match export.status {
        ExportStatus::Completed => {}
        ExportStatus::Failed => {
            return Err(AppError::Internal(
                export
                    .error_message
                    .unwrap_or_else(|| "Export failed".to_string()),
            ))
        }
        _ => {
            return Err(AppError::Validation(
                "Export is not ready yet. Please check status.".to_string(),
            ))
        }
    }

    // Mark as downloaded
    sqlx::query!(
        "UPDATE territory_dk.data_exports SET downloaded_at = NOW() WHERE id = $1",
        export_id
    )
    .execute(pool.as_ref())
    .await?;

    // Fetch all user data
    let exported_data = collect_user_data(pool.as_ref(), user_id).await?;

    Ok(HttpResponse::Ok().json(exported_data))
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Process export (in real app, this would be async job)
async fn process_export(pool: &PgPool, export_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    // Mark as processing
    sqlx::query!(
        "UPDATE territory_dk.data_exports SET status = 'processing', started_at = NOW() WHERE id = $1",
        export_id
    )
    .execute(pool)
    .await?;

    // Collect all user data
    let data = match collect_user_data(pool, user_id).await {
        Ok(data) => data,
        Err(e) => {
            // Mark as failed
            sqlx::query!(
                "UPDATE territory_dk.data_exports SET status = 'failed', error_message = $1 WHERE id = $2",
                e.to_string(),
                export_id
            )
            .execute(pool)
            .await?;
            return Err(e);
        }
    };

    // Calculate file size (approximate)
    let json_str = serde_json::to_string(&data)?;
    let file_size = json_str.len() as i64;

    // Mark as completed
    sqlx::query!(
        r#"
        UPDATE territory_dk.data_exports 
        SET status = 'completed', 
            completed_at = NOW(),
            file_size = $1
        WHERE id = $2
        "#,
        file_size,
        export_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Collect all user data for export
async fn collect_user_data(pool: &PgPool, user_id: Uuid) -> Result<ExportedUserData, AppError> {
    // Get user
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, full_name, territory_code, is_active, is_verified, verified_at, totp_enabled, deleted_at, created_at, updated_at FROM territory_dk.users WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await?;

    // Get profile
    let profile = sqlx::query!(
        "SELECT user_id, display_name, avatar_url, bio, about, interests, skills, languages, location, created_at, updated_at FROM territory_dk.users_profiles WHERE user_id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?
    .map(|row| UserProfile {
        user_id: row.user_id,
        display_name: row.display_name,
        avatar_url: row.avatar_url,
        bio: row.bio,
        about: row.about,
        interests: row.interests.unwrap_or_default(),
        skills: row.skills.unwrap_or_default(),
        languages: row.languages.unwrap_or_default(),
        location: row.location,
        created_at: row.created_at,
        updated_at: row.updated_at,
    });

    // Get links
    let links = sqlx::query_as!(
        ProfileLink,
        "SELECT id, user_id, label, url, icon, display_order, is_visible, created_at, updated_at FROM territory_dk.users_profile_links WHERE user_id = $1 ORDER BY display_order",
        user_id
    )
    .fetch_all(pool)
    .await?;

    // Get languages
    let languages = sqlx::query_as!(
        LanguageProficiency,
        r#"SELECT id, user_id, language_code, language_name, spoken_level as "spoken_level: _", written_level as "written_level: _", reading_level as "reading_level: _", listening_level as "listening_level: _", is_preferred, display_order, show_on_profile, created_at, updated_at FROM territory_dk.users_language_proficiency WHERE user_id = $1 ORDER BY display_order"#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    // Get settings
    let settings = sqlx::query_as!(
        UserSettings,
        r#"SELECT user_id, theme_mode as "theme_mode: _", color_scheme as "color_scheme: _", reduced_motion, wide_content_view, compact_mode, preferred_language, timezone, auto_translate, translation_provider as "translation_provider: _", contribute_translations, fallback_to_english, created_at, updated_at FROM territory_dk.users_settings WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    // Get notification settings
    let notification_settings = sqlx::query_as!(
        NotificationSettings,
        "SELECT user_id, email_digest, email_messages, email_followers, email_community, email_updates, inapp_messages, inapp_followers, inapp_community, inapp_mentions, inapp_likes, push_enabled, push_messages, push_followers, push_community, created_at, updated_at FROM territory_dk.users_notification_settings WHERE user_id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    // Get connections - followers
    let followers = sqlx::query_as!(
        UserConnectionWithProfile,
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            p.display_name,
            p.avatar_url,
            uc.connection_type as "connection_type: _",
            uc.status as "status: _",
            uc.created_at
        FROM territory_dk.user_connections uc
        JOIN territory_dk.users u ON u.id = uc.user_id
        LEFT JOIN territory_dk.users_profiles p ON p.user_id = u.id
        WHERE uc.target_user_id = $1
        AND uc.connection_type = 'follow'
        AND uc.status = 'active'
        ORDER BY uc.created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    // Get connections - following
    let following = sqlx::query_as!(
        UserConnectionWithProfile,
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            p.display_name,
            p.avatar_url,
            uc.connection_type as "connection_type: _",
            uc.status as "status: _",
            uc.created_at
        FROM territory_dk.user_connections uc
        JOIN territory_dk.users u ON u.id = uc.target_user_id
        LEFT JOIN territory_dk.users_profiles p ON p.user_id = u.id
        WHERE uc.user_id = $1
        AND uc.connection_type = 'follow'
        AND uc.status = 'active'
        ORDER BY uc.created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    // Get connections - blocked
    let blocked = sqlx::query_as!(
        UserConnectionWithProfile,
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            p.display_name,
            p.avatar_url,
            uc.connection_type as "connection_type: _",
            uc.status as "status: _",
            uc.created_at
        FROM territory_dk.user_connections uc
        JOIN territory_dk.users u ON u.id = uc.target_user_id
        LEFT JOIN territory_dk.users_profiles p ON p.user_id = u.id
        WHERE uc.user_id = $1
        AND uc.connection_type = 'block'
        AND uc.status = 'active'
        ORDER BY uc.created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(ExportedUserData {
        export_date: Utc::now(),
        user,
        profile,
        links,
        languages,
        settings,
        notification_settings,
        connections: ExportedConnections {
            followers,
            following,
            blocked,
        },
    })
}
