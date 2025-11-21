use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Validate invitation token request
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ValidateInvitationRequest {
    /// Invitation token in format XXXX-XXXX-XXXX-XXXX
    #[validate(length(min = 1, max = 255))]
    pub token: String,
}

/// Validate invitation token response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidateInvitationResponse {
    /// Whether the token is valid and usable
    pub valid: bool,

    /// Invitation ID (only if valid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<Uuid>,

    /// User who created the invitation (only if valid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Uuid>,

    /// Number of uses remaining (only if valid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_remaining: Option<i32>,

    /// Expiration timestamp (only if valid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,

    /// Custom metadata (only if valid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl ValidateInvitationResponse {
    /// Create invalid response
    pub fn invalid() -> Self {
        Self {
            valid: false,
            invitation_id: None,
            created_by: None,
            uses_remaining: None,
            expires_at: None,
            metadata: None,
        }
    }

    /// Create valid response with invitation details
    pub fn valid(
        invitation_id: Uuid,
        created_by: Uuid,
        uses_remaining: i32,
        expires_at: Option<DateTime<Utc>>,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            valid: true,
            invitation_id: Some(invitation_id),
            created_by: Some(created_by),
            uses_remaining: Some(uses_remaining),
            expires_at,
            metadata,
        }
    }
}
