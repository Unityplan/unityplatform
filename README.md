# UnityPlan Platform

A decentralized learning and communication platform with user sovereignty at its core.

## 🎯 Vision

UnityPlan is a global platform for communication, learning, and community building that implements an "inverted pyramid" model where users have complete sovereignty over their data and democratic control over communities.

## 🏗️ Architecture

This platform follows a microservices architecture with progressive decentralization:

- **Phase 1 (MVP)**: Rust microservices + PostgreSQL multi-tenancy + React frontend
- **Phase 2 (Scale)**: Regional deployment + Kubernetes + Enhanced federation
- **Phase 3 (Decentralization)**: Full Holochain migration + Pure P2P

### Current Architecture (Phase 1)

- **Backend**: Rust-based microservices containerized with Docker
- **Frontend**: React 19 + Vite + TailwindCSS + ShadCN
- **Database**: PostgreSQL 16 + TimescaleDB (schema-per-territory multi-tenancy)
- **Message Bus**: NATS with JetStream
- **Caching**: Redis
- **Storage**: IPFS for content
- **Communication**: Matrix Protocol (Synapse)

## 📁 Project Structure

```
.
├── services/          # Rust microservices
│   ├── auth-service/
│   ├── user-service/
│   ├── territory-service/
│   ├── badge-service/
│   ├── course-service/
│   ├── forum-service/
│   ├── ipfs-service/
│   ├── translation-service/
│   ├── matrix-gateway/
│   └── shared-lib/    # Shared utilities and types
├── frontend/          # Vite + React application
├── docker/            # Docker configuration files
│   ├── postgres/      # PostgreSQL init scripts
│   └── [service-data] # Volume mounts (gitignored)
├── docs/              # General documentation
├── project_docs/      # Project planning documents
│   ├── 1-project-summary.md
│   ├── 2-project-overview.md
│   ├── 3-project-techstack.md
│   └── 4-project-infrastructure.md
├── project_status/    # Implementation tracking
│   ├── phase-1-implementation-checklist.md
│   ├── phase-1-status.md
│   ├── phase-1-mvp-roadmap.md
│   ├── phase-2-scale-roadmap.md
│   └── phase-3-decentralization-roadmap.md
├── scripts/           # Utility scripts
├── temp/              # Temporary work files
└── docker-compose.yml # Service orchestration
```

## 🚀 Getting Started

### Prerequisites

- **Docker** and **Docker Compose** (v2.0+)
- **Rust** (1.91.0 or latest stable)
- **Node.js** (v20+) and **npm**
- **SQLx CLI**: `cargo install sqlx-cli --no-default-features --features postgres`

### Phase 1 Development Setup (Minimal)

1. **Clone repository:**
   ```bash
   git clone <repository-url>
   cd workspace
   ```

2. **Start Phase 1 development environment:**
   ```bash
   # Option 1: Minimal Phase 1 (Forgejo + Registry only)
   ./scripts/start-phase1-dev.sh
   
   # Option 2: Use new architecture script
   ./scripts/start-new-architecture.sh --phase1
   ```
   
   This starts:
   - ✅ Forgejo (version control + MCP integration)
   - ✅ Docker Registry (local image storage)

3. **Configure Forgejo (first-time setup):**
   - Open http://localhost:3000
   - Create admin account
   - Create `unityplan_platform` repository
   - Push code: `git remote add forgejo http://localhost:3000/admin/unityplan_platform.git`

4. **Optional: Install forgejo-mcp for AI assistance:**
   ```bash
   npm install -g @goern/forgejo-mcp
   # See docs/forgejo-mcp-setup.md for configuration
   ```

5. **Configure Docker to use local registry:**
   ```bash
   # Add to /etc/docker/daemon.json:
   { "insecure-registries": ["localhost:5000"] }
   
   sudo systemctl restart docker
   ```

6. **Start building Rust backend:**
   ```bash
   cd services
   cargo build --release
   cargo test
   ```

### Full Development Environment (Optional)

For complete development setup with all tools:

```bash
# Option 1: Using new architecture script
./scripts/start-new-architecture.sh --dev-tools

# Option 2: Using docker compose directly
docker compose -f docker-compose.dev.yml up -d

# Access:
# - Dev Dashboard: http://localhost:8888
# - Adminer (DB UI): http://localhost:8080
# - MailHog: http://localhost:8025
# - Redis Commander: http://localhost:8082
# - Forgejo: http://localhost:3000
# - Docker Registry: http://localhost:5000
```

### Monitoring Stack (Optional)

```bash
# Option 1: Using new architecture script
./scripts/start-new-architecture.sh --monitoring

# Option 2: Using docker compose directly
docker compose -f docker-compose.monitoring.yml up -d

# Access:
# - Prometheus: http://192.168.60.133:9090
# - Grafana: http://192.168.60.133:3001 (admin/admin)
# - Jaeger: http://192.168.60.133:16686
```

### Multi-Pod Deployment (Phase 2)

