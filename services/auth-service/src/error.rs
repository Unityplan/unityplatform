use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid invitation token")]
    InvalidInvitation,

    #[error("Invitation token expired")]
    InvitationExpired,

    #[error("Invitation token already used")]
    InvitationUsed,

    #[error("Username already exists")]
    UsernameExists,

    #[error("Email already exists")]
    EmailExists,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal server error")]
    InternalError,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl AuthError {
    pub fn error_code(&self) -> &str {
        match self {
            AuthError::InvalidCredentials => "INVALID_CREDENTIALS",
            AuthError::InvalidToken(_) => "INVALID_TOKEN",
            AuthError::TokenExpired => "TOKEN_EXPIRED",
            AuthError::InvalidInvitation => "INVALID_INVITATION",
            AuthError::InvitationExpired => "INVITATION_EXPIRED",
            AuthError::InvitationUsed => "INVITATION_USED",
            AuthError::UsernameExists => "USERNAME_EXISTS",
            AuthError::EmailExists => "EMAIL_EXISTS",
            AuthError::ValidationError(_) => "VALIDATION_ERROR",
            AuthError::DatabaseError(_) => "DATABASE_ERROR",
            AuthError::InternalError => "INTERNAL_ERROR",
            AuthError::Unauthorized => "UNAUTHORIZED",
            AuthError::BadRequest(_) => "BAD_REQUEST",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
            AuthError::InvalidInvitation => StatusCode::BAD_REQUEST,
            AuthError::InvitationExpired => StatusCode::BAD_REQUEST,
            AuthError::InvitationUsed => StatusCode::BAD_REQUEST,
            AuthError::UsernameExists => StatusCode::CONFLICT,
            AuthError::EmailExists => StatusCode::CONFLICT,
            AuthError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let error_response = crate::response::ApiResponse::<()>::error(
            self.error_code().to_string(),
            self.to_string(),
            None,
        );

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        self.status_code()
    }
}

impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
