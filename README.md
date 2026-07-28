# HexBuffer Framework & `hexbuffer-cli`

**HexBuffer Framework** is a modular, production-ready Rust application framework built on the principles of **Hexagonal Architecture (Ports and Adapters)**. It isolates pure domain logic from external dependencies, transport layers, and databases, allowing for high testability, maintainability, and seamless infrastructure swapping.

Includes **`hexbuffer-cli`** (alias: `arch-cli`), an interactive CLI tool inspired by modern web/Go frameworks to scaffold microservices, domain models, ports, HTTP/gRPC adapters, Docker environments, and SQL migrations.

---

## Features

- **Hexagonal Architecture**: Strict separation between core Domain logic, Inbound/Outbound Ports, and Adapter implementations.
- **Interactive Scaffolding CLI (`hexbuffer-cli` / `arch-cli`)**: Interactive terminal prompts for generating projects, domain models, ports, adapters, gRPC services, and migrations.
- **Asynchronous HTTP & gRPC Runtimes**: Powered by Tokio, Axum for REST APIs, and Tonic for gRPC services.
- **Pluggable Persistence**: Built-in repository implementations using SQLx (PostgreSQL and SQLite) with automatic thread-safe in-memory fallback for local development.
- **Database Migrations (`hexbuffer-cli migrate`)**: Integrated CLI for generating timestamped SQL migrations and running database schema updates.
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
│   └── cli/                    # Scaffolder CLI binary (`hexbuffer-cli` / `arch-cli`)
│       ├── src/
│       │   ├── bin/           # arch-cli compatibility alias binary
│       │   ├── commands/      # Subcommands: new, generate (g), migrate, run
│       │   └── templates/     # MiniJinja code & config generation templates
│       └── Cargo.toml
```

---

## Data Flow Diagram

```text
[ HTTP Client / gRPC Client / External Services ]
                        |
                        v
         +-----------------------------+
         |      Inbound Adapter        |  (Axum REST / Tonic gRPC / CLI)
         +-----------------------------+
                        |
                        v
         +-----------------------------+
         |        Inbound Port         |  (Service Trait Interface)
         +-----------------------------+
                        |
                        v
         +-----------------------------+
         |        Domain Logic         |  (Pure Entities & Domain Errors)
         +-----------------------------+
                        |
                        v
         +-----------------------------+
         |        Outbound Port        |  (Repository Trait Interface)
         +-----------------------------+
                        |
                        v
         +-----------------------------+
         |      Outbound Adapter       |  (PostgreSQL / SQLite / Memory Repo)
         +-----------------------------+
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

### 2. Install `hexbuffer-cli` Globally

Install the CLI binary onto your local Cargo path:

```bash
cargo install --path crates/cli
```

### 3. Run the Framework Application

To start the server with default in-memory storage fallback:

```bash
cargo run -p hexbuffer-framework
```

The server binds to `http://0.0.0.0:3000` by default.

---

## CLI Tooling (`hexbuffer-cli` / `arch-cli`)

`hexbuffer-cli` enforces architectural boundaries while eliminating manual boilerplate.

### Subcommands Overview

| Command | Alias | Description |
| --- | --- | --- |
| `hexbuffer-cli new` | — | Interactively scaffold a new Hexagonal Rust microservice |
| `hexbuffer-cli generate` | `hexbuffer-cli g` | Interactively generate Domain Models, Repositories, HTTP handlers, or gRPC services |
| `hexbuffer-cli migrate` | — | Manage SQL migrations (`create`, `run`, `status`) |
| `hexbuffer-cli run` | — | Run project via `cargo run` |

### Scaffolding a New Project

```bash
hexbuffer-cli new
```

Prompts for:
- Project Name
- Database Driver (PostgreSQL, SQLite, In-Memory)
- Primary Inbound Adapter (Axum HTTP, Tonic gRPC, CLI)
- Docker & Docker-Compose inclusion

### Generating Architecture Components

```bash
hexbuffer-cli generate
# or
hexbuffer-cli g
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
hexbuffer-cli migrate create

# Run pending database migrations
hexbuffer-cli migrate run

# Check migration status
hexbuffer-cli migrate status
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
