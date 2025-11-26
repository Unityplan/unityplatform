# Quick Start - Setting Up Forgejo Issue Tracking

Follow these steps to get started with Forgejo issue management in **5 minutes**.

## Step 1: Generate Forgejo API Token (2 minutes)

1. Open Forgejo in your browser:

   ```bash
   open http://localhost:3000
   ```

2. Login to Forgejo (or create account if first time)

3. Go to **Settings** → **Applications**

4. Under "Generate New Token":
   - **Token Name**: `CLI Scripts` or `Issue Automation`
   - **Select scopes**: Check ✅ **repo** (all repo permissions)
   - Click **Generate Token**

5. **IMPORTANT**: Copy the token immediately (you can't see it again!)

   ```
   Example token: a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0
   ```

## Step 2: Configure Environment (1 minute)

```bash
# Navigate to scripts directory
cd scripts/forgejo

# Copy example environment file
cp .env.example .env

# Edit .env file
nano .env
```

**Update these values in `.env`:**

```bash
FORGEJO_URL=http://localhost:3000
FORGEJO_TOKEN=paste_your_token_here  # ← PASTE YOUR TOKEN
FORGEJO_OWNER=Unityplan
FORGEJO_REPO=unityplatform
DEFAULT_MILESTONE=v0.1.0-alpha.2
```

**Save and close** (Ctrl+O, Enter, Ctrl+X)

## Step 3: Test Connection (30 seconds)

```bash
# List your labels (should show the 18 labels you created)
./list-labels.sh
```

**Expected output:**

```
📋 Labels in Unityplan/unityplatform:

  1 - priority:critical (d73a4a)
  2 - priority:high (ff6b6b)
  3 - priority:medium (feca57)
  ...

Total: 18
```

✅ **If you see your labels, you're connected!**

## Step 4: Create Your First Issue (1 minute)

```bash
# Create a simple test issue
./create-issue.sh \
  "Test issue from CLI" \
  "This is a test to verify the API connection works" \
  "priority:low,type:feature" \
  "v0.1.0-alpha.2"
```

**Expected output:**

```
✅ Created issue #1: Test issue from CLI
🔗 http://localhost:3000/Unityplan/unityplatform/issues/1
```

Click the link to verify the issue was created in Forgejo!

## Step 5: Explore Your Status (30 seconds)

```bash
# See what tasks are in your status file
python3 parse-status.py --format stats
```

This shows you how many tasks exist in each stage of your project.

---

## What's Next?

Now that you're set up, you can:

### Create Issues Manually

```bash
# Feature request
./create-issue.sh \
  "Add dark mode toggle" \
  "Users want a dark theme option in settings" \
  "priority:medium,type:feature,area:frontend" \
  "v0.1.0-alpha.2"

# Bug report
./create-issue.sh \
  "Login fails with special characters" \
  "Users with @ in username cannot login" \
  "priority:high,type:bug,area:backend"

# Documentation task
./create-issue.sh \
  "Update API documentation" \
  "Add examples for all auth endpoints" \
  "priority:low,type:docs,area:backend"
```

### Convert Roadmap to Issues

When you're ready to plan Stage 5 (Frontend Auth), you can manually create issues based on the roadmap:

```bash
# Example: Create issue for frontend login page
./create-issue.sh \
  "Stage 5: Implement login page" \
  "Create React login form with username/password fields, form validation, and error handling" \
  "priority:high,type:feature,area:frontend" \
  "v0.1.0-alpha.2"
```

### Check Your Work

```bash
# List all labels
./list-labels.sh

# List all milestones
./list-milestones.sh

# View issues in browser
open http://localhost:3000/Unityplan/unityplatform/issues
```

---

## Troubleshooting

### "Error: .env file not found!"

```bash
# Make sure you're in the right directory
cd /home/henrik/projects/unityplan_platform/workspace/scripts/forgejo

# Copy the example file
cp .env.example .env
nano .env
```

### "Failed to create issue" or "Unauthorized"

1. Check your token is correct in `.env`
2. Regenerate token in Forgejo if needed
3. Make sure token has **repo** scope permissions

### "Label not found"

```bash
# List your actual labels
./list-labels.sh

# Use exact label names from the list
./create-issue.sh "My Issue" "Description" "priority/high,type/feature"
```

---

## Daily Workflow

**Morning routine (2 minutes):**

1. Check project board: <http://localhost:3000/Unityplan/unityplatform/projects/1>
2. Move issues to "In Progress" as you start work
3. Create new issues for any discoveries

**During work:**

- Update issue descriptions as you learn more
- Add comments with progress notes
- Move issues through the board

**End of day (2 minutes):**

1. Update issue status (move to "Testing" or "Done")
2. Create issues for tomorrow's work
3. Review milestone progress

---

## Success Checklist

- ✅ API token generated and saved in `.env`
- ✅ `list-labels.sh` shows your 18 labels
- ✅ Created first test issue successfully
- ✅ Can view issues in Forgejo web interface
- ✅ Project board shows your issues

**You're ready to go! 🚀**

---

## Related Docs

- `README.md` - Full script documentation
- `docs/guides/development/forgejo-quickstart.md` - Beginner guide
- `docs/guides/development/forgejo-project-management.md` - Advanced strategy
