use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User model from territory schema
/// ALL personal data stored in territory_*.users table
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,

    // Cryptographic identity (for cross-territory coordination)
    pub public_key_hash: Option<String>,

    // Current authentication
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,

    // Profile data (stays in territory)
    pub username: String,
    pub full_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,

    // Privacy controls
    pub email_visible: bool,
    pub profile_public: bool,
    pub data_export_requested: bool,

    // Status
    pub is_verified: bool,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,

    // Invitation tracking
    pub invited_by_token_id: Option<Uuid>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Public user info (safe to share across territories)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUserInfo {
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>, // Only if email_visible = true
}

impl From<User> for PublicUserInfo {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            bio: user.bio,
            email: if user.email_visible { user.email } else { None },
        }
    }
}

/// User info returned after authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
}

impl From<User> for AuthUserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            is_verified: user.is_verified,
        }
    }
}
