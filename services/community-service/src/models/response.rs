use crate::models::community::{Community, CommunityRole, CommunityType};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
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

/// Minimal geo marker data for map view (lightweight)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GeoMarker {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub community_type: CommunityType,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub coverage_area: Option<serde_json::Value>,
    pub parent_community_id: Option<Uuid>,
}

/// Internal row type for geo marker query
#[derive(Debug, FromRow)]
pub struct GeoMarkerRow {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub community_type: CommunityType,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub coverage_area: Option<sqlx::types::Json<serde_json::Value>>,
    pub parent_community_id: Option<Uuid>,
}

impl From<GeoMarkerRow> for GeoMarker {
    fn from(row: GeoMarkerRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            slug: row.slug,
            community_type: row.community_type,
            location_lat: row.location_lat,
            location_lng: row.location_lng,
            coverage_area: row.coverage_area.map(|j| j.0),
            parent_community_id: row.parent_community_id,
        }
    }
}

/// Community with context (ancestors and children) for flow view
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommunityContext {
    /// The focal community
    pub community: Community,
    /// Ancestors from root to parent (ordered root-first)
    pub ancestors: Vec<Community>,
    /// Direct children of this community
    pub children: Vec<Community>,
    /// Whether this community has more children than returned
    pub has_more_children: bool,
    /// Total count of children
    pub children_count: i64,
}

/// Paginated response for infinite scroll
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedCommunities {
    pub items: Vec<Community>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}
