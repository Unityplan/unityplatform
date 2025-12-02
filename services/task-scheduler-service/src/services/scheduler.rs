use shared_lib::Result;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{info, error};
use std::sync::Arc;

use crate::services::CleanupService;

/// Initialize and start the cron scheduler for cleanup jobs
/// 
/// Schedules:
/// - User cleanup: Every Sunday at 2:00 AM UTC (weekly)
pub async fn start_scheduler(cleanup_service: Arc<CleanupService>) -> Result<JobScheduler> {
    let scheduler = JobScheduler::new().await
        .map_err(|e| shared_lib::AppError::Internal(format!("Failed to create scheduler: {}", e)))?;

    // Weekly user cleanup job - Sundays at 2:00 AM UTC
    let cleanup_job = Job::new_async("0 0 2 * * Sun", move |_uuid, _lock| {
        let cleanup_service = cleanup_service.clone();
        Box::pin(async move {
            info!("🕐 Running scheduled user cleanup job (weekly, Sunday 2:00 AM UTC)");
            
            match cleanup_service.cleanup_deleted_users(false).await {
                Ok(stats) => {
                    info!(
                        soft_deleted = stats.soft_deleted_users,
                        eligible = stats.eligible_for_deletion,
                        deleted = stats.deleted,
                        territories = ?stats.territories_processed,
                        "✅ Scheduled cleanup completed successfully"
                    );
                }
                Err(e) => {
                    error!(
                        error = %e,
                        "❌ Scheduled cleanup failed"
                    );
                }
            }
        })
    })
    .map_err(|e| shared_lib::AppError::Internal(format!("Failed to create cleanup job: {}", e)))?;

    scheduler.add(cleanup_job).await
        .map_err(|e| shared_lib::AppError::Internal(format!("Failed to add cleanup job: {}", e)))?;

    scheduler.start().await
        .map_err(|e| shared_lib::AppError::Internal(format!("Failed to start scheduler: {}", e)))?;

    info!("✅ Cron scheduler started");
    info!("   📅 User cleanup: Every Sunday at 2:00 AM UTC");

    Ok(scheduler)
}
