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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub use_memory_fallback: bool,
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
            .merge(("server.port", 3000))
            .merge(("database.url", "postgres://postgres:postgres@localhost:5432/hexbuffer"))
            .merge(("database.max_connections", 5))
            .merge(("database.use_memory_fallback", true))
            .merge(("auth.token_type", "paseto"))
            .merge(("auth.token_secret", "YELLOW SUBMARINE, BLACK SUBMARINE"))
            .merge(("auth.expiration_secs", 86400))
            .merge(Toml::file("App.toml").nested())
            .merge(Env::raw())
            .extract()
    }
}
