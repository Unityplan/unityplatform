use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::community::{Community, CommunityType, CommunityRole};

#[derive(Debug, Serialize)]
pub struct CommunityResponse {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub community_type: CommunityType,
    pub territory_id: Option<String>,
    pub parent_community_id: Option<Uuid>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_public: bool,
    pub member_count: i32,
    pub created_at: DateTime<Utc>,
    pub user_membership: Option<UserMembershipResponse>,
}

#[derive(Debug, Serialize)]
pub struct UserMembershipResponse {
    pub is_member: bool,
    pub role: Option<CommunityRole>,
    pub joined_at: Option<DateTime<Utc>>,
}

impl From<Community> for CommunityResponse {
    fn from(c: Community) -> Self {
        Self {
            id: c.id,
            slug: c.slug,
            name: c.name,
            description: c.description,
            community_type: c.community_type,
            territory_id: c.territory_id,
            parent_community_id: c.parent_community_id,
            avatar_url: c.avatar_url,
            banner_url: c.banner_url,
            is_public: c.is_public,
            member_count: c.member_count,
            created_at: c.created_at,
            user_membership: None, // Populated separately if needed
        }
    }
}
