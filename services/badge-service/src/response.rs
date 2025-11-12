use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::error::ApiErrorResponse>,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseMeta {
    pub timestamp: String,
    pub request_id: String,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: ResponseMeta {
                timestamp: Utc::now().to_rfc3339(),
                request_id: Uuid::new_v4().to_string(),
            },
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn error(code: String, message: String, details: Option<serde_json::Value>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(crate::error::ApiErrorResponse {
                code,
                message,
                details,
            }),
            meta: ResponseMeta {
                timestamp: Utc::now().to_rfc3339(),
                request_id: Uuid::new_v4().to_string(),
            },
        }
    }
}
