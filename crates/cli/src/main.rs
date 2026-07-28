use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod templates;

use commands::{handle_generate, handle_new, handle_run};

#[derive(Parser)]
#[command(
    name = "arch-cli",
    about = "Interactive Rust Hexagonal Framework Generator & Tooling",
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
    /// Interactively generate domain models, ports, and adapters
    #[command(alias = "g")]
    Generate,
    /// Run the current project using cargo run
    Run,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New => handle_new()?,
        Commands::Generate => handle_generate()?,
        Commands::Run => handle_run()?,
    }

    Ok(())
}
