# Shared Development Resources

Cross-cutting concerns and tools that apply to both backend and frontend development.

---

## 📚 Guides in This Section

### [Versioning Strategy](./versioning-strategy.md)

SemVer 2.0.0 versioning guidelines for all UnityPlan components.

**Contents:**

- Semantic versioning rules
- Version number format (MAJOR.MINOR.PATCH-prerelease)
- When to increment each version component
- Pre-release versioning (alpha, beta, rc)
- Changelog maintenance
- Version management across microservices

**Use when:** Releasing new versions, updating changelogs, or coordinating versions across services.

---

### [Development Tools](./development-tools.md)

Development and debugging tools for the entire platform.

**Contents:**

- Database management (Adminer for PostgreSQL)
- Cache management (Redis Commander)
- Email testing (MailHog)
- Service routing (Traefik)
- Quick health checks
- Development workflow
- Common tasks and troubleshooting

**Use when:** Setting up development environment, debugging issues, or managing infrastructure.

---

## 🎯 Quick Reference

### Version Format

```
MAJOR.MINOR.PATCH-prerelease+build

Examples:
0.1.0-alpha.1   # Initial alpha release
0.1.0-beta.1    # Beta release
0.1.0-rc.1      # Release candidate
0.1.0           # First stable release
1.0.0           # Major version 1
```

### Development Tools

| Tool | URL | Purpose |
|------|-----|---------|
| **Adminer** | <http://localhost:8080> | PostgreSQL management |
| **Redis Commander** | <http://localhost:8082> | Redis cache management |
| **MailHog** | <http://localhost:8025> | Email testing (captures all outgoing emails) |
| **Traefik** | <http://localhost:8083/dashboard/> | Service routing dashboard |

For backend-specific observability tools (Prometheus, Grafana, Jaeger), see the [Backend Observability Guide](../backend/observability.md).

---

## 🚀 Common Workflows

### Starting Development Environment

```bash
# Start all infrastructure services
docker compose -f docker-compose.dev.yml up -d

# Verify all services are running
docker compose ps

# Check health
curl http://localhost:8222/healthz  # NATS
curl http://localhost:9090/-/healthy  # Prometheus
```

### Running Backend Service

```bash
cd services/auth-service
RUST_LOG=debug cargo run
```

### Running Frontend Application

```bash
cd frontend
npm run dev
```

### Stopping Services

```bash
# Stop all services but keep data
docker compose -f docker-compose.dev.yml down

# Stop and remove all data (clean slate)
docker compose -f docker-compose.dev.yml down -v
```

---

## 📝 Cross-Service Guidelines

### Logging Standards

**Backend (Rust):**

```rust
use tracing::{info, warn, error};

info!("User logged in: {}", user_id);
warn!("Rate limit approaching: {}/{}", current, limit);
error!("Database connection failed: {}", err);
```

**Frontend (React):**

```typescript
console.log('User action:', action);
console.warn('API slow response:', duration);
console.error('API call failed:', error);
```

### Error Handling

**Backend:**

- Return structured error responses
- Include error codes and messages
- Log errors with context

**Frontend:**

- Display user-friendly messages
- Log detailed errors to console
- Report errors to monitoring (future)

### Environment Variables

**Backend (.env):**

```bash
DATABASE_URL=postgres://...
NATS_URL=nats://localhost:4222
JWT_SECRET=...
```

**Frontend (.env.development):**

```bash
VITE_AUTH_SERVICE_URL=http://localhost:8080
VITE_USER_SERVICE_URL=http://localhost:8081
```

---

## 🔧 Common Tasks

### Reset Everything

```bash
# Stop all services
docker compose -f docker-compose.dev.yml down -v

# Start fresh
docker compose -f docker-compose.dev.yml up -d

# Run migrations
cd services/shared-lib
sqlx migrate run
```

### View Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f postgres
docker compose logs -f nats
docker compose logs -f redis
```

### Database Access

```bash
# Web interface
open http://localhost:8080  # Adminer

# CLI
docker compose exec postgres psql -U unityplan -d unityplan_dev
```

### Cache Management

```bash
# Web interface
open http://localhost:8082  # Redis Commander

# CLI
docker compose exec redis redis-cli
> KEYS *
> GET user:123
> FLUSHALL
```

---

## 📚 Related Documentation

### Backend Development

- [Backend README](../backend/README.md)
- [Backend Testing Guide](../backend/testing-guide.md)
- [Backend Observability](../backend/observability.md)

### Frontend Development

- [Frontend README](../frontend/README.md)
- [Frontend Development Guide](../frontend/development-guide.md)
- [Frontend Testing Guide](../frontend/testing-guide.md)

### Architecture

- [Infrastructure Guide](../../architecture/infrastructure.md)
- [Multi-Pod Architecture](../../architecture/multi-pod-architecture.md)
- [Frontend Stack Rationale](../../architecture/frontend-stack-rationale.md)

### Project Status

- [Phase 1 Status](../../status/current/phase-1-status.md)
- [Phase 1 Checklist](../../status/current/phase-1-checklist.md)

---

## 🎯 Best Practices

### Version Control

- Commit often with descriptive messages
- Follow conventional commits format
- Keep commits atomic and focused
- Update CHANGELOG.md with each change

### Code Quality

**Backend:**

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Write tests for new features
- Document public APIs

**Frontend:**

- Run `npm run lint` before committing
- Format with Prettier (automatic on save)
- Write tests for components
- Use TypeScript strict mode

### Documentation

- Update README when adding features
- Document breaking changes in CHANGELOG
- Add inline comments for complex logic
- Keep guides up to date

---

**Questions?** See the backend or frontend README for domain-specific guidance.
