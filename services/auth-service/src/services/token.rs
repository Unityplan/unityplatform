use crate::models::Claims;
use anyhow::Result;
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
        public_key_hash: &str,
        territory_code: &str,
        user_id: Uuid,
        username: &str,
    ) -> Result<String> {
        let now = Utc::now().timestamp();
        let exp = now + self.access_token_ttl;

        let claims = Claims {
            sub: public_key_hash.to_string(),
            territory_code: territory_code.to_string(),
            user_id: user_id.to_string(),
            username: username.to_string(),
            iat: now,
            exp,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow::anyhow!("Failed to generate access token: {}", e))
    }

    /// Generate refresh token (random string)
    pub fn generate_refresh_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    /// Validate and decode access token
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| anyhow::anyhow!("Invalid token: {}", e))?;

        Ok(token_data.claims)
    }

    /// Get access token TTL in seconds
    pub fn get_access_token_ttl(&self) -> i64 {
        self.access_token_ttl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let service = TokenService::new("test_secret", 900, 604800);
        let public_key_hash = "test_hash_123";
        let territory_code = "DK";
        let user_id = Uuid::new_v4();
        let username = "testuser";

        let token = service
            .generate_access_token(public_key_hash, territory_code, user_id, username)
            .unwrap();

        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, public_key_hash);
        assert_eq!(claims.territory_code, territory_code);
        assert_eq!(claims.user_id, user_id.to_string());
        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_invalid_token() {
        let service = TokenService::new("test_secret", 900, 604800);
        let result = service.validate_token("invalid.token.here");

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_refresh_token() {
        let service = TokenService::new("test_secret", 900, 604800);
        let token1 = service.generate_refresh_token();
        let token2 = service.generate_refresh_token();

        // Refresh tokens should be unique
        assert_ne!(token1, token2);

        // Should be valid UUIDs
        assert!(Uuid::parse_str(&token1).is_ok());
        assert!(Uuid::parse_str(&token2).is_ok());
    }
}
