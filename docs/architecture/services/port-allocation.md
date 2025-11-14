# Unity Platform Port Allocation

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
| 7008 | Matrix Synapse | Decentralized chat server (HTTP) |
| 8080 | Adminer | PostgreSQL web UI |
| 8082 | Redis Commander | Redis web UI |
| 8083 | Traefik | Service routing dashboard |
| 8222 | NATS | HTTP monitoring |
| 8448 | Matrix Synapse | Federation port (HTTPS) |
| 9090 | Prometheus | Metrics collection |
| 16686 | Jaeger | Distributed tracing UI |

### Backend Services (Rust)

| Port | Service | Status | Phase | Purpose |
|------|---------|--------|-------|---------|
| 8000 | api-gateway | 🔮 Future | Phase 3 | Unified API entry point |
| 8001 | auth-service | ✅ Complete | Phase 1 | Authentication & JWT management |
| 8002 | user-service | ✅ Complete | Phase 1 | Profiles, connections, GDPR |
| 8003 | settings-service | ⏳ Scaffolded | Phase 1 | User preferences & privacy |
| 8004 | invitation-service | ⏳ Scaffolded | Phase 1 | Invitation management & trust graph |
| 8005 | notification-service | ⏳ Scaffolded | Phase 1 | Notifications & email |
| 8006 | community-service | ⏳ Scaffolded | Phase 2 | Communities & membership |
| 8007 | badge-service | ⏳ Scaffolded | Phase 2 | Gamification & achievements |
| 8008 | territory-service | ⏳ Scaffolded | Phase 2 | Pod management & federation |
| 8009 | event-service | 📋 Planned | Phase 2 | Events & calendar |
| 8010 | course-service | 📋 Planned | Phase 2 | LMS & certifications |
| 8011 | forum-service | 📋 Planned | Phase 2 | Matrix-based forums |
| 8012 | translation-service | 📋 Planned | Phase 2 | i18n & community translations |
| 8013 | ipfs-service | 📋 Planned | Phase 2 | Decentralized file storage |

### Reserved Ranges

- **800x**: Backend microservices (8000-8099)
  - 8000: API Gateway (future)
  - 8001-8005: Phase 1 services (auth, user, settings, invitations, notifications)
  - 8006-8008: Phase 2 core services (community, badge, territory)
  - 8009-8013: Phase 2 advanced services (event, course, forum, translation, ipfs)
  - 8014-8099: Reserved for future services
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

#### Settings Service (8003)

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

#### Forum Service (8011)

```bash
PORT=8011
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
MATRIX_HOMESERVER_URL=https://matrix.unityplatform.org
MATRIX_ACCESS_TOKEN=your-matrix-token
```

#### Translation Service (8012)

```bash
PORT=8012
DATABASE_URL=postgres://user:password@localhost:5432/unityplatform
NATS_URL=nats://localhost:4222
LIBRETRANSLATE_URL=http://localhost:5000
```

#### IPFS Service (8013)

```bash
PORT=8013
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
VITE_SETTINGS_SERVICE_URL=http://localhost:8003
VITE_INVITATION_SERVICE_URL=http://localhost:8004
VITE_NOTIFICATION_SERVICE_URL=http://localhost:8005

# Phase 2 Services
VITE_COMMUNITY_SERVICE_URL=http://localhost:8006
VITE_BADGE_SERVICE_URL=http://localhost:8007
VITE_TERRITORY_SERVICE_URL=http://localhost:8008
VITE_EVENT_SERVICE_URL=http://localhost:8009
VITE_COURSE_SERVICE_URL=http://localhost:8010
VITE_FORUM_SERVICE_URL=http://localhost:8011
VITE_TRANSLATION_SERVICE_URL=http://localhost:8012
VITE_IPFS_SERVICE_URL=http://localhost:8013

# Future API Gateway (Phase 3)
VITE_API_URL=http://localhost:8000  # Single entry point
```

## Port Conflict Prevention

### Development Tools Reserve

- **8080**: Adminer (PostgreSQL UI) - DO NOT USE for backend services
- **8082**: Redis Commander
- **8083**: Traefik Dashboard

### Future API Gateway (8000)

When implementing an API gateway (Phase 3):

- All frontend requests → `http://localhost:8000`
- Gateway routes to individual services
- Services remain on 8001-8013
- Example routing:
  - `GET /api/auth/*` → auth-service:8001
  - `GET /api/users/*` → user-service:8002
  - `GET /api/settings/*` → settings-service:8003
  - `GET /api/invitations/*` → invitation-service:8004
  - `GET /api/notifications/*` → notification-service:8005
  - `GET /api/communities/*` → community-service:8006
  - `GET /api/badges/*` → badge-service:8007
  - `GET /api/territories/*` → territory-service:8008
  - `GET /api/events/*` → event-service:8009
  - `GET /api/courses/*` → course-service:8010
  - `GET /api/forum/*` → forum-service:8011
  - `GET /api/translations/*` → translation-service:8012
  - `GET /api/ipfs/*` → ipfs-service:8013

## Service Implementation Status

### ✅ Complete (Production Ready)

- **auth-service (8001)**: 5/5 endpoints, 19/19 tests passing
- **user-service (8002)**: 28/28 endpoints, 22/22 tests passing

### ⏳ Scaffolded (Week 1-3 Implementation)

- **settings-service (8003)**: 0/5 endpoints
- **invitation-service (8004)**: 0/6 endpoints
- **notification-service (8005)**: 0/7 endpoints

### ⏳ Scaffolded (Phase 2)

- **community-service (8006)**: 0/8 endpoints
- **badge-service (8007)**: 0/6 endpoints
- **territory-service (8008)**: 0/4 endpoints

### 📋 Planned (Phase 2)

- **event-service (8009)**: Not yet started
- **course-service (8010)**: Not yet started
- **forum-service (8011)**: Not yet started
- **translation-service (8012)**: Not yet started
- **ipfs-service (8013)**: Not yet started

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
   - 8001-8005: Phase 1 (Core user functionality)
   - 8006-8008: Phase 2 Core (Social features)
   - 8009-8013: Phase 2 Advanced (Content & federation)
   - 8014-8099: Reserved for future expansion

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

---

**Last Updated:** November 12, 2025  
**Total Services:** 13 microservices + 1 API gateway (planned)  
**Implementation Status:** 2/13 complete, 6/13 scaffolded, 5/13 planned
