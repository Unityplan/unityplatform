# UnityPlan Port Allocation

## Standard Port Assignment

### Infrastructure Layer (Docker)

| Port | Service | Purpose |
|------|---------|---------|
| 3000 | Forgejo | Git version control server |
| 3001 | Grafana | Monitoring dashboards |
| 4222 | NATS | Message bus (client connections) |
| 5173 | Vite | Frontend development server |
| 5432 | PostgreSQL | Database |
| 6222 | NATS | Cluster connections |
| 6379 | Redis | Cache and session store |
| 8080 | Adminer | PostgreSQL web UI |
| 8082 | Redis Commander | Redis web UI |
| 8083 | Traefik | Service routing dashboard |
| 8222 | NATS | HTTP monitoring |
| 9090 | Prometheus | Metrics collection |
| 16686 | Jaeger | Distributed tracing UI |

### Backend Services (Rust)

| Port | Service | Status | Purpose |
|------|---------|--------|---------|
| 8000 | api-gateway | 🔮 Future | Unified API entry point |
| 8001 | auth-service | ✅ Active | Authentication & authorization |
| 8002 | user-service | ✅ Active | User profiles & avatars |
| 8003 | territory-service | 📋 Planned | Territory management |
| 8004 | badge-service | 📋 Planned | Achievement badges |
| 8005 | course-service | 📋 Planned | Learning courses & LMS |
| 8006 | forum-service | 📋 Planned | Discussion forums |

### Reserved Ranges

- **800x**: Backend microservices (8000-8099)
- **900x**: Monitoring & observability (9000-9099)
- **300x**: Infrastructure web UIs (3000-3099)
- **400x-600x**: Message buses & databases (4000-6999)

## Environment Variables

### Auth Service

```bash
SERVER_PORT=8001  # Default: 8001
```

### User Service

```bash
PORT=8002  # Default: 8002
```

### Frontend

```bash
VITE_AUTH_SERVICE_URL=http://localhost:8001
VITE_USER_SERVICE_URL=http://localhost:8002
VITE_API_URL=http://localhost:8001  # Primary entry (currently auth-service)
```

## Port Conflict Prevention

### Development Tools Reserve

- **8080**: Adminer (PostgreSQL UI) - DO NOT USE for backend services
- **8082**: Redis Commander
- **8083**: Traefik Dashboard

### Future API Gateway (8000)

When implementing an API gateway:

- All frontend requests → `http://localhost:8000`
- Gateway routes to individual services
- Services remain on 8001-8006
- Example:
  - `GET /api/auth/*` → auth-service:8001
  - `GET /api/users/*` → user-service:8002
  - `GET /api/territories/*` → territory-service:8003

## Migration Notes

### Changed Ports

- **user-service**: 8081 → 8002 (standardization)
- **auth-service**: Never used 8080 in code (documented incorrectly)

### Why 8000-8006 Range

1. **Consistency**: All backend services in same range
2. **No conflicts**: Avoids dev tool ports (8080-8089)
3. **Scalability**: Room for 100 microservices (8000-8099)
4. **Future gateway**: 8000 as entry point, services at 8001+
