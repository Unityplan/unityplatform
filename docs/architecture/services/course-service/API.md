# course-service API Endpoints

**Base URL:** `http://localhost:8010`  
**Version:** v1  
**Status:** 📋 Planned (Future Phase - LMS)

---

## Learning Management System

### 1. List Courses

**Endpoint:** `GET /api/v1/courses`  
**Status:** 📋 Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "courses": [
      {
        "id": "uuid",
        "title": "Introduction to Rust",
        "description": "Learn Rust from scratch",
        "instructor_id": "uuid",
        "duration_hours": 40,
        "lesson_count": 12,
        "enrolled_count": 150,
        "difficulty": "beginner",
        "is_enrolled": false
      }
    ]
  }
}
```

---

### 2. Get Course Details

**Endpoint:** `GET /api/v1/courses/{id}`  
**Status:** 📋 Planned

---

### 3. Enroll in Course

**Endpoint:** `POST /api/v1/courses/{id}/enroll`  
**Status:** 📋 Planned

---

### 4. Get Lessons

**Endpoint:** `GET /api/v1/courses/{id}/lessons`  
**Status:** 📋 Planned

---

### 5. Complete Lesson

**Endpoint:** `POST /api/v1/courses/{course_id}/lessons/{lesson_id}/complete`  
**Status:** 📋 Planned

---

### 6. Get Progress

**Endpoint:** `GET /api/v1/courses/{id}/progress`  
**Status:** 📋 Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "course_id": "uuid",
    "completed_lessons": 7,
    "total_lessons": 12,
    "progress_percentage": 58.3,
    "started_at": "2025-11-01T10:00:00Z",
    "last_accessed": "2025-11-12T10:00:00Z"
  }
}
```

---

### 7. Issue Certificate

**Endpoint:** `POST /api/v1/courses/{id}/certificate`  
**Status:** 📋 Planned  
**Note:** Cryptographically signed in Holochain

---

## Content Types

- Video lessons
- Text content (Markdown)
- Quizzes (multiple choice)
- Assignments (code submission)
- Live sessions (integration with event-service)

---

**Last Updated:** November 12, 2025  
**Implementation Status:** Future phase (LMS system)
