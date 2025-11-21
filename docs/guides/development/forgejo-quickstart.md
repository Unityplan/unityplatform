# Forgejo Quick Start Guide - Step by Step

## 🎯 Goal

Set up a practical, maintainable issue tracking system in Forgejo for the Unity Platform mono-repo.

**Philosophy:** Start simple, add complexity as needed. Better to have a working basic system than a perfect unused one.

---

## Phase 1: Essential Setup (Week 1)

### Step 1: Create Core Labels (30 minutes)

Labels are tags that categorize issues. Start with just the essentials.

#### Go to: Repository → Issues → Labels → New Label

**Priority Labels (4 total):**

```
Name: priority: critical
Color: #d73a4a (red)
Description: Blocking production or critical bug

Name: priority: high  
Color: #ff9800 (orange)
Description: Important for current milestone

Name: priority: medium
Color: #fbca04 (yellow)
Description: Normal priority work

Name: priority: low
Color: #0e8a16 (green)
Description: Nice to have, future work
```

**Type Labels (4 total):**

```
Name: type: feature
Color: #1d76db (blue)
Description: New functionality

Name: type: bug
Color: #d73a4a (red)
Description: Something is broken

Name: type: enhancement
Color: #84b6eb (light blue)
Description: Improve existing feature

Name: type: infrastructure
Color: #5319e7 (purple)
Description: DevOps, deployment, infrastructure work
```

**Area Labels (7 total - your main services):**

```
Name: area: auth-service
Color: #c5def5
Description: Authentication service (port 8001)

Name: area: user-service
Color: #c5def5
Description: User service (port 8002)

Name: area: badge-service
Color: #c5def5
Description: Badge service (port 8003)

Name: area: territory-service
Color: #c5def5
Description: Territory service (port 8004)

Name: area: utility-service
Color: #c5def5
Description: Utility service (port 8014)

Name: area: frontend
Color: #d4c5f9
Description: React frontend application

Name: area: infrastructure
Color: #e99695
Description: Docker, databases, deployment
```

**Status Labels (3 total):**

```
Name: status: blocked
Color: #000000 (black)
Description: Cannot proceed, waiting on something

Name: status: in-progress
Color: #0075ca (blue)
Description: Currently being worked on

Name: status: needs-review
Color: #6f42c1 (purple)
Description: Ready for code review or feedback
```

**Total: 18 labels to start** ✅

---

### Step 2: Create Your First Milestone (15 minutes)

Milestones = Release goals or time-based sprints

#### Go to: Repository → Issues → Milestones → New Milestone

**First Milestone:**

```
Title: v0.1.0-alpha.2
Description: 
Complete frontend authentication and profile management.

Goals:
- Frontend auth flow working
- Profile view and edit pages functional
- Settings pages connected to backend
- User profile API integration complete

Due Date: [2 weeks from now]
```

**Why start here?** This is likely your next deliverable based on the current project status.

---

### Step 3: Create ONE Project Board (20 minutes)

Project Board = Kanban board for visual tracking

#### Go to: Repository → Projects → New Project

**Setup:**

```
Name: Unity Platform Development
Description: Main development tracking board
Template: Basic Kanban
```

**Columns to create:**

1. **📋 Backlog** - Everything not yet started
2. **🎯 Ready** - Refined and ready to work on
3. **🚧 In Progress** - Currently being developed
4. **👀 Review** - Code review or testing
5. **✅ Done** - Completed this week
6. **🗄️ Closed** - Archived (move here weekly)

**Why ONE board?** Start simple. You can add specialized boards later (Infrastructure, Documentation) when you need them.

---

### Step 4: Create Your First Issue (10 minutes)

Let's create a real issue for your next task.

#### Go to: Repository → Issues → New Issue

**Example Issue:**

