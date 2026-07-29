use async_trait::async_trait;
use crate::domain::user::{DomainError, Email, User, UserId};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user(&self, id: &UserId) -> Result<User, DomainError>;
    async fn create_user(&self, name: String, email: Email, password: String) -> Result<User, DomainError>;
    async fn update_user(&self, id: &UserId, name: String, email: Email) -> Result<User, DomainError>;
    async fn delete_user(&self, id: &UserId) -> Result<(), DomainError>;
    async fn list_users(&self) -> Result<Vec<User>, DomainError>;
    async fn login(&self, email: &Email, password: &str) -> Result<User, DomainError>;
}
