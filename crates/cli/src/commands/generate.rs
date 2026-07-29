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
            "Outbound Adapter (Repository Port + Postgres/SQLite/Memory)",
            "Inbound Adapter (Service Port + Axum Route Handler)",
            "gRPC Service (Proto Spec + Tonic Server Adapter)",
            "Full Feature Slice (Domain + Ports + HTTP/gRPC Adapters)",
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
        c if c.starts_with("gRPC Service") => {
            generate_grpc(&generator, root, &struct_name, &snake_name)?;
        }
        _ => {
            generate_domain(&generator, root, &struct_name, &snake_name)?;
            generate_outbound(&generator, root, &struct_name, &snake_name, &table_name)?;
            generate_inbound(&generator, root, &struct_name, &snake_name)?;
            generate_grpc(&generator, root, &struct_name, &snake_name)?;
        }
    }

    println!("\n🎉 Component generation complete for '{}'!", struct_name);
    Ok(())
}

/// Append `pub mod <module>;\npub use <module>::*;\n` to a mod.rs file,
/// but only if the module declaration isn't already present.
fn upsert_mod_rs(mod_rs_path: &Path, module_name: &str) -> Result<()> {
    let existing = if mod_rs_path.exists() {
        fs::read_to_string(mod_rs_path)?
    } else {
        String::new()
    };

    let mod_line = format!("pub mod {};", module_name);
    if existing.contains(&mod_line) {
        return Ok(()); // Already registered
    }

    let addition = format!("pub mod {module_name};\npub use {module_name}::*;\n");
    fs::write(mod_rs_path, format!("{existing}{addition}"))?;
    println!("  [+] Updated mod.rs: added module '{}'", module_name);
    Ok(())
}

fn generate_domain(generator: &CodeGenerator, root: &Path, struct_name: &str, snake_name: &str) -> Result<()> {
    let domain_dir = root.join("src/domain");
    fs::create_dir_all(&domain_dir)?;

    let code = generator.render_domain_model(struct_name)?;
    let target = domain_dir.join(format!("{}.rs", snake_name));
    fs::write(&target, code)?;
    println!("  [+] Generated Domain: {}", target.display());

    upsert_mod_rs(&domain_dir.join("mod.rs"), snake_name)?;
    Ok(())
}

fn generate_outbound(generator: &CodeGenerator, root: &Path, struct_name: &str, snake_name: &str, table_name: &str) -> Result<()> {
    let repo_trait   = format!("{}Repository", struct_name);
    let pg_adapter   = format!("Postgres{}Repository", struct_name);
    let sq_adapter   = format!("Sqlite{}Repository", struct_name);
    let mem_adapter  = format!("Memory{}Repository", struct_name);

    let outbound_ports_dir    = root.join("src/ports/outbound");
    let outbound_adapters_dir = root.join("src/adapters/outbound");
    fs::create_dir_all(&outbound_ports_dir)?;
    fs::create_dir_all(&outbound_adapters_dir)?;

    // Outbound port trait
    let port_code = generator.render_outbound_port(snake_name, struct_name, &repo_trait)?;
    let port_module = format!("{}_repo", snake_name);
    let port_target = outbound_ports_dir.join(format!("{}.rs", port_module));
    fs::write(&port_target, port_code)?;
    println!("  [+] Generated Outbound Port: {}", port_target.display());
    upsert_mod_rs(&outbound_ports_dir.join("mod.rs"), &port_module)?;

    // In-memory adapter
    let mem_code = generator.render_outbound_memory_adapter(snake_name, struct_name, &repo_trait, &mem_adapter)?;
    let mem_module = format!("memory_{}", snake_name);
    let mem_target = outbound_adapters_dir.join(format!("{}.rs", mem_module));
    fs::write(&mem_target, mem_code)?;
    println!("  [+] Generated Memory Adapter: {}", mem_target.display());
    upsert_mod_rs(&outbound_adapters_dir.join("mod.rs"), &mem_module)?;

    // Postgres adapter
    let pg_code = generator.render_outbound_postgres_adapter(snake_name, struct_name, &repo_trait, &pg_adapter, table_name)?;
    let pg_module = format!("postgres_{}", snake_name);
    let pg_target = outbound_adapters_dir.join(format!("{}.rs", pg_module));
    fs::write(&pg_target, pg_code)?;
    println!("  [+] Generated Postgres Adapter: {}", pg_target.display());
    upsert_mod_rs(&outbound_adapters_dir.join("mod.rs"), &pg_module)?;

    // SQLite adapter
    let sq_code = generator.render_outbound_sqlite_adapter(snake_name, struct_name, &repo_trait, &sq_adapter, table_name)?;
    let sq_module = format!("sqlite_{}", snake_name);
    let sq_target = outbound_adapters_dir.join(format!("{}.rs", sq_module));
    fs::write(&sq_target, sq_code)?;
    println!("  [+] Generated SQLite Adapter: {}", sq_target.display());
    upsert_mod_rs(&outbound_adapters_dir.join("mod.rs"), &sq_module)?;

    Ok(())
}

