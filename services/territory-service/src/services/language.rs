use crate::models::language::{LanguageResponse, SearchLanguagesParams};
use shared_lib::{AppError, Database, Result};

pub struct LanguageService;

impl LanguageService {
    /// Search languages in the global registry
    pub async fn search_languages(
        db: &Database,
        params: SearchLanguagesParams,
    ) -> Result<Vec<LanguageResponse>> {
        let limit = params.limit.unwrap_or(50).min(500);
        let active_only = params.active_only.unwrap_or(true);

        let languages = if let Some(query) = params.q {
            // Search by name or code
            let search_pattern = format!("%{}%", query.to_lowercase());

            sqlx::query_as::<_, LanguageResponse>(
                r#"
                SELECT 
                    language_code,
                    language_name,
                    iso639_1,
                    iso639_2b,
                    iso639_2t,
                    language_scope,
                    language_type,
                    part_of_macro,
                    native_name,
                    script_code,
                    is_active
                FROM global.registry_languages
                WHERE ($1 = false OR is_active = true)
                  AND (
                    LOWER(language_name) LIKE $2
                    OR LOWER(language_code) LIKE $2
                    OR LOWER(iso639_1) LIKE $2
                    OR LOWER(native_name) LIKE $2
                  )
                ORDER BY 
                    CASE 
                        WHEN LOWER(language_code) = $3 THEN 0
                        WHEN LOWER(iso639_1) = $3 THEN 1
                        WHEN LOWER(language_name) = $3 THEN 2
                        WHEN LOWER(language_name) LIKE $2 THEN 3
                        ELSE 4
                    END,
                    language_name
                LIMIT $4
                "#,
            )
            .bind(!active_only)
            .bind(&search_pattern)
            .bind(query.to_lowercase())
            .bind(limit)
            .fetch_all(db.pool())
            .await?
        } else {
            // Return most common languages (those with iso639_1 codes)
            sqlx::query_as::<_, LanguageResponse>(
                r#"
                SELECT 
                    language_code,
                    language_name,
                    iso639_1,
                    iso639_2b,
                    iso639_2t,
                    language_scope,
                    language_type,
                    part_of_macro,
                    native_name,
                    script_code,
                    is_active
                FROM global.registry_languages
                WHERE ($1 = false OR is_active = true)
                  AND iso639_1 IS NOT NULL
                ORDER BY language_name
                LIMIT $2
                "#,
            )
            .bind(!active_only)
            .bind(limit)
            .fetch_all(db.pool())
            .await?
        };

        Ok(languages)
    }

    /// Get language details by code
    pub async fn get_language(db: &Database, code: &str) -> Result<LanguageResponse> {
        let language = sqlx::query_as::<_, LanguageResponse>(
            r#"
            SELECT 
                language_code,
                language_name,
                iso639_1,
                iso639_2b,
                iso639_2t,
                language_scope,
                language_type,
                part_of_macro,
                native_name,
                script_code,
                is_active
            FROM global.registry_languages
            WHERE language_code = $1
               OR iso639_1 = $1
               OR iso639_2b = $1
               OR iso639_2t = $1
            "#,
        )
        .bind(code)
        .fetch_optional(db.pool())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Language '{}' not found", code)))?;

        Ok(language)
    }
}
