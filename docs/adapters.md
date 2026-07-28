# Adapters

Adapters are **concrete implementations** of the port traits. They bridge the domain with external infrastructure (HTTP, databases, cache, gRPC).

---

## Inbound Adapters (Driving)

Inbound adapters receive external triggers and call inbound port methods.

### `UserServiceImpl` — Application Service Adapter

**File:** `src/adapters/inbound/user_service_impl.rs`

This is the **core application service** that implements the `UserService` inbound port. It contains the business rule logic (e.g. validation) and coordinates the outbound ports.

```rust
pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, name: String, email: String) -> Result<User, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".into()));
        }
        if !email.contains('@') {
            return Err(DomainError::ValidationError("Invalid email address".into()));
        }
        // generate ID, save via outbound port, return User
    }
}
```

---

### `Axum HTTP Handlers` — REST API Adapter

**File:** `src/adapters/inbound/http/user_handler.rs`

Exposes all user and auth operations as REST endpoints using Axum.

| Method | Path | Auth | Description |
| --- | --- | --- | --- |
| `GET` | `/users` | No | List all users |
| `POST` | `/users` | No | Create a new user |
| `GET` | `/users/:id` | No | Get user by ID |
| `POST` | `/auth/login` | No | Login and receive token |
| `GET` | `/auth/me` | **Yes** | Get current authenticated user claims |

The `AppState` struct wires both `UserService` and `TokenService` into handlers via Axum's `State` extractor:

```rust
#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
    pub token_service: Arc<dyn TokenService>,
}
```

---

### `AuthUser` — Authentication Middleware Extractor

**File:** `src/adapters/inbound/http/auth_middleware.rs`

An Axum `FromRequestParts` extractor. Any handler that includes `AuthUser` as a parameter is automatically **protected** — the extractor validates the `Authorization: Bearer <token>` header and injects decoded `Claims` into the handler.

```rust
// Usage in any handler:
async fn me_handler(auth_user: AuthUser) -> impl IntoResponse {
    Json(auth_user.claims)
}
```

Returns `401 Unauthorized` if the header is missing, malformed, or token verification fails.

---

## Outbound Adapters (Driven)

Outbound adapters implement outbound port traits and connect to infrastructure.

### `MemoryUserRepository`

**File:** `src/adapters/outbound/memory_user.rs`

Thread-safe, zero-dependency in-memory repository for local development and testing.

```rust
pub struct MemoryUserRepository {
    users: RwLock<HashMap<String, User>>,
}
```

- Uses `RwLock<HashMap>` for concurrent reads and exclusive writes.
- No setup required — starts empty and lives in process memory.
- **Default in `main.rs`** when `database.use_memory_fallback = true`.

---

### `PostgresUserRepository`

**File:** `src/adapters/outbound/postgres_user.rs`

Production-grade SQLx Postgres adapter implementing `UserRepository`.

```rust
pub struct PostgresUserRepository {
    pool: PgPool,
}
```

- Requires a running Postgres instance and `DATABASE_URL`.
- Uses raw SQL queries via `sqlx::query` (avoids compile-time macro limitations).
- Supports upsert via `ON CONFLICT (id) DO UPDATE`.

---

### `JwtTokenService`

**File:** `src/adapters/outbound/jwt_token_service.rs`

Implements `TokenService` using HMAC-SHA256 signed JWTs via the `jsonwebtoken` crate.

```rust
pub struct JwtTokenService {
    secret: String,
    expiration_secs: i64,
}
```

Select via config: `auth.token_type = "jwt"`.

---

### `PasetoTokenService` *(Default)*

**File:** `src/adapters/outbound/paseto_token_service.rs`

Implements `TokenService` using **PASETO V4 Local** authenticated encryption via the `pasetors` crate. PASETO tokens are encrypted (not just signed), providing stronger security guarantees than JWTs.

```rust
pub struct PasetoTokenService {
    key: SymmetricKey<V4>,
    expiration_secs: i64,
}
```

- Requires a 32-byte symmetric key (`[u8; 32]`).
- Tokens are prefixed `v4.local.` and encrypted with XChaCha20-Poly1305.
- **Default token adapter** — `auth.token_type = "paseto"`.

---

## Adapter Comparison

| Adapter | Port | Type | Use Case |
| --- | --- | --- | --- |
| `UserServiceImpl` | `UserService` | Inbound | Core business logic, input validation |
| Axum `user_handler` | — | Inbound | HTTP REST API |
| `AuthUser` extractor | — | Inbound Middleware | JWT/PASETO bearer token guard |
| `MemoryUserRepository` | `UserRepository` | Outbound | Dev/testing with no DB |
| `PostgresUserRepository` | `UserRepository` | Outbound | Production Postgres storage |
| `PasetoTokenService` | `TokenService` | Outbound | **Default** — PASETO V4 auth tokens |
| `JwtTokenService` | `TokenService` | Outbound | JWT HMAC-SHA256 auth tokens |
