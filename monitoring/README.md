# Monitoring & Observability

Unity Platform monitoring infrastructure and development dashboard.

## 📁 Directory Structure

```
monitoring/
├── README.md (this file)
└── dashboard/
    └── index.html         # Development dashboard (http://localhost:8888)
```

## 🎯 Development Dashboard

**Location:** `dashboard/index.html`  
**URL:** <http://localhost:8888>  
**Purpose:** Central hub for accessing all development and monitoring tools

**Access:**

```bash
# Start dashboard
./scripts/start.sh  # or docker compose -f docker-compose.dev.yml up -d

# Access dashboard
open http://localhost:8888
```

**Dashboard Links:**

**Development Tools:**

- Adminer (Database UI) - <http://localhost:8080>
- Forgejo (Git) - <http://192.168.60.133:3000>
- MailHog (Email testing) - <http://localhost:8025>
- Redis Commander - <http://localhost:8081>

**Monitoring:**

- Grafana - <http://localhost:3001> (admin/admin)
- Prometheus - <http://localhost:9090>
- Jaeger (Tracing) - <http://localhost:16686>

**Services:**

- Auth Service Swagger - <http://localhost:8001/swagger-ui/>
- User Service Swagger - <http://localhost:8002/swagger-ui/>
- Badge Service Swagger - <http://localhost:8003/swagger-ui/>
- Territory Service Swagger - <http://localhost:8004/swagger-ui/>

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

### Dashboard

Edit `dashboard/index.html` to add/remove links or customize appearance.

**Changes take effect immediately** (nginx serves static files, no restart needed).

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

### Start All Monitoring

```bash
# Start development tools + monitoring
./scripts/start.sh

# Or start monitoring separately
docker compose -f docker-compose.monitoring.yml up -d
```

### Access Services

1. **Dashboard:** <http://localhost:8888>
2. **Grafana:** <http://localhost:3001> (login: admin/admin)
3. **Prometheus:** <http://localhost:9090>
4. **Jaeger:** <http://localhost:16686>

### Check Status

```bash
./scripts/status.sh
```

### View Logs

```bash
# Grafana logs
docker logs -f monitoring-grafana

# Prometheus logs
docker logs -f monitoring-prometheus

# All monitoring logs
docker compose -f docker-compose.monitoring.yml logs -f
```

### Stop Monitoring

```bash
docker compose -f docker-compose.monitoring.yml down
```

## 📚 Metrics Documentation

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

# Verify volume mount
docker inspect dev-dashboard | grep -A 5 Mounts
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

- [Docker Config Files](../docker/README.md) - Configuration structure
- [Scripts](../scripts/README.md) - Management scripts
- [Architecture](../docs/architecture/) - System design
- [Grafana Docs](https://grafana.com/docs/) - Official documentation
- [Prometheus Docs](https://prometheus.io/docs/) - Official documentation

---

**Last Updated:** 2025-11-14  
**Maintainer:** Unity Platform Team
