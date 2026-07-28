use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

use hexbuffer_framework::{
    adapters::{
        inbound::{http::{user_routes, AppState}, UserServiceImpl},
        outbound::{JwtTokenService, MemoryUserRepository, PasetoTokenService, PostgresUserRepository},
    },
    config::AppConfig,
    ports::{
        inbound::UserService,
        outbound::{TokenService, UserRepository},
    },
    telemetry::init_telemetry,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Telemetry & Logging
    init_telemetry();
    info!("🚀 Initializing Hexagonal Architecture Application...");

    // 2. Load Configuration
    let config = AppConfig::load().unwrap_or_else(|err| {
        warn!("Failed to load config, using default settings: {}", err);
        AppConfig {
            server: hexbuffer_framework::config::ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            database: hexbuffer_framework::config::DatabaseConfig {
                url: "postgres://postgres:postgres@localhost:5432/hexbuffer".to_string(),
                max_connections: 5,
                use_memory_fallback: true,
            },
            auth: hexbuffer_framework::config::AuthConfig {
                token_type: "paseto".to_string(),
                token_secret: "YELLOW SUBMARINE, BLACK SUBMARINE".to_string(),
                expiration_secs: 86400,
            },
        }
    });

    // 3. Instantiate Outbound Repository Adapter based on configuration
    let user_repo: Arc<dyn UserRepository> = if config.database.use_memory_fallback {
        info!("📦 Using In-Memory UserRepository (Development mode)");
        Arc::new(MemoryUserRepository::new())
    } else {
        info!("🐘 Connecting to Postgres DB: {}", config.database.url);
        match sqlx::PgPool::connect(&config.database.url).await {
            Ok(pool) => Arc::new(PostgresUserRepository::new(pool)),
            Err(e) => {
                warn!("Failed to connect to Postgres ({}), falling back to MemoryUserRepository", e);
                Arc::new(MemoryUserRepository::new())
            }
        }
    };

    // 4. Instantiate Outbound Token Adapter based on configuration (PASETO or JWT)
    let token_service: Arc<dyn TokenService> = if config.auth.token_type.to_lowercase() == "jwt" {
        info!("🔑 Using JWT TokenService Outbound Adapter");
        Arc::new(JwtTokenService::new(config.auth.token_secret, config.auth.expiration_secs))
    } else {
        info!("🔐 Using PASETO TokenService Outbound Adapter");
        let mut key_bytes = [0u8; 32];
        let bytes = config.auth.token_secret.as_bytes();
        let len = bytes.len().min(32);
        key_bytes[..len].copy_from_slice(&bytes[..len]);
        Arc::new(PasetoTokenService::new(&key_bytes, config.auth.expiration_secs)?)
    };

    // 5. Wire Outbound Adapters to Inbound Port Services (Dependency Injection)
    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(user_repo));
    let state = AppState { user_service, token_service };

    // 6. Build Axum HTTP Router
    let app = user_routes(state).layer(TraceLayer::new_for_http());

    // 7. Bind listener and start server
    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("⚡ Server listening on http://{}", bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
