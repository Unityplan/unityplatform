use crate::models::{
    AddRequirementRequest, AssignManagerRequest, Community, CommunityBadgeRequirement,
    CommunityFilter, CommunityType, CreateCommunityRequest, EffectiveBadgeRequirement,
    EffectiveManager, GroupSummary, RemoveRequirementQuery, RequirementContext,
    UpdateCommunityRequest,
};
use crate::services::CommunityService;
use actix_web::{web, HttpResponse};
use shared_lib::{AppError, AuthUser, NatsClient, Result, ValidatedJson};
use uuid::Uuid;

/// Create a new community
#[utoipa::path(
    post,
    path = "/api/v1/communities",
    tag = "communities",
    request_body = CreateCommunityRequest,
    responses(
        (status = 201, description = "Community created successfully", body = Community),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Community slug already exists")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_community(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    body: ValidatedJson<CreateCommunityRequest>,
) -> Result<HttpResponse> {
    let community = service
        .create_community(body.into_inner(), auth_user.id)
        .await?;
    Ok(HttpResponse::Created().json(community))
}

/// Get community details
#[utoipa::path(
    get,
    path = "/api/v1/communities/{id}",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    responses(
        (status = 200, description = "Community details", body = Community),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_community(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let community = service.get_community(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(community))
}

/// List communities with filtering
#[utoipa::path(
    get,
    path = "/api/v1/communities",
    tag = "communities",
    params(
        ("community_type" = Option<CommunityType>, Query, description = "Filter by community type"),
        ("parent_id" = Option<Uuid>, Query, description = "Filter by parent community ID"),
        ("territory_id" = Option<String>, Query, description = "Filter by territory ID"),
        ("search" = Option<String>, Query, description = "Search by name or slug")
    ),
    responses(
        (status = 200, description = "List of communities", body = Vec<Community>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_communities(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    query: web::Query<CommunityFilter>,
) -> Result<HttpResponse> {
    let communities = service.list_communities(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(communities))
}

/// Assign a manager to a community
#[utoipa::path(
    post,
    path = "/api/v1/communities/{id}/manager",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    request_body = AssignManagerRequest,
    responses(
        (status = 200, description = "Manager assigned successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User is not an admin of this community"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn assign_manager(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    path: web::Path<Uuid>,
    nats: web::Data<NatsClient>,
    body: ValidatedJson<AssignManagerRequest>,
) -> Result<HttpResponse> {
    service
        .assign_manager(path.into_inner(), body.user_id, auth_user.id, &nats)
        .await?;
    Ok(HttpResponse::Ok().finish())
}

/// Revoke a manager from a community
#[utoipa::path(
    delete,
    path = "/api/v1/communities/{id}/manager/{user_id}",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID"),
        ("user_id" = Uuid, Path, description = "User ID to revoke")
    ),
    responses(
        (status = 200, description = "Manager revoked successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User is not an admin of this community"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn revoke_manager(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    path: web::Path<(Uuid, Uuid)>,
    nats: web::Data<NatsClient>,
) -> Result<HttpResponse> {
    let (community_id, user_id) = path.into_inner();
    service
        .revoke_manager(community_id, user_id, auth_user.id, &nats)
        .await?;
    Ok(HttpResponse::Ok().finish())
}

/// Add a badge requirement to a community
#[utoipa::path(
    post,
    path = "/api/v1/communities/{id}/requirements",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    request_body = AddRequirementRequest,
    responses(
        (status = 200, description = "Requirement added successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User is not an admin"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn add_requirement(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    path: web::Path<Uuid>,
    body: ValidatedJson<AddRequirementRequest>,
) -> Result<HttpResponse> {
    let community_id = path.into_inner();

    if !service
        .is_community_admin(community_id, auth_user.id)
        .await?
    {
        return Err(shared_lib::AppError::Forbidden(
            "Only admins can manage requirements".into(),
        ));
    }

    service
        .add_badge_requirement(community_id, body.badge_id, body.context.clone())
        .await?;
    Ok(HttpResponse::Ok().finish())
}

/// Remove a badge requirement from a community
#[utoipa::path(
    delete,
    path = "/api/v1/communities/{id}/requirements/{badge_id}",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID"),
        ("badge_id" = Uuid, Path, description = "Badge ID"),
        ("context" = RequirementContext, Query, description = "Requirement context")
    ),
    responses(
        (status = 200, description = "Requirement removed successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User is not an admin"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn remove_requirement(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<RemoveRequirementQuery>,
) -> Result<HttpResponse> {
    let (community_id, badge_id) = path.into_inner();

    if !service
        .is_community_admin(community_id, auth_user.id)
        .await?
    {
        return Err(shared_lib::AppError::Forbidden(
            "Only admins can manage requirements".into(),
        ));
    }

    service
        .remove_badge_requirement(community_id, badge_id, query.context.clone())
        .await?;
    Ok(HttpResponse::Ok().finish())
}

/// List badge requirements for a community
#[utoipa::path(
    get,
    path = "/api/v1/communities/{id}/requirements",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    responses(
        (status = 200, description = "List of requirements", body = Vec<CommunityBadgeRequirement>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_requirements(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let requirements = service.get_badge_requirements(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(requirements))
}

/// Get effective badge requirements for a community (including inherited)
///
/// Returns all badge requirements that apply to this community, including
/// requirements inherited from parent communities. Each requirement includes
/// information about whether it's direct or inherited and from which ancestor.
#[utoipa::path(
    get,
    path = "/api/v1/communities/{id}/effective-requirements",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    responses(
        (status = 200, description = "List of effective requirements", body = Vec<EffectiveBadgeRequirement>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_effective_requirements(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let requirements = service
        .get_effective_requirements(path.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(requirements))
}

/// Update a community
#[utoipa::path(
    put,
    path = "/api/v1/communities/{id}",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    request_body = UpdateCommunityRequest,
    responses(
        (status = 200, description = "Community updated successfully", body = Community),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User is not an admin"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_community(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    path: web::Path<Uuid>,
    body: ValidatedJson<UpdateCommunityRequest>,
) -> Result<HttpResponse> {
    let community_id = path.into_inner();

    if !service
        .is_community_admin(community_id, auth_user.id)
        .await?
    {
        return Err(shared_lib::AppError::Forbidden(
            "Only admins can update community details".into(),
        ));
    }

    let community = service
        .update_community(community_id, body.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(community))
}

/// Get effective managers for a community
#[utoipa::path(
    get,
    path = "/api/v1/communities/{id}/managers",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    responses(
        (status = 200, description = "List of effective managers", body = Vec<EffectiveManager>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_effective_managers(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let community_id = path.into_inner();
    let managers = service.get_effective_managers(community_id).await?;
    Ok(HttpResponse::Ok().json(managers))
}

/// Get IDs of communities managed by the current user
#[utoipa::path(
    get,
    path = "/api/v1/communities/managed-by-me",
    tag = "communities",
    responses(
        (status = 200, description = "List of managed community IDs", body = Vec<Uuid>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_managed_communities(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
) -> Result<HttpResponse> {
    let community_ids = service.get_managed_communities(auth_user.id).await?;
    Ok(HttpResponse::Ok().json(community_ids))
}

/// Join a community
#[utoipa::path(
    post,
    path = "/api/v1/communities/{id}/join",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    responses(
        (status = 200, description = "Joined community successfully"),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Missing required badges"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn join_community(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let community_id = path.into_inner();

    // Get effective requirements (including inherited from parent communities)
    let requirements = service.get_effective_requirements(community_id).await?;

    // Filter to required badges (excluding CoC which everyone should have)
    let required_slugs: Vec<&str> = requirements
        .iter()
        .filter(|r| r.badge_slug != "code-of-conduct")
        .map(|r| r.badge_slug.as_str())
        .collect();

    // Validate user has all required badges
    if !required_slugs.is_empty() && !auth_user.has_all_badges(&required_slugs) {
        let missing: Vec<&str> = required_slugs
            .iter()
            .filter(|slug| !auth_user.has_badge(slug))
            .copied()
            .collect();

        return Err(AppError::Forbidden(format!(
            "Missing required badges: {}",
            missing.join(", ")
        )));
    }

    service.join_community(community_id, auth_user.id).await?;
    Ok(HttpResponse::Ok().finish())
}

/// Leave a community
#[utoipa::path(
    post,
    path = "/api/v1/communities/{id}/leave",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    responses(
        (status = 200, description = "Left community successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn leave_community(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    service
        .leave_community(path.into_inner(), auth_user.id)
        .await?;
    Ok(HttpResponse::Ok().finish())
}

/// Get membership status
#[utoipa::path(
    get,
    path = "/api/v1/communities/{id}/membership",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    responses(
        (status = 200, description = "Membership status", body = serde_json::Value,
            example = json!({ "is_member": true })
        ),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_membership(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let is_member = service.is_member(path.into_inner(), auth_user.id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "is_member": is_member })))
}

/// Get group summary for a community
///
/// Returns a summary of child groups (Guilds and Study Groups) under a community,
/// including counts, member totals, and badge requirements.
#[utoipa::path(
    get,
    path = "/api/v1/communities/{id}/group-summary",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID")
    ),
    responses(
        (status = 200, description = "Group summary", body = GroupSummary),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_group_summary(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let summary = service.get_group_summary(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(summary))
}

/// Query params for children endpoint
#[derive(Debug, serde::Deserialize)]
pub struct ChildrenQuery {
    pub limit: Option<i64>,
}

/// Query params for context endpoint
#[derive(Debug, serde::Deserialize)]
pub struct ContextQuery {
    pub children_limit: Option<i64>,
}

/// Query params for hierarchy endpoint
#[derive(Debug, serde::Deserialize)]
pub struct HierarchyQuery {
    pub max_depth: Option<i32>,
}

/// Query params for geo markers endpoint
#[derive(Debug, serde::Deserialize)]
pub struct GeoMarkersQuery {
    /// Comma-separated community types (default: zone,neighborhood)
    pub types: Option<String>,
}

/// List communities with pagination metadata (for infinite scroll)
#[utoipa::path(
    get,
    path = "/api/v1/communities/paginated",
    tag = "communities",
    params(
        ("community_type" = Option<CommunityType>, Query, description = "Filter by community type"),
        ("parent_id" = Option<Uuid>, Query, description = "Filter by parent community ID"),
        ("territory_id" = Option<String>, Query, description = "Filter by territory ID"),
        ("search" = Option<String>, Query, description = "Search by name or slug"),
        ("limit" = Option<i64>, Query, description = "Number of items per page (default: 50, max: 500)"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination")
    ),
    responses(
        (status = 200, description = "Paginated list of communities", body = crate::models::PaginatedCommunities),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_communities_paginated(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    query: web::Query<CommunityFilter>,
) -> Result<HttpResponse> {
    let result = service
        .list_communities_paginated(query.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get root communities (communities with no parent)
#[utoipa::path(
    get,
    path = "/api/v1/communities/roots",
    tag = "communities",
    responses(
        (status = 200, description = "List of root communities", body = Vec<Community>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_root_communities(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
) -> Result<HttpResponse> {
    let communities = service.get_root_communities().await?;
    Ok(HttpResponse::Ok().json(communities))
}

/// Get direct children of a community
#[utoipa::path(
    get,
    path = "/api/v1/communities/{id}/children",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID"),
        ("limit" = Option<i64>, Query, description = "Maximum number of children to return (default: 100, max: 500)")
    ),
    responses(
        (status = 200, description = "List of child communities", body = Vec<Community>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_children(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    path: web::Path<Uuid>,
    query: web::Query<ChildrenQuery>,
) -> Result<HttpResponse> {
    let children = service.get_children(path.into_inner(), query.limit).await?;
    Ok(HttpResponse::Ok().json(children))
}

/// Get community with context (ancestors and children) for flow view
#[utoipa::path(
    get,
    path = "/api/v1/communities/{id}/context",
    tag = "communities",
    params(
        ("id" = Uuid, Path, description = "Community ID"),
        ("children_limit" = Option<i64>, Query, description = "Maximum number of children to return (default: 100)")
    ),
    responses(
        (status = 200, description = "Community with context", body = crate::models::CommunityContext),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Community not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_community_context(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    path: web::Path<Uuid>,
    query: web::Query<ContextQuery>,
) -> Result<HttpResponse> {
    let context = service
        .get_community_context(path.into_inner(), query.children_limit)
        .await?;
    Ok(HttpResponse::Ok().json(context))
}

/// Get geo markers for map view (minimal data for all geographic communities)
#[utoipa::path(
    get,
    path = "/api/v1/communities/geo-markers",
    tag = "communities",
    params(
        ("types" = Option<String>, Query, description = "Comma-separated community types (default: zone,neighborhood)")
    ),
    responses(
        (status = 200, description = "List of geo markers", body = Vec<crate::models::GeoMarker>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_geo_markers(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    query: web::Query<GeoMarkersQuery>,
) -> Result<HttpResponse> {
    let types = query.types.as_ref().map(|s| {
        s.split(',')
            .filter_map(|t| match t.trim().to_lowercase().as_str() {
                "zone" => Some(CommunityType::Zone),
                "neighborhood" => Some(CommunityType::Neighborhood),
                "guild" => Some(CommunityType::Guild),
                "study_group" => Some(CommunityType::StudyGroup),
                "group" => Some(CommunityType::Group),
                _ => None,
            })
            .collect()
    });
    let markers = service.get_geo_markers(types).await?;
    Ok(HttpResponse::Ok().json(markers))
}

/// Get community hierarchy up to a certain depth
#[utoipa::path(
    get,
    path = "/api/v1/communities/hierarchy",
    tag = "communities",
    params(
        ("max_depth" = Option<i32>, Query, description = "Maximum depth to fetch (default: 2)")
    ),
    responses(
        (status = 200, description = "List of communities in hierarchy", body = Vec<Community>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_hierarchy(
    service: web::Data<CommunityService>,
    _auth_user: AuthUser,
    query: web::Query<HierarchyQuery>,
) -> Result<HttpResponse> {
    let max_depth = query.max_depth.unwrap_or(2).min(10).max(0);
    let communities = service.get_hierarchy(max_depth).await?;
    Ok(HttpResponse::Ok().json(communities))
}
