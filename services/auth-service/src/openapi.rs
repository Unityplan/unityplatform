use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health,
        crate::handlers::register,
        crate::handlers::login,
        crate::handlers::refresh,
        crate::handlers::logout,
        crate::handlers::validate_token,
    ),
    components(
        schemas(
            crate::models::RegisterRequest,
            crate::models::RegisterResponse,
            crate::models::LoginRequest,
            crate::models::LoginResponse,
            crate::models::UserInfo,
            crate::models::RefreshRequest,
            crate::models::RefreshResponse,
            crate::models::ValidateResponse,
            crate::models::HealthResponse,
            crate::response::ApiResponse<crate::models::RegisterResponse>,
            crate::response::ApiResponse<crate::models::LoginResponse>,
            crate::response::ApiResponse<crate::models::RefreshResponse>,
            crate::response::ApiResponse<crate::models::ValidateResponse>,
            crate::response::ApiResponse<crate::models::HealthResponse>,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Health", description = "Health check endpoints")
    ),
    info(
        title = "Auth Service API",
        version = "0.1.0-alpha.1",
        description = "Authentication and authorization service for Unity Platform"
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
