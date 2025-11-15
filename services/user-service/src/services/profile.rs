use shared_lib::{AppError, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Profile, ProfileResponse};

/// Profile service - business logic for profile management
pub struct ProfileService;

impl ProfileService {
    /// Get user profile by ID with username
    pub async fn get_profile(
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<ProfileResponse> {
        // First check if user exists
        let user_query = format!(
            "SELECT id, username FROM territory_{}.auth_users_core WHERE id = $1",
            territory
        );

        let user_row: Option<(Uuid, String)> = sqlx::query_as(&user_query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        let (user_id, username) =
            user_row.ok_or_else(|| AppError::NotFound("User not found".into()))?;

        // Get or create profile
        let profile = Self::get_or_create_profile(user_id, territory, pool).await?;

        Ok(ProfileResponse {
            id: user_id,
            username,
            display_name: profile.display_name,
            avatar_url: profile.avatar_url,
            bio: profile.bio,
            about: profile.about,
            location: profile.location,
            website: profile.website,
            interests: profile.interests,
            skills: profile.skills,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        })
    }

    /// Create empty profile for new user
    pub async fn create_profile(user_id: Uuid, territory: &str, pool: &PgPool) -> Result<Profile> {
        let query = format!(
            "INSERT INTO territory_{}.user_users_profiles (user_id) 
             VALUES ($1) 
             ON CONFLICT (user_id) DO NOTHING
             RETURNING *",
            territory
        );

        sqlx::query_as::<_, Profile>(&query)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))
    }

    /// Update user profile (creates if doesn't exist)
    pub async fn update_profile(
        user_id: Uuid,
        territory: &str,
        display_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        about: Option<String>,
        location: Option<String>,
        website: Option<String>,
        interests: Option<Vec<String>>,
        skills: Option<Vec<String>>,
        pool: &PgPool,
    ) -> Result<ProfileResponse> {
        // First, ensure profile exists
        let _ = Self::get_or_create_profile(user_id, territory, pool).await?;

        // Build dynamic UPDATE query based on provided fields
        let query = format!(
            "UPDATE territory_{}.user_users_profiles 
             SET display_name = COALESCE($2, display_name),
                 avatar_url = COALESCE($3, avatar_url),
                 bio = COALESCE($4, bio),
                 about = COALESCE($5, about),
                 location = COALESCE($6, location),
                 website = COALESCE($7, website),
                 interests = COALESCE($8, interests),
                 skills = COALESCE($9, skills),
                 updated_at = NOW()
             WHERE user_id = $1
             RETURNING *",
            territory
        );

        let _profile: Profile = sqlx::query_as(&query)
            .bind(user_id)
            .bind(display_name)
            .bind(avatar_url)
            .bind(bio)
            .bind(about)
            .bind(location)
            .bind(website)
            .bind(interests)
            .bind(skills)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        // Get username to return complete ProfileResponse
        Self::get_profile(user_id, territory, pool).await
    }

    /// Get profile or create if doesn't exist
    async fn get_or_create_profile(
        user_id: Uuid,
        territory: &str,
        pool: &PgPool,
    ) -> Result<Profile> {
        let query = format!(
            "SELECT * FROM territory_{}.user_users_profiles WHERE user_id = $1",
            territory
        );

        let profile = sqlx::query_as::<_, Profile>(&query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        match profile {
            Some(p) => Ok(p),
            None => Self::create_profile(user_id, territory, pool).await,
        }
    }
}
