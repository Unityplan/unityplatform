# Unity Platform Port Allocation

## Standard Port Assignment

### Infrastructure Layer (Docker)

| Port | Service | Purpose | Status |
|------|---------|---------|--------|
| 80 | Traefik | HTTP traffic routing | ✅ Running |
| 443 | Traefik | HTTPS traffic routing | ✅ Running |
| 1025 | MailHog | SMTP server (dev) | ✅ Running |
| 2222 | Forgejo | SSH git access | ✅ Running |
| 3000 | Forgejo | Git version control web UI | ✅ Running |
| 3001 | Grafana | Monitoring dashboards | ✅ Running |
| 4001 | IPFS | Swarm port (P2P networking) | ✅ Running |
| 4222 | NATS | Message bus (client connections) | ✅ Running |
| 5000 | Docker Registry | Container image registry | ✅ Running |
| 5001 | IPFS | API port | ✅ Running |
| 5173 | Vite | Frontend development server | ✅ Running |
| 5432 | PostgreSQL | Database server | ✅ Running |
| 5775 | Jaeger | Zipkin-compatible UDP endpoint | ✅ Running |
| 5778 | Jaeger | Serve configs, sampling strategies | ✅ Running |
| 6222 | NATS | Cluster connections | ✅ Running |
| 6379 | Redis | Cache and session store | ✅ Running |
| 6831-6832 | Jaeger | Jaeger Thrift UDP | ✅ Running |
| 7008 | Matrix Synapse | Decentralized chat server (HTTP) | ⚠️ Needs config |
| 7777 | NATS Exporter | NATS metrics | ✅ Running |
| 8025 | MailHog | Web UI for email testing | ✅ Running |
| 8080 | Adminer | PostgreSQL web UI | ✅ Running |
| 8081 | IPFS | Gateway port (HTTP) | ✅ Running |
| 8082 | Redis Commander | Redis web UI | ✅ Running |
| 8083 | Traefik | Service routing dashboard | ✅ Running |
| 8089 | cAdvisor | Container metrics | ✅ Running |
| 8222 | NATS | HTTP monitoring | ✅ Running |
| 8448 | Matrix Synapse | Federation port (HTTPS) | ⚠️ Needs config |
| 8888 | Dev Dashboard | Central development hub | ✅ Running |
| 9090 | Prometheus | Metrics collection | ✅ Running |
| 9100 | Node Exporter | System metrics | ✅ Running |
| 9121 | Redis Exporter | Redis metrics | ✅ Running |
| 9187 | Postgres Exporter | PostgreSQL metrics | ✅ Running |
| 9411 | Jaeger | Zipkin-compatible HTTP endpoint | ✅ Running |
| 14250 | Jaeger | Model proto | ✅ Running |
| 14268 | Jaeger | Jaeger Thrift HTTP | ✅ Running |
| 16686 | Jaeger | Distributed tracing UI | ✅ Running |

### Backend Services (Rust)

| Port | Service | Status | Phase | Purpose |
|------|---------|--------|-------|---------|
| 8000 | api-gateway | 🔮 Future | Phase 3 | Unified API entry point |
| 8001 | auth-service | ✅ Complete | Phase 1 | Authentication & JWT management |
| 8002 | user-service | ✅ Complete | Phase 1 | Profiles, connections, GDPR, settings |
| 8003 | event-service | 📋 Planned | Phase 2 | Events & calendar |
| 8004 | invitation-service | ⏳ Scaffolded | Phase 1 | Invitation management & trust graph |
| 8005 | notification-service | ⏳ Scaffolded | Phase 1 | Notifications & email |
| 8006 | community-service | 🚧 In Development | Phase 1 | Communities & membership |
| 8007 | badge-service | ✅ Complete | Phase 1 | Gamification & achievements |
| 8008 | territory-service | ✅ Complete | Phase 1 | Pod management & federation |
| 8009 | course-service | 📋 Planned | Phase 2 | LMS & certifications |
| 8010 | forum-service | 📋 Planned | Phase 2 | Matrix-based forums |
| 8011 | translation-service | 📋 Planned | Phase 2 | i18n & community translations |
| 8012 | ipfs-service | 📋 Planned | Phase 2 | Decentralized file storage |
| 8013 | _(reserved)_ | - | - | Reserved for future services |
| 8014 | utility-service | ✅ Complete | Phase 1 | Favicon fetching, utilities (no DB) |

