use actix_web::{FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use validator::Validate;

use super::error_handler::{validation_failed, ErrorResponse};

/// Validated JSON extractor
///
/// Wrapper around actix_web::web::Json that automatically validates
/// the request body using the `validator` crate.
///
/// # Example
///
/// ```rust
/// use actix_web::{web, HttpResponse};
/// use serde::{Deserialize, Serialize};
/// use validator::Validate;
/// use shared_lib::middleware::ValidatedJson;
///
/// #[derive(Deserialize, Serialize, Validate)]
/// struct CreateUserRequest {
///     #[validate(length(min = 3, max = 30))]
///     username: String,
///     
///     #[validate(email)]
///     email: String,
///     
///     #[validate(length(min = 8))]
///     password: String,
/// }
///
/// async fn create_user(
///     user: ValidatedJson<CreateUserRequest>,
/// ) -> HttpResponse {
///     // user.into_inner() is guaranteed to be valid
///     let user = user.into_inner();
///     HttpResponse::Ok().json(user)
/// }
/// ```
pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    /// Unwrap the validated value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let json_fut = actix_web::web::Json::<T>::from_request(req, payload);

        Box::pin(async move {
            let json = json_fut.await?;

            // Validate the deserialized data
            json.validate().map_err(|validation_errors| {
                // Convert validation errors to our error format
                let details = validation_errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            serde_json::json!({
                                "field": field,
                                "message": error.message.as_ref()
                                    .map(|m| m.to_string())
                                    .unwrap_or_else(|| format!("Invalid value for {}", field))
                            })
                        })
                    })
                    .collect::<Vec<_>>();

                validation_failed(
                    "Request validation failed",
                    Some(serde_json::json!(details)),
                )
            })?;

            Ok(ValidatedJson(json.into_inner()))
        })
    }
}

/// Validated query parameters extractor
///
/// Wrapper around actix_web::web::Query that automatically validates
/// the query parameters using the `validator` crate.
///
/// # Example
///
/// ```rust
/// use actix_web::{web, HttpResponse};
/// use serde::Deserialize;
/// use validator::Validate;
/// use shared_lib::middleware::ValidatedQuery;
///
/// #[derive(Deserialize, Validate)]
/// struct SearchQuery {
///     #[validate(length(min = 1, max = 100))]
///     q: String,
///     
///     #[validate(range(min = 1, max = 100))]
///     limit: Option<u32>,
///     
///     #[validate(range(min = 0))]
///     offset: Option<u32>,
/// }
///
/// async fn search(
///     query: ValidatedQuery<SearchQuery>,
/// ) -> HttpResponse {
///     let q = &query.q;
///     HttpResponse::Ok().json(serde_json::json!({ "query": q }))
/// }
/// ```
pub struct ValidatedQuery<T>(pub T);

impl<T> ValidatedQuery<T> {
    /// Unwrap the validated value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for ValidatedQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromRequest for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let query_result = actix_web::web::Query::<T>::from_query(req.query_string());

        Box::pin(async move {
            let query = query_result.map_err(|e| {
                actix_web::error::ErrorBadRequest(ErrorResponse::new(
                    "bad_request",
                    format!("Invalid query parameters: {}", e),
                ))
            })?;

            // Validate the query parameters
            query.validate().map_err(|validation_errors| {
                let details = validation_errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            serde_json::json!({
                                "field": field,
                                "message": error.message.as_ref()
                                    .map(|m| m.to_string())
                                    .unwrap_or_else(|| format!("Invalid value for {}", field))
                            })
                        })
                    })
                    .collect::<Vec<_>>();

                validation_failed("Query validation failed", Some(serde_json::json!(details)))
            })?;

            Ok(ValidatedQuery(query.into_inner()))
        })
    }
}

/// Validated path parameters extractor
///
/// Wrapper around actix_web::web::Path that automatically validates
/// the path parameters using the `validator` crate.
///
/// # Example
///
/// ```rust
/// use actix_web::{web, HttpResponse};
/// use serde::Deserialize;
/// use validator::Validate;
/// use shared_lib::middleware::ValidatedPath;
/// use uuid::Uuid;
///
/// #[derive(Deserialize, Validate)]
/// struct UserPath {
///     #[validate(custom(function = "validate_uuid"))]
///     user_id: String,
/// }
///
/// fn validate_uuid(id: &str) -> Result<(), validator::ValidationError> {
///     Uuid::parse_str(id)
///         .map(|_| ())
///         .map_err(|_| validator::ValidationError::new("invalid_uuid"))
/// }
///
/// async fn get_user(
///     path: ValidatedPath<UserPath>,
/// ) -> HttpResponse {
///     let user_id = &path.user_id;
///     HttpResponse::Ok().json(serde_json::json!({ "user_id": user_id }))
/// }
/// ```
pub struct ValidatedPath<T>(pub T);

impl<T> ValidatedPath<T> {
    /// Unwrap the validated value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for ValidatedPath<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromRequest for ValidatedPath<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let path_result = actix_web::web::Path::<T>::extract(req);

        Box::pin(async move {
            let path = path_result.await.map_err(|e| {
                actix_web::error::ErrorBadRequest(ErrorResponse::new(
                    "bad_request",
                    format!("Invalid path parameters: {}", e),
                ))
            })?;

            // Validate the path parameters
            path.validate().map_err(|validation_errors| {
                let details = validation_errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            serde_json::json!({
                                "field": field,
                                "message": error.message.as_ref()
                                    .map(|m| m.to_string())
                                    .unwrap_or_else(|| format!("Invalid value for {}", field))
                            })
                        })
                    })
                    .collect::<Vec<_>>();

                validation_failed("Path validation failed", Some(serde_json::json!(details)))
            })?;

            Ok(ValidatedPath(path.into_inner()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Validate)]
    struct TestData {
        #[validate(length(min = 3))]
        name: String,

        #[validate(range(min = 18, max = 120))]
        age: u32,
    }

    #[test]
    fn test_validation_types_compile() {
        // Just ensure the types compile
        let _data = TestData {
            name: "John".to_string(),
            age: 25,
        };
    }
}
