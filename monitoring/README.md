# Monitoring & Observability

Unity Platform monitoring infrastructure and development dashboard.

## 📁 Directory Structure

```
monitoring/
├── README.md                    # This file
└── dev-dashboard/
    ├── app/                     # React + Vite dashboard frontend
    │   ├── src/
    │   ├── public/
    │   └── package.json
    └── backend/                 # Rust monitoring service
        ├── src/
        │   └── main.rs
        └── Cargo.toml
```

## 🎯 Development Dashboard

**Location:** `dev-dashboard/`  
**URL:** <http://localhost:8888>  
**Purpose:** Real-time monitoring dashboard with resource stats for all services

**Architecture:**

- **Frontend**: React 19 + Vite + shadcn/ui (port 8888)
- **Backend**: Rust monitoring service with Docker API integration (port 8090)

**Features:**

- **Issues Tab** (default): Compact view showing only services with problems
- **Dashboard Tab**: Visual service cards grouped by category
- **All Services Tab**: Complete table with resource monitoring
  - CPU usage percentage
  - Memory usage (MB and %)
  - Network I/O (RX/TX in MB)
  - Disk I/O (Read/Write in MB)
- Real-time status updates every 30 seconds
- Theme toggle (light/dark mode)

**Access:**

```bash
# Start dashboard and backend
./scripts/start.sh

# Or manually start services
docker compose -f docker-compose.dev.yml up -d dev-dashboard
cd monitoring/dev-dashboard/backend && cargo run --release

# Access dashboard
open http://localhost:8888
```

**Monitored Services:**

**Application:**

- Frontend (React + Vite) - <http://localhost:5173>

**Backend Services:**

- Auth Service - <http://localhost:8001/api/v1/health>
- User Service - <http://localhost:8002/api/v1/health>

**Infrastructure:**

