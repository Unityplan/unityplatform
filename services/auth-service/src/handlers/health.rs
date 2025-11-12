use crate::models::HealthResponse;
use crate::ApiResponse;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

/// Health check endpoint
/// GET /health
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy")
    )
)]
pub async fn health(pool: web::Data<PgPool>) -> HttpResponse {
    let mut dependencies = std::collections::HashMap::new();

    // Check database connection
    let db_healthy = sqlx::query("SELECT 1")
        .execute(pool.get_ref())
        .await
        .is_ok();

    dependencies.insert(
        "database".to_string(),
        if db_healthy {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
    );

    let all_healthy = db_healthy;

    let response = HealthResponse::healthy(dependencies);

    if all_healthy {
        HttpResponse::Ok().json(ApiResponse::success(response))
    } else {
        HttpResponse::ServiceUnavailable().json(ApiResponse::success(response))
    }
}
