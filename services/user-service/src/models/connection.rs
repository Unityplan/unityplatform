use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Connection type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionType {
    Follow,
    Block,
}

impl ConnectionType {
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionType::Follow => "follow",
            ConnectionType::Block => "block",
        }
    }
}

/// Connection status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Active,
    Pending,
    Rejected,
}

impl ConnectionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionStatus::Active => "active",
            ConnectionStatus::Pending => "pending",
            ConnectionStatus::Rejected => "rejected",
        }
    }
}

/// User connection database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConnection {
    pub user_id: Uuid,
    pub target_user_id: Uuid,
    pub connection_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User connection response with user details
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionResponse {
    /// Target user ID
    pub user_id: Uuid,
    /// Target user's username
    pub username: String,
    /// Connection type (follow or block)
    pub connection_type: ConnectionType,
    /// Connection status
    pub status: ConnectionStatus,
    /// When the connection was created
    pub created_at: DateTime<Utc>,
}

/// Paginated connections response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsListResponse {
    /// List of connections
    pub connections: Vec<ConnectionResponse>,
    /// Total count of connections
    pub total: i64,
    /// Current page offset
    pub offset: i64,
    /// Page size limit
    pub limit: i64,
}

/// User search result
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResult {
    /// User ID
    pub id: Uuid,
    /// Username
    pub username: String,
    /// User's display name (from profile)
    pub display_name: Option<String>,
    /// User's avatar URL (from profile)
    pub avatar_url: Option<String>,
    /// User's bio (from profile)
    pub bio: Option<String>,
    /// Whether current user follows this user
    pub is_following: bool,
    /// Whether this user follows current user
    pub is_follower: bool,
    /// Whether current user has blocked this user
    pub is_blocked: bool,
}

/// User search response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResponse {
    /// List of users
    pub users: Vec<UserSearchResult>,
    /// Total count matching search
    pub total: i64,
    /// Current page offset
    pub offset: i64,
    /// Page size limit
    pub limit: i64,
}
