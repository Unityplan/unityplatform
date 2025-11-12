# notification-service API Endpoints

**Base URL:** `http://localhost:8005`  
**Version:** v1  
**Status:** ⏳ Scaffolded (0/7 endpoints)

---

## Notification Endpoints

### 1. List Notifications

**Endpoint:** `GET /api/v1/notifications`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Query Parameters:**

- `unread_only` - Boolean (default: false)
- `type` - Filter by type
- `page` - Page number
- `limit` - Results per page

**Response:**

```json
{
  "success": true,
  "data": {
    "notifications": [
      {
        "id": "uuid",
        "type": "follower",
        "title": "New Follower",
        "message": "alice started following you",
        "data": {
          "user_id": "uuid",
          "username": "alice"
        },
        "link": "/users/alice",
        "is_read": false,
        "created_at": "2025-11-12T10:00:00Z"
      }
    ],
    "unread_count": 5,
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 42
    }
  }
}
```

---

### 2. Get Unread Count

**Endpoint:** `GET /api/v1/notifications/unread/count`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "count": 5
  }
}
```

---

### 3. Mark as Read

**Endpoint:** `PATCH /api/v1/notifications/{id}/read`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "is_read": true,
    "read_at": "2025-11-12T10:05:00Z"
  }
}
```

---

### 4. Mark All as Read

**Endpoint:** `PATCH /api/v1/notifications/read-all`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "updated_count": 5
  }
}
```

---

### 5. Delete Notification

**Endpoint:** `DELETE /api/v1/notifications/{id}`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

---

### 6. Get Notification Settings

**Endpoint:** `GET /api/v1/notifications/settings`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "email": {
      "enabled": true,
      "on_follower": true,
      "on_message": true,
      "on_badge": true,
      "on_invitation_used": true,
      "on_community_invite": true
    },
    "in_app": {
      "enabled": true,
      "on_follower": true,
      "on_message": true,
      "on_badge": true,
      "on_invitation_used": true,
      "on_community_invite": true
    },
    "digest": {
      "frequency": "daily",
      "time": "09:00:00"
    }
  }
}
```

---

### 7. Update Notification Settings

**Endpoint:** `PATCH /api/v1/notifications/settings`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Request:**

```json
{
  "email": {
    "on_follower": false,
    "on_message": false
  },
  "digest": {
    "frequency": "weekly"
  }
}
```

---

## Notification Types

- `follower` - Someone followed you
- `message` - New message (future)
- `badge` - Badge awarded
- `invitation_used` - Your invitation was used
- `community_invite` - Invited to community (future)
- `community_post` - New community post (future)
- `course_update` - Course updated (future)
- `forum_reply` - Forum reply (future)

---

## NATS Events Subscribed

- `user.followed` → Create follower notification
- `badge.awarded` → Create badge notification
- `invitation.used` → Create invitation used notification
- `message.received` → Create message notification
- `community.invited` → Create community invite notification

---

**Last Updated:** November 12, 2025  
**Implementation Status:** 0/7 endpoints (0%)  
**Priority:** Week 2 of migration plan
