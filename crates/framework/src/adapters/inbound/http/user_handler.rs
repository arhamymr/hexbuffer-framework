use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use crate::adapters::inbound::http::auth_middleware::AuthUser;
use crate::ports::inbound::user_service::UserService;
use crate::ports::outbound::token_service::TokenService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
    pub token_service: Arc<dyn TokenService>,
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
}

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/users", post(create_user_handler).get(list_users_handler))
        .route("/users/{id}", get(get_user_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/me", get(me_handler))
        .with_state(state)
}

async fn get_user_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.user_service.get_user(&id).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
    }
}

async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.user_service.create_user(payload.name, payload.email).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match state.user_service.list_users().await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users = state.user_service.list_users().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = users.into_iter().find(|u| u.email == payload.email)
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User with this email not found".to_string()))?;

    let token = state.token_service.generate_token(&user).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(TokenResponse {
            token,
            token_type: "Bearer".to_string(),
        }),
    ))
}

async fn me_handler(
    auth_user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok((StatusCode::OK, Json(auth_user.claims)))
}
