use serde::{Deserialize, Serialize};

/// Privacy settings for user profiles (embedded in UserProfile)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub profile_visibility: String, // public, connections, private
    pub show_email: bool,
    pub show_real_name: bool,
    pub allow_messages_from: String, // everyone, connections, nobody
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            profile_visibility: "public".to_string(),
            show_email: false,
            show_real_name: true,
            allow_messages_from: "everyone".to_string(),
        }
    }
}
