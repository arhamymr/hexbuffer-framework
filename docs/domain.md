# Domain Layer

The domain layer is the **innermost layer** of the Hexagonal Architecture. It contains pure Rust structs and enums representing business concepts with **zero external I/O dependencies**.

> **Rule:** Nothing in `src/domain/` may import from `adapters/`, `ports/`, `axum`, `sqlx`, `tokio`, or any I/O crate.

---

## Files

### `src/domain/user.rs`

Defines the core `User` entity and all domain-level errors.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User with ID '{0}' not found")]
    NotFound(String),

    #[error("Database failure: {0}")]
    RepositoryError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),
}
```

### `src/domain/auth.rs`

Defines the `Claims` struct representing the decoded contents of an authentication token.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    pub sub: String,    // User ID (subject)
    pub email: String,
    pub name: String,
    pub exp: i64,       // Expiration timestamp (Unix)
    pub iat: i64,       // Issued-at timestamp (Unix)
}
```

---

## Design Rules

| Rule | Reason |
| --- | --- |
| No `async` functions | Domain logic is synchronous and pure |
| No `Arc` or `Mutex` | No shared state in the domain |
| Derives `Clone`, `PartialEq`, `Eq` | Enables value-based equality and testability |
| All errors via `thiserror` | Enforces structured, descriptive error messages |
