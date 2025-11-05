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
- **Node.js** (v18+) and **pnpm** (recommended) or npm
- **SQLx CLI**: `cargo install sqlx-cli --no-default-features --features postgres`

### Quick Start (Development)

1. **Clone and initialize:**
   ```bash
   git clone <repository-url>
   cd workspace
   ```

2. **Set up environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start infrastructure:**
   ```bash
   docker-compose up -d
   ```

4. **Run database migrations:**
   ```bash
   cd services
   sqlx migrate run
   ```

5. **Start backend services:**
   ```bash
   # In services/ directory
   cargo run --bin auth-service
   # Repeat for other services
   ```

6. **Start frontend:**
   ```bash
   cd frontend
   pnpm install
   pnpm dev
   ```

7. **Access the application and development tools:**
   
   **Application:**
   - Frontend: http://localhost:5173
   - API Gateway: http://localhost:8000
   
   **Infrastructure Management:**
   - Adminer (PostgreSQL UI): http://localhost:8080
   - Redis Commander: http://localhost:8082
   - MailHog (Email Testing): http://localhost:8025
   
   **Observability & Monitoring:**
   - Prometheus (Metrics): http://localhost:9090
   - Grafana (Dashboards): http://localhost:3001 (admin/admin)
   - Jaeger (Tracing): http://localhost:16686
   - Traefik Dashboard: http://localhost:8083/dashboard/
   - NATS Monitoring: http://localhost:8222

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

- **[Project Summary](./project_docs/1-project-summary.md)** - Executive overview
- **[Project Overview](./project_docs/2-project-overview.md)** - Detailed project description
- **[Tech Stack](./project_docs/3-project-techstack.md)** - Technology documentation
- **[Infrastructure](./project_docs/4-project-infrastructure.md)** - Infrastructure architecture
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
