# Forgejo Scripts

**Purpose:** Automate creation and management of Forgejo issues for Unity Platform  
**Last Updated:** November 17, 2025  
**Total Issues Created:** 137 (55 closed, 82 open)

Helper scripts for managing Forgejo issues, labels, and milestones via API. All Phase 1 MVP stages have been migrated to Forgejo issue tracking.

---

## 📋 Migration Overview

All 14 stages of Phase 1 MVP are now tracked in Forgejo:

| Script | Issues | Range | State | Purpose |
|--------|--------|-------|-------|---------|
| `create-stages-1-4-closed-issues.sh` | 26 | #42-#67 | Closed | Stages 1-4 (Foundation) |
| `create-stage5-open-issues.sh` | 27 | #4-#30 | Open | Stage 5 (Frontend) |
| `create-stage6-closed-issues.sh` | 11 | #31-#41 | Closed | Stage 6 (Territory/Badge) |
| `create-stage7-open-issues.sh` | 6 | #68-#73 | Open | Stage 7 (Course Service) |
| `create-stage8-open-issues.sh` | 6 | #74-#79 | Open | Stage 8 (Matrix) |
| `create-stage9-open-issues.sh` | 6 | #82-#87 | Open | Stage 9 (IPFS) |
| `create-stage10-open-issues.sh` | 8 | #88-#95 | Open | Stage 10 (Forum) |
| `create-stage11-open-issues.sh` | 3 | #96-#98 | Open | Stage 11 (Translation) |
| `create-stage12-open-issues.sh` | 10 | #99-#108 | Open | Stage 12 (Frontend UI) |
| `create-stage13-open-issues.sh` | 11 | #109-#119 | Open | Stage 13 (Testing/Deploy) |
| `create-stage14-closed-issues.sh` | 18 | #120-#136 | Closed | Stage 14 (Utility Service) |

