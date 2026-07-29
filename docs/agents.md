# AI Agents Guide

This document provides context for AI agents (Antigravity, Gemini, Claude, etc.) working on the `hexbuffer-framework` codebase.

---

## Project Summary

**HexBuffer Framework** is a Rust Cargo Workspace implementing **Hexagonal Architecture (Ports & Adapters)**.

- **`crates/framework`** (`hexbuffer-framework`): Core library with domain models, port traits, concrete adapters (HTTP, gRPC, Postgres, Memory, JWT, PASETO), config, and telemetry.
- **`crates/cli`** (`hexbuffer-cli` / `hb-cli`): Interactive scaffolding CLI using `clap`, `inquire`, and `minijinja`.

---

## Architecture Map (for Agents)

```
crates/framework/src/
├── domain/          ← NEVER import infrastructure here. Pure Rust only.
│   ├── user.rs      ← User entity, DomainError enum
│   └── auth.rs      ← Claims struct (auth token payload)
│
├── ports/           ← Trait interfaces. No implementation logic.
│   ├── inbound/
│   │   └── user_service.rs   ← UserService trait (use cases)
│   └── outbound/
│       ├── user_repo.rs      ← UserRepository trait (DB)
│       └── token_service.rs  ← TokenService trait (JWT/PASETO)
│
├── adapters/        ← Concrete implementations. Import infra crates here.
│   ├── inbound/
│   │   ├── user_service_impl.rs  ← Core business logic (implements UserService)
│   │   └── http/
│   │       ├── user_handler.rs   ← Axum HTTP routes
│   │       └── auth_middleware.rs ← AuthUser Axum extractor
│   └── outbound/
│       ├── memory_user.rs         ← In-memory UserRepository
│       ├── postgres_user.rs       ← SQLx Postgres UserRepository
│       ├── jwt_token_service.rs   ← JWT TokenService
│       └── paseto_token_service.rs ← PASETO V4 TokenService (DEFAULT)
│
├── config/mod.rs    ← AppConfig, ServerConfig, DatabaseConfig, AuthConfig (Figment)
├── telemetry/mod.rs ← tracing-subscriber init
├── lib.rs           ← Module re-exports
└── main.rs          ← DI container, server bootstrap
```

---

## Key Constraints for Agents

| Layer | May Import | Must NOT Import |
| --- | --- | --- |
| `domain/` | `serde`, `thiserror` | `axum`, `sqlx`, `tokio`, `reqwest`, anything I/O |
| `ports/` | `async_trait`, `domain` | `axum`, `sqlx`, adapter crates |
| `adapters/` | Any crate | No cross-adapter imports (outbound ≠ inbound) |
| `config/` | `figment`, `serde` | Adapter crates |
| `telemetry/` | `tracing`, `tracing-subscriber` | Adapter crates |

---

## Default State

| Setting | Default |
| --- | --- |
| Token provider | **PASETO V4 Local** (`PasetoTokenService`) |
| Database driver | **In-memory** (`MemoryUserRepository`) when `database.driver = "memory"` (default) |
| HTTP port | `3000` |
| Token expiry | `86400s` (24h) |
| Auth config default | `auth.token_type = "paseto"` |

---

## Test Suite

```bash
cargo test --workspace
```

**Current tests (12 total):**
| Test | File | What it tests |
| --- | --- | --- |
| `test_memory_repo_save_and_find` | `memory_user.rs` | In-memory repo save, find, list |
| `test_memory_repo_find_by_email` | `memory_user.rs` | In-memory repo find_by_email |
| `test_memory_repo_delete` | `memory_user.rs` | In-memory repo delete |
| `test_create_and_get_user` | `user_service_impl.rs` | Business logic service creates user with password |
| `test_validation_error` | `user_service_impl.rs` | Empty name returns ValidationError |
| `test_login_success` | `user_service_impl.rs` | Login with correct password succeeds |
| `test_login_wrong_password` | `user_service_impl.rs` | Login with wrong password returns Unauthorized |
| `test_duplicate_email_conflict` | `user_service_impl.rs` | Duplicate email returns Conflict |
| `test_update_user` | `user_service_impl.rs` | Update user name and email |
| `test_delete_user` | `user_service_impl.rs` | Delete user removes from repo |
| `test_jwt_generate_and_verify` | `jwt_token_service.rs` | JWT round-trip: generate → verify claims |
| `test_paseto_generate_and_verify` | `paseto_token_service.rs` | PASETO V4 round-trip: generate → verify claims |

---

## Adding a New Feature (Agent Checklist)

When an agent implements a new feature (e.g. new domain entity `Order`), follow this order:

1. **Domain** — Add `Order` struct and `OrderDomainError` to `src/domain/order.rs`
2. **Outbound Port** — Add `OrderRepository` trait to `src/ports/outbound/order_repo.rs`
3. **Outbound Adapters** — Implement `MemoryOrderRepository` and `PostgresOrderRepository`
4. **Inbound Port** — Add `OrderService` trait to `src/ports/inbound/order_service.rs`
5. **Inbound Adapters** — Implement `OrderServiceImpl` and Axum `order_handler.rs`
6. **Wire in `main.rs`** — Inject `Arc<dyn OrderRepository>` and `Arc<dyn OrderService>` into `AppState`
7. **Tests** — Add unit tests for the repository and service adapters
8. **Docs** — Update the relevant `docs/` files

**Never skip steps.** Never implement a concern in the wrong layer.

---

## CLI Template System (for Agents)

When extending `hb-cli` with a new generator:

1. Add a `.j2` MiniJinja template to `crates/cli/src/templates/`
2. Register the template with `env.add_template("name", include_str!("file.j2"))?` in `templates/mod.rs`
3. Add a render method to `CodeGenerator`
4. Call the render method in the appropriate `commands/` handler

---

## Common Commands for Agents

```bash
# Build all workspace crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Run framework server
cargo run -p hexbuffer-framework

# Run CLI (interactive)
cargo run -p hexbuffer-cli -- new
cargo run -p hexbuffer-cli -- generate
cargo run -p hexbuffer-cli -- migrate create

# Check for compile errors
cargo check --workspace

# Lint
cargo clippy --workspace
```

---

## External Crates Reference

| Crate | Version | Used In | Purpose |
| --- | --- | --- | --- |
| `axum` | `0.8` | Framework | HTTP web framework |
| `tokio` | `1.43` | Framework + CLI | Async runtime |
| `sqlx` | `0.8` | Framework | Async SQL + Postgres/SQLite |
| `serde` | `1.0` | Framework + CLI | Serialization |
| `jsonwebtoken` | `9.3` | Framework | JWT signing/verification |
| `pasetors` | `0.7` + `v4` feature | Framework | PASETO V4 local token encryption |
| `chrono` | `0.4` | Framework | Timestamp handling |
| `figment` | `0.10` | Framework | Hierarchical config loading |
| `tracing` | `0.1` | Framework | Instrumentation macros |
| `tracing-subscriber` | `0.3` | Framework | Log formatting + filtering |
| `tower-http` | `0.6` | Framework | HTTP middleware (TraceLayer) |
| `async-trait` | `0.1` | Framework | Async trait support |
| `thiserror` | `2.0` | Framework | Structured error derives |
| `clap` | `4.5` | CLI | Subcommand parsing |
| `inquire` | `0.7` | CLI | Interactive terminal prompts |
| `minijinja` | `2.7` | CLI | Code template rendering |
| `heck` | `0.5` | CLI | Case conversion (snake_case, PascalCase) |
| `anyhow` | `1.0` | CLI | Error propagation |
