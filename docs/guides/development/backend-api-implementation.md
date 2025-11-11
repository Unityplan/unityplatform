# Backend API Implementation Guide

**Status:** Planning Phase  
**Created:** November 11, 2025  
**Purpose:** Define REST API endpoints for all microservices  
**Version:** 0.1.0-alpha.1

---

## Table of Contents

1. [Overview](#overview)
2. [API Design Principles](#api-design-principles)
3. [Common Patterns](#common-patterns)
4. [Authentication Service API](#authentication-service-api)
5. [User Service API](#user-service-api)
6. [Badge Service API](#badge-service-api)
7. [Community Service API](#community-service-api)
8. [Course Service API](#course-service-api)
9. [Forum Service API](#forum-service-api)
10. [Territory Service API](#territory-service-api)
11. [Notification Service API](#notification-service-api)
12. [Translation Service API](#translation-service-api)
13. [Event Service API](#event-service-api)
14. [Matrix Gateway API](#matrix-gateway-api)
15. [Error Handling](#error-handling)
16. [Rate Limiting](#rate-limiting)
17. [Versioning Strategy](#versioning-strategy)

---

## Overview

This document defines the REST API specifications for all UnityPlan backend microservices. Each service operates independently and communicates via:

- **Synchronous HTTP/REST**: Client-to-service, service-to-service when immediate response needed
- **Asynchronous NATS**: Inter-service events, background processing
- **WebSocket**: Real-time notifications, chat, live updates

### Microservices Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         API Gateway (Traefik)                   │
│                    https://api.unityplan.org                    │
└────────────────────────────┬────────────────────────────────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ Auth Service │    │ User Service │    │Badge Service │
│   :8001      │    │   :8002      │    │   :8003      │
└──────────────┘    └──────────────┘    └──────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│Course Service│    │Forum Service │    │ Community    │
│   :8004      │    │   :8005      │    │   Service    │
│              │    │              │    │   :8010      │
└──────────────┘    └──────────────┘    └──────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│Matrix Gateway│    │ Translation  │    │ Territory    │
│   :8006      │    │   Service    │    │   Service    │
│              │    │   :8007      │    │   :8008      │
└──────────────┘    └──────────────┘    └──────────────┘

┌──────────────┐    ┌──────────────┐
│ Notification │    │Event Service │
│   Service    │    │   :8011      │
│   :8009      │    │              │
└──────────────┘    └──────────────┘
```

---

## API Design Principles

### RESTful Standards

1. **Resource-Oriented**: URLs represent resources, not actions
2. **HTTP Verbs**: GET, POST, PUT, PATCH, DELETE used semantically
3. **Status Codes**: Proper HTTP status codes for all responses
4. **JSON Format**: Request/response bodies in JSON (application/json)
5. **Idempotency**: PUT, PATCH, DELETE are idempotent

### URL Structure

```
https://api.unityplan.org/v1/{service}/{resource}/{id}/{sub-resource}

Examples:
GET    /v1/users/123e4567-e89b-12d3-a456-426614174000
POST   /v1/courses
GET    /v1/badges/definitions
PUT    /v1/communities/abc-123/members/user-456
DELETE /v1/forums/thread-789/comments/comment-123
```

### Territory Context

All requests include territory context via header:

```http
X-Territory-ID: dk
X-Territory-ID: navajo-fn-us
X-Territory-ID: sami-fn-no
```

Services resolve the appropriate database schema based on this header.

### Authentication

All authenticated endpoints require JWT bearer token:

```http
Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Pagination

```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total_pages": 5,
    "total_count": 97,
    "has_next": true,
    "has_prev": false
  }
}
```

Query parameters:
- `page`: Page number (default: 1)
- `per_page`: Items per page (default: 20, max: 100)

### Filtering & Sorting

```
GET /v1/courses?status=active&sort=created_at:desc&category=technology
```

### Response Envelope

All responses wrapped in consistent envelope:

```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "meta": {
    "timestamp": "2025-11-11T14:30:00Z",
    "request_id": "req_abc123"
  }
}
```

Error response:

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": {
      "email": ["Must be a valid email address"]
    }
  },
  "meta": {
    "timestamp": "2025-11-11T14:30:00Z",
    "request_id": "req_abc123"
  }
}
```

---

## Common Patterns

### Health Check (All Services)

```
GET /{service}/health
```

**Response:**
```json
{
  "status": "healthy",
  "service": "auth-service",
  "version": "0.1.0-alpha.1",
  "timestamp": "2025-11-11T14:30:00Z",
  "dependencies": {
    "database": "healthy",
    "nats": "healthy",
    "redis": "healthy"
  }
}
```

### Metrics (All Services)

```
GET /{service}/metrics
```

Prometheus-compatible metrics endpoint.

---

## Authentication Service API

**Base URL:** `/v1/auth`  
**Port:** 8001  
**Database:** `global.users`

### Endpoints

#### Register New User

```
POST /v1/auth/register
```

**Request:**
```json
{
  "invitation_token": "inv_dk_bootstrap_abc123",
  "username": "john_doe",
  "email": "john@example.com",
  "password": "SecureP@ssw0rd!",
  "display_name": "John Doe",
  "preferred_language": "en",
  "accept_terms": true,
  "accept_privacy": true,
  "accept_code_of_conduct": true
}
```

**Response:** `201 Created`
```json
{
  "success": true,
  "data": {
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "john_doe",
    "email": "john@example.com",
    "home_territory_id": "dk",
    "created_at": "2025-11-11T14:30:00Z"
  }
}
```

**Errors:**
- `400`: Invalid invitation token
- `409`: Username or email already exists
- `422`: Validation errors

#### Login

```
POST /v1/auth/login
```

**Request:**
```json
{
  "username": "john_doe",
  "password": "SecureP@ssw0rd!"
}
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "refresh_abc123",
    "token_type": "Bearer",
    "expires_in": 3600,
    "user": {
      "user_id": "123e4567-e89b-12d3-a456-426614174000",
      "username": "john_doe",
      "display_name": "John Doe",
      "home_territory_id": "dk"
    }
  }
}
```

**Errors:**
- `401`: Invalid credentials
- `403`: Account suspended/deleted
- `429`: Too many login attempts

#### Refresh Token

```
POST /v1/auth/refresh
```

**Request:**
```json
{
  "refresh_token": "refresh_abc123"
}
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "refresh_def456",
    "token_type": "Bearer",
    "expires_in": 3600
  }
}
```

#### Logout

```
POST /v1/auth/logout
```

**Headers:** `Authorization: Bearer {token}`

**Response:** `204 No Content`

#### Validate Token

```
GET /v1/auth/validate
```

**Headers:** `Authorization: Bearer {token}`

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "valid": true,
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "expires_at": "2025-11-11T15:30:00Z"
  }
}
```

#### OIDC Integration (Future)

```
GET /v1/auth/oidc/providers
POST /v1/auth/oidc/login/{provider}
GET /v1/auth/oidc/callback/{provider}
```

### NATS Events Published

- `auth.user.registered` - New user registration
- `auth.user.logged_in` - Successful login
- `auth.user.logged_out` - User logout
- `auth.token.refreshed` - Token refresh
- `auth.session.expired` - Session expiration

---

## User Service API

**Base URL:** `/v1/users`  
**Port:** 8002  
**Database:** `territory_{id}` schemas

### Endpoints

#### Get Current User Profile

```
GET /v1/users/me
```

**Headers:** `Authorization: Bearer {token}`

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "john_doe",
    "display_name": "John Doe",
    "email": "john@example.com",
    "avatar_url": "ipfs://Qm...",
    "bio": "Software developer passionate about decentralization",
    "website": "https://johndoe.dev",
    "location": "Copenhagen, Denmark",
    "home_territory_id": "dk",
    "preferred_language": "en",
    "created_at": "2025-11-11T14:30:00Z",
    "profile_links": [
      {
        "platform": "github",
        "url": "https://github.com/johndoe",
        "verified": true
      }
    ],
    "language_proficiencies": [
      {
        "language_code": "en",
        "proficiency_level": "native"
      },
      {
        "language_code": "da",
        "proficiency_level": "fluent"
      }
    ],
    "privacy_settings": {
      "profile_visibility": "public",
      "email_visibility": "private",
      "badge_visibility": "public"
    }
  }
}
```

#### Get User By ID

```
GET /v1/users/{user_id}
```

**Response:** `200 OK`

Returns public profile data based on privacy settings.

#### Update Profile

```
PATCH /v1/users/me
```

**Headers:** `Authorization: Bearer {token}`

**Request:**
```json
{
  "display_name": "John Doe",
  "bio": "Updated bio",
  "website": "https://johndoe.dev",
  "location": "Copenhagen, Denmark"
}
```

**Response:** `200 OK`

#### Update Avatar

```
POST /v1/users/me/avatar
```

**Headers:** 
- `Authorization: Bearer {token}`
- `Content-Type: multipart/form-data`

**Request:** File upload

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "avatar_url": "ipfs://QmNewHash123",
    "file_size": 125648,
    "mime_type": "image/jpeg"
  }
}
```

#### Update Privacy Settings

```
PATCH /v1/users/me/privacy
```

**Request:**
```json
{
  "profile_visibility": "public",
  "email_visibility": "private",
  "badge_visibility": "public",
  "activity_visibility": "connections_only"
}
```

**Response:** `200 OK`

#### Update Notification Settings

```
PATCH /v1/users/me/notifications
```

**Request:**
```json
{
  "email_notifications": {
    "badge_expiry": true,
    "role_election": true,
    "course_updates": false,
    "forum_replies": true
  },
  "push_notifications": {
    "enabled": true,
    "badge_expiry": true,
    "messages": true
  },
  "in_app_notifications": {
    "enabled": true
  }
}
```

**Response:** `200 OK`

#### Add Profile Link

```
POST /v1/users/me/links
```

**Request:**
```json
{
  "platform": "github",
  "url": "https://github.com/johndoe"
}
```

**Response:** `201 Created`

#### Delete Profile Link

```
DELETE /v1/users/me/links/{link_id}
```

**Response:** `204 No Content`

#### Add Language Proficiency

```
POST /v1/users/me/languages
```

**Request:**
```json
{
  "language_code": "es",
  "proficiency_level": "intermediate"
}
```

**Response:** `201 Created`

#### User Connections (Follow/Friend/Block)

```
GET /v1/users/me/connections
GET /v1/users/me/connections?type=following
POST /v1/users/me/connections
DELETE /v1/users/me/connections/{user_id}
```

**Create Connection Request:**
```json
{
  "target_user_id": "user-456",
  "connection_type": "follow"
}
```

**Response:** `201 Created`

#### Block User

```
POST /v1/users/me/blocks
```

**Request:**
```json
{
  "blocked_user_id": "user-789",
  "reason": "spam"
}
```

#### Export User Data (GDPR)

```
GET /v1/users/me/export
```

**Response:** `202 Accepted`

Triggers async export job. Returns download URL when ready.

#### Delete Account

```
DELETE /v1/users/me
```

**Request:**
```json
{
  "password": "SecureP@ssw0rd!",
  "confirm": true,
  "reason": "privacy_concerns"
}
```

**Response:** `202 Accepted`

Triggers soft delete (14-day grace period).

### NATS Events Published

- `user.profile.updated`
- `user.privacy.changed`
- `user.avatar.updated`
- `user.connection.created`
- `user.blocked`
- `user.data.exported`
- `user.deleted`

---

## Badge Service API

**Base URL:** `/v1/badges`  
**Port:** 8003  
**Database:** `global.badge_definitions` + `territory_{id}.badge_awards`

### Endpoints

#### List Badge Definitions

```
GET /v1/badges/definitions
```

**Query Parameters:**
- `scope`: `global`, `territory`, `community`
- `category`: Filter by category
- `required_for_role`: Role code

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "badge_id": "badge-123",
      "code": "code_of_conduct",
      "name": "Code of Conduct Agreement",
      "description": "Signed and accepted the platform Code of Conduct",
      "icon_url": "ipfs://Qm...",
      "color": "#3B82F6",
      "scope": "global",
      "scope_id": null,
      "category": "governance",
      "is_renewable": true,
      "renewal_period_days": 365,
      "requires_approval": false,
      "prerequisites": [],
      "created_at": "2025-01-01T00:00:00Z"
    }
  ]
}
```

#### Get Badge Definition

```
GET /v1/badges/definitions/{badge_id}
```

#### Get User's Badges

```
GET /v1/users/{user_id}/badges
GET /v1/users/me/badges
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "award_id": "award-456",
      "badge": {
        "badge_id": "badge-123",
        "code": "code_of_conduct",
        "name": "Code of Conduct Agreement",
        "icon_url": "ipfs://Qm..."
      },
      "awarded_at": "2025-11-11T14:30:00Z",
      "awarded_by_user_id": "system",
      "expires_at": "2026-11-11T14:30:00Z",
      "is_expired": false,
      "days_until_expiry": 365,
      "renewal_status": "active"
    }
  ]
}
```

#### Award Badge (Admin/System)

```
POST /v1/badges/awards
```

**Headers:** `Authorization: Bearer {admin_token}`

**Request:**
```json
{
  "user_id": "user-123",
  "badge_id": "badge-456",
  "awarded_by_user_id": "admin-789",
  "reason": "Completed course XYZ",
  "metadata": {
    "course_id": "course-abc",
    "score": 95
  }
}
```

**Response:** `201 Created`

#### Check Badge Access

```
GET /v1/badges/check-access
```

**Query Parameters:**
- `user_id`: User to check
- `badge_code`: Badge code required

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "has_access": true,
    "badge_award": {
      "badge_id": "badge-123",
      "awarded_at": "2025-11-11T14:30:00Z",
      "expires_at": "2026-11-11T14:30:00Z"
    }
  }
}
```

#### Request Badge Renewal

```
POST /v1/badges/awards/{award_id}/renew
```

**Response:** `200 OK`

Triggers renewal workflow (may require re-taking course).

#### Revoke Badge

```
DELETE /v1/badges/awards/{award_id}
```

**Request:**
```json
{
  "revoked_by_user_id": "admin-123",
  "reason": "Code of Conduct violation"
}
```

**Response:** `200 OK`

#### Badge Progress Tracking

```
GET /v1/users/me/badges/{badge_id}/progress
POST /v1/users/me/badges/{badge_id}/progress
```

Track progress toward earning a badge (e.g., course completion).

### NATS Events

**Published:**
- `badge.awarded`
- `badge.expired`
- `badge.renewed`
- `badge.revoked`
- `badge.expiry.warning` (30 days, 7 days before)

**Subscribed:**
- `course.completed` → Auto-award badges
- `coc.signed` → Award Code of Conduct badge

---

## Community Service API

**Base URL:** `/v1/communities`  
**Port:** 8010  
**Database:** `territory_{id}.communities`

### Endpoints

#### List Communities

```
GET /v1/communities
```

**Query Parameters:**
- `type`: `territory`, `guild`
- `parent_id`: Filter by parent community
- `scope`: `global`, `territory`, `community`
- `search`: Search by name
- `page`, `per_page`: Pagination

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "community_id": "comm-123",
      "code": "platform_dev",
      "name": "Platform Development",
      "description": "Building and maintaining the UnityPlan platform",
      "community_type": "guild",
      "join_policy": "badge_required",
      "access_badge_id": "badge-456",
      "parent_id": null,
      "scope": "global",
      "scope_id": null,
      "member_count": 47,
      "icon_url": "ipfs://Qm...",
      "created_at": "2025-01-01T00:00:00Z",
      "user_membership": {
        "is_member": true,
        "role": "member",
        "joined_at": "2025-11-01T00:00:00Z"
      }
    }
  ],
  "pagination": { ... }
}
```

#### Get Community

```
GET /v1/communities/{community_id}
```

**Response:** `200 OK`

#### Create Community

```
POST /v1/communities
```

**Headers:** `Authorization: Bearer {token}`

**Request:**
```json
{
  "code": "beekeepers_dk",
  "name": "Danish Beekeepers Guild",
  "description": "Guild for beekeeping enthusiasts in Denmark",
  "community_type": "guild",
  "join_policy": "open",
  "parent_id": null,
  "scope": "territory",
  "scope_id": "territory-dk-uuid",
  "icon_url": "ipfs://Qm...",
  "metadata": {
    "topics": ["beekeeping", "sustainability", "nature"]
  }
}
```

**Response:** `201 Created`

#### Update Community

```
PATCH /v1/communities/{community_id}
```

**Request:**
```json
{
  "name": "Updated Name",
  "description": "Updated description",
  "icon_url": "ipfs://QmNew..."
}
```

**Response:** `200 OK`

#### Community Members

```
GET /v1/communities/{community_id}/members
```

**Query Parameters:**
- `role`: Filter by role (founder, elected, member)
- `page`, `per_page`

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "user_id": "user-123",
      "username": "john_doe",
      "display_name": "John Doe",
      "avatar_url": "ipfs://Qm...",
      "role": "member",
      "joined_at": "2025-11-01T00:00:00Z",
      "is_active": true
    }
  ]
}
```

