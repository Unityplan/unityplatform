use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health,
    ),
    components(
        schemas(
            crate::models::HealthResponse,
            crate::models::DependencyStatus,
            crate::response::ApiResponse<crate::models::HealthResponse>,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
    ),
    info(
        title = "Event Service API",
        version = "0.1.0-alpha.1",
        description = "Event Service for Unity Platform"
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
