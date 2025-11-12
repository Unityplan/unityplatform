use crate::error::{AuthError, AuthResult};
use crate::models::Claims;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

/// Token service for JWT generation and validation
pub struct TokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_ttl: i64, // seconds
    #[allow(dead_code)] // Used for future refresh token implementation
    refresh_token_ttl: i64, // seconds
}

impl TokenService {
    pub fn new(secret: &str, access_token_ttl: i64, refresh_token_ttl: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_token_ttl,
            refresh_token_ttl,
        }
    }

    /// Generate access token (short-lived, 15 minutes default)
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        username: &str,
        territory: &str,
    ) -> AuthResult<String> {
        let now = Utc::now().timestamp() as usize;
        let exp = now + self.access_token_ttl as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            territory: territory.to_string(),
            iat: now,
            exp,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| AuthError::InternalError)
    }

    /// Generate refresh token (random string)
    pub fn generate_refresh_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    /// Validate and decode access token
    pub fn validate_token(&self, token: &str) -> AuthResult<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        Ok(token_data.claims)
    }

    /// Get access token TTL in seconds
    pub fn get_access_token_ttl(&self) -> i64 {
        self.access_token_ttl
    }

    /// Get refresh token TTL in seconds
    pub fn get_refresh_token_ttl(&self) -> i64 {
        self.refresh_token_ttl
    }
}
