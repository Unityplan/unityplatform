# Versioning Strategy

**Last Updated:** November 5, 2025  
**Status:** Active

---

## 🎯 Overview

UnityPlan follows industry best practices for version management across a distributed microservices architecture. This document defines the versioning strategy for all components of the platform.

---

## 📦 Semantic Versioning (SemVer)

All services, libraries, and the platform use **Semantic Versioning 2.0.0**:

```
MAJOR.MINOR.PATCH
```

### Version Components

- **MAJOR**: Incompatible API changes (breaking changes)
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Examples

```
0.1.0 → 0.1.1  # Bug fix
0.1.1 → 0.2.0  # New feature
0.2.0 → 1.0.0  # Breaking change or production release
```

### Pre-release Versions

For development and testing:

```
0.1.0-alpha.1   # Alpha release (early development, incomplete)
0.1.0-alpha.2   # Alpha iteration
0.1.0-beta.1    # Beta release (feature complete, testing)
0.1.0-beta.2    # Beta iteration
0.1.0-rc.1      # Release candidate (production ready, final testing)
0.1.0           # Stable release
```

**Release Stage Definitions:**

- **Alpha** (`-alpha.N`): 
  - Early development
  - Incomplete features
  - Internal testing only
  - Breaking changes expected
  - Example: Infrastructure complete but no working services

- **Beta** (`-beta.N`):
  - Feature complete for this version
  - External testing and feedback
  - Bug fixes and refinements
  - API should be stable
  - Example: All services working, testing with users

- **Release Candidate** (`-rc.N`):
  - Production ready
  - Final testing before release
  - Only critical bug fixes
  - No new features
  - Example: Ready to deploy, final validation

- **Stable** (no suffix):
  - Production release
  - Fully tested and validated
  - No known critical bugs
  - Supported for use

**Current Platform Stage:** `0.1.0-alpha.1` (Infrastructure only, no services)

**Progression Plan:**
```
0.1.0-alpha.1  → Infrastructure + database (current)
0.1.0-alpha.2  → Auth service working
0.1.0-alpha.3  → User service working
0.1.0-beta.1   → All core services integrated
0.1.0-rc.1     → Ready for MVP launch
0.1.0          → MVP Phase 1 complete
```

---

## 🏗️ Component Versioning

### 1. Platform Version

The overall platform has a single version tracking major milestones:

- **Current:** `0.1.0` (MVP Phase 1)
- **Location:** `VERSIONS.md`, root `README.md`
- **Scope:** Major feature releases, phase completions

**Platform Version History:**
```
0.1.0 - MVP Phase 1 (In Progress)
1.0.0 - Production Ready (Planned)
2.0.0 - Holochain Integration (Planned)
```

### 2. Service Versions

Each microservice has an independent version:

```toml
# services/auth-service/Cargo.toml
[package]
name = "auth-service"
version = "0.1.0"
```

**Services are versioned independently** because:
- Each service has its own release cycle
- Bug fixes in one service don't affect others
- Clear dependency tracking between services

### 3. Shared Library Version

The `shared-lib` crate has its own version:

```toml
# services/shared-lib/Cargo.toml
[package]
name = "shared-lib"
version = "0.1.0"
```

**Breaking changes in shared-lib** require:
1. Bump major version of shared-lib
2. Update all dependent services
3. Document migration path

### 4. Database Schema Versions

Database migrations use **timestamp-based versioning**:

```
YYYYMMDDHHMMSS_description.up.sql
YYYYMMDDHHMMSS_description.down.sql

Example:
20251105000001_initial_schema.up.sql
20251105000002_add_user_preferences.up.sql
```

**Current Schema Version:** `20251105000001`  
**Location:** `services/shared-lib/migrations/`

### 5. API Versions

REST APIs are versioned in the URL path:

```
/api/v1/auth/login
/api/v1/users/me
/api/v2/auth/login  # Future breaking change
```

