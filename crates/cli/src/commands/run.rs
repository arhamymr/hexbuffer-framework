use std::process::Command;
use anyhow::{Context, Result};

pub fn handle_run() -> Result<()> {
    println!("🚀 Launching application (cargo run)...");

    let status = Command::new("cargo")
        .arg("run")
        .status()
        .context("Failed to execute cargo run")?;

    if !status.success() {
        anyhow::bail!("Application exited with status code: {:?}", status.code());
    }

    Ok(())
}
