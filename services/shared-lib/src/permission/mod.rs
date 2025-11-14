//! Permission system for badge-based authorization
//!
//! This module provides:
//! - `PermissionChecker`: Service for checking user permissions based on badges
//! - `RequirePermission`: Middleware for enforcing permission requirements
//! - Permission caching for performance
//! - Support for service-owned role badges

mod checker;
mod middleware;

pub use checker::PermissionChecker;
pub use middleware::{RequireAnyPermission, RequirePermission};

/// Common permission constants used across services
pub mod permissions {
    // Platform-level permissions
    pub const MANAGE_PLATFORM: &str = "manage_platform";
    pub const CREATE_TERRITORY: &str = "create_territory";
    pub const MANAGE_TERRITORIES: &str = "manage_territories";
    pub const ASSIGN_BADGES: &str = "assign_badges";
    pub const VIEW_ALL_USERS: &str = "view_all_users";

    // Territory-level permissions
    pub const MANAGE_TERRITORY_SETTINGS: &str = "manage_territory_settings";
    pub const VIEW_TERRITORY_STATS: &str = "view_territory_stats";
    pub const MANAGE_COMMUNITIES: &str = "manage_communities";

    // Community-level permissions
    pub const MANAGE_COMMUNITY: &str = "manage_community";
    pub const MODERATE_CONTENT: &str = "moderate_content";
    pub const ASSIGN_ROLES: &str = "assign_roles";

    // User permissions (granted by Code of Conduct badge)
    pub const CREATE_POST: &str = "create_post";
    pub const CREATE_TOPIC: &str = "create_topic";
    pub const COMMENT: &str = "comment";
    pub const VOTE: &str = "vote";
    pub const JOIN_COMMUNITY: &str = "join_community";

    // Portal-specific permissions
    pub const MANAGE_PORTAL: &str = "manage_portal";
    pub const DEVELOP_PORTAL: &str = "develop_portal";
    pub const TRANSLATE_PORTAL: &str = "translate_portal";
    pub const MANAGE_PORTAL_INFRASTRUCTURE: &str = "manage_portal_infrastructure";
    pub const TEST_PORTAL: &str = "test_portal";

    // Course-specific permissions
    pub const MANAGE_COURSES: &str = "manage_courses";
    pub const CREATE_COURSE: &str = "create_course";
    pub const MODERATE_COURSE: &str = "moderate_course";
    pub const GRADE_ASSIGNMENTS: &str = "grade_assignments";

    // Forum-specific permissions
    pub const MANAGE_FORUM: &str = "manage_forum";
    pub const MODERATE_FORUM: &str = "moderate_forum";
    pub const PIN_TOPICS: &str = "pin_topics";
    pub const LOCK_TOPICS: &str = "lock_topics";
}
