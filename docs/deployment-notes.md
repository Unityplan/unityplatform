# Deployment Notes & Lessons Learned

## Critical Configuration Points

### Pod Deployment
Always use `--env-file` when deploying pods to ensure correct container naming:
```bash
docker compose -f docker-compose.pod.yml -p pod-dk --env-file pods/denmark/.env up -d
```

Or use the deployment script:
```bash
bash scripts/deploy-multi-pod.sh denmark
```

### Prometheus Label Convention
- `pod`: Use short ID (e.g., "dk", "no", "se")
- `territory`: Use full name (e.g., "denmark", "norway", "sweden")
- This allows dashboards to filter by `pod="dk"` while maintaining readable territory names

Example Prometheus configuration:
```yaml
- job_name: "postgres-dk"
  static_configs:
    - targets: ["monitoring-postgres-exporter-dk:9187"]
      labels:
        pod: "dk"
        territory: "denmark"
        service: "postgres"
```

### Grafana Datasource Configuration
Datasources MUST have a `uid` field matching what dashboards reference:
```yaml
datasources:
  - name: Prometheus
    type: prometheus
    uid: prometheus  # Critical - dashboards reference this UID
    url: http://monitoring-prometheus:9090
```

### Network Architecture
Pod exporters need BOTH networks for proper monitoring:
- `pod-net`: For internal pod communication
- `mesh-network`: For cross-pod/monitoring communication

Example from docker-compose.pod.yml:
```yaml
postgres-exporter:
  networks:
    - pod-net        # Access to PostgreSQL
    - mesh-network   # Access from Prometheus
```

### Container Naming
Containers follow pattern: `{category}-{service}-{POD_ID}`
- Services: `service-postgres-dk`, `service-redis-dk`
- Exporters: `monitoring-postgres-exporter-dk`
- POD_ID comes from environment file (e.g., dk, no, se)

### Port Mapping Notes
- cAdvisor internal port: 8080 (not the external mapped port)
- Prometheus scrapes containers on internal Docker network ports
- External port mappings are only for host access

## Development Environment Access

### Services
- **Dev Dashboard**: http://localhost:8080
- **Grafana**: http://localhost:3001 (admin/admin)
- **Prometheus**: http://localhost:9090
- **Jaeger**: http://localhost:16686
- **Forgejo**: http://192.168.60.133:3000
- **Adminer**: http://localhost:8082
- **Redis Commander**: http://localhost:8083
- **MailHog**: http://localhost:8025
- **Docker Registry**: http://localhost:5000

### Denmark Pod Services
- **PostgreSQL**: localhost:5432 (unityplan/unityplan_dev_password_dk)
- **Redis**: localhost:6379
- **NATS**: localhost:4222 (monitor: 8222)
- **IPFS API**: localhost:5001
- **IPFS Gateway**: localhost:8081

## Troubleshooting

### Grafana "Datasource not found"
1. Check datasource has `uid` field in provisioning config
2. Remove Grafana database: `sudo rm docker/grafana-data/grafana.db`
3. Restart Grafana container
4. Verify datasource provisioned: Check Grafana UI → Data Sources

### Prometheus Targets Down
1. Verify containers are on `mesh-network`: `docker inspect <container> | grep Networks`
2. Check Prometheus can resolve DNS: `docker exec monitoring-prometheus nslookup <target>`
3. Verify port in Prometheus config matches container's internal port
4. Check target container logs: `docker logs <container>`

### Container Name Issues
If containers have empty suffix (e.g., `service-postgres-` instead of `service-postgres-dk`):
- Re-deploy using `--env-file` parameter with correct pod .env file
- Verify POD_ID is set in the .env file

## File Locations

### Important Configuration Files
- Prometheus (monitoring): `docker/prometheus/prometheus-central.yml`
- Prometheus (pod template): `docker/prometheus/prometheus.yml`
- Grafana datasources: `docker/grafana/provisioning/datasources/datasources.yml`
- Grafana dashboards: `docker/grafana/provisioning/dashboards/*.json`
- Pod template: `docker-compose.pod.yml`
- Pod configs: `pods/{territory}/.env`

### Data Volumes
- Prometheus data: `docker/prometheus-data/`
- Grafana data: `docker/grafana-data/`
- PostgreSQL data: `docker/pods/dk/postgres-data/`
- NATS JetStream: `docker/pods/dk/nats-data/jetstream/`

## Next Steps
With infrastructure complete, ready to begin:
1. Stage 2: Rust backend development (shared-lib crate)
2. Database schema design and migrations
3. Authentication service implementation
