use crate::models::invitation::{InvitationToken, InvitationUse};
use shared_lib::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

/// Generate a cryptographically secure invitation token
/// Format: "inv_" + 32 hexadecimal characters
pub fn generate_invitation_token() -> String {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 16] = rng.gen();
    let hex_string = hex::encode(random_bytes);

    format!("inv_{}", hex_string)
}

/// Validate an invitation token without consuming it
///
/// This checks:
/// 1. Token exists and is active
/// 2. Token has not expired
/// 3. Token has available uses (used_count < max_uses)
/// 4. For single_use tokens: email matches (if provided)
///
/// Returns the token if valid, error otherwise
pub async fn validate_invitation_token(
    pool: &PgPool,
    schema_name: &str,
    token: &str,
    email: Option<&str>,
) -> Result<InvitationToken, AppError> {
    // Query token from territory schema
    let query = format!(
        r#"
        SELECT 
            id, token, token_type, created_by_user_id,
            invited_email, community_id, role,
            max_uses, current_uses,
            expires_at, is_active,
            created_at, updated_at
        FROM {}.invitation_tokens
        WHERE token = $1
        "#,
        schema_name
    );

    let token_record = sqlx::query_as::<_, InvitationToken>(&query)
        .bind(token)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Validation("Invalid invitation token".to_string()))?;

    // Check if token is active
    if !token_record.is_active {
        return Err(AppError::Validation(
            "This invitation token has been revoked".to_string(),
        ));
    }

    // Check if token has expired
    if let Some(expires_at) = token_record.expires_at {
        if chrono::Utc::now() > expires_at {
            return Err(AppError::Validation(
                "This invitation token has expired".to_string(),
            ));
        }
    }

    // Check if token has available uses
    if let Some(max_uses) = token_record.max_uses {
        if token_record.current_uses >= max_uses {
            return Err(AppError::Validation(
                "This invitation token has reached its maximum number of uses".to_string(),
            ));
        }
    }

    // For single_use tokens, verify email matches if provided
    if token_record.token_type == "single_use" {
        if let Some(token_email) = &token_record.invited_email {
            if let Some(user_email) = email {
                if token_email.to_lowercase() != user_email.to_lowercase() {
                    return Err(AppError::Validation(
                        "This invitation token is for a different email address".to_string(),
                    ));
                }
            }
        }
    }

    Ok(token_record)
}

/// Mark an invitation token as used
///
/// This:
/// 1. Increments the used_count
/// 2. Creates an audit record in invitation_uses
/// 3. Deactivates the token if max_uses is reached
///
/// Must be called within a transaction along with user creation
pub async fn use_invitation_token(
    pool: &PgPool,
    schema_name: &str,
    token_id: Uuid,
    user_id: Uuid,
    ip_address: Option<String>,
) -> Result<(), AppError> {
    // Increment current_uses
    let update_query = format!(
        r#"
        UPDATE {}.invitation_tokens
        SET 
            current_uses = current_uses + 1,
            is_active = CASE 
                WHEN max_uses IS NOT NULL AND current_uses + 1 >= max_uses THEN false
                ELSE is_active
            END,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id
        "#,
        schema_name
    );

    sqlx::query(&update_query)
        .bind(token_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update invitation token: {}", e)))?;

    // Create audit record
    let audit_query = format!(
        r#"
        INSERT INTO {}.invitation_uses (id, token_id, used_by_user_id, used_at, ip_address)
        VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4::inet)
        "#,
        schema_name
    );

    let audit_result = sqlx::query(&audit_query)
        .bind(Uuid::new_v4())
        .bind(token_id)
        .bind(user_id)
        .bind(ip_address)
        .execute(pool)
        .await;

    match audit_result {
        Ok(_) => {
            tracing::info!(
                "Invitation usage audit record created successfully for user {}",
                user_id
            );
            Ok(())
        }
        Err(e) => {
            tracing::error!(
                "Failed to create invitation use audit record: {} (token_id: {}, user_id: {})",
                e,
                token_id,
                user_id
            );
            Err(AppError::Internal(format!(
                "Failed to create invitation use audit record: {}",
                e
            )))
        }
    }
}