#### Join Community

```
POST /v1/communities/{community_id}/join
```

**Headers:** `Authorization: Bearer {token}`

**Response:** 
- `201 Created`: Joined successfully (open/badge-gated)
- `202 Accepted`: Approval required

```json
{
  "success": true,
  "data": {
    "status": "approved",
    "joined_at": "2025-11-11T14:30:00Z"
  }
}
```

**Errors:**
- `403`: Badge required or invitation needed
- `409`: Already a member

#### Leave Community

```
POST /v1/communities/{community_id}/leave
```

**Response:** `200 OK`

#### Invite User to Community

```
POST /v1/communities/{community_id}/invitations
```

**Request:**
```json
{
  "invited_user_id": "user-456",
  "message": "We'd love to have you in our community!",
  "role": "member"
}
```

**Response:** `201 Created`

#### Community Roles

```
GET /v1/communities/{community_id}/roles
POST /v1/communities/{community_id}/roles
PATCH /v1/communities/{community_id}/roles/{role_id}
DELETE /v1/communities/{community_id}/roles/{role_id}
```

#### Assign Role

```
POST /v1/communities/{community_id}/members/{user_id}/roles
```

**Request:**
```json
{
  "role_id": "role-789",
  "assigned_by_user_id": "admin-123"
}
```

