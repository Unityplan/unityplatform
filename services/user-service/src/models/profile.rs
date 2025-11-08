use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use super::privacy::PrivacySettings;

/// User profile (extended profile data)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub about: Option<String>,
    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub location: Option<String>,
    pub website_url: Option<String>,
    pub github_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub twitter_handle: Option<String>,
    pub theme: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub profile_visibility: Option<String>,
    pub show_email: Option<bool>,
    pub show_real_name: Option<bool>,
    pub allow_messages_from: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Request to update user profile
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(max = 2000))]
    pub about: Option<Option<String>>,

    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,

    #[validate(length(max = 255))]
    pub location: Option<Option<String>>,

    #[validate(url)]
    pub website_url: Option<Option<String>>,

    #[validate(url)]
    pub github_url: Option<Option<String>>,

    #[validate(url)]
    pub linkedin_url: Option<Option<String>>,

    #[validate(length(max = 100))]
    pub twitter_handle: Option<Option<String>>,

    #[validate(custom = "validate_theme")]
    pub theme: Option<String>,

    pub metadata: Option<serde_json::Value>,

    #[validate(custom = "validate_visibility")]
    pub profile_visibility: Option<String>,

    pub show_email: Option<bool>,
    pub show_real_name: Option<bool>,

    #[validate(custom = "validate_message_policy")]
    pub allow_messages_from: Option<String>,
}

fn validate_theme(theme: &str) -> Result<(), validator::ValidationError> {
    if ["light", "dark", "auto"].contains(&theme) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_theme"))
    }
}

fn validate_visibility(visibility: &str) -> Result<(), validator::ValidationError> {
    if ["public", "connections", "private"].contains(&visibility) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_visibility"))
    }
}

fn validate_message_policy(policy: &str) -> Result<(), validator::ValidationError> {
    if ["everyone", "connections", "nobody"].contains(&policy) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_message_policy"))
    }
}

/// Full user profile (combines territory.users + territory.user_profiles)
#[derive(Debug, Serialize)]
pub struct FullUserProfile {
    // From territory.users
    pub user_id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,

    // From territory.user_profiles
    pub about: Option<String>,
    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub location: Option<String>,
    pub website_url: Option<String>,
    pub github_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub twitter_handle: Option<String>,
    pub theme: Option<String>,
    pub metadata: Option<serde_json::Value>,

    // Privacy settings
    pub privacy: PrivacySettings,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Public user profile (filtered based on privacy settings)
#[derive(Debug, Serialize)]
pub struct PublicUserProfile {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub about: Option<String>,
    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub location: Option<String>,
    pub website_url: Option<String>,
    pub github_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub twitter_handle: Option<String>,

    // Conditionally shown based on privacy
    pub full_name: Option<String>,
    pub email: Option<String>,
}
