use chrono::{Duration, Utc};
use shared_lib::{AppError, Result};
use sqlx::PgPool;
use uuid::Uuid;
use tracing::{info, error};

use crate::models::UserCleanupStats;

/// User cleanup service - Handles hard deletion of soft-deleted users
#[derive(Clone)]
pub struct CleanupService {
    pool: PgPool,
}

impl CleanupService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Clean up soft-deleted users older than 30 days
    /// 
    /// This implements GDPR-compliant data deletion:
    /// 1. Users request deletion (soft delete in auth-service)
    /// 2. Data marked deleted, user cannot login
    /// 3. After 30 days, this job permanently removes all user data
    /// 
    /// # Arguments
    /// * `dry_run` - If true, only report what would be deleted without actually deleting
    /// 
    /// # Returns
    /// * `UserCleanupStats` - Statistics about the cleanup operation
    pub async fn cleanup_deleted_users(&self, dry_run: bool) -> Result<UserCleanupStats> {
        let cutoff = Utc::now() - Duration::days(30);
        
        info!(
            dry_run = dry_run,
            cutoff = %cutoff,
            "Starting deleted user cleanup job"
        );

        // Find soft-deleted users older than 30 days from global registry
        let deleted_users = sqlx::query_as::<_, (Uuid, String, chrono::DateTime<Utc>)>(
            "SELECT user_id, territory_code, deleted_at 
             FROM global.registry_username 
             WHERE deleted_at IS NOT NULL 
             AND deleted_at < $1
             ORDER BY deleted_at ASC"
        )
        .bind(cutoff)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        let eligible_count = deleted_users.len() as u64;
        let mut territories_processed: Vec<String> = Vec::new();
        let mut deleted_count = 0u64;

        info!(
            eligible_users = eligible_count,
            "Found users eligible for hard deletion"
        );

        if dry_run {
            info!("DRY RUN MODE - No actual deletions will occur");
            for (user_id, territory, deleted_at) in &deleted_users {
                info!(
                    user_id = %user_id,
                    territory = %territory,
                    deleted_at = %deleted_at,
                    "Would delete user"
                );
                if !territories_processed.contains(territory) {
                    territories_processed.push(territory.clone());
                }
            }
            
            return Ok(UserCleanupStats {
                soft_deleted_users: eligible_count,
                eligible_for_deletion: eligible_count,
                deleted: 0,
                territories_processed,
            });
        }

        // Actually delete users
        for (user_id, territory, deleted_at) in deleted_users {
            match self.cleanup_user_data(user_id, &territory).await {
                Ok(_) => {
                    info!(
                        user_id = %user_id,
                        territory = %territory,
                        deleted_at = %deleted_at,
                        "Successfully hard-deleted user"
                    );
                    deleted_count += 1;
                    if !territories_processed.contains(&territory) {
                        territories_processed.push(territory.clone());
                    }
                }
                Err(e) => {
                    error!(
                        user_id = %user_id,
                        territory = %territory,
                        error = %e,
                        "Failed to delete user data"
                    );
                    // Continue with other users even if one fails
                }
            }
        }

        info!(
            deleted = deleted_count,
            eligible = eligible_count,
            territories = ?territories_processed,
            "Cleanup job completed"
        );

        Ok(UserCleanupStats {
            soft_deleted_users: eligible_count,
            eligible_for_deletion: eligible_count,
            deleted: deleted_count,
            territories_processed,
        })
    }

    /// Delete all data for a specific user across all services
    /// 
    /// This respects service boundaries by deleting from each service's tables
    /// in the correct order (children first, then parent data)
    async fn cleanup_user_data(&self, user_id: Uuid, territory: &str) -> Result<()> {
        let schema = format!("territory_{}", territory);
        
        // Use a transaction to ensure all-or-nothing deletion
        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

        // Delete in reverse dependency order (children first)
        
        // 1. User service data
        Self::delete_from_table(&mut tx, &schema, "user_users_profile_links", user_id).await?;
        Self::delete_from_table(&mut tx, &schema, "user_users_profile_language_proficiency", user_id).await?;
        Self::delete_from_table(&mut tx, &schema, "user_users_connections", user_id).await?;
        // Also delete where user is target
        sqlx::query(&format!("DELETE FROM {}.user_users_connections WHERE target_user_id = $1", schema))
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;
        Self::delete_from_table(&mut tx, &schema, "user_users_data_exports", user_id).await?;
        Self::delete_from_table(&mut tx, &schema, "user_users_account_deletion_requests", user_id).await?;
        Self::delete_from_table(&mut tx, &schema, "user_users_settings", user_id).await?;
        Self::delete_from_table(&mut tx, &schema, "user_users_profiles", user_id).await?;

        // 2. Badge service data
        Self::delete_from_table(&mut tx, &schema, "badge_users_progress", user_id).await?;
        Self::delete_from_table(&mut tx, &schema, "badge_users_badges", user_id).await?;

        // 3. Community service data  
        Self::delete_from_table(&mut tx, &schema, "community_communities_managers", user_id).await?;
        Self::delete_from_table(&mut tx, &schema, "community_communities_members", user_id).await?;
        // Update communities where user was creator (set to NULL, already allowed by schema)
        sqlx::query(&format!("UPDATE {}.community_communities SET created_by = NULL WHERE created_by = $1", schema))
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        // 4. Invitation service data (if user created invitations)
        sqlx::query(&format!("UPDATE {}.invitation_invitations_tokens SET created_by = NULL WHERE created_by = $1", schema))
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        // 5. Auth service data (refresh tokens, then core user)
        Self::delete_from_table(&mut tx, &schema, "auth_users_refresh_tokens", user_id).await?;
        // auth_users_core uses 'id' not 'user_id'
        sqlx::query(&format!("DELETE FROM {}.auth_users_core WHERE id = $1", schema))
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        // 6. Finally, delete from global registry
        sqlx::query("DELETE FROM global.registry_username WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        sqlx::query("DELETE FROM global.registry_email WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        // Commit transaction
        tx.commit().await.map_err(AppError::Database)?;

        Ok(())
    }

    /// Helper to delete from a table by user_id
    async fn delete_from_table(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        schema: &str,
        table: &str,
        user_id: Uuid,
    ) -> Result<()> {
        let query = format!("DELETE FROM {}.{} WHERE user_id = $1", schema, table);
        sqlx::query(&query)
            .bind(user_id)
            .execute(&mut **tx)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
}
