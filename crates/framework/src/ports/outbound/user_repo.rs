use async_trait::async_trait;
use crate::domain::user::{DomainError, Email, User, UserId};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<User, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<User, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn list_all(&self) -> Result<Vec<User>, DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
}
