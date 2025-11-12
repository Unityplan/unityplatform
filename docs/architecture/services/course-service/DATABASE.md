# course-service Database Schema

**Service:** course-service  
**Port:** 8010  
**Database:** Global + Territory schemas

---

## Schema Distribution

### Global Schema (Course Catalog)

**Table:** `global.courses` (Future)  
**Purpose:** Global course catalog available to all territories

```sql
CREATE TABLE global.courses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    
    instructor_id UUID,
    instructor_territory VARCHAR(10),
    
    difficulty VARCHAR(20) NOT NULL,
    duration_hours INT,
    
    is_published BOOLEAN NOT NULL DEFAULT false,
    is_global BOOLEAN NOT NULL DEFAULT false,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (difficulty IN ('beginner', 'intermediate', 'advanced'))
);
```

**Table:** `global.course_lessons` (Future)

```sql
CREATE TABLE global.course_lessons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    course_id UUID NOT NULL REFERENCES global.courses(id) ON DELETE CASCADE,
    
    title VARCHAR(255) NOT NULL,
    content TEXT,
    
    lesson_order INT NOT NULL,
    
    content_type VARCHAR(20) NOT NULL,
    
    duration_minutes INT,
    
    CHECK (content_type IN ('video', 'text', 'quiz', 'assignment'))
);
```

**Why Global?** Courses are educational content - shared across all territories (like a library).

---

### Territory Schema (Enrollments)

**Table:** `territory_{code}.course_enrollments` (Future)

```sql
CREATE TABLE territory_{code}.course_enrollments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    course_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    enrolled_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    
    progress_percentage DECIMAL(5,2) NOT NULL DEFAULT 0.00,
    
    UNIQUE (course_id, user_id)
);
```

**Table:** `territory_{code}.course_progress` (Future)

```sql
CREATE TABLE territory_{code}.course_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    enrollment_id UUID NOT NULL REFERENCES territory_{code}.course_enrollments(id) ON DELETE CASCADE,
    lesson_id UUID NOT NULL,
    
    completed BOOLEAN NOT NULL DEFAULT false,
    completed_at TIMESTAMPTZ,
    
    UNIQUE (enrollment_id, lesson_id)
);
```

**Why Territory?** Enrollment and progress are personal data - stay in user's pod.

---

## Multi-Pod Strategy

**Course Content:** Global (shared knowledge)  
**User Enrollment:** Territory (personal progress)

**Example:**

- "Introduction to Rust" course → global.courses
- Alice (Denmark) enrolls → territory_dk.course_enrollments
- Alice completes Lesson 3 → territory_dk.course_progress

## Cross-Territory Access

Users from any territory can enroll in global courses. Progress tracked locally.

## NATS Events

**Publishes:** `course.enrolled`, `lesson.completed`, `course.completed`

---

**Last Updated:** November 12, 2025
