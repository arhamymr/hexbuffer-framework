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
use crate::domain::user::{DomainError, Email, UserId};
use crate::ports::inbound::user_service::UserService;
use crate::ports::outbound::token_service::TokenService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
    pub token_service: Arc<dyn TokenService>,
}

// ── Request payloads ─────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

// ── Helper: turn DomainError into a structured JSON error ────────────────────

fn domain_err(status: StatusCode, err: DomainError) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: err.to_string() }))
}

fn str_err(status: StatusCode, msg: &str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: msg.to_string() }))
}

// ── Router ───────────────────────────────────────────────────────────────────

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        // Users
        .route("/users", post(create_user_handler).get(list_users_handler))
        .route("/users/{id}", get(get_user_handler).put(update_user_handler).delete(delete_user_handler))
        // Auth
        .route("/auth/login", post(login_handler))
        .route("/auth/me", get(me_handler))
        // Health
        .route("/health", get(health_handler))
        .with_state(state)
}

// ── Handlers ─────────────────────────────────────────────────────────────────

async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            version: env!("CARGO_PKG_VERSION"),
        }),
    )
}

async fn get_user_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user_id = UserId::new(id);
    state.user_service.get_user(&user_id).await
        .map(|user| (StatusCode::OK, Json(user)))
        .map_err(|e| domain_err(StatusCode::NOT_FOUND, e))
}

async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let email = Email::new(payload.email)
        .map_err(|e| domain_err(StatusCode::UNPROCESSABLE_ENTITY, e))?;

    state.user_service.create_user(payload.name, email, payload.password).await
        .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|e| match e {
            DomainError::ValidationError(_) => domain_err(StatusCode::UNPROCESSABLE_ENTITY, e),
            DomainError::Conflict(_)        => domain_err(StatusCode::CONFLICT, e),
            _                               => domain_err(StatusCode::BAD_REQUEST, e),
        })
}

async fn update_user_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user_id = UserId::new(id);
    let email = Email::new(payload.email)
        .map_err(|e| domain_err(StatusCode::UNPROCESSABLE_ENTITY, e))?;

    state.user_service.update_user(&user_id, payload.name, email).await
        .map(|user| (StatusCode::OK, Json(user)))
        .map_err(|e| match e {
            DomainError::NotFound(_)        => domain_err(StatusCode::NOT_FOUND, e),
            DomainError::ValidationError(_) => domain_err(StatusCode::UNPROCESSABLE_ENTITY, e),
            DomainError::Conflict(_)        => domain_err(StatusCode::CONFLICT, e),
            _                               => domain_err(StatusCode::BAD_REQUEST, e),
        })
}

async fn delete_user_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user_id = UserId::new(id);
    state.user_service.delete_user(&user_id).await
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .map_err(|e| domain_err(StatusCode::NOT_FOUND, e))
}

async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    state.user_service.list_users().await
        .map(|users| (StatusCode::OK, Json(users)))
        .map_err(|e| domain_err(StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let email = Email::new(payload.email)
        .map_err(|_| str_err(StatusCode::UNAUTHORIZED, "Invalid email or password"))?;

    let user = state.user_service.login(&email, &payload.password).await
        .map_err(|_| str_err(StatusCode::UNAUTHORIZED, "Invalid email or password"))?;

    let token = state.token_service.generate_token(&user).await
        .map_err(|e| domain_err(StatusCode::INTERNAL_SERVER_ERROR, e))?;

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
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    Ok((StatusCode::OK, Json(auth_user.claims)))
}
