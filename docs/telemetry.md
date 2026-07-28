# Telemetry

HexBuffer Framework uses the **`tracing`** ecosystem for structured logging and HTTP request span tracing.

---

## Setup

The `init_telemetry()` function is called once at application startup in `main.rs` before any other initialization:

```rust
// src/telemetry/mod.rs
pub fn init_telemetry() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,hexbuffer_framework=debug,tower_http=debug"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
```

---

## Log Levels

Control verbosity via the `RUST_LOG` environment variable:

```bash
# Default (info)
cargo run

# Show debug output for the framework
RUST_LOG=debug cargo run

# Targeted filtering
RUST_LOG=info,hexbuffer_framework=debug,tower_http=trace cargo run

# Silent
RUST_LOG=error cargo run
```

---

## HTTP Request Tracing

HTTP request tracing is added via `tower-http`'s `TraceLayer` in `main.rs`:

```rust
let app = user_routes(state).layer(TraceLayer::new_for_http());
```

This automatically logs:
- Incoming request method and URI
- Response status code
- Request duration

Example output:
```
2026-07-28T16:00:00Z INFO  request{method=POST uri=/users}: tower_http::trace: started processing request
2026-07-28T16:00:00Z INFO  request{method=POST uri=/users}: tower_http::trace: finished processing request status=201 latency=2ms
```

---

## Adding Spans in Application Code

Use `tracing` macros anywhere in your application:

```rust
use tracing::{info, warn, error, debug, instrument};

// Simple log
info!("User created: {}", user.id);

// Structured fields
info!(user_id = %user.id, email = %user.email, "User created successfully");

// Instrument async functions with automatic span tracking
#[instrument(skip(self))]
async fn create_user(&self, name: String, email: String) -> Result<User, DomainError> {
    // ...
}
```

---

## Crates

| Crate | Purpose |
| --- | --- |
| `tracing` | Span/event instrumentation macros |
| `tracing-subscriber` | Subscriber implementation (formats + filters logs) |
| `tower-http` | HTTP middleware including `TraceLayer` |
