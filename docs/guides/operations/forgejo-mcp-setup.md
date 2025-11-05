# Forgejo with MCP Integration - Setup Guide

**Purpose:** Self-hosted Git repository with Model Context Protocol for AI-assisted development  
**Phase:** Phase 1 (MVP Development)  
**MCP Integration:** https://codeberg.org/goern/forgejo-mcp

---

## Overview

Forgejo provides version control for the UnityPlan platform with:
- ✅ Self-hosted Git repositories
- ✅ MCP server integration for AI coding assistance
- ✅ Web UI for repository management
- ✅ Lightweight (SQLite for Phase 1)
- ✅ Can add CI/CD later (Forgejo Actions, Woodpecker CI)

---

## Deployment

### 1. Start Forgejo

```bash
# Start Forgejo + Registry
docker compose -f docker-compose.dev.yml up -d forgejo registry

# Check status
docker compose -f docker-compose.dev.yml ps
```

**Access:**
- Web UI: http://localhost:3000
- SSH: `ssh://git@localhost:2222`

### 2. Initial Setup

1. Open http://localhost:3000
2. Click "Register" (first user becomes admin)
3. Create admin account:
   - Username: `admin`
   - Email: `admin@unityplan.local`
   - Password: (choose strong password)

4. Initial configuration (pre-filled):
   - Database: SQLite (fine for Phase 1)
   - Domain: `forgejo.local`
   - SSH Port: `2222`
   - HTTP Port: `3000`

5. Click "Install Forgejo"

### 3. Create UnityPlan Repository

```bash
# Method 1: Web UI
# 1. Click "+" → "New Repository"
# 2. Name: unityplan_platform
# 3. Description: UnityPlan Global Learning Platform
# 4. Visibility: Private
# 5. Initialize with README: No (we have existing code)

# Method 2: CLI
# Add forgejo as remote
cd /home/henrik/code/data/projects/unityplan_platform/workspace
git remote add forgejo http://localhost:3000/admin/unityplan_platform.git

# Push existing code
git push forgejo main
```

---

## MCP Integration Setup

### What is forgejo-mcp?

The `forgejo-mcp` server allows AI assistants (like GitHub Copilot) to:
- Read repository structure
- Search code across branches
- Fetch file contents
- List issues, pull requests
- Create/update files via commits
- Analyze repository context

### Installation

```bash
# Install forgejo-mcp (Node.js based)
npm install -g @goern/forgejo-mcp

# Or with npx (no global install)
npx @goern/forgejo-mcp
```

### Configuration

Create MCP config file:

```json
// ~/.config/mcp/forgejo.json
{
  "mcpServers": {
    "forgejo": {
      "command": "npx",
      "args": ["@goern/forgejo-mcp"],
      "env": {
        "FORGEJO_URL": "http://localhost:3000",
        "FORGEJO_TOKEN": "YOUR_FORGEJO_API_TOKEN"
      }
    }
  }
}
```

### Generate API Token

1. Login to Forgejo: http://localhost:3000
2. Settings → Applications → Generate New Token
3. Token name: `MCP Integration`
4. Permissions:
   - ✅ Read repositories
   - ✅ Write repositories
   - ✅ Read issues
   - ✅ Write issues
5. Copy token and add to config above

### Test MCP Connection

```bash
# Test MCP server
npx @goern/forgejo-mcp --test

# Should output:
# ✅ Connected to Forgejo at http://localhost:3000
# ✅ API token valid
# ✅ MCP server running on stdio
```

---

## Development Workflow (Phase 1)

### Manual Build Workflow

```bash
# 1. Edit code (AI-assisted via MCP)
# Your editor connects to Forgejo via MCP for context

# 2. Commit changes
git add .
git commit -m "feat: implement auth service"
git push forgejo main

# 3. Build Rust backend
cd services
cargo build --release
cargo test

# 4. Build Docker image
docker build -t localhost:5000/unityplan/auth-service:latest \
  -f services/auth-service/Dockerfile .

# 5. Push to local registry
docker push localhost:5000/unityplan/auth-service:latest

# 6. Deploy to local pod
docker compose -f docker-compose.pod.yml -p pod-dk \
  pull auth
docker compose -f docker-compose.pod.yml -p pod-dk \
  up -d --no-deps auth

# 7. Test
curl http://localhost:8080/health
```

### Benefits of This Workflow

- ✅ Fast iteration (no CI/CD overhead)
- ✅ Full control over builds
- ✅ AI assistance via MCP
- ✅ Version control from day 1
- ✅ Local registry for image storage

---

## When to Add CI/CD (Phase 1.5 / Phase 2)

Add automated pipelines when:

### Triggers for CI/CD

- [ ] **Multiple developers** - Need automated testing coordination
- [ ] **Deploying to multiple pods** - DK, NO, SE, EU need consistent updates
- [ ] **Frequent releases** - Manual builds become tedious
- [ ] **Staging environment** - Need pre-production testing
- [ ] **Complex test suites** - Integration tests, E2E tests take time

### CI/CD Options for Forgejo

