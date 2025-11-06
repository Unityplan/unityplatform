use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Invitation token model from territory schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InvitationToken {
    pub id: Uuid,
    pub token: String,
    pub token_type: String, // "single_use" or "group"

    // Restrictions
    pub email: Option<String>,
    pub max_uses: i32,
    pub used_count: i32,

    // Metadata
    pub created_by_user_id: Uuid,
    pub purpose: Option<String>,
    pub metadata: Option<serde_json::Value>,

    // Lifecycle
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by_user_id: Option<Uuid>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Record of invitation token usage
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InvitationUse {
    pub id: Uuid,
    pub invitation_token_id: Uuid,
    pub user_id: Uuid,
    pub used_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Request to create a new invitation token
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateInvitationRequest {
    #[validate(custom(function = "validate_token_type"))]
    pub token_type: String, // "single_use" or "group"

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>, // Required for single_use, null for group

    #[validate(range(min = 1, max = 1000, message = "Max uses must be between 1 and 1000"))]
    pub max_uses: i32,

    #[validate(range(min = 1, max = 365, message = "Expiration must be between 1 and 365 days"))]
    pub expires_in_days: Option<i64>, // Default: 7 days

    #[validate(length(max = 500, message = "Purpose must be 500 characters or less"))]
    pub purpose: Option<String>,
}

/// Response containing invitation token details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationResponse {
    pub id: Uuid,
    pub token: String,
    pub token_type: String,
    pub email: Option<String>,
    pub max_uses: i32,
    pub used_count: i32,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub purpose: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<InvitationToken> for InvitationResponse {
    fn from(token: InvitationToken) -> Self {
        Self {
            id: token.id,
            token: token.token,
            token_type: token.token_type,
            email: token.email,
            max_uses: token.max_uses,
            used_count: token.used_count,
            expires_at: token.expires_at,
            is_active: token.is_active,
            purpose: token.purpose,
            created_at: token.created_at,
        }
    }
}

/// Public invitation validation response (no sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationValidationResponse {
    pub valid: bool,
    pub token_type: Option<String>,
    pub email: Option<String>, // Only for single_use tokens
    pub expires_at: Option<DateTime<Utc>>,
    pub uses_remaining: Option<i32>,
    pub error: Option<String>,
}

/// Custom validation for token_type
fn validate_token_type(token_type: &str) -> Result<(), validator::ValidationError> {
    if token_type == "single_use" || token_type == "group" {
        Ok(())
    } else {
        Err(validator::ValidationError::new(
            "token_type must be 'single_use' or 'group'",
        ))
    }
}

impl CreateInvitationRequest {
    /// Validate business logic rules
    pub fn validate_business_rules(&self) -> Result<(), String> {
        // Single-use tokens must have email
        if self.token_type == "single_use" {
            if self.email.is_none() {
                return Err("Email is required for single_use invitation tokens".to_string());
            }
            if self.max_uses != 1 {
                return Err("Single-use tokens must have max_uses = 1".to_string());
            }
        }

        // Group tokens must not have email
        if self.token_type == "group" {
            if self.email.is_some() {
                return Err("Email must not be set for group invitation tokens".to_string());
            }
            if self.max_uses <= 1 {
                return Err("Group tokens must have max_uses > 1".to_string());
            }
        }

        Ok(())
    }
}
