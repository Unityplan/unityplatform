# Forgejo Project Management Strategy for Unity Platform

## Overview

This guide provides a comprehensive strategy for managing the Unity Platform project using Forgejo (9.0.3 + Gitea 1.22.0), replacing the current markdown-based status tracking with a structured issue-based workflow.

## Current Architecture Complexity

### Multi-Component System
- **Multi-pod architecture**: denmark, norway, sweden, europe (federation layer)
- **Backend services**: auth-service, user-service, badge-service, territory-service, utility-service, shared-lib
- **Frontend SPA**: Vite + React application
- **Infrastructure**: Docker, PostgreSQL, NATS, Redis, Traefik
- **Monitoring**: Prometheus, Grafana (central + per-pod)
- **Development**: Dev pods, portal management interfaces

### Current Pain Points
- ❌ Status scattered across multiple markdown files
- ❌ Hard to track cross-service dependencies
- ❌ No visual progress tracking
- ❌ Difficult to assign ownership
- ❌ No automated workflow triggers

---

## Recommended Structure

### 1. Repository Organization

#### **Option A: Mono-repo with Labels (RECOMMENDED)**

**Current Setup:** All components in one repository ✅

**Pros:**
- Single source of truth
- Easy cross-component refactoring
- Shared infrastructure code
- Atomic commits across services
- Simplified CI/CD

**Structure:**
```
unityplatform/
├── services/          (Backend microservices)
├── app/              (Frontend SPA)
├── docker/           (Infrastructure configs)
├── pods/             (Territory pod configs)
├── monitoring/       (Monitoring setup)
└── docs/            (Documentation)
```

**Use Forgejo Labels to Separate Concerns:**
- `area: auth-service`
- `area: user-service`
- `area: frontend`
- `area: infrastructure`
- `area: monitoring`
- `pod: denmark`
- `pod: norway`
- etc.

#### **Option B: Multi-repo (Alternative)**

Only consider if teams are completely independent:
- `unityplatform-services` (all backend services)
- `unityplatform-app` (frontend)
- `unityplatform-infrastructure` (Docker, configs)
- `unityplatform-pods` (pod configurations)

**Recommendation:** Stick with mono-repo for now, split later if team grows beyond 20 developers.

---

### 2. Project Boards (Kanban)

Forgejo/Gitea Projects = Kanban boards for visual tracking

#### **Recommended Project Structure**

Create **4 Main Projects**:

##### **Project 1: Platform Core Development**
**Purpose:** Track core feature development across all services

**Columns:**
1. 📋 **Backlog** - All planned features
2. 🎯 **Ready** - Refined, ready to start
3. 🚧 **In Progress** - Active development
4. 👀 **Review** - Code review / PR open
5. 🧪 **Testing** - QA / Integration testing
6. ✅ **Done** - Completed & deployed

**Scope:**
- Feature development
- Service implementations
- API endpoints
- Frontend components

##### **Project 2: Infrastructure & DevOps**
**Purpose:** Track infrastructure, deployment, and operational tasks

**Columns:**
1. 📋 **Planned**
2. 🚧 **In Progress**
3. 🔍 **Testing**
4. ✅ **Live**

**Scope:**
- Docker configurations
- Pod deployments
- CI/CD pipelines
- Monitoring setup
- Database migrations

##### **Project 3: Security & Compliance**
**Purpose:** Track security features, audits, and compliance

**Columns:**
1. 🔍 **Identified**
2. 🎯 **Prioritized**
3. 🔧 **Implementing**
4. ✅ **Resolved**

**Scope:**
- Security features (auth, session lock, etc.)
- Vulnerability fixes
- Compliance requirements
- Audit tasks

##### **Project 4: Documentation**
**Purpose:** Track documentation needs

**Columns:**
1. 📝 **Needed**
2. ✍️ **Writing**
3. 👀 **Review**
4. ✅ **Published**

**Scope:**
- API documentation
- User guides
- Developer guides
- Architecture docs

---

