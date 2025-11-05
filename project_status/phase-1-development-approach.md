# Phase 1 Development Approach

**Date:** November 5, 2025  
**Status:** Active Development Phase  
**Approach:** Minimal, Pragmatic, Fast Iteration

---

## Overview

Phase 1 focuses on **building the MVP** with minimal infrastructure overhead, allowing fast iteration and AI-assisted development.

## What We're Using (Phase 1)

### ✅ Essential Tools

| Tool | Purpose | Why Now? |
|------|---------|----------|
| **Forgejo** | Version control + MCP integration | • Self-hosted Git<br>• AI coding assistance via MCP<br>• Foundation for future CI/CD |
| **Docker Registry** | Local image storage | • Store built images locally<br>• Quick iteration<br>• No external dependencies |
| **Manual Builds** | Build process | • Fast iteration<br>• Full control<br>• No CI/CD overhead |
| **Docker Compose** | Orchestration | • Simple deployment<br>• Easy to understand<br>• Matches production |

### ⏳ Deferred to Phase 1.5 / Phase 2

| Tool | Purpose | When to Add? |
|------|---------|--------------|
| **CI/CD Pipeline** | Automated testing & deployment | When:<br>• Multiple developers<br>• Deploying to multiple pods<br>• Frequent releases |
| **Staging Environment** | Pre-production testing | When:<br>• Need QA environment<br>• Production deployments start<br>• Multi-pod setup |
| **Automated Testing** | Integration/E2E tests | When:<br>• Core features stable<br>• Need regression testing<br>• CI/CD pipeline ready |

---

## Phase 1 Development Workflow

### Daily Development

```bash
# 1. Start development environment
./scripts/start-phase1-dev.sh

# 2. Edit code (AI-assisted via Forgejo MCP)
# Your editor/AI connects to Forgejo for repository context

# 3. Build Rust backend
cd services
cargo build --release
cargo test

# 4. Build Docker image
docker build -t localhost:5000/unityplan/auth-service:latest \
  -f services/auth-service/Dockerfile .

# 5. Push to local registry
docker push localhost:5000/unityplan/auth-service:latest

# 6. Deploy locally
docker compose -f docker-compose.pod.yml -p pod-dk up -d auth

# 7. Test
curl http://localhost:8080/api/auth/health

# 8. Commit to Forgejo
git add .
git commit -m "feat: implement JWT validation"
git push forgejo main
```

### Benefits of This Approach

✅ **Fast Iteration**
- No waiting for CI/CD pipelines
- Build and test immediately
- Full control over deployment

✅ **AI Assistance**
- Forgejo MCP provides repository context
- AI can read code, suggest changes
- Smart code completion

✅ **Simple Infrastructure**
- Just Forgejo + Registry (2 containers)
- Easy to understand and debug
- Low resource usage

✅ **Production-Ready Path**
- Docker images already built
- Same workflow as production
- Easy to add CI/CD later

---

## When to Add CI/CD

### Triggers

Add automated CI/CD pipeline when you hit **2+ of these conditions**:

- [ ] **Multiple developers** - Coordination needed between team members
- [ ] **Multiple pods** - Deploying to DK, NO, SE, EU simultaneously
- [ ] **Frequent releases** - Manual builds become tedious (>5/day)
- [ ] **Complex tests** - Integration tests take >5 minutes
- [ ] **Staging needed** - QA team needs dedicated environment
- [ ] **Production deployments** - Need rollback capability

### Options for CI/CD

**Option 1: Forgejo Actions** (GitHub Actions compatible)
```yaml
# .forgejo/workflows/build.yml
name: Build and Test
on: [push]
jobs:
  build:
    runs-on: docker
    steps:
      - uses: actions/checkout@v3
      - run: cargo build --release
      - run: cargo test
```

**Option 2: Woodpecker CI** (lightweight, Docker-native)
```yaml
# .woodpecker.yml
pipeline:
  build:
    image: rust:1.91
    commands:
      - cargo build --release
      - cargo test
```

**Option 3: Drone CI** (more features, enterprise-ready)
- See `project_docs/10-application-deployment-cicd.md`

---

## Phase 1 vs Phase 2 Comparison

