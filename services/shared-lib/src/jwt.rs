use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::AppError;

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: Uuid,
    /// Territory code
    pub territory: String,
    /// Badge slugs the user has (e.g., ["code-of-conduct", "community-manager"])
    /// Used for permission validation without database lookups
    #[serde(default)]
    pub badges: Vec<String>,
    /// Expiration time (as UTC timestamp)
    pub exp: u64,
    /// Issued at (as UTC timestamp)
    pub iat: u64,
}

/// Authenticated user extracted from JWT token
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub territory: String,
    /// Badge slugs from JWT claims - used for permission checks
    pub badges: Vec<String>,
}

impl AuthUser {
    /// Check if the user has a specific badge by slug
    pub fn has_badge(&self, badge_slug: &str) -> bool {
        self.badges.iter().any(|b| b == badge_slug)
    }

    /// Check if the user has all required badges
    pub fn has_all_badges(&self, required_slugs: &[&str]) -> bool {
        required_slugs.iter().all(|slug| self.has_badge(slug))
    }

    /// Check if the user has any of the specified badges
    pub fn has_any_badge(&self, badge_slugs: &[&str]) -> bool {
        badge_slugs.iter().any(|slug| self.has_badge(slug))
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Extract Authorization header
        let auth_header = match req.headers().get("Authorization") {
            Some(h) => h,
            None => {
                return ready(Err(AppError::Unauthorized(
                    "Missing Authorization header".into(),
                )
                .into()))
            }
        };

        // Parse Bearer token
        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return ready(Err(AppError::Unauthorized(
                    "Invalid Authorization header".into(),
                )
                .into()))
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return ready(Err(AppError::Unauthorized(
                "Authorization header must start with 'Bearer '".into(),
            )
            .into()));
        }

        let token = &auth_str[7..]; // Remove "Bearer " prefix

        // Get JWT secret from environment
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "dev_jwt_secret_please_change_in_production".to_string());

        // Validate token
        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
        let validation = Validation::default();

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => ready(Ok(AuthUser {
                id: token_data.claims.sub,
                territory: token_data.claims.territory,
                badges: token_data.claims.badges,
            })),
            Err(e) => ready(Err(
                AppError::Unauthorized(format!("Invalid token: {}", e)).into()
            )),
        }
    }
}
