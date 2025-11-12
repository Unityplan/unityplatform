# Application Deployment & CI/CD Architecture

**Document Version:** 1.0  
**Last Updated:** November 5, 2025  
**Target Phase:** Phase 1.3 (Backend Services) & Phase 2 (CI/CD)

---

## Table of Contents

1. [Overview](#overview)
2. [Application Services per Pod](#application-services-per-pod)
3. [Frontend Deployment](#frontend-deployment)
4. [Backend Microservices](#backend-microservices)
5. [Staging Environment](#staging-environment)
6. [CI/CD Pipeline](#cicd-pipeline)
7. [Deployment Workflows](#deployment-workflows)
8. [Rollback Procedures](#rollback-procedures)

---

## Overview

This document describes how **application services** (frontend and backend) are deployed across the multi-pod architecture, along with staging environments and CI/CD pipelines.

### Key Principles

1. **Per-Pod Deployment** - Each pod runs its own complete copy of all application services
2. **Territory Isolation** - Application layer maintains same isolation as infrastructure layer
3. **Staging Mirrors Production** - Staging environment identical to production, smaller scale
4. **Automated CI/CD** - Docker-based pipeline for building, testing, and deploying
5. **Rolling Updates** - Zero-downtime deployments across pods

---

## Application Services per Pod

### Complete Pod Stack

Each pod contains **infrastructure services** + **application services**:

```
┌──────────────────────────────────────────────────────────────┐
│                    Pod Denmark (pod-dk)                      │
├──────────────────────────────────────────────────────────────┤
│  APPLICATION LAYER                                           │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ Frontend (Nginx)           - Port 3000                 │  │
│  │ API Gateway (Actix-web)    - Port 8080                 │  │
│  │ Auth Service               - Internal                  │  │
│  │ Community Service          - Internal                  │  │
│  │ User Service               - Internal                  │  │
│  │ Event Service              - Internal                  │  │
│  │ WebSocket Gateway          - Port 9000                 │  │
│  └────────────────────────────────────────────────────────┘  │
├──────────────────────────────────────────────────────────────┤
│  INFRASTRUCTURE LAYER                                        │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ PostgreSQL                 - Port 5432                 │  │
│  │ Redis                      - Port 6379                 │  │
│  │ NATS                       - Port 4222/6222/8222       │  │
│  │ IPFS                       - Port 5001/8080            │  │
│  │ Matrix Synapse             - Port 8008                 │  │
│  │ Monitoring Exporters       - Various ports             │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

### Why Per-Pod Application Deployment?

**Benefits:**

- ✅ **Complete isolation**: Territory data never leaves its pod
- ✅ **Independent scaling**: Scale Denmark's backend without affecting Norway
- ✅ **Fault tolerance**: Pod failure doesn't cascade
- ✅ **Simple routing**: Users always hit their territory's pod (no complex load balancing)
- ✅ **Territory sovereignty**: Each territory controls its application layer

**Alternative (Centralized) - Not Used:**

- ❌ One set of backend services routes to multiple databases (complex, single point of failure)
- ❌ Shared frontend serves all territories (violates data locality)

---

## Frontend Deployment

### Architecture

**React + Vite** built as static assets, served via **Nginx**:

```
┌─────────────────────────────────────────────────────┐
│  User Browser                                       │
│  https://dk.unityplatform.org                          │
└────────────┬────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────┐
│  Traefik Reverse Proxy (Global)                    │
│  Routes by subdomain:                              │
│  - dk.unityplatform.org  → frontend-dk:80              │
│  - no.unityplatform.org  → frontend-no:80              │
│  - de.europe.unityplatform.org → frontend-eu:80        │
└────────────┬────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────┐
│  Frontend Container (Nginx)                        │
│  app-frontend-dk                                   │
│                                                     │
│  /usr/share/nginx/html/                            │
│  ├── index.html                                    │
│  ├── assets/                                       │
│  │   ├── index-abc123.js    (React bundle)        │
│  │   ├── index-def456.css   (Tailwind styles)     │
│  │   └── logo.svg                                 │
│  └── config.json             (Runtime config)      │
│                                                     │
│  Environment-injected config:                      │
│  - API_URL=http://api-gateway-dk:8080              │
│  - WEBSOCKET_URL=ws://websocket-dk:9000            │
│  - TERRITORY_ID=DK                                 │
└─────────────────────────────────────────────────────┘
```

### Docker Compose Configuration

```yaml
# docker-compose.pod.yml (excerpt)
services:
  frontend:
    image: ${DOCKER_REGISTRY}/unityplatform/frontend:${VERSION:-latest}
    container_name: app-frontend-${POD_ID}
    ports:
      - "${FRONTEND_PORT:-3000}:80"
    environment:
      # Runtime configuration (injected into config.json)
      - API_URL=http://api-gateway-${POD_ID}:8080
      - WEBSOCKET_URL=ws://websocket-${POD_ID}:9000
      - TERRITORY_ID=${TERRITORY_ID}
      - TERRITORY_NAME=${TERRITORY_NAME}
      - TERRITORY_LOCALE=${TERRITORY_LOCALE}
      - ENVIRONMENT=${ENVIRONMENT:-production}
    networks:
      - pod-${POD_ID}-net
      - mesh-network  # For Traefik routing
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.frontend-${POD_ID}.rule=Host(`${TERRITORY_ID}.unityplatform.org`)"
      - "traefik.http.routers.frontend-${POD_ID}.entrypoints=web,websecure"
      - "traefik.http.services.frontend-${POD_ID}.loadbalancer.server.port=80"
    restart: unless-stopped
```

### Frontend Build Process

```dockerfile
# frontend/Dockerfile
FROM node:20-alpine AS builder

WORKDIR /app
COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build
# Outputs to /app/dist

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf

# Entrypoint script to inject environment variables into config.json
COPY docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

EXPOSE 80
CMD ["/docker-entrypoint.sh"]
```

```bash
# frontend/docker-entrypoint.sh
#!/bin/sh
# Generate config.json from environment variables
cat > /usr/share/nginx/html/config.json <<EOF
{
  "apiUrl": "${API_URL}",
  "websocketUrl": "${WEBSOCKET_URL}",
  "territoryId": "${TERRITORY_ID}",
  "territoryName": "${TERRITORY_NAME}",
  "locale": "${TERRITORY_LOCALE}",
  "environment": "${ENVIRONMENT}"
}
EOF

# Start nginx
nginx -g "daemon off;"
```

### Port Allocation

| Pod     | Frontend Port | API Gateway | WebSocket |
|---------|---------------|-------------|-----------|
| Denmark | 3000          | 8080        | 9000      |
| Norway  | 3100          | 8180        | 9100      |
| Sweden  | 3200          | 8280        | 9200      |
| Europe  | 3300          | 8380        | 9300      |

---

## Backend Microservices

### Service Architecture

```
┌───────────────────────────────────────────────────────────────┐
│  External Request                                             │
│  https://dk.unityplatform.org/api/communities                    │
└────────────┬──────────────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────────┐
│  API Gateway (Actix-web)                                    │
│  service-api-gateway-dk:8080                                │
│                                                              │
│  - Request validation                                        │
│  - JWT token verification                                    │
│  - Rate limiting                                             │
│  - Route to internal microservices                           │
└────────────┬─────────────────┬──────────────┬────────────────┘
             │                 │              │
      ┌──────▼─────┐    ┌─────▼─────┐   ┌────▼────┐
      │   Auth     │    │ Community │   │  User   │
      │  Service   │    │  Service  │   │ Service │
      │            │    │           │   │         │
      │  - Login   │    │  - Posts  │   │ - CRUD  │
      │  - JWT     │    │  - Events │   │ - Photos│
      │  - RBAC    │    │  - Forums │   │ - Prefs │
      └──────┬─────┘    └─────┬─────┘   └────┬────┘
             │                 │              │
             └─────────┬───────┴──────────────┘
                       │
                       ▼
      ┌────────────────────────────────────────┐
      │  Infrastructure Services               │
      │  - PostgreSQL (territory_DK schema)    │
      │  - Redis (dk:* keys)                   │
      │  - NATS (territory.dk.* topics)        │
      │  - IPFS (file storage)                 │
      └────────────────────────────────────────┘
```

### Microservices List

| Service            | Container Name                | Port | Function                          |
|--------------------|-------------------------------|------|-----------------------------------|
| **API Gateway**    | `service-api-gateway-{POD}`  | 8080 | HTTP entry, routing, validation   |
| **Auth Service**   | `service-auth-{POD}`         | Internal | JWT, SSO, RBAC           |
| **Community**      | `service-community-{POD}`    | Internal | Posts, discussions, forums |
| **User Service**   | `service-user-{POD}`         | Internal | Profiles, preferences      |
| **Event Service**  | `service-event-{POD}`        | Internal | Event sourcing, audit trail |
| **Search Service** | `service-search-{POD}`       | Internal | Meilisearch integration    |
| **WebSocket**      | `service-websocket-{POD}`    | 9000 | Real-time updates               |

### Docker Compose Configuration

```yaml
# docker-compose.pod.yml (full application services)
services:
  # =================================================================
  # APPLICATION SERVICES
  # =================================================================
  
  # API Gateway - HTTP Entry Point
  api-gateway:
    image: ${DOCKER_REGISTRY}/unityplatform/api-gateway:${VERSION:-latest}
    container_name: service-api-gateway-${POD_ID}
    ports:
      - "${API_PORT:-8080}:8080"
    environment:
      - RUST_LOG=info,actix_web=debug
      - DATABASE_URL=postgres://unityplatform:${POSTGRES_PASSWORD}@service-postgres-${POD_ID}:5432/${POSTGRES_DB}
      - REDIS_URL=redis://:${REDIS_PASSWORD}@service-redis-${POD_ID}:6379
      - NATS_URL=nats://service-nats-${POD_ID}:4222
      - TERRITORY_ID=${TERRITORY_ID}
      - JWT_SECRET=${JWT_SECRET}
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
      nats:
        condition: service_healthy
    networks:
      - pod-${POD_ID}-net
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  # Auth Service - Authentication & Authorization
  auth:
    image: ${DOCKER_REGISTRY}/unityplatform/auth-service:${VERSION:-latest}
    container_name: service-auth-${POD_ID}
    environment:
      - RUST_LOG=info
      - DATABASE_URL=postgres://unityplatform:${POSTGRES_PASSWORD}@service-postgres-${POD_ID}:5432/${POSTGRES_DB}
      - REDIS_URL=redis://:${REDIS_PASSWORD}@service-redis-${POD_ID}:6379
      - JWT_SECRET=${JWT_SECRET}
      - JWT_EXPIRY=3600
      - TERRITORY_ID=${TERRITORY_ID}
      - OIDC_ISSUER=${OIDC_ISSUER:-}
      - OIDC_CLIENT_ID=${OIDC_CLIENT_ID:-}
    depends_on:
      - postgres
      - redis
    networks:
      - pod-${POD_ID}-net
    restart: unless-stopped

  # Community Service - Posts, Discussions, Forums
  community:
    image: ${DOCKER_REGISTRY}/unityplatform/community-service:${VERSION:-latest}
    container_name: service-community-${POD_ID}
    environment:
      - RUST_LOG=info
      - DATABASE_URL=postgres://unityplatform:${POSTGRES_PASSWORD}@service-postgres-${POD_ID}:5432/${POSTGRES_DB}
      - NATS_URL=nats://service-nats-${POD_ID}:4222
      - REDIS_URL=redis://:${REDIS_PASSWORD}@service-redis-${POD_ID}:6379
      - TERRITORY_ID=${TERRITORY_ID}
    depends_on:
      - postgres
      - nats
      - redis
    networks:
      - pod-${POD_ID}-net
    restart: unless-stopped

  # User Service - Profiles, Preferences
  user:
    image: ${DOCKER_REGISTRY}/unityplatform/user-service:${VERSION:-latest}
    container_name: service-user-${POD_ID}
    environment:
      - RUST_LOG=info
      - DATABASE_URL=postgres://unityplatform:${POSTGRES_PASSWORD}@service-postgres-${POD_ID}:5432/${POSTGRES_DB}
      - IPFS_URL=http://service-ipfs-${POD_ID}:5001
      - NATS_URL=nats://service-nats-${POD_ID}:4222
      - TERRITORY_ID=${TERRITORY_ID}
    depends_on:
      - postgres
      - ipfs
      - nats
    networks:
      - pod-${POD_ID}-net
    restart: unless-stopped

  # Event Service - Event Sourcing, Audit Trail
  event:
    image: ${DOCKER_REGISTRY}/unityplatform/event-service:${VERSION:-latest}
    container_name: service-event-${POD_ID}
    environment:
      - RUST_LOG=info
      - DATABASE_URL=postgres://unityplatform:${POSTGRES_PASSWORD}@service-postgres-${POD_ID}:5432/${POSTGRES_DB}
      - NATS_URL=nats://service-nats-${POD_ID}:4222
      - TERRITORY_ID=${TERRITORY_ID}
    depends_on:
      - postgres
      - nats
    networks:
      - pod-${POD_ID}-net
    restart: unless-stopped

  # WebSocket Gateway - Real-time Updates
  websocket:
    image: ${DOCKER_REGISTRY}/unityplatform/websocket-gateway:${VERSION:-latest}
    container_name: service-websocket-${POD_ID}
    ports:
      - "${WEBSOCKET_PORT:-9000}:9000"
    environment:
      - RUST_LOG=info,tokio_tungstenite=debug
      - REDIS_URL=redis://:${REDIS_PASSWORD}@service-redis-${POD_ID}:6379
      - NATS_URL=nats://service-nats-${POD_ID}:4222
      - TERRITORY_ID=${TERRITORY_ID}
    depends_on:
      - redis
      - nats
    networks:
      - pod-${POD_ID}-net
      - mesh-network
    restart: unless-stopped

  # Frontend - Static React App
  frontend:
    image: ${DOCKER_REGISTRY}/unityplatform/frontend:${VERSION:-latest}
    container_name: app-frontend-${POD_ID}
    ports:
      - "${FRONTEND_PORT:-3000}:80"
    environment:
      - API_URL=http://api-gateway-${POD_ID}:8080
      - WEBSOCKET_URL=ws://websocket-${POD_ID}:9000
      - TERRITORY_ID=${TERRITORY_ID}
      - TERRITORY_NAME=${TERRITORY_NAME}
      - TERRITORY_LOCALE=${TERRITORY_LOCALE}
      - ENVIRONMENT=${ENVIRONMENT:-production}
    networks:
      - pod-${POD_ID}-net
      - mesh-network
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.frontend-${POD_ID}.rule=Host(`${TERRITORY_ID}.unityplatform.org`)"
    restart: unless-stopped
```

### Multi-Territory Routing (Europe Pod)

For **multi-territory pods**, the API Gateway routes requests to the correct database:

```rust
// services/api-gateway/src/territory_router.rs
use actix_web::{web, HttpRequest, HttpResponse, Result};
use sqlx::PgPool;

pub struct TerritoryRouter {
    pools: HashMap<String, PgPool>,
}

impl TerritoryRouter {
    pub async fn new(pod_id: &str) -> Result<Self> {
        let mut pools = HashMap::new();
        
        // For multi-territory pods (e.g., Europe)
        if pod_id == "eu" {
            pools.insert("DE".to_string(), 
                PgPool::connect("postgres://...@postgres-eu:5432/unityplatform_de").await?);
            pools.insert("FR".to_string(), 
                PgPool::connect("postgres://...@postgres-eu:5432/unityplatform_fr").await?);
            pools.insert("ES".to_string(), 
                PgPool::connect("postgres://...@postgres-eu:5432/unityplatform_es").await?);
        } else {
            // Single-territory pod
            pools.insert(pod_id.to_uppercase(), 
                PgPool::connect(&format!("postgres://...@postgres-{}:5432/unityplatform_{}", 
                    pod_id, pod_id)).await?);
        }
        
        Ok(Self { pools })
    }
    
    pub fn get_pool(&self, territory_id: &str) -> Result<&PgPool, AppError> {
        self.pools.get(territory_id)
            .ok_or(AppError::UnknownTerritory(territory_id.to_string()))
    }
}

// Detect territory from subdomain or JWT token
pub fn extract_territory_id(req: &HttpRequest) -> Result<String> {
    // Option 1: From subdomain (de.europe.unityplatform.org)
    if let Some(host) = req.headers().get("host") {
        if let Ok(host_str) = host.to_str() {
            if let Some(subdomain) = host_str.split('.').next() {
                if ["de", "fr", "es"].contains(&subdomain) {
                    return Ok(subdomain.to_uppercase());
                }
            }
        }
    }
    
    // Option 2: From JWT token claim
    if let Some(jwt) = extract_jwt_from_header(req) {
        if let Ok(claims) = verify_jwt(&jwt) {
            return Ok(claims.territory_id);
        }
    }
    
    // Option 3: From X-Territory-ID header
    if let Some(territory) = req.headers().get("x-territory-id") {
        return Ok(territory.to_str()?.to_uppercase());
    }
    
    Err(AppError::MissingTerritoryId)
}

// Usage in endpoint
async fn get_communities(
    req: HttpRequest,
    router: web::Data<TerritoryRouter>,
) -> Result<HttpResponse> {
    let territory_id = extract_territory_id(&req)?;
    let pool = router.get_pool(&territory_id)?;
    
    let communities = sqlx::query_as!(
        Community,
        "SELECT * FROM territory_{}.communities WHERE active = true",
        territory_id.to_lowercase()
    )
    .fetch_all(pool)
    .await?;
    
    Ok(HttpResponse::Ok().json(communities))
}
```

---

## Staging Environment

### Overview

**Staging mirrors production** but with:

- Smaller resource allocation (50% of production)
- Separate domains: `staging.unityplatform.org`
- Same code, different data
- Auto-deployed from `develop` branch

### Staging Pod Structure

```
Production                    Staging
┌──────────────┐             ┌──────────────────┐
│ pod-dk       │             │ staging-pod-dk   │
│ 192.168.60.  │             │ 192.168.70.      │
│ Ports: 543x  │      →      │ Ports: 1543x     │
│ Resources:   │             │ Resources:       │
│ - 4 CPU      │             │ - 2 CPU          │
│ - 8GB RAM    │             │ - 4GB RAM        │
└──────────────┘             └──────────────────┘
```

### Staging Configuration

```bash
# pods/denmark/.env.staging
ENVIRONMENT=staging
POD_ID=dk
TERRITORY_ID=DK
TERRITORY_NAME=Denmark

# Port offsets (+10000 to avoid conflicts)
POSTGRES_PORT=15432
REDIS_PORT=16379
NATS_CLIENT_PORT=14222
NATS_CLUSTER_PORT=16222
NATS_MONITOR_PORT=18222
API_PORT=18080
FRONTEND_PORT=13000
WEBSOCKET_PORT=19000

# Smaller resource limits
POSTGRES_MAX_CONNECTIONS=50     # vs 200 production
POSTGRES_SHARED_BUFFERS=512MB   # vs 2GB production
REDIS_MAXMEMORY=512mb           # vs 2gb production
NATS_MAX_PAYLOAD=2MB            # vs 8MB production

# Staging-specific settings
DEBUG_MODE=true
LOG_LEVEL=debug
ENABLE_PROFILING=true

# Same structure as production
POSTGRES_DB=unityplatform_dk
REDIS_PASSWORD=${REDIS_PASSWORD_STAGING}
JWT_SECRET=${JWT_SECRET_STAGING}
```

### Staging Deployment

```bash
# Deploy all staging pods
./scripts/deploy-staging.sh
```

```bash
#!/bin/bash
# scripts/deploy-staging.sh

set -e

echo "🚀 Deploying Staging Environment"

# Create staging network
docker network create unityplatform-staging-mesh || true

# Deploy staging pods
for POD in denmark norway sweden europe; do
    echo "📦 Deploying staging-pod-${POD}..."
    
    docker compose \
        -f docker-compose.staging.yml \
        -p staging-pod-${POD} \
        --env-file pods/${POD}/.env.staging \
        up -d
    
    echo "✅ staging-pod-${POD} deployed"
done

echo "🎉 Staging environment ready at https://staging.unityplatform.org"
```

### Staging Compose File

```yaml
# docker-compose.staging.yml
name: staging-pod-${POD_ID}

services:
  # Same services as production
  # But with:
  # - Different port mappings (+ 10000)
  # - Smaller resource limits
  # - Debug mode enabled
  # - staging- prefix on container names
  
  postgres:
    image: postgres:${POSTGRES_VERSION:-16-alpine}
    container_name: staging-postgres-${POD_ID}
    environment:
      - POSTGRES_USER=unityplatform
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
      - POSTGRES_DB=${POSTGRES_DB}
      # Reduced limits
      - POSTGRES_SHARED_BUFFERS=${POSTGRES_SHARED_BUFFERS:-512MB}
      - POSTGRES_MAX_CONNECTIONS=${POSTGRES_MAX_CONNECTIONS:-50}
    ports:
      - "${POSTGRES_PORT}:5432"
    volumes:
      - staging-postgres-data-${POD_ID}:/var/lib/postgresql/data
    networks:
      - staging-pod-${POD_ID}-net
    restart: unless-stopped

  # ... (all other services follow same pattern)

volumes:
  staging-postgres-data-${POD_ID}:
  staging-redis-data-${POD_ID}:
  # ...

networks:
  staging-pod-${POD_ID}-net:
    driver: bridge
  staging-mesh-network:
    external: true
```

---

## CI/CD Pipeline

### Architecture

```
┌──────────────┐
│  Developer   │
│  git push    │
└──────┬───────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│  GitHub / Gitea Repository                              │
│  - develop branch → auto-deploy to staging              │
│  - main branch → manual approval → production           │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│  Drone CI/CD Server                                     │
│  global-drone-server:8000                               │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Pipeline Steps:                                   │  │
│  │ 1. Clone repository                               │  │
│  │ 2. Run tests (cargo test, npm test)              │  │
│  │ 3. Build Docker images                            │  │
│  │ 4. Push to registry                               │  │
│  │ 5. Deploy to staging (auto)                       │  │
│  │ 6. Run integration tests                          │  │
│  │ 7. Deploy to production (manual approval)         │  │
│  └───────────────────────────────────────────────────┘  │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│  Docker Registry                                        │
│  global-registry:5000                                   │
│  - unityplatform/frontend:v1.2.3                            │
│  - unityplatform/api-gateway:v1.2.3                         │
│  - unityplatform/auth-service:v1.2.3                        │
│  - unityplatform/community-service:v1.2.3                   │
└──────┬──────────────────────────────────────────────────┘
       │
       ├──────────────────┬──────────────────┐
       ▼                  ▼                  ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  Staging     │  │  Staging     │  │  Staging     │
│  Pod DK      │  │  Pod NO      │  │  Pod SE      │
└──────┬───────┘  └──────────────┘  └──────────────┘
       │
       │ (Manual Approval after QA)
       ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ Production   │  │ Production   │  │ Production   │
│  Pod DK      │  │  Pod NO      │  │  Pod SE      │
└──────────────┘  └──────────────┘  └──────────────┘
```

### CI/CD Services

```yaml
# docker-compose.dev.yml (add CI/CD services)
services:
  # Drone CI/CD Server
  drone-server:
    image: drone/drone:2
    container_name: global-drone-server
    ports:
      - "8000:80"
      - "8443:443"
    environment:
      - DRONE_GITEA_SERVER=${GITEA_SERVER:-http://gitea:3000}
      - DRONE_GITEA_CLIENT_ID=${GITEA_CLIENT_ID}
      - DRONE_GITEA_CLIENT_SECRET=${GITEA_CLIENT_SECRET}
      - DRONE_SERVER_HOST=drone.unityplatform.local
      - DRONE_SERVER_PROTO=http
      - DRONE_RPC_SECRET=${DRONE_RPC_SECRET}
      - DRONE_USER_CREATE=username:admin,admin:true
    volumes:
      - drone-data:/data
    networks:
      - mesh-network
    restart: unless-stopped

  # Drone Docker Runner
  drone-runner:
    image: drone/drone-runner-docker:1
    container_name: global-drone-runner
    environment:
      - DRONE_RPC_PROTO=http
      - DRONE_RPC_HOST=drone-server
      - DRONE_RPC_SECRET=${DRONE_RPC_SECRET}
      - DRONE_RUNNER_CAPACITY=2
      - DRONE_RUNNER_NAME=docker-runner-1
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    networks:
      - mesh-network
    depends_on:
      - drone-server
    restart: unless-stopped

  # Docker Registry - Store Built Images
  registry:
    image: registry:2
    container_name: global-registry
    ports:
      - "5000:5000"
    environment:
      - REGISTRY_STORAGE_DELETE_ENABLED=true
      - REGISTRY_HTTP_SECRET=${REGISTRY_SECRET}
    volumes:
      - registry-data:/var/lib/registry
    networks:
      - mesh-network
    restart: unless-stopped

  # Gitea - Git Repository (Optional, can use GitHub)
  gitea:
    image: gitea/gitea:latest
    container_name: global-gitea
    ports:
      - "3000:3000"
      - "2222:22"
    environment:
      - USER_UID=1000
      - USER_GID=1000
      - GITEA__database__DB_TYPE=postgres
      - GITEA__database__HOST=service-postgres-dk:5432
      - GITEA__database__NAME=gitea
      - GITEA__database__USER=unityplatform
      - GITEA__database__PASSWD=${POSTGRES_PASSWORD}
    volumes:
      - gitea-data:/data
      - /etc/timezone:/etc/timezone:ro
      - /etc/localtime:/etc/localtime:ro
    networks:
      - mesh-network
    restart: unless-stopped

volumes:
  drone-data:
  registry-data:
  gitea-data:
```

### Pipeline Definition

```yaml
# .drone.yml (Backend Services)
kind: pipeline
type: docker
name: backend-services

steps:
  # Step 1: Build Rust workspace
  - name: build-rust
    image: rust:1.91-alpine
    commands:
      - apk add --no-cache musl-dev openssl-dev
      - cd services
      - cargo build --release --workspace
      - cargo test --workspace
    when:
      branch:
        - develop
        - main

  # Step 2: Build Docker images for each service
  - name: build-api-gateway
    image: plugins/docker
    settings:
      registry: registry:5000
      repo: registry:5000/unityplatform/api-gateway
      tags:
        - latest
        - ${DRONE_COMMIT_SHA:0:8}
        - ${DRONE_BRANCH}
      dockerfile: services/api-gateway/Dockerfile
      context: services/api-gateway
    when:
      event:
        - push
        - tag

  - name: build-auth-service
    image: plugins/docker
    settings:
      registry: registry:5000
      repo: registry:5000/unityplatform/auth-service
      tags: [latest, ${DRONE_COMMIT_SHA:0:8}]
      dockerfile: services/auth-service/Dockerfile
      context: services/auth-service

  - name: build-community-service
    image: plugins/docker
    settings:
      registry: registry:5000
      repo: registry:5000/unityplatform/community-service
      tags: [latest, ${DRONE_COMMIT_SHA:0:8}]
      dockerfile: services/community-service/Dockerfile
      context: services/community-service

  # Step 3: Deploy to staging (auto on develop branch)
  - name: deploy-staging
    image: docker:latest
    environment:
      DOCKER_HOST: tcp://192.168.70.1:2376  # Staging host
    commands:
      - docker compose -f docker-compose.staging.yml pull
      - docker compose -f docker-compose.staging.yml up -d --no-deps api-gateway auth community user
    when:
      branch:
        - develop
      event:
        - push

  # Step 4: Run integration tests on staging
  - name: integration-tests
    image: rust:1.91-alpine
    commands:
      - cd tests/integration
      - cargo test -- --test-threads=1
    environment:
      API_URL: http://staging-api-gateway-dk:18080
    when:
      branch:
        - develop

  # Step 5: Deploy to production (manual approval required)
  - name: deploy-production
    image: docker:latest
    environment:
      DOCKER_HOST: tcp://192.168.60.1:2376  # Production host
    commands:
      - |
        for POD in dk no se eu; do
          echo "Deploying to pod-$POD..."
          docker compose -f docker-compose.pod.yml -p pod-$POD pull
          docker compose -f docker-compose.pod.yml -p pod-$POD up -d --no-deps api-gateway auth community user
          sleep 30  # Wait for health checks
        done
    when:
      branch:
        - main
      event:
        - promote  # Requires manual trigger

---
kind: pipeline
type: docker
name: frontend

steps:
  # Step 1: Build React app
  - name: build-frontend
    image: node:20-alpine
    commands:
      - cd frontend
      - npm ci
      - npm run lint
      - npm run test
      - npm run build

  # Step 2: Build Docker image
  - name: docker-build
    image: plugins/docker
    settings:
      registry: registry:5000
      repo: registry:5000/unityplatform/frontend
      tags:
        - latest
        - ${DRONE_COMMIT_SHA:0:8}
      dockerfile: frontend/Dockerfile
      context: frontend

  # Step 3: Deploy to staging
  - name: deploy-staging
    image: docker:latest
    commands:
      - docker compose -f docker-compose.staging.yml pull frontend
      - docker compose -f docker-compose.staging.yml up -d --no-deps frontend
    when:
      branch:
        - develop

  # Step 4: Deploy to production
  - name: deploy-production
    image: docker:latest
    commands:
      - |
        for POD in dk no se eu; do
          docker compose -f docker-compose.pod.yml -p pod-$POD pull frontend
          docker compose -f docker-compose.pod.yml -p pod-$POD up -d --no-deps frontend
        done
    when:
      branch:
        - main
      event:
        - promote
```

---

## Deployment Workflows

### Development Workflow

```
┌─────────────────────────────────────────────────────────┐
│ 1. Developer writes code locally                        │
│    - Test with docker-compose.dev.yml                   │
│    - Uses local dev database                            │
│    - Hot-reload enabled (Vite, cargo-watch)             │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 2. Push to develop branch                               │
│    - git push origin develop                            │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 3. Drone CI triggers                                    │
│    - Runs tests                                         │
│    - Builds Docker images                               │
│    - Tags: latest, develop, commit-sha                  │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 4. Auto-deploy to staging                               │
│    - Pulls new images                                   │
│    - Rolling restart (--no-deps)                        │
│    - Runs integration tests                             │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 5. QA testing on staging.unityplatform.org                  │
│    - Manual testing                                     │
│    - Automated E2E tests                                │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 6. Merge to main (if tests pass)                        │
│    - git checkout main                                  │
│    - git merge develop                                  │
│    - git push origin main                               │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 7. Drone builds production images                       │
│    - Tags: latest, main, v1.2.3 (if git tag)            │
│    - Waits for manual approval                          │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 8. Manual promotion to production                       │
│    - Drone UI: Click "Promote"                          │
│    - Or CLI: drone build promote <repo> <build> prod    │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 9. Rolling deployment to all pods                       │
│    - Deploy to pod-dk (wait for health check)           │
│    - Deploy to pod-no (wait for health check)           │
│    - Deploy to pod-se (wait for health check)           │
│    - Deploy to pod-eu (wait for health check)           │
└─────────────────────────────────────────────────────────┘
```

### Hotfix Workflow

```
┌─────────────────────────────────────────────────────────┐
│ 1. Create hotfix branch from main                       │
│    - git checkout -b hotfix/critical-bug main           │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 2. Fix bug, commit, push                                │
│    - git push origin hotfix/critical-bug                │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 3. Drone CI runs tests                                  │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 4. Merge to main (fast-track)                           │
│    - git checkout main                                  │
│    - git merge hotfix/critical-bug                      │
│    - git tag v1.2.4                                     │
│    - git push origin main --tags                        │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 5. Immediate promote to production                      │
│    - drone build promote unityplatform/platform 123 prod    │
└──────┬──────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ 6. Backport to develop                                  │
│    - git checkout develop                               │
│    - git merge main                                     │
└─────────────────────────────────────────────────────────┘
```

---

## Rollback Procedures

### Quick Rollback (Last Known Good)

```bash
#!/bin/bash
# scripts/rollback.sh

POD_ID=${1:-dk}
SERVICE=${2:-api-gateway}
PREVIOUS_VERSION=${3:-}

echo "🔄 Rolling back $SERVICE on pod-$POD_ID to version $PREVIOUS_VERSION"

if [ -z "$PREVIOUS_VERSION" ]; then
    # Rollback to 'stable' tag
    PREVIOUS_VERSION="stable"
fi

# Pull previous version
docker pull registry:5000/unityplatform/$SERVICE:$PREVIOUS_VERSION

# Update pod to use previous version
docker compose -f docker-compose.pod.yml -p pod-$POD_ID \
    up -d --no-deps --force-recreate $SERVICE

# Wait for health check
sleep 30

# Verify health
docker exec service-$SERVICE-$POD_ID curl -f http://localhost:8080/health

echo "✅ Rollback complete"
```

### Database Migration Rollback

```bash
#!/bin/bash
# scripts/rollback-migration.sh

POD_ID=${1:-dk}
MIGRATION_VERSION=${2:-}

echo "🔄 Rolling back database migration on pod-$POD_ID to $MIGRATION_VERSION"

# Run sqlx migration revert
docker exec service-api-gateway-$POD_ID \
    sqlx migrate revert --target-version $MIGRATION_VERSION

echo "✅ Migration rollback complete"
```

### Full Pod Rollback

```bash
#!/bin/bash
# scripts/rollback-pod.sh

POD_ID=${1:-dk}
SNAPSHOT_DATE=${2:-$(date -d "1 hour ago" +%Y%m%d_%H%M%S)}

echo "🔄 Rolling back entire pod-$POD_ID to snapshot $SNAPSHOT_DATE"

# Stop pod
docker compose -f docker-compose.pod.yml -p pod-$POD_ID down

# Restore database backup
docker run --rm \
    -v pod-$POD_ID-postgres-data:/backup \
    -v ./backups:/host-backups \
    alpine sh -c "cd /backup && tar xzf /host-backups/postgres-$POD_ID-$SNAPSHOT_DATE.tar.gz"

# Restore Redis snapshot
docker run --rm \
    -v pod-$POD_ID-redis-data:/backup \
    -v ./backups:/host-backups \
    alpine sh -c "cp /host-backups/redis-$POD_ID-$SNAPSHOT_DATE.rdb /backup/dump.rdb"

# Restart pod
docker compose -f docker-compose.pod.yml -p pod-$POD_ID up -d

echo "✅ Pod rollback complete"
```

---

## Summary

### Application Service Deployment Model

✅ **Per-Pod Deployment**: Each pod runs complete copy of frontend + backend  
✅ **Territory Isolation**: Application layer respects territory boundaries  
✅ **Multi-Territory Routing**: API Gateway routes to correct database in multi-territory pods  
✅ **Port Allocation**: Systematic offsets per pod (DK: base, NO: +100, SE: +200, EU: +300)

### Staging Environment

✅ **Production Mirror**: Same architecture, smaller resources  
✅ **Separate Infrastructure**: Different ports/networks to avoid conflicts  
✅ **Auto-Deploy**: `develop` branch → staging automatically  
✅ **QA Testing**: Validate before production release

### CI/CD Pipeline

✅ **Docker-Based**: Drone CI/CD running as global service  
✅ **Automated Testing**: Unit tests, integration tests, E2E tests  
✅ **Docker Registry**: Centralized image storage (registry:5000)  
✅ **Rolling Updates**: Deploy pods one at a time, zero downtime  
✅ **Manual Approval**: Production deployments require promotion  
✅ **Rollback Capability**: Quick revert to previous version

---

**Next Steps:**

1. Implement Rust backend services (Phase 1.3)
2. Build frontend React app with TanStack Router
3. Deploy Drone CI/CD server
4. Create staging pod configurations
5. Test deployment workflows

**Related Documentation:**

- [Multi-Pod Architecture](./5-multi-pod-architecture.md)
- [Multi-Pod Deployment Guide](./6-multi-pod-deployment-guide.md)
- [Territory Management Standard](./9-territory-management-standard.md)
