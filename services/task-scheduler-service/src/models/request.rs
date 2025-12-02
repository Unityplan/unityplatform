use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Request to manually trigger cleanup (admin only)
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TriggerCleanupRequest {
    /// Force cleanup even if not enough time has passed
    #[serde(default)]
    pub force: bool,
    
    /// Dry run mode (don't actually delete, just report what would be deleted)
    #[serde(default)]
    pub dry_run: bool,
}
