use std::fmt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

impl UserId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Email(pub String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, DomainError> {
        let s = email.into();
        if !s.contains('@') || s.trim().is_empty() {
            return Err(DomainError::ValidationError("Invalid email address".to_string()));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Email {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: Email,
    /// Bcrypt-hashed password. `None` for users created without a password (e.g. OAuth/SSO).
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
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
    #[error("Conflict: {0}")]
    Conflict(String),
}