#### Community Elections

```
GET /v1/communities/{community_id}/elections
POST /v1/communities/{community_id}/elections
GET /v1/communities/{community_id}/elections/{election_id}
POST /v1/communities/{community_id}/elections/{election_id}/vote
```

**Create Election:**
```json
{
  "role_id": "role-moderator",
  "election_type": "simple_majority",
  "nomination_start": "2025-11-15T00:00:00Z",
  "nomination_end": "2025-11-20T00:00:00Z",
  "voting_start": "2025-11-21T00:00:00Z",
  "voting_end": "2025-11-28T00:00:00Z",
  "required_badge_id": "badge-coc"
}
```

**Vote:**
```json
{
  "candidate_user_id": "user-456"
}
```

#### Sub-Communities

```
GET /v1/communities/{community_id}/sub-communities
POST /v1/communities/{community_id}/sub-communities
```

### NATS Events Published

- `community.created`
- `community.updated`
- `community.member.joined`
- `community.member.left`
- `community.role.assigned`
- `community.election.started`
- `community.election.completed`

---

## Course Service API

**Base URL:** `/v1/courses`  
**Port:** 8004  
**Database:** `territory_{id}` (future: group_courses table)

### Endpoints

#### List Courses

```
GET /v1/courses
```

**Query Parameters:**
- `category`: Filter by category
- `difficulty`: `beginner`, `intermediate`, `advanced`
- `status`: `draft`, `published`, `archived`
- `enrolled`: `true` (my courses)
- `page`, `per_page`

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "course_id": "course-123",
      "title": "Introduction to Decentralized Systems",
      "description": "Learn the fundamentals of decentralization",
      "category": "technology",
      "difficulty": "beginner",
      "duration_hours": 10,
      "badge_awarded": {
        "badge_id": "badge-456",
        "name": "Decentralization Fundamentals"
      },
      "prerequisite_badges": [],
      "enrollment_count": 234,
      "completion_rate": 0.78,
      "thumbnail_url": "ipfs://Qm...",
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-11-01T00:00:00Z",
      "user_enrollment": {
        "is_enrolled": true,
        "progress_percentage": 45,
        "started_at": "2025-11-01T00:00:00Z",
        "last_accessed": "2025-11-10T00:00:00Z"
      }
    }
  ]
}
```

#### Get Course

```
GET /v1/courses/{course_id}
```

**Response:** `200 OK`

Includes full course structure with modules and lessons.

#### Enroll in Course

```
POST /v1/courses/{course_id}/enroll
```

**Headers:** `Authorization: Bearer {token}`

**Response:** `201 Created`
```json
{
  "success": true,
  "data": {
    "enrollment_id": "enroll-789",
    "course_id": "course-123",
    "user_id": "user-456",
    "enrolled_at": "2025-11-11T14:30:00Z",
    "status": "active"
  }
}
```

**Errors:**
- `403`: Missing prerequisite badges
- `409`: Already enrolled

#### Get Course Progress

```
GET /v1/courses/{course_id}/progress
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "course_id": "course-123",
    "progress_percentage": 45,
    "modules": [
      {
        "module_id": "mod-1",
        "title": "Introduction",
        "completed": true,
        "lessons_completed": 5,
        "lessons_total": 5
      },
      {
        "module_id": "mod-2",
        "title": "Core Concepts",
        "completed": false,
        "lessons_completed": 2,
        "lessons_total": 7
      }
    ],
    "last_lesson_id": "lesson-12",
    "estimated_completion_date": "2025-12-01T00:00:00Z"
  }
}
```

#### Complete Lesson

```
POST /v1/courses/{course_id}/lessons/{lesson_id}/complete
```

**Request:**
```json
{
  "time_spent_seconds": 1200,
  "quiz_score": 95
}
```

**Response:** `200 OK`

#### Complete Course

When all lessons are completed:

**Response:**
```json
{
  "success": true,
  "data": {
    "course_id": "course-123",
    "completed_at": "2025-11-11T14:30:00Z",
    "badge_awarded": {
      "award_id": "award-999",
      "badge_id": "badge-456",
      "name": "Decentralization Fundamentals"
    }
  }
}
```

Publishes `course.completed` event → Badge Service awards badge.

#### Course Content (Modules, Lessons)

```
GET /v1/courses/{course_id}/modules
GET /v1/courses/{course_id}/modules/{module_id}
GET /v1/courses/{course_id}/modules/{module_id}/lessons
GET /v1/courses/{course_id}/lessons/{lesson_id}
```

#### Course Creation (Admin/Instructors)

```
POST /v1/courses
PATCH /v1/courses/{course_id}
DELETE /v1/courses/{course_id}
```

**Create Course:**
```json
{
  "title": "Advanced Holochain Development",
  "description": "Build production-ready hApps",
  "category": "technology",
  "difficulty": "advanced",
  "prerequisite_badge_ids": ["badge-holochain-basics"],
  "badge_to_award_id": "badge-holochain-advanced",
  "thumbnail_url": "ipfs://Qm..."
}
```

#### Course Reviews (Future)

```
GET /v1/courses/{course_id}/reviews
POST /v1/courses/{course_id}/reviews
```

### NATS Events

**Published:**
- `course.enrolled`
- `course.lesson.completed`
- `course.module.completed`
- `course.completed` → Badge Service listens
- `course.progress.updated`

**Subscribed:**
- `badge.expired` → Notify re-enrollment if course needed

---

## Forum Service API

**Base URL:** `/v1/forums`  
**Port:** 8005  
**Database:** `territory_{id}` (future: group_forums table)

### Endpoints

#### List Forums

```
GET /v1/forums
```

**Query Parameters:**
- `group_id`: Filter by group (bubble)
- `category`: Filter by category
- `accessible`: `true` (only forums I can access)

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "forum_id": "forum-123",
      "name": "Platform Development Discussion",
      "description": "Technical discussions about platform architecture",
      "group_id": "group-456",
      "access_badge_id": "badge-platform-mgmt",
      "topic_count": 47,
      "post_count": 523,
      "icon_url": "ipfs://Qm...",
      "created_at": "2025-01-01T00:00:00Z",
      "user_access": {
        "can_view": true,
        "can_post": true,
        "can_moderate": false
      }
    }
  ]
}
```

