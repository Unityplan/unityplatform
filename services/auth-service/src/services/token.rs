use base64::Engine;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use shared_lib::{AppError, Result};
use uuid::Uuid;

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: Uuid,
    /// Territory code
    pub territory: String,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// JWT ID (unique token identifier)
    pub jti: String,
}

/// Token service for JWT generation and validation
#[derive(Clone)]
pub struct TokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl TokenService {
    /// Create a new TokenService with the given secret
    ///
    /// # Arguments
    /// * `secret` - JWT secret key for signing tokens
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    /// Generate an access token with 15 minute expiration
    ///
    /// # Arguments
    /// * `user_id` - The user's UUID
    /// * `territory` - The territory code (e.g., "dk")
    ///
    /// # Returns
    /// * `Result<String>` - The encoded JWT token
    pub fn generate_access_token(&self, user_id: Uuid, territory: &str) -> Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::minutes(15);

        let claims = Claims {
            sub: user_id,
            territory: territory.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("Failed to generate token: {}", e)))
    }

    /// Generate a refresh token (secure random string, 7 day expiration)
    ///
    /// # Returns
    /// * `String` - A 32-byte base64url encoded random string
    pub fn generate_refresh_token() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: [u8; 32] = rng.gen();
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    /// Validate and decode a JWT token
    ///
    /// # Arguments
    /// * `token` - The JWT token string to validate
    ///
    /// # Returns
    /// * `Result<Claims>` - The decoded claims if valid
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }

    /// Extract token from Authorization header
    ///
    /// # Arguments
    /// * `auth_header` - The Authorization header value (e.g., "Bearer <token>")
    ///
    /// # Returns
    /// * `Result<String>` - The extracted token
    pub fn extract_token(auth_header: &str) -> Result<String> {
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized(
                "Invalid Authorization header format".to_string(),
            ));
        }

        let token = auth_header.trim_start_matches("Bearer ").to_string();
        if token.is_empty() {
            return Err(AppError::Unauthorized("Missing token".to_string()));
        }

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_access_token() {
        let service = TokenService::new("test_secret_key_1234567890");
        let user_id = Uuid::new_v4();
        let territory = "dk";

        let token = service.generate_access_token(user_id, territory).unwrap();
        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.territory, territory);
    }

    #[test]
    fn test_generate_refresh_token() {
        let token1 = TokenService::generate_refresh_token();
        let token2 = TokenService::generate_refresh_token();

        // Should be unique random strings
        assert_ne!(token1, token2);
        // Should be 43 characters (32 bytes base64url encoded)
        assert_eq!(token1.len(), 43);
        assert_eq!(token2.len(), 43);
    }

    #[test]
    fn test_validate_expired_token() {
        let service = TokenService::new("test_secret_key_1234567890");

        // Create a token that's already expired
        let claims = Claims {
            sub: Uuid::new_v4(),
            territory: "dk".to_string(),
            exp: (Utc::now() - Duration::hours(1)).timestamp(),
            iat: (Utc::now() - Duration::hours(2)).timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        let token = encode(&Header::default(), &claims, &service.encoding_key).unwrap();
        let result = service.validate_token(&token);

        assert!(result.is_err());
    }

    #[test]
    fn test_extract_token() {
        let auth_header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test";
        let token = TokenService::extract_token(auth_header).unwrap();
        assert_eq!(token, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test");
    }

    #[test]
    fn test_extract_token_invalid_format() {
        let result = TokenService::extract_token("Invalid header");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_token_missing_token() {
        let result = TokenService::extract_token("Bearer ");
        assert!(result.is_err());
    }
}
