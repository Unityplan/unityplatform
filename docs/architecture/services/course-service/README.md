# course-service

**Port:** 8010  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Planned - Phase 2  
**Bounded Context:** Learning Management System (LMS)

---

## 📋 Overview

The course-service manages courses, lessons, progress tracking, and certifications.

### **Responsibilities**

- ⏳ Create and manage courses
- ⏳ Lesson content (video, text, quiz)
- ⏳ Track user progress
- ⏳ Issue certificates
- ⏳ Course enrollment
- ⏳ Instructor management

### **Not Responsible For**

- ❌ Video hosting (external service like Vimeo/YouTube)
- ❌ Payment/subscriptions (future: separate payment service)
- ❌ Live classes (future: separate service)

---

## 🗄️ Database Schema (Planned)

```sql
CREATE TABLE territory_{code}.courses (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    instructor_id UUID REFERENCES users(id),
    visibility VARCHAR(20) DEFAULT 'public',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE territory_{code}.lessons (
    id UUID PRIMARY KEY,
    course_id UUID REFERENCES courses(id),
    title VARCHAR(255) NOT NULL,
    content_type VARCHAR(20),  -- video/text/quiz
    content_url VARCHAR(500),
    order_index INT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE territory_{code}.course_enrollments (
    course_id UUID REFERENCES courses(id),
    user_id UUID REFERENCES users(id),
    enrolled_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    progress DECIMAL(5,2) DEFAULT 0.0,  -- 0-100%
    PRIMARY KEY (course_id, user_id)
);

CREATE TABLE territory_{code}.certificates (
    id UUID PRIMARY KEY,
    course_id UUID REFERENCES courses(id),
    user_id UUID REFERENCES users(id),
    issued_at TIMESTAMPTZ DEFAULT NOW(),
    certificate_url VARCHAR(500)
);
```

---

## 🔌 API Endpoints (Planned)

- POST /v1/courses - Create course
- GET /v1/courses - List courses
- GET /v1/courses/{id} - Get course details
- POST /v1/courses/{id}/enroll - Enroll in course
- GET /v1/courses/{id}/lessons - List lessons
- POST /v1/courses/{id}/lessons/{lesson_id}/complete - Mark lesson complete
- GET /v1/courses/{id}/progress - Get user progress
- POST /v1/courses/{id}/certificate - Issue certificate

---

## 📡 NATS Events (Planned)

**Published:**

- course.created
- course.enrolled
- lesson.completed
- course.completed
- certificate.issued

**Subscribed:**

- badge.criteria (award badge for course completion)

---

## 🔮 Holochain Migration (Future)

```rust
#[hdk_entry_helper]
struct Course {
    title: String,
    description: String,
    instructor: AgentPubKey,
    lessons: Vec<Lesson>,
    visibility: CourseVisibility,
}

#[hdk_entry_helper]
struct Certificate {
    course_hash: EntryHash,
    student: AgentPubKey,
    issued_at: Timestamp,
    instructor_signature: Signature,
}
```

**Key Concept:** Certificates as cryptographically signed entries (unforgeable credentials)

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Phase 2 - Not Yet Started  
**Dependencies:** user-service, notification-service, badge-service  
**Future:** Holochain-based verifiable credentials