**🔗 View All Issues:** [Forgejo Milestone v0.1.0-alpha.2](http://localhost:3000/henrik/unity_platform/milestones)

---

## ⚙️ Setup

1. **Copy the environment template:**

   ```bash
   cp .env.example .env
   ```

2. **Generate a Forgejo API token:**
   - Go to: <http://localhost:3000> (or your Forgejo instance)
   - Settings → Applications → Generate New Token
   - Name: "CLI Scripts" or similar
   - Select scopes: **repo** (all repo permissions)
   - Copy the generated token

3. **Edit `.env` file:**

   ```bash
   nano .env
   ```

   Update these values:

   ```bash
   FORGEJO_URL=http://localhost:3000
   FORGEJO_TOKEN=your_actual_token_here
   FORGEJO_OWNER=henrik
   FORGEJO_REPO=unity_platform
   DEFAULT_MILESTONE=v0.1.0-alpha.2
   ```

4. **Make scripts executable:**

   ```bash
   chmod +x *.sh
   ```

---

## 🚀 Migration Scripts

### Stage 1-4: Foundation (Closed)

**Script:** `create-stages-1-4-closed-issues.sh`  
**Issues:** 26 (#42-#67)  
**Status:** Completed historical work

```bash
./create-stages-1-4-closed-issues.sh
```

Covers:
- Stage 1: Foundation & Infrastructure
- Stage 2: Database Schema & Migrations  
- Stage 3: Authentication Service
- Stage 4: User Service (incl. Settings)

### Stage 5: Frontend (Open)

**Script:** `create-stage5-open-issues.sh`  
**Issues:** 27 (#4-#30)  
**Status:** Active development

```bash
./create-stage5-open-issues.sh
```

Includes: Vite setup, TanStack Router, authentication UI, profile pages, settings pages, responsive design.

### Stage 6: Territory & Badge Services (Closed)

**Script:** `create-stage6-closed-issues.sh`  
**Issues:** 11 (#31-#41)  
**Status:** Completed historical work

```bash
./create-stage6-closed-issues.sh
```

### Stage 7-13: Future Work (Open)

All future stages are tracked as open issues for planning:

```bash
./create-stage7-open-issues.sh   # Course Service (6 issues)
./create-stage8-open-issues.sh   # Matrix Protocol (6 issues)
./create-stage9-open-issues.sh   # IPFS Service (6 issues)
./create-stage10-open-issues.sh  # Forum Service (8 issues)
./create-stage11-open-issues.sh  # Translation Service (3 issues)
./create-stage12-open-issues.sh  # Frontend UI (10 issues)
./create-stage13-open-issues.sh  # Testing/Deploy (11 issues)
```

### Stage 14: Utility Service (Closed)

**Script:** `create-stage14-closed-issues.sh`  
**Issues:** 18 (#120-#136)  
**Status:** Completed historical work

```bash
./create-stage14-closed-issues.sh
```

---

## 🔧 Original Utility Scripts

### 🔄 `convert-status-to-issues.sh`

Convert incomplete tasks from `phase-1-status.md` to Forgejo issues (legacy tool, migration complete).

**Usage:**

```bash
# Dry run (preview without creating)
./convert-status-to-issues.sh --dry-run

# Create all issues
./convert-status-to-issues.sh

# Only convert Stage 5 tasks
./convert-status-to-issues.sh --stage 5

# Limit to first 10 issues
./convert-status-to-issues.sh --limit 10

# Combine options
./convert-status-to-issues.sh --dry-run --stage 5 --limit 5
```

**Features:**

- ✅ Parses markdown checkboxes (❌ and ⏸️ = incomplete)
- ✅ Maps tasks to appropriate labels (priority, type, area)
- ✅ Sets milestone from `.env` configuration
- ✅ Dry-run mode for testing
- ✅ Stage filtering
- ✅ Issue limit

**What it converts:**

- Stage 5: Frontend Auth & Profile (0% complete)
- Stage 7: Course Service (LMS) (0% complete)
- Stage 8: Matrix Protocol Integration (0% complete)
- Stage 9: IPFS Service (0% complete)
- Stage 10: Forum Service (0% complete)
- Stage 11: Translation Service (0% complete)
- Stage 12: Frontend Course & Forum UI (0% complete)
- Stage 13: Testing & Documentation (0% complete)

### 📋 `list-labels.sh`

List all labels in your repository.

**Usage:**

```bash
# Human-readable output
./list-labels.sh

# JSON output
./list-labels.sh --json
```

**Example output:**

```
📋 Labels in Unityplan/unityplatform:

  1 - priority:critical (d73a4a)
  2 - priority:high (ff6b6b)
  3 - priority:medium (feca57)
  4 - type:feature (48dbfb)
  ...

Total: 18
```

### 🎯 `list-milestones.sh`

List all milestones in your repository.

**Usage:**

```bash
# Human-readable output
./list-milestones.sh

# JSON output
./list-milestones.sh --json
```

**Example output:**

```
🎯 Milestones in Unityplan/unityplatform:

  1 - v0.1.0-alpha.2 [open] (5/0 issues)
  2 - v0.1.0-alpha.3 [open] (0/0 issues)

Total: 2
```

### ➕ `create-issue.sh`

Quick helper to create a single issue from command line.

**Usage:**

```bash
./create-issue.sh "Issue Title" "Issue Body" [label1,label2,...] [milestone]
```

**Examples:**

```bash
# Simple issue (no labels or milestone)
./create-issue.sh "Fix login bug" "Users cannot login with special characters"

# With labels
./create-issue.sh "Add dark mode" "Implement dark theme toggle" "priority:medium,type:feature,area:frontend"

# With labels and milestone
./create-issue.sh \
  "Implement JWT refresh" \
  "Add automatic token refresh on 401" \
  "priority:high,type:feature,area:backend" \
  "v0.1.0-alpha.2"
```

**Output:**

```
✅ Created issue #42: Add dark mode
🔗 http://localhost:3000/Unityplan/unityplatform/issues/42
```

## Workflow

### Initial Setup (One-time)

1. Create labels (already done if you followed the quick-start guide)
2. Create milestone `v0.1.0-alpha.2` (already done)
3. Create `.env` file with your API token

### Daily Usage

**Convert markdown tasks to issues:**

```bash
# Preview first (always recommended)
./convert-status-to-issues.sh --dry-run --stage 5

# If it looks good, create them
./convert-status-to-issues.sh --stage 5
```

**Create ad-hoc issues:**

```bash
./create-issue.sh \
  "Update documentation" \
  "Add API examples to README" \
  "priority:low,type:docs" \
  "v0.1.0-alpha.2"
```

**Check your labels/milestones:**

```bash
./list-labels.sh
./list-milestones.sh
```

## Security Notes

- ⚠️ **NEVER commit `.env` to git** - it contains your API token!
- ✅ `.env` is already in `.gitignore`
- 🔒 Treat API tokens like passwords
- 🔄 Rotate tokens periodically
- 🚫 Don't share tokens in chat/email

## Troubleshooting

### "Error: .env file not found!"

**Solution:**

```bash
cp .env.example .env
nano .env  # Add your token
```

### "Failed to create issue"

**Check:**

1. Token is valid (regenerate if needed)
2. Token has `repo` scope permissions
3. Repository owner/name in `.env` is correct
4. Forgejo is running (`curl http://localhost:3000`)

### "Label not found: priority:high"

**Solution:**
Create the label first in Forgejo UI or check the exact label name:

```bash
./list-labels.sh
```

### "Milestone not found"

**Solution:**
Check milestone name matches exactly:

```bash
./list-milestones.sh
```

Update `.env` with correct milestone title.

## Next Steps

1. ✅ Setup complete - you can now create issues via scripts
2. 📋 Convert Stage 5 tasks: `./convert-status-to-issues.sh --stage 5`
3. 🔄 Repeat for other stages as needed
4. 📝 Use `create-issue.sh` for ad-hoc issues
5. 🎯 Track progress in Forgejo project board

## Related Documentation

- `docs/guides/development/forgejo-quickstart.md` - Beginner setup guide
- `docs/guides/development/forgejo-project-management.md` - Comprehensive strategy
- Forgejo API docs: <https://forgejo.org/docs/latest/api/>
