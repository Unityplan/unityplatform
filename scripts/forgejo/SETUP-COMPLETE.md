# Forgejo Scripts Setup Complete! 🎉

## What We Created

You now have a complete set of scripts for managing Forgejo issues via API:

### 📁 Files Created

```
scripts/forgejo/
├── .env.example          # Environment template (copy to .env)
├── QUICKSTART.md         # 5-minute setup guide
├── README.md             # Complete documentation
├── convert-status-to-issues.sh   # Convert markdown → Forgejo issues
├── create-from-roadmap.sh        # Interactive issue creator
├── create-issue.sh               # Quick CLI issue creator
├── list-labels.sh                # List all repository labels
├── list-milestones.sh            # List all milestones
└── parse-status.py               # Python status file parser
```

### 🚀 Quick Start (5 minutes)

Follow **`scripts/forgejo/QUICKSTART.md`** for step-by-step setup:

1. **Generate API token** in Forgejo (Settings → Applications)
2. **Copy `.env.example` to `.env`** and add your token
3. **Test connection**: `./list-labels.sh`
4. **Create first issue**: `./create-issue.sh "Test" "Description"`

## How to Use

### Option 1: Quick CLI (Fastest)

```bash
cd scripts/forgejo

# Create issue with all details in one command
./create-issue.sh \
  "Implement login page" \
  "Create React login form with validation" \
  "priority:high,type:feature,area:frontend" \
  "v0.1.0-alpha.2"
```

### Option 2: Interactive (Easiest)

```bash
cd scripts/forgejo

# Answer prompts to create issue
./create-from-roadmap.sh

# It will ask:
# - What stage? (5 for Frontend, 7 for Course, etc.)
# - Issue title?
# - Description?
# - Priority? (critical/high/medium/low)
# - Type? (feature/bug/docs/testing/refactor)
```

### Option 3: Batch Convert (Future)

When you're ready to convert all markdown tasks to issues:

```bash
cd scripts/forgejo

# Preview what would be created
./convert-status-to-issues.sh --dry-run --stage 5

# Actually create issues
./convert-status-to-issues.sh --stage 5
```

## Your Next Steps

### 1. Setup (Required - 5 minutes)

```bash
cd scripts/forgejo
cp .env.example .env
nano .env  # Add your Forgejo API token
./list-labels.sh  # Test connection
```

### 2. Create First Real Issue (2 minutes)

Use the interactive tool:

```bash
./create-from-roadmap.sh

# Example inputs:
Stage: 5
Title: Implement login page UI
Description: (press Enter for default)
Priority: 2 (high)
Type: 1 (feature)
Confirm: y
```

### 3. Plan Stage 5 Work (10 minutes)

Create 5-10 issues for your immediate work on Frontend Auth:

```bash
# Issue 1: Login page
./create-from-roadmap.sh
# Stage: 5, Title: "Create login page", Priority: high, Type: feature

# Issue 2: Registration page
./create-from-roadmap.sh
# Stage: 5, Title: "Create registration form", Priority: high, Type: feature

# Issue 3: Protected routes
./create-from-roadmap.sh
# Stage: 5, Title: "Implement route protection", Priority: high, Type: feature

# Continue for 2-3 more issues...
```

### 4. Use Your Project Board

1. Open: http://localhost:3000/Unityplan/unityplatform/projects/1
2. Drag issues to "In Progress" as you work
3. Move to "Done" when complete
4. Celebrate progress! 🎉

## Security Notes

⚠️ **IMPORTANT:**

- `.env` contains your API token - **NEVER commit this to git!**
- ✅ Already in `.gitignore` - you're protected
- 🔒 Treat API tokens like passwords
- 🔄 Regenerate tokens if compromised

## Scripts Reference

| Script | Purpose | Example |
|--------|---------|---------|
| `list-labels.sh` | Show all labels | `./list-labels.sh` |
| `list-milestones.sh` | Show all milestones | `./list-milestones.sh` |
| `create-issue.sh` | Quick CLI create | `./create-issue.sh "Title" "Body" "labels"` |
| `create-from-roadmap.sh` | Interactive create | `./create-from-roadmap.sh` |
| `parse-status.py` | Parse status file | `python3 parse-status.py --format stats` |
| `convert-status-to-issues.sh` | Batch convert | `./convert-status-to-issues.sh --stage 5` |

## Workflow

**Daily (5 minutes):**

```bash
# Morning: Create issues for today's work
cd scripts/forgejo
./create-from-roadmap.sh  # Create 2-3 issues

# During work: Update in Forgejo web UI
# - Move issues through project board
# - Add comments with progress

# Evening: Review progress in project board
```

**Weekly (15 minutes):**

```bash
# Review milestone progress
./list-milestones.sh

# Create issues for next week
./create-from-roadmap.sh  # Run 5-10 times for upcoming work

# Clean up: Close completed issues, update descriptions
```

## Git Status

✅ **Committed locally** (commit c9ae906)

⚠️ **Not pushed to Forgejo yet** - Repository doesn't exist in Forgejo

**To push:**
1. Create repository in Forgejo: http://localhost:3000/repo/create
   - Owner: `Unityplan`
   - Repository Name: `unityplatform`
2. Then push: `git push forgejo main`

**Alternative:** The commit is saved locally, so you can push later when repository is created.

## Documentation

- **Quick Start**: `scripts/forgejo/QUICKSTART.md` - Start here!
- **Full Docs**: `scripts/forgejo/README.md` - Complete reference
- **Forgejo Strategy**: `docs/guides/development/forgejo-project-management.md`
- **Beginner Guide**: `docs/guides/development/forgejo-quickstart.md`

## What's Different from Before?

**Before:**
- ❌ Markdown status files (messy, hard to track)
- ❌ Manual todo lists
- ❌ No visual progress tracking

**Now:**
- ✅ Forgejo issues (structured, searchable)
- ✅ Project board (visual Kanban)
- ✅ Labels, milestones, filtering
- ✅ API automation
- ✅ Professional workflow

## Get Help

**Common Issues:**

1. "Error: .env file not found!"
   → Run: `cp .env.example .env`

2. "Failed to create issue" / "Unauthorized"
   → Check token in `.env` is correct

3. "Label not found"
   → Run `./list-labels.sh` to see exact names

**Still stuck?** Check the full README: `scripts/forgejo/README.md`

---

**Ready to start?** → Open `scripts/forgejo/QUICKSTART.md` and follow the 5-minute setup! 🚀
