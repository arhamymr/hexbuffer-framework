# Ports

Ports are **Rust trait interfaces** that decouple the domain from infrastructure. There are two directions:

| Direction | Name | Role |
| --- | --- | --- |
| **Inbound** (Driving) | `UserService` | External triggers drive business logic into the domain |
| **Outbound** (Driven) | `UserRepository`, `TokenService` | Domain drives infrastructure (DB, tokens) |

---

## Inbound Ports

### `src/ports/inbound/user_service.rs`

The inbound port is the **use case interface**. Adapters (Axum HTTP, gRPC) depend on this trait to invoke business logic.

```rust
#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user(&self, id: &str) -> Result<User, DomainError>;
    async fn create_user(&self, name: String, email: String) -> Result<User, DomainError>;
    async fn list_users(&self) -> Result<Vec<User>, DomainError>;
}
```

**Implemented by:** `UserServiceImpl` in `adapters/inbound/user_service_impl.rs`

---

## Outbound Ports

### `src/ports/outbound/user_repo.rs`

The outbound repository port defines the **storage contract**. The domain uses this trait without knowing whether it's Postgres, SQLite, or an in-memory map.

```rust
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn list_all(&self) -> Result<Vec<User>, DomainError>;
}
```

**Implemented by:**
- `PostgresUserRepository` — SQLx Postgres adapter
- `MemoryUserRepository` — Thread-safe in-memory adapter

---

### `src/ports/outbound/token_service.rs`

The token service port defines the **authentication token contract**. Adapters can provide JWT, PASETO, or any future token standard.

```rust
#[async_trait]
pub trait TokenService: Send + Sync {
    async fn generate_token(&self, user: &User) -> Result<String, DomainError>;
    async fn verify_token(&self, token: &str) -> Result<Claims, DomainError>;
}
```

**Implemented by:**
- `JwtTokenService` — HMAC-SHA256 JWT adapter
- `PasetoTokenService` — PASETO V4 local encrypted token adapter *(default)*

---

## Dependency Injection

All port traits are injected as `Arc<dyn Trait>` at application startup in `main.rs`:

```rust
let user_repo: Arc<dyn UserRepository> = Arc::new(MemoryUserRepository::new());
let token_service: Arc<dyn TokenService> = Arc::new(PasetoTokenService::new(&key, 86400)?);
let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(user_repo));
```

Swapping implementations requires **only changing the concrete type** at the wiring point — no domain or port changes needed.
