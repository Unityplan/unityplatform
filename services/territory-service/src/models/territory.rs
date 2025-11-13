use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Territory response model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TerritoryResponse {
    pub code: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub pod_url: String,
    pub api_url: String,
    pub status: String,
    pub language_code: String,
    pub timezone: String,
    pub currency_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Territory settings request
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

// Manual Validate implementation since all fields are optional
impl validator::Validate for UpdateSettingsRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

/// Create territory request (Platform Manager only)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTerritoryRequest {
    pub code: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub pod_url: String,
    pub api_url: String,
    pub language_code: String,
    pub timezone: String,
    pub currency_code: Option<String>,
}

/// Territory statistics response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TerritoryStatsResponse {
    pub total_users: i64,
    pub active_users_7d: i64,
    pub active_users_30d: i64,
    pub total_communities: i64,
    pub total_posts: i64,
    pub storage_used_mb: i64,
    pub calculated_at: DateTime<Utc>,
}

/// Territory settings response (full details for managers)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TerritorySettingsResponse {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub language_code: String,
    pub timezone: String,
    pub currency_code: Option<String>,
    pub registration_enabled: bool,
    pub invitation_required: bool,
    pub max_users: i32,
    pub primary_color: Option<String>,
    pub logo_url: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// Territory manager response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TerritoryManagerResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub territory_code: String,
    pub assigned_by: Option<Uuid>,
    pub assigned_at: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Assign territory manager request (Platform Manager only)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssignManagerRequest {
    pub user_id: Uuid,
    pub notes: Option<String>,
}