**Option 1: Forgejo Actions** (built-in, GitHub Actions compatible)
```yaml
# .forgejo/workflows/build.yml
name: Build and Test
on: [push, pull_request]

jobs:
  build:
    runs-on: docker
    steps:
      - uses: actions/checkout@v3
      - name: Build Rust
        run: cargo build --release
      - name: Run tests
        run: cargo test
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
  
  docker-build:
    image: plugins/docker
    settings:
      registry: localhost:5000
      repo: localhost:5000/unityplan/auth-service
      tags: [latest, ${CI_COMMIT_SHA:0:8}]
```

**Option 3: Drone CI** (similar to Woodpecker, more features)
- See `project_docs/10-application-deployment-cicd.md` for full Drone setup

---

## Docker Registry Usage

### Build and Push Images

```bash
# Build image
docker build -t localhost:5000/unityplan/api-gateway:v0.1.0 \
  -f services/api-gateway/Dockerfile .

# Push to local registry
docker push localhost:5000/unityplan/api-gateway:v0.1.0

# Tag as latest
docker tag localhost:5000/unityplan/api-gateway:v0.1.0 \
  localhost:5000/unityplan/api-gateway:latest
docker push localhost:5000/unityplan/api-gateway:latest
```

### Use Images in Compose

```yaml
# docker-compose.pod.yml
services:
  api-gateway:
    image: localhost:5000/unityplan/api-gateway:${VERSION:-latest}
    # ... rest of config
```

### List Images in Registry

```bash
# List repositories
curl http://localhost:5000/v2/_catalog

# Output:
# {
#   "repositories": [
#     "unityplan/api-gateway",
#     "unityplan/auth-service",
#     "unityplan/community-service"
#   ]
# }

# List tags for a repository
curl http://localhost:5000/v2/unityplan/api-gateway/tags/list

# Output:
# {
#   "name": "unityplan/api-gateway",
#   "tags": ["latest", "v0.1.0", "develop"]
# }
```

### Clean Up Old Images

```bash
# Delete image tag
curl -X DELETE http://localhost:5000/v2/unityplan/api-gateway/manifests/<digest>

# Run garbage collection (prune unused layers)
docker exec dev-registry registry garbage-collect /etc/docker/registry/config.yml
```

---

## MCP Tools Available

Once `forgejo-mcp` is configured, your AI assistant can use:

### Repository Tools
- `forgejo_list_repos` - List all repositories
- `forgejo_get_repo` - Get repository details
- `forgejo_search_code` - Search code across branches
- `forgejo_get_file` - Fetch file contents
- `forgejo_get_tree` - Get directory tree

### Issue/PR Tools
- `forgejo_list_issues` - List issues
- `forgejo_create_issue` - Create new issue
- `forgejo_list_pulls` - List pull requests
- `forgejo_get_pull` - Get PR details

### Commit Tools
- `forgejo_create_file` - Create file via API commit
- `forgejo_update_file` - Update file via API commit
- `forgejo_get_commits` - List commit history

### Example: AI creates a file

```typescript
// AI assistant can execute:
await forgejo_create_file({
  owner: "admin",
  repo: "unityplan_platform",
  path: "services/auth-service/src/jwt.rs",
  content: "// JWT token validation...",
  message: "feat: add JWT validation module",
  branch: "develop"
});
```

---

## Troubleshooting

### Forgejo won't start

```bash
# Check logs
docker logs dev-forgejo

# Common issues:
# - Port 3000 already in use
# - Port 2222 already in use (SSH)
# - Permission issues with /data volume

# Fix port conflicts
# Edit docker-compose.dev.yml, change ports:
#   - "3001:3000"  # Use 3001 instead
#   - "2223:22"    # Use 2223 instead
```

### Can't push to registry

```bash
# Registry must allow insecure connections for localhost
# Add to /etc/docker/daemon.json:
{
  "insecure-registries": ["localhost:5000"]
}

# Restart Docker
sudo systemctl restart docker
```

### MCP connection fails

```bash
# Verify Forgejo API token
curl -H "Authorization: token YOUR_TOKEN" \
  http://localhost:3000/api/v1/user

# Should return user details

# Check MCP server logs
npx @goern/forgejo-mcp 2>&1 | tee mcp.log
```

---

## Summary

### Phase 1 Setup (Minimal)

✅ **Forgejo** - Version control + MCP  
✅ **Docker Registry** - Local image storage  
✅ **Manual builds** - Fast iteration  

### Phase 1.5 / Phase 2 (When Needed)

⏳ **Forgejo Actions / Woodpecker CI** - Automated pipelines  
⏳ **Staging environment** - Pre-production testing  
⏳ **Multi-pod deployments** - DK, NO, SE, EU coordination  

### Key Benefits

- 🚀 **Fast development** - No CI/CD overhead in Phase 1
- 🤖 **AI assistance** - MCP integration for context-aware coding
- 📦 **Local registry** - Quick image builds and testing
- 🔄 **Easy upgrade** - Add CI/CD when needed without disruption

---

**Next Steps:**
1. Start Forgejo: `docker compose -f docker-compose.dev.yml up -d forgejo registry`
2. Create admin account: http://localhost:3000
3. Push code to Forgejo
4. Install `forgejo-mcp` and configure API token
5. Start building Rust backend services!

**Related Documentation:**
- [Application Deployment & CI/CD](../project_docs/10-application-deployment-cicd.md) - Full CI/CD setup for Phase 2
- [Development Tools](./development-tools.md) - All dev environment tools
