use actix_web::{web, HttpRequest, HttpResponse};
use shared_lib::{AuthUser, Result, ValidatedQuery};
use std::sync::Arc;
use tracing::info;

use crate::models::FaviconQuery;
use crate::services::FaviconService;

/// Fetch and cache a website's favicon
#[utoipa::path(
    get,
    path = "/api/v1/utility/favicon",
    params(
        ("url" = String, Query, description = "Website URL"),
        ("size" = Option<u32>, Query, description = "Icon size (16, 32, 64, or 128)")
    ),
    responses(
        (status = 200, description = "Favicon image (PNG)", content_type = "image/png"),
        (status = 400, description = "Invalid URL or size parameter"),
        (status = 401, description = "Unauthorized"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 503, description = "Service unavailable")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Utilities"
)]
pub async fn get_favicon(
    _user: AuthUser,
    query: ValidatedQuery<FaviconQuery>,
    favicon_service: web::Data<Arc<FaviconService>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let query = query.into_inner();

    info!(
        user_id = %_user.id,
        url = %query.url,
        size = query.size,
        "Fetching favicon"
    );

    // Get favicon (from cache or fetch)
    let (image_data, etag, cache_hit) =
        favicon_service
            .get_favicon(&query.url, query.size)
            .await
            .map_err(|e| shared_lib::AppError::Internal(e.to_string()))?;

    // Check If-None-Match header for 304 Not Modified
    if let Some(if_none_match) = req.headers().get("If-None-Match") {
        if if_none_match.to_str().unwrap_or("") == etag {
            return Ok(HttpResponse::NotModified().finish());
        }
    }

    // Return image with caching headers
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("Cache-Control", "public, max-age=604800"))
        .insert_header(("ETag", etag))
        .insert_header(("X-Cache-Status", if cache_hit { "HIT" } else { "MISS" }))
        .body(image_data))
}
