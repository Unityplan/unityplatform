use shared_lib::{AppError, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    CreateLanguageProficiencyRequest, LanguageProficiency, UpdateLanguageProficiencyRequest,
};

/// Language proficiency service - business logic for managing language skills
pub struct LanguageProficiencyService;

impl LanguageProficiencyService {
    /// Get all language proficiencies for a user
    pub async fn get_user_languages(
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<Vec<LanguageProficiency>> {
        let query = format!(
            "SELECT * FROM territory_{}.users_language_proficiency 
             WHERE user_id = $1 
             ORDER BY display_order ASC, created_at ASC",
            territory
        );

        sqlx::query_as::<_, LanguageProficiency>(&query)
            .bind(user_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Get a specific language proficiency by ID
    pub async fn get_language(
        lang_id: Uuid,
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<LanguageProficiency> {
        let query = format!(
            "SELECT * FROM territory_{}.users_language_proficiency 
             WHERE id = $1 AND user_id = $2",
            territory
        );

        sqlx::query_as::<_, LanguageProficiency>(&query)
            .bind(lang_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e))?
            .ok_or_else(|| AppError::NotFound("Language proficiency not found".into()))
    }

    /// Create a new language proficiency
    pub async fn create_language(
        user_id: Uuid,
        territory: &str,
        req: CreateLanguageProficiencyRequest,
        pool: &PgPool,
    ) -> Result<LanguageProficiency> {
        // Check if user already has this language
        let exists_query = format!(
            "SELECT COUNT(*) as count FROM territory_{}.users_language_proficiency 
             WHERE user_id = $1 AND language_code = $2",
            territory
        );

        let count: (i64,) = sqlx::query_as(&exists_query)
            .bind(user_id)
            .bind(&req.language_code)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        if count.0 > 0 {
            return Err(AppError::Validation(
                "Language already added to profile".into(),
            ));
        }

        // Determine display_order (default to last position)
        let display_order = match req.display_order {
            Some(order) => order,
            None => {
                let max_query = format!(
                    "SELECT COALESCE(MAX(display_order), -1) as max_order 
                     FROM territory_{}.users_language_proficiency 
                     WHERE user_id = $1",
                    territory
                );
                let max: (i32,) = sqlx::query_as(&max_query)
                    .bind(user_id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| AppError::Database(e))?;
                max.0 + 1
            }
        };

        let query = format!(
            "INSERT INTO territory_{}.users_language_proficiency 
             (user_id, language_code, language_name, spoken_level, written_level, 
              reading_level, listening_level, display_order, is_preferred, show_on_profile) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
             RETURNING *",
            territory
        );

        sqlx::query_as::<_, LanguageProficiency>(&query)
            .bind(user_id)
            .bind(req.language_code)
            .bind(req.language_name)
            .bind(req.spoken_level)
            .bind(req.written_level)
            .bind(req.reading_level)
            .bind(req.listening_level)
            .bind(display_order)
            .bind(req.is_preferred.unwrap_or(false))
            .bind(req.show_on_profile.unwrap_or(true))
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Update a language proficiency
    pub async fn update_language(
        lang_id: Uuid,
        user_id: Uuid,
        territory: &str,
        req: UpdateLanguageProficiencyRequest,
        pool: &PgPool,
    ) -> Result<LanguageProficiency> {
        // Verify ownership
        let _ = Self::get_language(lang_id, user_id, territory, pool).await?;

        let query = format!(
            "UPDATE territory_{}.users_language_proficiency 
             SET language_name = COALESCE($3, language_name),
                 spoken_level = COALESCE($4, spoken_level),
                 written_level = COALESCE($5, written_level),
                 reading_level = COALESCE($6, reading_level),
                 listening_level = COALESCE($7, listening_level),
                 display_order = COALESCE($8, display_order),
                 is_preferred = COALESCE($9, is_preferred),
                 show_on_profile = COALESCE($10, show_on_profile),
                 updated_at = NOW()
             WHERE id = $1 AND user_id = $2
             RETURNING *",
            territory
        );

        sqlx::query_as::<_, LanguageProficiency>(&query)
            .bind(lang_id)
            .bind(user_id)
            .bind(req.language_name)
            .bind(req.spoken_level)
            .bind(req.written_level)
            .bind(req.reading_level)
            .bind(req.listening_level)
            .bind(req.display_order)
            .bind(req.is_preferred)
            .bind(req.show_on_profile)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Delete a language proficiency
    pub async fn delete_language(
        lang_id: Uuid,
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<()> {
        // Verify ownership
        let _ = Self::get_language(lang_id, user_id, territory, pool).await?;

        let query = format!(
            "DELETE FROM territory_{}.users_language_proficiency 
             WHERE id = $1 AND user_id = $2",
            territory
        );

        sqlx::query(&query)
            .bind(lang_id)
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(())
    }
}