fn generate_inbound(generator: &CodeGenerator, root: &Path, struct_name: &str, snake_name: &str) -> Result<()> {
    let service_trait = format!("{}Service", struct_name);
    let repo_trait    = format!("{}Repository", struct_name);
    let service_impl  = format!("{}ServiceImpl", struct_name);

    let inbound_ports_dir    = root.join("src/ports/inbound");
    let inbound_adapters_dir = root.join("src/adapters/inbound");
    let http_adapters_dir    = root.join("src/adapters/inbound/http");

    fs::create_dir_all(&inbound_ports_dir)?;
    fs::create_dir_all(&inbound_adapters_dir)?;
    fs::create_dir_all(&http_adapters_dir)?;

    // Inbound port trait
    let port_code = generator.render_inbound_port(snake_name, struct_name, &service_trait)?;
    let port_module = format!("{}_service", snake_name);
    let port_target = inbound_ports_dir.join(format!("{}.rs", port_module));
    fs::write(&port_target, port_code)?;
    println!("  [+] Generated Inbound Port: {}", port_target.display());
    upsert_mod_rs(&inbound_ports_dir.join("mod.rs"), &port_module)?;

    // Service impl adapter
    let impl_code = generator.render_inbound_service_impl(snake_name, struct_name, &service_trait, &repo_trait, &service_impl)?;
    let impl_module = format!("{}_service_impl", snake_name);
    let impl_target = inbound_adapters_dir.join(format!("{}.rs", impl_module));
    fs::write(&impl_target, impl_code)?;
    println!("  [+] Generated Service Impl Adapter: {}", impl_target.display());
    upsert_mod_rs(&inbound_adapters_dir.join("mod.rs"), &impl_module)?;

    // HTTP handler
    let http_code = generator.render_inbound_http_handler(snake_name, struct_name, &service_trait)?;
    let http_module = format!("{}_handler", snake_name);
    let http_target = http_adapters_dir.join(format!("{}.rs", http_module));
    fs::write(&http_target, http_code)?;
    println!("  [+] Generated HTTP Handler Adapter: {}", http_target.display());
    upsert_mod_rs(&http_adapters_dir.join("mod.rs"), &http_module)?;

    Ok(())
}

fn generate_grpc(generator: &CodeGenerator, root: &Path, struct_name: &str, snake_name: &str) -> Result<()> {
    let service_trait    = format!("{}Service", struct_name);
    let proto_dir        = root.join("proto");
    let grpc_adapters_dir = root.join("src/adapters/inbound/grpc");

    fs::create_dir_all(&proto_dir)?;
    fs::create_dir_all(&grpc_adapters_dir)?;

    let proto_code = generator.render_grpc_proto(snake_name, struct_name)?;
    let proto_target = proto_dir.join(format!("{}.proto", snake_name));
    fs::write(&proto_target, proto_code)?;
    println!("  [+] Generated Protobuf Spec: {}", proto_target.display());

    let grpc_code = generator.render_grpc_server_adapter(snake_name, struct_name, &service_trait)?;
    let grpc_module = format!("{}_grpc", snake_name);
    let grpc_target = grpc_adapters_dir.join(format!("{}.rs", grpc_module));
    fs::write(&grpc_target, grpc_code)?;
    println!("  [+] Generated Tonic gRPC Server Adapter: {}", grpc_target.display());
    upsert_mod_rs(&grpc_adapters_dir.join("mod.rs"), &grpc_module)?;

    Ok(())
}
