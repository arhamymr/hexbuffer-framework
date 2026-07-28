# Hexbuffer Framework

**HexBuffer Framework** is a modular, production-ready Rust application framework built on the principles of **Hexagonal Architecture (Ports and Adapters)**. It isolates pure domain logic from external dependencies, transport layers, and databases, allowing for high testability, maintainability, and seamless infrastructure swapping.

Includes **`hb-cli`** (alias: `hexbuffer-cli`), a developer CLI tool to scaffold microservices, domain models, ports, HTTP/gRPC adapters, Docker environments, and SQL migrations.

---

## Features

- **Hexagonal Architecture**: Strict separation between core Domain logic, Inbound/Outbound Ports, and Adapter implementations.
- **Interactive Scaffolding CLI (`hb-cli`)**: Short & fast terminal commands for generating projects, domain models, ports, adapters, gRPC services, and migrations.
- **Asynchronous HTTP & gRPC Runtimes**: Powered by Tokio, Axum for REST APIs, and Tonic for gRPC services.
- **Pluggable Persistence**: Built-in repository implementations using SQLx (PostgreSQL and SQLite) with automatic thread-safe in-memory fallback for local development.
- **Database Migrations (`hb-cli migrate`)**: Integrated CLI for generating timestamped SQL migrations and running database schema updates.
- **Docker & Container Scaffolding**: Multi-stage `Dockerfile` and `docker-compose.yml` generation for quick deployment with PostgreSQL and Redis.
- **Layered Configuration**: Environment and file-based configuration loading via Figment.
- **Structured Observability**: Integrated `tracing` and `tracing-subscriber` setup for structured logging and HTTP request span tracing.

---

## Workspace Architecture

The workspace is organized into discrete crates within the `crates/` directory:

```text
hexbuffer-framework/
├── Cargo.toml
├── README.md
├── crates/
│   ├── framework/              # Core framework library & example HTTP runtime (`hexbuffer-framework`)
│   │   ├── src/
│   │   │   ├── domain/        # Pure business logic and domain entities (zero I/O dependencies)
│   │   │   ├── ports/         # Trait definitions for Inbound & Outbound interfaces
│   │   │   │   ├── inbound/   # Driver ports (Use cases, application services)
│   │   │   │   └── outbound/  # Driven ports (Repository, cache, external API traits)
│   │   │   ├── adapters/      # Implementations of Ports
│   │   │   │   ├── inbound/   # HTTP (Axum) handlers, gRPC (Tonic) servers, CLI runners
│   │   │   │   └── outbound/  # Database repositories (Postgres, SQLite, Memory)
│   │   │   ├── config/        # Environment and application configuration (Figment)
│   │   │   └── telemetry/     # Tracing and logging initialization
│   │   └── Cargo.toml
│   └── cli/                    # Scaffolder CLI binary (`hb-cli` / `hexbuffer-cli`)
│       ├── src/
│       │   ├── bin/           # hb-cli binary entrypoint
│       │   ├── commands/      # Subcommands: new, generate (g), migrate, run
│       │   └── templates/     # MiniJinja code & config generation templates
│       └── Cargo.toml
```

---

## Prerequisites

- **Rust**: Toolchain 1.84+ (2024 edition support)
- **Cargo**: Standard package manager included with Rust

---

## Quick Start

### 1. Build Workspace Crates

Clone the repository and compile all workspace crates:

```bash
cargo build --workspace
```

### 2. Install `hb-cli` Globally

Install the CLI binary onto your local Cargo path:

```bash
cargo install --path crates/cli
```

Now `hb-cli` is available globally in your shell!

### 3. Run the Framework Application

To start the server with default in-memory storage fallback:

```bash
cargo run -p hexbuffer-framework
```

---

## CLI Tooling (`hb-cli`)

`hb-cli` enforces architectural boundaries while eliminating manual boilerplate.

### Subcommands Overview

| Command | Alias | Description |
| --- | --- | --- |
| `hb-cli new` | — | Interactively scaffold a new Hexagonal Rust microservice |
| `hb-cli generate` | `hb-cli g` | Interactively generate Domain Models, Repositories, HTTP handlers, or gRPC services |
| `hb-cli migrate` | — | Manage SQL migrations (`create`, `run`, `status`) |
| `hb-cli run` | — | Run project via `cargo run` |

### Scaffolding a New Project

```bash
hb-cli new
```

Prompts for:
- Project Name
- Database Driver (PostgreSQL, SQLite, In-Memory)
- Primary Inbound Adapter (Axum HTTP, Tonic gRPC, CLI)
- Docker & Docker-Compose inclusion

### Generating Architecture Components

```bash
hb-cli generate
# or
hb-cli g
```

Generates:
1. **Domain Model**: Entity struct & Domain Error enum
2. **Outbound Adapter**: Repository Trait Port + Postgres & In-Memory Adapters
3. **Inbound Adapter**: Service Trait Port + Axum HTTP Handler
4. **gRPC Service**: Protobuf `.proto` spec + Tonic Server Adapter
5. **Full Feature Slice**: Generates all of the above in one command

### Database Migrations

```bash
# Create a new SQL migration file in migrations/
hb-cli migrate create

# Run pending database migrations
hb-cli migrate run

# Check migration status
hb-cli migrate status
```

---

## Testing

Run unit and integration tests across all workspace crates:

```bash
cargo test --workspace
```

---

## License

Distributed under the MIT License. See `LICENSE` for more information.
