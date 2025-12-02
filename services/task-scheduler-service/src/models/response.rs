use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Response for cleanup job execution
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanupJobResponse {
    /// Job name
    pub job_name: String,
    /// Number of records cleaned up
    pub records_cleaned: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Job status
    pub status: String,
}

/// Statistics for user cleanup
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserCleanupStats {
    /// Total soft-deleted users found
    pub soft_deleted_users: u64,
    /// Users eligible for hard deletion (>30 days)
    pub eligible_for_deletion: u64,
    /// Users actually deleted
    pub deleted: u64,
    /// Territories processed
    pub territories_processed: Vec<String>,
}

/// Response for manual cleanup trigger
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ManualCleanupResponse {
    /// Success status
    pub success: bool,
    /// Cleanup statistics
    pub stats: UserCleanupStats,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}
