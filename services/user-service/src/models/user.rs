use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

// ============================================================================
// USER MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub territory_code: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub totp_enabled: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// USER PROFILE MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub about: Option<String>,
    #[sqlx(default)]
    pub interests: Vec<String>,
    #[sqlx(default)]
    pub skills: Vec<String>,
    #[sqlx(default)]
    pub languages: Vec<String>,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileRequest {
    #[validate(length(max = 100))]
    pub display_name: Option<String>,

    #[validate(url)]
    pub avatar_url: Option<String>,

    #[validate(length(max = 280))]
    pub bio: Option<String>,

    pub about: Option<String>,

    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub location: Option<String>,
}

// ============================================================================
// PROFILE LINK MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ProfileLink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub label: String,
    pub url: String,
    pub icon: Option<String>,
    pub display_order: i32,
    pub is_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateProfileLinkRequest {
    #[validate(length(min = 1, max = 100))]
    pub label: String,

    #[validate(url)]
    pub url: String,

    pub icon: Option<String>,
    pub is_visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileLinkRequest {
    #[validate(length(min = 1, max = 100))]
    pub label: Option<String>,

    #[validate(url)]
    pub url: Option<String>,

    pub icon: Option<String>,
    pub is_visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ReorderLinksRequest {
    pub link_ids: Vec<Uuid>,
}

// ============================================================================
// LANGUAGE PROFICIENCY MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ProficiencyLevel {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "fluent")]
    Fluent,
    #[serde(rename = "advanced")]
    Advanced,
    #[serde(rename = "intermediate")]
    Intermediate,
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "learning")]
    Learning,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LanguageProficiency {
    pub id: Uuid,
    pub user_id: Uuid,
    pub language_code: String,
    pub language_name: String,
    pub spoken_level: ProficiencyLevel,
    pub written_level: ProficiencyLevel,
    pub reading_level: ProficiencyLevel,
    pub listening_level: ProficiencyLevel,
    pub display_order: i32,
    pub is_preferred: bool,
    pub show_on_profile: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateLanguageProficiencyRequest {
    #[validate(length(min = 2, max = 10))]
    pub language_code: String,

    #[validate(length(min = 1, max = 100))]
    pub language_name: String,

    pub spoken_level: ProficiencyLevel,
    pub written_level: ProficiencyLevel,
    pub reading_level: ProficiencyLevel,
    pub listening_level: ProficiencyLevel,
    pub is_preferred: Option<bool>,
    pub show_on_profile: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateLanguageProficiencyRequest {
    pub spoken_level: Option<ProficiencyLevel>,
    pub written_level: Option<ProficiencyLevel>,
    pub reading_level: Option<ProficiencyLevel>,
    pub listening_level: Option<ProficiencyLevel>,
    pub is_preferred: Option<bool>,
    pub show_on_profile: Option<bool>,
}

// ============================================================================
// USER SETTINGS MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ThemeMode {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "system")]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "kebab-case")]
pub enum ColorScheme {
    #[serde(rename = "forest-green")]
    ForestGreen,
    #[serde(rename = "ocean-blue")]
    OceanBlue,
    #[serde(rename = "royal-purple")]
    RoyalPurple,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "kebab-case")]
pub enum TranslationProvider {
    #[serde(rename = "libre-translate")]
    LibreTranslate,
    #[serde(rename = "deepl")]
    DeepL,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "microsoft")]
    Microsoft,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserSettings {
    pub user_id: Uuid,
    pub theme_mode: ThemeMode,
    pub color_scheme: ColorScheme,
    pub reduced_motion: bool,
    pub wide_content_view: bool,
    pub compact_mode: bool,
    pub preferred_language: String,
    pub timezone: String,
    pub auto_translate: bool,
    pub translation_provider: TranslationProvider,
    pub contribute_translations: bool,
    pub fallback_to_english: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserSettingsRequest {
    pub theme_mode: Option<ThemeMode>,
    pub color_scheme: Option<ColorScheme>,
    pub reduced_motion: Option<bool>,
    pub wide_content_view: Option<bool>,
    pub compact_mode: Option<bool>,

    #[validate(length(min = 2, max = 10))]
    pub preferred_language: Option<String>,

    pub timezone: Option<String>,
    pub auto_translate: Option<bool>,
    pub translation_provider: Option<TranslationProvider>,
    pub contribute_translations: Option<bool>,
    pub fallback_to_english: Option<bool>,
}

// ============================================================================
// NOTIFICATION SETTINGS MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct NotificationSettings {
    pub user_id: Uuid,
    pub email_digest: bool,
    pub email_messages: bool,
    pub email_followers: bool,
    pub email_community: bool,
    pub email_updates: bool,
    pub inapp_messages: bool,
    pub inapp_followers: bool,
    pub inapp_community: bool,
    pub inapp_mentions: bool,
    pub inapp_likes: bool,
    pub push_enabled: bool,
    pub push_messages: bool,
    pub push_followers: bool,
    pub push_community: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateNotificationSettingsRequest {
    pub email_digest: Option<bool>,
    pub email_messages: Option<bool>,
    pub email_followers: Option<bool>,
    pub email_community: Option<bool>,
    pub email_updates: Option<bool>,
    pub inapp_messages: Option<bool>,
    pub inapp_followers: Option<bool>,
    pub inapp_community: Option<bool>,
    pub inapp_mentions: Option<bool>,
    pub inapp_likes: Option<bool>,
    pub push_enabled: Option<bool>,
    pub push_messages: Option<bool>,
    pub push_followers: Option<bool>,
    pub push_community: Option<bool>,
}

// ============================================================================
// COMBINED PROFILE RESPONSE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompleteProfile {
    pub user: User,
    pub profile: UserProfile,
    pub links: Vec<ProfileLink>,
    pub languages: Vec<LanguageProficiency>,
}

// ============================================================================
// USER CONNECTIONS MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ConnectionType {
    #[serde(rename = "follow")]
    Follow,
    #[serde(rename = "friend")]
    Friend,
    #[serde(rename = "block")]
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ConnectionStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserConnection {
    pub user_id: Uuid,
    pub target_user_id: Uuid,
    pub connection_type: ConnectionType,
    pub status: ConnectionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserConnectionWithProfile {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub connection_type: ConnectionType,
    pub status: ConnectionStatus,
    pub created_at: DateTime<Utc>,
}
