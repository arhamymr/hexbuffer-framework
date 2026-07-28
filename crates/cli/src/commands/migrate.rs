use std::fs;
use std::path::Path;
use anyhow::Result;
use inquire::Text;
use crate::MigrateAction;
use crate::templates::CodeGenerator;

pub fn handle_migrate(action: Option<MigrateAction>) -> Result<()> {
    let selected_action = match action {
        Some(a) => a,
        None => {
            let choice = inquire::Select::new(
                "Select Migration Action:",
                vec!["Create Migration", "Run Pending Migrations", "Check Migration Status"],
            ).prompt()?;

            match choice {
                "Create Migration" => MigrateAction::Create,
                "Run Pending Migrations" => MigrateAction::Run,
                _ => MigrateAction::Status,
            }
        }
    };

    let migrations_dir = Path::new("migrations");
    if !migrations_dir.exists() {
        fs::create_dir_all(migrations_dir)?;
    }

    match selected_action {
        MigrateAction::Create => {
            let name = Text::new("Enter migration name (e.g. create_users_table):")
                .with_default("create_table")
                .prompt()?;

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let filename = format!("{:04}_{}.sql", timestamp, name);
            let target = migrations_dir.join(&filename);

            let generator = CodeGenerator::new()?;
            let sql_code = generator.render_migration_sql(&name, &name, &timestamp.to_string())?;

            fs::write(&target, sql_code)?;
            println!("✅ Created SQL migration file: {}", target.display());
        }
        MigrateAction::Run => {
            println!("🚀 Running SQL migrations...");
            let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/app".to_string());
            println!("Using DATABASE_URL: {}", db_url);
            println!("ℹ️ Ensure your database is running. Executing sqlx migrate run...");
            let status = std::process::Command::new("cargo")
                .args(["sqlx", "migrate", "run"])
                .status();

            if let Ok(st) = status {
                if st.success() {
                    println!("🎉 Migrations applied successfully!");
                    return Ok(());
                }
            }
            println!("💡 Note: Install 'sqlx-cli' via `cargo install sqlx-cli` to execute automated SQL migrations.");
        }
        MigrateAction::Status => {
            let entries = fs::read_dir(migrations_dir)?;
            println!("📜 Pending / Applied Migration Files in migrations/:");
            for entry in entries {
                if let Ok(e) = entry {
                    println!("  - {}", e.file_name().to_string_lossy());
                }
            }
        }
    }

    Ok(())
}
