use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Badge definition response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: String,
    pub criteria_type: String,
    pub criteria_value: Option<i32>,
    pub rarity: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // User-specific fields (if authenticated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_has_badge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_progress: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_target: Option<i32>,
}

/// User badge award response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserBadgeResponse {
    pub id: Uuid,
    pub badge_id: Uuid,
    pub badge_name: String,
    pub badge_slug: String,
    pub badge_icon: String,
    pub badge_rarity: String,
    pub awarded_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarded_by: Option<Uuid>,
    pub is_featured: bool,
}

/// Award badge request (admin only)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AwardBadgeRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub badge_slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500))]
    pub reason: Option<String>,
}

/// Revoke badge request (admin only)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RevokeBadgeRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub badge_slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500))]
    pub reason: Option<String>,
}

/// Update badge progress request (internal/system)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgressRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub badge_slug: String,
    #[validate(range(min = 0))]
    pub current_value: i32,
}

/// Toggle featured badge request
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFeaturedRequest {
    pub badge_id: Uuid,
    pub is_featured: bool,
}

/// Register badge request (for services to register their role badges)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBadgeRequest {
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 1000))]
    pub description: String,
    #[validate(length(min = 1, max = 100))]
    pub icon: String,
    #[validate(length(min = 1, max = 50))]
    pub category: String, // 'role', 'achievement', 'code_of_conduct', 'special'
    #[validate(length(min = 1, max = 50))]
    pub criteria_type: String, // 'manual', 'course_completion', etc.
    pub criteria_value: Option<i32>,
    #[validate(length(min = 1, max = 20))]
    pub rarity: String, // 'common', 'rare', 'epic', 'legendary'
    pub is_renewable: Option<bool>,
    pub renewal_days: Option<i32>,
    pub grants_permissions: Vec<String>, // Array of permission strings
}

/// NATS event payloads
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
    pub user_id: Uuid,
    pub username: String,
    pub territory_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseCompletedEvent {
    pub user_id: Uuid,
    pub course_code: String,
    pub score: Option<i32>,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeAwardedEvent {
    pub user_id: Uuid,
    pub badge_slug: String,
    pub badge_name: String,
    pub awarded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeRevokedEvent {
    pub user_id: Uuid,
    pub badge_slug: String,
    pub reason: Option<String>,
    pub revoked_at: DateTime<Utc>,
}
