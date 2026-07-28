use clap::{Parser, Subcommand};
use anyhow::Result;

pub mod commands;
pub mod templates;

use commands::{handle_generate, handle_migrate, handle_new, handle_run};

#[derive(Parser)]
#[command(
    name = "hexbuffer-cli",
    about = "Interactive Hexagonal Architecture Rust Framework Generator & Tooling",
    version,
    long_about = "CLI tool enforcing Ports & Adapters (Hexagonal Architecture) in Rust with rapid scaffolding."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new Hexagonal Rust project
    New,
    /// Interactively generate domain models, ports, adapters, and gRPC services
    #[command(alias = "g")]
    Generate,
    /// Manage database migrations
    Migrate {
        #[command(subcommand)]
        action: Option<MigrateAction>,
    },
    /// Run the current project using cargo run
    Run,
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum MigrateAction {
    /// Create a new SQL migration file
    Create,
    /// Run pending database migrations
    Run,
    /// View migration status
    Status,
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New => handle_new()?,
        Commands::Generate => handle_generate()?,
        Commands::Migrate { action } => handle_migrate(action)?,
        Commands::Run => handle_run()?,
    }

    Ok(())
}
