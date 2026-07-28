# `hb-cli` — Developer CLI Reference

`hb-cli` is the interactive scaffolding and developer tooling companion for `hexbuffer-framework`.

**Aliases:** `hb-cli` (primary) · `hexbuffer-cli` · `arch-cli`

---

## Installation

```bash
# Install from workspace source
cargo install --path crates/cli

# Verify installation
hb-cli --version
hb-cli --help
```

---

## Subcommands

### `hb-cli new`

Interactively scaffold a complete new Hexagonal Architecture Rust microservice.

```bash
hb-cli new
```

**Interactive prompts:**

| Prompt | Options |
| --- | --- |
| Project name | Text input (default: `my-rust-app`) |
| Database driver | PostgreSQL (SQLx) · SQLite (SQLx) · None (In-Memory) |
| Primary inbound adapter | Axum HTTP Server · Tonic gRPC Server · CLI Runner |
| Include Docker & Docker-Compose? | Yes / No |

**Generated structure:**

```text
<project-name>/
├── Cargo.toml
├── Dockerfile                    # (if Docker selected)
├── docker-compose.yml            # (if Docker selected)
├── migrations/
│   └── 0001_init_schema.sql
├── proto/
│   └── user.proto
└── src/
    ├── main.rs
    ├── domain/
    ├── ports/
    │   ├── inbound/
    │   └── outbound/
    ├── adapters/
    │   ├── inbound/
    │   │   ├── http/
    │   │   └── grpc/
    │   └── outbound/
    ├── config/
    └── telemetry/
```

---

### `hb-cli generate` / `hb-cli g`

Interactively generate individual Hexagonal Architecture components into the **current working directory**.

```bash
hb-cli generate
# or
hb-cli g
```

**Interactive prompts:**

| Prompt | Options |
| --- | --- |
| Component type | Domain Model · Outbound Adapter · Inbound Adapter · gRPC Service · Full Feature Slice |
| Component name | Text input (e.g. `Product`, `Order`, `Invoice`) |

**What gets generated per component:**

| Component | Generated Files |
| --- | --- |
| Domain Model | `src/domain/<name>.rs` |
| Outbound Adapter | `src/ports/outbound/<name>_repo.rs` · `src/adapters/outbound/memory_<name>.rs` · `src/adapters/outbound/postgres_<name>.rs` |
| Inbound Adapter | `src/ports/inbound/<name>_service.rs` · `src/adapters/inbound/<name>_service_impl.rs` · `src/adapters/inbound/http/<name>_handler.rs` |
| gRPC Service | `proto/<name>.proto` · `src/adapters/inbound/grpc/<name>_grpc.rs` |
| Full Feature Slice | All of the above |

---

### `hb-cli migrate`

Manage database SQL migrations for your project.

```bash
hb-cli migrate           # Interactive selection
hb-cli migrate create    # Create a new migration file
hb-cli migrate run       # Run pending migrations
hb-cli migrate status    # List all migration files
```

**`migrate create`** prompts for a migration name and generates:
```text
migrations/<timestamp>_<name>.sql
```

**`migrate run`** invokes `cargo sqlx migrate run` using `DATABASE_URL` from the environment.

---

### `hb-cli run`

Run the current Hexagonal Rust project using `cargo run`.

```bash
hb-cli run
```

Equivalent to `cargo run` from the project root.

---

## MiniJinja Template Engine

All code generation is powered by **MiniJinja** templates located in `crates/cli/src/templates/`.

| Template File | Generates |
| --- | --- |
| `cargo_toml.j2` | `Cargo.toml` |
| `main_rs.j2` | `src/main.rs` |
| `domain_model.j2` | Domain entity + error enum |
| `outbound_port.j2` | Repository trait |
| `outbound_postgres_adapter.j2` | SQLx Postgres repository |
| `outbound_memory_adapter.j2` | In-memory repository |
| `inbound_port.j2` | Service trait |
| `inbound_service_impl.j2` | Service implementation |
| `inbound_http_handler.j2` | Axum HTTP handler |
| `grpc_proto.j2` | `.proto` service definition |
| `grpc_server_adapter.j2` | Tonic gRPC server stub |
| `dockerfile.j2` | Multi-stage Dockerfile |
| `docker_compose.j2` | `docker-compose.yml` |
| `migration_sql.j2` | SQL migration file |
