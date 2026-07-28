use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User with ID '{0}' not found")]
    NotFound(String),
    #[error("Database failure: {0}")]
    RepositoryError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
}
