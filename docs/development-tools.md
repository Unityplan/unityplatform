# Development Tools Guide

This document describes all the development and observability tools available in the UnityPlan development environment.

## 📊 Observability Stack

### Prometheus - Metrics Collection
**URL:** http://localhost:9090

Prometheus collects time-series metrics from all services.

**Features:**
- Real-time metrics scraping from microservices
- PromQL query language for metrics analysis
- Built-in alerting capabilities
- NATS server metrics monitoring

**Usage:**
```promql
# Example queries
rate(http_requests_total[5m])                    # Request rate
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))  # 95th percentile latency
```

**Configuration:** `docker/prometheus/prometheus.yml`

### Grafana - Metrics Visualization
**URL:** http://localhost:3001  
**Credentials:** admin / admin (change on first login)

Grafana provides beautiful dashboards for visualizing Prometheus metrics.

**Features:**
- Pre-configured Prometheus datasource
- Jaeger datasource for tracing correlation
- Dashboard creation and sharing
- Alerting and notifications

**Pre-configured Datasources:**
- Prometheus (default)
- Jaeger (distributed tracing)

**Configuration:** `docker/grafana/provisioning/`

### Jaeger - Distributed Tracing
**URL:** http://localhost:16686

Jaeger provides distributed tracing for tracking requests across microservices.

**Features:**
- Request flow visualization across services
- Performance bottleneck identification
- Service dependency mapping
- OpenTelemetry compatible

**Ports:**
- 16686: Web UI
- 14268: Jaeger collector HTTP
- 14250: Jaeger gRPC
- 6831: Jaeger compact thrift (UDP)
- 9411: Zipkin compatible endpoint

**Integration:**
```rust
// In your Rust services, configure OpenTelemetry to send to Jaeger
let jaeger_endpoint = env::var("JAEGER_ENDPOINT").unwrap();
// Configure tracer to send spans to Jaeger
```

## 🗄️ Database & Storage Tools

### Adminer - PostgreSQL Management
**URL:** http://localhost:8080

Lightweight database management interface.

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

### Redis Commander - Redis Management
**URL:** http://localhost:8082

Visual Redis management interface.

**Features:**
- Key-value browsing
- Real-time monitoring
- Data editing and deletion
- TTL management
- CLI console

**Usage:**
- Browse keys by pattern
- Inspect data structures (strings, hashes, lists, sets)
- Monitor memory usage
- Execute Redis commands

## 📧 Email Testing

### MailHog - Email Capture
**URL:** http://localhost:8025  
**SMTP:** localhost:1025

MailHog captures all outgoing emails for testing.

**Features:**
- No configuration required
- All emails sent to localhost:1025 are captured
- View emails in web interface
- Search and filter emails
- In-memory storage (emails cleared on restart)

**Backend Configuration:**
```rust
// Configure SMTP in your email service
SMTP_HOST=localhost
SMTP_PORT=1025
```

## 🔀 Service Routing

### Traefik - Reverse Proxy & Service Mesh
**URL:** http://localhost:8083/dashboard/

Traefik provides dynamic service routing with a dashboard.

**Features:**
- Automatic service discovery via Docker labels
- Load balancing
- HTTP/HTTPS routing
- Middleware (authentication, rate limiting, etc.)
- Real-time configuration updates

**Ports:**
- 80: HTTP entrypoint
- 443: HTTPS entrypoint
- 8083: Dashboard

**Future Use:**
When microservices are containerized, Traefik will automatically route traffic based on Docker labels.

## 📡 Message Bus Monitoring

### NATS Monitoring
**URL:** http://localhost:8222

NATS provides built-in HTTP monitoring endpoints.

**Endpoints:**
- `/varz` - General server information
- `/connz` - Connection information
- `/routez` - Route information  
- `/subsz` - Subscription information
- `/jsz` - JetStream information
- `/healthz` - Health check

**Example:**
```bash
curl http://localhost:8222/varz | jq .
curl http://localhost:8222/jsz | jq .
```

## 🔍 Service Health Checks

Quick health check for all services:

```bash
# Check all services status
docker compose ps

# Individual service checks
curl http://localhost:9090/-/healthy          # Prometheus
curl http://localhost:3001/api/health         # Grafana
curl http://localhost:16686/                  # Jaeger
curl http://localhost:8222/healthz            # NATS
curl http://localhost:8025/api/v1/messages    # MailHog

# Database connection
docker compose exec postgres psql -U unityplan -d unityplan_dev -c 'SELECT 1;'

# Redis connection
docker compose exec redis redis-cli ping
```

## 🚀 Performance Monitoring Workflow

1. **Development Phase:**
   - Write code with OpenTelemetry instrumentation
   - Add Prometheus metrics endpoints to services
   - Use tracing spans for critical paths

2. **Testing Phase:**
   - Monitor request flow in Jaeger
   - Check metrics in Prometheus
   - Create Grafana dashboards for key metrics

3. **Debugging:**
   - Use Jaeger to find slow requests
   - Correlate traces with logs
   - Check service dependencies
   - Monitor resource usage in Prometheus

4. **Optimization:**
   - Identify bottlenecks via tracing
   - Set up alerts in Grafana
   - Monitor database query performance
   - Track cache hit rates in Redis Commander

## 📚 Best Practices

### Metrics
- Expose `/metrics` endpoint on all services
- Use consistent naming (service_operation_duration_seconds)
- Track error rates, latency, and throughput (RED method)
- Add labels for territory, user type, etc.

### Tracing
- Create spans for database queries
- Add span attributes for debugging (user_id, territory_id)
- Propagate trace context across service boundaries
- Sample strategically in production

### Logging
- Use structured logging (JSON format)
- Include trace_id in logs for correlation
- Set appropriate log levels
- Centralize logs (consider adding Loki later)

## 🛠️ Troubleshooting

### Grafana Permission Issues
```bash
sudo chown -R 472:472 docker/grafana-data
docker compose restart grafana
```

### Prometheus Permission Issues
```bash
sudo chown -R 65534:65534 docker/prometheus-data
docker compose restart prometheus
```

### Jaeger Not Receiving Traces
- Check service configuration for Jaeger endpoint
- Verify OTEL_EXPORTER_JAEGER_ENDPOINT in .env
- Check network connectivity between services

### Redis Commander Can't Connect
- Verify Redis is running: `docker compose ps redis`
- Check REDIS_HOSTS environment variable
- Restart: `docker compose restart redis-commander`

## 📖 Additional Resources

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
- [Jaeger Documentation](https://www.jaegertracing.io/docs/)
- [Traefik Documentation](https://doc.traefik.io/traefik/)
- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust)
