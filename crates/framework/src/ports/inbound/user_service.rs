use async_trait::async_trait;
use crate::domain::user::{DomainError, User};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user(&self, id: &str) -> Result<User, DomainError>;
    async fn create_user(&self, name: String, email: String) -> Result<User, DomainError>;
    async fn list_users(&self) -> Result<Vec<User>, DomainError>;
}
