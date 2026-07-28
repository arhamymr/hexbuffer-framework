use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::user::{DomainError, User};
use crate::ports::outbound::user_repo::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<User, DomainError> {
        let row = sqlx::query("SELECT id, name, email FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        match row {
            Some(row) => Ok(User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            }),
            None => Err(DomainError::NotFound(id.to_string())),
        }
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO users (id, name, email) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, email = $3"
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query("SELECT id, name, email FROM users")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let users = rows
            .into_iter()
            .map(|row| User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            })
            .collect();

        Ok(users)
    }
}
