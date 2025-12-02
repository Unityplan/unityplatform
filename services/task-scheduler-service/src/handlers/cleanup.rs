use actix_web::{web, HttpResponse};
use shared_lib::{AuthUser, Result, ValidatedJson};
use std::time::Instant;

use crate::models::{ManualCleanupResponse, TriggerCleanupRequest};
use crate::services::CleanupService;

/// Manually trigger user cleanup job (admin only)
///
/// This endpoint allows administrators to manually trigger the cleanup job
/// for testing or immediate execution without waiting for the scheduled cron.
///
/// # Security
/// - Requires JWT authentication
/// - Requires `task-admin` badge
///
/// # Arguments
/// * `user` - Authenticated user from JWT
/// * `cleanup_service` - Cleanup service instance
/// * `body` - Request with force and dry_run flags
#[utoipa::path(
    post,
    path = "/api/v1/cleanup/users",
    tag = "cleanup",
    request_body = TriggerCleanupRequest,
    responses(
        (status = 200, description = "Cleanup executed successfully", body = ManualCleanupResponse),
        (status = 401, description = "Unauthorized - missing or invalid JWT"),
        (status = 403, description = "Forbidden - requires task-admin badge"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn trigger_cleanup(
    user: AuthUser,
    cleanup_service: web::Data<CleanupService>,
    body: ValidatedJson<TriggerCleanupRequest>,
) -> Result<HttpResponse> {
    // Check admin permission (requires task-admin badge)
    if !user.has_badge("task-admin") {
        return Err(shared_lib::AppError::Forbidden(
            "Requires task-admin badge to trigger cleanup jobs".into(),
        ));
    }

    let start = Instant::now();
    let request = body.into_inner();

    tracing::info!(
        user_id = %user.id,
        force = request.force,
        dry_run = request.dry_run,
        "Admin triggered user cleanup job"
    );

    let stats = cleanup_service.cleanup_deleted_users(request.dry_run).await?;
    let execution_time_ms = start.elapsed().as_millis() as u64;

    let response = ManualCleanupResponse {
        success: true,
        stats,
        execution_time_ms,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Get cleanup job statistics (admin only)
///
/// Returns information about soft-deleted users eligible for cleanup
/// without actually deleting anything (equivalent to dry_run=true)
#[utoipa::path(
    get,
    path = "/api/v1/cleanup/stats",
    tag = "cleanup",
    responses(
        (status = 200, description = "Cleanup statistics retrieved", body = ManualCleanupResponse),
        (status = 401, description = "Unauthorized - missing or invalid JWT"),
        (status = 403, description = "Forbidden - requires task-admin badge"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn get_cleanup_stats(
    user: AuthUser,
    cleanup_service: web::Data<CleanupService>,
) -> Result<HttpResponse> {
    // Check admin permission
    if !user.has_badge("task-admin") {
        return Err(shared_lib::AppError::Forbidden(
            "Requires task-admin badge to view cleanup statistics".into(),
        ));
    }

    let start = Instant::now();
    
    // Dry run to get statistics without deleting
    let stats = cleanup_service.cleanup_deleted_users(true).await?;
    let execution_time_ms = start.elapsed().as_millis() as u64;

    let response = ManualCleanupResponse {
        success: true,
        stats,
        execution_time_ms,
    };

    Ok(HttpResponse::Ok().json(response))
}