### 3. Issue Management

#### **Issue Types (via Labels)**

**Priority Labels:**
- 🔴 `priority: critical` - Blocking production
- 🟠 `priority: high` - Important for release
- 🟡 `priority: medium` - Planned work
- 🟢 `priority: low` - Nice to have

**Type Labels:**
- `type: feature` - New functionality
- `type: bug` - Something broken
- `type: enhancement` - Improvement to existing feature
- `type: refactor` - Code quality improvement
- `type: security` - Security-related
- `type: documentation` - Documentation work
- `type: infrastructure` - DevOps/infra work

**Area Labels (Component):**
- `area: auth-service`
- `area: user-service`
- `area: badge-service`
- `area: territory-service`
- `area: utility-service`
- `area: shared-lib`
- `area: frontend`
- `area: api-gateway` (future)
- `area: infrastructure`
- `area: monitoring`

**Pod Labels:**
- `pod: denmark`
- `pod: norway`
- `pod: sweden`
- `pod: europe-federation`
- `pod: all` (affects all pods)

**Status Labels:**
- `status: blocked` - Cannot proceed
- `status: needs-review` - Awaiting feedback
- `status: ready` - Ready to start
- `status: in-progress` - Active work

**Special Labels:**
- `good-first-issue` - For new contributors
- `help-wanted` - Need assistance
- `breaking-change` - API/schema breaking change
- `dependencies` - Related to dependencies

#### **Issue Templates**

Create templates in `.forgejo/ISSUE_TEMPLATE/`:

**1. Feature Request**
```markdown
## Feature Description
Brief description of the feature

## User Story
As a [user type], I want [goal] so that [benefit]

## Affected Components
- [ ] Frontend
- [ ] Backend (specify service)
- [ ] Infrastructure
- [ ] Database

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Technical Notes
Any technical considerations

## Related Issues
#
```

**2. Bug Report**
```markdown
## Bug Description
What went wrong?

## Steps to Reproduce
1. 
2. 
3. 

## Expected Behavior
What should happen?

## Actual Behavior
What actually happened?

## Environment
- Pod: 
- Service: 
- Version: 

## Screenshots/Logs
```

**3. Infrastructure Task**
```markdown
## Task Description
What needs to be done?

## Affected Pods
- [ ] Denmark
- [ ] Norway
- [ ] Sweden
- [ ] Europe Federation
- [ ] Monitoring

## Steps
- [ ] Step 1
- [ ] Step 2

## Rollback Plan
How to revert if needed?

## Related Documentation
```

---

### 4. Milestone Planning

#### **Milestone Structure**

Use Milestones for **Phases** and **Releases**:

##### **Phase-Based Milestones (Strategic)**
- **Phase 1: MVP Foundation** (Current)
  - Due: 2025-12-31
  - Goal: Core authentication, user profiles, basic infrastructure
  
- **Phase 2: Community Features**
  - Due: 2026-Q1
  - Goal: Communities, guilds, forums
  
- **Phase 3: Learning Platform**
  - Due: 2026-Q2
  - Goal: Courses, badges, LMS integration

##### **Release Milestones (Tactical)**
- **v0.1.0-alpha.1** (Current)
  - Backend services operational
  - Frontend auth complete
  
- **v0.1.0-alpha.2**
  - Profile management
  - Settings pages
  
- **v0.2.0-beta.1**
  - First community features

#### **How to Use Milestones**

1. **Create milestone** with clear goal and due date
2. **Assign issues** to milestone
3. **Track progress** - Forgejo shows completion %
4. **Review regularly** - Weekly milestone review meetings
5. **Adjust** - Move issues if needed

---

### 5. Roadmap Strategy

#### **Since Gitea 1.22.0 may not have built-in roadmaps:**

**Option A: Milestone-Based Roadmap**
- Use milestones as roadmap items
- Create `roadmap` label for roadmap-specific issues
- Pin important roadmap issues to repository

