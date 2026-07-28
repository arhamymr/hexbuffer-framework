# Configuration

HexBuffer Framework uses **Figment** for hierarchical configuration loading — supporting default values, TOML config files, and environment variable overrides.

---

## Configuration Sources (Priority Order)

```
Defaults (hardcoded) → App.toml → Environment Variables
```

Environment variables have the highest priority and always override file and default values.

---

## Full Configuration Schema

```rust
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

pub struct ServerConfig {
    pub host: String,   // Default: "0.0.0.0"
    pub port: u16,      // Default: 3000
}

pub struct DatabaseConfig {
    pub url: String,               // Default: "postgres://postgres:postgres@localhost:5432/hexbuffer"
    pub max_connections: u32,      // Default: 5
    pub use_memory_fallback: bool, // Default: true
}

pub struct AuthConfig {
    pub token_type: String,       // Default: "paseto"   (options: "paseto" | "jwt")
    pub token_secret: String,     // Default: "YELLOW SUBMARINE, BLACK SUBMARIN" (32 bytes)
    pub expiration_secs: i64,     // Default: 86400 (24 hours)
}
```

---

## Environment Variables

All config fields are mapped directly as environment variable names (uppercase, with `.` replaced by `_`):

| Variable | Default | Description |
| --- | --- | --- |
| `SERVER_HOST` | `0.0.0.0` | HTTP server bind address |
| `SERVER_PORT` | `3000` | HTTP server port |
| `DATABASE_URL` | `postgres://...` | Postgres connection string |
| `DATABASE_MAX_CONNECTIONS` | `5` | Database connection pool size |
| `DATABASE_USE_MEMORY_FALLBACK` | `true` | Use in-memory repo when DB is unavailable |
| `AUTH_TOKEN_TYPE` | `paseto` | Token adapter selection: `"paseto"` or `"jwt"` |
| `AUTH_TOKEN_SECRET` | *(default key)* | 32-byte secret key for token signing/encryption |
| `AUTH_EXPIRATION_SECS` | `86400` | Token TTL in seconds |

---

## TOML Config File (`App.toml`)

Create an `App.toml` in the project root to override defaults:

```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
url = "postgres://user:pass@db:5432/myapp"
use_memory_fallback = false

[auth]
token_type = "paseto"
token_secret = "my-secure-32-byte-secret-key!!"
expiration_secs = 3600
```

---

## In Code

```rust
let config = AppConfig::load()?;
println!("Binding to {}:{}", config.server.host, config.server.port);
```

The `load()` method merges all sources in priority order automatically.
