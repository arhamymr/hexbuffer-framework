use async_trait::async_trait;
use crate::domain::auth::Claims;
use crate::domain::user::{DomainError, User};

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn generate_token(&self, user: &User) -> Result<String, DomainError>;
    async fn verify_token(&self, token: &str) -> Result<Claims, DomainError>;
}
