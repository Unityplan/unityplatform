use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CommunityType {
    Physical,
    Guild,
    StudyGroup,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Community {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(rename = "type")]
    pub community_type: CommunityType,
    pub territory_id: Option<String>,
    pub parent_community_id: Option<Uuid>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_public: bool,
    pub member_count: i32,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CommunityRole {
    Admin,
    Moderator,
    Member,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunityMember {
    pub community_id: Uuid,
    pub user_id: Uuid,
    pub role: CommunityRole,
    pub joined_at: DateTime<Utc>,
    pub invited_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunitySettings {
    pub community_id: Uuid,
    pub allow_join_requests: bool,
    pub require_approval: bool,
    pub allow_invitations: bool,
    pub allow_public_posts: bool,
    pub updated_at: DateTime<Utc>,
}
