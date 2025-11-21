use shared_lib::{AppError, Result, NatsClient};
use sqlx::{PgPool, QueryBuilder, Postgres};
use uuid::Uuid;
use chrono::Utc;
use crate::models::{
    CreateCommunityRequest, CommunityFilter,
    Community, CommunityType, CommunityManagerAssignedEvent, CommunityManagerRevokedEvent
};



#[derive(Clone)]
pub struct CommunityService {
    pool: PgPool,
}

impl CommunityService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_community(&self, req: CreateCommunityRequest, user_id: Uuid) -> Result<Community> {
        // Validate hierarchy logic
        if req.community_type == CommunityType::Physical && req.territory_id.is_none() {
             return Err(AppError::Validation("Physical communities must have a territory_id".into()));
        }

        let community = sqlx::query_as::<_, Community>(
            r#"
            INSERT INTO territory_dk.communities (
                slug, name, description, type, territory_id, parent_community_id,
                avatar_url, banner_url, is_public, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#
        )
        .bind(req.slug)
        .bind(req.name)
        .bind(req.description)
        .bind(req.community_type)
        .bind(req.territory_id)
        .bind(req.parent_community_id)
        .bind(req.avatar_url)
        .bind(req.banner_url)
        .bind(req.is_public)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique constraint") {
                AppError::Conflict("Community slug already exists".into())
            } else {
                AppError::Database(e)
            }
        })?;

        // Add creator as admin
        sqlx::query(
            r#"
            INSERT INTO territory_dk.community_members (community_id, user_id, role)
            VALUES ($1, $2, 'admin')
            "#
        )
        .bind(community.id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Create default settings
        sqlx::query(
            r#"
            INSERT INTO territory_dk.community_settings (community_id)
            VALUES ($1)
            "#
        )
        .bind(community.id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(community)
    }

    pub async fn get_community(&self, id: Uuid) -> Result<Community> {
        sqlx::query_as::<_, Community>(
            "SELECT * FROM territory_dk.communities WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Community not found".into()))
    }

    pub async fn list_communities(&self, filter: CommunityFilter) -> Result<Vec<Community>> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM territory_dk.communities WHERE 1=1");

        if let Some(ctype) = filter.community_type {
            query_builder.push(" AND type = ");
            query_builder.push_bind(ctype);
        }

        if let Some(pid) = filter.parent_id {
            query_builder.push(" AND parent_community_id = ");
            query_builder.push_bind(pid);
        }

        if let Some(tid) = filter.territory_id {
            query_builder.push(" AND territory_id = ");
            query_builder.push_bind(tid);
        }
        
        if let Some(search) = filter.search {
             let search_term = format!("%{}%", search);
             query_builder.push(" AND (name ILIKE ");
             query_builder.push_bind(search_term.clone());
             query_builder.push(" OR slug ILIKE ");
             query_builder.push_bind(search_term);
             query_builder.push(")");
        }

        query_builder.push(" ORDER BY created_at DESC LIMIT 50");

        query_builder.build_query_as::<Community>()
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    pub async fn assign_manager(&self, community_id: Uuid, user_id: Uuid, assigned_by: Uuid, nats: &NatsClient) -> Result<()> {
        // Upsert member with role 'admin'
        sqlx::query(
            r#"
            INSERT INTO territory_dk.community_members (community_id, user_id, role, invited_by)
            VALUES ($1, $2, 'admin', $3)
            ON CONFLICT (community_id, user_id) 
            DO UPDATE SET role = 'admin'
            "#
        )
        .bind(community_id)
        .bind(user_id)
        .bind(assigned_by)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Publish event
        let event = CommunityManagerAssignedEvent {
            community_id,
            user_id,
            assigned_by,
            timestamp: Utc::now(),
        };
        
        nats.publish("community.manager_assigned", serde_json::to_vec(&event)?).await.map_err(|e| AppError::Nats(e.to_string()))?;
        
        Ok(())
    }

    pub async fn revoke_manager(&self, community_id: Uuid, user_id: Uuid, revoked_by: Uuid, nats: &NatsClient) -> Result<()> {
        // Update role to 'member' (or delete if we want to remove them completely, but usually we just demote)
        // Requirement says "revoke if manager role is removed".
        // Let's demote to 'member'.
        sqlx::query(
            r#"
            UPDATE territory_dk.community_members
            SET role = 'member'
            WHERE community_id = $1 AND user_id = $2 AND role = 'admin'
            "#
        )
        .bind(community_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Publish event
        let event = CommunityManagerRevokedEvent {
            community_id,
            user_id,
            revoked_by,
            timestamp: Utc::now(),
        };
        
        nats.publish("community.manager_revoked", serde_json::to_vec(&event)?).await.map_err(|e| AppError::Nats(e.to_string()))?;
        
        Ok(())
    }
}


