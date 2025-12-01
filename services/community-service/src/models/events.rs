use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityManagerAssignedEvent {
    pub community_id: Uuid,
    pub user_id: Uuid,
    pub assigned_by: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityManagerRevokedEvent {
    pub community_id: Uuid,
    pub user_id: Uuid,
    pub revoked_by: Uuid,
    pub timestamp: DateTime<Utc>,
}

