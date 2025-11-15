use shared_lib::{AppError, Database, Result};
use sqlx::Row;
use uuid::Uuid;

use crate::models::{
    ConnectionResponse, ConnectionStatus, ConnectionType, ConnectionsListResponse,
    UserSearchResponse, UserSearchResult,
};

pub struct ConnectionService;

impl ConnectionService {
    /// Follow a user
    pub async fn follow_user(
        db: &Database,
        territory: &str,
        user_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<()> {
        // Prevent self-follow (enforced by CHECK constraint but check early)
        if user_id == target_user_id {
            return Err(AppError::Validation("Cannot follow yourself".to_string()));
        }

        // Check if target user exists
        let target_exists = sqlx::query_scalar::<_, bool>(&format!(
            "SELECT EXISTS(SELECT 1 FROM territory_{}.auth_users_core WHERE id = $1)",
            territory
        ))
        .bind(target_user_id)
        .fetch_one(db.pool())
        .await?;

        if !target_exists {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        // Check if already blocked (can't follow if blocked)
        let is_blocked = sqlx::query_scalar::<_, bool>(&format!(
            "SELECT EXISTS(
                SELECT 1 FROM territory_{}.user_users_connections
                WHERE user_id = $1 AND target_user_id = $2 AND connection_type = 'block'
            )",
            territory
        ))
        .bind(user_id)
        .bind(target_user_id)
        .fetch_one(db.pool())
        .await?;

        if is_blocked {
            return Err(AppError::Validation(
                "Cannot follow a blocked user. Unblock first.".to_string(),
            ));
        }

        // Insert or update follow connection
        sqlx::query(&format!(
            "INSERT INTO territory_{}.user_users_connections 
                (user_id, target_user_id, connection_type, status)
             VALUES ($1, $2, 'follow', 'active')
             ON CONFLICT (user_id, target_user_id, connection_type)
             DO UPDATE SET status = 'active', updated_at = now()",
            territory
        ))
        .bind(user_id)
        .bind(target_user_id)
        .execute(db.pool())
        .await?;

        Ok(())
    }

    /// Unfollow a user
    pub async fn unfollow_user(
        db: &Database,
        territory: &str,
        user_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<()> {
        let result = sqlx::query(&format!(
            "DELETE FROM territory_{}.user_users_connections
             WHERE user_id = $1 AND target_user_id = $2 AND connection_type = 'follow'",
            territory
        ))
        .bind(user_id)
        .bind(target_user_id)
        .execute(db.pool())
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Follow connection not found".to_string(),
            ));
        }