### Reserved Ranges

- **800x**: Backend microservices (8000-8099)
  - 8000: API Gateway (future)
  - 8001-8002: Phase 1 core services (auth, user with settings)
  - 8003: Phase 2 event service (events & calendar)
  - 8004-8005: Phase 1 services (invitations, notifications)
  - 8006-8008: Phase 1 core services (community, badge, territory)
  - 8009-8012: Phase 2 advanced services (course, forum, translation, ipfs)
  - 8013: Reserved for future services
  - 8014: Infrastructure utilities (favicon, QR codes, etc.) - Phase 1
  - 8015-8099: Reserved for future services
- **900x**: Monitoring & observability (9000-9099)
- **300x**: Infrastructure web UIs (3000-3099)
- **400x-600x**: Message buses & databases (4000-6999)

## Environment Variables

### Phase 1 Services (Complete/In Progress)

#### Auth Service (8001)

```bash
SERVER_PORT=8001
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
JWT_SECRET=your-secret-key
JWT_ACCESS_TOKEN_EXPIRY=900  # 15 minutes
JWT_REFRESH_TOKEN_EXPIRY=604800  # 7 days
```

#### User Service (8002)

```bash
PORT=8002
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
REDIS_URL=redis://localhost:6379
```

#### Event Service (8003)

```bash
PORT=8003
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
```

#### Invitation Service (8004)

```bash
PORT=8004
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
```

#### Notification Service (8005)

```bash
PORT=8005
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=notifications@unityplatform.org
SMTP_PASSWORD=secure_password
FROM_EMAIL=notifications@unityplatform.org
```

### Phase 2 Services (Planned)

#### Community Service (8006)

```bash
PORT=8006
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
```

#### Badge Service (8007)

```bash
PORT=8007
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
```

#### Territory Service (8008)

```bash
PORT=8008
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
```

#### Course Service (8009)

```bash
PORT=8009
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
```

#### Forum Service (8010)

```bash
PORT=8010
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
MATRIX_HOMESERVER_URL=https://matrix.unityplatform.org
MATRIX_ACCESS_TOKEN=your-matrix-token
```

#### Translation Service (8011)

```bash
PORT=8011
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
LIBRETRANSLATE_URL=http://localhost:5000
```

#### IPFS Service (8012)

```bash
PORT=8012
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
IPFS_API_URL=http://localhost:5001
IPFS_GATEWAY_URL=https://ipfs.unityplatform.org
```

### Frontend

```bash
# Phase 1 Services
VITE_AUTH_SERVICE_URL=http://localhost:8001
VITE_USER_SERVICE_URL=http://localhost:8002
VITE_INVITATION_SERVICE_URL=http://localhost:8004
VITE_NOTIFICATION_SERVICE_URL=http://localhost:8005

# Phase 2 Services
VITE_EVENT_SERVICE_URL=http://localhost:8003
VITE_COMMUNITY_SERVICE_URL=http://localhost:8006
VITE_BADGE_SERVICE_URL=http://localhost:8007
VITE_TERRITORY_SERVICE_URL=http://localhost:8008
VITE_COURSE_SERVICE_URL=http://localhost:8009
VITE_FORUM_SERVICE_URL=http://localhost:8010
VITE_TRANSLATION_SERVICE_URL=http://localhost:8011
VITE_IPFS_SERVICE_URL=http://localhost:8012

# Future API Gateway (Phase 3)
VITE_API_URL=http://localhost:8000  # Single entry point
```

## Port Conflict Prevention

### Critical Port Assignments (DO NOT CHANGE)

**Backend Services (8001-8013):**

- These ports are RESERVED for Rust microservices only
- ❌ DO NOT assign infrastructure services to 800x range
- ❌ DO NOT use 8080, 8082, 8083, 8089 for backend services (reserved for dev tools)

**Infrastructure Services:**

- Must use ports OUTSIDE the 8001-8013 range
- Development tools: 8080, 8082, 8083, 8088, 8089, 8888
- Matrix HTTP: 7008 (changed from 8008 to avoid conflict with territory-service)
- Monitoring exporters: 7777, 9100, 9121, 9187

