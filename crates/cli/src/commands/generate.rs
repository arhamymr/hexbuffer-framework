use std::fs;
use std::path::Path;
use anyhow::Result;
use inquire::{Select, Text};
use heck::{ToPascalCase, ToSnakeCase};
use crate::templates::CodeGenerator;

pub fn handle_generate() -> Result<()> {
    let component_type = Select::new(
        "What component would you like to generate?",
        vec![
            "Domain Model (Entity & Errors)",
            "Outbound Adapter (Repository Port + Postgres/Memory)",
            "Inbound Adapter (Service Port + Axum Route Handler)",
            "Full Feature Slice (Domain + Ports + Adapters)",
        ],
    ).prompt()?;

    let name = Text::new("Enter component name (e.g. Product, Order, Invoice):")
        .with_default("Product")
        .prompt()?;

    let struct_name = name.to_pascal_case();
    let snake_name = name.to_snake_case();
    let table_name = format!("{}s", snake_name);

    let generator = CodeGenerator::new()?;
    let root = Path::new(".");

    match component_type {
        c if c.starts_with("Domain Model") => {
            generate_domain(&generator, root, &struct_name, &snake_name)?;
        }
        c if c.starts_with("Outbound Adapter") => {
            generate_outbound(&generator, root, &struct_name, &snake_name, &table_name)?;
        }
        c if c.starts_with("Inbound Adapter") => {
            generate_inbound(&generator, root, &struct_name, &snake_name)?;
        }
        _ => {
            generate_domain(&generator, root, &struct_name, &snake_name)?;
            generate_outbound(&generator, root, &struct_name, &snake_name, &table_name)?;
            generate_inbound(&generator, root, &struct_name, &snake_name)?;
        }
    }

    println!("\n🎉 Component generation complete for '{}'!", struct_name);
    Ok(())
}

fn generate_domain(generator: &CodeGenerator, root: &Path, struct_name: &str, snake_name: &str) -> Result<()> {
    let domain_dir = root.join("src/domain");
    fs::create_dir_all(&domain_dir)?;

    let code = generator.render_domain_model(struct_name)?;
    let target = domain_dir.join(format!("{}.rs", snake_name));
    fs::write(&target, code)?;
    println!("  [+] Generated Domain: {}", target.display());
    Ok(())
}

fn generate_outbound(generator: &CodeGenerator, root: &Path, struct_name: &str, snake_name: &str, table_name: &str) -> Result<()> {
    let repo_trait = format!("{}Repository", struct_name);
    let pg_adapter = format!("Postgres{}Repository", struct_name);
    let mem_adapter = format!("Memory{}Repository", struct_name);

    let outbound_ports_dir = root.join("src/ports/outbound");
    let outbound_adapters_dir = root.join("src/adapters/outbound");
    fs::create_dir_all(&outbound_ports_dir)?;
    fs::create_dir_all(&outbound_adapters_dir)?;

    let port_code = generator.render_outbound_port(snake_name, struct_name, &repo_trait)?;
    let port_target = outbound_ports_dir.join(format!("{}_repo.rs", snake_name));
    fs::write(&port_target, port_code)?;
    println!("  [+] Generated Outbound Port: {}", port_target.display());

    let mem_code = generator.render_outbound_memory_adapter(snake_name, struct_name, &repo_trait, &mem_adapter)?;
    let mem_target = outbound_adapters_dir.join(format!("memory_{}.rs", snake_name));
    fs::write(&mem_target, mem_code)?;
    println!("  [+] Generated Memory Adapter: {}", mem_target.display());

    let pg_code = generator.render_outbound_postgres_adapter(snake_name, struct_name, &repo_trait, &pg_adapter, table_name)?;
    let pg_target = outbound_adapters_dir.join(format!("postgres_{}.rs", snake_name));
    fs::write(&pg_target, pg_code)?;
    println!("  [+] Generated Postgres Adapter: {}", pg_target.display());

    Ok(())
}

fn generate_inbound(generator: &CodeGenerator, root: &Path, struct_name: &str, snake_name: &str) -> Result<()> {
    let service_trait = format!("{}Service", struct_name);
    let repo_trait = format!("{}Repository", struct_name);
    let service_impl = format!("{}ServiceImpl", struct_name);

    let inbound_ports_dir = root.join("src/ports/inbound");
    let inbound_adapters_dir = root.join("src/adapters/inbound");
    let http_adapters_dir = root.join("src/adapters/inbound/http");

    fs::create_dir_all(&inbound_ports_dir)?;
    fs::create_dir_all(&inbound_adapters_dir)?;
    fs::create_dir_all(&http_adapters_dir)?;

    let port_code = generator.render_inbound_port(snake_name, struct_name, &service_trait)?;
    let port_target = inbound_ports_dir.join(format!("{}_service.rs", snake_name));
    fs::write(&port_target, port_code)?;
    println!("  [+] Generated Inbound Port: {}", port_target.display());

    let impl_code = generator.render_inbound_service_impl(snake_name, struct_name, &service_trait, &repo_trait, &service_impl)?;
    let impl_target = inbound_adapters_dir.join(format!("{}_service_impl.rs", snake_name));
    fs::write(&impl_target, impl_code)?;
    println!("  [+] Generated Service Impl Adapter: {}", impl_target.display());

    let http_code = generator.render_inbound_http_handler(snake_name, struct_name, &service_trait)?;
    let http_target = http_adapters_dir.join(format!("{}_handler.rs", snake_name));
    fs::write(&http_target, http_code)?;
    println!("  [+] Generated HTTP Handler Adapter: {}", http_target.display());

    Ok(())
}
