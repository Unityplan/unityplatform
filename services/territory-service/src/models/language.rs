use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LanguageResponse {
    #[schema(example = "eng")]
    pub language_code: String,

    #[schema(example = "English")]
    pub language_name: String,

    #[schema(example = "en")]
    pub iso639_1: Option<String>,

    #[schema(example = "eng")]
    pub iso639_2b: Option<String>,

    #[schema(example = "eng")]
    pub iso639_2t: Option<String>,

    #[schema(example = "I")]
    pub language_scope: String,

    #[schema(example = "L")]
    pub language_type: String,

    #[schema(example = "nor")]
    pub part_of_macro: Option<String>,

    #[schema(example = "English")]
    pub native_name: Option<String>,

    #[schema(example = "Latn")]
    pub script_code: Option<String>,

    #[schema(example = true)]
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchLanguagesParams {
    #[schema(example = "eng")]
    pub q: Option<String>,

    #[schema(example = 50)]
    pub limit: Option<i64>,

    #[schema(example = true)]
    pub active_only: Option<bool>,
}

impl Default for SearchLanguagesParams {
    fn default() -> Self {
        Self {
            q: None,
            limit: Some(50),
            active_only: Some(true),
        }
    }
}