#### Get Forum

```
GET /v1/forums/{forum_id}
```

#### List Topics

```
GET /v1/forums/{forum_id}/topics
```

**Query Parameters:**
- `sort`: `latest`, `popular`, `unanswered`
- `status`: `open`, `closed`, `pinned`
- `page`, `per_page`

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "topic_id": "topic-789",
      "title": "How to implement WebSocket gateway?",
      "author": {
        "user_id": "user-123",
        "username": "alice",
        "avatar_url": "ipfs://Qm..."
      },
      "created_at": "2025-11-10T10:00:00Z",
      "last_activity_at": "2025-11-11T14:00:00Z",
      "reply_count": 12,
      "view_count": 145,
      "is_pinned": false,
      "is_closed": false,
      "tags": ["websocket", "backend", "rust"]
    }
  ]
}
```

#### Get Topic

```
GET /v1/forums/{forum_id}/topics/{topic_id}
```

**Response:** `200 OK`

Returns topic with paginated replies.

#### Create Topic

```
POST /v1/forums/{forum_id}/topics
```

**Headers:** `Authorization: Bearer {token}`

**Request:**
```json
{
  "title": "Best practices for error handling in Rust?",
  "content": "I'm building a microservice and wondering about...",
  "tags": ["rust", "error-handling", "best-practices"]
}
```

**Response:** `201 Created`

**Errors:**
- `403`: Missing required badge to post

#### Reply to Topic

```
POST /v1/forums/{forum_id}/topics/{topic_id}/replies
```

**Request:**
```json
{
  "content": "Great question! I recommend using the `thiserror` crate...",
  "parent_reply_id": null
}
```

**Response:** `201 Created`

#### Edit Post/Reply

```
PATCH /v1/forums/{forum_id}/topics/{topic_id}
PATCH /v1/forums/{forum_id}/replies/{reply_id}
```

#### Delete Post/Reply

```
DELETE /v1/forums/{forum_id}/topics/{topic_id}
DELETE /v1/forums/{forum_id}/replies/{reply_id}
```

**Response:** `204 No Content`

Soft delete, content marked as `[deleted]`.

#### Vote on Topic/Reply

```
POST /v1/forums/{forum_id}/topics/{topic_id}/vote
POST /v1/forums/{forum_id}/replies/{reply_id}/vote
```

**Request:**
```json
{
  "vote_type": "upvote"
}
```

Values: `upvote`, `downvote`, `remove`

#### Pin/Close Topic (Moderators)

```
POST /v1/forums/{forum_id}/topics/{topic_id}/pin
POST /v1/forums/{forum_id}/topics/{topic_id}/close
```

#### Report Content

```
POST /v1/forums/reports
```

**Request:**
```json
{
  "content_type": "topic",
  "content_id": "topic-789",
  "report_reason": "spam",
  "description": "This is promotional spam for external service"
}
```

**Response:** `201 Created`

Creates entry in `content_reports` table, notifies moderators.

#### Moderation Actions

```
GET /v1/forums/{forum_id}/moderation/reports
GET /v1/forums/{forum_id}/moderation/actions
POST /v1/forums/{forum_id}/moderation/actions
```

**Take Moderation Action:**
```json
{
  "report_id": "report-123",
  "action_type": "delete_content",
  "reason": "Spam violation",
  "notify_user": true
}
```

### NATS Events Published

- `forum.topic.created`
- `forum.reply.posted`
- `forum.content.reported`
- `forum.moderation.action_taken`

---

## Territory Service API

**Base URL:** `/v1/territories`  
**Port:** 8008  
**Database:** `global.territories`

### Endpoints

#### List Territories

```
GET /v1/territories
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "territory_id": "dk",
      "name": "Denmark",
      "territory_type": "country",
      "iso_code": "DK",
      "parent_territory_id": null,
      "status": "active",
      "database_schema": "territory_dk",
      "database_server": "postgres://eu-server:5432",
      "matrix_server": "https://matrix.dk.unityplan.org",
      "manager_user_ids": ["user-123", "user-456"],
      "created_at": "2025-01-01T00:00:00Z"
    }
  ]
}
```

#### Get Territory

```
GET /v1/territories/{territory_id}
```

#### Create Territory (Global Admin)

```
POST /v1/territories
```

**Request:**
```json
{
  "territory_id": "navajo-fn-us",
  "name": "Navajo Nation",
  "territory_type": "first_nation",
  "iso_code": null,
  "parent_territory_id": "us",
  "database_schema": "territory_navajo_fn_us",
  "database_server": "postgres://us-server:5432"
}
```

#### Update Territory Settings

```
PATCH /v1/territories/{territory_id}
```

#### Territory Managers

```
GET /v1/territories/{territory_id}/managers
POST /v1/territories/{territory_id}/managers
DELETE /v1/territories/{territory_id}/managers/{user_id}
```

#### Territory Stats

```
GET /v1/territories/{territory_id}/stats
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "territory_id": "dk",
    "user_count": 1247,
    "community_count": 23,
    "course_count": 15,
    "badge_awards_count": 3456,
    "active_users_30d": 876
  }
}
```

### NATS Events Published

- `territory.created`
- `territory.updated`
- `territory.manager.added`

---

## Notification Service API

**Base URL:** `/v1/notifications`  
**Port:** 8009  
**Database:** `territory_{id}.notifications`

### Endpoints

#### Get User Notifications

```
GET /v1/notifications
```

**Headers:** `Authorization: Bearer {token}`

**Query Parameters:**
- `type`: Filter by type (badge_expiry, role_election, etc.)
- `read`: `true`/`false`
- `page`, `per_page`

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "notification_id": "notif-123",
      "user_id": "user-456",
      "type": "badge_expiry_warning",
      "title": "Code of Conduct Badge Expiring Soon",
      "message": "Your Code of Conduct badge expires in 7 days",
      "data": {
        "badge_id": "badge-789",
        "badge_name": "Code of Conduct Agreement",
        "expires_at": "2025-11-18T00:00:00Z",
        "action_url": "/badges/renew/badge-789"
      },
      "is_read": false,
      "read_at": null,
      "created_at": "2025-11-11T14:30:00Z"
    }
  ],
  "pagination": { ... }
}
```

