use crate::models::{BadgeResponse, RegisterBadgeRequest, UserBadgeResponse};
use shared_lib::{AppError, Database, NatsClient, Result};
use sqlx::Row;
use uuid::Uuid;

/// List all available badges with optional user progress
pub async fn list_badges(db: &Database, user_id: Option<Uuid>) -> Result<Vec<BadgeResponse>> {
    // TODO: Get territory from AppConfig instead of hardcoding
    let territory = "dk";

    let query = if let Some(_uid) = user_id {
        format!(
            r#"
            SELECT 
                b.id, b.name, b.slug, b.description, b.icon,
                b.criteria_type, b.criteria_value, b.rarity,
                b.is_active, b.created_at, b.updated_at,
                EXISTS(SELECT 1 FROM territory_{}.badge_users_badges WHERE user_id = $1 AND badge_id = b.id) as user_has_badge,
                bp.current_value as user_progress,
                bp.target_value as user_target
            FROM global.registry_badge b
            LEFT JOIN territory_{}.badge_users_progress bp ON b.id = bp.badge_id AND bp.user_id = $1
            WHERE b.is_active = true
            ORDER BY 
                CASE b.rarity
                    WHEN 'legendary' THEN 1
                    WHEN 'epic' THEN 2
                    WHEN 'rare' THEN 3
                    WHEN 'common' THEN 4
                END,
                b.name ASC
            "#,
            territory, territory
        )
    } else {
        r#"
            SELECT 
                id, name, slug, description, icon,
                criteria_type, criteria_value, rarity,
                is_active, created_at, updated_at
            FROM global.registry_badge
            WHERE is_active = true
            ORDER BY 
                CASE rarity
                    WHEN 'legendary' THEN 1
                    WHEN 'epic' THEN 2
                    WHEN 'rare' THEN 3
                    WHEN 'common' THEN 4
                END,
                name ASC
            "#
        .to_string()
    };

    let rows = if let Some(uid) = user_id {
        sqlx::query(&query).bind(uid).fetch_all(db.pool()).await?
    } else {
        sqlx::query(&query).fetch_all(db.pool()).await?
    };

    let badges = rows
        .iter()
        .map(|row| {
            Ok(BadgeResponse {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                slug: row.try_get("slug")?,
                description: row.try_get("description")?,
                icon: row.try_get("icon")?,
                criteria_type: row.try_get("criteria_type")?,
                criteria_value: row.try_get("criteria_value")?,
                rarity: row.try_get("rarity")?,
                is_active: row.try_get("is_active")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
                user_has_badge: row.try_get("user_has_badge").ok(),
                user_progress: row.try_get("user_progress").ok(),
                user_target: row.try_get("user_target").ok(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(badges)
}

/// Get badges for a specific user
pub async fn get_user_badges(db: &Database, user_id: Uuid) -> Result<Vec<UserBadgeResponse>> {
    let territory = "dk";

    let query = format!(
        r#"
        SELECT 
            ub.id, ub.badge_id, ub.awarded_at, ub.awarded_by, ub.is_featured,
            b.name as badge_name, b.slug as badge_slug, 
            b.icon as badge_icon, b.rarity as badge_rarity
        FROM territory_{}.badge_users_badges ub
        JOIN global.registry_badge b ON ub.badge_id = b.id
        WHERE ub.user_id = $1
        ORDER BY ub.awarded_at DESC
        "#,
        territory
    );

    let rows = sqlx::query(&query)
        .bind(user_id)
        .fetch_all(db.pool())
        .await?;

    let badges = rows
        .iter()
        .map(|row| {
            Ok(UserBadgeResponse {
                id: row.try_get("id")?,
                badge_id: row.try_get("badge_id")?,
                badge_name: row.try_get("badge_name")?,
                badge_slug: row.try_get("badge_slug")?,
                badge_icon: row.try_get("badge_icon")?,
                badge_rarity: row.try_get("badge_rarity")?,
                awarded_at: row.try_get("awarded_at")?,
                awarded_by: row.try_get("awarded_by")?,
                is_featured: row.try_get("is_featured")?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(badges)
}

/// Award a badge to a user
pub async fn award_badge(
    db: &Database,
    nats: &NatsClient,
    user_id: Uuid,
    badge_slug: &str,
    awarded_by: Option<Uuid>,
    reason: Option<String>,
) -> Result<Uuid> {
    let territory = "dk";

    // Get badge info from slug
    let badge_query =
        "SELECT id, name FROM global.registry_badge WHERE slug = $1 AND is_active = true";
    let badge_row = sqlx::query(badge_query)
        .bind(badge_slug)
        .fetch_optional(db.pool())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Badge '{}' not found", badge_slug)))?;

    let badge_id: Uuid = badge_row.try_get("id")?;
    let badge_name: String = badge_row.try_get("name")?;

    // Check if user already has the badge
    let check_query = format!(
        "SELECT EXISTS(SELECT 1 FROM territory_{}.badge_users_badges WHERE user_id = $1 AND badge_id = $2)",
        territory
    );
    let already_has: bool = sqlx::query_scalar(&check_query)
        .bind(user_id)
        .bind(badge_id)
        .fetch_one(db.pool())
        .await?;

    if already_has {
        return Err(AppError::Validation(format!(
            "User already has badge '{}'",
            badge_slug
        )));
    }

    // Award the badge
    let insert_query = format!(
        r#"
        INSERT INTO territory_{}.badge_users_badges 
        (badge_id, user_id, awarded_by, awarded_at)
        VALUES ($1, $2, $3, NOW())
        RETURNING id
        "#,
        territory
    );
    let award_id: Uuid = sqlx::query_scalar(&insert_query)
        .bind(badge_id)
        .bind(user_id)
        .bind(awarded_by)
        .fetch_one(db.pool())
        .await?;

    tracing::info!(
        "Badge '{}' awarded to user {} (award_id: {}, reason: {:?})",
        badge_slug,
        user_id,
        award_id,
        reason
    );

    // Publish badge.awarded event
    let event_payload = serde_json::json!({
        "user_id": user_id,
        "badge_slug": badge_slug,
        "badge_name": badge_name,
        "awarded_at": chrono::Utc::now(),
        "awarded_by": awarded_by,
        "reason": reason,
    });

    let payload_bytes = serde_json::to_vec(&event_payload).unwrap_or_default();
    if let Err(e) = nats.publish("badge.awarded", payload_bytes).await {
        tracing::error!(
            "Failed to publish badge.awarded event for user {} badge '{}': {}",
            user_id,
            badge_slug,
            e
        );
        // Don't fail the operation if event publishing fails
    }

    Ok(award_id)
}

/// Revoke a badge from a user
pub async fn revoke_badge(
    db: &Database,
    nats: &NatsClient,
    user_id: Uuid,
    badge_slug: &str,
    reason: Option<String>,
) -> Result<()> {
    let territory = "dk";

    // Get badge info from slug
    let badge_query = "SELECT id, name FROM global.registry_badge WHERE slug = $1";
    let badge_row = sqlx::query(badge_query)
        .bind(badge_slug)
        .fetch_optional(db.pool())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Badge '{}' not found", badge_slug)))?;

    let badge_id: Uuid = badge_row.try_get("id")?;
    let badge_name: String = badge_row.try_get("name")?;

    // Delete the badge award
    let delete_query = format!(
        "DELETE FROM territory_{}.badge_users_badges WHERE user_id = $1 AND badge_id = $2",
        territory
    );
    let result = sqlx::query(&delete_query)
        .bind(user_id)
        .bind(badge_id)
        .execute(db.pool())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "User does not have badge '{}'",
            badge_slug
        )));
    }

    tracing::info!(
        "Badge '{}' revoked from user {} (reason: {:?})",
        badge_slug,
        user_id,
        reason
    );

    // Publish badge.revoked event
    let event_payload = serde_json::json!({
        "user_id": user_id,
        "badge_slug": badge_slug,
        "badge_name": badge_name,
        "revoked_at": chrono::Utc::now(),
        "reason": reason,
    });

    let payload_bytes = serde_json::to_vec(&event_payload).unwrap_or_default();
    if let Err(e) = nats.publish("badge.revoked", payload_bytes).await {
        tracing::error!(
            "Failed to publish badge.revoked event for user {} badge '{}': {}",
            user_id,
            badge_slug,
            e
        );
        // Don't fail the operation if event publishing fails
    }

    Ok(())
}

/// Update badge progress for a user
pub async fn update_badge_progress(
    db: &Database,
    nats: &NatsClient,
    user_id: Uuid,
    badge_slug: &str,
    current_value: i32,
) -> Result<bool> {
    let territory = "dk";

    // Get badge info
    let badge_query =
        "SELECT id, criteria_value FROM global.registry_badge WHERE slug = $1 AND is_active = true";
    let row = sqlx::query(badge_query)
        .bind(badge_slug)
        .fetch_optional(db.pool())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Badge '{}' not found", badge_slug)))?;

    let badge_id: Uuid = row.try_get("id")?;
    let target_value: Option<i32> = row.try_get("criteria_value")?;

    let target = target_value.unwrap_or(1);

    // Upsert progress
    let upsert_query = format!(
        r#"
        INSERT INTO territory_{}.badge_users_progress 
        (badge_id, user_id, current_value, target_value, updated_at)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (user_id, badge_id) 
        DO UPDATE SET current_value = $3, updated_at = NOW()
        "#,
        territory
    );
    sqlx::query(&upsert_query)
        .bind(badge_id)
        .bind(user_id)
        .bind(current_value)
        .bind(target)
        .execute(db.pool())
        .await?;

    // Check if badge should be auto-awarded
    let should_award = current_value >= target;

    if should_award {
        // Check if already has badge
        let check_query = format!(
            "SELECT EXISTS(SELECT 1 FROM territory_{}.badge_users_badges WHERE user_id = $1 AND badge_id = $2)",
            territory
        );
        let already_has: bool = sqlx::query_scalar(&check_query)
            .bind(user_id)
            .bind(badge_id)
            .fetch_one(db.pool())
            .await?;

        if !already_has {
            // Auto-award the badge
            award_badge(
                db,
                nats,
                user_id,
                badge_slug,
                None,
                Some("Auto-awarded based on progress".to_string()),
            )
            .await?;
            tracing::info!(
                "Auto-awarded badge '{}' to user {} (progress: {}/{})",
                badge_slug,
                user_id,
                current_value,
                target
            );
            return Ok(true);
        }
    }

    Ok(false)
}

/// Toggle featured status of a user's badge
pub async fn toggle_featured_badge(
    db: &Database,
    user_id: Uuid,
    badge_id: Uuid,
    is_featured: bool,
) -> Result<()> {
    let territory = "dk";

    // If setting to featured, unfeatured all other badges first
    if is_featured {
        let unfeatured_query = format!(
            "UPDATE territory_{}.badge_users_badges SET is_featured = false WHERE user_id = $1",
            territory
        );
        sqlx::query(&unfeatured_query)
            .bind(user_id)
            .execute(db.pool())
            .await?;
    }

    // Update the target badge
    let update_query = format!(
        "UPDATE territory_{}.badge_users_badges SET is_featured = $1 WHERE user_id = $2 AND badge_id = $3",
        territory
    );
    let result = sqlx::query(&update_query)
        .bind(is_featured)
        .bind(user_id)
        .bind(badge_id)
        .execute(db.pool())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User badge not found".to_string()));
    }

    Ok(())
}

/// Register a new badge (for services to register their role badges)
pub async fn register_badge(db: &Database, req: RegisterBadgeRequest) -> Result<BadgeResponse> {
    // Check if badge already exists
    let existing = sqlx::query(
        "SELECT id, name, slug, description, icon, criteria_type, criteria_value, rarity, is_active, created_at, updated_at FROM global.registry_badge WHERE slug = $1"
    )
    .bind(&req.slug)
    .fetch_optional(db.pool())
    .await?;

    if let Some(row) = existing {
        // Badge already exists, return it
        return Ok(BadgeResponse {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            description: row.try_get("description")?,
            icon: row.try_get("icon")?,
            criteria_type: row.try_get("criteria_type")?,
            criteria_value: row.try_get("criteria_value")?,
            rarity: row.try_get("rarity")?,
            is_active: row.try_get("is_active")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            user_has_badge: None,
            user_progress: None,
            user_target: None,
        });
    }

    // Insert new badge
    let permissions_json = serde_json::to_value(&req.grants_permissions)
        .map_err(|e| AppError::Internal(format!("Failed to serialize permissions: {}", e)))?;

    let row = sqlx::query(
        r#"
        INSERT INTO global.registry_badge (
            slug, name, description, icon, category, criteria_type, criteria_value,
            rarity, is_renewable, renewal_days, grants_permissions
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, name, slug, description, icon, criteria_type, criteria_value, rarity, is_active, created_at, updated_at
        "#
    )
    .bind(&req.slug)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.icon)
    .bind(&req.category)
    .bind(&req.criteria_type)
    .bind(req.criteria_value)
    .bind(&req.rarity)
    .bind(req.is_renewable.unwrap_or(false))
    .bind(req.renewal_days)
    .bind(permissions_json)
    .fetch_one(db.pool())
    .await?;

    Ok(BadgeResponse {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
        slug: row.try_get("slug")?,
        description: row.try_get("description")?,
        icon: row.try_get("icon")?,
        criteria_type: row.try_get("criteria_type")?,
        criteria_value: row.try_get("criteria_value")?,
        rarity: row.try_get("rarity")?,
        is_active: row.try_get("is_active")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        user_has_badge: None,
        user_progress: None,
        user_target: None,
    })
}
