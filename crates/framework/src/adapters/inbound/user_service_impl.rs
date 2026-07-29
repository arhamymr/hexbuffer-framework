use std::sync::Arc;
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::domain::user::{DomainError, Email, User, UserId};
use crate::ports::inbound::user_service::UserService;
use crate::ports::outbound::user_repo::UserRepository;

pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    fn generate_id() -> UserId {
        UserId::new(format!("usr_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)))
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(&self, id: &UserId) -> Result<User, DomainError> {
        self.repo.find_by_id(id).await
    }

    async fn create_user(&self, name: String, email: Email, password: String) -> Result<User, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".to_string()));
        }
        if password.len() < 8 {
            return Err(DomainError::ValidationError("Password must be at least 8 characters".to_string()));
        }

        // Check for existing email
        if self.repo.find_by_email(&email).await.is_ok() {
            return Err(DomainError::Conflict(format!("Email '{}' is already in use", email)));
        }

        let password_hash = hash(&password, DEFAULT_COST)
            .map_err(|e| DomainError::RepositoryError(format!("Failed to hash password: {}", e)))?;

        let user = User {
            id: Self::generate_id(),
            name,
            email,
            password_hash: Some(password_hash),
        };
        self.repo.save(&user).await?;
        Ok(user)
    }

    async fn update_user(&self, id: &UserId, name: String, email: Email) -> Result<User, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".to_string()));
        }

        let mut user = self.repo.find_by_id(id).await?;

        // If email changed, check it's not taken by another user
        if user.email != email {
            if let Ok(existing) = self.repo.find_by_email(&email).await {
                if existing.id != *id {
                    return Err(DomainError::Conflict(format!("Email '{}' is already in use", email)));
                }
            }
        }

        user.name = name;
        user.email = email;
        self.repo.save(&user).await?;
        Ok(user)
    }

    async fn delete_user(&self, id: &UserId) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }

    async fn list_users(&self) -> Result<Vec<User>, DomainError> {
        self.repo.list_all().await
    }

    async fn login(&self, email: &Email, password: &str) -> Result<User, DomainError> {
        let user = self.repo.find_by_email(email).await
            .map_err(|_| DomainError::Unauthorized("Invalid email or password".to_string()))?;

        let hash = user.password_hash.as_deref()
            .ok_or_else(|| DomainError::Unauthorized("Account has no password set".to_string()))?;

        let valid = verify(password, hash)
            .map_err(|e| DomainError::RepositoryError(format!("Password verification error: {}", e)))?;

        if !valid {
            return Err(DomainError::Unauthorized("Invalid email or password".to_string()));
        }

        Ok(user)
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

        let email = Email::new("bob@example.com").unwrap();
        let user = service.create_user("Bob".to_string(), email, "password123".to_string()).await.unwrap();
        assert_eq!(user.name, "Bob");
        assert!(user.password_hash.is_some());

        let fetched = service.get_user(&user.id).await.unwrap();
        assert_eq!(fetched.email.as_str(), "bob@example.com");
    }

    #[tokio::test]
    async fn test_validation_error() {
        let repo = Arc::new(MemoryUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let email = Email::new("valid@example.com").unwrap();
        let err = service.create_user("".to_string(), email, "password123".to_string()).await.unwrap_err();
        assert!(matches!(err, DomainError::ValidationError(_)));
    }

    #[tokio::test]
    async fn test_login_success() {
        let repo = Arc::new(MemoryUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let email = Email::new("alice@example.com").unwrap();
        service.create_user("Alice".to_string(), email.clone(), "secret123".to_string()).await.unwrap();
        let user = service.login(&email, "secret123").await.unwrap();
        assert_eq!(user.email.as_str(), "alice@example.com");
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let repo = Arc::new(MemoryUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let email = Email::new("alice@example.com").unwrap();
        service.create_user("Alice".to_string(), email.clone(), "secret123".to_string()).await.unwrap();
        let err = service.login(&email, "wrong").await.unwrap_err();
        assert!(matches!(err, DomainError::Unauthorized(_)));
    }

    #[tokio::test]
    async fn test_duplicate_email_conflict() {
        let repo = Arc::new(MemoryUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let email = Email::new("alice@example.com").unwrap();
        service.create_user("Alice".to_string(), email.clone(), "secret123".to_string()).await.unwrap();
        let err = service.create_user("Alice2".to_string(), email, "secret456".to_string()).await.unwrap_err();
        assert!(matches!(err, DomainError::Conflict(_)));
    }

    #[tokio::test]
    async fn test_update_user() {
        let repo = Arc::new(MemoryUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let email = Email::new("bob@example.com").unwrap();
        let user = service.create_user("Bob".to_string(), email, "password123".to_string()).await.unwrap();
        let new_email = Email::new("robert@example.com").unwrap();
        let updated = service.update_user(&user.id, "Robert".to_string(), new_email).await.unwrap();
        assert_eq!(updated.name, "Robert");
    }

    #[tokio::test]
    async fn test_delete_user() {
        let repo = Arc::new(MemoryUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let email = Email::new("carol@example.com").unwrap();
        let user = service.create_user("Carol".to_string(), email, "password123".to_string()).await.unwrap();
        service.delete_user(&user.id).await.unwrap();
        let err = service.get_user(&user.id).await.unwrap_err();
        assert!(matches!(err, DomainError::NotFound(_)));
    }
}
