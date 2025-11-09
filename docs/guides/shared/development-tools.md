# Development Tools Guide

This document describes general development tools available in the UnityPlan development environment.

**Last Updated:** November 9, 2025

---

## 🗄️ Database & Storage Tools

### Adminer - PostgreSQL Management

**URL:** <http://localhost:8080>

Lightweight database management interface for PostgreSQL.

**Login:**

- System: PostgreSQL
- Server: postgres
- Username: unityplan
- Password: (from .env)
- Database: unityplan_dev

**Features:**

- SQL query execution
- Table browsing and editing
- Database schema visualization
- Export/import capabilities
- Multi-database support

**Usage:**

```sql
-- Execute queries directly
SELECT * FROM territory.users LIMIT 10;

-- View table structure
DESCRIBE territory.users;

-- Export data as CSV, JSON, XML
```

---

### Redis Commander - Redis Management

**URL:** <http://localhost:8082>

Visual Redis management interface.

**Features:**

- Key-value browsing
- Real-time monitoring
- Data editing and deletion
- TTL management
- CLI console

**Usage:**

- Browse keys by pattern (`user:*`, `session:*`)
- Inspect data structures (strings, hashes, lists, sets)
- Monitor memory usage
- Execute Redis commands via built-in CLI

---

## 📧 Email Testing

### MailHog - Email Capture

**URL:** <http://localhost:8025>  
**SMTP:** localhost:1025

MailHog captures all outgoing emails for testing - no emails are actually sent.

**Features:**

- Zero configuration required
- All emails sent to `localhost:1025` are captured
- View emails in web interface
- Search and filter emails
- In-memory storage (emails cleared on restart)

**Backend Configuration:**

```rust
// Configure SMTP in your email service
SMTP_HOST=localhost
SMTP_PORT=1025
SMTP_FROM=noreply@unityplan.local
```

**Frontend Configuration:**
Not applicable - backend sends emails.

**Usage:**

1. Configure your service to send emails to `localhost:1025`
2. Trigger email (password reset, invitation, etc.)
3. View captured email at <http://localhost:8025>
4. Click email to see HTML/text content

---

## 🔀 Service Routing

### Traefik - Reverse Proxy

**URL:** <http://localhost:8083/dashboard/>

Traefik provides dynamic service routing with a dashboard.

**Features:**

- Automatic service discovery via Docker labels
- Load balancing
- HTTP/HTTPS routing
- Middleware (authentication, rate limiting, etc.)
- Real-time configuration updates

**Ports:**

- **80:** HTTP entrypoint
- **443:** HTTPS entrypoint
- **8083:** Dashboard

**Future Use:**
When microservices are fully containerized, Traefik will automatically route traffic based on Docker labels.

**Example Docker Labels:**

```yaml
services:
  auth-service:
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.auth.rule=Host(`auth.localhost`)"
      - "traefik.http.services.auth.loadbalancer.server.port=8080"
```

---

## 🔍 Quick Health Checks

Check if all services are running:

```bash
# All services status
docker compose ps

# Backend observability
curl http://localhost:9090/-/healthy          # Prometheus
curl http://localhost:3001/api/health         # Grafana
curl http://localhost:16686/                  # Jaeger

# Message bus
curl http://localhost:8222/healthz            # NATS

# Database & cache
docker compose exec postgres psql -U unityplan -d unityplan_dev -c 'SELECT 1;'
docker compose exec redis redis-cli ping

# Email testing
curl http://localhost:8025/api/v1/messages    # MailHog
```

---

## 🛠️ Development Workflow

### Local Development Setup

1. **Start all services:**

   ```bash
   docker compose -f docker-compose.dev.yml up -d
   ```

2. **Verify services are running:**

   ```bash
   docker compose ps
   ```

3. **Access tools:**
   - Database: <http://localhost:8080> (Adminer)
   - Redis: <http://localhost:8082> (Redis Commander)
   - Email: <http://localhost:8025> (MailHog)
   - Metrics: <http://localhost:9090> (Prometheus)
   - Dashboards: <http://localhost:3001> (Grafana)
   - Tracing: <http://localhost:16686> (Jaeger)

