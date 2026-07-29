# Engineering Roadmap

This document outlines the **Engineering Roadmap** for building a next-generation, high-performance Hexagonal Web Framework in Rust, updated for **2026 Rust ecosystem advancements** (including **Native Async Traits**, **Zero-Cost Abstractions**, and **Zero-Macro DI**).

---

## 🗺️ High-Level Framework Architecture

```text
┌───────────────────────────────────────────────────────────────────────────────────┐
│                                 INFRASTRUCTURE LAYER                              │
│  ┌───────────────────────┐   ┌────────────────────────┐   ┌────────────────────┐ │
│  │   Inbound Adapters    │   │   Outbound Adapters    │   │   Telemetry / Ops  │ │
│  │  Axum / Tonic (gRPC)  │   │ SQLx/SQLite/Postgres/  │   │ OpenTelemetry /    │ │
│  │                       │   │  Redis / In-Memory     │   │ TracingSubscriber  │ │
│  └───────────┬───────────┘   └───────────▲────────────┘   └────────────────────┘ │
└──────────────│───────────────────────────│────────────────────────────────────────┘
               │ Calls                     │ Implements
┌──────────────▼───────────────────────────┴────────────────────────────────────────┐
│                                    PORTS LAYER                                    │
│  ┌─────────────────────────────────────────────────────────────────────────────┐  │
│  │  Inbound Ports (Use Cases)            Outbound Ports (Repository Traits)    │  │
│  │  pub trait UserService                pub trait UserRepository              │  │
│  └─────────────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────▲────────────────────────────────────────┘
                                           │ Enforces Rules
┌──────────────────────────────────────────┴────────────────────────────────────────┐
│                                   DOMAIN LAYER                                    │
│  ┌─────────────────────────────────────────────────────────────────────────────┐  │
│  │  Entities, Value Objects, Domain Errors (Zero External I/O Dependencies)   │  │
│  └─────────────────────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────────────────────────┘
```

---

## 📊 Summary Progress Scoreboard

| Phase | Description | Completion |
| --- | --- | --- |
| **Phase 1** | Zero-Dependency Core & Domain Foundation | 🟡 67% (2/3) |
| **Phase 2** | Modern Adapter Suite & Dependency Injection (DI) | 🟢 83% (2.5/3) |
| **Phase 3** | Observability, Resilience, & Middleware Stack | 🟡 50% (1.5/3) |
| **Phase 4** | Developer Experience (DX) & CLI Engine (`hb-cli`) | 🟢 75% (3/4) |

---

## 📅 Detailed Phase Breakdown

### Phase 1: Zero-Dependency Core & Domain Foundation

> **Objective:** Build a domain core that is 100% decoupled from web servers, databases, or third-party async runtimes.

- [ ] **1.1 Native Async Trait Contracts**
  - Utilize native async methods in traits (Rust 2024 edition standard) to eliminate dynamic allocation overhead (`Box<dyn Future>`) from legacy macros.
  - Enforce `Send + Sync + 'static` lifetime bounds across all port contracts to guarantee multi-threaded execution in async executors.

- [x] **1.2 Domain Invariants & Strongly Typed Identifiers**
  - [x] Enforce strict constructor validation rules (name length, email syntax, password strength, email uniqueness) returning domain errors instead of panics.
  - [ ] Implement Newtype patterns (e.g., `UserId(Uuid)`) and Value Objects to eliminate primitive obsession across domain entities.

- [x] **1.3 Fault Isolation & Error Mapping**
  - [x] Define a centralized `DomainError` enum using `thiserror` (`NotFound`, `RepositoryError`, `ValidationError`, `Unauthorized`, `InvalidToken`, `Conflict`).
  - [x] Maintain strict boundary rules: external driver errors (Postgres, SQLite, PASETO, JWT) must never leak into the domain layer.

---

### Phase 2: Modern Adapter Suite & Dependency Injection (DI)

> **Objective:** Deliver swappable infrastructure adapters using the latest Tokio ecosystem standards.

