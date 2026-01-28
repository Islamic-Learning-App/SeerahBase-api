use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use std::env;

pub struct ApiKey;

impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the API key from environment
        let api_key = env::var("API_KEY").unwrap_or_default();

        // Get the key from the headers
        let provided_key = parts
            .headers
            .get("x-api-key")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        if api_key.is_empty() || provided_key != api_key {
            return Err((StatusCode::UNAUTHORIZED, "Unauthorized").into_response());
        }

        Ok(ApiKey)
    }
}
