use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: String,
    pub dependencies: HashMap<String, String>,
}

impl HealthResponse {
    pub fn healthy(dependencies: HashMap<String, String>) -> Self {
        Self {
            status: "healthy".to_string(),
            service: "auth-service".to_string(),
            version: shared_lib::version::VERSION.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            dependencies,
        }
    }
}
