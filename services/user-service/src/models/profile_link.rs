use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Profile link data structure (social media, website, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProfileLink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub label: String,
    pub url: String,
    pub icon: Option<String>,
    pub display_order: i32,
    pub is_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Profile link response (API representation)
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfileLinkResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,

    #[schema(example = "GitHub")]
    pub label: String,

    #[schema(example = "https://github.com/johndoe")]
    pub url: String,

    #[schema(example = "github")]
    pub icon: Option<String>,

    #[schema(example = 0)]
    pub display_order: i32,

    #[schema(example = true)]
    pub is_visible: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ProfileLink> for ProfileLinkResponse {
    fn from(link: ProfileLink) -> Self {
        Self {
            id: link.id,
            label: link.label,
            url: link.url,
            icon: link.icon,
            display_order: link.display_order,
            is_visible: link.is_visible,
            created_at: link.created_at,
            updated_at: link.updated_at,
        }
    }
}

/// Create profile link request
#[derive(Debug, Deserialize, validator::Validate, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileLinkRequest {
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "GitHub")]
    pub label: String,

    #[validate(url)]
    #[schema(example = "https://github.com/johndoe")]
    pub url: String,

    #[validate(length(max = 50))]
    #[schema(example = "github")]
    pub icon: Option<String>,

    #[schema(example = 0)]
    pub display_order: Option<i32>,

    #[schema(example = true)]
    pub is_visible: Option<bool>,
}

/// Update profile link request
#[derive(Debug, Deserialize, validator::Validate, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileLinkRequest {
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "GitHub Profile")]
    pub label: Option<String>,

    #[validate(url)]
    #[schema(example = "https://github.com/johndoe")]
    pub url: Option<String>,

    #[validate(length(max = 50))]
    #[schema(example = "github")]
    pub icon: Option<String>,

    #[schema(example = 1)]
    pub display_order: Option<i32>,

    #[schema(example = true)]
    pub is_visible: Option<bool>,
}
