use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::ServiceError,
    models::{
        CreateProfileLinkRequest, ProfileLink, ReorderLinksRequest, UpdateProfileLinkRequest,
    },
    response::ApiResponse,
};

/// Get all profile links for a user
#[utoipa::path(
    get,
    path = "/v1/profiles/{id}/links",
    tag = "profile-links",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Links retrieved successfully", body = Vec<ProfileLink>),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_profile_links(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Verify user exists
    let user_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM territory_dk.users WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !user_exists {
        return Err(ServiceError::NotFound("User not found".to_string()));
    }

    let links = sqlx::query_as::<_, ProfileLink>(
        "SELECT id, user_id, label, url, icon, display_order, is_visible, created_at, updated_at
         FROM territory_dk.users_profile_links 
         WHERE user_id = $1 
         ORDER BY display_order ASC",
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(links)))
}

/// Create a new profile link
#[utoipa::path(
    post,
    path = "/v1/profiles/{id}/links",
    tag = "profile-links",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = CreateProfileLinkRequest,
    responses(
        (status = 201, description = "Link created successfully", body = ProfileLink),
        (status = 400, description = "Invalid input or limit exceeded"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_profile_link(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    req: web::Json<CreateProfileLinkRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Validate request
    req.validate()?;

    // Check link count limit (max 10 links per user)
    let link_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM territory_dk.users_profile_links WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await?;

    if link_count >= 10 {
        return Err(ServiceError::BadRequest(
            "Maximum of 10 profile links allowed".to_string(),
        ));
    }

    // Get next display_order
    let next_order = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT MAX(display_order) + 1 FROM territory_dk.users_profile_links WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await?
    .unwrap_or(0);

    let link = sqlx::query_as::<_, ProfileLink>(
        "INSERT INTO territory_dk.users_profile_links 
         (user_id, label, url, icon, display_order, is_visible)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, user_id, label, url, icon, display_order, is_visible, created_at, updated_at"
    )
    .bind(user_id)
    .bind(&req.label)
    .bind(&req.url)
    .bind(&req.icon)
    .bind(next_order)
    .bind(req.is_visible.unwrap_or(true))
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(link)))
}

/// Update a profile link
#[utoipa::path(
    put,
    path = "/v1/profiles/{id}/links/{link_id}",
    tag = "profile-links",
    params(
        ("id" = Uuid, Path, description = "User ID"),
        ("link_id" = Uuid, Path, description = "Link ID")
    ),
    request_body = UpdateProfileLinkRequest,
    responses(
        (status = 200, description = "Link updated successfully", body = ProfileLink),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Link not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_profile_link(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    req: web::Json<UpdateProfileLinkRequest>,
) -> Result<HttpResponse, ServiceError> {
    let (user_id, link_id) = path.into_inner();

    // Validate request
    req.validate()?;

    // Build dynamic update query
    let mut query = String::from("UPDATE territory_dk.users_profile_links SET updated_at = NOW()");
    let mut param_count = 2; // user_id and link_id are $1 and $2

    if req.label.is_some() {
        param_count += 1;
        query.push_str(&format!(", label = ${}", param_count));
    }
    if req.url.is_some() {
        param_count += 1;
        query.push_str(&format!(", url = ${}", param_count));
    }
    if req.icon.is_some() {
        param_count += 1;
        query.push_str(&format!(", icon = ${}", param_count));
    }
    if req.is_visible.is_some() {
        param_count += 1;
        query.push_str(&format!(", is_visible = ${}", param_count));
    }

    query.push_str(" WHERE user_id = $1 AND id = $2 ");
    query.push_str("RETURNING id, user_id, label, url, icon, display_order, is_visible, created_at, updated_at");

    let mut query_builder = sqlx::query_as::<_, ProfileLink>(&query)
        .bind(user_id)
        .bind(link_id);

    if let Some(ref label) = req.label {
        query_builder = query_builder.bind(label);
    }
    if let Some(ref url) = req.url {
        query_builder = query_builder.bind(url);
    }
    if let Some(ref icon) = req.icon {
        query_builder = query_builder.bind(icon);
    }
    if let Some(is_visible) = req.is_visible {
        query_builder = query_builder.bind(is_visible);
    }

    let link = query_builder
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(ServiceError::NotFound("Link not found".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(link)))
}

/// Delete a profile link
#[utoipa::path(
    delete,
    path = "/v1/profiles/{id}/links/{link_id}",
    tag = "profile-links",
    params(
        ("id" = Uuid, Path, description = "User ID"),
        ("link_id" = Uuid, Path, description = "Link ID")
    ),
    responses(
        (status = 204, description = "Link deleted successfully"),
        (status = 404, description = "Link not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_profile_link(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ServiceError> {
    let (user_id, link_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM territory_dk.users_profile_links 
         WHERE user_id = $1 AND id = $2",
    )
    .bind(user_id)
    .bind(link_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ServiceError::NotFound("Link not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}

/// Reorder profile links
#[utoipa::path(
    patch,
    path = "/v1/profiles/{id}/links/reorder",
    tag = "profile-links",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = ReorderLinksRequest,
    responses(
        (status = 200, description = "Links reordered successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn reorder_profile_links(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    req: web::Json<ReorderLinksRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    // Validate request
    req.validate()?;

    // Update display_order for each link
    let mut tx = pool.begin().await?;

    for (index, link_id) in req.link_ids.iter().enumerate() {
        sqlx::query(
            "UPDATE territory_dk.users_profile_links 
             SET display_order = $1, updated_at = NOW()
             WHERE user_id = $2 AND id = $3",
        )
        .bind(index as i32)
        .bind(user_id)
        .bind(link_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success("Links reordered successfully")))
}
