use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

/// Request to mark an invitation as used after successful registration
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UseInvitationRequest {
    /// The invitation token that was used
    #[validate(length(min = 19, max = 19, message = "Token must be exactly 19 characters (XXXX-XXXX-XXXX-XXXX)"))]
    pub token: String,
    
    /// User ID who used the invitation (from auth-service after successful registration)
    pub used_by: Uuid,
    
    /// Optional IP address of the user
    #[validate(length(max = 45))]
    pub ip_address: Option<String>,
    
    /// Optional user agent string
    #[validate(length(max = 512))]
    pub user_agent: Option<String>,
}

/// Response after recording invitation usage
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UseInvitationResponse {
    /// The invitation ID that was used
    pub invitation_id: Uuid,
    
    /// Number of uses remaining for this invitation
    pub uses_remaining: i32,
    
    /// Whether the invitation is now fully used
    pub fully_used: bool,
}

impl UseInvitationResponse {
    /// Create a new use response
    pub fn new(invitation_id: Uuid, uses_remaining: i32, fully_used: bool) -> Self {
        Self {
            invitation_id,
            uses_remaining,
            fully_used,
        }
    }
}
