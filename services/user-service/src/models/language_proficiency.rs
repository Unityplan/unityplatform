use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Proficiency level enum
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProficiencyLevel {
    Native,
    Fluent,
    Advanced,
    Intermediate,
    Basic,
    Learning,
}

impl ProficiencyLevel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Native => "native",
            Self::Fluent => "fluent",
            Self::Advanced => "advanced",
            Self::Intermediate => "intermediate",
            Self::Basic => "basic",
            Self::Learning => "learning",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "native" => Some(Self::Native),
            "fluent" => Some(Self::Fluent),
            "advanced" => Some(Self::Advanced),
            "intermediate" => Some(Self::Intermediate),
            "basic" => Some(Self::Basic),
            "learning" => Some(Self::Learning),
            _ => None,
        }
    }
}

/// Language proficiency data structure
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LanguageProficiency {
    pub id: Uuid,
    pub user_id: Uuid,
    pub language_code: String,
    pub language_name: String,
    pub spoken_level: String,
    pub written_level: String,
    pub reading_level: String,
    pub listening_level: String,
    pub display_order: i32,
    pub is_preferred: bool,
    pub show_on_profile: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Language proficiency response (API representation)
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LanguageProficiencyResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,

    #[schema(example = "en")]
    pub language_code: String,

    #[schema(example = "English")]
    pub language_name: String,

    #[schema(example = "fluent")]
    pub spoken_level: String,

    #[schema(example = "native")]
    pub written_level: String,

    #[schema(example = "native")]
    pub reading_level: String,

    #[schema(example = "fluent")]
    pub listening_level: String,

    #[schema(example = 0)]
    pub display_order: i32,

    #[schema(example = true)]
    pub is_preferred: bool,

    #[schema(example = true)]
    pub show_on_profile: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<LanguageProficiency> for LanguageProficiencyResponse {
    fn from(lang: LanguageProficiency) -> Self {
        Self {
            id: lang.id,
            language_code: lang.language_code,
            language_name: lang.language_name,
            spoken_level: lang.spoken_level,
            written_level: lang.written_level,
            reading_level: lang.reading_level,
            listening_level: lang.listening_level,
            display_order: lang.display_order,
            is_preferred: lang.is_preferred,
            show_on_profile: lang.show_on_profile,
            created_at: lang.created_at,
            updated_at: lang.updated_at,
        }
    }
}

/// Create language proficiency request
#[derive(Debug, Deserialize, validator::Validate, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateLanguageProficiencyRequest {
    #[validate(length(min = 2, max = 10))]
    #[schema(example = "en")]
    pub language_code: String,

    #[validate(length(min = 1, max = 100))]
    #[schema(example = "English")]
    pub language_name: String,

    #[validate(custom(function = "validate_proficiency_level"))]
    #[schema(example = "fluent")]
    pub spoken_level: String,

    #[validate(custom(function = "validate_proficiency_level"))]
    #[schema(example = "native")]
    pub written_level: String,

    #[validate(custom(function = "validate_proficiency_level"))]
    #[schema(example = "native")]
    pub reading_level: String,

    #[validate(custom(function = "validate_proficiency_level"))]
    #[schema(example = "fluent")]
    pub listening_level: String,

    #[schema(example = 0)]
    pub display_order: Option<i32>,

    #[schema(example = true)]
    pub is_preferred: Option<bool>,

    #[schema(example = true)]
    pub show_on_profile: Option<bool>,
}

/// Update language proficiency request
#[derive(Debug, Deserialize, validator::Validate, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLanguageProficiencyRequest {
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "English (US)")]
    pub language_name: Option<String>,

    #[validate(custom(function = "validate_proficiency_level_opt"))]
    #[schema(example = "native")]
    pub spoken_level: Option<String>,

    #[validate(custom(function = "validate_proficiency_level_opt"))]
    #[schema(example = "fluent")]
    pub written_level: Option<String>,

    #[validate(custom(function = "validate_proficiency_level_opt"))]
    #[schema(example = "native")]
    pub reading_level: Option<String>,

    #[validate(custom(function = "validate_proficiency_level_opt"))]
    #[schema(example = "fluent")]
    pub listening_level: Option<String>,

    #[schema(example = 1)]
    pub display_order: Option<i32>,

    #[schema(example = false)]
    pub is_preferred: Option<bool>,

    #[schema(example = true)]
    pub show_on_profile: Option<bool>,
}

/// Validate proficiency level
fn validate_proficiency_level(level: &str) -> Result<(), validator::ValidationError> {
    let valid_levels = [
        "native",
        "fluent",
        "advanced",
        "intermediate",
        "basic",
        "learning",
    ];
    if valid_levels.contains(&level) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_proficiency_level"))
    }
}

/// Validate optional proficiency level
fn validate_proficiency_level_opt(level: &&String) -> Result<(), validator::ValidationError> {
    validate_proficiency_level(level.as_str())
}
