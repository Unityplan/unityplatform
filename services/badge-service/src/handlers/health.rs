use actix_web::{web, HttpResponse};
use shared_lib::{Database, MetricsCollector};

/// Health check endpoint (no authentication required)
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = serde_json::Value,
            example = json!({
                "service": "badge-service",
                "status": "healthy",
                "version": "0.1.0-alpha.1"
            })
        )
    )
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "service": "badge-service",
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Readiness check endpoint - verifies database connectivity
#[utoipa::path(
    get,
    path = "/api/v1/ready",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready", body = serde_json::Value,
            example = json!({
                "service": "badge-service",
                "status": "ready",
                "version": "0.1.0-alpha.1"
            })
        ),
        (status = 503, description = "Service is not ready", body = serde_json::Value,
            example = json!({
                "service": "badge-service",
                "status": "not_ready",
                "reason": "database_unavailable",
                "version": "0.1.0-alpha.1"
            })
        )
    )
)]
pub async fn ready_check(db: web::Data<Database>) -> HttpResponse {
    // Check database connectivity
    let db_ok = sqlx::query("SELECT 1").fetch_one(db.pool()).await.is_ok();

    if db_ok {
        HttpResponse::Ok().json(serde_json::json!({
            "service": "badge-service",
            "status": "ready",
            "version": env!("CARGO_PKG_VERSION"),
        }))
    } else {
        HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "service": "badge-service",
            "status": "not_ready",
            "reason": "database_unavailable",
            "version": env!("CARGO_PKG_VERSION"),
        }))
    }
}

/// Metrics endpoint - Prometheus-compatible metrics
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    tag = "health",
    responses(
        (status = 200, description = "Prometheus-format metrics", content_type = "text/plain")
    )
)]
pub async fn metrics(
    db: web::Data<Database>,
    collector: web::Data<MetricsCollector>,
) -> HttpResponse {
    // Get database pool metrics
    let pool_size = db.pool().size();
    let pool_idle = db.pool().num_idle();

    // Generate comprehensive metrics using the collector
    let metrics_text = collector.generate_prometheus_metrics(Some(pool_size), Some(pool_idle));

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_text)
}
