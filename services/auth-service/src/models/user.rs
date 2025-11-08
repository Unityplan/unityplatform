use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User model from territory schema
/// ALL personal data stored in territory_*.users table
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,

    // Current authentication
    pub email: Option<String>, // Optional - privacy-first, not required for registration
    #[serde(skip_serializing)]
    pub password_hash: String,

    // Profile data (stays in territory)
    pub username: String,
    pub full_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub phone: Option<String>,

    // Privacy controls
    pub profile_visibility: Option<String>, // 'public', 'community', 'private'
    pub email_notifications: bool,
    pub push_notifications: bool,

    // Status
    pub is_verified: bool,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,

    // Invitation tracking
    pub invited_by_user_id: Option<Uuid>,
    pub invitation_by_token_id: Option<Uuid>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Unused - commenting out for future use
// /// Public user info (safe to share across territories)
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PublicUserInfo {
//     pub username: String,
//     pub display_name: Option<String>,
//     pub avatar_url: Option<String>,
//     pub bio: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub email: Option<String>, // Only if email_visible = true
// }
//
// impl From<User> for PublicUserInfo {
//     fn from(user: User) -> Self {
//         Self {
//             username: user.username,
//             display_name: user.display_name,
//             avatar_url: user.avatar_url,
//             bio: user.bio,
//             email: if user.profile_visibility == Some("public".to_string()) {
//                 user.email // Already Option<String>
//             } else {
//                 None
//             },
//         }
//     }
// }

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
            email: user.email, // Already Option<String>
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            is_verified: user.is_verified,
        }
    }
}
