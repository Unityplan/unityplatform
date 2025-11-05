# UnityPlan Documentation

Welcome to the UnityPlan platform documentation. This guide will help you navigate the documentation structure and find the information you need.

## 📚 Documentation Structure

```
docs/
├── project/           # WHAT - Project definition
├── architecture/      # HOW - System design
├── guides/           # DO - Implementation guides
└── status/           # TRACK - Current progress
```

## 📖 Quick Navigation

### 🎯 Project Definition (WHAT)
Start here to understand what UnityPlan is and its core purpose.

- **[Summary](project/summary.md)** - Executive overview of the UnityPlan platform
- **[Overview](project/overview.md)** - Comprehensive project overview with detailed feature descriptions
- **[Tech Stack](project/tech-stack.md)** - Complete technology stack and tool choices

### 🏗️ Architecture (HOW)
Learn how the system is designed and structured.

- **[Infrastructure](architecture/infrastructure.md)** - Infrastructure design and pod architecture
- **[Multi-Pod Architecture](architecture/multi-pod-architecture.md)** - Distributed pod deployment model
- **[Territory Management Standard](architecture/territory-management-standard.md)** - **CRITICAL** Territory ID format standard (countries, First Nations, communities)

### 🛠️ Implementation Guides (DO)

#### Deployment
- **[Multi-Pod Deployment](guides/deployment/multi-pod-deployment.md)** - How to deploy multiple territory pods
- **[NATS Clustering](guides/deployment/nats-clustering.md)** - NATS mesh network setup for cross-pod communication
- **[Testing & Verification](guides/deployment/testing-verification.md)** - How to verify deployment health
- **[CI/CD](guides/deployment/cicd.md)** - Continuous integration and deployment setup
- **[Deployment Notes](guides/deployment/deployment-notes.md)** - Lessons learned and troubleshooting

#### Development
- **[Rust Backend Plan](guides/development/rust-backend-plan.md)** - Comprehensive Rust microservices development roadmap
- **[Development Tools](guides/development/development-tools.md)** - Development environment setup and tools

#### Operations
- **[Forgejo MCP Setup](guides/operations/forgejo-mcp-setup.md)** - Forgejo git server and MCP integration
- **[Migration Notes](guides/operations/migration-notes.md)** - Migration from old to new architecture

### 📊 Current Status (TRACK)

#### Roadmaps
- **[Phase 1: MVP](status/roadmaps/phase-1-mvp.md)** - Minimum viable product roadmap
- **[Phase 2: Scale](status/roadmaps/phase-2-scale.md)** - Scaling and optimization roadmap
- **[Phase 3: Decentralization](status/roadmaps/phase-3-decentralization.md)** - Holochain integration roadmap

#### Current Progress
- **[Phase 1 Status](status/current/phase-1-status.md)** - Current implementation status
- **[Phase 1 Checklist](status/current/phase-1-checklist.md)** - Implementation task checklist
- **[Phase 1 Approach](status/current/phase-1-approach.md)** - Development approach and methodology

## 🚀 Quick Start Paths

### For New Developers
1. Read [Summary](project/summary.md) for context
2. Review [Tech Stack](project/tech-stack.md) to understand technologies
3. Check [Phase 1 Status](status/current/phase-1-status.md) for current state
4. Follow [Development Tools](guides/development/development-tools.md) setup

### For DevOps/Deployment
1. Read [Infrastructure](architecture/infrastructure.md)
2. Follow [Multi-Pod Deployment](guides/deployment/multi-pod-deployment.md)
3. Set up [NATS Clustering](guides/deployment/nats-clustering.md)
4. Verify with [Testing & Verification](guides/deployment/testing-verification.md)

### For Backend Developers
1. Review [Rust Backend Plan](guides/development/rust-backend-plan.md)
2. Understand [Territory Management Standard](architecture/territory-management-standard.md)
3. Check database schema in [Phase 1 Status](status/current/phase-1-status.md)

## ⚠️ Critical Documents

These documents define fundamental platform standards and **MUST NOT** be modified without comprehensive review:

- **[Territory Management Standard](architecture/territory-management-standard.md)** - Territory ID format (affects database, APIs, permissions, routing)

## 🗂️ Legacy Documentation

Previous documentation folders have been archived:
- `docs-archived/` - Old operational documentation (to be deleted)
- `project_docs/` - Migrated to docs/ structure (can be deleted after verification)
- `project_status/` - Migrated to docs/status/ (can be deleted after verification)

---

**Last Updated:** November 5, 2025  
**Maintained By:** UnityPlan Development Team
