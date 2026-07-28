use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::user::{DomainError, User};
use crate::ports::inbound::user_service::UserService;
use crate::ports::outbound::user_repo::UserRepository;

pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(&self, id: &str) -> Result<User, DomainError> {
        self.repo.find_by_id(id).await
    }

    async fn create_user(&self, name: String, email: String) -> Result<User, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".to_string()));
        }
        if !email.contains('@') {
            return Err(DomainError::ValidationError("Invalid email address".to_string()));
        }

        let id = format!("usr_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0));

        let user = User { id, name, email };
        self.repo.save(&user).await?;
        Ok(user)
    }

    async fn list_users(&self) -> Result<Vec<User>, DomainError> {
        self.repo.list_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::outbound::MemoryUserRepository;

    #[tokio::test]
    async fn test_create_and_get_user() {
        let repo = Arc::new(MemoryUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let user = service.create_user("Bob".to_string(), "bob@example.com".to_string()).await.unwrap();
        assert_eq!(user.name, "Bob");

        let fetched = service.get_user(&user.id).await.unwrap();
        assert_eq!(fetched.email, "bob@example.com");
    }

    #[tokio::test]
    async fn test_validation_error() {
        let repo = Arc::new(MemoryUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let err = service.create_user("".to_string(), "invalid".to_string()).await.unwrap_err();
        assert!(matches!(err, DomainError::ValidationError(_)));
    }
}
