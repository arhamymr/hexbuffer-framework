use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use crate::domain::user::{DomainError, Email, User, UserId};
use crate::ports::outbound::user_repo::UserRepository;

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn migrate(&self) -> Result<(), DomainError> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id            TEXT PRIMARY KEY,
                name          TEXT NOT NULL,
                email         TEXT NOT NULL UNIQUE,
                password_hash TEXT
            )"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<User, DomainError> {
        let row = sqlx::query("SELECT id, name, email, password_hash FROM users WHERE id = ?")
            .bind(id.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        match row {
            Some(row) => {
                let id_str: String = row.get("id");
                let email_str: String = row.get("email");
                Ok(User {
                    id: UserId::new(id_str),
                    name: row.get("name"),
                    email: Email::new(email_str)?,
                    password_hash: row.get("password_hash"),
                })
            }
            None => Err(DomainError::NotFound(id.to_string())),
        }
    }

    async fn find_by_email(&self, email: &Email) -> Result<User, DomainError> {
        let row = sqlx::query("SELECT id, name, email, password_hash FROM users WHERE email = ?")
            .bind(email.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        match row {
            Some(row) => {
                let id_str: String = row.get("id");
                let email_str: String = row.get("email");
                Ok(User {
                    id: UserId::new(id_str),
                    name: row.get("name"),
                    email: Email::new(email_str)?,
                    password_hash: row.get("password_hash"),
                })
            }
            None => Err(DomainError::NotFound(email.to_string())),
        }
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO users (id, name, email, password_hash) VALUES (?, ?, ?, ?) \
             ON CONFLICT(id) DO UPDATE SET name = excluded.name, email = excluded.email, password_hash = excluded.password_hash"
        )
        .bind(user.id.as_str())
        .bind(&user.name)
        .bind(user.email.as_str())
        .bind(&user.password_hash)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query("SELECT id, name, email, password_hash FROM users")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let mut users = Vec::with_capacity(rows.len());
        for row in rows {
            let id_str: String = row.get("id");
            let email_str: String = row.get("email");
            users.push(User {
                id: UserId::new(id_str),
                name: row.get("name"),
                email: Email::new(email_str)?,
                password_hash: row.get("password_hash"),
            });
        }

        Ok(users)
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(id.to_string()));
        }
        Ok(())
    }
}