#### Mark as Read

```
PATCH /v1/notifications/{notification_id}/read
```

**Response:** `200 OK`

#### Mark All as Read

```
PATCH /v1/notifications/read-all
```

#### Delete Notification

```
DELETE /v1/notifications/{notification_id}
```

#### Get Notification Count

```
GET /v1/notifications/count
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "total": 15,
    "unread": 7
  }
}
```

#### WebSocket Connection

```
WS /v1/notifications/ws
```

**Headers:** `Authorization: Bearer {token}`

Real-time notification stream:

```json
{
  "type": "notification.new",
  "data": {
    "notification_id": "notif-999",
    "title": "New reply to your topic",
    "message": "Alice replied to 'How to implement WebSocket gateway?'",
    ...
  }
}
```

### NATS Events Subscribed

Listens to ALL events and creates notifications based on user preferences:

- `badge.expiry.warning` → Badge expiry notification
- `community.role.election.started` → Election notification
- `forum.reply.posted` → Forum reply notification
- `course.enrolled` → Course enrollment confirmation
- etc.

---

## Translation Service API

**Base URL:** `/v1/translations`  
**Port:** 8007  
**Database:** `public.translations`

### Endpoints

#### Translate Text

```
POST /v1/translations/translate
```

**Request:**
```json
{
  "text": "Hello, how are you?",
  "source_language": "en",
  "target_language": "da",
  "context": "greeting"
}
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "translated_text": "Hej, hvordan har du det?",
    "source_language": "en",
    "target_language": "da",
    "confidence": 0.98,
    "cached": false
  }
}
```

