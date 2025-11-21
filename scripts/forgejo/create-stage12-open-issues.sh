#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=1
PRIORITY_MEDIUM_ID=2
TYPE_FEATURE_ID=5
AREA_FRONTEND_APP_ID=14

# Milestone ID
MILESTONE_ID=1  # v0.1.0-alpha.2

# Function to create open issue
create_open_issue() {
    local title="$1"
    local body="$2"
    local label_ids="$3"
    
    # Convert label IDs to JSON array of numbers
    labels_json=$(printf '%s\n' ${label_ids//,/ } | jq -R 'tonumber' | jq -s .)
    
    # Create the issue (open by default)
    response=$(curl -s -X POST \
        -H "Authorization: token ${FORGEJO_TOKEN}" \
        -H "Content-Type: application/json" \
        "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues" \
        -d "{
            \"title\": \"${title}\",
            \"body\": \"${body}\",
            \"labels\": ${labels_json},
            \"milestone\": ${MILESTONE_ID}
        }")
    
    issue_number=$(echo "$response" | jq -r '.number')
    
    if [ "$issue_number" != "null" ]; then
        echo "✅ Created issue #${issue_number}: ${title}"
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

echo "=========================================="
echo "Creating Stage 12 open issues (Frontend Course & Forum UI)..."
echo "=========================================="
echo ""

# Stage 12: Frontend Course & Forum UI - 10 tasks total

create_open_issue \
    "Stage 12.1: Create course catalog page" \
    "## Description
Create the course catalog page showing all available courses with filtering and search.

## Tasks
- Create /courses route and page component
- Fetch courses from course-service API
- Display course cards with thumbnail, title, description, duration
- Add search functionality
- Add category filtering
- Add difficulty level filtering (beginner, intermediate, advanced)
- Add sorting (newest, popular, alphabetical)
- Implement pagination
- Add responsive grid layout

## Acceptance Criteria
- [ ] Course catalog displays all available courses
- [ ] Search filters courses by title/description
- [ ] Category filter working
- [ ] Difficulty filter working
- [ ] Sorting options functional
- [ ] Pagination working
- [ ] Responsive on mobile and desktop
- [ ] Loading states implemented

## Technical Notes
- Use TanStack Query for data fetching
- Course card component with shadcn Card
- Use shadcn Input for search
- Use shadcn Select for filters
- Cache course list (5 min stale time)

## API Endpoint
- GET /api/v1/courses?search=&category=&difficulty=&page=&limit=

## Dependencies
- Stage 11: Translation Service
- course-service API (Stage 7)" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.2: Create course detail page" \
    "## Description
Create the course detail page showing course information, lessons, and enrollment.

## Tasks
- Create /courses/{slug} route and page component
- Fetch course details from API
- Display course header (title, description, instructor, duration)
- Show lesson list with progress indicators
- Add enroll button (if not enrolled)
- Show completion percentage (if enrolled)
- Display course requirements and learning outcomes
- Add share button
- Implement breadcrumb navigation

## Acceptance Criteria
- [ ] Course details displayed correctly
- [ ] Lesson list showing all lessons
- [ ] Enroll button functional
- [ ] Progress tracking visible for enrolled users
- [ ] Breadcrumbs working
- [ ] Responsive layout
- [ ] Loading and error states

## Technical Notes
- Use TanStack Query with course slug
- Show progress bar for enrolled courses
- Lessons clickable if course is enrolled
- Use shadcn Progress for completion
- Use shadcn Button for enroll action

## API Endpoints
- GET /api/v1/courses/{slug}
- POST /api/v1/courses/{course_id}/enroll

## Dependencies
- Stage 12.1: course catalog page" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.3: Create lesson viewer page" \
    "## Description
Create the lesson viewer for consuming course content with navigation.

## Tasks
- Create /courses/{slug}/lessons/{lesson_id} route
- Fetch lesson content from API
- Render Markdown content with syntax highlighting
- Display video if present (video URL)
- Add previous/next lesson navigation
- Add mark as complete button
- Show lesson progress in sidebar
- Add notes section (optional)
- Implement fullscreen mode

## Acceptance Criteria
- [ ] Lesson content displays correctly
- [ ] Markdown rendered with proper styling
- [ ] Video player functional (if video present)
- [ ] Navigation between lessons working
- [ ] Mark as complete updates progress
- [ ] Sidebar shows course outline
- [ ] Fullscreen toggle working
- [ ] Responsive layout

## Technical Notes
- Use react-markdown for content
- Use syntax highlighting (react-syntax-highlighter)
- Track lesson completion
- Auto-save progress
- Use shadcn Separator for layout

## API Endpoints
- GET /api/v1/lessons/{lesson_id}
- POST /api/v1/lessons/{lesson_id}/complete

## Dependencies
- Stage 12.2: course detail page" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.4: Create quiz page" \
    "## Description
Create quiz page for course assessments with multiple choice questions.

## Tasks
- Create /courses/{slug}/quiz route
- Fetch quiz questions from API
- Display questions one at a time
- Add multiple choice answer selection
- Implement submit quiz functionality
- Show quiz results with score
- Display correct/incorrect answers
- Add retry option (if failed)
- Track quiz attempts

## Acceptance Criteria
- [ ] Quiz questions displayed correctly
- [ ] Answer selection working
- [ ] Submit quiz functionality working
- [ ] Results page shows score and feedback
- [ ] Correct answers highlighted
- [ ] Retry option available
- [ ] Progress indicator showing current question
- [ ] Responsive layout

## Technical Notes
- Use form state management (react-hook-form)
- Calculate score on frontend and verify on backend
- Passing score: 70%
- Use shadcn RadioGroup for answers
- Use shadcn Progress for quiz progress

## API Endpoints
- GET /api/v1/courses/{course_id}/quiz
- POST /api/v1/quizzes/{quiz_id}/submit

## Dependencies
- Stage 12.3: lesson viewer page" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.5: Create my learning page" \
    "## Description
Create dashboard showing user's enrolled courses and learning progress.

## Tasks
- Create /learning route
- Fetch user's enrolled courses
- Display course cards with progress
- Show recently viewed lessons
- Add continue learning buttons
- Display certificates earned
- Show learning stats (time spent, courses completed)
- Add filter by status (in-progress, completed)

## Acceptance Criteria
- [ ] Enrolled courses displayed with progress
- [ ] Recently viewed lessons shown
- [ ] Continue learning quick access working
- [ ] Certificates displayed
- [ ] Learning stats accurate
- [ ] Filter by status functional
- [ ] Empty state for no courses
- [ ] Responsive layout

## Technical Notes
- Use TanStack Query for enrolled courses
- Cache learning data (1 min stale time)
- Group by status (not started, in progress, completed)
- Use shadcn Tabs for status filter
- Use shadcn Badge for completion status

## API Endpoints
- GET /api/v1/users/me/courses
- GET /api/v1/users/me/learning-stats

## Dependencies
- Stage 12.4: quiz page" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.6: Create forum category list page" \
    "## Description
Create forum homepage showing all forum categories.

## Tasks
- Create /forum route
- Fetch forum categories from API
- Display category cards with metadata
- Show topic count and post count
- Display last activity timestamp
- Add create topic button
- Implement breadcrumb navigation
- Add search functionality

## Acceptance Criteria
- [ ] Categories displayed in organized layout
- [ ] Metadata (counts, last activity) visible
- [ ] Create topic button navigates correctly
- [ ] Search filters categories
- [ ] Breadcrumbs working
- [ ] Responsive layout
- [ ] Loading states implemented

## Technical Notes
- Use TanStack Query for categories
- Cache category list (5 min stale time)
- Use shadcn Card for category display
- Show relative timestamps (2 hours ago)
- Use shadcn Input for search

## API Endpoints
- GET /api/v1/forum/categories

## Dependencies
- Stage 12.5: my learning page
- forum-service API (Stage 10)" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.7: Create forum topic list page" \
    "## Description
Create page showing topics within a forum category.

## Tasks
- Create /forum/categories/{slug} route
- Fetch topics for category
- Display topic rows with title, author, replies, views, last activity
- Add pagination
- Show pinned topics at top
- Show locked icon for locked topics
- Add create new topic button
- Implement sorting (latest, most replies, most views)

## Acceptance Criteria
- [ ] Topics displayed in table/list format
- [ ] Pinned topics shown first
- [ ] Locked topics indicated
- [ ] Sorting options working
- [ ] Pagination functional
- [ ] Create topic button working
- [ ] Responsive layout (cards on mobile, table on desktop)
- [ ] Empty state for no topics

## Technical Notes
- Use TanStack Query for topics
- Cache topic list (1 min stale time)
- Use shadcn Table for desktop
- Use shadcn Card for mobile
- Show user avatars

## API Endpoints
- GET /api/v1/forum/categories/{slug}/topics?sort=&page=&limit=

## Dependencies
- Stage 12.6: forum category list" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.8: Create forum topic view with posts page" \
    "## Description
Create page displaying a forum topic with all posts and replies.

## Tasks
- Create /forum/topics/{slug} route
- Fetch topic with posts
- Display topic header (title, author, created date)
- Show posts in chronological order
- Add reply form at bottom
- Show user info with each post (avatar, username, badge)
- Add post reactions (like, helpful)
- Implement edit/delete for own posts
- Show pagination for posts

## Acceptance Criteria
- [ ] Topic and posts displayed correctly
- [ ] Reply form functional
- [ ] Reactions working
- [ ] Edit/delete for own posts
- [ ] User info displayed with posts
- [ ] Pagination working
- [ ] Markdown rendering for post content
- [ ] Responsive layout

## Technical Notes
- Use TanStack Query for topic and posts
- Use react-markdown for post content
- Real-time updates via polling (30 sec)
- Use shadcn Textarea for reply
- Use shadcn Avatar for user images
- Show relative timestamps

## API Endpoints
- GET /api/v1/forum/topics/{slug}
- POST /api/v1/forum/topics/{topic_id}/posts
- PUT /api/v1/forum/posts/{post_id}
- DELETE /api/v1/forum/posts/{post_id}
- POST /api/v1/forum/posts/{post_id}/reactions

## Dependencies
- Stage 12.7: topic list page" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.9: Create new topic form" \
    "## Description
Create form for creating new forum topics.

## Tasks
- Create /forum/topics/new route
- Add category selection dropdown
- Add title input with validation
- Add content editor (Markdown)
- Add preview functionality
- Implement draft autosave
- Add submit button
- Show validation errors
- Redirect to topic after creation

## Acceptance Criteria
- [ ] Category selection working
- [ ] Title validation (min 10, max 200 chars)
- [ ] Content editor functional
- [ ] Preview shows rendered Markdown
- [ ] Draft autosave every 30 seconds
- [ ] Submit creates topic
- [ ] Validation errors displayed
- [ ] Redirects to new topic

## Technical Notes
- Use react-hook-form with zod validation
- Use Markdown editor (textarea + preview)
- Store draft in localStorage
- Use shadcn Select for category
- Use shadcn Tabs for edit/preview
- Clear draft on successful submit

## API Endpoints
- POST /api/v1/forum/topics

## Dependencies
- Stage 12.8: topic view page" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

create_open_issue \
    "Stage 12.10: Create moderation dashboard" \
    "## Description
Create moderation dashboard for forum moderators to manage content.

## Tasks
- Create /forum/moderation route (moderator only)
- Display flagged posts queue
- Show post content and reason for flag
- Add approve/remove actions
- Implement issue strike functionality
- Show user strike history
- Add ban user functionality
- Display moderation activity log
- Add filters (pending, resolved)

## Acceptance Criteria
- [ ] Only accessible to moderators
- [ ] Flagged posts displayed in queue
- [ ] Approve/remove actions working
- [ ] Strike system functional
- [ ] Ban functionality working
- [ ] Activity log showing recent actions
- [ ] Filters working
- [ ] Responsive layout

## Technical Notes
- Check user role (moderator) before rendering
- Use TanStack Query for flagged posts
- Poll for new flags (1 min interval)
- Use shadcn Table for desktop view
- Use shadcn Badge for flag status
- Confirm dialogs for ban actions

## API Endpoints
- GET /api/v1/forum/moderation/queue
- POST /api/v1/forum/moderation/approve/{post_id}
- POST /api/v1/forum/moderation/remove/{post_id}
- POST /api/v1/forum/moderation/strike
- POST /api/v1/forum/moderation/ban

## Dependencies
- Stage 12.9: create topic form" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_FRONTEND_APP_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 12 issues created!"
echo "=========================================="
echo ""
echo "Summary: Created 10 open issues for Stage 12 (Frontend Course & Forum UI)"
echo ""