### Port Range Allocation

| Range | Purpose | Examples |
|-------|---------|----------|
| 80, 443 | HTTP/HTTPS (Traefik) | Web traffic routing |
| 1000-2999 | Email & SSH | 1025 (SMTP), 2222 (SSH) |
| 3000-3099 | Infrastructure Web UIs | 3000 (Forgejo), 3001 (Grafana) |
| 4000-4999 | P2P & Messaging | 4001 (IPFS), 4222 (NATS) |
| 5000-5999 | Services & APIs | 5000 (Registry), 5001 (IPFS), 5173 (Vite), 5432 (PostgreSQL), 5775-5778 (Jaeger) |
| 6000-6999 | Caching & Clustering | 6222 (NATS), 6379 (Redis), 6831-6832 (Jaeger) |
| 7000-7999 | Support Services | 7008 (Matrix), 7777 (NATS Exporter) |
| 8000-8013 | **BACKEND RUST SERVICES ONLY** | 8001-8013 (microservices) |
| 8014-8099 | Reserved for future backend | Available for new services |
| 8080-8089 | Development Tools | 8080 (Adminer), 8081 (IPFS), 8082 (Redis Commander), 8083 (Traefik), 8089 (cAdvisor) |
| 8100-8999 | Other Infrastructure | 8222 (NATS), 8448 (Matrix), 8888 (Dev Dashboard) |
| 9000-9999 | Monitoring & Observability | 9090 (Prometheus), 9100-9187 (Exporters), 9411 (Jaeger) |
| 14000-16999 | Distributed Tracing | 14250, 14268, 16686 (Jaeger) |

### Verification Commands

**Check if a port is in use:**

```bash
sudo lsof -i :PORT_NUMBER
```

**Check all Unity Platform services:**

```bash
docker ps --format "table {{.Names}}\t{{.Ports}}" | grep -E "(service-|monitoring-|dev-|reverse-proxy)"
```

**Verify no conflicts before starting:**

```bash
# Check backend service ports (8001-8013)
for port in {8001..8013}; do
  if lsof -i :$port >/dev/null 2>&1; then
    echo "⚠️  Port $port is in use"
  fi
done
```

### Common Conflict Resolutions

**Matrix vs Territory Service (Port 8008):**

- ❌ OLD: Matrix was configured on port 8008
- ✅ FIXED: Matrix moved to port 7008
- Reason: Port 8008 is officially allocated to territory-service

**Grafana/Prometheus Permission Issues:**

- ❌ OLD: Volume permission errors causing restart loops
- ✅ FIXED: Set correct ownership (UID 472 for Grafana, UID 65534 for Prometheus)

```bash
sudo chown -R 472:472 docker/grafana-data
sudo chown -R 65534:65534 docker/prometheus-data
```

### Development Tools Reserve

- **8080**: Adminer (PostgreSQL UI) - DO NOT USE for backend services
- **8082**: Redis Commander - DO NOT USE for backend services
- **8083**: Traefik Dashboard - DO NOT USE for backend services
- **8089**: cAdvisor - DO NOT USE for backend services
- **8888**: Dev Dashboard - DO NOT USE for backend services

### Future API Gateway (8000)

When implementing an API gateway (Phase 3):

- All frontend requests → `http://localhost:8000`
- Gateway routes to individual services
- Services remain on 8001-8012
- Example routing:
  - `GET /api/auth/*` → auth-service:8001
  - `GET /api/users/*` → user-service:8002
  - `GET /api/events/*` → event-service:8003
  - `GET /api/invitations/*` → invitation-service:8004
  - `GET /api/notifications/*` → notification-service:8005
  - `GET /api/communities/*` → community-service:8006
  - `GET /api/badges/*` → badge-service:8007
  - `GET /api/territories/*` → territory-service:8008
  - `GET /api/courses/*` → course-service:8009
  - `GET /api/forum/*` → forum-service:8010
  - `GET /api/translations/*` → translation-service:8011
  - `GET /api/ipfs/*` → ipfs-service:8012

## Service Implementation Status

### ✅ Complete (Production Ready)

- **auth-service (8001)**: 5/5 endpoints, 19/19 tests passing
- **user-service (8002)**: 28/28 endpoints, 22/22 tests passing (includes settings)

