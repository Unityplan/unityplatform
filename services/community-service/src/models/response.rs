use crate::models::community::{Community, CommunityRole, CommunityType};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

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
            member_count: c.member_count,
            created_at: c.created_at,
            user_membership: None, // Populated separately if needed
        }
    }
}

/// Summary of child groups (Guilds and Study Groups) under a community
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupSummary {
    /// The parent community ID this summary is for
    pub community_id: Uuid,
    /// Number of direct child guilds
    pub guild_count: i32,
    /// Number of direct child study groups
    pub study_group_count: i32,
    /// Total number of guilds (including nested)
    pub total_guild_count: i32,
    /// Total number of study groups (including nested)
    pub total_study_group_count: i32,
    /// Total member count across all child groups
    pub total_members: i32,
    /// Whether any direct child group has a badge requirement
    pub has_badge_requirement: bool,
    /// Name of the first badge requirement found (if any)
    pub badge_name: Option<String>,
    /// ID of the first badge requirement found (if any)
    pub badge_id: Option<Uuid>,
    /// List of direct child communities (only Guild/StudyGroup types)
    pub children: Vec<GroupChild>,
}

/// A child group community summary
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupChild {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub community_type: CommunityType,
    pub member_count: i32,
    pub has_badge_requirement: bool,
    pub badge_name: Option<String>,
}