/// Create a new invitation token
///
/// This generates a new token and stores it in the database
/// Returns the created token with all fields populated
pub async fn create_invitation_token(
    pool: &PgPool,
    schema_name: &str,
    token_type: &str,
    email: Option<String>,
    max_uses: i32,
    expires_in_days: Option<i64>,
    _purpose: Option<String>, // Deprecated - kept for API compatibility
    created_by: Option<Uuid>, // None for bootstrap tokens
) -> Result<InvitationToken, AppError> {
    // Generate token
    let token = generate_invitation_token();
    let id = Uuid::new_v4();

    // Calculate expiration (default to 30 days if not specified)
    let expires_at = match expires_in_days {
        Some(days) => chrono::Utc::now() + chrono::Duration::days(days),
        None => chrono::Utc::now() + chrono::Duration::days(30),
    };

    // Insert token
    let insert_query = format!(
        r#"
        INSERT INTO {}.invitation_tokens 
            (id, token, token_type, invited_email, max_uses, current_uses, expires_at, is_active, created_by_user_id)
        VALUES ($1, $2, $3, $4, $5, 0, $6, true, $7)
        RETURNING 
            id, token, token_type, created_by_user_id,
            invited_email, community_id, role,
            max_uses, current_uses,
            expires_at, is_active,
            created_at, updated_at
        "#,
        schema_name
    );

    let created_token = sqlx::query_as::<_, InvitationToken>(&insert_query)
        .bind(id)
        .bind(&token)
        .bind(token_type)
        .bind(email)
        .bind(Some(max_uses))
        .bind(expires_at)
        .bind(created_by)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create invitation token: {}", e)))?;

    Ok(created_token)
}

/// List invitation tokens created by a specific user
///
/// Returns all tokens (active and inactive) created by the user
/// Useful for territory managers to see their invitations
pub async fn list_user_invitations(
    pool: &PgPool,
    schema_name: &str,
    user_id: Uuid,
) -> Result<Vec<InvitationToken>, AppError> {
    let query = format!(
        r#"
        SELECT 
            id, token, token_type, created_by_user_id,
            invited_email, community_id, role,
            max_uses, current_uses,
            expires_at, is_active,
            created_at, updated_at
        FROM {}.invitation_tokens
        WHERE created_by_user_id = $1
        ORDER BY created_at DESC
        "#,
        schema_name
    );

    let tokens = sqlx::query_as::<_, InvitationToken>(&query)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to fetch invitation tokens: {}", e)))?;

    Ok(tokens)
}

/// Revoke an invitation token
///
/// Deactivates the token so it cannot be used for new registrations
/// Existing users who already used the token are not affected
pub async fn revoke_invitation_token(
    pool: &PgPool,
    schema_name: &str,
    token_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    // Only allow revoking tokens created by the user
    let query = format!(
        r#"
        UPDATE {}.invitation_tokens
        SET 
            is_active = false,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND created_by_user_id = $2
        RETURNING id
        "#,
        schema_name
    );

    let result = sqlx::query(&query)
        .bind(token_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to revoke invitation token: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Invitation token not found or you don't have permission to revoke it".to_string(),
        ));
    }

    Ok(())
}

/// Get usage statistics for an invitation token
///
/// Returns list of users who used this token
pub async fn get_invitation_uses(
    pool: &PgPool,
    schema_name: &str,
    token_id: Uuid,
) -> Result<Vec<InvitationUse>, AppError> {
    let query = format!(
        r#"
        SELECT id, token_id, used_by_user_id, ip_address, user_agent, used_at
        FROM {}.invitation_uses
        WHERE token_id = $1
        ORDER BY used_at DESC
        "#,
        schema_name
    );

    let uses = sqlx::query_as::<_, InvitationUse>(&query)
        .bind(token_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to fetch invitation uses: {}", e)))?;

    Ok(uses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_invitation_token_format() {
        let token = generate_invitation_token();

        // Should start with "inv_"
        assert!(token.starts_with("inv_"));

        // Should be "inv_" + 32 hex chars = 36 chars total
        assert_eq!(token.len(), 36);

        // Everything after "inv_" should be valid hex
        let hex_part = &token[4..];
        assert!(hex_part.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_invitation_token_uniqueness() {
        let token1 = generate_invitation_token();
        let token2 = generate_invitation_token();

        // Tokens should be unique (statistically certain with 128-bit random)
        assert_ne!(token1, token2);
    }
}
