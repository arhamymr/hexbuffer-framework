use std::fs;
use std::path::Path;
use anyhow::Result;
use inquire::{Confirm, Select, Text};
use crate::templates::CodeGenerator;

pub fn handle_new() -> Result<()> {
    println!("✨ Welcome to Hexagonal Rust Framework Generator!");

    let project_name = Text::new("What is your project name?")
        .with_default("my-rust-app")
        .prompt()?;

    let db_choice = Select::new(
        "Select a Database Driver:",
        vec!["PostgreSQL (SQLx)", "SQLite (SQLx)", "None (In-Memory)"],
    ).prompt()?;

    let primary_adapter = Select::new(
        "Select Primary (Inbound) Adapter:",
        vec!["Axum HTTP Server", "Tonic gRPC Server", "CLI Runner"],
    ).prompt()?;

    let include_docker = Confirm::new("Include Docker & Docker-Compose files?")
        .with_default(true)
        .prompt()?;

    println!("\n🚀 Scaffolding project '{}' [DB: {}, Adapter: {}, Docker: {}]...", project_name, db_choice, primary_adapter, include_docker);

    let generator = CodeGenerator::new()?;
    let root = Path::new(&project_name);

    if root.exists() {
        anyhow::bail!("Directory '{}' already exists!", project_name);
    }

    // Create folder structure
    fs::create_dir_all(root.join("src/domain"))?;
    fs::create_dir_all(root.join("src/ports/inbound"))?;
    fs::create_dir_all(root.join("src/ports/outbound"))?;
    fs::create_dir_all(root.join("src/adapters/inbound/http"))?;
    fs::create_dir_all(root.join("src/adapters/inbound/grpc"))?;
    fs::create_dir_all(root.join("src/adapters/outbound"))?;
    fs::create_dir_all(root.join("src/config"))?;
    fs::create_dir_all(root.join("src/telemetry"))?;
    fs::create_dir_all(root.join("migrations"))?;
    fs::create_dir_all(root.join("proto"))?;

    // Render & write Cargo.toml
    let cargo_toml = generator.render_cargo_toml(&project_name)?;
    fs::write(root.join("Cargo.toml"), cargo_toml)?;

    // Render & write main.rs
    let main_rs = generator.render_main_rs(&project_name)?;
    fs::write(root.join("src/main.rs"), main_rs)?;

    // Write Docker files
    if include_docker {
        let dockerfile = generator.render_dockerfile(&project_name)?;
        fs::write(root.join("Dockerfile"), dockerfile)?;

        let docker_compose = generator.render_docker_compose(&project_name)?;
        fs::write(root.join("docker-compose.yml"), docker_compose)?;
        println!("  [+] Scaffolding Docker & docker-compose.yml");
    }

    // Write initial SQL migration
    let migration_sql = generator.render_migration_sql("init_schema", "users", "0001")?;
    fs::write(root.join("migrations/0001_init_schema.sql"), migration_sql)?;

    // Write sample domain model (User)
    let domain_user = generator.render_domain_model("User")?;
    fs::write(root.join("src/domain/user.rs"), domain_user)?;
    fs::write(root.join("src/domain/mod.rs"), "pub mod user;\npub use user::*;\n")?;

    // Outbound ports & adapters
    let user_repo_port = generator.render_outbound_port("user", "User", "UserRepository")?;
    fs::write(root.join("src/ports/outbound/user_repo.rs"), user_repo_port)?;
    fs::write(root.join("src/ports/outbound/mod.rs"), "pub mod user_repo;\npub use user_repo::*;\n")?;

    let memory_adapter = generator.render_outbound_memory_adapter("user", "User", "UserRepository", "MemoryUserRepository")?;
    fs::write(root.join("src/adapters/outbound/memory_user.rs"), memory_adapter)?;

    let postgres_adapter = generator.render_outbound_postgres_adapter("user", "User", "UserRepository", "PostgresUserRepository", "users")?;
    fs::write(root.join("src/adapters/outbound/postgres_user.rs"), postgres_adapter)?;

    fs::write(root.join("src/adapters/outbound/mod.rs"), "pub mod memory_user;\npub mod postgres_user;\npub use memory_user::*;\npub use postgres_user::*;\n")?;

    // Inbound ports & adapters
    let user_service_port = generator.render_inbound_port("user", "User", "UserService")?;
    fs::write(root.join("src/ports/inbound/user_service.rs"), user_service_port)?;
    fs::write(root.join("src/ports/inbound/mod.rs"), "pub mod user_service;\npub use user_service::*;\n")?;
    fs::write(root.join("src/ports/mod.rs"), "pub mod inbound;\npub mod outbound;\n")?;

    let service_impl = generator.render_inbound_service_impl("user", "User", "UserService", "UserRepository", "UserServiceImpl")?;
    fs::write(root.join("src/adapters/inbound/user_service_impl.rs"), service_impl)?;

    let http_handler = generator.render_inbound_http_handler("user", "User", "UserService")?;
    fs::write(root.join("src/adapters/inbound/http/user_handler.rs"), http_handler)?;
    fs::write(root.join("src/adapters/inbound/http/mod.rs"), "pub mod user_handler;\npub use user_handler::*;\n")?;

    let grpc_proto = generator.render_grpc_proto("user", "User")?;
    fs::write(root.join("proto/user.proto"), grpc_proto)?;

    let grpc_server = generator.render_grpc_server_adapter("user", "User", "UserService")?;
    fs::write(root.join("src/adapters/inbound/grpc/user_grpc.rs"), grpc_server)?;
    fs::write(root.join("src/adapters/inbound/grpc/mod.rs"), "pub mod user_grpc;\npub use user_grpc::*;\n")?;

    fs::write(root.join("src/adapters/inbound/mod.rs"), "pub mod grpc;\npub mod http;\npub mod user_service_impl;\npub use user_service_impl::*;\n")?;
    fs::write(root.join("src/adapters/mod.rs"), "pub mod inbound;\npub mod outbound;\n")?;

    // Config & Telemetry
    fs::write(root.join("src/config/mod.rs"), r#"use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig { pub host: String, pub port: u16 }
impl Default for ServerConfig { fn default() -> Self { Self { host: "0.0.0.0".to_string(), port: 3000 } } }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig { pub url: String, pub max_connections: u32, pub use_memory_fallback: bool }
impl Default for DatabaseConfig { fn default() -> Self { Self { url: "postgres://localhost:5432/db".to_string(), max_connections: 5, use_memory_fallback: true } } }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig { pub token_type: String, pub token_secret: String, pub expiration_secs: i64 }
impl Default for AuthConfig { fn default() -> Self { Self { token_type: "paseto".to_string(), token_secret: "YELLOW SUBMARINE, BLACK SUBMARIN".to_string(), expiration_secs: 86400 } } }
impl AppConfig { pub fn load() -> Result<Self, String> { Ok(Self::default()) } }
"#)?;

    fs::write(root.join("src/telemetry/mod.rs"), r#"use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
pub fn init_telemetry() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry().with(filter).with(tracing_subscriber::fmt::layer()).init();
}
"#)?;

    println!("\n✅ Successfully created project '{}'!", project_name);
    println!("👉 Next steps:");
    println!("   cd {}", project_name);
    println!("   cargo run\n");

    Ok(())
}
