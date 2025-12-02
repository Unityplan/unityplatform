use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event published when a user requests account deletion (soft delete)
/// 
/// This event is published by auth-service when a user requests deletion.
/// All services should subscribe to this event and soft-delete their user data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeletedEvent {
    /// User ID being deleted
    pub user_id: Uuid,
    /// Territory code
    pub territory: String,
    /// When the user was soft-deleted
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl UserDeletedEvent {
    /// NATS subject for user deletion events
    pub const SUBJECT: &'static str = "user.deleted";
    
    pub fn new(user_id: Uuid, territory: String) -> Self {
        Self {
            user_id,
            territory,
            deleted_at: chrono::Utc::now(),
        }
    }
}
