use crate::models::{
    Community, CommunityBadgeRequirement, CommunityFilter, CommunityManagerAssignedEvent,
    CommunityManagerRevokedEvent, CommunityType, CreateCommunityRequest, EffectiveBadgeRequirement,
    EffectiveManager, GroupChild, GroupSummary, RequirementContext, UpdateCommunityRequest,
};
use chrono::Utc;
use shared_lib::{AppError, NatsClient, Result};
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct CommunityService {
    pool: PgPool,
}

impl CommunityService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_community(
        &self,
        req: CreateCommunityRequest,
        user_id: Uuid,
    ) -> Result<Community> {
        // Validate hierarchy logic
        if (req.community_type == CommunityType::Zone
            || req.community_type == CommunityType::Neighborhood)
            && req.territory_id.is_none()
        {
            return Err(AppError::Validation(
                "Physical communities (Zone/Neighborhood) must have a territory_id".into(),
            ));
        }

        // Validate parent type for Zone/Neighborhood
        if let Some(parent_id) = req.parent_community_id {
            if req.community_type == CommunityType::Zone
                || req.community_type == CommunityType::Neighborhood
            {
                let parent = self.get_community(parent_id).await?;
                if parent.community_type != CommunityType::Zone
                    && parent.community_type != CommunityType::Neighborhood
                {
                    return Err(AppError::Validation(
                        "Zone and Neighborhood communities can only have Zone or Neighborhood parents"
                            .into(),
                    ));
                }
            }
        }

        // Generate slug from name
        let slug = req
            .name
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .trim_matches('-')
            .to_string();

        // Ensure slug is unique (simple retry logic or append random string could be added,
        // but for now let's rely on DB constraint and maybe append random if needed in future)
        // For MVP, let's append a short random string to ensure uniqueness if it's common name
        let slug = format!(
            "{}-{}",
            slug,
            Uuid::new_v4().simple().to_string()[..6].to_string()
        );

        let community = sqlx::query_as::<_, Community>(
            r#"
            INSERT INTO territory_dk.community_communities (
                slug, name, description, type, territory_id, parent_community_id,
                avatar_url, banner_url, created_by, location_lat, location_lng, coverage_area
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(slug)
        .bind(req.name)
        .bind(req.description)
        .bind(req.community_type)
        .bind(req.territory_id)
        .bind(req.parent_community_id)
        .bind(req.avatar_url)
        .bind(req.banner_url)
        .bind(user_id)
        .bind(req.location_lat)
        .bind(req.location_lng)
        .bind(req.coverage_area.map(sqlx::types::Json))
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
            INSERT INTO territory_dk.community_communities_managers (community_id, user_id, assigned_by)
            VALUES ($1, $2, $2)
            "#,
        )
        .bind(community.id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Create default settings
        sqlx::query(
            r#"
            INSERT INTO territory_dk.community_communities_settings (community_id, inherit_requirements)
            VALUES ($1, $2)
            "#,
        )
        .bind(community.id)
        .bind(req.inherit_requirements)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Add initial requirements
        for req in req.initial_requirements {
            self.add_badge_requirement(community.id, req.badge_id, req.context)
                .await?;
        }

        Ok(community)
    }

    pub async fn get_community(&self, id: Uuid) -> Result<Community> {
        sqlx::query_as::<_, Community>(
            "SELECT * FROM territory_dk.community_communities WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Community not found".into()))
    }

    pub async fn list_communities(&self, filter: CommunityFilter) -> Result<Vec<Community>> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT * FROM territory_dk.community_communities WHERE 1=1");

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

        query_builder.push(" ORDER BY name ASC");

        // Apply pagination with sensible defaults
        let limit = filter.limit.unwrap_or(100).min(500).max(1);
        let offset = filter.offset.unwrap_or(0).max(0);

        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        query_builder
            .build_query_as::<Community>()
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    /// List communities with pagination metadata (for infinite scroll)
    pub async fn list_communities_paginated(
        &self,
        filter: CommunityFilter,
    ) -> Result<crate::models::PaginatedCommunities> {
        // First get total count
        let mut count_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT COUNT(*) FROM territory_dk.community_communities WHERE 1=1");

        if let Some(ref ctype) = filter.community_type {
            count_builder.push(" AND type = ");
            count_builder.push_bind(ctype.clone());
        }

        if let Some(pid) = filter.parent_id {
            count_builder.push(" AND parent_community_id = ");
            count_builder.push_bind(pid);
        }

        if let Some(ref tid) = filter.territory_id {
            count_builder.push(" AND territory_id = ");
            count_builder.push_bind(tid.clone());
        }

        if let Some(ref search) = filter.search {
            let search_term = format!("%{}%", search);
            count_builder.push(" AND (name ILIKE ");
            count_builder.push_bind(search_term.clone());
            count_builder.push(" OR slug ILIKE ");
            count_builder.push_bind(search_term);
            count_builder.push(")");
        }

        let total: (i64,) = count_builder
            .build_query_as()
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;

        // Then get items
        let limit = filter.limit.unwrap_or(50).min(500).max(1) as i64;
        let offset = filter.offset.unwrap_or(0).max(0) as i64;
        let items = self.list_communities(filter).await?;

        Ok(crate::models::PaginatedCommunities {
            has_more: offset + (items.len() as i64) < total.0,
            total: total.0,
            limit,
            offset,
            items,
        })
    }

    /// Get root communities (communities with no parent)
    pub async fn get_root_communities(&self) -> Result<Vec<Community>> {
        sqlx::query_as::<_, Community>(
            r#"
            SELECT * FROM territory_dk.community_communities 
            WHERE parent_community_id IS NULL
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    /// Get direct children of a community
    pub async fn get_children(
        &self,
        community_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<Community>> {
        let limit = limit.unwrap_or(100).min(500);
        sqlx::query_as::<_, Community>(
            r#"
            SELECT * FROM territory_dk.community_communities 
            WHERE parent_community_id = $1
            ORDER BY name ASC
            LIMIT $2
            "#,
        )
        .bind(community_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    /// Get count of direct children
    pub async fn get_children_count(&self, community_id: Uuid) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM territory_dk.community_communities 
            WHERE parent_community_id = $1
            "#,
        )
        .bind(community_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(result.0)
    }

    /// Get ancestors of a community (from root to parent)
    pub async fn get_ancestors(&self, community_id: Uuid) -> Result<Vec<Community>> {
        // Use recursive CTE to get all ancestors
        sqlx::query_as::<_, Community>(
            r#"
            WITH RECURSIVE ancestors AS (
                SELECT c.*, 0 as depth
                FROM territory_dk.community_communities c
                WHERE c.id = (
                    SELECT parent_community_id 
                    FROM territory_dk.community_communities 
                    WHERE id = $1
                )
                
                UNION ALL
                
                SELECT c.*, a.depth + 1
                FROM territory_dk.community_communities c
                JOIN ancestors a ON c.id = a.parent_community_id
            )
            SELECT id, slug, name, description, type, territory_id, parent_community_id,
                   avatar_url, banner_url, member_count, created_by, created_at, updated_at,
                   location_lat, location_lng, coverage_area
            FROM ancestors
            ORDER BY depth DESC
            "#,
        )
        .bind(community_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    /// Get community with full context (ancestors + children) for flow view
    pub async fn get_community_context(
        &self,
        community_id: Uuid,
        children_limit: Option<i64>,
    ) -> Result<crate::models::CommunityContext> {
        let community = self.get_community(community_id).await?;
        let ancestors = self.get_ancestors(community_id).await?;
        let children = self.get_children(community_id, children_limit).await?;
        let children_count = self.get_children_count(community_id).await?;

        Ok(crate::models::CommunityContext {
            community,
            ancestors,
            has_more_children: children.len() as i64 > children_count,
            children_count,
            children,
        })
    }

    /// Get geo markers for map view (minimal data)
    pub async fn get_geo_markers(
        &self,
        community_types: Option<Vec<CommunityType>>,
    ) -> Result<Vec<crate::models::GeoMarker>> {
        use crate::models::GeoMarkerRow;

        let types = community_types
            .unwrap_or_else(|| vec![CommunityType::Zone, CommunityType::Neighborhood]);

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"SELECT id, name, slug, type as community_type, 
                      location_lat, location_lng, coverage_area,
                      parent_community_id
               FROM territory_dk.community_communities 
               WHERE (location_lat IS NOT NULL OR coverage_area IS NOT NULL)
                 AND type IN ("#,
        );

        // Build IN clause for types
        let mut separated = query_builder.separated(", ");
        for t in &types {
            separated.push_bind(t.clone());
        }
        query_builder.push(") ORDER BY type, name");

        let rows: Vec<GeoMarkerRow> = query_builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Get communities up to a certain depth (for hierarchy view)
    pub async fn get_hierarchy(&self, max_depth: i32) -> Result<Vec<Community>> {
        sqlx::query_as::<_, Community>(
            r#"
            WITH RECURSIVE hierarchy AS (
                -- Start with roots
                SELECT c.*, 0 as depth
                FROM territory_dk.community_communities c
                WHERE c.parent_community_id IS NULL
                
                UNION ALL
                
                -- Recurse to children up to max_depth
                SELECT c.*, h.depth + 1
                FROM territory_dk.community_communities c
                JOIN hierarchy h ON c.parent_community_id = h.id
                WHERE h.depth < $1
            )
            SELECT id, slug, name, description, type, territory_id, parent_community_id,
                   avatar_url, banner_url, member_count, created_by, created_at, updated_at,
                   location_lat, location_lng, coverage_area
            FROM hierarchy
            ORDER BY depth, name
            "#,
        )
        .bind(max_depth)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn assign_manager(
        &self,
        community_id: Uuid,
        user_id: Uuid,
        assigned_by: Uuid,
        nats: &NatsClient,
    ) -> Result<()> {
        // Upsert manager
        sqlx::query(
            r#"
            INSERT INTO territory_dk.community_communities_managers (community_id, user_id, assigned_by)
            VALUES ($1, $2, $3)
            ON CONFLICT (community_id, user_id) 
            DO UPDATE SET assigned_at = NOW(), assigned_by = EXCLUDED.assigned_by
            "#,
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

        nats.publish("community.manager_assigned", serde_json::to_vec(&event)?)
            .await
            .map_err(|e| AppError::Nats(e.to_string()))?;

        Ok(())
    }

    pub async fn revoke_manager(
        &self,
        community_id: Uuid,
        user_id: Uuid,
        revoked_by: Uuid,
        nats: &NatsClient,
    ) -> Result<()> {
        // Remove from managers table
        sqlx::query(
            r#"
            DELETE FROM territory_dk.community_communities_managers
            WHERE community_id = $1 AND user_id = $2
            "#,
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

        nats.publish("community.manager_revoked", serde_json::to_vec(&event)?)
            .await
            .map_err(|e| AppError::Nats(e.to_string()))?;

        Ok(())
    }

    pub async fn add_badge_requirement(
        &self,
        community_id: Uuid,
        badge_id: Uuid,
        context: RequirementContext,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO territory_dk.community_communities_badge_requirements (community_id, badge_id, context)
            VALUES ($1, $2, $3)
            ON CONFLICT (community_id, badge_id, context) DO NOTHING
            "#,
        )
        .bind(community_id)
        .bind(badge_id)
        .bind(context)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn remove_badge_requirement(
        &self,
        community_id: Uuid,
        badge_id: Uuid,
        context: RequirementContext,
    ) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM territory_dk.community_communities_badge_requirements
            WHERE community_id = $1 AND badge_id = $2 AND context = $3
            "#,
        )
        .bind(community_id)
        .bind(badge_id)
        .bind(context)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn get_badge_requirements(
        &self,
        community_id: Uuid,
    ) -> Result<Vec<CommunityBadgeRequirement>> {
        sqlx::query_as::<_, CommunityBadgeRequirement>(
            "SELECT * FROM territory_dk.community_communities_badge_requirements WHERE community_id = $1",
        )
        .bind(community_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    /// Get effective badge requirements for a community, including inherited requirements
    /// from parent communities (if inherit_requirements is true in settings).
    /// Returns requirements with their source (direct vs inherited) and the ancestor that provides them.
    pub async fn get_effective_requirements(
        &self,
        community_id: Uuid,
    ) -> Result<Vec<EffectiveBadgeRequirement>> {
        // Use a recursive CTE to walk up the parent chain and collect requirements
        let requirements = sqlx::query_as::<_, EffectiveBadgeRequirement>(
            r#"
            WITH RECURSIVE ancestor_chain AS (
                -- Start with the target community
                SELECT 
                    c.id,
                    c.parent_community_id,
                    c.name as community_name,
                    COALESCE(s.inherit_requirements, true) as inherit_requirements,
                    0 as depth
                FROM territory_dk.community_communities c
                LEFT JOIN territory_dk.community_communities_settings s ON c.id = s.community_id
                WHERE c.id = $1
                
                UNION ALL
                
                -- Recursively get parents while inherit_requirements is true
                SELECT 
                    parent.id,
                    parent.parent_community_id,
                    parent.name as community_name,
                    COALESCE(ps.inherit_requirements, true) as inherit_requirements,
                    ac.depth + 1
                FROM territory_dk.community_communities parent
                LEFT JOIN territory_dk.community_communities_settings ps ON parent.id = ps.community_id
                INNER JOIN ancestor_chain ac ON parent.id = ac.parent_community_id
                WHERE ac.inherit_requirements = true
            )
            SELECT 
                r.id,
                r.community_id,
                r.badge_id,
                r.context,
                r.created_at,
                b.name as badge_name,
                b.slug as badge_slug,
                ac.community_name as source_community_name,
                ac.depth > 0 as is_inherited,
                ac.depth as inheritance_depth
            FROM ancestor_chain ac
            JOIN territory_dk.community_communities_badge_requirements r ON r.community_id = ac.id
            JOIN global.registry_badge b ON r.badge_id = b.id
            ORDER BY ac.depth ASC, r.context, b.name
            "#,
        )
        .bind(community_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(requirements)
    }

    /// Check if a community should inherit requirements from its parent
    pub async fn get_inherit_requirements_setting(&self, community_id: Uuid) -> Result<bool> {
        let inherit: Option<bool> = sqlx::query_scalar(
            "SELECT inherit_requirements FROM territory_dk.community_communities_settings WHERE community_id = $1"
        )
        .bind(community_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Default to true if no setting exists
        Ok(inherit.unwrap_or(true))
    }

    /// Update the inherit_requirements setting for a community
    pub async fn set_inherit_requirements(&self, community_id: Uuid, inherit: bool) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO territory_dk.community_communities_settings (community_id, inherit_requirements)
            VALUES ($1, $2)
            ON CONFLICT (community_id) 
            DO UPDATE SET inherit_requirements = EXCLUDED.inherit_requirements
            "#,
        )
        .bind(community_id)
        .bind(inherit)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn is_community_admin(&self, community_id: Uuid, user_id: Uuid) -> Result<bool> {
        // Check if user is an effective manager
        // This is a simplified check, could be optimized to just return boolean
        let managers = self.get_effective_managers(community_id).await?;
        Ok(managers.iter().any(|m| m.user_id == user_id))
    }

    pub async fn join_community(&self, community_id: Uuid, user_id: Uuid) -> Result<()> {
        let community = self.get_community(community_id).await?;
        if community.community_type == CommunityType::Zone {
            return Err(AppError::Validation(
                "Cannot join a Zone community directly".into(),
            ));
        }

        // Check if already member
        // Insert into members
        sqlx::query(
            r#"
            INSERT INTO territory_dk.community_communities_members (community_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (community_id, user_id) DO NOTHING
            "#,
        )
        .bind(community_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn leave_community(&self, community_id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query(
            "DELETE FROM territory_dk.community_communities_members WHERE community_id = $1 AND user_id = $2",
        )
        .bind(community_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn update_community(
        &self,
        id: Uuid,
        req: UpdateCommunityRequest,
    ) -> Result<Community> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "UPDATE territory_dk.community_communities SET updated_at = NOW()",
        );

        if let Some(name) = req.name {
            query_builder.push(", name = ");
            query_builder.push_bind(name);
        }

        if let Some(description) = req.description {
            query_builder.push(", description = ");
            query_builder.push_bind(description);
        }

        if let Some(parent_id) = req.parent_community_id {
            query_builder.push(", parent_community_id = ");
            query_builder.push_bind(parent_id);
        }

        if let Some(avatar_url) = req.avatar_url {
            query_builder.push(", avatar_url = ");
            query_builder.push_bind(avatar_url);
        }

        if let Some(banner_url) = req.banner_url {
            query_builder.push(", banner_url = ");
            query_builder.push_bind(banner_url);
        }

        if let Some(lat) = req.location_lat {
            query_builder.push(", location_lat = ");
            query_builder.push_bind(lat);
        }

        if let Some(lng) = req.location_lng {
            query_builder.push(", location_lng = ");
            query_builder.push_bind(lng);
        }

        if let Some(coverage) = req.coverage_area {
            query_builder.push(", coverage_area = ");
            query_builder.push_bind(sqlx::types::Json(coverage));
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);
        query_builder.push(" RETURNING *");

        let community = query_builder
            .build_query_as::<Community>()
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(community)
    }

    pub async fn get_effective_managers(
        &self,
        community_id: Uuid,
    ) -> Result<Vec<EffectiveManager>> {
        // 1. Get community hierarchy (recursive CTE)
        // 2. Find admins at each level
        // 3. Find territory/platform managers

        let managers = sqlx::query_as::<_, EffectiveManager>(
            r#"
            WITH RECURSIVE community_tree AS (
                -- Base case: the community itself
                SELECT id, parent_community_id, name, 0 as distance
                FROM territory_dk.community_communities
                WHERE id = $1
                
                UNION ALL
                
                -- Recursive case: parents
                SELECT c.id, c.parent_community_id, c.name, ct.distance + 1
                FROM territory_dk.community_communities c
                INNER JOIN community_tree ct ON c.id = ct.parent_community_id
            ),
            community_admins AS (
                -- Admins from the hierarchy who ALSO have the community-manager badge
                SELECT 
                    u.id as user_id,
                    u.username,
                    p.avatar_url,
                    'admin' as role,
                    CASE 
                        WHEN ct.distance = 0 THEN 'direct'
                        ELSE CONCAT('parent: ', ct.name)
                    END as source,
                    ct.distance
                FROM community_tree ct
                JOIN territory_dk.community_communities_managers cm ON ct.id = cm.community_id
                JOIN territory_dk.auth_users_core u ON cm.user_id = u.id
                LEFT JOIN territory_dk.user_users_profiles p ON u.id = p.user_id
                -- Check for community-manager badge
                JOIN territory_dk.badge_users_badges ub ON u.id = ub.user_id
                JOIN global.registry_badge b ON ub.badge_id = b.id
                WHERE b.slug = 'community-manager'
            ),
            global_admins AS (
                -- Platform and Territory Managers
                SELECT 
                    u.id as user_id,
                    u.username,
                    p.avatar_url,
                    'admin' as role,
                    CASE 
                        WHEN b.slug = 'platform-manager' THEN 'platform'
                        ELSE 'territory'
                    END as source,
                    CASE 
                        WHEN b.slug = 'platform-manager' THEN 2000
                        ELSE 1000
                    END as distance
                FROM territory_dk.badge_users_badges ub
                JOIN global.registry_badge b ON ub.badge_id = b.id
                JOIN territory_dk.auth_users_core u ON ub.user_id = u.id
                LEFT JOIN territory_dk.user_users_profiles p ON u.id = p.user_id
                -- For territory managers, they must ALSO be assigned in territory_territories_managers
                LEFT JOIN territory_dk.territory_territories_managers tm ON u.id = tm.user_id
                WHERE 
                    (b.slug = 'platform-manager') OR
                    (b.slug = 'territory-manager' AND tm.id IS NOT NULL)
            ),
            all_managers AS (
                SELECT * FROM community_admins
                UNION ALL
                SELECT * FROM global_admins
            )
            -- Deduplicate by user_id, keeping the one with the smallest distance
            SELECT DISTINCT ON (user_id) *
            FROM all_managers
            ORDER BY user_id, distance ASC
            "#,
        )
        .bind(community_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(managers)
    }

    pub async fn get_managed_community_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>> {
        // 1. Check for Platform Manager badge
        let is_platform_manager = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM territory_dk.badge_users_badges ub
                JOIN global.registry_badge b ON ub.badge_id = b.id
                WHERE ub.user_id = $1 AND b.slug = 'platform-manager' AND b.is_active = true 
                AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
            )
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        if is_platform_manager {
            // Return all community IDs
            return sqlx::query_scalar("SELECT id FROM territory_dk.community_communities")
                .fetch_all(&self.pool)
                .await
                .map_err(AppError::Database);
        }

        // 2. Check for Territory Manager badge
        let is_territory_manager = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM territory_dk.badge_users_badges ub
                JOIN global.registry_badge b ON ub.badge_id = b.id
                JOIN territory_dk.territory_territories_managers tm ON ub.user_id = tm.user_id
                WHERE ub.user_id = $1 AND b.slug = 'territory-manager' AND b.is_active = true
                AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
            )
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        if is_territory_manager {
            // Return all community IDs (in this territory/pod)
            return sqlx::query_scalar("SELECT id FROM territory_dk.community_communities")
                .fetch_all(&self.pool)
                .await
                .map_err(AppError::Database);
        }

        // 3. Check for Community Manager badge (required for inheritance)
        let has_community_manager_badge = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM territory_dk.badge_users_badges ub
                JOIN global.registry_badge b ON ub.badge_id = b.id
                WHERE ub.user_id = $1 AND b.slug = 'community-manager' AND b.is_active = true
                AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
            )
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // 4. Get directly assigned communities and recurse down if they have the badge
        let query = if has_community_manager_badge {
            r#"
            WITH RECURSIVE managed_trees AS (
                -- Base case: directly assigned
                SELECT c.id
                FROM territory_dk.community_communities c
                JOIN territory_dk.community_communities_managers cm ON c.id = cm.community_id
                WHERE cm.user_id = $1
                
                UNION
                
                -- Recursive case: children
                SELECT c.id
                FROM territory_dk.community_communities c
                JOIN managed_trees mt ON c.parent_community_id = mt.id
            )
            SELECT id FROM managed_trees
            "#
        } else {
            r#"
            SELECT community_id as id
            FROM territory_dk.community_communities_managers
            WHERE user_id = $1
            "#
        };

        sqlx::query_scalar(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    pub async fn get_managed_communities(&self, user_id: Uuid) -> Result<Vec<Uuid>> {
        // 1. Check if global admin (Platform Manager or Territory Manager)
        let is_global_admin = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM territory_dk.badge_users_badges ub
                JOIN global.registry_badge b ON ub.badge_id = b.id
                LEFT JOIN territory_dk.territory_territories_managers tm ON ub.user_id = tm.user_id
                WHERE ub.user_id = $1
                  AND (
                      b.slug = 'platform-manager'
                      OR (b.slug = 'territory-manager' AND tm.id IS NOT NULL)
                  )
                  AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
            )
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        if is_global_admin {
            // Return ALL community IDs
            let all_ids =
                sqlx::query_scalar::<_, Uuid>("SELECT id FROM territory_dk.community_communities")
                    .fetch_all(&self.pool)
                    .await
                    .map_err(AppError::Database)?;
            return Ok(all_ids);
        }

        // 2. Get directly managed communities (where user is in managers table AND has badge)
        // AND their children recursively
        let managed_ids = sqlx::query_scalar::<_, Uuid>(
            r#"
            WITH RECURSIVE managed_tree AS (
                -- Base case: directly managed communities
                SELECT c.id
                FROM territory_dk.community_communities c
                JOIN territory_dk.community_communities_managers cm ON c.id = cm.community_id
                JOIN territory_dk.badge_users_badges ub ON cm.user_id = ub.user_id
                JOIN global.registry_badge b ON ub.badge_id = b.id
                WHERE cm.user_id = $1
                  AND b.slug = 'community-manager'
                  AND (ub.expires_at IS NULL OR ub.expires_at > NOW())
                
                UNION
                
                -- Recursive case: children of managed communities
                SELECT c.id
                FROM territory_dk.community_communities c
                INNER JOIN managed_tree mt ON c.parent_community_id = mt.id
            )
            SELECT id FROM managed_tree
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(managed_ids)
    }

    pub async fn is_member(&self, community_id: Uuid, user_id: Uuid) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM territory_dk.community_communities_members WHERE community_id = $1 AND user_id = $2",
        )
        .bind(community_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(count > 0)
    }

    /// Get a summary of child groups (Guilds and Study Groups) under a community
    pub async fn get_group_summary(&self, community_id: Uuid) -> Result<GroupSummary> {
        // First verify the community exists
        self.get_community(community_id).await?;

        // Get direct child groups (Guild and StudyGroup types only)
        let direct_children = sqlx::query_as::<_, Community>(
            r#"
            SELECT * FROM territory_dk.community_communities 
            WHERE parent_community_id = $1 
              AND type IN ('guild', 'study_group')
            ORDER BY type, name
            "#,
        )
        .bind(community_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Count direct guilds and study groups
        let guild_count = direct_children
            .iter()
            .filter(|c| c.community_type == CommunityType::Guild)
            .count() as i32;
        let study_group_count = direct_children
            .iter()
            .filter(|c| c.community_type == CommunityType::StudyGroup)
            .count() as i32;

        // Get total counts including nested (using recursive CTE)
        #[derive(sqlx::FromRow)]
        struct Counts {
            total_guilds: i64,
            total_study_groups: i64,
            total_members: i64,
        }

        let counts = sqlx::query_as::<_, Counts>(
            r#"
            WITH RECURSIVE descendant_groups AS (
                -- Direct children that are groups
                SELECT id, type, member_count
                FROM territory_dk.community_communities
                WHERE parent_community_id = $1
                  AND type IN ('guild', 'study_group')
                
                UNION ALL
                
                -- Recursive: children of groups (only groups)
                SELECT c.id, c.type, c.member_count
                FROM territory_dk.community_communities c
                INNER JOIN descendant_groups dg ON c.parent_community_id = dg.id
                WHERE c.type IN ('guild', 'study_group')
            )
            SELECT 
                COUNT(*) FILTER (WHERE type = 'guild') as total_guilds,
                COUNT(*) FILTER (WHERE type = 'study_group') as total_study_groups,
                COALESCE(SUM(member_count), 0) as total_members
            FROM descendant_groups
            "#,
        )
        .bind(community_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Get badge requirements for direct children
        let mut children_with_badges: Vec<GroupChild> = Vec::new();
        let mut first_badge_name: Option<String> = None;
        let mut first_badge_id: Option<Uuid> = None;
        let mut has_any_badge_requirement = false;

        for child in &direct_children {
            // Check if this child has badge requirements
            let requirements = self.get_badge_requirements(child.id).await?;
            let has_badge = !requirements.is_empty();

            if has_badge && first_badge_name.is_none() {
                // Get the badge name for the first requirement
                if let Some(req) = requirements.first() {
                    first_badge_id = Some(req.badge_id);
                    // Try to get badge name from global registry
                    let badge_name: Option<String> =
                        sqlx::query_scalar("SELECT name FROM global.registry_badge WHERE id = $1")
                            .bind(req.badge_id)
                            .fetch_optional(&self.pool)
                            .await
                            .map_err(AppError::Database)?;
                    first_badge_name = badge_name;
                }
                has_any_badge_requirement = true;
            }

            let badge_name_for_child = if has_badge {
                if let Some(req) = requirements.first() {
                    sqlx::query_scalar::<_, String>(
                        "SELECT name FROM global.registry_badge WHERE id = $1",
                    )
                    .bind(req.badge_id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(AppError::Database)?
                } else {
                    None
                }
            } else {
                None
            };

            children_with_badges.push(GroupChild {
                id: child.id,
                name: child.name.clone(),
                slug: child.slug.clone(),
                community_type: child.community_type.clone(),
                member_count: child.member_count,
                has_badge_requirement: has_badge,
                badge_name: badge_name_for_child,
            });
        }

        Ok(GroupSummary {
            community_id,
            guild_count,
            study_group_count,
            total_guild_count: counts.total_guilds as i32,
            total_study_group_count: counts.total_study_groups as i32,
            total_members: counts.total_members as i32,
            has_badge_requirement: has_any_badge_requirement,
            badge_name: first_badge_name,
            badge_id: first_badge_id,
            children: children_with_badges,
        })
    }
}