4. **Backend development:**

   ```bash
   cd services/auth-service
   cargo run
   ```

5. **Frontend development:**

   ```bash
   cd frontend
   npm run dev
   ```

### Stopping Services

```bash
# Stop all services
docker compose -f docker-compose.dev.yml down

# Stop but keep data
docker compose -f docker-compose.dev.yml stop

# Stop and remove volumes (clean slate)
docker compose -f docker-compose.dev.yml down -v
```

---

## 🔧 Common Tasks

### Reset Database

```bash
# Drop and recreate database
docker compose exec postgres psql -U unityplan -c "DROP DATABASE unityplan_dev;"
docker compose exec postgres psql -U unityplan -c "CREATE DATABASE unityplan_dev;"

# Run migrations
cd services/shared-lib
sqlx migrate run
```

### Clear Redis Cache

```bash
# Flush all keys
docker compose exec redis redis-cli FLUSHALL

# Or use Redis Commander UI
```

### View Service Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f postgres
docker compose logs -f redis
docker compose logs -f nats

# Backend service logs (if running via cargo)
cd services/auth-service
RUST_LOG=debug cargo run
```

### Database Migrations

```bash
# Create new migration
cd services/shared-lib
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

---

## 🐛 Troubleshooting

### Port Already in Use

```bash
# Find process using port
lsof -i :5432  # PostgreSQL
lsof -i :6379  # Redis
lsof -i :4222  # NATS
lsof -i :3000  # Frontend dev server

# Kill process
kill -9 <PID>
```

### Permission Issues

```bash
# Grafana
sudo chown -R 472:472 docker/grafana-data
docker compose restart grafana

# Prometheus
sudo chown -R 65534:65534 docker/prometheus-data
docker compose restart prometheus

# PostgreSQL
sudo chown -R 999:999 docker/postgres-data
docker compose restart postgres
```

### Database Connection Failed

1. Verify PostgreSQL is running: `docker compose ps postgres`
2. Check connection string in `.env`
3. Test connection:

   ```bash
   docker compose exec postgres psql -U unityplan -d unityplan_dev
   ```

4. Check logs: `docker compose logs postgres`

### Redis Connection Failed

1. Verify Redis is running: `docker compose ps redis`
2. Test connection: `docker compose exec redis redis-cli ping`
3. Check logs: `docker compose logs redis`

### Frontend Won't Start

1. Check Node.js version: `node --version` (should be 20+)
2. Clear node_modules: `rm -rf node_modules package-lock.json && npm install`
3. Check port 3000 is free: `lsof -i :3000`
4. Check Vite config for proxy settings

---

## 📚 Additional Resources

### Backend Development

- [Backend Observability Guide](../backend/observability.md)
- [Backend Testing Guide](../backend/testing-guide.md)
- [Database Query Patterns](../backend/database-query-patterns.md)

### Frontend Development

- [Frontend Development Guide](../frontend/development-guide.md)
- [Frontend Testing Guide](../frontend/testing-guide.md)
- [State Management Guide](../frontend/state-management.md)

### General

- [Versioning Strategy](./versioning-strategy.md)
- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)

---

## 🎯 Quick Links

| Tool | URL | Purpose |
|------|-----|---------|
| **Adminer** | <http://localhost:8080> | PostgreSQL management |
| **Redis Commander** | <http://localhost:8082> | Redis management |
| **MailHog** | <http://localhost:8025> | Email testing |
| **Traefik** | <http://localhost:8083/dashboard/> | Service routing |
| **Prometheus** | <http://localhost:9090> | Metrics |
| **Grafana** | <http://localhost:3001> | Dashboards |
| **Jaeger** | <http://localhost:16686> | Tracing |
| **NATS Monitor** | <http://localhost:8222> | Message bus |

---

**For backend-specific observability tools (Prometheus, Grafana, Jaeger), see:** [Backend Observability Guide](../backend/observability.md)
