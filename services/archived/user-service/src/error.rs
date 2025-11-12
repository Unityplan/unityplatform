use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

pub type AuthResult<T> = Result<T, ServiceError>;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
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

    #[error("Not found: {0}")]
    NotFound(String),

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

impl From<validator::ValidationErrors> for ServiceError {
    fn from(err: validator::ValidationErrors) -> Self {
        ServiceError::ValidationError(err.to_string())
    }
}

impl ServiceError {
    pub fn error_code(&self) -> &str {
        match self {
            ServiceError::InvalidCredentials => "INVALID_CREDENTIALS",
            ServiceError::InvalidToken(_) => "INVALID_TOKEN",
            ServiceError::TokenExpired => "TOKEN_EXPIRED",
            ServiceError::InvalidInvitation => "INVALID_INVITATION",
            ServiceError::InvitationExpired => "INVITATION_EXPIRED",
            ServiceError::InvitationUsed => "INVITATION_USED",
            ServiceError::UsernameExists => "USERNAME_EXISTS",
            ServiceError::EmailExists => "EMAIL_EXISTS",
            ServiceError::NotFound(_) => "NOT_FOUND",
            ServiceError::ValidationError(_) => "VALIDATION_ERROR",
            ServiceError::DatabaseError(_) => "DATABASE_ERROR",
            ServiceError::InternalError => "INTERNAL_ERROR",
            ServiceError::Unauthorized => "UNAUTHORIZED",
            ServiceError::BadRequest(_) => "BAD_REQUEST",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ServiceError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            ServiceError::TokenExpired => StatusCode::UNAUTHORIZED,
            ServiceError::InvalidInvitation => StatusCode::BAD_REQUEST,
            ServiceError::InvitationExpired => StatusCode::BAD_REQUEST,
            ServiceError::InvitationUsed => StatusCode::BAD_REQUEST,
            ServiceError::UsernameExists => StatusCode::CONFLICT,
            ServiceError::EmailExists => StatusCode::CONFLICT,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl ResponseError for ServiceError {
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
