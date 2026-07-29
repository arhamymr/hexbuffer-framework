use std::collections::HashMap;
use std::sync::RwLock;
use async_trait::async_trait;
use crate::domain::user::{DomainError, Email, User, UserId};
use crate::ports::outbound::user_repo::UserRepository;

#[derive(Default)]
pub struct MemoryUserRepository {
    users: RwLock<HashMap<UserId, User>>,
}

impl MemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for MemoryUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<User, DomainError> {
        let lock = self.users.read().map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        lock.get(id)
            .cloned()
            .ok_or_else(|| DomainError::NotFound(id.to_string()))
    }

    async fn find_by_email(&self, email: &Email) -> Result<User, DomainError> {
        let lock = self.users.read().map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        lock.values()
            .find(|u| &u.email == email)
            .cloned()
            .ok_or_else(|| DomainError::NotFound(email.to_string()))
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut lock = self.users.write().map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        lock.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        let lock = self.users.read().map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        Ok(lock.values().cloned().collect())
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        let mut lock = self.users.write().map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        lock.remove(id).ok_or_else(|| DomainError::NotFound(id.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_repo_save_and_find() {
        let repo = MemoryUserRepository::new();
        let user = User {
            id: UserId::new("1"),
            name: "Alice".to_string(),
            email: Email::new("alice@example.com").unwrap(),
            password_hash: None,
        };

        repo.save(&user).await.unwrap();
        let found = repo.find_by_id(&UserId::new("1")).await.unwrap();
        assert_eq!(found, user);

        let list = repo.list_all().await.unwrap();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn test_memory_repo_find_by_email() {
        let repo = MemoryUserRepository::new();
        let user = User {
            id: UserId::new("2"),
            name: "Bob".to_string(),
            email: Email::new("bob@example.com").unwrap(),
            password_hash: None,
        };
        repo.save(&user).await.unwrap();
        let found = repo.find_by_email(&Email::new("bob@example.com").unwrap()).await.unwrap();
        assert_eq!(found.id, UserId::new("2"));
    }

    #[tokio::test]
    async fn test_memory_repo_delete() {
        let repo = MemoryUserRepository::new();
        let user = User {
            id: UserId::new("3"),
            name: "Carol".to_string(),
            email: Email::new("carol@example.com").unwrap(),
            password_hash: None,
        };
        repo.save(&user).await.unwrap();
        repo.delete(&UserId::new("3")).await.unwrap();
        let err = repo.find_by_id(&UserId::new("3")).await.unwrap_err();
        assert!(matches!(err, DomainError::NotFound(_)));
    }
}
