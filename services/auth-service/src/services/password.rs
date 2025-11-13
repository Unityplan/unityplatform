use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use shared_lib::{AppError, Result};

/// Password hashing service using Argon2id
pub struct PasswordService;

impl PasswordService {
    /// Hash a plaintext password using Argon2id
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash
    ///
    /// # Returns
    /// * `Result<String>` - The hashed password in PHC string format
    pub fn hash(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?;

        Ok(password_hash.to_string())
    }

    /// Verify a plaintext password against a hashed password
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify
    /// * `hash` - The hashed password to verify against
    ///
    /// # Returns
    /// * `Result<bool>` - True if password matches, false otherwise
    pub fn verify(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "secure_password_123";
        let hash = PasswordService::hash(password).unwrap();

        // Hash should be in PHC string format starting with $argon2
        assert!(hash.starts_with("$argon2"));
        assert!(hash.len() > 50);
    }

    #[test]
    fn test_verify_password_success() {
        let password = "secure_password_123";
        let hash = PasswordService::hash(password).unwrap();

        let result = PasswordService::verify(password, &hash).unwrap();
        assert!(result);
    }

    #[test]
    fn test_verify_password_failure() {
        let password = "secure_password_123";
        let wrong_password = "wrong_password";
        let hash = PasswordService::hash(password).unwrap();

        let result = PasswordService::verify(wrong_password, &hash).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let password = "secure_password_123";
        let hash1 = PasswordService::hash(password).unwrap();
        let hash2 = PasswordService::hash(password).unwrap();

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(PasswordService::verify(password, &hash1).unwrap());
        assert!(PasswordService::verify(password, &hash2).unwrap());
    }
}
