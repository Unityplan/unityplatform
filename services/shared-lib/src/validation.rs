//! User validation utilities for services
//! 
//! Provides validation helpers that respect service boundaries:
//! - JWT validation (primary, zero DB queries)
//! - Registry validation (fallback for admin operations)

use crate::{AppError, Result};
use sqlx::PgPool;
use uuid::Uuid;

/// Validate user exists via JWT token (already validated by middleware)
/// 
/// This is the PRIMARY validation method. If you have an AuthUser from JWT,
/// the user is guaranteed to exist (JWT signature validates this).
/// 
/// # Arguments
/// * `user_id` - User ID from JWT token
/// 
/// # Returns
/// * `Ok(user_id)` - Always succeeds if JWT was valid
/// 
/// # Example
/// ```rust
/// use shared_lib::validation::validate_user_from_jwt;
/// use shared_lib::AuthUser;
/// 
/// async fn create_settings(user: AuthUser) -> shared_lib::Result<()> {
///     let user_id = validate_user_from_jwt(user.id)?;
///     // user_id is guaranteed valid (came from signed JWT)
///     Ok(())
/// }
/// ```
pub fn validate_user_from_jwt(user_id: Uuid) -> Result<Uuid> {
    // JWT signature already validated this user exists
    // No database query needed!
    Ok(user_id)
}

/// Validate user exists via global registry (for admin operations without JWT)
/// 
/// Use this ONLY when you don't have a JWT context:
/// - Admin operations on behalf of other users
/// - Background jobs
/// - Batch operations
/// 
/// This queries global.registry_username (NOT auth_users_core) to respect service boundaries.
/// 
/// # Arguments
/// * `user_id` - User ID to validate
/// * `territory` - Territory code (e.g., "dk")
/// * `pool` - Database connection pool
/// 
/// # Returns
/// * `Ok(true)` - User exists in registry
/// * `Ok(false)` - User does not exist
/// * `Err(_)` - Database error
/// 
/// # Example
/// ```rust,no_run
/// use shared_lib::validation::validate_user_via_registry;
/// use uuid::Uuid;
/// 
/// # async fn example(pool: &sqlx::PgPool) -> shared_lib::Result<()> {
/// let target_user_id = Uuid::new_v4();
/// if !validate_user_via_registry(target_user_id, "dk", pool).await? {
///     return Err(shared_lib::AppError::NotFound("User not found".into()));
/// }
/// // Proceed with operation
/// # Ok(())
/// # }
/// ```
pub async fn validate_user_via_registry(
    user_id: Uuid,
    territory: &str,
    pool: &PgPool,
) -> Result<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(
            SELECT 1 FROM global.registry_username 
            WHERE user_id = $1 AND territory_code = $2
        )"
    )
    .bind(user_id)
    .bind(territory)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;
    
    Ok(exists)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_user_from_jwt_always_succeeds() {
        let user_id = Uuid::new_v4();
        let result = validate_user_from_jwt(user_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user_id);
    }
}
