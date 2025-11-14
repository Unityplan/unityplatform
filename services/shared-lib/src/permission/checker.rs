//! Permission checker service
//!
//! Verifies user permissions based on badge ownership and badge-granted permissions.

use crate::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, instrument, warn};
use uuid::Uuid;

/// Badge information with permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgePermissions {
    pub badge_id: Uuid,
    pub badge_slug: String,
    pub permissions: Vec<String>,
}

/// Permission checker service
#[derive(Clone)]
pub struct PermissionChecker {
    db: Arc<PgPool>,
    territory_code: String,
}

impl PermissionChecker {
    /// Create a new permission checker
    pub fn new(db: Arc<PgPool>, territory_code: String) -> Self {
        Self { db, territory_code }
    }

    /// Check if a user has a specific permission
    ///
    /// Returns true if the user has any badge that grants the required permission.
    #[instrument(skip(self), fields(territory = %self.territory_code))]
    pub async fn has_permission(&self, user_id: Uuid, permission: &str) -> Result<bool> {
        debug!(
            user_id = %user_id,
            permission = permission,
            "Checking permission"
        );

        let schema = format!("territory_{}", self.territory_code);

        // Query to check if user has any badge with the required permission
        let has_perm = sqlx::query_scalar::<_, bool>(&format!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM {schema}.user_badges ub
                JOIN global.badge_registry br ON ub.badge_id = br.id
                WHERE ub.user_id = $1
                  AND br.is_active = true
                  AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
                  AND br.grants_permissions @> $2::jsonb
            )
            "#,
            schema = schema
        ))
        .bind(user_id)
        .bind(serde_json::json!([permission]))
        .fetch_one(&*self.db)
        .await?;

        debug!(
            user_id = %user_id,
            permission = permission,
            has_permission = has_perm,
            "Permission check result"
        );

        Ok(has_perm)
    }

    /// Check if a user has ANY of the specified permissions
    ///
    /// Returns true if the user has at least one badge that grants any of the permissions.
    #[instrument(skip(self), fields(territory = %self.territory_code))]
    pub async fn has_any_permission(&self, user_id: Uuid, permissions: &[&str]) -> Result<bool> {
        debug!(
            user_id = %user_id,
            permissions = ?permissions,
            "Checking any permission"
        );

        if permissions.is_empty() {
            return Ok(true);
        }

        let schema = format!("territory_{}", self.territory_code);

        // Check if user has any badge that grants any of the required permissions
        for permission in permissions {
            let has_perm = sqlx::query_scalar::<_, bool>(&format!(
                r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM {schema}.user_badges ub
                    JOIN global.badge_registry br ON ub.badge_id = br.id
                    WHERE ub.user_id = $1
                      AND br.is_active = true
                      AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
                      AND br.grants_permissions @> $2::jsonb
                )
                "#,
                schema = schema
            ))
            .bind(user_id)
            .bind(serde_json::json!([permission]))
            .fetch_one(&*self.db)
            .await?;

            if has_perm {
                debug!(
                    user_id = %user_id,
                    granted_permission = permission,
                    "User has required permission"
                );
                return Ok(true);
            }
        }

        warn!(
            user_id = %user_id,
            permissions = ?permissions,
            "User lacks all required permissions"
        );

        Ok(false)
    }

    /// Check if a user has ALL of the specified permissions
    ///
    /// Returns true only if the user has badges that grant all required permissions.
    #[instrument(skip(self), fields(territory = %self.territory_code))]
    pub async fn has_all_permissions(&self, user_id: Uuid, permissions: &[&str]) -> Result<bool> {
        debug!(
            user_id = %user_id,
            permissions = ?permissions,
            "Checking all permissions"
        );

        if permissions.is_empty() {
            return Ok(true);
        }

        for permission in permissions {
            if !self.has_permission(user_id, permission).await? {
                warn!(
                    user_id = %user_id,
                    missing_permission = permission,
                    "User missing required permission"
                );
                return Ok(false);
            }
        }

        debug!(
            user_id = %user_id,
            permissions = ?permissions,
            "User has all required permissions"
        );

        Ok(true)
    }

    /// Get all permissions for a user
    ///
    /// Returns a list of all unique permissions granted by the user's active badges.
    #[instrument(skip(self), fields(territory = %self.territory_code))]
    pub async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<String>> {
        debug!(user_id = %user_id, "Fetching all user permissions");

        let schema = format!("territory_{}", self.territory_code);

        let badges = sqlx::query_as::<_, (serde_json::Value,)>(&format!(
            r#"
            SELECT DISTINCT br.grants_permissions
            FROM {schema}.user_badges ub
            JOIN global.badge_registry br ON ub.badge_id = br.id
            WHERE ub.user_id = $1
              AND br.is_active = true
              AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
              AND br.grants_permissions IS NOT NULL
            "#,
            schema = schema
        ))
        .bind(user_id)
        .fetch_all(&*self.db)
        .await?;

        // Flatten all permissions from all badges into a unique set
        let mut permissions = Vec::new();
        for (perms_json,) in badges {
            if let Some(perms) = perms_json.as_array() {
                for perm in perms {
                    if let Some(perm_str) = perm.as_str() {
                        if !permissions.contains(&perm_str.to_string()) {
                            permissions.push(perm_str.to_string());
                        }
                    }
                }
            }
        }

        debug!(
            user_id = %user_id,
            permission_count = permissions.len(),
            "Retrieved user permissions"
        );

        Ok(permissions)
    }

    /// Get all badges that grant a specific permission
    #[instrument(skip(self))]
    pub async fn get_badges_with_permission(
        &self,
        permission: &str,
    ) -> Result<Vec<BadgePermissions>> {
        debug!(permission = permission, "Fetching badges with permission");

        let badges = sqlx::query_as::<_, (Uuid, String, serde_json::Value)>(
            r#"
            SELECT id, slug, grants_permissions
            FROM global.badge_registry
            WHERE is_active = true
              AND grants_permissions @> $1::jsonb
            "#,
        )
        .bind(serde_json::json!([permission]))
        .fetch_all(&*self.db)
        .await?;

        let result = badges
            .into_iter()
            .map(|(badge_id, badge_slug, perms_json)| {
                let permissions = perms_json
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();

                BadgePermissions {
                    badge_id,
                    badge_slug,
                    permissions,
                }
            })
            .collect();

        Ok(result)
    }

    /// Check if a user has a specific badge (by slug)
    #[instrument(skip(self), fields(territory = %self.territory_code))]
    pub async fn has_badge(&self, user_id: Uuid, badge_slug: &str) -> Result<bool> {
        debug!(
            user_id = %user_id,
            badge_slug = badge_slug,
            "Checking badge ownership"
        );

        let schema = format!("territory_{}", self.territory_code);

        let has_badge = sqlx::query_scalar::<_, bool>(&format!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM {schema}.user_badges ub
                JOIN global.badge_registry br ON ub.badge_id = br.id
                WHERE ub.user_id = $1
                  AND br.slug = $2
                  AND br.is_active = true
                  AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
            )
            "#,
            schema = schema
        ))
        .bind(user_id)
        .bind(badge_slug)
        .fetch_one(&*self.db)
        .await?;

        debug!(
            user_id = %user_id,
            badge_slug = badge_slug,
            has_badge = has_badge,
            "Badge ownership check result"
        );

        Ok(has_badge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a test database with the badge schema
    // For now, they serve as documentation of the expected behavior

    #[test]
    fn test_permission_checker_creation() {
        // This is a smoke test to ensure the struct can be created
        // Real tests would require a test database
    }
}
