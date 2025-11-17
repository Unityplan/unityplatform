# Versioning & Release Strategy

**Last Updated:** November 17, 2025  
**Current Version:** 0.1.0-alpha.1  
**Current Phase:** Phase 1 (MVP)  
**Current Stage:** Stage 5 (of 14)

This document defines how Unity Platform uses **Phases**, **Stages**, **Versions**, and **Forgejo Milestones** to organize development and releases.

---

## 📋 Table of Contents

- [Hierarchy Overview](#hierarchy-overview)
- [Phases (Strategic)](#phases-strategic)
- [Stages (Tactical)](#stages-tactical)
- [Versions (Releases)](#versions-releases)
- [Forgejo Milestones (Tracking)](#forgejo-milestones-tracking)
- [Version Numbering](#version-numbering)
- [Release Process](#release-process)
- [Examples](#examples)

---

## 🏗️ Hierarchy Overview

Unity Platform uses a four-level hierarchy for organizing development:

```
┌────────────────────────────────────────────────────────────┐
│ PHASE 1: MVP (6-9 months)                                  │
│ Major architectural milestone                              │
│ Version Range: 0.1.x                                       │
├────────────────────────────────────────────────────────────┤
│   ├─ Stage 1: Foundation & Infrastructure                  │
│   ├─ Stage 2: Database Schema & Migrations                 │
│   ├─ Stage 3: Authentication Service                       │
│   ├─ Stage 4: User Service                                 │
│   ├─ Stage 5: Frontend Auth & Profile ← CURRENT           │
│   ├─ Stage 6: Territory & Badge Services                   │
│   ├─ Stage 7: Course Service (LMS)                         │
│   ├─ Stage 8: Matrix Protocol Integration                  │
│   ├─ Stage 9: IPFS Service                                 │
│   ├─ Stage 10: Forum Service                               │
│   ├─ Stage 11: Translation Service                         │
│   ├─ Stage 12: Frontend Course & Forum UI                  │
│   ├─ Stage 13: Testing, Documentation & Deployment         │
│   └─ Stage 14: Utility Service & Language Registry         │
│                                                             │
│   Versions within Phase 1:                                 │
│   ├─ 0.1.0-alpha.1 (current) ← RELEASED                   │
│   ├─ 0.1.0-alpha.2 (milestone) ← IN PROGRESS              │
│   ├─ 0.1.0-alpha.3 (planned)                              │
│   ├─ 0.1.0-beta.1 (planned)                               │
│   └─ 0.1.0 (Phase 1 complete)                             │
│                                                             │
│   Forgejo Milestones:                                       │
│   └─ v0.1.0-alpha.2 (137 issues)                          │
└────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────┐
│ PHASE 2: Scale (future)                                    │
│ Version Range: 0.2.x or 1.x                                │
│ Regional deployment, Kubernetes, Enhanced federation       │
└────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────┐
│ PHASE 3: Decentralization (future)                         │
│ Version Range: 2.x                                         │
│ Full Holochain migration, Pure P2P                         │
└────────────────────────────────────────────────────────────┘
```

---

## 🎯 Phases (Strategic)

**Phases** represent major architectural milestones spanning months/years.

### Phase 1: MVP (Current)

**Timeline:** 6-9 months  
**Version Range:** `0.1.x`  
**Goal:** Functional platform with core features for 3-5 territories  
**Architecture:** Rust microservices + PostgreSQL multi-tenancy + React frontend

**Completion Criteria:**
- All 14 stages complete
- All 137 issues in milestone closed
- Platform tested and stable
- Documentation complete
- Ready for limited production use

**End State:** Version `0.1.0` (stable)

### Phase 2: Scale (Future)

**Timeline:** 6-12 months  
**Version Range:** `0.2.x` or `1.x`  
**Goal:** Production-ready platform supporting 20+ territories  
**Architecture:** Kubernetes deployment, enhanced federation, advanced monitoring

**Completion Criteria:**
- Regional multi-pod deployments
- Kubernetes orchestration
- Advanced federation features
- Performance optimization
- Security hardening

**End State:** Version `1.0.0` (production-ready)

### Phase 3: Decentralization (Future)

**Timeline:** 12+ months  
**Version Range:** `2.x`  
**Goal:** Fully decentralized platform  
**Architecture:** Holochain migration, pure P2P, cryptographic ownership

**Completion Criteria:**
- Full Holochain integration
- Peer-to-peer architecture
- Cryptographic data ownership
- Zero-trust security
- Offline-first functionality

**End State:** Version `2.0.0` (fully decentralized)

---

## 📊 Stages (Tactical)

**Stages** are sequential development milestones within a Phase. Each stage represents a cohesive set of features.

### Stage Characteristics

- **Granularity:** 1-4 weeks of development
- **Size:** 3-27 issues per stage
- **Completion:** All stage issues closed in Forgejo
- **Dependencies:** Stages may have dependencies (e.g., Stage 5 depends on Stage 3)

### Phase 1 Stages

| Stage | Name | Issues | Status | Duration |
|-------|------|--------|--------|----------|
| 1 | Foundation & Infrastructure | 6 (#42-#47) | ✅ Complete | 2 weeks |
| 2 | Database Schema & Migrations | 3 (#48-#50) | ✅ Complete | 1 week |
| 3 | Authentication Service | 8 (#51-#58) | ✅ Complete | 2 weeks |
| 4 | User Service | 9 (#59-#67) | ✅ Complete | 2 weeks |
| 5 | Frontend Auth & Profile | 27 (#4-#30) | 🔄 In Progress | 4 weeks |
| 6 | Territory & Badge Services | 11 (#31-#41) | ✅ Complete | 2 weeks |
| 7 | Course Service (LMS) | 6 (#68-#73) | 📋 Planned | 2 weeks |
| 8 | Matrix Protocol | 6 (#74-#79) | 📋 Planned | 2 weeks |
| 9 | IPFS Service | 6 (#82-#87) | 📋 Planned | 2 weeks |
| 10 | Forum Service | 8 (#88-#95) | 📋 Planned | 2 weeks |
| 11 | Translation Service | 3 (#96-#98) | 📋 Planned | 1 week |
| 12 | Frontend Course & Forum UI | 10 (#99-#108) | 📋 Planned | 3 weeks |
| 13 | Testing & Deployment | 11 (#109-#119) | 📋 Planned | 3 weeks |
| 14 | Utility Service | 18 (#120-#136) | ✅ Complete | 2 weeks |

**Total:** 137 issues across 14 stages

### Stages vs Versions

Stages do NOT directly map to version numbers. Instead:

- **Multiple stages** can be completed within one version
- **One stage** can span multiple version releases
- **Version bumps** happen based on stability and feature completeness, not stage completion

**Example:**
```
Version 0.1.0-alpha.1:
  ✅ Stage 1: Foundation
  ✅ Stage 2: Database
  ✅ Stage 3: Auth Service
  ✅ Stage 4: User Service
  ✅ Stage 6: Territory/Badge
  ✅ Stage 14: Utility Service

Version 0.1.0-alpha.2 (in progress):
  🔄 Stage 5: Frontend (50% complete)
  📋 Stage 7: Course Service
  📋 Stage 8: Matrix Protocol
  ... etc

Version 0.1.0-alpha.3 (planned):
  ✅ Stage 5: Frontend (complete)
  ✅ Stage 7: Course Service (complete)
  🔄 Stage 8: Matrix Protocol (in progress)
```

---

## 🏷️ Versions (Releases)

**Versions** represent actual software releases following **Semantic Versioning 2.0.0** (SemVer).

### Version Format

```
MAJOR.MINOR.PATCH-PRERELEASE+BUILD

Example: 0.1.0-alpha.1+build.20251117
         │ │ │  │     │  └─ Build metadata (optional)
         │ │ │  │     └──── Pre-release number
         │ │ │  └────────── Pre-release type (alpha, beta, rc)
         │ │ └───────────── Patch version (bug fixes)
         │ └─────────────── Minor version (new features)
         └───────────────── Major version (breaking changes)
```

### Version Components

**MAJOR (0.x.x):**
- Increment when Phase changes (Phase 1 → Phase 2 → Phase 3)
- Breaking changes to architecture
- Examples: `0.x.x` (Phase 1), `1.x.x` (Phase 2), `2.x.x` (Phase 3)

**MINOR (x.1.x):**
- New features or services added
- Backward-compatible changes
- Examples: `0.1.x` (MVP features), `0.2.x` (Scale features)

**PATCH (x.x.0):**
- Bug fixes and small improvements
- No new features
- Examples: `0.1.1` (auth bug fix), `0.1.2` (database patch)

**PRE-RELEASE (alpha/beta/rc):**
- `alpha.N` - Early development, unstable, internal testing
- `beta.N` - Feature-complete, external testing, may have bugs
- `rc.N` - Release Candidate, stable, final testing
- `(none)` - Stable release

### Phase 1 Version Progression

```
0.1.0-alpha.1  ← CURRENT (Nov 17, 2025)
  ├─ Stages 1-4, 6, 14 complete
  ├─ Infrastructure operational
  └─ Basic services working

0.1.0-alpha.2  ← MILESTONE (in progress)
  ├─ Stage 5 complete (Frontend)
  ├─ Stages 7-11 complete (Course, Matrix, IPFS, Forum, Translation)
  └─ Stage 12 complete (Frontend UI)

0.1.0-alpha.3  ← PLANNED
  ├─ Stage 13 complete (Testing & Deployment)
  ├─ All 137 issues closed
  └─ Platform functional end-to-end

0.1.0-beta.1   ← PLANNED
  ├─ External testing phase
  ├─ Bug fixes from testing
  └─ Performance optimization

0.1.0-rc.1     ← PLANNED
  ├─ Release candidate
  ├─ Final testing
  └─ Documentation complete

0.1.0          ← PHASE 1 COMPLETE
  ├─ Stable release
  ├─ Production-ready for MVP
  └─ Ready for limited deployment
```

### Version Lifecycle

```
Development → Alpha → Beta → Release Candidate → Stable
    ↓           ↓       ↓            ↓              ↓
 Internal   Internal External    Final         Production
  testing    testing  testing    testing          use
```

---

## 🎯 Forgejo Milestones (Tracking)

**Forgejo Milestones** track work toward the NEXT version release.

### Milestone Characteristics

- **Name:** Matches target version (e.g., `v0.1.0-alpha.2`)
- **Purpose:** Group all issues for a specific release
- **Duration:** Typically 4-8 weeks
- **Issues:** All work required for that version
- **Status:** Open (in progress) or Closed (completed)

### Current Milestone

**v0.1.0-alpha.2** (Milestone ID: 1)

**Goal:** Complete Phase 1 MVP  
**Duration:** 6-9 months total (started Nov 2025)  
**Issues:** 137 total

**Progress:**
- ✅ Closed: 55 issues (40%)
- 🔄 Open: 82 issues (60%)

**Link:** [View Milestone](http://localhost:3000/henrik/unity_platform/milestones)

### Milestone Workflow

1. **Create Milestone:**
   ```
   Name: v0.1.0-alpha.2
   Due Date: 2026-01-15 (example)
   Description: Complete Phase 1 Stage 5-13
   ```

2. **Assign Issues:**
   - All issues for next release assigned to milestone
   - Issues can be reassigned between milestones

3. **Track Progress:**
   - Monitor completion percentage
   - Review burndown charts (if available)
   - Adjust scope if needed

4. **Complete Milestone:**
   - All issues closed
   - Version released
   - Milestone closed
   - Git tag created

5. **Create Next Milestone:**
   - New milestone for next version
   - Plan next set of issues
   - Repeat cycle

### Milestone Naming Convention

```
v{MAJOR}.{MINOR}.{PATCH}-{PRERELEASE}

Examples:
  v0.1.0-alpha.2   ✅ Correct
  v0.1.0-alpha.3   ✅ Correct
  v0.1.0-beta.1    ✅ Correct
  v0.1.0           ✅ Correct (stable release)
  
  0.1.0-alpha.2    ❌ Missing 'v' prefix
  Phase-1-alpha-2  ❌ Wrong format
  MVP-Release      ❌ Not SemVer
```

---

## 🔢 Version Numbering

### SemVer 2.0.0 Compliance

Unity Platform follows [Semantic Versioning 2.0.0](https://semver.org/):

```
Given a version number MAJOR.MINOR.PATCH, increment:

1. MAJOR version when you make incompatible API changes
2. MINOR version when you add functionality in a backward compatible manner
3. PATCH version when you make backward compatible bug fixes

Additional labels for pre-release and build metadata are available 
as extensions to the MAJOR.MINOR.PATCH format.
```

### Platform vs Service Versions

**Platform Version (VERSIONS.md):**
- Represents overall platform state
- Updated when major milestone reached
- Example: `0.1.0-alpha.1`

**Service Versions (individual Cargo.toml):**
- Each microservice has own version
- Can increment independently
- Usually aligned with platform version
- Example: `auth-service v0.1.0-alpha.1`

**Alignment Strategy:**

During **Phase 1 (MVP)**:
- All services use same version as platform
- Simpler to manage during rapid development
- Example: All services at `0.1.0-alpha.1`

Starting **Phase 2 (Scale)**:
- Services can version independently
- Semantic versioning per service
- Platform version represents compatibility matrix
- Example: `platform v1.0.0` with `auth-service v1.2.3`

### Version Increment Triggers

**When to bump MAJOR (0.x → 1.x):**
- Phase completion (Phase 1 → Phase 2)
- Breaking API changes
- Architecture redesign
- Migration requiring user action

**When to bump MINOR (x.0.x → x.1.x):**
- New service added
- New major feature
- Stage completion (sometimes)
- Backward-compatible API additions

**When to bump PATCH (x.x.0 → x.x.1):**
- Bug fixes
- Security patches
- Performance improvements
- Documentation updates

**When to bump PRE-RELEASE (alpha.1 → alpha.2):**
- Regular development progress
- Stage completions
- Sprint completions
- Milestone progress

### Pre-Release Progression

```
alpha → beta → rc → stable

alpha (α):
  - Early development
  - Unstable
  - Internal testing only
  - Frequent breaking changes
  - Example: 0.1.0-alpha.1

beta (β):
  - Feature-complete
  - Mostly stable
  - External testing
  - Minor bugs expected
  - Example: 0.1.0-beta.1

rc (Release Candidate):
  - Stable
  - No new features
  - Final testing
  - Ready for release if no issues found
  - Example: 0.1.0-rc.1

stable:
  - Production-ready
  - No pre-release suffix
  - Long-term support
  - Example: 0.1.0
```

---

## 🚀 Release Process

### Step-by-Step Release Workflow

#### 1. Pre-Release Preparation

```bash
# Verify all milestone issues closed
# Check: http://localhost:3000/henrik/unity_platform/milestones

# Run full test suite
cargo test --workspace
cd app && npm test

# Update version in all files
# - VERSIONS.md
# - All Cargo.toml files
# - app/package.json
# - README.md
```

#### 2. Update Documentation

```bash
# Update CHANGELOG.md
# Add new version section with all changes

# Update VERSIONS.md
# Update platform version
# Update service versions
# Update last updated date

# Update README.md
# Update project status
# Update version badges
```

#### 3. Create Git Tag

```bash
# Create annotated tag
git tag -a v0.1.0-alpha.2 -m "Release v0.1.0-alpha.2

Phase 1 MVP - Alpha 2 Release

Features:
- Frontend authentication complete
- User profiles and settings
- Course service implementation
- Matrix protocol integration

Changes:
- See CHANGELOG.md for full details

Issues Closed: 82/137 (60%)
"

# Push tag to remote
git push origin v0.1.0-alpha.2
```

#### 4. Close Forgejo Milestone

```bash
# In Forgejo web UI:
# 1. Go to milestone
# 2. Verify all issues closed
# 3. Click "Close Milestone"
# 4. Add completion notes
```

#### 5. Create GitHub/Forgejo Release

```bash
# In Forgejo web UI:
# 1. Go to Releases
# 2. Click "New Release"
# 3. Select tag: v0.1.0-alpha.2
# 4. Title: Unity Platform v0.1.0-alpha.2
# 5. Description: Copy from CHANGELOG.md
# 6. Attach binaries (if applicable)
# 7. Mark as "Pre-release" (for alpha/beta/rc)
# 8. Publish
```

#### 6. Deploy (if applicable)

```bash
# Deploy to test environment
./scripts/deploy/deploy-test.sh v0.1.0-alpha.2

# Verify deployment
./scripts/test/smoke-tests.sh

# Monitor logs
docker compose logs -f
```

#### 7. Create Next Milestone

```bash
# In Forgejo web UI:
# 1. Go to Milestones
# 2. Click "New Milestone"
# 3. Name: v0.1.0-alpha.3
# 4. Due Date: +6 weeks
# 5. Description: Next sprint goals
# 6. Create
```

#### 8. Announce Release

```bash
# Internal announcement (team)
# - Slack/Discord message
# - Email to team

# External announcement (if public)
# - Blog post
# - Social media
# - Forum post
```

### Release Checklist

Use this checklist for every release:

```markdown
## Pre-Release
- [ ] All milestone issues closed
- [ ] Tests passing (backend + frontend)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] VERSIONS.md updated
- [ ] README.md updated
- [ ] No known critical bugs

## Version Bump
- [ ] Cargo.toml files updated (all services)
- [ ] package.json updated (frontend)
- [ ] VERSIONS.md version updated
- [ ] Version constants in code updated

## Git Operations
- [ ] All changes committed
- [ ] Git tag created (annotated)
- [ ] Tag pushed to remote
- [ ] Branch merged (if applicable)

## Forgejo
- [ ] Milestone closed
- [ ] Release created
- [ ] Release notes complete
- [ ] Binaries attached (if applicable)

## Deployment
- [ ] Deployed to test environment
- [ ] Smoke tests passing
- [ ] Monitoring verified
- [ ] Rollback plan ready

## Post-Release
- [ ] Next milestone created
- [ ] Issues planned for next release
- [ ] Team announced
- [ ] Documentation published

## Optional (for stable releases)
- [ ] Docker images tagged and pushed
- [ ] Deployment guides updated
- [ ] API documentation published
- [ ] Migration guides written (if needed)
```

---

## 📖 Examples

### Example 1: Current State (Nov 17, 2025)

```
Phase: Phase 1 (MVP)
Stage: Stage 5 (Frontend Auth & Profile)
Version: 0.1.0-alpha.1 (released)
Milestone: v0.1.0-alpha.2 (in progress, 40% complete)

Status:
  ✅ Stages 1-4, 6, 14 complete
  🔄 Stage 5 in progress (15% complete)
  📋 Stages 7-13 planned

VERSIONS.md shows: 0.1.0-alpha.1
Forgejo milestone: v0.1.0-alpha.2
Git latest tag: v0.1.0-alpha.1
Next release: 0.1.0-alpha.2 (when milestone 40% → 100%)
```

### Example 2: Completing Stage 5

```
Action: Close all Stage 5 issues (#4-#30)
Result:
  - Milestone progress: 40% → 60%
  - Stage 5 status: In Progress → Complete
  - Version: Still 0.1.0-alpha.1 (no release yet)
  
Decision: Continue to Stage 7 or release?
  
Option A (Continue):
  - Keep working toward v0.1.0-alpha.2
  - Complete more stages
  - Release when 80-100% complete
  
Option B (Release):
  - Release v0.1.0-alpha.2 now
  - Update VERSIONS.md → 0.1.0-alpha.2
  - Create milestone v0.1.0-alpha.3
  - Continue with remaining stages
```

### Example 3: Phase 1 Completion

```
Action: All 14 stages complete, all 137 issues closed
Result:
  - Phase 1: Complete
  - Milestone v0.1.0-alpha.2: Closed
  - Ready for beta testing
  
Version Progression:
  0.1.0-alpha.2 → 0.1.0-beta.1
  
Next Steps:
  1. Create milestone v0.1.0-beta.1
  2. Plan beta testing issues
  3. External testing phase
  4. Bug fixes and optimization
  5. Release v0.1.0-beta.1
  
Eventually:
  0.1.0-beta.1 → 0.1.0-rc.1 → 0.1.0 (stable)
  
Phase 1 Complete:
  - Version 0.1.0 released
  - Begin Phase 2 planning
  - Create milestone v0.2.0-alpha.1 or v1.0.0-alpha.1
```

### Example 4: Phase Transition

```
Phase 1 → Phase 2 transition:

Completion:
  ✅ Phase 1 complete (version 0.1.0)
  ✅ MVP functional and stable
  ✅ 3-5 territories supported
  
Decision: Major version bump?
  
Option A (Conservative):
  - Next version: 0.2.0-alpha.1
  - Minor version bump
  - Gradual transition to Phase 2
  
Option B (Aggressive):
  - Next version: 1.0.0-alpha.1
  - Major version bump
  - Signifies production-ready milestone
  
Typical Choice: Option A
  - Use 0.x.x for Phase 1 & 2
  - Reserve 1.0.0 for production release
  - Use 2.0.0 for Phase 3 (Holochain)
```

---

## 📊 Summary Table

| Concept | Scope | Duration | Example | Tracked In |
|---------|-------|----------|---------|------------|
| **Phase** | Strategic | 6-12 months | Phase 1 (MVP) | Roadmaps |
| **Stage** | Tactical | 1-4 weeks | Stage 5 (Frontend) | phase-1-status.md |
| **Version** | Release | Variable | 0.1.0-alpha.2 | VERSIONS.md, Git tags |
| **Milestone** | Tracking | 4-8 weeks | v0.1.0-alpha.2 | Forgejo |

### Relationship Diagram

```
PHASE 1 (0.1.x)
  │
  ├─── Version 0.1.0-alpha.1 ✅ RELEASED
  │      ├─ Stage 1: Foundation ✅
  │      ├─ Stage 2: Database ✅
  │      ├─ Stage 3: Auth Service ✅
  │      ├─ Stage 4: User Service ✅
  │      ├─ Stage 6: Territory/Badge ✅
  │      └─ Stage 14: Utility Service ✅
  │
  ├─── Milestone v0.1.0-alpha.2 🔄 IN PROGRESS (40%)
  │      ├─ Stage 5: Frontend 🔄 (15%)
  │      ├─ Stage 7: Course Service 📋
  │      ├─ Stage 8: Matrix Protocol 📋
  │      ├─ Stage 9: IPFS Service 📋
  │      ├─ Stage 10: Forum Service 📋
  │      ├─ Stage 11: Translation 📋
  │      ├─ Stage 12: Frontend UI 📋
  │      └─ Stage 13: Testing 📋
  │
  ├─── Version 0.1.0-alpha.2 📋 PLANNED
  │      └─ Released when milestone complete
  │
  ├─── Version 0.1.0-alpha.3 📋 PLANNED
  ├─── Version 0.1.0-beta.1 📋 PLANNED
  ├─── Version 0.1.0-rc.1 📋 PLANNED
  └─── Version 0.1.0 ⭐ STABLE (Phase 1 Complete)

PHASE 2 (0.2.x or 1.x)
  └─── Future...

PHASE 3 (2.x)
  └─── Future...
```

---

## 🔗 Related Documentation

- **VERSIONS.md** - Current version matrix
- **CHANGELOG.md** - Change history
- **phase-1-status.md** - Stage tracking
- **forgejo-workflow.md** - Milestone management
- **Forgejo Milestones** - [View in Forgejo](http://localhost:3000/henrik/unity_platform/milestones)

---

## 📝 Quick Reference

**What am I looking for?**

- "What's deployed/working now?" → **VERSIONS.md**
- "What's being worked on?" → **Forgejo Milestone**
- "What's the roadmap?" → **Phase 1 Status**
- "When's the next release?" → **Milestone due date**
- "What changed recently?" → **CHANGELOG.md**
- "What's the overall plan?" → **Phase Roadmaps**

**When do I update?**

- **VERSIONS.md** → When releasing a version
- **Forgejo Milestone** → When planning/completing work
- **phase-1-status.md** → When completing stages
- **CHANGELOG.md** → Before every release
- **Git tags** → When releasing a version

**How are they related?**

```
Phases contain Stages
Stages contain Issues
Issues grouped in Milestones
Milestones target Versions
Versions document Releases
Releases complete Phases
```

---

**Questions or suggestions?** Open an issue in [Forgejo](http://localhost:3000/henrik/unity_platform/issues/new)
