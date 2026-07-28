use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    routing::{get, post},
};
use serde::Deserialize;
use crate::ports::inbound::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
}

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/users", post(create_user_handler).get(list_users_handler))
        .route("/users/{id}", get(get_user_handler))
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
