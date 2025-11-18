use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Invitation token database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Invitation {
    pub id: Uuid,
    pub token: String,
    /// User ID who created this invitation (references global.registry_username)
    pub created_by: Uuid,
    pub max_uses: i32,
    pub uses_count: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub revoked_at: Option<DateTime<Utc>>,
    /// User ID who revoked this invitation (references global.registry_username)
    pub revoked_by: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create invitation request
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateInvitationRequest {
    /// Maximum number of uses (1 = single-use, 0 = unlimited)
    #[serde(default = "default_max_uses")]
    pub max_uses: i32,

    /// Number of days until expiration (null = never expires)
    pub expires_in_days: Option<i32>,

    /// Optional metadata (e.g., community_id, purpose)
    pub metadata: Option<serde_json::Value>,
}

fn default_max_uses() -> i32 {
    1
}

/// Create invitation response
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateInvitationResponse {
    pub id: Uuid,
    pub token: String,
    pub created_by: Uuid,
    pub max_uses: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub invite_url: String,
}
