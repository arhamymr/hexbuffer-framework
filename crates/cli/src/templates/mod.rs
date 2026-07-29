use anyhow::Result;
use minijinja::{context, Environment};

pub struct CodeGenerator<'a> {
    env: Environment<'a>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new() -> Result<Self> {
        let mut env = Environment::new();

        env.add_template("cargo_toml", include_str!("cargo_toml.j2"))?;
        env.add_template("main_rs", include_str!("main_rs.j2"))?;
        env.add_template("domain_model", include_str!("domain_model.j2"))?;
        env.add_template("outbound_port", include_str!("outbound_port.j2"))?;
        env.add_template("outbound_postgres_adapter", include_str!("outbound_postgres_adapter.j2"))?;
        env.add_template("outbound_memory_adapter", include_str!("outbound_memory_adapter.j2"))?;
        env.add_template("inbound_port", include_str!("inbound_port.j2"))?;
        env.add_template("inbound_service_impl", include_str!("inbound_service_impl.j2"))?;
        env.add_template("inbound_http_handler", include_str!("inbound_http_handler.j2"))?;
        env.add_template("dockerfile", include_str!("dockerfile.j2"))?;
        env.add_template("docker_compose", include_str!("docker_compose.j2"))?;
        env.add_template("grpc_proto", include_str!("grpc_proto.j2"))?;
        env.add_template("grpc_server_adapter", include_str!("grpc_server_adapter.j2"))?;
        env.add_template("outbound_sqlite_adapter", include_str!("outbound_sqlite_adapter.j2"))?;
        env.add_template("migration_sql", include_str!("migration_sql.j2"))?;

        Ok(Self { env })
    }

    pub fn render_cargo_toml(&self, project_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("cargo_toml")?;
        Ok(tmpl.render(context! { project_name => project_name })?)
    }

    pub fn render_main_rs(&self, project_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("main_rs")?;
        Ok(tmpl.render(context! { project_name => project_name })?)
    }

    pub fn render_domain_model(&self, struct_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("domain_model")?;
        Ok(tmpl.render(context! { struct_name => struct_name })?)
    }

    pub fn render_outbound_port(&self, snake_name: &str, struct_name: &str, trait_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("outbound_port")?;
        Ok(tmpl.render(context! {
            snake_name => snake_name,
            struct_name => struct_name,
            trait_name => trait_name,
        })?)
    }

    pub fn render_outbound_postgres_adapter(&self, snake_name: &str, struct_name: &str, trait_name: &str, adapter_name: &str, table_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("outbound_postgres_adapter")?;
        Ok(tmpl.render(context! {
            snake_name => snake_name,
            struct_name => struct_name,
            trait_name => trait_name,
            adapter_name => adapter_name,
            table_name => table_name,
        })?)
    }

    pub fn render_outbound_sqlite_adapter(&self, snake_name: &str, struct_name: &str, trait_name: &str, adapter_name: &str, table_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("outbound_sqlite_adapter")?;
        Ok(tmpl.render(context! {
            snake_name => snake_name,
            struct_name => struct_name,
            trait_name => trait_name,
            adapter_name => adapter_name,
            table_name => table_name,
        })?)
    }

    pub fn render_outbound_memory_adapter(&self, snake_name: &str, struct_name: &str, trait_name: &str, adapter_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("outbound_memory_adapter")?;
        Ok(tmpl.render(context! {
            snake_name => snake_name,
            struct_name => struct_name,
            trait_name => trait_name,
            adapter_name => adapter_name,
        })?)
    }

    pub fn render_inbound_port(&self, snake_name: &str, struct_name: &str, service_trait_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("inbound_port")?;
        Ok(tmpl.render(context! {
            snake_name => snake_name,
            struct_name => struct_name,
            service_trait_name => service_trait_name,
        })?)
    }

    pub fn render_inbound_service_impl(&self, snake_name: &str, struct_name: &str, service_trait_name: &str, repo_trait_name: &str, impl_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("inbound_service_impl")?;
        Ok(tmpl.render(context! {
            snake_name => snake_name,
            struct_name => struct_name,
            service_trait_name => service_trait_name,
            repo_trait_name => repo_trait_name,
            impl_name => impl_name,
        })?)
    }

    pub fn render_inbound_http_handler(&self, snake_name: &str, struct_name: &str, service_trait_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("inbound_http_handler")?;
        Ok(tmpl.render(context! {
            snake_name => snake_name,
            struct_name => struct_name,
            service_trait_name => service_trait_name,
        })?)
    }

    pub fn render_dockerfile(&self, project_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("dockerfile")?;
        Ok(tmpl.render(context! { project_name => project_name })?)
    }

    pub fn render_docker_compose(&self, project_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("docker_compose")?;
        Ok(tmpl.render(context! { project_name => project_name })?)
    }

    pub fn render_grpc_proto(&self, snake_name: &str, struct_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("grpc_proto")?;
        Ok(tmpl.render(context! { snake_name => snake_name, struct_name => struct_name })?)
    }

    pub fn render_grpc_server_adapter(&self, snake_name: &str, struct_name: &str, service_trait_name: &str) -> Result<String> {
        let tmpl = self.env.get_template("grpc_server_adapter")?;
        Ok(tmpl.render(context! { snake_name => snake_name, struct_name => struct_name, service_trait_name => service_trait_name })?)
    }

    pub fn render_migration_sql(&self, migration_name: &str, table_name: &str, timestamp: &str) -> Result<String> {
        let tmpl = self.env.get_template("migration_sql")?;
        Ok(tmpl.render(context! { migration_name => migration_name, table_name => table_name, timestamp => timestamp })?)
    }
}