**Option B: Project Board Roadmap**
- Create dedicated "Roadmap" project
- Columns: Q1 2026, Q2 2026, Q3 2026, Q4 2026
- Add high-level feature cards

**Option C: Wiki Roadmap (Recommended)**
- Use Forgejo Wiki for roadmap
- Link to issues and milestones
- Update monthly
- Keep markdown docs as reference

---

### 6. Workflow Integration

#### **Issue → Branch → PR → Merge Workflow**

1. **Create Issue**
   ```
   Title: [AUTH] Add session lock feature
   Labels: type: feature, area: frontend, priority: high
   Milestone: v0.1.0-alpha.2
   Project: Platform Core Development
   ```

2. **Create Branch from Issue**
   ```bash
   git checkout -b feature/auth-session-lock-#123
   ```

3. **Reference Issue in Commits**
   ```bash
   git commit -m "feat(auth): add session lock component

   Implements session lock screen with password re-auth.
   Resolves #123"
   ```

4. **Create Pull Request**
   - Auto-links to issue #123
   - Assign reviewers
   - Add to project board (moves to "Review" column)

5. **Merge PR**
   - Issue auto-closes
   - Project card moves to "Done"
   - Milestone progress updates

#### **Automated Actions (via Forgejo Actions - if available)**

Create `.forgejo/workflows/issue-automation.yml`:

```yaml
name: Issue Automation

on:
  issues:
    types: [opened, labeled]

jobs:
  add-to-project:
    runs-on: ubuntu-latest
    steps:
      - name: Add feature issues to Platform Core
        if: contains(github.event.issue.labels.*.name, 'type: feature')
        # Add to project board logic
        
      - name: Add infra issues to Infrastructure project
        if: contains(github.event.issue.labels.*.name, 'area: infrastructure')
        # Add to infrastructure board
```

---

### 7. Migration Plan (Markdown → Forgejo)

#### **Step 1: Audit Current Status**
- [ ] Review `docs/status/current/phase-1-status.md`
- [ ] List all tracked items
- [ ] Identify owners and status

#### **Step 2: Create Label Structure**
- [ ] Create all recommended labels in Forgejo
- [ ] Add label descriptions
- [ ] Set label colors

#### **Step 3: Create Milestones**
- [ ] Phase 1: MVP Foundation
- [ ] v0.1.0-alpha.2
- [ ] v0.2.0-beta.1

#### **Step 4: Create Project Boards**
- [ ] Platform Core Development
- [ ] Infrastructure & DevOps
- [ ] Security & Compliance
- [ ] Documentation

#### **Step 5: Convert Status to Issues**

For each item in phase-1-status.md:
1. Create issue with:
   - Title from status item
   - Description with details
   - Appropriate labels
   - Assign to milestone
   - Add to project board

**Bulk Creation Script:**
```bash
#!/bin/bash
# create-issues-from-status.sh

# Example: Create issue via Forgejo API
curl -X POST "https://your-forgejo.com/api/v1/repos/Unityplan/unityplatform/issues" \
  -H "Authorization: token YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Implement user profile API",
    "body": "Create GET/PATCH endpoints for user profile",
    "labels": ["type: feature", "area: user-service"],
    "milestone": 1
  }'
```

#### **Step 6: Archive Old Status Files**
- [ ] Move `docs/status/current/*.md` to `docs/status/archived/`
- [ ] Update README to point to Forgejo issues/projects
- [ ] Keep architecture docs in markdown

#### **Step 7: Establish New Workflow**
- [ ] Document issue creation process
- [ ] Train team on new workflow
- [ ] Set up weekly review meetings

---

### 8. Best Practices

#### **Issue Writing**
✅ **Good:**
```
Title: [USER] Add profile avatar upload endpoint
Body: 
- Implement POST /api/v1/users/{id}/avatar
- Support image validation (max 5MB, jpg/png)
- Resize to 256x256
- Store in uploads/avatars/

Acceptance:
- [ ] Endpoint accepts multipart/form-data
- [ ] Images validated and resized
- [ ] Returns avatar URL
- [ ] Tests written
```

