use crate::models::territory::{
    TerritoryResponse, TerritorySettingsResponse, TerritoryStatsResponse, UpdateSettingsRequest,
};
use shared_lib::{AppError, Database, Result};
use sqlx::Row;
use uuid::Uuid;

/// Territory service business logic
pub struct TerritoryService;

impl TerritoryService {
    /// List all active territories from global registry
    pub async fn list_active_territories(db: &Database) -> Result<Vec<TerritoryResponse>> {
        let query = r#"
            SELECT 
                code,
                name,
                display_name,
                description,
                pod_url,
                api_url,
                status,
                language_code,
                timezone,
                currency_code,
                created_at,
                updated_at
            FROM global.territories_registry
            WHERE status = 'active'
            ORDER BY name ASC
        "#;

        let rows = sqlx::query(query).fetch_all(db.pool()).await?;

        let territories = rows
            .into_iter()
            .map(|row| TerritoryResponse {
                code: row.get("code"),
                name: row.get("name"),
                display_name: row.get("display_name"),
                description: row.get("description"),
                pod_url: row.get("pod_url"),
                api_url: row.get("api_url"),
                status: row.get("status"),
                language_code: row.get("language_code"),
                timezone: row.get("timezone"),
                currency_code: row.get("currency_code"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(territories)
    }

    /// Get territory by code from global registry
    pub async fn get_territory(db: &Database, code: &str) -> Result<TerritoryResponse> {
        let query = r#"
            SELECT 
                code,
                name,
                display_name,
                description,
                pod_url,
                api_url,
                status,
                language_code,
                timezone,
                currency_code,
                created_at,
                updated_at
            FROM global.territories_registry
            WHERE code = $1
        "#;

        let row = sqlx::query(query)
            .bind(code)
            .fetch_optional(db.pool())
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Territory '{}' not found", code)))?;

        Ok(TerritoryResponse {
            code: row.get("code"),
            name: row.get("name"),
            display_name: row.get("display_name"),
            description: row.get("description"),
            pod_url: row.get("pod_url"),
            api_url: row.get("api_url"),
            status: row.get("status"),
            language_code: row.get("language_code"),
            timezone: row.get("timezone"),
            currency_code: row.get("currency_code"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// Check if user is a territory manager for the given territory
    async fn verify_territory_manager(
        db: &Database,
        territory_code: &str,
        user_id: &Uuid,
    ) -> Result<()> {
        let query = r#"
            SELECT COUNT(*) as count
            FROM territory_dk.territory_managers
            WHERE territory_code = $1 AND user_id = $2
        "#;

        let row = sqlx::query(query)
            .bind(territory_code)
            .bind(user_id)
            .fetch_one(db.pool())
            .await?;

        let count: i64 = row.get("count");

        if count == 0 {
            return Err(AppError::Forbidden(
                "You are not a territory manager for this territory".to_string(),
            ));
        }

        Ok(())
    }

    /// Get territory statistics (Territory Manager only)
    pub async fn get_territory_stats(
        db: &Database,
        territory_code: &str,
        user_id: &Uuid,
    ) -> Result<TerritoryStatsResponse> {
        // Verify user is a territory manager
        Self::verify_territory_manager(db, territory_code, user_id).await?;

        // Get stats from territory_stats table (one row per territory)
        let query = r#"
            SELECT 
                total_users,
                active_users_7d,
                active_users_30d,
                total_communities,
                total_posts,
                storage_used_mb,
                calculated_at
            FROM territory_dk.territory_stats
            LIMIT 1
        "#;

        let row = sqlx::query(query)
            .fetch_optional(db.pool())
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Stats for territory '{}' not found",
                    territory_code
                ))
            })?;

        Ok(TerritoryStatsResponse {
            total_users: row.get::<i32, _>("total_users") as i64,
            active_users_7d: row.get::<i32, _>("active_users_7d") as i64,
            active_users_30d: row.get::<i32, _>("active_users_30d") as i64,
            total_communities: row.get::<i32, _>("total_communities") as i64,
            total_posts: row.get::<i32, _>("total_posts") as i64,
            storage_used_mb: row.get("storage_used_mb"),
            calculated_at: row.get("calculated_at"),
        })
    }

    /// Update territory settings (Territory Manager only)
    pub async fn update_territory_settings(
        db: &Database,
        territory_code: &str,
        user_id: &Uuid,
        updates: UpdateSettingsRequest,
    ) -> Result<TerritorySettingsResponse> {
        // Verify user is a territory manager
        Self::verify_territory_manager(db, territory_code, user_id).await?;

        // Build dynamic UPDATE query
        let mut update_fields = Vec::new();
        let mut query_params: Vec<String> = Vec::new();
        let mut param_count = 1;

        if let Some(ref name) = updates.name {
            update_fields.push(format!("name = ${}", param_count));
            query_params.push(name.clone());
            param_count += 1;
        }
        if let Some(ref display_name) = updates.display_name {
            update_fields.push(format!("display_name = ${}", param_count));
            query_params.push(display_name.clone());
            param_count += 1;
        }
        if let Some(ref description) = updates.description {
            update_fields.push(format!("description = ${}", param_count));
            query_params.push(description.clone());
            param_count += 1;
        }
        if let Some(ref language_code) = updates.language_code {
            update_fields.push(format!("language_code = ${}", param_count));
            query_params.push(language_code.clone());
            param_count += 1;
        }
        if let Some(ref timezone) = updates.timezone {
            update_fields.push(format!("timezone = ${}", param_count));
            query_params.push(timezone.clone());
            param_count += 1;
        }
        if let Some(ref currency_code) = updates.currency_code {
            update_fields.push(format!("currency_code = ${}", param_count));
            query_params.push(currency_code.clone());
            // No increment needed - this is the last field
        }

        if update_fields.is_empty() {
            return Err(AppError::Validation("No fields to update".to_string()));
        }

        // Update territory_settings (will trigger replication to global.territories_registry)
        // Note: There is only ONE row in territory_settings per pod, no WHERE clause needed
        let update_query = format!(
            r#"
            UPDATE territory_dk.territory_settings
            SET {}, updated_at = NOW()
            RETURNING 
                name,
                display_name,
                description,
                language_code,
                timezone,
                currency_code,
                registration_enabled,
                invitation_required,
                max_users,
                primary_color,
                logo_url,
                updated_at
            "#,
            update_fields.join(", ")
        );

        let mut query = sqlx::query(&update_query);
        for param in &query_params {
            query = query.bind(param);
        }

        let row = query.fetch_optional(db.pool()).await?.ok_or_else(|| {
            AppError::NotFound(format!(
                "Territory settings for '{}' not found",
                territory_code
            ))
        })?;

        Ok(TerritorySettingsResponse {
            name: row.get("name"),
            display_name: row.get("display_name"),
            description: row.get("description"),
            language_code: row.get("language_code"),
            timezone: row.get("timezone"),
            currency_code: row.get("currency_code"),
            registration_enabled: row.get("registration_enabled"),
            invitation_required: row.get("invitation_required"),
            max_users: row.get("max_users"),
            primary_color: row.get("primary_color"),
            logo_url: row.get("logo_url"),
            updated_at: row.get("updated_at"),
        })
    }
}
