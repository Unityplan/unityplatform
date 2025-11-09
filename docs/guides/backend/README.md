# Backend Development Guide

Rust-based microservices with actix-web, sqlx, PostgreSQL, and NATS.

**Current Services:**

- `auth-service` - Authentication and authorization
- `user-service` - User profile management
- `shared-lib` - Shared utilities, database, config

---

## 📚 Guides in This Section

### [Development Plan](./development-plan.md)

Phase-by-phase backend roadmap for building all microservices.

**Contents:**

- Phase 2: Database schema & migrations
- Phase 3: Shared library development
- Phase 4: Auth service implementation
- Phase 5: User service implementation
- Future phases: Territory, Badge, Course, Forum services

**Use when:** Planning new services or understanding the overall backend architecture.

---

### [Testing Guide](./testing-guide.md)

Integration testing best practices for Rust microservices.

**Contents:**

- Critical principle: Test production configuration
- Middleware testing patterns
- Integration test setup
- Database test isolation
- Mock vs real dependencies

**Use when:** Writing tests for backend services, especially integration tests with middleware.

---

### [Database Query Patterns](./database-query-patterns.md)

Multi-pod database architecture and query patterns.

**Contents:**

- Critical rule: Runtime queries only (no compile-time macros)
- Territory schema isolation
- Dynamic schema queries
- Connection pool management
- Query performance optimization

**Use when:** Writing database queries in any service. **MUST READ** before writing SQLx queries.

---

### [Observability](./observability.md)

Monitoring, metrics, tracing, and debugging for backend services.

**Contents:**

- Prometheus metrics collection
- Grafana dashboards
- Jaeger distributed tracing
- NATS monitoring
- Performance monitoring workflow
- Alerting strategy

**Use when:** Setting up observability for a service, debugging performance issues, or creating dashboards.

---

## 🚀 Quick Start

### Prerequisites

- Rust 1.91.0+
- Docker & Docker Compose
- PostgreSQL client tools
- SQLx CLI: `cargo install sqlx-cli --features postgres`

### Start Development Environment

```bash
# Start infrastructure (PostgreSQL, NATS, Redis, etc.)
docker compose -f docker-compose.dev.yml up -d

# Verify services
docker compose ps

# Check health
curl http://localhost:8222/healthz  # NATS
```

### Run a Service Locally

```bash
# Example: Auth service
cd services/auth-service

# Run migrations (first time only)
cd ../shared-lib
sqlx migrate run
cd ../auth-service

# Run service
RUST_LOG=debug cargo run

# Service runs on http://localhost:8080
```

### Run Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# All tests with output
cargo test -- --nocapture
```

---

## 🏗️ Architecture Overview

### Microservices Pattern

Each service is:

- **Independent:** Can be deployed separately
- **Isolated:** Has its own database schema
- **Event-driven:** Communicates via NATS
- **Observable:** Exposes metrics and traces

### Technology Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Language** | Rust 1.91.0 | High-performance, safe system programming |
| **Web Framework** | actix-web 4.x | HTTP server and routing |
| **Database** | PostgreSQL 17 + TimescaleDB | Relational database with time-series |
| **ORM** | sqlx 0.7.x | Async database access |
| **Message Bus** | NATS JetStream | Inter-service communication |
| **Cache** | Redis 7.x | Session storage, caching |
| **Observability** | Prometheus + Grafana + Jaeger | Metrics, dashboards, tracing |
| **Authentication** | JWT (jsonwebtoken) | Token-based auth |
| **Configuration** | config crate + dotenvy | Environment-based config |
| **Logging** | tracing + tracing-subscriber | Structured logging |

### Service Communication

```
┌─────────────┐     NATS      ┌──────────────┐
│ Auth Service│ ◄──────────── │ User Service │
│   :8080     │               │    :8081     │
└──────┬──────┘               └──────┬───────┘
       │                             │
       │        PostgreSQL           │
       └─────────►┌──────┐◄──────────┘
                  │ :5432│
                  └──────┘
```

---

## 📝 Development Workflow

### 1. Create New Service

```bash
# Create service directory
cd services
cargo new my-service --bin

# Add to workspace Cargo.toml
[workspace]
members = ["shared-lib", "auth-service", "user-service", "my-service"]
```

### 2. Add Dependencies

```toml
[dependencies]
shared-lib = { path = "../shared-lib" }
actix-web = "4"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

### 3. Implement Service

1. Create routes in `src/routes/`
2. Create handlers in `src/handlers/`
3. Create models in `src/models/`
4. Setup in `src/main.rs`

### 4. Add Database Schema

```bash
cd services/shared-lib
sqlx migrate add add_my_table
# Edit migration file
sqlx migrate run
```

### 5. Write Tests

```bash
# Create integration test
mkdir tests
touch tests/integration_test.rs
```

### 6. Add Observability

```rust
// Add metrics endpoint
use actix_web_prom::PrometheusMetrics;

let prometheus = PrometheusMetrics::new("my_service", Some("/metrics"), None);

HttpServer::new(move || {
    App::new()
        .wrap(prometheus.clone())
        // ... routes
})
```

---

## 🔧 Common Tasks

### Run Migrations

```bash
cd services/shared-lib
sqlx migrate run
```

### Revert Last Migration

```bash
sqlx migrate revert
```

### Check SQL Queries

```bash
# Prepare for offline mode (CI/CD)
cargo sqlx prepare
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy -- -D warnings
```

### Build for Production

```bash
cargo build --release
```

---

## 🐛 Troubleshooting

### Database Connection Failed

```bash
# Check PostgreSQL is running
docker compose ps postgres

# Test connection
docker compose exec postgres psql -U unityplan -d unityplan_dev

# Check connection string in .env
cat .env | grep DATABASE_URL
```

### NATS Connection Failed

```bash
# Check NATS is running
docker compose ps nats

# Test connection
curl http://localhost:8222/varz

# Check connection string
cat .env | grep NATS_URL
```

### Compilation Errors with sqlx

If you get errors about `sqlx-data.json`:

```bash
# Regenerate query metadata
cargo sqlx prepare

# Or set DATABASE_URL and compile online
export DATABASE_URL=postgres://unityplan:password@localhost/unityplan_dev
cargo build
```

---

## 📚 Additional Resources

### Internal Documentation

- [Multi-Pod Architecture](../../architecture/multi-pod-architecture.md)
- [Infrastructure Guide](../../architecture/infrastructure.md)
- [Versioning Strategy](../shared/versioning-strategy.md)

### External Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [actix-web Documentation](https://actix.rs/docs/)
- [sqlx Documentation](https://docs.rs/sqlx/)
- [NATS Documentation](https://docs.nats.io/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)

---

## 🎯 Next Steps

1. **Read the Development Plan:** Understand the phased approach
2. **Review Testing Guide:** Learn integration testing patterns
3. **Study Database Patterns:** Critical for multi-pod architecture
4. **Set Up Observability:** Add metrics and tracing to your service
5. **Follow the Checklist:** See Phase 1 status for current progress

---

**Questions?** Check the [shared development tools guide](../shared/development-tools.md) for database management, email testing, and more.
