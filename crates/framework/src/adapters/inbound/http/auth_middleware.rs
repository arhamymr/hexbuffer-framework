use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use crate::adapters::inbound::http::user_handler::AppState;
use crate::domain::auth::Claims;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub claims: Claims,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

        let token = if auth_header.starts_with("Bearer ") {
            &auth_header[7..]
        } else {
            return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header format".to_string()));
        };

        let claims = state
            .token_service
            .verify_token(token)
            .await
            .map_err(|err| (StatusCode::UNAUTHORIZED, err.to_string()))?;

        Ok(AuthUser { claims })
    }
}