**API Versioning Rules:**
- v1, v2, v3... (no decimals)
- Breaking changes require new version
- Old versions supported for at least 6 months
- Deprecation warnings before removal

---

## 🔖 Git Tagging Strategy

### Service Tags

Each service is tagged independently:

```bash
git tag shared-lib-v0.1.0
git tag auth-service-v0.1.0
git tag user-service-v0.2.0
```

### Platform Tags

Major platform milestones get platform tags:

```bash
git tag platform-v0.1.0 -m "MVP Phase 1 Complete"
git tag platform-v1.0.0 -m "Production Release"
```

### Tag Format

```
<component>-v<version>

Examples:
auth-service-v0.1.0
shared-lib-v0.2.0
platform-v1.0.0
```

### Creating Tags

```bash
# Annotated tag with message (preferred)
git tag -a auth-service-v0.1.0 -m "Initial auth service release"

# Push tags to remote
git push origin auth-service-v0.1.0

# Or push all tags
git push --tags
```

---

## 📝 Changelog Management

### Service Changelogs

Each service maintains its own `CHANGELOG.md`:

```markdown
# auth-service CHANGELOG

## [Unreleased]
### Added
- JWT refresh token rotation

## [0.2.0] - 2025-11-10
### Added
- Password reset endpoint
- Email verification

### Changed
- Improved token validation performance

### Fixed
- Session cleanup race condition

## [0.1.0] - 2025-11-05
- Initial release
```

### Changelog Categories

Use these standard categories:
- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Features to be removed
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security fixes

### Platform Changelog

Platform-level changes tracked in root `CHANGELOG.md`:

```markdown
# UnityPlan Platform CHANGELOG

## [0.1.0] - 2025-11-05
### Added
- Multi-pod architecture
- Denmark pod deployment
- Database schema with multi-territory support
- Monitoring stack (Prometheus, Grafana, Jaeger)
```

---

## 🐳 Docker Image Versioning

### Image Naming

```
<registry>/<service>:<tag>

Examples:
unityplan/auth-service:0.1.0
unityplan/auth-service:0.1.0-abc1234  # with git hash
unityplan/auth-service:latest
```

### Image Tags

Each service image gets multiple tags:

```bash
# Version tag
docker tag auth-service unityplan/auth-service:0.1.0

# Version + git hash tag
docker tag auth-service unityplan/auth-service:0.1.0-abc1234

# Latest tag (production)
docker tag auth-service unityplan/auth-service:latest

# Environment-specific tags
docker tag auth-service unityplan/auth-service:dev
docker tag auth-service unityplan/auth-service:staging
```

### Image Versioning Strategy

- **Development:** `dev`, `latest` (auto-updated)
- **Staging:** `staging`, specific versions
- **Production:** Specific versions only (e.g., `0.1.0`)

---

## 📊 Version Tracking

### VERSIONS.md

Central version registry at project root:

```markdown
# UnityPlan Version Matrix

| Service | Version | Status |
|---------|---------|--------|
| shared-lib | 0.1.0 | Active |
| auth-service | 0.1.0 | Development |
| user-service | - | Not Started |
```

**Updated:** Every deployment  
**Location:** `/VERSIONS.md`

### Deployment Versions

Track deployed versions per pod:

```yaml
# pods/denmark/versions.yaml
pod: denmark
deployed: 2025-11-05T20:00:00Z
platform_version: 0.1.0

services:
  auth-service:
    version: 0.1.0
    image: unityplan/auth-service:0.1.0
    deployed: 2025-11-05T20:00:00Z
```

---

## 🔄 Version Update Workflow

### 1. Development Workflow

```bash
# Feature development on feature branch
git checkout -b feature/user-preferences

# Make changes...

# Before PR: update version if needed
# services/user-service/Cargo.toml
# version = "0.2.0"

# Update CHANGELOG.md
# ## [0.2.0] - 2025-11-10
# ### Added
# - User preference endpoints

# Create PR, review, merge to main
```

### 2. Release Workflow

