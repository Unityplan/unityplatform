# Backend Observability Guide

This document describes the observability stack for UnityPlan's Rust microservices.

**Last Updated:** November 9, 2025  
**Applies To:** All backend services (auth-service, user-service, etc.)

---

## 📊 Observability Stack Overview

The UnityPlan platform uses the industry-standard observability stack:

- **Prometheus** - Metrics collection and storage
- **Grafana** - Metrics visualization and dashboards
- **Jaeger** - Distributed tracing
- **NATS Monitoring** - Message bus monitoring

---

## Prometheus - Metrics Collection

**URL:** <http://localhost:9090>

Prometheus collects time-series metrics from all services.

### Features

- Real-time metrics scraping from microservices
- PromQL query language for metrics analysis
- Built-in alerting capabilities
- NATS server metrics monitoring

### Usage

```promql
# Example queries
rate(http_requests_total[5m])                    # Request rate
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))  # 95th percentile latency
```

### Configuration

**File:** `docker/prometheus/prometheus.yml`

```yaml
scrape_configs:
  - job_name: 'auth-service'
    static_configs:
      - targets: ['auth-service:8080']
  - job_name: 'user-service'
    static_configs:
      - targets: ['user-service:8081']
```

### Adding Metrics to Your Service

```rust
use actix_web_prom::PrometheusMetrics;

// In main.rs
let prometheus = PrometheusMetrics::new("auth_service", Some("/metrics"), None);

HttpServer::new(move || {
    App::new()
        .wrap(prometheus.clone())
        .service(...)
})
```

---

## Grafana - Metrics Visualization

**URL:** <http://localhost:3001>  
**Credentials:** admin / admin (change on first login)

Grafana provides beautiful dashboards for visualizing Prometheus metrics.

### Features

- Pre-configured Prometheus datasource
- Jaeger datasource for tracing correlation
- Dashboard creation and sharing
- Alerting and notifications

### Pre-configured Datasources

- **Prometheus** (default)
- **Jaeger** (distributed tracing)

### Configuration

**Directory:** `docker/grafana/provisioning/`

### Creating Dashboards

1. Navigate to <http://localhost:3001>
2. Click "+" → "Dashboard"
3. Add panel with PromQL query
4. Save dashboard

### Recommended Dashboards

**Service Overview:**

- Request rate by endpoint
- Error rate by endpoint
- Response time (p50, p95, p99)
- Active connections

**Database:**

- Query duration
- Connection pool usage
- Query errors

**NATS:**

- Message publish rate
- Message consumption rate
- Stream storage usage

---

## Jaeger - Distributed Tracing

**URL:** <http://localhost:16686>

Jaeger provides distributed tracing for tracking requests across microservices.

### Features

- Request flow visualization across services
- Performance bottleneck identification
- Service dependency mapping
- OpenTelemetry compatible

### Ports

- **16686:** Web UI
- **14268:** Jaeger collector HTTP
- **14250:** Jaeger gRPC
- **6831:** Jaeger compact thrift (UDP)
- **9411:** Zipkin compatible endpoint

### Integration with Rust Services

```rust
use opentelemetry::trace::Tracer;
use opentelemetry_jaeger::Propagator;

// Configure Jaeger exporter
let tracer = opentelemetry_jaeger::new_pipeline()
    .with_service_name("auth-service")
    .with_agent_endpoint("jaeger:6831")
    .install_simple()
    .unwrap();

// Create spans
let span = tracer.start("process_login");
// ... operation ...
span.end();
```

### Using Jaeger UI

1. **Find traces:** Search by service, operation, tags
2. **Analyze timeline:** See request flow across services
3. **Identify bottlenecks:** Find slow operations
4. **Correlate with logs:** Use trace_id in logs

---

## NATS Monitoring

**URL:** <http://localhost:8222>

NATS provides built-in HTTP monitoring endpoints.

### Monitoring Endpoints

- `/varz` - General server information
- `/connz` - Connection information
- `/routez` - Route information  
- `/subsz` - Subscription information
- `/jsz` - JetStream information
- `/healthz` - Health check

### Example Usage

```bash
# Server info
curl http://localhost:8222/varz | jq .

# JetStream info
curl http://localhost:8222/jsz | jq .

# Subscriptions
curl http://localhost:8222/subsz | jq .
```

### Key Metrics to Monitor

- **Message rate:** Messages/second published and consumed
- **Stream storage:** JetStream stream size and limits
- **Connections:** Active client connections
- **Subscriptions:** Active subscriptions per subject

