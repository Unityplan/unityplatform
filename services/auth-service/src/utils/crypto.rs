use sha2::{Digest, Sha256};

/// Generate public_key_hash from user credentials
/// This is a temporary solution until we have WebAuthn/Holochain
/// Format: SHA256(email || username || salt)
pub fn generate_public_key_hash(email: &str, username: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(email.as_bytes());
    hasher.update(b"::"); // Separator
    hasher.update(username.as_bytes());
    hasher.update(b"::unityplan"); // Platform salt

    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_public_key_hash() {
        let hash1 = generate_public_key_hash("test@example.com", "testuser");
        let hash2 = generate_public_key_hash("test@example.com", "testuser");
        let hash3 = generate_public_key_hash("different@example.com", "testuser");

        // Same input = same hash
        assert_eq!(hash1, hash2);

        // Different input = different hash
        assert_ne!(hash1, hash3);

        // Hash is 64 characters (SHA256 hex)
        assert_eq!(hash1.len(), 64);
    }
}
