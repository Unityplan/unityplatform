use crate::models::{
    connection::{ConnectionResponse, UserBlock, UserConnection},
    privacy::PrivacySettings,
    profile::{FullUserProfile, PublicUserProfile, UpdateProfileRequest, UserProfile},
};
use sqlx::PgPool;
use uuid::Uuid;

/// User service for managing user profiles, connections, and blocks
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    /// Create a new user service instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ==================== Profile Operations ====================

    /// Get full profile for a user (includes privacy settings)
    pub async fn get_profile(&self, user_id: Uuid) -> Result<Option<FullUserProfile>, sqlx::Error> {
        let profile = sqlx::query_as::<_, UserProfile>(
            r#"
            SELECT 
                user_id,
                about,
                interests,
                skills,
                languages,
                location,
                website_url,
                github_url,
                linkedin_url,
                twitter_handle,
                theme,
                metadata,
                profile_visibility,
                show_email,
                show_real_name,
                allow_messages_from,
                created_at,
                updated_at
            FROM territory.user_profiles
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(p) = profile {
            // Get user basic info from territory.users
            let user_info = sqlx::query_as::<
                _,
                (
                    String,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                ),
            >(
                r#"
                SELECT username, email, full_name, display_name, avatar_url, bio
                FROM territory.users
                WHERE id = $1
                "#,
            )
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(user) = user_info {
                Ok(Some(FullUserProfile {
                    user_id: p.user_id,
                    username: user.0,
                    email: user.1,
                    full_name: user.2,
                    display_name: user.3,
                    avatar_url: user.4,
                    bio: user.5,
                    about: p.about,
                    interests: p.interests,
                    skills: p.skills,
                    languages: p.languages,
                    location: p.location,
                    website_url: p.website_url,
                    github_url: p.github_url,
                    linkedin_url: p.linkedin_url,
                    twitter_handle: p.twitter_handle,
                    theme: p.theme,
                    metadata: p.metadata,
                    privacy: PrivacySettings {
                        profile_visibility: p
                            .profile_visibility
                            .unwrap_or_else(|| "public".to_string()),
                        show_email: p.show_email.unwrap_or(false),
                        show_real_name: p.show_real_name.unwrap_or(true),
                        allow_messages_from: p
                            .allow_messages_from
                            .unwrap_or_else(|| "everyone".to_string()),
                    },
                    created_at: p.created_at,
                    updated_at: p.updated_at,
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Get public profile for viewing by other users
    pub async fn get_public_profile(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<PublicUserProfile>, sqlx::Error> {
        // Get full profile
        let full_profile = self.get_profile(user_id).await?;

        if let Some(profile) = full_profile {
            // Check if viewer is connected
            let is_connected = if let Some(vid) = viewer_id {
                self.is_connected(vid, user_id).await?
            } else {
                false
            };

            // Apply privacy rules
            let visibility = &profile.privacy.profile_visibility;
            let show_profile = match visibility.as_str() {
                "public" => true,
                "connections" => is_connected,
                "private" => viewer_id == Some(user_id), // Only owner can see private
                _ => false,
            };

            if !show_profile {
                return Ok(None);
            }

            // Build public profile with privacy-filtered data
            Ok(Some(PublicUserProfile {
                user_id: profile.user_id,
                username: profile.username,
                display_name: profile.display_name,
                avatar_url: profile.avatar_url,
                bio: profile.bio,
                about: profile.about,
                interests: profile.interests,
                skills: profile.skills,
                languages: profile.languages,
                location: profile.location,
                website_url: profile.website_url,
                github_url: profile.github_url,
                linkedin_url: profile.linkedin_url,
                twitter_handle: profile.twitter_handle,
                full_name: if profile.privacy.show_real_name {
                    profile.full_name
                } else {
                    None
                },
                email: if profile.privacy.show_email {
                    profile.email
                } else {
                    None
                },
            }))
        } else {
            Ok(None)
        }
    }

    /// Update user profile
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        request: UpdateProfileRequest,
    ) -> Result<UserProfile, sqlx::Error> {
        // Upsert profile
        let profile = sqlx::query_as::<_, UserProfile>(
            r#"
            INSERT INTO territory.user_profiles (
                user_id, about, interests, skills, languages, location,
                website_url, github_url, linkedin_url, twitter_handle,
                theme, metadata, profile_visibility, show_email,
                show_real_name, allow_messages_from
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (user_id) DO UPDATE SET
                about = COALESCE($2, territory.user_profiles.about),
                interests = COALESCE($3, territory.user_profiles.interests),
                skills = COALESCE($4, territory.user_profiles.skills),
                languages = COALESCE($5, territory.user_profiles.languages),
                location = COALESCE($6, territory.user_profiles.location),
                website_url = COALESCE($7, territory.user_profiles.website_url),
                github_url = COALESCE($8, territory.user_profiles.github_url),
                linkedin_url = COALESCE($9, territory.user_profiles.linkedin_url),
                twitter_handle = COALESCE($10, territory.user_profiles.twitter_handle),
                theme = COALESCE($11, territory.user_profiles.theme),
                metadata = COALESCE($12, territory.user_profiles.metadata),
                profile_visibility = COALESCE($13, territory.user_profiles.profile_visibility),
                show_email = COALESCE($14, territory.user_profiles.show_email),
                show_real_name = COALESCE($15, territory.user_profiles.show_real_name),
                allow_messages_from = COALESCE($16, territory.user_profiles.allow_messages_from),
                updated_at = NOW()
            RETURNING 
                user_id, about, interests, skills, languages, location,
                website_url, github_url, linkedin_url, twitter_handle,
                theme, metadata, profile_visibility, show_email,
                show_real_name, allow_messages_from, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(request.about)
        .bind(request.interests.as_deref())
        .bind(request.skills.as_deref())
        .bind(request.languages.as_deref())
        .bind(request.location)
        .bind(request.website_url)
        .bind(request.github_url)
        .bind(request.linkedin_url)
        .bind(request.twitter_handle)
        .bind(request.theme)
        .bind(request.metadata)
        .bind(request.profile_visibility)
        .bind(request.show_email)
        .bind(request.show_real_name)
        .bind(request.allow_messages_from)
        .fetch_one(&self.pool)
        .await?;

        Ok(profile)
    }

    // ==================== Connection Operations ====================

    /// Check if user A is connected to (following) user B
    pub async fn is_connected(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM territory.user_connections
                WHERE follower_id = $1 AND following_id = $2
            )
            "#,
        )
        .bind(follower_id)
        .bind(following_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    /// Follow a user
    pub async fn follow_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<UserConnection, sqlx::Error> {
        // Check if either user has blocked the other
        let block_exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM territory.user_blocks
                WHERE (blocker_id = $1 AND blocked_id = $2)
                   OR (blocker_id = $2 AND blocked_id = $1)
            )
            "#,
        )
        .bind(follower_id)
        .bind(following_id)
        .fetch_one(&self.pool)
        .await?;

        if block_exists {
            return Err(sqlx::Error::RowNotFound);
        }

        // First try to insert
        let result = sqlx::query_as::<_, UserConnection>(
            r#"
            INSERT INTO territory.user_connections (follower_id, following_id)
            VALUES ($1, $2)
            ON CONFLICT (follower_id, following_id) DO NOTHING
            RETURNING follower_id, following_id, created_at
            "#,
        )
        .bind(follower_id)
        .bind(following_id)
        .fetch_optional(&self.pool)
        .await?;

        // If insert was skipped (conflict), fetch existing connection
        match result {
            Some(conn) => Ok(conn),
            None => {
                sqlx::query_as::<_, UserConnection>(
                    r#"
                    SELECT follower_id, following_id, created_at
                    FROM territory.user_connections
                    WHERE follower_id = $1 AND following_id = $2
                    "#,
                )
                .bind(follower_id)
                .bind(following_id)
                .fetch_one(&self.pool)
                .await
            }
        }
    }

    /// Unfollow a user
    pub async fn unfollow_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM territory.user_connections
            WHERE follower_id = $1 AND following_id = $2
            "#,
        )
        .bind(follower_id)
        .bind(following_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get followers of a user
    pub async fn get_followers(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ConnectionResponse>, sqlx::Error> {
        let followers = sqlx::query_as::<_, ConnectionResponse>(
            r#"
            SELECT 
                uc.follower_id as user_id,
                u.username,
                u.display_name,
                u.avatar_url,
                uc.created_at
            FROM territory.user_connections uc
            JOIN territory.users u ON u.id = uc.follower_id
            WHERE uc.following_id = $1
            ORDER BY uc.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(followers)
    }

    /// Get users that a user is following
    pub async fn get_following(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ConnectionResponse>, sqlx::Error> {
        let following = sqlx::query_as::<_, ConnectionResponse>(
            r#"
            SELECT 
                uc.following_id as user_id,
                u.username,
                u.display_name,
                u.avatar_url,
                uc.created_at
            FROM territory.user_connections uc
            JOIN territory.users u ON u.id = uc.following_id
            WHERE uc.follower_id = $1
            ORDER BY uc.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(following)
    }

    // ==================== Block Operations ====================

    /// Block a user
    pub async fn block_user(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid,
        reason: Option<String>,
    ) -> Result<UserBlock, sqlx::Error> {
        // First, remove any existing connection
        let _ = self.unfollow_user(blocker_id, blocked_id).await;
        let _ = self.unfollow_user(blocked_id, blocker_id).await;

        // Then create block
        let block = sqlx::query_as::<_, UserBlock>(
            r#"
            INSERT INTO territory.user_blocks (blocker_id, blocked_id, reason)
            VALUES ($1, $2, $3)
            ON CONFLICT (blocker_id, blocked_id) DO UPDATE SET
                reason = COALESCE($3, territory.user_blocks.reason),
                created_at = NOW()
            RETURNING blocker_id, blocked_id, reason, created_at
            "#,
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await?;

        Ok(block)
    }

    /// Unblock a user
    pub async fn unblock_user(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM territory.user_blocks
            WHERE blocker_id = $1 AND blocked_id = $2
            "#,
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Check if user A has blocked user B
    pub async fn is_blocked(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_as::<_, (bool,)>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM territory.user_blocks
                WHERE blocker_id = $1 AND blocked_id = $2
            )
            "#,
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    /// Get list of users blocked by a user
    pub async fn get_blocked_users(&self, user_id: Uuid) -> Result<Vec<UserBlock>, sqlx::Error> {
        let blocks = sqlx::query_as::<_, UserBlock>(
            r#"
            SELECT blocker_id, blocked_id, reason, created_at
            FROM territory.user_blocks
            WHERE blocker_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(blocks)
    }
}