❌ **Bad:**
```
Title: fix avatar
Body: avatar broken
```

#### **Project Board Hygiene**
- **Daily:** Move cards as work progresses
- **Weekly:** Review "In Progress" - is anything stuck?
- **Bi-weekly:** Groom backlog - add estimates, refine
- **Monthly:** Close completed project, start new one

#### **Milestone Management**
- **Don't overload** - Max 20-30 issues per milestone
- **Review weekly** - Are we on track?
- **Be flexible** - Move issues to next milestone if needed
- **Celebrate** - Close milestone with team review

#### **Label Discipline**
- **Always use labels** - At minimum: type, area, priority
- **Update labels** - Change priority if needed
- **Create sparingly** - Don't create redundant labels
- **Document** - Keep label descriptions up to date

---

### 9. Example Issue Structure

#### **Example 1: Backend Feature**
```
Title: [AUTH] Implement refresh token rotation
Labels: type: feature, area: auth-service, priority: high
Milestone: v0.1.0-alpha.2
Project: Platform Core Development
Assignee: @backend-dev

Description:
Implement refresh token rotation for improved security.

**Current Behavior:**
- Refresh tokens are long-lived (30 days)
- Same token can be reused multiple times

**Desired Behavior:**
- Each refresh issues new refresh token
- Old refresh token invalidated
- Detect token theft (multiple uses)

**Technical Approach:**
- Update refresh endpoint to return new refresh token
- Store token family ID in database
- Invalidate all tokens in family if theft detected

**Acceptance Criteria:**
- [ ] New refresh token returned on each refresh
- [ ] Old token invalidated immediately
- [ ] Token theft detection works
- [ ] Tests cover all scenarios
- [ ] Documentation updated

**Related:**
- Closes #security-audit-2025
- Related to #session-management
```

#### **Example 2: Infrastructure Task**
```
Title: [INFRA] Deploy Norway pod
Labels: type: infrastructure, pod: norway, priority: medium
Milestone: Phase 1: MVP Foundation
Project: Infrastructure & DevOps
Assignee: @devops-lead

Description:
Set up production Norway territory pod.

**Tasks:**
- [ ] Create docker-compose.norway.yml
- [ ] Configure PostgreSQL database (unityplatform_no)
- [ ] Set up NATS messaging
- [ ] Configure Redis cache
- [ ] Deploy monitoring (Prometheus + Grafana)
- [ ] Set up Traefik routing (no.unityplatform.org)
- [ ] Run database migrations
- [ ] Deploy auth-service
- [ ] Deploy user-service
- [ ] Smoke tests

**Environment:**
- Domain: no.unityplatform.org
- Territory code: no
- Region: Europe North

**Rollback Plan:**
- Keep Denmark pod as backup
- DNS can redirect to Denmark if needed

**Documentation:**
- Update pods/norway/README.md
- Add deployment runbook
```

#### **Example 3: Cross-Service Feature**
```
Title: [PLATFORM] Implement badge system
Labels: type: feature, area: badge-service, area: user-service, area: frontend, priority: medium
Milestone: Phase 2: Community Features
Project: Platform Core Development
Assignees: @backend-dev, @frontend-dev

Description:
Full badge/achievement system implementation.

**Components:**
1. **Backend (badge-service):**
   - Badge definition API
   - Badge award logic
   - Badge criteria evaluation

2. **Backend (user-service):**
   - User badge collection
   - Badge display on profile

3. **Frontend:**
   - Badge gallery
   - Badge award notifications
   - Badge display components

**Dependencies:**
- #user-profile-api (must be complete)
- #notification-service (nice to have)

**Acceptance Criteria:**
- [ ] Badges can be defined via API
- [ ] Badges automatically awarded when criteria met
- [ ] Users can view earned badges
- [ ] Frontend displays badges on profile
- [ ] Award notifications work

**Related:**
- Part of gamification epic #gamification-2026
- Depends on #user-profile-api
```