- [x] **2.1 Web Inbound Adapters (Axum & Tower)**
  - [x] Implement modular Axum handlers that consume **Inbound Ports** via state extractors (`AppState`).
  - [x] Build payload validation and password security (bcrypt hashing & verification).
  - [x] Provide structured JSON error responses with standard status codes (`400`, `401`, `404`, `409`, `422`, `500`).
  - [ ] Standardize error conversion by turning `DomainError` into standard RFC 7807 Problem Details (`application/problem+json`) responses.

- [x] **2.2 Persistence Outbound Adapters (SQLx & Redis)**
  - [x] Implement PostgreSQL adapters using `sqlx` (`PostgresUserRepository`).
  - [x] Implement SQLite adapters using `sqlx` with auto-migration schema initialization (`SqliteUserRepository`).
  - [x] Provide thread-safe in-memory repository implementations (`MemoryUserRepository`) out of the box for zero-database local development and unit testing.
  - [ ] Build non-blocking Redis caching decorators wrapped around repository ports.

- [x] **2.3 Zero-Cost Dependency Injection Container**
  - [x] Implement thread-safe, lock-free DI using `Arc<dyn Trait>` and state composition in Axum (`AppState`).
  - [x] Provide factory functions and configuration-driven driver selection (`database.driver = "postgres" | "sqlite" | "memory"`) at application bootstrap (`main.rs`).

---

### Phase 3: Observability, Resilience, & Middleware Stack

> **Objective:** Ensure the framework is cloud-native, observable, and resilient under high concurrency.

- [x] **3.1 Tracing & OpenTelemetry Pipeline**
  - [x] Integrate `tracing` and `tracing-subscriber` for structured context propagation and HTTP request span tracing via `tower-http::TraceLayer`.
  - [ ] Provide ready-to-use OTLP exporters for Jaeger, Prometheus, and Grafana Tempo.

- [ ] **3.2 Fault Tolerance Middleware**
  - [x] Construct standard Tower middleware pipelines for HTTP trace logging, CORS (`tower-http`).
  - [ ] Implement rate limiting (`governor`), timeout, and circuit breaker patterns on inbound routes.

- [ ] **3.3 Graceful Lifecycle Management**
  - [ ] Implement signal listeners (`SIGTERM`/`SIGINT` via `tokio::signal`) for graceful TCP listener shutdown, flushing database connection pools and open trace spans.
  - [x] Provide `GET /health` endpoint for container health probes.

---

### Phase 4: Developer Experience (DX) & CLI Engine (`hb-cli`)

> **Objective:** Deliver standard developer tooling to scaffold projects, run hot-reloading dev servers, and generate hexagonal components interactively.

- [x] **4.1 Interactive Scaffolder (`hb-cli new`)**
  - [x] Build terminal UI workflows using `clap` and `inquire` to select project options (DB driver: Postgres/SQLite/Memory, Primary adapter: Axum/Tonic/CLI, Docker).
  - [x] Render project skeletons via MiniJinja templates embedded directly inside the CLI binary.
  - [x] Conditionally scaffold only relevant driver & adapter code files based on user prompts.

- [x] **4.2 Code Generation Subcommands (`hb-cli generate` / `hb-cli g`)**
  - [x] Generate complete vertical slices with short commands: `hb-cli g` -> generates Domain Entity, Inbound Port, Outbound Port, Postgres/SQLite/Memory Adapters, Axum Handler, and Tonic gRPC Server Adapter.
  - [x] Automatically update target `mod.rs` files (`upsert_mod_rs`) to export generated modules without manual editing.

- [x] **4.3 Integrated Dev Mode & Migration Engine (`hb-cli run` & `hb-cli migrate`)**
  - [x] `hb-cli run` command to execute project binaries via `cargo run`.
  - [x] `hb-cli migrate` subcommand to manage timestamped SQL migrations (`create`, `run`, `status`).
  - [ ] Embed file-watching capabilities (`notify`) to hot-reload and recompile binaries on save.

- [ ] **4.4 Spec-Driven Code Generation (`hb-cli spec`)**
  - [ ] Add parsing support for OpenAPI 3.1 and AsyncAPI spec files to auto-generate traits, structs, and route boilerplate automatically.
