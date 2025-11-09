// Library interface for user-service
// This allows tests to import from user_service::*

pub mod handlers;
pub mod models;
pub mod services;

// Re-export commonly used types
pub use models::{
    connection::{ConnectionResponse, UserBlock, UserConnection},
    privacy::PrivacySettings,
    profile::{FullUserProfile, PublicUserProfile, UpdateProfileRequest, UserProfile},
};

pub use services::{StorageService, UserService};