| Aspect | Phase 1 (Now) | Phase 2 (Later) |
|--------|---------------|-----------------|
| **Version Control** | ✅ Forgejo + MCP | ✅ Forgejo + MCP |
| **Build Process** | Manual (`cargo build`) | Automated (CI/CD pipeline) |
| **Testing** | Manual (`cargo test`) | Automated (CI runs tests) |
| **Image Storage** | Local registry | Local + Cloud registry |
| **Deployment** | Manual (`docker compose up`) | Automated (CI deploys) |
| **Environments** | Development only | Dev + Staging + Production |
| **Pods** | 1 pod (local testing) | 4 pods (DK, NO, SE, EU) |
| **Team Size** | 1-2 developers | 4-6 developers |
| **Iteration Speed** | ⚡ Instant | 🔄 Minutes (pipeline) |
| **Safety** | Manual review | Automated tests + approval |

---

## Infrastructure Costs

### Phase 1 (Current)

```
Development Machine (Local):
- Forgejo: ~200MB RAM
- Docker Registry: ~100MB RAM
- Total: ~300MB RAM, minimal CPU

Monthly Cost: $0 (runs on dev machine)
```

### Phase 2 (Multi-Pod Production)

```
Production Infrastructure:
- Pod DK (Denmark): $30-50/month
- Pod NO (Norway): $30-50/month
- Pod SE (Sweden): $30-50/month
- Pod EU (Europe multi-territory): $30-50/month
- CI/CD Server: $10-20/month
- Monitoring (global): $10-20/month

Total: ~$150-250/month for 6 territories
```

**Savings in Phase 1:**
- No cloud costs yet
- Build locally, deploy locally
- Add production infrastructure when ready

---

## Current Status

### ✅ Completed

- Multi-pod architecture designed
- Docker Compose files created (dev, monitoring, pod, multi-territory)
- Territory ID Format standard documented
- NATS clustering guide
- Deployment scripts
- Forgejo + Registry added to docker-compose.dev.yml
- Phase 1 startup script (`scripts/start-phase1-dev.sh`)

### 🔄 In Progress

- Setting up Forgejo with MCP integration
- Configuring Docker Registry for insecure localhost
- Ready to start Rust backend development

### ⏳ Next Steps

1. **Start Phase 1 environment:**
   ```bash
   ./scripts/start-phase1-dev.sh
   ```

2. **Configure Forgejo:**
   - Create admin account
   - Create `unityplan_platform` repository
   - Generate API token for MCP

3. **Install forgejo-mcp:**
   ```bash
   npm install -g @goern/forgejo-mcp
   # Configure with Forgejo API token
   ```

4. **Begin Rust development:**
   - Create `services/` workspace
   - Implement `shared-lib` (config, db, nats, error handling)
   - Build first service: `auth-service`

---

## Architecture Decision Record

### Decision: Manual Builds in Phase 1

**Context:**
- Single developer building MVP
- Need fast iteration
- Want AI assistance (Forgejo MCP)
- Don't need multi-environment deployments yet

**Decision:**
Use manual builds with Forgejo + local registry, defer CI/CD to Phase 2.

**Consequences:**

**Positive:**
- ✅ Faster development (no pipeline overhead)
- ✅ Simpler infrastructure (2 containers vs 10+)
- ✅ Lower costs (no cloud infrastructure yet)
- ✅ Full control over builds
- ✅ AI assistance via MCP from day 1

**Negative:**
- ⚠️ Manual testing (must remember to run `cargo test`)
- ⚠️ No automated deployments (manual `docker compose up`)
- ⚠️ Single developer only (doesn't scale to teams yet)

**Mitigation:**
- Add CI/CD in Phase 1.5 when needed
- Forgejo supports Actions (easy to add pipelines later)
- Manual builds teach good habits
- Foundation is already production-ready

**Review Date:** After 1 month of development or when adding second developer

---

## Summary

### Phase 1 Philosophy

> **"Build the MVP first, add automation when it hurts"**

- Start simple: Forgejo + Registry
- Build fast: Manual builds, instant feedback
- AI-assisted: MCP integration from day 1
- Production-ready: Docker images, same workflow
- Add complexity when needed: CI/CD in Phase 2

### Success Criteria

Phase 1 is successful when:
- ✅ Rust backend services running
- ✅ React frontend deployed
- ✅ Authentication working (JWT)
- ✅ Database migrations stable
- ✅ Local testing complete
- ✅ Ready to deploy to first production pod

**Then we add:** CI/CD, staging, multi-pod deployment

---

**Related Documentation:**
- [Forgejo MCP Setup](../docs/forgejo-mcp-setup.md)
- [Application Deployment & CI/CD](../project_docs/10-application-deployment-cicd.md)
- [Phase 1 MVP Roadmap](./phase-1-mvp-roadmap.md)
- [Multi-Pod Architecture](../project_docs/5-multi-pod-architecture.md)
