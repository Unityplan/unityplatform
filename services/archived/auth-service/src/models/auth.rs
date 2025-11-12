use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 1))]
    pub invitation_token: String,

    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 8))]
    pub password: String,

    pub display_name: String,
    pub preferred_language: String,
    pub accept_terms: bool,
    pub accept_privacy: bool,
    pub accept_code_of_conduct: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub home_territory_code: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub username: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub full_name: String,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RefreshRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidateResponse {
    pub valid: bool,
    pub user_id: Uuid,
    pub username: String,
    pub territory: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub territory: String,
    pub exp: usize,
    pub iat: usize,
}

impl Clone for Claims {
    fn clone(&self) -> Self {
        Self {
            sub: self.sub.clone(),
            username: self.username.clone(),
            territory: self.territory.clone(),
            exp: self.exp,
            iat: self.iat,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub full_name: String,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
}
