use crate::error::{AuthError, AuthResult};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

/// Details about an invitation token
#[derive(Debug)]
pub struct InvitationTokenDetails {
    pub id: Uuid,
    pub max_uses: i32,
    pub used_count: i32,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub restricted_email: Option<String>,
}

/// Generate a new invitation token
pub fn generate_invitation_token() -> String {
    format!("inv_{}", Uuid::new_v4().simple().to_string())
}

/// Get the territory code for an invitation token from the global registry
pub async fn get_token_territory(pool: &PgPool, token: &str) -> AuthResult<String> {
    let result = sqlx::query_scalar::<_, String>(
        r#"
        SELECT territory_code
        FROM global.invitation_token_registry
        WHERE token = $1
        "#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;

    result.ok_or(AuthError::InvalidInvitation)
}

/// Validate an invitation token and return its details
pub async fn validate_invitation_token(
    pool: &PgPool,
    schema: &str,
    token: &str,
    email: Option<&str>,
) -> AuthResult<InvitationTokenDetails> {
    let query = format!(
        r#"
        SELECT id, max_uses, used_count, expires_at, email
        FROM {}.invitation_tokens
        WHERE token = $1 AND is_active = true
        "#,
        schema
    );

    let result = sqlx::query_as::<
        _,
        (
            Uuid,
            i32,
            i32,
            Option<chrono::DateTime<chrono::Utc>>,
            Option<String>,
        ),
    >(&query)
    .bind(token)
    .fetch_optional(pool)
    .await?
    .ok_or(AuthError::InvalidInvitation)?;

    let details = InvitationTokenDetails {
        id: result.0,
        max_uses: result.1,
        used_count: result.2,
        expires_at: result.3,
        restricted_email: result.4,
    };

    // Check if token is expired
    if let Some(expires_at) = details.expires_at {
        if expires_at < chrono::Utc::now() {
            return Err(AuthError::InvitationExpired);
        }
    }

    // Check if token has reached max uses
    if details.used_count >= details.max_uses {
        return Err(AuthError::InvitationUsed);
    }

    // Check email restriction if applicable
    if let Some(restricted_email) = &details.restricted_email {
        if let Some(provided_email) = email {
            if restricted_email != provided_email {
                return Err(AuthError::InvalidInvitation);
            }
        } else {
            // Email is restricted but none provided
            return Err(AuthError::InvalidInvitation);
        }
    }

    Ok(details)
}

/// Mark an invitation token as used (using existing transaction)
pub async fn use_invitation_token<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
    schema: &str,
    token_id: Uuid,
    user_id: Uuid,
) -> AuthResult<()> {
    // Increment used_count
    let update_query = format!(
        r#"
        UPDATE {}.invitation_tokens
        SET used_count = used_count + 1
        WHERE id = $1
        "#,
        schema
    );

    sqlx::query(&update_query)
        .bind(token_id)
        .execute(&mut **tx)
        .await?;

    // Record the usage
    let insert_query = format!(
        r#"
        INSERT INTO {}.invitation_uses (invitation_token_id, user_id)
        VALUES ($1, $2)
        "#,
        schema
    );

    sqlx::query(&insert_query)
        .bind(token_id)
        .bind(user_id)
        .execute(&mut **tx)
        .await?;

    Ok(())
}

/// Hash a refresh token for secure storage
pub fn hash_refresh_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}