---

## Service Health Checks

Quick health check for all observability services:

```bash
# Check all services status
docker compose ps

# Individual service checks
curl http://localhost:9090/-/healthy          # Prometheus
curl http://localhost:3001/api/health         # Grafana
curl http://localhost:16686/                  # Jaeger
curl http://localhost:8222/healthz            # NATS

# Database connection
docker compose exec postgres psql -U unityplan -d unityplan_dev -c 'SELECT 1;'

# Redis connection
docker compose exec redis redis-cli ping
```

---

## Performance Monitoring Workflow

### 1. Development Phase

- Write code with OpenTelemetry instrumentation
- Add Prometheus metrics endpoints to services
- Use tracing spans for critical paths

```rust
// Example: Instrumented function
#[tracing::instrument(skip(pool))]
async fn get_user(pool: &PgPool, user_id: Uuid) -> Result<User> {
    // Automatically creates span
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
}
```

### 2. Testing Phase

- Monitor request flow in Jaeger
- Check metrics in Prometheus
- Create Grafana dashboards for key metrics

### 3. Debugging

- Use Jaeger to find slow requests
- Correlate traces with logs
- Check service dependencies
- Monitor resource usage in Prometheus

### 4. Optimization

- Identify bottlenecks via tracing
- Set up alerts in Grafana
- Monitor database query performance
- Track cache hit rates

---

## Best Practices

### Metrics Naming

Use consistent naming following Prometheus conventions:

```
<service>_<subsystem>_<metric>_<unit>

Examples:
- auth_service_login_requests_total
- auth_service_login_duration_seconds
- user_service_db_connections_active
- nats_messages_published_total
```

### RED Method

Track these three metrics for every service:

- **Rate:** Requests per second
- **Errors:** Error rate
- **Duration:** Request latency (p50, p95, p99)

```promql
# Rate
rate(http_requests_total[5m])

# Errors
rate(http_requests_total{status=~"5.."}[5m])

# Duration
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))
```

### Tracing Best Practices

- Create spans for database queries
- Add span attributes for debugging (user_id, territory_id)
- Propagate trace context across service boundaries
- Sample strategically in production (e.g., 10% of requests)

```rust
// Add attributes to spans
span.set_attribute(KeyValue::new("user_id", user_id.to_string()));
span.set_attribute(KeyValue::new("territory", "dk"));
```

### Logging Integration

- Use structured logging (JSON format)
- Include trace_id in logs for correlation
- Set appropriate log levels
- Use tracing crate for integrated logs + spans

```rust
use tracing::{info, error};

#[tracing::instrument]
async fn process_request(user_id: Uuid) {
    info!("Processing request for user {}", user_id);
    // trace_id automatically included
}
```

---

## Alerting Strategy

### Critical Alerts

Set up alerts for:

- **Error rate > 5%** - Service degradation
- **Response time p95 > 1s** - Performance issues
- **Service down** - Service unavailable
- **Database connection pool exhausted** - Resource exhaustion

### Example Alert Rules

```yaml
groups:
  - name: auth_service
    rules:
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5..",service="auth"}[5m]) > 0.05
        for: 5m
        annotations:
          summary: "High error rate in auth service"
      
      - alert: SlowRequests
        expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 1
        for: 10m
        annotations:
          summary: "95th percentile response time > 1s"
```

---

## Troubleshooting

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

1. Check service configuration for Jaeger endpoint
2. Verify `OTEL_EXPORTER_JAEGER_ENDPOINT` in `.env`
3. Check network connectivity between services
4. Verify spans are being created in code

```bash
# Check Jaeger logs
docker compose logs jaeger

# Test connectivity
docker compose exec auth-service curl http://jaeger:14268
```

### Missing Metrics

1. Verify `/metrics` endpoint is exposed
2. Check Prometheus scrape configuration
3. Verify service is running and reachable
4. Check Prometheus targets: <http://localhost:9090/targets>

---

## Additional Resources

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
- [Jaeger Documentation](https://www.jaegertracing.io/docs/)
- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust)
- [actix-web-prom](https://github.com/nlopes/actix-web-prom)
- [tracing crate](https://docs.rs/tracing/)

---

## Next Steps

After setting up observability:

1. Create Grafana dashboards for each service
2. Set up alerts for critical metrics
3. Document custom metrics in service README
4. Train team on using Jaeger for debugging
5. Consider adding log aggregation (Loki) later