```bash
# After merge to main, create release
git checkout main
git pull

# Tag the release
git tag user-service-v0.2.0 -m "Add user preferences feature"

# Push tag
git push origin user-service-v0.2.0

# Build and tag Docker image
docker build -t unityplan/user-service:0.2.0 -f services/user-service/Dockerfile .
docker push unityplan/user-service:0.2.0

# Update VERSIONS.md
# | user-service | 0.2.0 | Active |
```

### 3. Deployment Workflow

```bash
# Update pod deployment config
# pods/denmark/docker-compose.yml
# image: unityplan/user-service:0.2.0

# Deploy
./scripts/deploy-pod.sh denmark

# Update pod version tracking
# pods/denmark/versions.yaml
# user-service:
#   version: 0.2.0
#   deployed: 2025-11-10T15:30:00Z
```

---

## 🛡️ Breaking Changes

### Identifying Breaking Changes

Changes that require a MAJOR version bump:

**API Changes:**
- Removing endpoints
- Changing request/response formats
- Changing authentication requirements
- Removing query parameters

**Database Changes:**
- Dropping tables or columns
- Changing column types (incompatible)
- Removing indexes that queries depend on

**Service Changes:**
- Changing environment variable names
- Changing configuration format
- Removing features

### Managing Breaking Changes

1. **Plan Migration Path**
   - Document all breaking changes
   - Provide migration guide
   - Create database migration scripts

2. **Deprecation Period**
   - Mark old API as deprecated (add warnings)
   - Support both old and new versions
   - Set removal date (minimum 6 months)

3. **Communication**
   - Update CHANGELOG with BREAKING CHANGE
   - Notify all stakeholders
   - Update documentation

4. **Version Bump**
   ```bash
   # From 0.9.5 to 1.0.0
   # Update Cargo.toml
   version = "1.0.0"
   
   # Tag release
   git tag auth-service-v1.0.0
   ```

---

## 📚 Version Information in Code

### Exposing Version Info

Each service exposes version information at runtime:

```rust
// services/shared-lib/src/lib.rs
pub mod version {
    pub const VERSION: &str = env!("SERVICE_VERSION");
    pub const GIT_HASH: &str = env!("GIT_HASH");
    
    pub fn full_version() -> String {
        format!("{} ({})", VERSION, GIT_HASH)
    }
}
```

### Health Endpoint

All services include version in health check:

```rust
// GET /health
{
  "status": "healthy",
  "service": "auth-service",
  "version": "0.1.0",
  "git_hash": "abc1234",
  "build_timestamp": "1699200000"
}
```

### Logging

Version logged at startup:

```rust
tracing::info!(
    service = shared_lib::version::NAME,
    version = shared_lib::version::VERSION,
    git_hash = shared_lib::version::GIT_HASH,
    "Service starting"
);
```

---

## 🎓 Best Practices

### DO ✅

- **Version everything**: Services, APIs, database schemas
- **Use SemVer**: Consistent versioning across all components
- **Tag releases**: Every production deployment gets a Git tag
- **Update CHANGELOGs**: Document all changes
- **Track deployments**: Know what version is running where
- **Automate**: Use build scripts to inject version info

### DON'T ❌

- **Skip versions**: Don't jump from 0.1.0 to 0.3.0
- **Reuse tags**: Never change what a tag points to
- **Delete tags**: Tags are permanent version markers
- **Break without versioning**: Breaking changes = major version bump
- **Forget migration**: Always provide upgrade path

---

## 🔗 Related Documentation

- [VERSIONS.md](/VERSIONS.md) - Current version matrix
- [Development Guide](development-tools.md) - Development workflow
- [Deployment Guide](../deployment/multi-pod-deployment.md) - Deployment process
- [Database Migrations](../../architecture/infrastructure.md#database) - Schema versioning

---

## 📖 References

- [Semantic Versioning 2.0.0](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Git Tagging](https://git-scm.com/book/en/v2/Git-Basics-Tagging)

---

**Maintained by:** UnityPlan Development Team  
**Last Review:** November 5, 2025
