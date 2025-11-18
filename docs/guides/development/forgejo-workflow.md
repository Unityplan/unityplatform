# Forgejo Workflow Guide

**Last Updated:** November 17, 2025  
**Forgejo Instance:** <http://localhost:3000>  
**Repository:** henrik/unity_platform

This guide explains how to use Forgejo for issue tracking, project management, and collaboration on the Unity Platform.

---

## 📋 Table of Contents

- [Overview](#overview)
- [Accessing Forgejo](#accessing-forgejo)
- [Issue Tracking](#issue-tracking)
- [Label System](#label-system)
- [Workflow](#workflow)
- [Best Practices](#best-practices)
- [Milestones](#milestones)
- [Troubleshooting](#troubleshooting)

---

## Overview

The Unity Platform uses Forgejo (self-hosted Git forge) for comprehensive issue tracking. All Phase 1 MVP work is tracked through 137 issues organized by stage, with proper labeling, milestones, and status tracking.

**Why Forgejo?**

- ✅ Self-hosted (data sovereignty)
- ✅ Open source (freedom)
- ✅ Git integration (version control)
- ✅ Issue tracking (project management)
- ✅ Pull requests (code review)
- ✅ Milestones (release planning)

---

## Accessing Forgejo

### Web Interface

```
http://localhost:3000
```

**Quick Links:**

- [All Issues](http://localhost:3000/henrik/unity_platform/issues)
- [Open Issues](http://localhost:3000/henrik/unity_platform/issues?state=open)
- [Closed Issues](http://localhost:3000/henrik/unity_platform/issues?state=closed)
- [Milestone v0.1.0-alpha.2](http://localhost:3000/henrik/unity_platform/milestones)
- [Labels](http://localhost:3000/henrik/unity_platform/labels)

### API Access

```bash
# Set environment variables
export FORGEJO_URL="http://localhost:3000"
export FORGEJO_TOKEN="your-token-here"
export FORGEJO_OWNER="henrik"
export FORGEJO_REPO="unity_platform"

# Example: List all issues
curl -s -H "Authorization: token ${FORGEJO_TOKEN}" \
  "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues"
```

---

## Issue Tracking

### Issue Structure

All Phase 1 issues follow this pattern:

**Title Format:** `Stage X.Y: Task Description`

- `X` = Stage number (1-14)
- `Y` = Task number within stage
- Example: `Stage 5.1: Create Vite + React + TypeScript project`

**Description Components:**

1. **Context** - What this task is about
2. **Requirements** - What needs to be done
3. **Acceptance Criteria** - How to verify completion
4. **Dependencies** - Related issues or prerequisites
5. **Notes** - Additional information or considerations

### Issue States

| State | Meaning | When to Use |
|-------|---------|-------------|
| **Open** | Not yet started or in progress | Active development, future work |
| **Closed** | Completed and verified | Task done, merged, tested |

### Finding Issues

**By Stage:**

```
Stage 1-4: #42-#67 (closed - historical)
Stage 5: #4-#30 (open - active)
Stage 6: #31-#41 (closed - historical)
Stage 7: #68-#73 (open - planned)
Stage 8: #74-#79 (open - planned)
Stage 9: #82-#87 (open - planned)
Stage 10: #88-#95 (open - planned)
Stage 11: #96-#98 (open - planned)
Stage 12: #99-#108 (open - planned)
Stage 13: #109-#119 (open - planned)
Stage 14: #120-#136 (closed - historical)
```

**By Label:**

- [Frontend Issues](http://localhost:3000/henrik/unity_platform/issues?labels=14)
- [High Priority](http://localhost:3000/henrik/unity_platform/issues?labels=1)
- [In Progress](http://localhost:3000/henrik/unity_platform/issues?labels=17)
- [Auth Service](http://localhost:3000/henrik/unity_platform/issues?labels=9)

**By Milestone:**

- [v0.1.0-alpha.2](http://localhost:3000/henrik/unity_platform/issues?milestone=1)

---

## Label System

We use 24 labels organized into 4 categories:

### Priority Labels (4)

| Label | ID | Color | When to Use |
|-------|-----|-------|-------------|
| **priority/critical** | 0 | #d73a4a | Production-breaking bugs, security issues |
| **priority/high** | 1 | #ff6b6b | MVP blockers, core functionality |
| **priority/medium** | 2 | #fbca04 | Important but not blocking |
| **priority/low** | 4 | #0e8a16 | Nice to have, future improvements |

### Type Labels (4)

| Label | ID | Color | When to Use |
|-------|-----|-------|-------------|
| **type/bug** | 6 | #d73a4a | Something is broken or incorrect |
| **type/feature** | 5 | #a2eeef | New functionality or capability |
| **type/enhancement** | 7 | #84b6eb | Improvement to existing feature |
| **type/infrastructure** | 8 | #0052cc | DevOps, deployment, architecture |

### Area Labels - Services (10)

| Label | ID | Service | Stage |
|-------|-----|---------|-------|
| **area/auth-service** | 9 | Authentication | 3 |
| **area/user-service** | 10 | User Management | 4 |
| **area/badge-service** | 11 | Badges & Achievements | 6 |
| **area/territory-service** | 12 | Territory Management | 6 |
| **area/utility-service** | 13 | Utilities (favicon, etc.) | 14 |
| **area/course-service** | 19 | LMS/Courses | 7 |
| **area/matrix-bridge** | 20 | Matrix Integration | 8 |
| **area/ipfs-service** | 21 | IPFS Storage | 9 |
| **area/forum-service** | 22 | Forum/Discussion | 10 |
| **area/translation-service** | 23 | i18n/l10n | 11 |

### Area Labels - General (3)

| Label | ID | Area | Stage |
|-------|-----|------|-------|
| **area/frontend-app** | 14 | React Frontend | 5, 12 |
| **area/infrastructure** | 15 | Docker, DB, etc. | 1, 2 |
| **area/testing** | 24 | Tests, QA | 13 |

### Status Labels (3)

| Label | ID | Color | When to Use |
|-------|-----|-------|-------------|
| **status/blocked** | 16 | #b60205 | Cannot proceed (dependency, decision needed) |
| **status/in-progress** | 17 | #fbca04 | Actively being worked on |
| **status/needs-review** | 18 | #0075ca | Ready for code review |

### Label Combinations

**Recommended combinations:**

```
New Feature:
  type/feature + priority/high + area/auth-service

Bug Fix:
  type/bug + priority/critical + area/user-service + status/in-progress

Infrastructure:
  type/infrastructure + priority/medium + area/infrastructure

Enhancement:
  type/enhancement + priority/low + area/frontend-app + status/needs-review
```

---

## Workflow

### Standard Development Flow

```
1. Pick Issue
   └─> Find open issue in current stage
   └─> Check dependencies are met
   └─> Assign to yourself
   └─> Add status/in-progress label

2. Create Branch
   └─> git checkout -b issue-#-description
   └─> Example: git checkout -b issue-4-vite-react-setup

3. Implement
   └─> Follow coding guidelines
   └─> Write tests
   └─> Update documentation
   └─> Commit regularly with clear messages

4. Test
   └─> Run unit tests
   └─> Test in dev environment
   └─> Verify acceptance criteria

5. Create PR
   └─> Push branch to remote
   └─> Create pull request
   └─> Link to issue (#4)
   └─> Add status/needs-review label

6. Review
   └─> Address feedback
   └─> Update code as needed
   └─> Get approval

7. Merge
   └─> Squash and merge
   └─> Delete branch
   └─> Close issue
   └─> Remove labels
```

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

Refs: #<issue-number>
```

**Examples:**

```bash
# Feature implementation
git commit -m "feat(auth): implement JWT token generation

- Add token generation service
- Add token validation middleware
- Add refresh token support

Refs: #51"

# Bug fix
git commit -m "fix(user): handle null email in profile update

- Add null check before email validation
- Return clear error message
- Add test case

Refs: #62"

# Documentation
git commit -m "docs(forgejo): add workflow guide

- Explain label system
- Document development flow
- Add troubleshooting section

Refs: #4"
```

### Branch Naming

```
issue-<number>-<short-description>

Examples:
  issue-4-vite-react-setup
  issue-68-course-service-scaffolding
  issue-51-jwt-token-generation
```

---

## Best Practices

### Issue Management

✅ **DO:**

- Assign issues to yourself when starting work
- Update labels to reflect current status
- Comment on progress and blockers
- Link related issues with `#<number>`
- Close issues only when fully complete
- Add acceptance criteria to issue descriptions
- Break large tasks into smaller issues

❌ **DON'T:**

- Leave issues assigned when not actively working
- Close issues before code is merged
- Create duplicate issues (search first)
- Use vague titles like "Fix bug" or "Update code"
- Skip testing before closing
- Ignore dependencies

### Communication

**Issue Comments:**

```markdown
<!-- Progress update -->
Working on this now. Completed:
- [x] Database schema
- [x] API endpoints
- [ ] Frontend integration (in progress)

<!-- Blocker -->
⚠️ Blocked by #68 - need course schema before implementing enrollment

<!-- Question -->
@reviewer Should we use JWT or session tokens here?

<!-- Solution found -->
✅ Fixed by using shared-lib middleware pattern
```

### Testing

Before closing an issue:

```bash
# 1. Run unit tests
cargo test            # Rust services
npm test              # Frontend

# 2. Run integration tests
./scripts/test/integration.sh

# 3. Test in dev environment
docker compose -f docker-compose.dev.yml up

# 4. Verify acceptance criteria
- [ ] All requirements met
- [ ] Tests passing
- [ ] Documentation updated
- [ ] No regressions
```

---

## Milestones

### v0.1.0-alpha.2 (Current)

**Goal:** Complete Phase 1 MVP  
**Due Date:** TBD  
**Progress:** 55/137 issues closed (40%)

**Stages:**

- ✅ Stages 1-4: Foundation complete
- 🔄 Stage 5: Frontend in progress
- ✅ Stage 6: Territory/Badge complete
- 📋 Stages 7-13: Planned
- ✅ Stage 14: Utility service complete

**Links:**

- [Milestone Overview](http://localhost:3000/henrik/unity_platform/milestones)
- [Open Issues](http://localhost:3000/henrik/unity_platform/issues?milestone=1&state=open)
- [Closed Issues](http://localhost:3000/henrik/unity_platform/issues?milestone=1&state=closed)

---

## Troubleshooting

### Common Issues

**Issue not appearing after creation:**

```bash
# Check API response
curl -s -H "Authorization: token ${FORGEJO_TOKEN}" \
  "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues" | jq .

# Verify environment variables
echo $FORGEJO_URL
echo $FORGEJO_TOKEN
```

**Labels not applying:**

```bash
# List all labels with IDs
curl -s -H "Authorization: token ${FORGEJO_TOKEN}" \
  "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/labels" | jq .

# Check label IDs in script match Forgejo
```

**Cannot access Forgejo:**

```bash
# Check if running
docker ps | grep forgejo

# Start if not running
cd /path/to/forgejo && docker compose up -d

# Check logs
docker logs forgejo
```

### Getting Help

1. **Check documentation:**
   - [Forgejo Docs](https://forgejo.org/docs/)
   - [API Reference](https://forgejo.org/docs/latest/api/)

2. **Review existing issues:**
   - Search for similar problems
   - Check closed issues for solutions

3. **Ask in Matrix:**
   - (Coming in Stage 8)

4. **Create support issue:**
   - Label: `type/bug + area/infrastructure`
   - Include error messages and logs

---

## Quick Reference

### Useful Queries

```bash
# Issues by priority
http://localhost:3000/henrik/unity_platform/issues?labels=1  # High
http://localhost:3000/henrik/unity_platform/issues?labels=0  # Critical

# Issues by status
http://localhost:3000/henrik/unity_platform/issues?labels=17  # In Progress
http://localhost:3000/henrik/unity_platform/issues?labels=16  # Blocked

# Issues by area
http://localhost:3000/henrik/unity_platform/issues?labels=9   # Auth Service
http://localhost:3000/henrik/unity_platform/issues?labels=14  # Frontend

# Open issues in milestone
http://localhost:3000/henrik/unity_platform/issues?milestone=1&state=open
```

### API Endpoints

```bash
# List issues
GET /api/v1/repos/{owner}/{repo}/issues

# Get issue
GET /api/v1/repos/{owner}/{repo}/issues/{index}

# Create issue
POST /api/v1/repos/{owner}/{repo}/issues

# Update issue
PATCH /api/v1/repos/{owner}/{repo}/issues/{index}

# Close issue
PATCH /api/v1/repos/{owner}/{repo}/issues/{index}
Body: {"state": "closed"}

# Add label
POST /api/v1/repos/{owner}/{repo}/issues/{index}/labels
Body: {"labels": [1, 2, 3]}
```

---

## Related Documentation

- [Migration Scripts README](../../scripts/forgejo/README.md)
- [Phase 1 Status](../../docs/status/current/phase-1-status.md)
- [Development Guidelines](./coding-guidelines.md)
- [Git Workflow](./git-workflow.md)

---

**Questions?** Create an issue with label `type/enhancement + area/infrastructure`
