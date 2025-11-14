use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

// ============================================================================
// Full Settings Response
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SettingsResponse {
    pub user_id: Uuid,
    pub theme: String,
    pub language: String,
    pub timezone: String,
    pub profile_visibility: String,
    pub show_email: bool,
    pub show_location: bool,
    pub allow_messages: String,
    pub email_notifications: bool,
    pub badge_notifications: bool,
    pub course_notifications: bool,
    pub forum_notifications: bool,
    pub marketing_emails: bool,
    pub show_activity: bool,
    pub show_online_status: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// Update Requests
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsRequest {
    #[validate(custom(function = "validate_theme"))]
    pub theme: Option<String>,

    #[validate(length(min = 2, max = 10))]
    pub language: Option<String>,

    #[validate(length(min = 1, max = 50))]
    pub timezone: Option<String>,

    #[validate(custom(function = "validate_profile_visibility"))]
    pub profile_visibility: Option<String>,

    pub show_email: Option<bool>,
    pub show_location: Option<bool>,

    #[validate(custom(function = "validate_allow_messages"))]
    pub allow_messages: Option<String>,

    pub email_notifications: Option<bool>,
    pub badge_notifications: Option<bool>,
    pub course_notifications: Option<bool>,
    pub forum_notifications: Option<bool>,
    pub marketing_emails: Option<bool>,
    pub show_activity: Option<bool>,
    pub show_online_status: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePrivacySettingsRequest {
    #[validate(custom(function = "validate_profile_visibility"))]
    pub profile_visibility: Option<String>,

    pub show_email: Option<bool>,
    pub show_location: Option<bool>,

    #[validate(custom(function = "validate_allow_messages"))]
    pub allow_messages: Option<String>,

    pub show_activity: Option<bool>,
    pub show_online_status: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationSettingsRequest {
    pub email_notifications: Option<bool>,
    pub badge_notifications: Option<bool>,
    pub course_notifications: Option<bool>,
    pub forum_notifications: Option<bool>,
    pub marketing_emails: Option<bool>,
}

// ============================================================================
// Partial Response Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PrivacySettingsResponse {
    pub profile_visibility: String,
    pub show_email: bool,
    pub show_location: bool,
    pub allow_messages: String,
    pub show_activity: bool,
    pub show_online_status: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettingsResponse {
    pub email_notifications: bool,
    pub badge_notifications: bool,
    pub course_notifications: bool,
    pub forum_notifications: bool,
    pub marketing_emails: bool,
}

// ============================================================================
// Validation Functions
// ============================================================================

fn validate_theme(theme: &str) -> Result<(), validator::ValidationError> {
    match theme {
        "light" | "dark" | "system" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_theme")),
    }
}

fn validate_profile_visibility(visibility: &str) -> Result<(), validator::ValidationError> {
    match visibility {
        "public" | "territory" | "private" => Ok(()),
        _ => Err(validator::ValidationError::new(
            "invalid_profile_visibility",
        )),
    }
}

fn validate_allow_messages(allow: &str) -> Result<(), validator::ValidationError> {
    match allow {
        "everyone" | "connections" | "none" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_allow_messages")),
    }
}