### ⏳ Scaffolded (Week 1-3 Implementation)

- **invitation-service (8004)**: 0/6 endpoints
- **notification-service (8005)**: 0/7 endpoints

### ⏳ Scaffolded (Phase 2)

- **community-service (8006)**: 0/8 endpoints
- **badge-service (8007)**: 0/6 endpoints
- **territory-service (8008)**: 0/4 endpoints

### 📋 Planned (Phase 2)

- **event-service (8003)**: Not yet started
- **course-service (8009)**: Not yet started
- **forum-service (8010)**: Not yet started
- **translation-service (8011)**: Not yet started
- **ipfs-service (8012)**: Not yet started

## Migration Notes

### Changed Ports

- **user-service**: 8081 → 8002 (standardization, November 2025)
- **auth-service**: Never used 8080 in code (documented incorrectly)

### Port Allocation Strategy

1. **Consistency**: All backend services in same range (8000-8099)
2. **No conflicts**: Avoids dev tool ports (8080-8089)
3. **Scalability**: Room for 100 microservices (8000-8099)
4. **Future gateway**: 8000 as entry point, services at 8001+
5. **Phase-based allocation**:
   - 8001-8002: Phase 1 core (auth, user with settings)
   - 8003: Phase 2 events (moved up for earlier implementation)
   - 8004-8005: Phase 1 (invitations, notifications)
   - 8006-8008: Phase 2 core (community, badge, territory)
   - 8009-8012: Phase 2 advanced (course, forum, translation, ipfs)
   - 8013-8099: Reserved for future expansion

### Service Documentation

Complete documentation for all services available at:

- `docs/architecture/services/{service-name}/README.md`

Each service documentation includes:

- Database schema
- API endpoints
- Service dependencies
- NATS events
- Holochain migration strategy
- Implementation status

## Troubleshooting

### Service Won't Start - Port Already in Use

**Symptom:** Docker container fails with "address already in use" error

**Diagnosis:**

```bash
# Find what's using the port
sudo lsof -i :PORT_NUMBER

# Example: Check port 8008
sudo lsof -i :8008
```

**Solution:**

1. Identify the service using the port
2. Check if it's supposed to be running (consult this document)
3. If it's a Rust backend service, it should be on 8001-8013
4. If it's an infrastructure service conflicting with 8001-8013, reconfigure it to use a different port range

### Monitoring Services Restarting (Grafana/Prometheus)

**Symptom:** Container status shows "Restarting" repeatedly

**Diagnosis:**

```bash
# Check logs
docker logs monitoring-grafana --tail 50
docker logs monitoring-prometheus --tail 50
```

**Common causes:**

1. **Permission errors** - Volume data owned by wrong user
2. **Missing configuration** - Config file not found
3. **Port conflict** - Another service using the same port

**Solutions:**

```bash
# Fix Grafana permissions (UID 472)
sudo chown -R 472:472 docker/grafana-data

# Fix Prometheus permissions (UID 65534 = nobody)
sudo chown -R 65534:65534 docker/prometheus-data

# Restart services
docker compose -f docker-compose.monitoring.yml -p unityplatform-monitoring restart
```

### Matrix Configuration Missing

**Symptom:** Matrix container shows "Config file '/data/homeserver.yaml' does not exist"

**Diagnosis:**

```bash
docker logs service-matrix-dk --tail 20
```

**Solution:** Matrix requires initial configuration (setup pending)

```bash
# Generate config (when ready to set up Matrix)
docker run -it --rm \
  -v matrix-data:/data \
  matrixdotorg/synapse:latest \
  generate
```

### Checking All Services Status

**Quick overview:**

```bash
./scripts/status.sh
```

**Detailed container listing:**

```bash
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
```

**Service-specific health checks:**

```bash
# PostgreSQL
docker exec service-postgres-dk pg_isready -U unityplatform

# NATS
curl http://localhost:8222/varz

# Redis
docker exec service-redis-dk redis-cli ping
```

---

**Last Updated:** November 14, 2025  
**Total Services:** 12 microservices + 1 API gateway (planned)  
**Implementation Status:** 2/12 complete, 5/12 scaffolded, 5/12 planned  
**Infrastructure Status:** 28/28 ports allocated and documented  
**Note:** Settings functionality integrated into user-service (8002)
