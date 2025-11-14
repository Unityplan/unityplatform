use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Registration request
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    #[schema(example = "johndoe", min_length = 3, max_length = 50)]
    pub username: String,

    #[validate(email)]
    #[schema(example = "john@example.com", nullable = true)]
    pub email: Option<String>,

    #[validate(length(min = 8, max = 128))]
    #[schema(example = "secure_password_123", min_length = 8, max_length = 128)]
    pub password: String,

    #[validate(length(min = 2, max = 10))]
    #[schema(example = "dk", min_length = 2, max_length = 10)]
    pub territory: String,

    /// Optional invitation token (for validation if invitation-service is available)
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub invitation_token: Option<String>,
}

/// Login request
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    #[schema(example = "johndoe")]
    pub username: String,

    #[validate(length(min = 1))]
    #[schema(example = "secure_password_123")]
    pub password: String,

    #[validate(length(min = 2, max = 10))]
    #[schema(example = "dk")]
    pub territory: String,
}

/// Token refresh request
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    #[schema(example = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")]
    pub refresh_token: String,
}

/// Logout request
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {
    #[schema(example = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")]
    pub refresh_token: String,
}

/// Authentication response (returned on login/register)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,

    #[schema(example = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")]
    pub refresh_token: String,

    #[schema(example = "Bearer")]
    pub token_type: String,

    #[schema(example = 900)]
    pub expires_in: i64,
}

impl AuthResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900, // 15 minutes in seconds
        }
    }
}

/// Token validation response
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidateResponse {
    pub valid: bool,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub user_id: Option<Uuid>,

    #[schema(example = "dk")]
    pub territory: Option<String>,
}

/// Health check response
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    #[schema(example = "ok")]
    pub status: String,

    #[schema(example = "auth-service")]
    pub service: String,

    #[schema(example = "0.1.0-alpha.1")]
    pub version: String,
}