#### Batch Translation

```
POST /v1/translations/batch
```

**Request:**
```json
{
  "texts": [
    "Hello",
    "How are you?",
    "Thank you"
  ],
  "source_language": "en",
  "target_language": "da"
}
```

#### Detect Language

```
POST /v1/translations/detect
```

**Request:**
```json
{
  "text": "Bonjour, comment allez-vous?"
}
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "detected_language": "fr",
    "confidence": 0.99
  }
}
```

#### Supported Languages

```
GET /v1/translations/languages
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "code": "en",
      "name": "English",
      "native_name": "English"
    },
    {
      "code": "da",
      "name": "Danish",
      "native_name": "Dansk"
    }
  ]
}
```

---

## Event Service API

**Base URL:** `/v1/events`  
**Port:** 8011  
**Database:** `territory_{id}.community_events`

### Endpoints

#### List Events

```
GET /v1/events
```

**Query Parameters:**
- `community_id`: Filter by community
- `start_date`, `end_date`: Date range
- `type`: `in_person`, `virtual`, `hybrid`
- `rsvp_status`: `attending`, `maybe`, `not_attending`

**Response:** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "event_id": "event-123",
      "community_id": "comm-456",
      "title": "Monthly Community Meetup",
      "description": "Join us for our monthly in-person gathering",
      "event_type": "in_person",
      "start_time": "2025-11-20T18:00:00Z",
      "end_time": "2025-11-20T21:00:00Z",
      "location": "Community Center, Copenhagen",
      "virtual_link": null,
      "capacity": 50,
      "rsvp_count": 32,
      "rsvp_deadline": "2025-11-19T18:00:00Z",
      "created_by_user_id": "user-789",
      "user_rsvp": {
        "status": "attending",
        "guest_count": 1
      }
    }
  ]
}
```

#### Get Event

```
GET /v1/events/{event_id}
```

#### Create Event

```
POST /v1/events
```

**Request:**
```json
{
  "community_id": "comm-456",
  "title": "Introduction to Holochain Workshop",
  "description": "Hands-on workshop building your first hApp",
  "event_type": "hybrid",
  "start_time": "2025-12-01T14:00:00Z",
  "end_time": "2025-12-01T17:00:00Z",
  "location": "Innovation Hub",
  "virtual_link": "https://meet.unityplan.org/workshop-123",
  "capacity": 30,
  "rsvp_deadline": "2025-11-30T14:00:00Z",
  "require_badge_id": "badge-coc"
}
```

**Response:** `201 Created`

#### Update Event

```
PATCH /v1/events/{event_id}
```

#### Delete Event

```
DELETE /v1/events/{event_id}
```

#### RSVP to Event

```
POST /v1/events/{event_id}/rsvp
```

**Request:**
```json
{
  "status": "attending",
  "guest_count": 2,
  "notes": "Vegetarian meal preference"
}
```

**Response:** `201 Created`

**Errors:**
- `403`: Event requires badge
- `409`: Event at capacity
- `410`: RSVP deadline passed

#### Update RSVP

```
PATCH /v1/events/{event_id}/rsvp
```

#### Cancel RSVP

```
DELETE /v1/events/{event_id}/rsvp
```

#### Event Attendees

```
GET /v1/events/{event_id}/attendees
```

**Response:** `200 OK`
```json
{
  "success": true,
  "data": {
    "attending": 28,
    "maybe": 5,
    "capacity": 30,
    "attendees": [
      {
        "user_id": "user-123",
        "username": "alice",
        "display_name": "Alice",
        "avatar_url": "ipfs://Qm...",
        "status": "attending",
        "guest_count": 1
      }
    ]
  }
}
```

### NATS Events Published

- `event.created`
- `event.updated`
- `event.cancelled`
- `event.rsvp.created`
- `event.reminder` (24 hours before)

---

## Matrix Gateway API

**Base URL:** `/v1/matrix`  
**Port:** 8006  
**Protocol:** Matrix Protocol Bridge

### Endpoints

#### Create Matrix Room

```
POST /v1/matrix/rooms
```

**Request:**
```json
{
  "name": "Platform Development",
  "topic": "Technical discussions",
  "visibility": "private",
  "preset": "trusted_private_chat",
  "invite_user_ids": ["user-123", "user-456"]
}
```

**Response:** `201 Created`
```json
{
  "success": true,
  "data": {
    "room_id": "!abc123:matrix.dk.unityplan.org",
    "room_alias": "#platform-dev:matrix.dk.unityplan.org"
  }
}
```

#### Send Message

```
POST /v1/matrix/rooms/{room_id}/send
```

**Request:**
```json
{
  "msgtype": "m.text",
  "body": "Hello everyone!",
  "format": "org.matrix.custom.html",
  "formatted_body": "<p>Hello <strong>everyone</strong>!</p>"
}
```

#### Get Room Messages

```
GET /v1/matrix/rooms/{room_id}/messages
```

**Query Parameters:**
- `from`: Pagination token
- `limit`: Max messages (default: 20)

#### Join Room

```
POST /v1/matrix/rooms/{room_id}/join
```

#### Leave Room

```
POST /v1/matrix/rooms/{room_id}/leave
```

#### Invite User

```
POST /v1/matrix/rooms/{room_id}/invite
```

**Request:**
```json
{
  "user_id": "user-789"
}
```

---

## Error Handling

### Standard Error Response

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": {
      "field": ["Validation error message"]
    }
  },
  "meta": {
    "timestamp": "2025-11-11T14:30:00Z",
    "request_id": "req_abc123"
  }
}
```