- PostgreSQL - Port 5432 (via Adminer: <http://localhost:8080>)
- Redis - Port 6379 (via Redis Commander: <http://localhost:8082>)
- NATS - Port 4222
- IPFS - <http://localhost:8081/webui>

**Development Tools:**

- Adminer (Database UI) - <http://localhost:8080>
- Redis Commander - <http://localhost:8082>
- Forgejo (Git) - <http://localhost:3000>
- Docker Registry - <http://localhost:5000/v2/_catalog>
- MailHog (Email testing) - <http://localhost:8025>
- Traefik Dashboard - <http://localhost:8083/dashboard/>

**Observability:**

- Grafana - <http://localhost:3001> (admin/admin)
- Prometheus - <http://localhost:9090>
- Jaeger (Tracing) - <http://localhost:16686>
- PostgreSQL Exporter - <http://localhost:9187/metrics>
- Redis Exporter - <http://localhost:9121/metrics>
- NATS Exporter - <http://localhost:7777/metrics>
- Node Exporter - <http://localhost:9100/metrics>
- cAdvisor - <http://localhost:8089>

## 🔧 Monitoring Backend Service

**Location:** `dev-dashboard/backend/`  
**Language:** Rust  
**Port:** 8090  
**Container:** `dev-monitoring-backend`  
**Docker Socket:** Requires read access to `/var/run/docker.sock`

**Features:**

- Docker API integration via `bollard` crate
- Real-time container stats collection
- HTTP health checking for non-containerized services
- CORS-enabled REST API
- Standalone service (no shared-lib dependencies)

### Running the Backend

#### Docker (Recommended - Production Ready)

The backend runs as a container with Docker socket access:

```bash
# Build and start (included in ./scripts/start.sh)
docker compose -f docker-compose.dev.yml up -d monitoring-backend

# Or rebuild after code changes
docker compose -f docker-compose.dev.yml build monitoring-backend
docker compose -f docker-compose.dev.yml up -d monitoring-backend

# Check status
docker ps | grep monitoring-backend
curl http://localhost:8090/health

# View logs
docker logs -f dev-monitoring-backend

# Stop
docker compose -f docker-compose.dev.yml stop monitoring-backend
```

**Container Details:**

- **Image:** Built from `monitoring/dev-dashboard/backend/Dockerfile`
- **Build:** Multi-stage (Rust 1.83-slim → Debian bookworm-slim)
- **Binary:** Standalone executable (no shared-lib dependencies)
- **Socket mount:** `/var/run/docker.sock:/var/run/docker.sock:ro`
- **Networks:** `global-net`, `mesh-network`
- **Restart policy:** `unless-stopped`

#### Local Development (Alternative)

For rapid development iteration without Docker rebuilds:

For faster development cycles, run directly on the host:

```bash
# Navigate to backend
cd monitoring/dev-dashboard/backend

# Run in foreground (recommended for development)
PORT=8090 cargo run --release

# Or run in background
PORT=8090 cargo run --release > /tmp/monitoring-backend.log 2>&1 &

# Stop background process
pkill -f monitoring-service
```

**Local Requirements:**

- Rust toolchain installed
- Docker socket at `/var/run/docker.sock` (Linux default)
- Access permissions to Docker socket (user in `docker` group)

**API Endpoints:**

```bash
# Health check
curl http://localhost:8090/health

# Get all services status with resource stats
curl http://localhost:8090/api/v1/services | jq
```

**Response Format:**

```json
{
  "services": [
    {
      "name": "PostgreSQL",
      "status": "operational",
      "container_state": "running",
      "health": "healthy",
      "resources": {
        "cpu_usage_percent": 0.05,
        "memory_usage_mb": 152.0,
        "memory_limit_mb": 15993.0,
        "memory_percent": 0.95,
        "network_rx_mb": 6.5,
        "network_tx_mb": 37.8,
        "block_read_mb": 10.3,
        "block_write_mb": 6.8
      }
    }
  ],
  "timestamp": 1731609600
}
```

**Build & Run:**

```bash
# Docker (production-like)
docker compose -f docker-compose.dev.yml up -d monitoring-backend

# Local development
cd monitoring/dev-dashboard/backend
cargo build --release
PORT=8090 ./target/release/monitoring-service
```

## 📊 Monitoring Stack

Monitoring services are defined in `docker-compose.monitoring.yml` (project root).

### Grafana

**Purpose:** Metrics visualization and dashboards  
**URL:** <http://localhost:3001>  
**Credentials:** admin/admin (default)

**Pre-provisioned Dashboards:**

- Multi-Pod Overview - Global view of all territory pods
- Pod Denmark Overview - Denmark pod metrics

**Datasources:**

- Prometheus (automatically configured)

**Configuration:** `docker/grafana/provisioning/`

---

### Prometheus

**Purpose:** Metrics collection and time-series database  
**URL:** <http://localhost:9090>

**Scrape Targets:**

- Service metrics exporters (auth, user, badge, territory)
- PostgreSQL exporter
- Redis exporter
- NATS exporter
- Node exporter

**Configuration:** `docker/prometheus/prometheus.yml`

---

### Jaeger

**Purpose:** Distributed tracing  
**URL:** <http://localhost:16686>

**Traces:** Service-to-service communication, request flows

---

### Traefik

**Purpose:** Reverse proxy and load balancer  
**Dashboard:** <http://localhost:8080> (if enabled)

**Configuration:** `docker/traefik/traefik.yml`

## 🔧 Customization

### Dashboard Frontend

**Location:** `dev-dashboard/app/`

**Development:**

```bash
cd monitoring/dev-dashboard/app
npm install
npm run dev  # Dev server on port 5173
npm run build  # Build for production
```

**Theme Customization:**

Edit `src/index.css` to modify color theme (currently using soft green/sage theme).

**Add Services:**

Edit `src/lib/services-api.ts` to add new services to the monitoring list.

### Monitoring Backend

**Location:** `dev-dashboard/backend/`

**Add Containers to Monitor:**

Edit `src/main.rs`, update the `monitored_containers` vector:

```rust
let monitored_containers = vec![
    ("container-name", "Display Name"),
    // Add more...
];
```

**Add HTTP Services:**

Add to the `get_services_status` function:

```rust
services.push(check_http_service("Service Name", "http://localhost:PORT/health").await);
```

### Grafana Dashboards

**Add new dashboard:**

1. Create JSON file in `docker/grafana/provisioning/dashboards/`
2. Restart Grafana: `docker compose -f docker-compose.monitoring.yml restart grafana`

**Export existing dashboard:**

1. Open dashboard in Grafana UI
2. Click "Share" → "Export" → "Save to file"
3. Copy JSON to `docker/grafana/provisioning/dashboards/`

### Prometheus Config

Edit `docker/prometheus/prometheus.yml` to add/modify scrape targets.

**Apply changes:**

```bash
docker compose -f docker-compose.monitoring.yml restart prometheus
```

## 🚀 Quick Start

### Start Development Dashboard

```bash
# Start all services including dashboard
./scripts/start.sh

# Or start just the dashboard container
docker compose -f docker-compose.dev.yml up -d dev-dashboard

# Start monitoring backend
cd monitoring/dev-dashboard/backend
PORT=8090 cargo run --release &
```

### Access Services

1. **Dev Dashboard:** <http://localhost:8888> - Main monitoring interface
2. **Backend API:** <http://localhost:8090/api/v1/services> - Raw monitoring data
3. **Grafana:** <http://localhost:3001> (login: admin/admin)
4. **Prometheus:** <http://localhost:9090>
5. **Jaeger:** <http://localhost:16686>

### Check Status

```bash
# Overall system status
./scripts/status.sh

# Check monitoring backend
curl http://localhost:8090/health

# Check dashboard container
docker ps | grep dev-dashboard
docker logs dev-dashboard
```

### View Logs

```bash
# Dashboard frontend logs
docker logs -f dev-dashboard

# Monitoring backend logs
# (if running in foreground, logs appear in terminal)

# Grafana logs
docker logs -f monitoring-grafana

# Prometheus logs
docker logs -f monitoring-prometheus

# All monitoring logs
docker compose -f docker-compose.monitoring.yml logs -f
```

### Stop Services

```bash
# Stop monitoring backend
pkill -f monitoring-service

# Stop dashboard
docker compose -f docker-compose.dev.yml stop dev-dashboard

# Stop all monitoring
docker compose -f docker-compose.monitoring.yml down
```

## 📚 Metrics Documentation

### Dashboard Resource Metrics

The dev-dashboard displays real-time resource usage from Docker containers:

**CPU Usage:** Percentage of CPU capacity used (multi-core aware)  
**Memory:** MB used and percentage of container limit  
**Network I/O:** Cumulative RX/TX in megabytes  
**Disk I/O:** Cumulative read/write in megabytes

**Data Source:** Docker stats API via bollard Rust crate  
**Update Frequency:** Every 30 seconds (frontend polling)  
**Calculations:** CPU percentage based on delta between stats snapshots

### Service Metrics

All Rust services expose metrics at `/metrics` endpoint:

**Example:** <http://localhost:8001/metrics> (auth-service)

**Standard metrics:**

- HTTP request duration
- HTTP request count
- Active connections
- Error rates
- Custom business metrics

**Access in Prometheus:**

```promql
# HTTP request rate
rate(http_requests_total[5m])

# Error rate
rate(http_requests_total{status=~"5.."}[5m])

# Request duration (95th percentile)
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))
```

### Infrastructure Metrics

**PostgreSQL:**

- Connections, queries/sec, cache hit ratio
- Database size, table sizes

**Redis:**

- Memory usage, hit rate, key count
- Commands/sec, connected clients

**NATS:**

- Messages/sec, subscriptions, connections
- JetStream storage usage

## 🔍 Troubleshooting

### Dashboard Not Loading

```bash
# Check container
docker ps | grep dev-dashboard

# Check logs
docker logs dev-dashboard

# Verify build exists
ls -la monitoring/dev-dashboard/app/dist/

# Rebuild if needed
cd monitoring/dev-dashboard/app
npm run build
docker compose -f ../../docker-compose.dev.yml restart dev-dashboard
```

### Monitoring Backend Not Responding

```bash
# Check if running
ps aux | grep monitoring-service
curl http://localhost:8090/health

# Check Docker connection
docker ps  # Should work if Docker daemon accessible

# Restart backend
pkill -f monitoring-service
cd monitoring/dev-dashboard/backend
PORT=8090 cargo run --release &
```

### Services Showing as Down

```bash
# Verify services are actually running
docker ps

# Check backend logs for errors
# (will show connection failures)

# Test service directly
curl http://localhost:8001/api/v1/health  # Auth service
curl http://localhost:5173  # Frontend

# Check backend API response
curl -s http://localhost:8090/api/v1/services | jq '.services[] | select(.status == "down")'
```

### Resource Stats Not Showing

```bash
# Verify backend has Docker socket access
docker exec dev-dashboard ls -la /var/run/docker.sock  # Should exist if mounted

# Check if container is actually running
docker ps | grep <service-name>

# Only running containers have resource stats
# Stopped containers show "—" in the dashboard
```

### Grafana Dashboards Not Showing

```bash
# Check provisioning
docker exec monitoring-grafana ls -la /etc/grafana/provisioning/dashboards/

# Check logs
docker logs monitoring-grafana | grep provision

# Restart Grafana
docker compose -f docker-compose.monitoring.yml restart grafana
```

### Prometheus Not Scraping

```bash
# Check targets in Prometheus UI
open http://localhost:9090/targets

# Check config
docker exec monitoring-prometheus cat /etc/prometheus/prometheus.yml

# Check logs
docker logs monitoring-prometheus
```

## 📖 Related Documentation

- [Dev Dashboard Frontend](./dev-dashboard/app/README.md) - React + Vite setup
- [Monitoring Backend](./dev-dashboard/backend/) - Rust service docs
- [Docker Config Files](../docker/README.md) - Configuration structure
- [Scripts](../scripts/README.md) - Management scripts
- [Architecture](../docs/architecture/) - System design
- [Grafana Docs](https://grafana.com/docs/) - Official documentation
- [Prometheus Docs](https://prometheus.io/docs/) - Official documentation

## 🏗️ Technology Stack

**Frontend:**

- React 19.2.0
- Vite 5.x
- shadcn/ui (Tailwind CSS 4.x)
- TanStack Query v5
- TypeScript

**Backend:**

- Rust (actix-web)
- bollard (Docker API)
- reqwest (HTTP client)
- chrono (timestamps)

**Infrastructure:**

- Docker + Docker Compose
- Nginx (serving frontend)
- Docker Socket (resource stats)

---

**Last Updated:** 2025-11-14  
**Maintainer:** Unity Platform Team
