use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

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
#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateInvitationRequest {
    /// Maximum number of uses (1 = single-use, 0 = unlimited)
    #[serde(default = "default_max_uses")]
    #[validate(range(min = 0, max = 1000))]
    pub max_uses: i32,

    /// Number of days until expiration (null = never expires)
    #[validate(range(min = 1, max = 365))]
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

/// List invitations query parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct ListInvitationsQuery {
    /// Filter by status (active, used, expired, revoked)
    pub status: Option<String>,
    /// Page number (default: 1)
    #[serde(default = "default_page")]
    pub page: i64,
    /// Results per page (default: 20, max: 100)
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

/// Invitation use record
#[derive(Debug, Serialize, ToSchema)]
pub struct InvitationUse {
    pub user_id: Uuid,
    pub username: String,
    pub used_at: DateTime<Utc>,
    pub ip_address: Option<String>,
}

/// Invitation with usage details
#[derive(Debug, Serialize, ToSchema)]
pub struct InvitationWithUses {
    pub id: Uuid,
    pub token: String,
    pub max_uses: i32,
    pub uses_count: i32,
    pub is_active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub status: String,
    pub uses: Vec<InvitationUse>,
}

/// List invitations response
#[derive(Debug, Serialize, ToSchema)]
pub struct ListInvitationsResponse {
    pub invitations: Vec<InvitationWithUses>,
    pub pagination: PaginationInfo,
}

/// Pagination information
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationInfo {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

/// Get invitation uses response
#[derive(Debug, Serialize, ToSchema)]
pub struct GetInvitationUsesResponse {
    pub invitation_id: Uuid,
    pub token: String,
    pub uses: Vec<InvitationUse>,
    pub total_uses: i32,
    pub max_uses: i32,
}

/// Revoke invitation response
#[derive(Debug, Serialize, ToSchema)]
pub struct RevokeInvitationResponse {
    pub invitation_id: Uuid,
    pub revoked_at: DateTime<Utc>,
}

