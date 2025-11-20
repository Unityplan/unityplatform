use shared_lib::Result;
use sqlx::PgPool;
use uuid::Uuid;

/// Check if a user has permission to create invitations
///
/// This function implements the cross-pod manager verification logic:
/// 1. Find the user's home territory
/// 2. Check if they have a manager badge in their home territory
/// 3. Check if they are assigned to manage the target territory
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `user_id` - UUID of the user to check
/// * `target_territory` - Territory code where invitation will be created (e.g., "dk")
///
/// # Returns
/// * `Ok(true)` - User has manager permissions
/// * `Ok(false)` - User does not have manager permissions
/// * `Err(_)` - Database error occurred
pub async fn check_manager_permissions(
    pool: &PgPool,
    user_id: Uuid,
    target_territory: &str,
) -> Result<bool> {
    // Step 1: Find user's home territory
    let home_territory: Option<String> = sqlx::query_scalar(
        "SELECT territory_code FROM global.registry_username WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let Some(home_territory) = home_territory else {
        // User not found in global registry
        tracing::warn!(
            user_id = %user_id,
            "User not found in global registry"
        );
        return Ok(false);
    };

    tracing::debug!(
        user_id = %user_id,
        home_territory = %home_territory,
        target_territory = %target_territory,
        "Checking manager permissions"
    );

    // Step 2: Check if user has manager badge in their HOME territory
    let has_badge_query = format!(
        "SELECT EXISTS(
            SELECT 1 FROM territory_{}.badge_users_badges ub
            JOIN global.registry_badge b ON ub.badge_id = b.id
            WHERE ub.user_id = $1
              AND b.slug IN ('territory-manager', 'community-manager')
              AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
        )",
        home_territory
    );

    let has_badge: bool = sqlx::query_scalar(&has_badge_query)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    if !has_badge {
        tracing::info!(
            user_id = %user_id,
            home_territory = %home_territory,
            "User does not have manager badge in home territory"
        );
        return Ok(false);
    }

    // Step 3: Check if user is assigned to manage the TARGET territory
    let is_assigned_query = format!(
        "SELECT EXISTS(
            SELECT 1 FROM territory_{}.territory_territories_managers
            WHERE user_id = $1 AND territory_code = $2
        )",
        target_territory
    );

    let is_assigned: bool = sqlx::query_scalar(&is_assigned_query)
        .bind(user_id)
        .bind(target_territory)
        .fetch_one(pool)
        .await?;

    if !is_assigned {
        tracing::info!(
            user_id = %user_id,
            home_territory = %home_territory,
            target_territory = %target_territory,
            "User has manager badge but is not assigned to manage target territory"
        );
        return Ok(false);
    }

    tracing::info!(
        user_id = %user_id,
        home_territory = %home_territory,
        target_territory = %target_territory,
        "User has manager permissions"
    );

    Ok(true)
}
