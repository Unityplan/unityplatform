use shared_lib::{AppError, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    NotificationSettingsResponse, PrivacySettingsResponse, SettingsResponse,
    UpdateNotificationSettingsRequest, UpdatePrivacySettingsRequest, UpdateSettingsRequest,
};

/// Settings service - business logic for user settings management
pub struct SettingsService;

impl SettingsService {
    /// Get user settings (creates default settings if not exists)
    pub async fn get_settings(
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<SettingsResponse> {
        let query = format!(
            "SELECT * FROM territory_{}.user_users_settings WHERE user_id = $1",
            territory
        );

        let settings = sqlx::query_as::<_, SettingsResponse>(&query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        // If no settings exist, create default settings
        match settings {
            Some(s) => Ok(s),
            None => Self::create_default_settings(user_id, territory, pool).await,
        }
    }

    /// Get privacy settings only
    pub async fn get_privacy_settings(
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<PrivacySettingsResponse> {
        let query = format!(
            "SELECT profile_visibility, show_email, show_location, allow_messages, 
                    show_activity, show_online_status
             FROM territory_{}.user_users_settings 
             WHERE user_id = $1",
            territory
        );

        let settings = sqlx::query_as::<_, PrivacySettingsResponse>(&query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        // If no settings exist, create defaults and return privacy portion
        match settings {
            Some(s) => Ok(s),
            None => {
                let full_settings = Self::create_default_settings(user_id, territory, pool).await?;
                Ok(PrivacySettingsResponse {
                    profile_visibility: full_settings.profile_visibility,
                    show_email: full_settings.show_email,
                    show_location: full_settings.show_location,
                    allow_messages: full_settings.allow_messages,
                    show_activity: full_settings.show_activity,
                    show_online_status: full_settings.show_online_status,
                })
            }
        }
    }

    /// Get notification settings only
    pub async fn get_notification_settings(
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<NotificationSettingsResponse> {
        let query = format!(
            "SELECT email_notifications, badge_notifications, course_notifications, 
                    forum_notifications, marketing_emails
             FROM territory_{}.user_users_settings 
             WHERE user_id = $1",
            territory
        );

        let settings = sqlx::query_as::<_, NotificationSettingsResponse>(&query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        // If no settings exist, create defaults and return notification portion
        match settings {
            Some(s) => Ok(s),
            None => {
                let full_settings = Self::create_default_settings(user_id, territory, pool).await?;
                Ok(NotificationSettingsResponse {
                    email_notifications: full_settings.email_notifications,
                    badge_notifications: full_settings.badge_notifications,
                    course_notifications: full_settings.course_notifications,
                    forum_notifications: full_settings.forum_notifications,
                    marketing_emails: full_settings.marketing_emails,
                })
            }
        }
    }

    /// Update full settings
    pub async fn update_settings(
        user_id: Uuid,
        territory: &str,
        request: UpdateSettingsRequest,
        pool: &PgPool,
    ) -> Result<SettingsResponse> {
        // Ensure settings exist
        let _ = Self::get_settings(user_id, territory, pool).await?;

        let query = format!(
            "UPDATE territory_{}.user_users_settings 
             SET theme = COALESCE($2, theme),
                 language = COALESCE($3, language),
                 timezone = COALESCE($4, timezone),
                 profile_visibility = COALESCE($5, profile_visibility),
                 show_email = COALESCE($6, show_email),
                 show_location = COALESCE($7, show_location),
                 allow_messages = COALESCE($8, allow_messages),
                 email_notifications = COALESCE($9, email_notifications),
                 badge_notifications = COALESCE($10, badge_notifications),
                 course_notifications = COALESCE($11, course_notifications),
                 forum_notifications = COALESCE($12, forum_notifications),
                 marketing_emails = COALESCE($13, marketing_emails),
                 show_activity = COALESCE($14, show_activity),
                 show_online_status = COALESCE($15, show_online_status),
                 updated_at = NOW()
             WHERE user_id = $1
             RETURNING *",
            territory
        );

        sqlx::query_as::<_, SettingsResponse>(&query)
            .bind(user_id)
            .bind(request.theme)
            .bind(request.language)
            .bind(request.timezone)
            .bind(request.profile_visibility)
            .bind(request.show_email)
            .bind(request.show_location)
            .bind(request.allow_messages)
            .bind(request.email_notifications)
            .bind(request.badge_notifications)
            .bind(request.course_notifications)
            .bind(request.forum_notifications)
            .bind(request.marketing_emails)
            .bind(request.show_activity)
            .bind(request.show_online_status)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Update privacy settings only
    pub async fn update_privacy_settings(
        user_id: Uuid,
        territory: &str,
        request: UpdatePrivacySettingsRequest,
        pool: &PgPool,
    ) -> Result<PrivacySettingsResponse> {
        // Ensure settings exist
        let _ = Self::get_settings(user_id, territory, pool).await?;

        let query = format!(
            "UPDATE territory_{}.user_users_settings 
             SET profile_visibility = COALESCE($2, profile_visibility),
                 show_email = COALESCE($3, show_email),
                 show_location = COALESCE($4, show_location),
                 allow_messages = COALESCE($5, allow_messages),
                 show_activity = COALESCE($6, show_activity),
                 show_online_status = COALESCE($7, show_online_status),
                 updated_at = NOW()
             WHERE user_id = $1
             RETURNING profile_visibility, show_email, show_location, allow_messages,
                       show_activity, show_online_status",
            territory
        );

        sqlx::query_as::<_, PrivacySettingsResponse>(&query)
            .bind(user_id)
            .bind(request.profile_visibility)
            .bind(request.show_email)
            .bind(request.show_location)
            .bind(request.allow_messages)
            .bind(request.show_activity)
            .bind(request.show_online_status)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Update notification settings only
    pub async fn update_notification_settings(
        user_id: Uuid,
        territory: &str,
        request: UpdateNotificationSettingsRequest,
        pool: &PgPool,
    ) -> Result<NotificationSettingsResponse> {
        // Ensure settings exist
        let _ = Self::get_settings(user_id, territory, pool).await?;

        let query = format!(
            "UPDATE territory_{}.user_users_settings 
             SET email_notifications = COALESCE($2, email_notifications),
                 badge_notifications = COALESCE($3, badge_notifications),
                 course_notifications = COALESCE($4, course_notifications),
                 forum_notifications = COALESCE($5, forum_notifications),
                 marketing_emails = COALESCE($6, marketing_emails),
                 updated_at = NOW()
             WHERE user_id = $1
             RETURNING email_notifications, badge_notifications, course_notifications,
                       forum_notifications, marketing_emails",
            territory
        );

        sqlx::query_as::<_, NotificationSettingsResponse>(&query)
            .bind(user_id)
            .bind(request.email_notifications)
            .bind(request.badge_notifications)
            .bind(request.course_notifications)
            .bind(request.forum_notifications)
            .bind(request.marketing_emails)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Create default settings for a new user
    async fn create_default_settings(
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<SettingsResponse> {
        let query = format!(
            "INSERT INTO territory_{}.user_users_settings (user_id)
             VALUES ($1)
             ON CONFLICT (user_id) DO NOTHING
             RETURNING *",
            territory
        );

        sqlx::query_as::<_, SettingsResponse>(&query)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }
}