### Error Codes

| HTTP Status | Error Code | Description |
|-------------|------------|-------------|
| 400 | `VALIDATION_ERROR` | Request validation failed |
| 401 | `UNAUTHORIZED` | Missing or invalid authentication |
| 403 | `FORBIDDEN` | User lacks permission |
| 404 | `NOT_FOUND` | Resource not found |
| 409 | `CONFLICT` | Resource conflict (duplicate, constraint) |
| 410 | `GONE` | Resource no longer available |
| 422 | `UNPROCESSABLE_ENTITY` | Semantic validation error |
| 429 | `RATE_LIMITED` | Too many requests |
| 500 | `INTERNAL_ERROR` | Server error |
| 503 | `SERVICE_UNAVAILABLE` | Service temporarily down |

### Badge-Specific Errors

```json
{
  "error": {
    "code": "BADGE_REQUIRED",
    "message": "This action requires a badge you don't have",
    "details": {
      "required_badge_id": "badge-123",
      "required_badge_name": "Platform Management Access",
      "how_to_obtain": "Complete the 'Platform Architecture' course"
    }
  }
}
```

---

## Rate Limiting

### Default Limits

| Endpoint Type | Limit | Window |
|---------------|-------|--------|
| Authentication | 5 requests | 1 minute |
| Read (GET) | 100 requests | 1 minute |
| Write (POST/PUT/PATCH) | 30 requests | 1 minute |
| File Upload | 10 requests | 1 hour |
| WebSocket | 1 connection | Per user |

