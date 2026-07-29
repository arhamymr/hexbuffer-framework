use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// Which database driver to use.
///
/// Accepted values for `database.driver` in `App.toml` or the `DATABASE_DRIVER` env var:
/// - `"postgres"` — connect to a PostgreSQL server via `database.url`
/// - `"sqlite"`   — open/create an SQLite file via `database.url` (e.g. `"sqlite://app.db"`)
/// - `"memory"`   — in-process in-memory store (no persistence, ideal for dev/testing)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseDriver {
    Postgres,
    Sqlite,
    Memory,
}

impl Default for DatabaseDriver {
    fn default() -> Self {
        DatabaseDriver::Memory
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    /// Logical driver selection: `postgres`, `sqlite`, or `memory`.
    pub driver: DatabaseDriver,
    /// Connection URL used by the postgres and sqlite drivers.
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    pub token_type: String,
    pub token_secret: String,
    pub expiration_secs: i64,
}

impl AppConfig {
    pub fn load() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(("server.host", "0.0.0.0"))
            .merge(("server.port", 3000u16))
            .merge(("database.driver", "memory"))
            .merge(("database.url", "postgres://postgres:postgres@localhost:5432/hexbuffer"))
            .merge(("database.max_connections", 5u32))
            .merge(("auth.token_type", "paseto"))
            .merge(("auth.token_secret", "YELLOW SUBMARINE, BLACK SUBMARINE"))
            .merge(("auth.expiration_secs", 86400i64))
            .merge(Toml::file("App.toml").nested())
            .merge(Env::raw())
            .extract()
    }
}
