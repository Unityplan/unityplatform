use actix_web::{web, HttpResponse};
use shared_lib::{Database, MetricsCollector};

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "territory-service",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Ready check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/ready",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready"),
        (status = 503, description = "Service is not ready")
    )
)]
pub async fn ready_check(db: web::Data<Database>) -> HttpResponse {
    // Check database connectivity
    match sqlx::query("SELECT 1").fetch_one(db.pool()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ready",
            "service": "territory-service",
            "version": env!("CARGO_PKG_VERSION"),
        })),
        Err(_) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "service": "territory-service",
            "version": env!("CARGO_PKG_VERSION"),
            "reason": "database_unavailable"
        })),
    }
}

/// Metrics endpoint (Prometheus format)
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    tag = "health",
    responses(
        (status = 200, description = "Prometheus metrics", content_type = "text/plain")
    )
)]
pub async fn metrics(db: web::Data<Database>, collector: web::Data<MetricsCollector>) -> HttpResponse {
    // Get database pool stats
    let pool_size = db.pool().size();
    let pool_idle = db.pool().num_idle();

    let metrics_text = collector.generate_prometheus_metrics(Some(pool_size), Some(pool_idle));

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_text)
}