---

### 10. Maintenance Schedule

#### **Daily**
- Review new issues (triage)
- Update issue status
- Move project board cards

#### **Weekly**
- **Monday:** Sprint planning (assign issues for week)
- **Wednesday:** Mid-week check-in
- **Friday:** Week review, close completed issues

#### **Bi-weekly**
- Backlog grooming
- Label cleanup
- Project board review

#### **Monthly**
- Milestone review
- Roadmap update
- Archive old issues
- Team retrospective

---

### 11. Reporting & Metrics

#### **Track These Metrics (via Forgejo)**

1. **Velocity**
   - Issues closed per week
   - Issues closed per milestone

2. **Cycle Time**
   - Time from issue creation → close
   - Time from "In Progress" → "Done"

3. **Bottlenecks**
   - Issues stuck in "Review"
   - Issues stuck in "Testing"

4. **Milestone Health**
   - % complete
   - Days remaining
   - Burndown chart (manual)

#### **Weekly Report Template**

```markdown
# Weekly Development Report - Week of 2025-11-17

## 📊 Metrics
- Issues Closed: 12
- Issues Created: 8
- PRs Merged: 15
- Milestone Progress: Phase 1 (65% complete)

## ✅ Completed This Week
- #123 Session lock feature
- #124 Profile avatar upload
- #125 Security documentation

## 🚧 In Progress
- #126 Badge service foundation
- #127 Territory service API

## 🚨 Blocked
- #128 Waiting on database migration approval

## 🎯 Next Week Focus
- Complete badge service API
- Deploy Norway pod
- Start community forum foundation
```

---

### 12. Integration with Existing Docs

#### **Keep These in Markdown:**
- ✅ Architecture documentation (`docs/architecture/`)
- ✅ API specifications (`docs/architecture/services/*/API.md`)
- ✅ Database schemas (`docs/architecture/services/*/DATABASE.md`)
- ✅ Development guides (`docs/guides/development/`)
- ✅ Deployment runbooks (`docs/guides/deployment/`)

#### **Move to Forgejo Issues:**
- ❌ Status tracking (`docs/status/current/`)
- ❌ Task lists
- ❌ Individual feature tracking

#### **Hybrid Approach:**
- **Markdown:** Technical specifications, architecture decisions
- **Forgejo Issues:** Work tracking, task management
- **Link Between:** Reference issues in markdown, link to docs in issues

**Example:**
```markdown
<!-- In docs/architecture/services/badge-service/ARCHITECTURE.md -->

# Badge Service Architecture

## Implementation Status
See issues: #126, #127, #128

## Current Progress
Track progress in [Platform Core Development](https://forgejo.com/Unityplan/unityplatform/projects/1)
```

---

## Quick Start Guide

### Day 1: Setup
1. Create labels (30 min)
2. Create milestones (15 min)
3. Create project boards (30 min)
4. Create issue templates (30 min)

### Day 2-3: Migration
1. Review phase-1-status.md
2. Create issues for all active work
3. Assign to milestones and projects
4. Add labels

### Day 4-5: Training
1. Document new workflow
2. Train team
3. Run pilot sprint

### Week 2+: Optimize
1. Collect feedback
2. Adjust labels/boards
3. Refine process
4. Archive old markdown status files

---

## Conclusion

This structure provides:
- ✅ Clear visual progress tracking (Project boards)
- ✅ Organized issue management (Labels, milestones)
- ✅ Cross-component visibility (Multi-label system)
- ✅ Automated workflows (Issue → PR → Merge)
- ✅ Roadmap planning (Milestones + Wiki)
- ✅ Team collaboration (Assignments, comments)
- ✅ Better than scattered markdown files

**Recommendation:** Start with 4 project boards and comprehensive labels. Migrate gradually over 2 weeks. Keep architecture docs in markdown but move all status tracking to Forgejo issues.

---

**Last Updated:** November 17, 2025  
**Version:** 1.0  
**Maintained by:** Unity Platform Core Team