```bash
# Option 1: Deploy specific pod
./scripts/start-new-architecture.sh --pod dk

# Option 2: Deploy all pods
./scripts/start-new-architecture.sh --all-pods
# Or use the dedicated script:
./scripts/deploy-multi-pod.sh

# Option 3: Start everything (dev tools + monitoring + all pods)
./scripts/start-new-architecture.sh --full

# Verify deployment
./scripts/verify-multi-pod.sh

# See MULTI-POD-README.md for details
```

### Stop Services

```bash
# Stop specific components
./scripts/stop-new-architecture.sh --dev-tools
./scripts/stop-new-architecture.sh --pod dk
./scripts/stop-new-architecture.sh --all-pods

# Stop everything
./scripts/stop-new-architecture.sh --all

# Stop and remove data (WARNING: deletes all data!)
./scripts/stop-new-architecture.sh --all --clean
```
   # Repeat for other services
   ```

6. **Start frontend:**
   ```bash
   cd frontend
   pnpm install
   pnpm dev
   ```

7. **Access the application and development tools:**
   
   **🎯 Development Dashboard (Quick Access):**
   - **Landing Page:** http://192.168.60.133 - Main entry point
   - **Dashboard:** http://192.168.60.133:8888 - All tools in one place!
   
   **Application:**
   - Frontend: http://192.168.60.133:5173
   - API Gateway: http://192.168.60.133:8000
   
   **Infrastructure Management:**
   - Adminer (PostgreSQL UI): http://192.168.60.133:8080
   - Redis Commander: http://192.168.60.133:8082
   - MailHog (Email Testing): http://192.168.60.133:8025
   
   **Observability & Monitoring:**
   - Prometheus (Metrics): http://192.168.60.133:9090
   - Grafana (Dashboards): http://192.168.60.133:3001 (admin/admin)
   - Jaeger (Tracing): http://192.168.60.133:16686
   - Traefik Dashboard: http://192.168.60.133:8083/dashboard/
   - NATS Monitoring: http://192.168.60.133:8222

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific service tests
cargo test -p auth-service

# Run frontend tests
cd frontend && pnpm test

# E2E tests
cd frontend && pnpm test:e2e
```

## 📚 Documentation

### Core Documentation
- **[Project Summary](./project_docs/1-project-summary.md)** - Executive overview
- **[Project Overview](./project_docs/2-project-overview.md)** - Detailed project description with **Territory ID Format**
- **[Tech Stack](./project_docs/3-project-techstack.md)** - Technology documentation
- **[Infrastructure](./project_docs/4-project-infrastructure.md)** - Infrastructure architecture

### Multi-Pod Architecture
- **[Multi-Pod Architecture](./project_docs/5-multi-pod-architecture.md)** - Complete multi-pod design with **Territory ID Format**
- **[Multi-Pod Deployment Guide](./project_docs/6-multi-pod-deployment-guide.md)** - Step-by-step deployment
- **[NATS Clustering Guide](./project_docs/7-nats-clustering-guide.md)** - NATS configuration and operations
- **[Testing & Verification Guide](./project_docs/8-testing-verification-guide.md)** - Comprehensive testing procedures
- **[Multi-Pod Quick Start](./MULTI-POD-README.md)** - Quick reference

### Critical Standards
- **[⚠️ Territory Management Standard](./project_docs/9-territory-management-standard.md)** - **CRITICAL:** Territory ID format for countries, First Nations, and communities

### Implementation Tracking
- **[Phase 1 Checklist](./project_status/phase-1-implementation-checklist.md)** - Implementation guide
- **[Phase 1 Status](./project_status/phase-1-status.md)** - Current progress tracking

## 🎓 Key Concepts

### Inverted Pyramid Model
Traditional hierarchies are inverted - users have the most power at the top, with global admins serving at the bottom.

### User Sovereignty
- Users own their data
- Democratic community governance
- Badge-based permissions (not traditional roles)
- No platform lock-in

### Multi-Tenancy
Each territory (country/region) operates in its own PostgreSQL schema, maintaining data sovereignty while sharing infrastructure.

### Badge-Based Permissions
Access is granted through earning badges by completing courses (e.g., Code of Conduct badge required for forum participation).

## 🛠️ Development Workflow

1. Check [Phase 1 Status](./project_status/phase-1-status.md) for current progress
2. Pick a task from [Phase 1 Checklist](./project_status/phase-1-implementation-checklist.md)
3. Create a feature branch: `git checkout -b feature/task-name`
4. Implement with tests
5. Run tests and ensure they pass
6. Submit PR for review
7. Update status document when complete

## 🤝 Contributing

Contributions are welcome! Please read our contribution guidelines (coming soon) before submitting PRs.

## 📝 License

TBD

## 🔗 Links

- Project Documentation: `./project_docs/`
- Implementation Roadmap: `./project_status/`
- API Documentation: http://localhost:8000/docs (when running)

---

**Current Phase:** Phase 1 - MVP Development  
**Status:** In Planning  
**Target Completion:** 6-9 months
