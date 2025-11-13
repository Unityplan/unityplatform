use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User profile data structure (matches database schema exactly)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Profile {
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub about: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Profile response (includes username from users table)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,

    #[schema(example = "johndoe")]
    pub username: String,

    #[schema(example = "John Doe")]
    pub display_name: Option<String>,

    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,

    #[schema(example = "Software developer and open source enthusiast")]
    pub bio: Option<String>,

    #[schema(example = "I'm passionate about building user-centric applications...")]
    pub about: Option<String>,

    #[schema(example = "Copenhagen, Denmark")]
    pub location: Option<String>,

    #[schema(example = "https://example.com")]
    pub website: Option<String>,

    #[schema(example = json!(["rust", "web development", "open source"]))]
    pub interests: Option<Vec<String>>,

    #[schema(example = json!(["Rust", "TypeScript", "PostgreSQL"]))]
    pub skills: Option<Vec<String>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
