# Docker & Container Scaffolding

HexBuffer Framework and `hb-cli` provide production-ready Docker configurations for containerizing your Hexagonal Rust microservices.

---

## Generating Docker Files

During `hb-cli new`, selecting **Yes** to the Docker prompt generates:

```text
<project>/
├── Dockerfile
└── docker-compose.yml
```

You can also run `hb-cli generate` and select the docker template at any time.

---

## Dockerfile

A **multi-stage build** Dockerfile for minimal final image size:

```dockerfile
# Stage 1: Build
FROM rust:1.84 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/<project-name> /app/server
EXPOSE 3000
CMD ["/app/server"]
```

**Key design choices:**
- Uses `rust:1.84` for the build stage (full Rust toolchain)
- Uses `debian:bookworm-slim` for the runtime stage (minimal footprint)
- Only the compiled binary is copied to the final image
- Installs `ca-certificates` and `libssl-dev` for HTTPS/TLS support

### Building the Image

```bash
docker build -t my-app:latest .
```

---

## Docker Compose

The `docker-compose.yml` wires up the app container with a Postgres database:

```yaml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgres://postgres:postgres@db:5432/<project-name>
    depends_on:
      - db

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: <project-name>
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

### Running with Docker Compose

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f app

# Stop all services
docker-compose down

# Teardown including volumes
docker-compose down -v
```

---

## Environment Variables in Containers

Pass configuration to the containerized app via environment variables (they override all defaults):

```bash
docker run -e AUTH_TOKEN_TYPE=paseto \
           -e AUTH_TOKEN_SECRET="my-32-byte-secret-key!!!!!!!!!" \
           -e DATABASE_URL="postgres://user:pass@host:5432/db" \
           -e SERVER_PORT=8080 \
           -p 8080:8080 \
           my-app:latest
```

---

## Tips

- Set `DATABASE_USE_MEMORY_FALLBACK=false` in production to enforce Postgres.
- Use Docker secrets or environment injection from CI/CD for `AUTH_TOKEN_SECRET`.
- The Postgres volume `postgres_data` persists DB data across container restarts.
