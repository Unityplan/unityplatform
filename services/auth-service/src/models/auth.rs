use serde::{Deserialize, Serialize};
use validator::Validate;

/// Registration request
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(max = 255))]
    pub full_name: Option<String>,

    #[validate(length(min = 2, max = 10, message = "Territory code must be 2-10 characters"))]
    pub territory_code: String,
}

/// Login request
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 2, max = 10))]
    pub territory_code: String,
}

/// Refresh token request
#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,

    #[validate(length(min = 2, max = 10))]
    pub territory_code: String,
}

/// Logout request
#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: super::AuthUserInfo,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64, // seconds
}

/// JWT claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // public_key_hash
    pub territory_code: String,
    pub user_id: String, // UUID as string
    pub username: String,
    pub exp: i64, // Expiration time (Unix timestamp)
    pub iat: i64, // Issued at (Unix timestamp)
}
