# Architecture

HexBuffer Framework enforces **Hexagonal Architecture** (Ports & Adapters), structurally separating your application into three concentric layers:

```
┌────────────────────────────────────────────────────────────┐
│                     Inbound Adapters                       │
│         (Axum HTTP Handlers, Tonic gRPC Services)          │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                  Inbound Ports                       │  │
│  │          (Application Service Traits)                │  │
│  │  ┌────────────────────────────────────────────────┐  │  │
│  │  │                Domain Layer                    │  │  │
│  │  │    (Pure Entities, Business Logic, Errors)     │  │  │
│  │  └────────────────────────────────────────────────┘  │  │
│  │                  Outbound Ports                      │  │
│  │       (Repository & Token Service Traits)            │  │
│  └──────────────────────────────────────────────────────┘  │
│                    Outbound Adapters                       │
│         (Postgres, SQLite, In-Memory, JWT, PASETO)         │
└────────────────────────────────────────────────────────────┘
```

---

## Workspace Layout

```text
hexbuffer-framework/
├── Cargo.toml                    # Workspace root
├── README.md
├── docs/                         # Documentation (this folder)
└── crates/
    ├── framework/                # Core library + HTTP runtime binary
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs            # Module root (re-exports)
    │       ├── main.rs           # DI container & server bootstrap
    │       ├── domain/           # Pure business models (no I/O)
    │       │   ├── mod.rs
    │       │   ├── user.rs       # User entity + DomainError
    │       │   └── auth.rs       # Claims entity
    │       ├── ports/
    │       │   ├── inbound/
    │       │   │   ├── mod.rs
    │       │   │   └── user_service.rs    # UserService trait
    │       │   └── outbound/
    │       │       ├── mod.rs
    │       │       ├── user_repo.rs       # UserRepository trait
    │       │       └── token_service.rs   # TokenService trait
    │       ├── adapters/
    │       │   ├── inbound/
    │       │   │   ├── mod.rs
    │       │   │   ├── user_service_impl.rs  # Service logic adapter
    │       │   │   └── http/
    │       │   │       ├── mod.rs
    │       │   │       ├── user_handler.rs   # Axum routes
    │       │   │       └── auth_middleware.rs # AuthUser extractor
    │       │   └── outbound/
    │       │       ├── mod.rs
    │       │       ├── memory_user.rs         # In-memory repository
    │       │       ├── postgres_user.rs       # SQLx Postgres repository
    │       │       ├── jwt_token_service.rs   # JWT adapter
    │       │       └── paseto_token_service.rs # PASETO v4 adapter
    │       ├── config/
    │       │   └── mod.rs        # Figment-based config loader
    │       └── telemetry/
    │           └── mod.rs        # tracing-subscriber setup
    └── cli/                      # hb-cli scaffolding tool
        ├── Cargo.toml
        └── src/
            ├── lib.rs            # CLI entrypoint logic
            ├── main.rs           # hb-cli binary
            ├── bin/
            │   ├── hb_cli.rs     # hb-cli alias binary
            │   └── arch_cli.rs   # arch-cli compatibility alias
            ├── commands/
            │   ├── mod.rs
            │   ├── new.rs        # Project scaffolder
            │   ├── generate.rs   # Component generator
            │   ├── migrate.rs    # Migration manager
            │   └── run.rs        # Cargo runner
            └── templates/        # MiniJinja .j2 code templates
```

---

## Data Flow

```text
HTTP Request
     │
     ▼
┌─────────────────────┐
│   Axum HTTP Handler  │  (Inbound Adapter)
│  user_handler.rs    │
└─────────┬───────────┘
          │  calls
          ▼
┌─────────────────────┐
│   UserService Trait  │  (Inbound Port)
│  user_service.rs    │
└─────────┬───────────┘
          │  implemented by
          ▼
┌─────────────────────┐
│  UserServiceImpl    │  (Business Logic / Application Service)
│  user_service_impl  │
└─────────┬───────────┘
          │  calls
          ▼
┌─────────────────────┐
│ UserRepository Trait │  (Outbound Port)
│  user_repo.rs       │
└─────────┬───────────┘
          │  implemented by
          ▼
┌──────────────────────────────────────┐
│ Postgres / Memory UserRepository     │  (Outbound Adapter)
│ postgres_user.rs / memory_user.rs   │
└──────────────────────────────────────┘
```

---

## Design Principles

| Principle | Application |
| --- | --- |
| **Dependency Rule** | Inner layers never import outer layers. Domain has zero external I/O crate imports. |
| **Dependency Inversion** | Adapters depend on port traits, not concrete implementations. |
| **Single Responsibility** | Each file/module handles exactly one concern. |
| **Open/Closed** | Adding new adapters (e.g., Redis, Kafka) requires zero domain/port changes. |
