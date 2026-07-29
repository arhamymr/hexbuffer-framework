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

    let include_docker = if primary_adapter != "CLI Runner" {
        Confirm::new("Include Docker & Docker-Compose files?")
            .with_default(true)
            .prompt()?
    } else {
        false
    };

    println!("\n🚀 Scaffolding project '{}' [DB: {}, Adapter: {}, Docker: {}]...", project_name, db_choice, primary_adapter, include_docker);

    let generator = CodeGenerator::new()?;
    let root = Path::new(&project_name);

    if root.exists() {
        anyhow::bail!("Directory '{}' already exists!", project_name);
    }

    // ── Determine driver tag used in config ──────────────────────────────────
    let db_driver = match db_choice {
        d if d.starts_with("PostgreSQL") => "postgres",
        d if d.starts_with("SQLite")     => "sqlite",
        _                                => "memory",
    };

    let db_url = match db_driver {
        "postgres" => "postgres://postgres:postgres@localhost:5432/app",
        "sqlite"   => "sqlite://app.db",
        _          => "",
    };

    // ── Create folder structure ───────────────────────────────────────────────
    fs::create_dir_all(root.join("src/domain"))?;
    fs::create_dir_all(root.join("src/ports/inbound"))?;
    fs::create_dir_all(root.join("src/ports/outbound"))?;
    fs::create_dir_all(root.join("src/adapters/outbound"))?;
    fs::create_dir_all(root.join("src/config"))?;
    fs::create_dir_all(root.join("src/telemetry"))?;
    fs::create_dir_all(root.join("migrations"))?;

    if primary_adapter != "CLI Runner" {
        fs::create_dir_all(root.join("src/adapters/inbound/http"))?;
    }
    if primary_adapter == "Tonic gRPC Server" || primary_adapter == "Axum HTTP Server" {
        fs::create_dir_all(root.join("src/adapters/inbound/grpc"))?;
        fs::create_dir_all(root.join("proto"))?;
    }

    // ── Cargo.toml ───────────────────────────────────────────────────────────
    let cargo_toml = generator.render_cargo_toml(&project_name)?;
    fs::write(root.join("Cargo.toml"), cargo_toml)?;
    println!("  [+] Scaffolded Cargo.toml");

    // ── Docker files (HTTP/gRPC only) ────────────────────────────────────────
    if include_docker {
        let dockerfile = generator.render_dockerfile(&project_name)?;
        fs::write(root.join("Dockerfile"), dockerfile)?;
        let docker_compose = generator.render_docker_compose(&project_name)?;
        fs::write(root.join("docker-compose.yml"), docker_compose)?;
        println!("  [+] Scaffolded Dockerfile & docker-compose.yml");
    }

    // ── SQL migration ────────────────────────────────────────────────────────
    let migration_sql = generator.render_migration_sql("init_schema", "users", "0001")?;
    fs::write(root.join("migrations/0001_init_schema.sql"), migration_sql)?;
    println!("  [+] Scaffolded migrations/0001_init_schema.sql");

    // ── Domain model ─────────────────────────────────────────────────────────
    let domain_user = generator.render_domain_model("User")?;
    fs::write(root.join("src/domain/user.rs"), domain_user)?;
    fs::write(root.join("src/domain/mod.rs"), "pub mod user;\npub use user::*;\n")?;
    println!("  [+] Scaffolded src/domain/");

    // ── Outbound ports ───────────────────────────────────────────────────────
    let user_repo_port = generator.render_outbound_port("user", "User", "UserRepository")?;
    fs::write(root.join("src/ports/outbound/user_repo.rs"), user_repo_port)?;
    fs::write(root.join("src/ports/outbound/mod.rs"), "pub mod user_repo;\npub use user_repo::*;\n")?;
    fs::write(root.join("src/ports/mod.rs"), "pub mod inbound;\npub mod outbound;\n")?;
    println!("  [+] Scaffolded src/ports/outbound/");

    // ── Outbound adapters — conditional by DB driver ─────────────────────────
    let mut outbound_mods = String::new();

    // Always include in-memory fallback
    let memory_adapter = generator.render_outbound_memory_adapter("user", "User", "UserRepository", "MemoryUserRepository")?;
    fs::write(root.join("src/adapters/outbound/memory_user.rs"), memory_adapter)?;
    outbound_mods.push_str("pub mod memory_user;\npub use memory_user::*;\n");

    if db_driver == "postgres" {
        let postgres_adapter = generator.render_outbound_postgres_adapter("user", "User", "UserRepository", "PostgresUserRepository", "users")?;
        fs::write(root.join("src/adapters/outbound/postgres_user.rs"), postgres_adapter)?;
        outbound_mods.push_str("pub mod postgres_user;\npub use postgres_user::*;\n");
        println!("  [+] Scaffolded Postgres adapter");
    }

    if db_driver == "sqlite" {
        let sqlite_adapter = generator.render_outbound_sqlite_adapter("user", "User", "UserRepository", "SqliteUserRepository", "users")?;
        fs::write(root.join("src/adapters/outbound/sqlite_user.rs"), sqlite_adapter)?;
        outbound_mods.push_str("pub mod sqlite_user;\npub use sqlite_user::*;\n");
        println!("  [+] Scaffolded SQLite adapter");
    }

    fs::write(root.join("src/adapters/outbound/mod.rs"), &outbound_mods)?;
    println!("  [+] Scaffolded src/adapters/outbound/");

    // ── Inbound ports ────────────────────────────────────────────────────────
    let user_service_port = generator.render_inbound_port("user", "User", "UserService")?;
    fs::write(root.join("src/ports/inbound/user_service.rs"), user_service_port)?;
    fs::write(root.join("src/ports/inbound/mod.rs"), "pub mod user_service;\npub use user_service::*;\n")?;
    println!("  [+] Scaffolded src/ports/inbound/");

    // ── Inbound adapters — conditional by primary adapter ────────────────────
    let mut inbound_mods = String::new();
    let service_impl = generator.render_inbound_service_impl("user", "User", "UserService", "UserRepository", "UserServiceImpl")?;
    fs::write(root.join("src/adapters/inbound/user_service_impl.rs"), service_impl)?;
    inbound_mods.push_str("pub mod user_service_impl;\npub use user_service_impl::*;\n");

    if primary_adapter == "Axum HTTP Server" || primary_adapter == "Tonic gRPC Server" {
        let http_handler = generator.render_inbound_http_handler("user", "User", "UserService")?;
        fs::write(root.join("src/adapters/inbound/http/user_handler.rs"), http_handler)?;
        fs::write(root.join("src/adapters/inbound/http/mod.rs"), "pub mod user_handler;\npub use user_handler::*;\n")?;
        inbound_mods.push_str("pub mod http;\n");
        println!("  [+] Scaffolded Axum HTTP handler");
    }

    if primary_adapter == "Tonic gRPC Server" {
        let grpc_proto = generator.render_grpc_proto("user", "User")?;
        fs::write(root.join("proto/user.proto"), grpc_proto)?;
        let grpc_server = generator.render_grpc_server_adapter("user", "User", "UserService")?;
        fs::write(root.join("src/adapters/inbound/grpc/user_grpc.rs"), grpc_server)?;
        fs::write(root.join("src/adapters/inbound/grpc/mod.rs"), "pub mod user_grpc;\npub use user_grpc::*;\n")?;
        inbound_mods.push_str("pub mod grpc;\n");
        println!("  [+] Scaffolded Tonic gRPC adapter");
    }

    fs::write(root.join("src/adapters/inbound/mod.rs"), &inbound_mods)?;
    fs::write(root.join("src/adapters/mod.rs"), "pub mod inbound;\npub mod outbound;\n")?;
    println!("  [+] Scaffolded src/adapters/");

    // ── Config module (with correct driver) ──────────────────────────────────
    let config_code = format!(
        r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct AppConfig {{
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseDriver {{ Postgres, Sqlite, Memory }}
impl Default for DatabaseDriver {{ fn default() -> Self {{ DatabaseDriver::{driver_variant} }} }}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {{ pub host: String, pub port: u16 }}
impl Default for ServerConfig {{ fn default() -> Self {{ Self {{ host: "0.0.0.0".to_string(), port: 3000 }} }} }}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {{ pub driver: DatabaseDriver, pub url: String, pub max_connections: u32 }}
impl Default for DatabaseConfig {{ fn default() -> Self {{ Self {{ driver: DatabaseDriver::default(), url: "{db_url}".to_string(), max_connections: 5 }} }} }}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {{ pub token_type: String, pub token_secret: String, pub expiration_secs: i64 }}
impl Default for AuthConfig {{ fn default() -> Self {{ Self {{ token_type: "paseto".to_string(), token_secret: "YELLOW SUBMARINE, BLACK SUBMARIN".to_string(), expiration_secs: 86400 }} }} }}

impl AppConfig {{ pub fn load() -> Result<Self, String> {{ Ok(Self::default()) }} }}
"#,
        driver_variant = match db_driver {
            "postgres" => "Postgres",
            "sqlite"   => "Sqlite",
            _          => "Memory",
        },
        db_url = db_url,
    );
    fs::write(root.join("src/config/mod.rs"), config_code)?;
    println!("  [+] Scaffolded src/config/ (driver = {})", db_driver);

    // ── Telemetry module ─────────────────────────────────────────────────────
    fs::write(root.join("src/telemetry/mod.rs"), r#"use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
pub fn init_telemetry() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry().with(filter).with(tracing_subscriber::fmt::layer()).init();
}
"#)?;
    println!("  [+] Scaffolded src/telemetry/");

    // ── main.rs ──────────────────────────────────────────────────────────────
    let main_rs = generator.render_main_rs(&project_name)?;
    fs::write(root.join("src/main.rs"), main_rs)?;
    println!("  [+] Scaffolded src/main.rs");

    println!("\n✅ Successfully created project '{}'!", project_name);
    println!("👉 Next steps:");
    println!("   cd {}", project_name);
    if db_driver == "postgres" {
        println!("   # Start Postgres and set DATABASE_URL, then:");
    } else if db_driver == "sqlite" {
        println!("   # SQLite file will be created automatically at app.db");
    }
    println!("   cargo run\n");

    Ok(())
}
