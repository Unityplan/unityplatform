use crate::{models::*, ApiResponse};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health(pool: web::Data<PgPool>) -> HttpResponse {
    let db_status = match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "badge-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        dependencies: crate::models::DependencyStatus {
            database: db_status.to_string(),
        },
    };

    HttpResponse::Ok().json(ApiResponse::success(response))
}
