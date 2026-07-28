# HexBuffer Framework

HexBuffer Framework is a modular, production-ready Rust application framework built on the principles of Hexagonal Architecture (Ports and Adapters). It isolates core domain logic from external dependencies, transport layers, and databases, allowing for high testability, maintainability, and seamless infrastructure swapping.

---

## Features

- **Hexagonal Architecture**: Strict separation of concerns between core Domain logic, Inbound/Outbound Ports, and Adapter implementations.
- **Interactive Scaffolding CLI**: Includes `arch-cli`, a command-line utility for generating new projects, domain models, ports, and adapters.
- **Asynchronous Web Runtime**: Powered by Tokio and Axum for high-throughput HTTP handling.
- **Pluggable Persistence**: Built-in repository implementations using SQLx (PostgreSQL and SQLite) with automatic in-memory fallback for local development.
- **Layered Configuration**: Environment and file-based configuration parsing utilizing Figment.
- **Structured Observability**: Integrated tracing subscriber setup for structured logging and HTTP request tracing.

---

## Workspace Architecture

The workspace is organized into discrete crates within the `crates/` directory:

```text
hexbuffer-framework/
├── Cargo.toml
├── crates/
│   ├── framework/              # Core framework library and HTTP runtime
│   │   ├── src/
│   │   │   ├── domain/        # Pure business logic and domain entities
│   │   │   ├── ports/         # Trait definitions for Inbound & Outbound interfaces
│   │   │   │   ├── inbound/   # Driver ports (Use cases, application services)
│   │   │   │   └── outbound/  # Driven ports (Repository, external API traits)
│   │   │   ├── adapters/      # Implementations of Ports
│   │   │   │   ├── inbound/   # HTTP handlers, REST routes, CLI interfaces
│   │   │   │   └── outbound/  # Database repositories (Postgres, Memory)
│   │   │   ├── config/        # Environment and application configuration
│   │   │   └── telemetry/     # Tracing and logging initialization
│   │   └── Cargo.toml
│   └── cli/                    # Code generator and CLI developer tools (`arch-cli`)
│       ├── src/
│       │   ├── commands/      # CLI subcommand logic (new, generate, run)
│       │   └── templates/     # Scaffolding templates
│       └── Cargo.toml
```

---

## Data Flow Diagram

```text
[ HTTP Client / External Services ]
                 |
                 v
     +-----------------------+
     |   Inbound Adapter     |  (Axum Web Handler / REST)
     +-----------------------+
                 |
                 v
     +-----------------------+
     |     Inbound Port      |  (Service Interface Trait)
     +-----------------------+
                 |
                 v
     +-----------------------+
     |     Domain Logic      |  (Entities & Business Rules)
     +-----------------------+
                 |
                 v
     +-----------------------+
     |     Outbound Port     |  (Repository Interface Trait)
     +-----------------------+
                 |
                 v
     +-----------------------+
     |   Outbound Adapter    |  (PostgreSQL / Memory Repository)
     +-----------------------+
```

---

## Prerequisites

- **Rust**: Toolchain 1.85 or later (2024 edition support)
- **Cargo**: Standard package manager included with Rust

---

## Quick Start

### 1. Clone & Build Workspace

Clone the repository and compile all workspace crates:

```bash
cargo build --workspace
```

### 2. Run the Framework Application

To start the HTTP server with default in-memory storage fallback:

```bash
cargo run -p hexbuffer-framework
```

The server binds to `http://0.0.0.0:3000` by default.

### 3. Run the CLI Tool

The workspace includes `arch-cli` to assist with project management and code generation:

```bash
cargo run -p arch-cli -- --help
```

---

## CLI Tooling (`arch-cli`)

`arch-cli` simplifies adding new capabilities while enforcing architecture boundaries.

### Scaffolding a Project

```bash
cargo run -p arch-cli -- new
```

### Generating Architecture Components

Interactively generate domain entities, inbound service ports, outbound repository ports, and adapters:

```bash
cargo run -p arch-cli -- generate
```

Alias:

```bash
cargo run -p arch-cli -- g
```

### Executing the Project

```bash
cargo run -p arch-cli -- run
```

---

## Configuration

Configuration is managed via environment variables and loaded using `Figment`.

| Environment Variable | Default Value | Description |
|----------------------|---------------|-------------|
| `SERVER_HOST` | `0.0.0.0` | Host interface address for HTTP server |
| `SERVER_PORT` | `3000` | Port number for HTTP server |
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/hexbuffer` | Connection string for PostgreSQL database |
| `USE_MEMORY_FALLBACK` | `true` | When set to `true`, falls back to in-memory store if DB is unreachable |

---

## Testing

Run unit and integration tests across all workspace crates:

```bash
cargo test --workspace
```

---

## License

Distributed under the MIT License. See `LICENSE` for more information.
