use crate::models::community::{CommunityType, CoverageArea, RequirementContext};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCommunityRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub community_type: CommunityType,
    pub territory_id: Option<String>,
    pub parent_community_id: Option<Uuid>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub coverage_area: Option<CoverageArea>,
    #[serde(default = "default_inherit_requirements")]
    pub inherit_requirements: bool,
    #[serde(default)]
    pub initial_requirements: Vec<AddRequirementRequest>,
}

fn default_inherit_requirements() -> bool {
    true
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCommunityRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_community_id: Option<Uuid>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub coverage_area: Option<CoverageArea>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CommunityFilter {
    pub community_type: Option<CommunityType>,
    pub parent_id: Option<Uuid>,
    pub territory_id: Option<String>,
    pub search: Option<String>,
    /// Maximum number of results to return (default: 100, max: 500)
    pub limit: Option<i32>,
    /// Number of results to skip for pagination
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignManagerRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AddRequirementRequest {
    pub badge_id: Uuid,
    pub context: RequirementContext,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RemoveRequirementQuery {
    pub context: RequirementContext,
}
