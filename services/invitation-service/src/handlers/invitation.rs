use actix_web::{web, HttpResponse};
use shared_lib::{AppConfig, Database, NatsClient, ValidatedJson};
use tracing::info;

use crate::models::usage::{UseInvitationRequest, UseInvitationResponse};
use crate::models::validation::{ValidateInvitationRequest, ValidateInvitationResponse};
use crate::services::invitation::{use_invitation, validate_invitation};

/// Validate invitation token
///
/// Called by auth-service during registration to check if invitation token is valid.
/// This endpoint is public (no authentication required).
#[utoipa::path(
    post,
    path = "/api/v1/invitations/validate",
    tag = "invitations",
    request_body = ValidateInvitationRequest,
    responses(
        (status = 200, description = "Validation result", body = ValidateInvitationResponse),
        (status = 400, description = "Invalid request"),
    )
)]
pub async fn validate_invitation_handler(
    database: web::Data<Database>,
    config: web::Data<AppConfig>,
    body: ValidatedJson<ValidateInvitationRequest>,
) -> HttpResponse {
    let token = &body.token;
    
    // Get territory code from config
    let territory_code = &config.server.territory;
    
    info!(
        token = %token,
        territory = %territory_code,
        "Validating invitation token"
    );
    
    // Validate invitation
    match validate_invitation(database.pool(), token, territory_code).await {
        Ok(Some(invitation)) => {
            // Calculate uses remaining
            let uses_remaining = if invitation.max_uses == 0 {
                0 // Unlimited uses
            } else {
                invitation.max_uses - invitation.uses_count
            };
            
            info!(
                invitation_id = %invitation.id,
                created_by = %invitation.created_by,
                uses_remaining = uses_remaining,
                "Invitation token is valid"
            );
            
            HttpResponse::Ok().json(ValidateInvitationResponse::valid(
                invitation.id,
                invitation.created_by,
                uses_remaining,
                invitation.expires_at,
                invitation.metadata,
            ))
        }
        Ok(None) => {
            info!(token = %token, "Invitation token is invalid");
            HttpResponse::Ok().json(ValidateInvitationResponse::invalid())
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to validate invitation");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to validate invitation"
            }))
        }
    }
}

/// Use invitation token after successful registration
///
/// Called by auth-service AFTER successful user registration to record usage.
/// This endpoint requires authentication from auth-service (service-to-service).
#[utoipa::path(
    post,
    path = "/api/v1/invitations/use",
    tag = "invitations",
    request_body = UseInvitationRequest,
    responses(
        (status = 200, description = "Usage recorded successfully", body = UseInvitationResponse),
        (status = 400, description = "Invalid request or token fully used"),
        (status = 404, description = "Invalid or expired token"),
        (status = 409, description = "User already used this token"),
    )
)]
pub async fn use_invitation_handler(
    database: web::Data<Database>,
    config: web::Data<AppConfig>,
    nats: web::Data<NatsClient>,
    body: ValidatedJson<UseInvitationRequest>,
) -> HttpResponse {
    let token = &body.token;
    let used_by = body.used_by;
    let territory_code = &config.server.territory;
    
    info!(
        token = %token,
        used_by = %used_by,
        territory = %territory_code,
        "Recording invitation usage"
    );
    
    // Record usage
    match use_invitation(
        database.pool(),
        token,
        used_by,
        territory_code,
        body.ip_address.clone(),
        body.user_agent.clone(),
    )
    .await
    {
        Ok(response) => {
            info!(
                invitation_id = %response.invitation_id,
                uses_remaining = response.uses_remaining,
                fully_used = response.fully_used,
                "Invitation usage recorded successfully"
            );
            
            // Publish NATS event: invitation.used
            let event_payload = serde_json::json!({
                "invitation_id": response.invitation_id,
                "used_by": used_by,
                "token": token,
                "territory": territory_code,
                "uses_remaining": response.uses_remaining,
                "fully_used": response.fully_used,
                "timestamp": chrono::Utc::now(),
            });
            
            if let Err(e) = nats
                .publish("invitation.used", event_payload.to_string().as_bytes().to_vec())
                .await
            {
                tracing::warn!(
                    error = %e,
                    "Failed to publish invitation.used event (non-critical)"
                );
            }
            
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to record invitation usage");
            HttpResponse::from(actix_web::Error::from(e))
        }
    }
}
