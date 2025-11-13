use shared_lib::{AppError, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateProfileLinkRequest, ProfileLink, UpdateProfileLinkRequest};

/// Profile link service - business logic for managing profile links
pub struct ProfileLinkService;

impl ProfileLinkService {
    /// Get all profile links for a user
    pub async fn get_user_links(
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<Vec<ProfileLink>> {
        let query = format!(
            "SELECT * FROM territory_{}.users_profile_links 
             WHERE user_id = $1 
             ORDER BY display_order ASC, created_at ASC",
            territory
        );

        sqlx::query_as::<_, ProfileLink>(&query)
            .bind(user_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Get a specific profile link by ID
    pub async fn get_link(
        link_id: Uuid,
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<ProfileLink> {
        let query = format!(
            "SELECT * FROM territory_{}.users_profile_links 
             WHERE id = $1 AND user_id = $2",
            territory
        );

        sqlx::query_as::<_, ProfileLink>(&query)
            .bind(link_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e))?
            .ok_or_else(|| AppError::NotFound("Profile link not found".into()))
    }

    /// Create a new profile link
    pub async fn create_link(
        user_id: Uuid,
        territory: &str,
        req: CreateProfileLinkRequest,
        pool: &PgPool,
    ) -> Result<ProfileLink> {
        // Check if user already has 10 links (max limit)
        let count_query = format!(
            "SELECT COUNT(*) as count FROM territory_{}.users_profile_links WHERE user_id = $1",
            territory
        );

        let count: (i64,) = sqlx::query_as(&count_query)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        if count.0 >= 10 {
            return Err(AppError::Validation(
                "Maximum 10 profile links allowed".into(),
            ));
        }

        // Determine display_order (default to last position)
        let display_order = match req.display_order {
            Some(order) => order,
            None => {
                let max_query = format!(
                    "SELECT COALESCE(MAX(display_order), -1) as max_order 
                     FROM territory_{}.users_profile_links 
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
            "INSERT INTO territory_{}.users_profile_links 
             (user_id, label, url, icon, display_order, is_visible) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING *",
            territory
        );

        sqlx::query_as::<_, ProfileLink>(&query)
            .bind(user_id)
            .bind(req.label)
            .bind(req.url)
            .bind(req.icon)
            .bind(display_order)
            .bind(req.is_visible.unwrap_or(true))
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Update a profile link
    pub async fn update_link(
        link_id: Uuid,
        user_id: Uuid,
        territory: &str,
        req: UpdateProfileLinkRequest,
        pool: &PgPool,
    ) -> Result<ProfileLink> {
        // Verify ownership
        let _ = Self::get_link(link_id, user_id, territory, pool).await?;

        let query = format!(
            "UPDATE territory_{}.users_profile_links 
             SET label = COALESCE($3, label),
                 url = COALESCE($4, url),
                 icon = COALESCE($5, icon),
                 display_order = COALESCE($6, display_order),
                 is_visible = COALESCE($7, is_visible),
                 updated_at = NOW()
             WHERE id = $1 AND user_id = $2
             RETURNING *",
            territory
        );

        sqlx::query_as::<_, ProfileLink>(&query)
            .bind(link_id)
            .bind(user_id)
            .bind(req.label)
            .bind(req.url)
            .bind(req.icon)
            .bind(req.display_order)
            .bind(req.is_visible)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Delete a profile link
    pub async fn delete_link(
        link_id: Uuid,
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<()> {
        // Verify ownership
        let _ = Self::get_link(link_id, user_id, territory, pool).await?;

        let query = format!(
            "DELETE FROM territory_{}.users_profile_links 
             WHERE id = $1 AND user_id = $2",
            territory
        );

        sqlx::query(&query)
            .bind(link_id)
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(())
    }
}
