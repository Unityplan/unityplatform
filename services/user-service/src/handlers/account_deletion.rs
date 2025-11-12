use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use shared_lib::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::user::{
    AccountDeletionCancelRequest, AccountDeletionConfirmRequest, AccountDeletionRequest,
    AccountDeletionResponse, DeletionStatus,
};
use crate::response::ApiResponse;

// ============================================================================
// REQUEST ACCOUNT DELETION
// ============================================================================

/// Request account deletion (GDPR Article 17 - Right to Erasure)
///
/// Initiates a 30-day soft delete process. User will receive an email
/// with a confirmation token. Account can be cancelled during the 30-day period.
#[utoipa::path(
    post,
    path = "/v1/users/{id}/account/delete",
    tag = "gdpr",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Deletion request created", body = ApiResponse<AccountDeletionResponse>),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "User not found"),
        (status = 409, description = "Deletion already pending")
    )
)]
pub async fn request_account_deletion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Verify user exists and is active
    let user = sqlx::query!(
        "SELECT id, is_active, deleted_at FROM territory_dk.users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    let user = user.ok_or(AppError::NotFound("User not found".to_string()))?;

    if user.deleted_at.is_some() {
        return Err(AppError::Validation(
            "Account is already deleted".to_string(),
        ));
    }

    // Check if there's already a pending deletion request
    let existing = sqlx::query!(
        r#"
        SELECT id, status as "status: DeletionStatus"
        FROM territory_dk.account_deletion_requests
        WHERE user_id = $1 AND status IN ('pending', 'confirmed')
        ORDER BY requested_at DESC
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    if let Some(existing_request) = existing {
        return Err(AppError::Validation(format!(
            "Account deletion already {} (Request ID: {})",
            match existing_request.status {
                DeletionStatus::Pending => "pending email confirmation",
                DeletionStatus::Confirmed => "confirmed and scheduled",
                _ => "in progress",
            },
            existing_request.id
        )));
    }

    // Generate confirmation token (in production, this should be cryptographically secure)
    let confirmation_token = format!("{}", Uuid::new_v4());
    let token_expires_at = Utc::now() + chrono::Duration::hours(24);

    // Get IP address and user agent
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Create deletion request
    let request = sqlx::query_as!(
        AccountDeletionRequest,
        r#"
        INSERT INTO territory_dk.account_deletion_requests (
            user_id, status, confirmation_token, token_expires_at, ip_address, user_agent
        ) VALUES ($1, 'pending', $2, $3, $4, $5)
        RETURNING 
            id, user_id, 
            status as "status: DeletionStatus", 
            confirmation_token, token_expires_at,
            requested_at, confirmed_at, scheduled_deletion_at,
            cancelled_at, cancellation_reason, ip_address, user_agent
        "#,
        user_id,
        confirmation_token,
        token_expires_at,
        ip_address,
        user_agent
    )
    .fetch_one(pool.as_ref())
    .await?;

    // TODO: Send confirmation email with token
    tracing::info!(
        "Account deletion requested for user {} - Confirmation token: {}",
        user_id,
        confirmation_token
    );

    let response = AccountDeletionResponse {
        id: request.id,
        status: request.status,
        requested_at: request.requested_at,
        confirmed_at: request.confirmed_at,
        scheduled_deletion_at: request.scheduled_deletion_at,
        days_until_deletion: None,
        can_cancel: true,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

// ============================================================================
// CONFIRM ACCOUNT DELETION
// ============================================================================

/// Confirm account deletion with email token
///
/// After confirming, account will be scheduled for deletion in 30 days.
/// User can still cancel during this period.
#[utoipa::path(
    post,
    path = "/v1/users/{id}/account/delete/confirm",
    tag = "gdpr",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = AccountDeletionConfirmRequest,
    responses(
        (status = 200, description = "Deletion confirmed and scheduled", body = ApiResponse<AccountDeletionResponse>),
        (status = 400, description = "Invalid or expired token"),
        (status = 404, description = "Deletion request not found"),
        (status = 410, description = "Token expired")
    )
)]
pub async fn confirm_account_deletion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AccountDeletionConfirmRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Validate the request body
    if let Err(validation_errors) = body.validate() {
        return Err(AppError::Validation(format!("{}", validation_errors)));
    }

    // Find the pending deletion request
    let request = sqlx::query!(
        r#"
        SELECT id, status as "status: DeletionStatus", token_expires_at
        FROM territory_dk.account_deletion_requests
        WHERE user_id = $1 
          AND status = 'pending'
          AND confirmation_token = $2
        ORDER BY requested_at DESC
        LIMIT 1
        "#,
        user_id,
        body.confirmation_token
    )
    .fetch_optional(pool.as_ref())
    .await?;

    let request = request.ok_or(AppError::NotFound(
        "Deletion request not found or already confirmed".to_string(),
    ))?;

    // Check if token is expired
    if let Some(expires_at) = request.token_expires_at {
        if Utc::now() > expires_at {
            return Err(AppError::Validation(
                "Confirmation token has expired. Please request deletion again.".to_string(),
            ));
        }
    }

    // Confirm the deletion (trigger will set scheduled_deletion_at to +30 days)
    let confirmed = sqlx::query!(
        r#"
        UPDATE territory_dk.account_deletion_requests
        SET status = 'confirmed',
            confirmed_at = NOW()
        WHERE id = $1
        RETURNING 
            id, user_id, 
            status as "status: DeletionStatus", 
            confirmation_token, token_expires_at,
            requested_at, confirmed_at, scheduled_deletion_at,
            cancelled_at, cancellation_reason, ip_address, user_agent
        "#,
        request.id
    )
    .fetch_one(pool.as_ref())
    .await?;

    // Build the response struct
    let confirmed_request = AccountDeletionRequest {
        id: confirmed.id,
        user_id: confirmed.user_id,
        status: confirmed.status,
        confirmation_token: confirmed.confirmation_token,
        token_expires_at: confirmed.token_expires_at,
        requested_at: confirmed.requested_at,
        confirmed_at: confirmed.confirmed_at,
        scheduled_deletion_at: confirmed.scheduled_deletion_at,
        cancelled_at: confirmed.cancelled_at,
        cancellation_reason: confirmed.cancellation_reason,
        ip_address: confirmed.ip_address,
        user_agent: confirmed.user_agent,
    };

    // Calculate days until deletion
    let days_until_deletion = if let Some(scheduled) = confirmed_request.scheduled_deletion_at {
        Some((scheduled - Utc::now()).num_days())
    } else {
        None
    };

    tracing::info!(
        "Account deletion confirmed for user {} - Scheduled for: {:?}",
        user_id,
        confirmed_request.scheduled_deletion_at
    );

    let response = AccountDeletionResponse {
        id: confirmed_request.id,
        status: confirmed_request.status,
        requested_at: confirmed_request.requested_at,
        confirmed_at: confirmed_request.confirmed_at,
        scheduled_deletion_at: confirmed_request.scheduled_deletion_at,
        days_until_deletion,
        can_cancel: true,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

// ============================================================================
// GET DELETION STATUS
// ============================================================================

/// Get account deletion status
///
/// Returns the current status of any pending or confirmed deletion request.
#[utoipa::path(
    get,
    path = "/v1/users/{id}/account/delete",
    tag = "gdpr",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Deletion status retrieved", body = ApiResponse<AccountDeletionResponse>),
        (status = 404, description = "No pending deletion request")
    )
)]
pub async fn get_deletion_status(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    let request = sqlx::query!(
        r#"
        SELECT 
            id, user_id, 
            status as "status: DeletionStatus", 
            confirmation_token, token_expires_at,
            requested_at, confirmed_at, scheduled_deletion_at,
            cancelled_at, cancellation_reason, ip_address, user_agent
        FROM territory_dk.account_deletion_requests
        WHERE user_id = $1 AND status IN ('pending', 'confirmed')
        ORDER BY requested_at DESC
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    let request = request.ok_or(AppError::NotFound(
        "No pending deletion request found".to_string(),
    ))?;

    // Build the response struct
    let deletion_request = AccountDeletionRequest {
        id: request.id,
        user_id: request.user_id,
        status: request.status,
        confirmation_token: request.confirmation_token,
        token_expires_at: request.token_expires_at,
        requested_at: request.requested_at,
        confirmed_at: request.confirmed_at,
        scheduled_deletion_at: request.scheduled_deletion_at,
        cancelled_at: request.cancelled_at,
        cancellation_reason: request.cancellation_reason,
        ip_address: request.ip_address,
        user_agent: request.user_agent,
    };

    // Calculate days until deletion
    let days_until_deletion = if let Some(scheduled) = deletion_request.scheduled_deletion_at {
        Some((scheduled - Utc::now()).num_days())
    } else {
        None
    };

    let can_cancel = matches!(
        deletion_request.status,
        DeletionStatus::Pending | DeletionStatus::Confirmed
    );

    let response = AccountDeletionResponse {
        id: deletion_request.id,
        status: deletion_request.status,
        requested_at: deletion_request.requested_at,
        confirmed_at: deletion_request.confirmed_at,
        scheduled_deletion_at: deletion_request.scheduled_deletion_at,
        days_until_deletion,
        can_cancel,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

// ============================================================================
// CANCEL ACCOUNT DELETION
// ============================================================================

/// Cancel pending account deletion
///
/// Can only cancel deletion requests that are pending or confirmed (before actual deletion).
#[utoipa::path(
    delete,
    path = "/v1/users/{id}/account/delete",
    tag = "gdpr",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = AccountDeletionCancelRequest,
    responses(
        (status = 200, description = "Deletion cancelled", body = ApiResponse<AccountDeletionResponse>),
        (status = 404, description = "No pending deletion request"),
        (status = 410, description = "Deletion already completed")
    )
)]
pub async fn cancel_account_deletion(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AccountDeletionCancelRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Find the active deletion request
    let request = sqlx::query!(
        r#"
        SELECT id, status as "status: DeletionStatus"
        FROM territory_dk.account_deletion_requests
        WHERE user_id = $1 AND status IN ('pending', 'confirmed')
        ORDER BY requested_at DESC
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    let request = request.ok_or(AppError::NotFound(
        "No active deletion request found to cancel".to_string(),
    ))?;

    // Cancel the deletion
    let cancelled = sqlx::query!(
        r#"
        UPDATE territory_dk.account_deletion_requests
        SET status = 'cancelled',
            cancelled_at = NOW(),
            cancellation_reason = $2
        WHERE id = $1
        RETURNING 
            id, user_id, 
            status as "status: DeletionStatus", 
            confirmation_token, token_expires_at,
            requested_at, confirmed_at, scheduled_deletion_at,
            cancelled_at, cancellation_reason, ip_address, user_agent
        "#,
        request.id,
        body.reason.as_deref()
    )
    .fetch_one(pool.as_ref())
    .await?;

    // Build the response struct
    let cancelled_request = AccountDeletionRequest {
        id: cancelled.id,
        user_id: cancelled.user_id,
        status: cancelled.status,
        confirmation_token: cancelled.confirmation_token,
        token_expires_at: cancelled.token_expires_at,
        requested_at: cancelled.requested_at,
        confirmed_at: cancelled.confirmed_at,
        scheduled_deletion_at: cancelled.scheduled_deletion_at,
        cancelled_at: cancelled.cancelled_at,
        cancellation_reason: cancelled.cancellation_reason,
        ip_address: cancelled.ip_address,
        user_agent: cancelled.user_agent,
    };

    tracing::info!(
        "Account deletion cancelled for user {} - Reason: {:?}",
        user_id,
        body.reason
    );

    let response = AccountDeletionResponse {
        id: cancelled_request.id,
        status: cancelled_request.status,
        requested_at: cancelled_request.requested_at,
        confirmed_at: cancelled_request.confirmed_at,
        scheduled_deletion_at: cancelled_request.scheduled_deletion_at,
        days_until_deletion: None,
        can_cancel: false,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