        Ok(())
    }

    /// Block a user
    pub async fn block_user(
        db: &Database,
        territory: &str,
        user_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<()> {
        // Prevent self-block (enforced by CHECK constraint but check early)
        if user_id == target_user_id {
            return Err(AppError::Validation("Cannot block yourself".to_string()));
        }

        // Check if target user exists
        let target_exists = sqlx::query_scalar::<_, bool>(&format!(
            "SELECT EXISTS(SELECT 1 FROM territory_{}.auth_users_core WHERE id = $1)",
            territory
        ))
        .bind(target_user_id)
        .fetch_one(db.pool())
        .await?;

        if !target_exists {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        // Start transaction to handle follow removal and block insertion
        let mut tx = db.pool().begin().await?;

        // Remove any mutual follow relationships
        sqlx::query(&format!(
            "DELETE FROM territory_{}.user_users_connections
             WHERE ((user_id = $1 AND target_user_id = $2) OR (user_id = $2 AND target_user_id = $1))
               AND connection_type = 'follow'",
            territory
        ))
        .bind(user_id)
        .bind(target_user_id)
        .execute(&mut *tx)
        .await?;

        // Insert or update block connection
        sqlx::query(&format!(
            "INSERT INTO territory_{}.user_users_connections 
                (user_id, target_user_id, connection_type, status)
             VALUES ($1, $2, 'block', 'active')
             ON CONFLICT (user_id, target_user_id, connection_type)
             DO UPDATE SET status = 'active', updated_at = now()",
            territory
        ))
        .bind(user_id)
        .bind(target_user_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    /// Unblock a user
    pub async fn unblock_user(
        db: &Database,
        territory: &str,
        user_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<()> {
        let result = sqlx::query(&format!(
            "DELETE FROM territory_{}.user_users_connections
             WHERE user_id = $1 AND target_user_id = $2 AND connection_type = 'block'",
            territory
        ))
        .bind(user_id)
        .bind(target_user_id)
        .execute(db.pool())
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Block connection not found".to_string()));
        }

        Ok(())
    }

    /// Get followers of a user (users who follow this user)
    pub async fn get_followers(
        db: &Database,
        territory: &str,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<ConnectionsListResponse> {
        // Get total count
        let total = sqlx::query_scalar::<_, i64>(&format!(
            "SELECT COUNT(*) FROM territory_{}.user_users_connections
             WHERE target_user_id = $1 AND connection_type = 'follow' AND status = 'active'",
            territory
        ))
        .bind(user_id)
        .fetch_one(db.pool())
        .await?;

        // Get paginated followers
        let rows = sqlx::query(&format!(
            "SELECT 
                uc.user_id,
                u.username,
                uc.connection_type,
                uc.status,
                uc.created_at
             FROM territory_{}.user_users_connections uc
             JOIN territory_{}.users u ON u.id = uc.user_id
             WHERE uc.target_user_id = $1 
               AND uc.connection_type = 'follow' 
               AND uc.status = 'active'
             ORDER BY uc.created_at DESC
             LIMIT $2 OFFSET $3",
            territory, territory
        ))
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(db.pool())
        .await?;

        let connections = rows
            .iter()
            .map(|row| ConnectionResponse {
                user_id: row.get("user_id"),
                username: row.get("username"),
                connection_type: ConnectionType::Follow,
                status: ConnectionStatus::Active,
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(ConnectionsListResponse {
            connections,
            total,
            offset,
            limit,
        })
    }

    /// Get users that a user is following
    pub async fn get_following(
        db: &Database,
        territory: &str,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<ConnectionsListResponse> {
        // Get total count
        let total = sqlx::query_scalar::<_, i64>(&format!(
            "SELECT COUNT(*) FROM territory_{}.user_users_connections
             WHERE user_id = $1 AND connection_type = 'follow' AND status = 'active'",
            territory
        ))
        .bind(user_id)
        .fetch_one(db.pool())
        .await?;

        // Get paginated following
        let rows = sqlx::query(&format!(
            "SELECT 
                uc.target_user_id,
                u.username,
                uc.connection_type,
                uc.status,
                uc.created_at
             FROM territory_{}.user_users_connections uc
             JOIN territory_{}.users u ON u.id = uc.target_user_id
             WHERE uc.user_id = $1 
               AND uc.connection_type = 'follow' 
               AND uc.status = 'active'
             ORDER BY uc.created_at DESC
             LIMIT $2 OFFSET $3",
            territory, territory
        ))
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(db.pool())
        .await?;

        let connections = rows
            .iter()
            .map(|row| ConnectionResponse {
                user_id: row.get("target_user_id"),
                username: row.get("username"),
                connection_type: ConnectionType::Follow,
                status: ConnectionStatus::Active,
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(ConnectionsListResponse {
            connections,
            total,
            offset,
            limit,
        })
    }

    /// Search for users by username or display name
    pub async fn search_users(
        db: &Database,
        territory: &str,
        current_user_id: Uuid,
        query: &str,
        limit: i64,
        offset: i64,
    ) -> Result<UserSearchResponse> {
        let search_pattern = format!("%{}%", query.to_lowercase());

        // Get total count
        let total = sqlx::query_scalar::<_, i64>(&format!(
            "SELECT COUNT(DISTINCT u.id) 
             FROM territory_{}.auth_users_core u
             LEFT JOIN territory_{}.user_users_profiles p ON p.user_id = u.id
             WHERE u.id != $1
               AND (LOWER(u.username) LIKE $2 OR LOWER(p.display_name) LIKE $2)",
            territory, territory
        ))
        .bind(current_user_id)
        .bind(&search_pattern)
        .fetch_one(db.pool())
        .await?;

        // Get paginated users with connection status
        let rows = sqlx::query(&format!(
            "SELECT 
                u.id,
                u.username,
                p.display_name,
                p.avatar_url,
                p.bio,
                EXISTS(
                    SELECT 1 FROM territory_{}.user_users_connections
                    WHERE user_id = $1 AND target_user_id = u.id 
                      AND connection_type = 'follow' AND status = 'active'
                ) as is_following,
                EXISTS(
                    SELECT 1 FROM territory_{}.user_users_connections
                    WHERE user_id = u.id AND target_user_id = $1 
                      AND connection_type = 'follow' AND status = 'active'
                ) as is_follower,
                EXISTS(
                    SELECT 1 FROM territory_{}.user_users_connections
                    WHERE user_id = $1 AND target_user_id = u.id 
                      AND connection_type = 'block' AND status = 'active'
                ) as is_blocked
             FROM territory_{}.auth_users_core u
             LEFT JOIN territory_{}.user_users_profiles p ON p.user_id = u.id
             WHERE u.id != $1
               AND (LOWER(u.username) LIKE $2 OR LOWER(p.display_name) LIKE $2)
             ORDER BY u.username
             LIMIT $3 OFFSET $4",
            territory, territory, territory, territory, territory
        ))
        .bind(current_user_id)
        .bind(&search_pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(db.pool())
        .await?;

        let users = rows
            .iter()
            .map(|row| UserSearchResult {
                id: row.get("id"),
                username: row.get("username"),
                display_name: row.get("display_name"),
                avatar_url: row.get("avatar_url"),
                bio: row.get("bio"),
                is_following: row.get("is_following"),
                is_follower: row.get("is_follower"),
                is_blocked: row.get("is_blocked"),
            })
            .collect();

        Ok(UserSearchResponse {
            users,
            total,
            offset,
            limit,
        })
    }
}
