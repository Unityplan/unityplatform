use serde::{Deserialize, Serialize};
use shared_lib::{AppError, Result};
use uuid::Uuid;

/// Invitation validation request
#[derive(Debug, Serialize)]
pub struct ValidateInvitationRequest {
    pub token: String,
}

/// Invitation validation response
#[derive(Debug, Deserialize)]
pub struct ValidateInvitationResponse {
    pub valid: bool,
}

/// Use invitation request
#[derive(Debug, Serialize)]
pub struct UseInvitationRequest {
    pub token: String,
    pub used_by: Uuid,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Use invitation response
#[derive(Debug, Deserialize)]
pub struct UseInvitationResponse {
    pub invitation_id: Uuid,
    pub uses_remaining: i32,
    pub fully_used: bool,
}

/// Validate an invitation token
///
/// # Arguments
/// * `invitation_service_url` - Base URL of invitation-service (e.g., "http://localhost:8004")
/// * `token` - Invitation token to validate
/// * `allow_open_registration` - If true, validation failure is non-fatal (dev mode)
///
/// # Returns
/// * Ok(true) - Token is valid
/// * Ok(false) - Token is invalid (only in dev mode with allow_open_registration=true)
/// * Err - Service unavailable or validation required but failed (production mode)
pub async fn validate_invitation(
    invitation_service_url: &str,
    token: &str,
    allow_open_registration: bool,
) -> Result<bool> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/invitations/validate", invitation_service_url);

    let request = ValidateInvitationRequest {
        token: token.to_string(),
    };

    match client.post(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let validation: ValidateInvitationResponse =
                    response.json().await.map_err(|e| {
                        AppError::Internal(format!("Failed to parse validation response: {}", e))
                    })?;
                Ok(validation.valid)
            } else {
                // Invitation is invalid
                if allow_open_registration {
                    tracing::warn!(
                        "Invitation validation failed but allow_open_registration=true, continuing"
                    );
                    Ok(false)
                } else {
                    Err(AppError::Validation("Invalid invitation token".to_string()))
                }
            }
        }
        Err(e) => {
            // Service unavailable
            if allow_open_registration {
                tracing::warn!(
                    "Invitation service unavailable but allow_open_registration=true: {}",
                    e
                );
                Ok(false) // Continue without validation in dev mode
            } else {
                Err(AppError::Internal(format!(
                    "Invitation service unavailable: {}",
                    e
                )))
            }
        }
    }
}

/// Mark an invitation as used
///
/// # Arguments
/// * `invitation_service_url` - Base URL of invitation-service
/// * `token` - Invitation token
/// * `used_by` - User ID who used the invitation
/// * `ip_address` - Optional IP address
/// * `user_agent` - Optional user agent
/// * `allow_open_registration` - If true, usage failure is non-fatal (dev mode)
///
/// # Returns
/// * Ok(Some(response)) - Invitation marked as used successfully
/// * Ok(None) - Failed to mark but non-fatal (dev mode)
/// * Err - Service error in production mode
pub async fn use_invitation(
    invitation_service_url: &str,
    token: &str,
    used_by: Uuid,
    ip_address: Option<String>,
    user_agent: Option<String>,
    allow_open_registration: bool,
) -> Result<Option<UseInvitationResponse>> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/invitations/use", invitation_service_url);

    let request = UseInvitationRequest {
        token: token.to_string(),
        used_by,
        ip_address,
        user_agent,
    };

    match client.post(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let usage: UseInvitationResponse = response.json().await.map_err(|e| {
                    AppError::Internal(format!("Failed to parse use invitation response: {}", e))
                })?;
                Ok(Some(usage))
            } else {
                // Failed to mark as used
                if allow_open_registration {
                    tracing::warn!(
                        "Failed to mark invitation as used but allow_open_registration=true"
                    );
                    Ok(None)
                } else {
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    Err(AppError::Internal(format!(
                        "Failed to mark invitation as used: {}",
                        error_text
                    )))
                }
            }
        }
        Err(e) => {
            // Service unavailable
            if allow_open_registration {
                tracing::warn!(
                    "Failed to connect to invitation service but allow_open_registration=true: {}",
                    e
                );
                Ok(None) // Non-fatal in dev mode
            } else {
                // In production, this is non-fatal since user is already created
                // Just log the error and continue
                tracing::error!(
                    "Failed to mark invitation as used (user already created): {}",
                    e
                );
                Ok(None)
            }
        }
    }
}
