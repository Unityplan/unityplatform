#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=1
PRIORITY_MEDIUM_ID=2
TYPE_FEATURE_ID=5
TYPE_INFRASTRUCTURE_ID=8
AREA_INFRASTRUCTURE_ID=14

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
        echo "${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}"
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

echo "=========================================="
echo "Creating Stage 7 open issues (Course Service)..."
echo "=========================================="
echo ""

# Stage 7: Course Service (LMS) - 13 tasks total

create_open_issue \
    "Stage 7.1: Create course-service scaffolding" \
    "## Description
Create the course-service Rust crate with standard structure.

## Acceptance Criteria
- [ ] course-service crate created in services/ directory
- [ ] Standard service structure (handlers, models, services)
- [ ] Cargo.toml with dependencies (actix-web, sqlx, shared-lib)
- [ ] main.rs with basic server setup
- [ ] All 5 middleware components integrated
- [ ] Port 8015 configured

## Dependencies
- Stage 6: Badge System (completed)

## Technical Notes
- Follow same patterns as auth-service, user-service
- Integrate with badge-service for course completion rewards
- Use AppConfig for configuration
- Include health, ready, metrics endpoints from start" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}"

create_open_issue \
    "Stage 7.2: Course database schema migration" \
    "## Description
Create database migration for LMS course tables.

## Tables Required
- **courses**: id, slug, title, description, level, status, created_at, updated_at
- **course_lessons**: id, course_id, order, title, content_type, content, duration
- **course_enrollments**: user_id, course_id, enrolled_at, progress, completed_at
- **course_progress**: user_id, lesson_id, completed_at, score
- **course_quizzes**: id, lesson_id, title, passing_score
- **course_quiz_questions**: id, quiz_id, question_text, question_type, correct_answer
- **course_quiz_attempts**: user_id, quiz_id, attempt_number, score, passed, submitted_at

## Acceptance Criteria
- [ ] Migration created (20251117000009_course_system.sql)
- [ ] All tables created in territory schema
- [ ] Proper indexes on foreign keys
- [ ] Timestamps with timezone
- [ ] Migration applied successfully to territory_dk

## Technical Notes
- Territory-scoped (courses per territory)
- Support for video, text, markdown lesson types
- Quiz types: multiple_choice, true_false, short_answer
- Track completion percentage and scores" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}"

create_open_issue \
    "Stage 7.3: Seed Code of Conduct training course" \
    "## Description
Create the first course: Code of Conduct training required for all users.

## Course Structure
- Course: Platform Code of Conduct
- Lessons (3-5 lessons):
  1. Welcome and Community Values
  2. Respectful Communication Guidelines
  3. Privacy and Data Sovereignty
  4. Reporting and Moderation
  5. Your Rights and Responsibilities
- Quiz: 10 questions covering key points, 80% passing score

## Acceptance Criteria
- [ ] Seed script created for Code of Conduct course
- [ ] All lessons written with clear content
- [ ] Quiz questions created with correct answers
- [ ] Course marked as required for new users
- [ ] Badge integration: completing course grants Code of Conduct badge

## Technical Notes
- Use markdown for lesson content
- Estimated completion time: 15-20 minutes
- Quiz must be passed to complete course
- Course completion triggers badge award via NATS" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_INFRASTRUCTURE_ID}"

create_open_issue \
    "Stage 7.4: Implement course endpoints (7 endpoints)" \
    "## Description
Implement all course management and learning endpoints.

## Endpoints
1. **GET /api/v1/courses** - List published courses (filterable by level, required)
2. **GET /api/v1/courses/{course_id}** - Get course details with lessons
3. **POST /api/v1/courses/{course_id}/enroll** - Enroll in course
4. **GET /api/v1/courses/{course_id}/lessons/{lesson_id}** - Get lesson content
5. **POST /api/v1/courses/{course_id}/lessons/{lesson_id}/complete** - Mark lesson complete
6. **POST /api/v1/quizzes/{quiz_id}/submit** - Submit quiz answers and get score
7. **GET /api/v1/users/me/enrollments** - Get my enrolled courses with progress

## Acceptance Criteria
- [ ] All 7 endpoints implemented with validation
- [ ] JWT authentication required on all endpoints
- [ ] Progress tracking working (lessons completed, course completion %)
- [ ] Quiz grading automated
- [ ] NATS event published on course completion: `course.completed`
- [ ] Badge awarded automatically via NATS when course marked required
- [ ] OpenAPI/Swagger documentation

## Technical Notes
- Use JwtAuth middleware for authentication
- Validate quiz answers server-side (do not trust client)
- Calculate progress percentage: completed lessons / total lessons
- Mark course complete when all lessons + passing quiz
- Publish event with user ID, course ID, and completion timestamp" \
    "${PRIORITY_HIGH_ID},${TYPE_FEATURE_ID},${AREA_INFRASTRUCTURE_ID}"

create_open_issue \
    "Stage 7.5: Course-service architecture compliance" \
    "## Description
Ensure course-service follows all architecture standards.

## Requirements
- [ ] camelCase JSON serialization on all models
- [ ] AppConfig integration (database_url, nats_url)
- [ ] NATS client initialized and connected
- [ ] Health endpoint: GET /api/v1/health
- [ ] Ready endpoint: GET /api/v1/ready (with DB check)
- [ ] Metrics endpoint: GET /api/v1/metrics (Prometheus format)
- [ ] MetricsCollector integrated with automatic HTTP tracking
- [ ] Logging with tracing_subscriber
- [ ] RUST_LOG environment variable configured

## Acceptance Criteria
- [ ] All models verified with camelCase
- [ ] All observability endpoints working
- [ ] NATS events publishing successfully
- [ ] Service added to dev scripts (start, stop, status)
- [ ] Service compiles in release mode
- [ ] All endpoints tested manually

## Technical Notes
- Follow patterns from auth-service, user-service, badge-service
- Use same middleware stack
- Ensure NATS connection for event publishing" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_INFRASTRUCTURE_ID}"

create_open_issue \
    "Stage 7.6: Course-service testing and documentation" \
    "## Description
Comprehensive testing and documentation for course service.

## Testing
- [ ] Manual endpoint testing (all 7 endpoints)
- [ ] Course enrollment flow tested
- [ ] Lesson completion tracking verified
- [ ] Quiz grading tested (pass/fail scenarios)
- [ ] Course completion triggers badge award
- [ ] NATS event publishing verified

## Documentation
- [ ] README.md with service overview
- [ ] API.md with endpoint documentation
- [ ] DATABASE.md with schema documentation
- [ ] Example course creation documented

## Acceptance Criteria
- [ ] All endpoints verified working
- [ ] Edge cases tested (re-enrollment, quiz retakes)
- [ ] Documentation complete and clear
- [ ] Ready for frontend integration

## Technical Notes
- Test with Code of Conduct course
- Verify badge integration with badge-service
- Document event payloads for NATS" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FEATURE_ID},${AREA_INFRASTRUCTURE_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 7 issues created!"
echo "=========================================="
echo ""
echo "Summary: Created 6 open issues for Stage 7 (Course Service / LMS)"
echo "These issues cover all 13 tasks organized into logical implementation groups."
echo ""
echo "View all issues: ${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues?state=open&milestone=1"
