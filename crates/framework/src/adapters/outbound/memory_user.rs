use std::collections::HashMap;
use std::sync::RwLock;
use async_trait::async_trait;
use crate::domain::user::{DomainError, User};
use crate::ports::outbound::user_repo::UserRepository;

#[derive(Default)]
pub struct MemoryUserRepository {
    users: RwLock<HashMap<String, User>>,
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
    async fn find_by_id(&self, id: &str) -> Result<User, DomainError> {
        let lock = self.users.read().map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        lock.get(id)
            .cloned()
            .ok_or_else(|| DomainError::NotFound(id.to_string()))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_repo_save_and_find() {
        let repo = MemoryUserRepository::new();
        let user = User {
            id: "1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };

        repo.save(&user).await.unwrap();
        let found = repo.find_by_id("1").await.unwrap();
        assert_eq!(found, user);

        let list = repo.list_all().await.unwrap();
        assert_eq!(list.len(), 1);
    }
}