```
Title: Implement frontend authentication flow

Description:
Complete the authentication flow in the React frontend, connecting to the auth-service backend.

**Current State:**
- Backend auth-service is operational (port 8001)
- Frontend has login/register pages but not connected
- AuthGuard and authStore are set up

**Tasks:**
- [ ] Connect login form to POST /api/v1/auth/login
- [ ] Connect register form to POST /api/v1/auth/register
- [ ] Implement token storage in authStore
- [ ] Add error handling for failed auth
- [ ] Test session persistence (page refresh)
- [ ] Add loading states to forms

**Acceptance Criteria:**
- User can successfully login and register
- Tokens are stored and persisted
- User is redirected after login
- Error messages are user-friendly
- Session survives page refresh

**Related:**
- Depends on auth-service (already complete)
- Blocks profile management work
```

**Labels to add:**

- `type: feature`
- `area: frontend`
- `priority: high`

**Milestone:** v0.1.0-alpha.2

**Assign to:** Yourself

**Add to Project:** Unity Platform Development → Ready column

---

### Step 5: Practice the Workflow (Daily)

Now that you have one issue, practice the workflow:

#### **Daily Routine:**

**Morning (5 minutes):**

1. Open your project board
2. Look at "Ready" column
3. Drag ONE issue to "In Progress"
4. Update issue: Add label `status: in-progress`

**During Work:**

1. Create a branch: `git checkout -b feature/auth-flow-#1`
2. Work on the task
3. Commit referencing issue: `git commit -m "feat(auth): connect login form - #1"`

**When Done:**

1. Create Pull Request (PR)
2. In PR description: `Closes #1` (auto-links and will close issue on merge)
3. Move card to "Review" column
4. Merge PR → Issue auto-closes → Card moves to "Done"

**Friday End of Week (10 minutes):**

1. Move all "Done" cards to "Closed" column
2. Review "In Progress" - anything stuck?
3. Add new issues to "Backlog" for next week

---

## Phase 2: Build Your Backlog (Week 1-2)

Now that you have the system set up, populate it with real work.

### Step 6: Convert Current Status to Issues (1-2 hours)

Look at your `docs/status/current/phase-1-status.md` file.

**For each incomplete task:**

1. Create an issue
2. Add labels (type, area, priority)
3. Add to milestone (if relevant)
4. Add to project board "Backlog" column

**Pro Tip:** Start with just 10-15 issues for your immediate work (next 2-4 weeks). Don't try to create issues for everything at once.

**Example Issues to Create:**

```
Issue #2: Connect profile view page to user-service API
Labels: type: feature, area: frontend, priority: high
Milestone: v0.1.0-alpha.2

Issue #3: Implement profile edit form submission
Labels: type: feature, area: frontend, priority: high  
Milestone: v0.1.0-alpha.2

Issue #4: Add avatar upload functionality
Labels: type: feature, area: frontend, priority: medium
Milestone: v0.1.0-alpha.2

Issue #5: Deploy Norway territory pod
Labels: type: infrastructure, area: infrastructure, priority: medium
Milestone: (none - backlog)

Issue #6: Set up automated database backups
Labels: type: infrastructure, area: infrastructure, priority: high
Milestone: (none - next milestone)
```

---

## Phase 3: Establish Habits (Week 2-4)

### Weekly Routine

**Monday Morning (30 minutes):**

- Review project board
- Move issues from "Backlog" to "Ready" for the week
- Set your focus: Pick 3-5 issues max for the week

**Wednesday Mid-week (15 minutes):**

- Quick board check
- Anything blocked? Add `status: blocked` label and comment why
- Anything need help? Comment or ask

**Friday Afternoon (30 minutes):**

- Close completed issues
- Update issue status
- Plan next week
- Celebrate progress! 🎉

### Daily Habit

**Start of work day (2 minutes):**

- Look at project board "In Progress"
- If empty, move one from "Ready"
- Keep only 1-2 in progress at a time

**During work:**

- Commit with issue reference: `#123`
- Comment on issue with progress updates

**End of day (2 minutes):**

- Update issue if made progress
- Move card if status changed

---

## Common Pitfalls to Avoid

### ❌ Don't Do This

1. **Creating 100 issues at once**
   - Start with 10-15 for immediate work
   - Add more as you need them

