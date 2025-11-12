use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub dependencies: DependencyStatus,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DependencyStatus {
    pub database: String,
}