### Rate Limit Headers

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1699718400
```

### Rate Limit Exceeded Response

```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMITED",
    "message": "Too many requests. Please try again in 45 seconds",
    "details": {
      "retry_after": 45
    }
  }
}
```

---

## Versioning Strategy

### URL Versioning

```
/v1/users
/v2/users (future)
```

### Version Support Policy

- **Current Version**: Full support, new features
- **Previous Version**: Security updates only, 6-month deprecation notice
- **Deprecated Version**: Read-only, 3-month shutdown notice

### Version Header (Alternative)

```http
Accept: application/vnd.unityplan.v1+json
```

---

## Next Steps

### Implementation Priority (MVP Phase 1)

1. ✅ **Authentication Service** - Core login/JWT (STARTED)
2. ✅ **User Service** - Profiles, settings (STARTED)
3. 🔲 **Badge Service** - Access control foundation
4. 🔲 **Community Service** - Territory/guild communities
5. 🔲 **Notification Service** - Badge expiry warnings
6. 🔲 **Event Service** - Community calendar
7. 🔲 **Course Service** - LMS basics (defer to Phase 2)
8. 🔲 **Forum Service** - Build on groups system (defer to Phase 2)

### Phase 2 Extensions

- Matrix Gateway integration
- Translation service
- Advanced LMS features
- Forum system with groups
- File upload service (IPFS)

### Testing Requirements

Each service must include:

- **Unit Tests**: Business logic validation
- **Integration Tests**: Database interactions
- **API Tests**: Endpoint contracts
- **Load Tests**: Performance benchmarks

### Documentation

- OpenAPI/Swagger specs for each service
- Postman collections for testing
- Service-to-service communication diagrams
- NATS event catalog

---

**Document Status:** Ready for Implementation  
**Next Action:** Begin Badge Service API implementation  
**Dependencies:** Database schema (COMPLETE), shared-lib (IN PROGRESS)
