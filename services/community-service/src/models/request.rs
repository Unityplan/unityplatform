use serde::Deserialize;
use validator::Validate;
use uuid::Uuid;
use crate::models::community::CommunityType;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommunityRequest {
    #[validate(length(min = 3, max = 100))]
    pub slug: String,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub community_type: CommunityType,
    pub territory_id: Option<String>,
    pub parent_community_id: Option<Uuid>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommunityRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CommunityFilter {
    pub community_type: Option<CommunityType>,
    pub parent_id: Option<Uuid>,
    pub territory_id: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AssignManagerRequest {
    pub user_id: Uuid,
}