2. **Making labels too granular**
   - Resist creating `area: auth-service-models` or `priority: somewhat-high`
   - Keep it simple: 18-20 labels total

3. **Multiple boards immediately**
   - One board is fine for 1-3 people
   - Add more boards when you have >50 active issues

4. **Perfect issue descriptions**
   - Good enough > perfect
   - You can edit and improve later

5. **Assigning everything to milestones**
   - Only assign when you're confident it'll be in that release
   - Backlog issues don't need milestones

### ✅ Do This Instead

1. **Start small, grow organically**
   - This week: Set up labels, milestone, board
   - Next week: Create 5-10 issues
   - Week 3: Establish daily habits
   - Week 4: Refine based on what you learned

2. **Focus on workflow, not perfection**
   - Create issue → Work on it → Close it
   - That's 80% of the value

3. **Use what you have**
   - 1 board, 18 labels, 1 milestone
   - This is enough for months of work

---

## Quick Reference Cheat Sheet

### Creating an Issue

```
1. Click "Issues" → "New Issue"
2. Write clear title: [AREA] What needs to be done
3. Add description with tasks and acceptance criteria
4. Add labels: type, area, priority (minimum 3)
5. Add to milestone if applicable
6. Add to project board
7. Assign to yourself
8. Create issue
```

### Working on an Issue

```
1. Move card to "In Progress" on project board
2. Create branch: git checkout -b type/description-#issue-number
3. Work and commit: git commit -m "type(area): description - #123"
4. Create PR when ready
5. In PR description: "Closes #123"
6. Move card to "Review"
7. Merge PR → Issue auto-closes
```

### Weekly Planning

```
Monday:
- Review backlog
- Move 3-5 issues to "Ready"
- Ensure each has labels and clear description

Friday:
- Archive "Done" issues to "Closed"
- Create new issues for next week
- Update milestone progress
```

---

## Your First Week Plan

### Day 1: Setup

- ✅ Create 18 core labels (30 min)
- ✅ Create first milestone (15 min)
- ✅ Create project board (20 min)

### Day 2: First Issues

- ✅ Create 5 issues for immediate work (1 hour)
- ✅ Add labels, milestone, assign
- ✅ Add to project board

### Day 3-5: Practice

- ✅ Work on 1-2 issues
- ✅ Practice the workflow
- ✅ Make commits with issue references

### Day 6: Review

- ✅ Look at what worked
- ✅ Look at what felt clunky
- ✅ Adjust as needed

### Day 7: Plan Week 2

- ✅ Create next week's issues
- ✅ Review project board
- ✅ Set goals for week 2

---

## Success Metrics

After 2 weeks, you should have:

- ✅ 10-20 issues created
- ✅ 3-5 issues closed
- ✅ Project board actively used daily
- ✅ Git commits referencing issues
- ✅ Clear view of what's next

**That's success!** Everything else is optimization.

---

## Next Steps (After Week 2)

Once you're comfortable with the basics:

1. **Add issue templates** (make creating issues faster)
2. **Add more labels** (as you discover you need them)
3. **Create second milestone** (for next release)
4. **Experiment with automation** (if Forgejo supports it)

But for now: **Just master the basics above** ⬆️

---

## Questions to Ask Yourself Weekly

**End of Week 1:**

- Did I create any issues this week? ✓
- Did I update the project board? ✓
- Did I reference issues in commits? ✓

**End of Week 2:**

- Is my project board accurate? ✓
- Are my labels helpful? ✓
- Do I check the board daily? ✓

**End of Week 4:**

- Has this helped me track work better? ✓
- What should I add next? ✓
- What should I simplify? ✓

---

## Get Help

If something isn't clear:

1. Check Forgejo/Gitea documentation: <https://docs.gitea.com>
2. Look at other open-source projects on GitHub/GitLab for inspiration
3. Remember: Your system should serve you, not the other way around

**Most Important:** Start simple, add complexity only when you feel the pain of not having it.

---

**Last Updated:** November 17, 2025  
**For:** Unity Platform mono-repo setup  
**Next Review:** After 2 weeks of usage
