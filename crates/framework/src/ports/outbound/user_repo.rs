use async_trait::async_trait;
use crate::domain::user::{DomainError, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn list_all(&self) -> Result<Vec<User>, DomainError>;
}
