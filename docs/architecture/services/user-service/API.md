# user-service API Endpoints

**Base URL:** `http://localhost:8002`  
**Version:** v1  
**Status:** ✅ Production Ready (28/28 endpoints complete)

---

## Authentication

All endpoints require JWT authentication unless marked as **Public**.

### **Authentication Header**

```http
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### **JWT Claims**

```json
{
  "sub": "550e8400-e29b-41d4-a716-446655440000",  // user_id
  "territory": "dk",                               // territory_code
  "exp": 1731427200,                               // expiration timestamp
  "iat": 1731426300                                // issued at timestamp
}
```

### **Validation Strategy**

- **Standard requests:** JWT signature validation only (~0.01ms, no database query)
- **Critical operations:** JWT + database check for real-time user status

### **Error Responses**

```json
// 401 Unauthorized - Missing or invalid token
{
  "error": "unauthorized",
  "message": "Missing or invalid Authorization header"
}

// 403 Forbidden - User deleted/inactive (critical operations only)
{
  "error": "forbidden",
  "message": "Account inactive or deleted"
}
```

**See [shared-lib/AUTHENTICATION.md](../shared-lib/AUTHENTICATION.md) for complete authentication documentation.**

---

## Table of Contents

1. [User Profile Endpoints](#user-profile-endpoints) (3 endpoints)
2. [Profile Links Endpoints](#profile-links-endpoints) (4 endpoints)
3. [Language Proficiency Endpoints](#language-proficiency-endpoints) (4 endpoints)
4. [User Connections Endpoints](#user-connections-endpoints) (7 endpoints)
5. [Search Endpoints](#search-endpoints) (2 endpoints)
6. [Avatar Management](#avatar-management) (2 endpoints)
7. [Multi-User Operations](#multi-user-operations) (3 endpoints)
8. [GDPR Data Export](#gdpr-data-export) (3 endpoints)

---

## User Profile Endpoints

### 1. Get User Profile

**Endpoint:** `GET /api/v1/profiles/:id`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Retrieve user profile by ID.

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "display_name": "Alice Anderson",
    "bio": "Rust developer from Denmark",
    "location": "Copenhagen",
    "avatar_url": "https://ipfs.unityplatform.org/ipfs/QmXxx",
    "website": "https://alice.dev",
    "created_at": "2025-11-01T10:00:00Z",
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

---

### 2. Update Own Profile

**Endpoint:** `PUT /api/v1/profiles/me`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Update authenticated user's profile.

**Request Body:**

```json
{
  "display_name": "Alice Anderson",
  "bio": "Rust developer from Denmark",
  "location": "Copenhagen",
  "website": "https://alice.dev"
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "display_name": "Alice Anderson",
    "bio": "Rust developer from Denmark",
    "location": "Copenhagen",
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

---

### 3. Get Own Profile

**Endpoint:** `GET /api/v1/profiles/me`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get authenticated user's profile.

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "email": "alice@example.com",
    "display_name": "Alice Anderson",
    "bio": "Rust developer from Denmark",
    "location": "Copenhagen",
    "avatar_url": null,
    "website": "https://alice.dev",
    "created_at": "2025-11-01T10:00:00Z",
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

---

## Profile Links Endpoints

### 4. Get Profile Links

**Endpoint:** `GET /api/v1/profiles/:id/links`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get all profile links for a user.

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "link-uuid-1",
      "type": "github",
      "url": "https://github.com/alice",
      "display_order": 1,
      "created_at": "2025-11-01T10:00:00Z"
    },
    {
      "id": "link-uuid-2",
      "type": "linkedin",
      "url": "https://linkedin.com/in/alice",
      "display_order": 2,
      "created_at": "2025-11-01T10:00:00Z"
    }
  ]
}
```

---

### 5. Add Profile Link

**Endpoint:** `POST /api/v1/profiles/:id/links`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Add a new profile link.

**Request Body:**

```json
{
  "type": "github",
  "url": "https://github.com/alice"
}
```

**Link Types:**

- `github`, `gitlab`, `twitter`, `linkedin`, `youtube`, `instagram`, `facebook`, `website`, `blog`, `other`

**Response (201 Created):**

```json
{
  "success": true,
  "data": {
    "id": "link-uuid",
    "type": "github",
    "url": "https://github.com/alice",
    "display_order": 1,
    "created_at": "2025-11-12T10:00:00Z"
  }
}
```

---

### 6. Update Profile Link

**Endpoint:** `PUT /api/v1/profiles/:id/links/:link_id`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Update an existing profile link.

**Request Body:**

```json
{
  "url": "https://github.com/alice-dev",
  "display_order": 2
}
```

---

### 7. Delete Profile Link

**Endpoint:** `DELETE /api/v1/profiles/:id/links/:link_id`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Delete a profile link.

**Response (200 OK):**

```json
{
  "success": true,
  "message": "Link deleted successfully"
}
```

---

## Language Proficiency Endpoints

### 8. Get Language Proficiencies

**Endpoint:** `GET /api/v1/profiles/:id/languages`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get all languages user speaks.

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "lang-uuid-1",
      "language_code": "da",
      "language_name": "Danish",
      "proficiency_level": "native",
      "created_at": "2025-11-01T10:00:00Z"
    },
    {
      "id": "lang-uuid-2",
      "language_code": "en",
      "language_name": "English",
      "proficiency_level": "fluent",
      "created_at": "2025-11-01T10:00:00Z"
    }
  ]
}
```

**Proficiency Levels:**

- `basic` - A1/A2
- `intermediate` - B1/B2
- `advanced` - C1/C2
- `fluent` - Near-native
- `native` - Mother tongue

---

### 9. Add Language Proficiency

**Endpoint:** `POST /api/v1/profiles/:id/languages`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Request Body:**

```json
{
  "language_code": "de",
  "proficiency_level": "intermediate"
}
```

**Response (201 Created):**

```json
{
  "success": true,
  "data": {
    "id": "lang-uuid",
    "language_code": "de",
    "language_name": "German",
    "proficiency_level": "intermediate",
    "created_at": "2025-11-12T10:00:00Z"
  }
}
```

---

### 10. Update Language Proficiency

**Endpoint:** `PUT /api/v1/profiles/:id/languages/:lang_id`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Request Body:**

```json
{
  "proficiency_level": "advanced"
}
```

---

### 11. Delete Language Proficiency

**Endpoint:** `DELETE /api/v1/profiles/:id/languages/:lang_id`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

---

## User Connections Endpoints

### 12. Follow User

**Endpoint:** `POST /api/v1/users/:id/follow`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Follow another user.

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "follower_id": "my-user-id",
    "followed_id": "their-user-id",
    "created_at": "2025-11-12T10:00:00Z"
  }
}
```

**NATS Event Published:** `user.followed`

---

### 13. Unfollow User

**Endpoint:** `DELETE /api/v1/users/:id/follow`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Unfollow a user.

**Response (200 OK):**

```json
{
  "success": true,
  "message": "Unfollowed successfully"
}
```

**NATS Event Published:** `user.unfollowed`

---

### 14. Get Followers

**Endpoint:** `GET /api/v1/users/:id/followers`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get list of users following this user.

**Query Parameters:**

- `page` - Page number (default: 1)
- `limit` - Results per page (default: 20, max: 100)

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "followers": [
      {
        "id": "follower-uuid",
        "username": "bob",
        "display_name": "Bob Builder",
        "avatar_url": "https://...",
        "followed_at": "2025-11-10T10:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 42
    }
  }
}
```

---

### 15. Get Following

**Endpoint:** `GET /api/v1/users/:id/following`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get list of users this user follows.

**Response:** Same format as Get Followers

---

### 16. Block User

**Endpoint:** `POST /api/v1/users/:id/block`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Block a user (prevents following, messaging, etc.).

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "blocker_id": "my-user-id",
    "blocked_id": "their-user-id",
    "blocked_at": "2025-11-12T10:00:00Z"
  }
}
```

**Actions:**

- Removes mutual follows
- Prevents future follows
- Hides content from blocked user

---

### 17. Unblock User

**Endpoint:** `DELETE /api/v1/users/:id/block`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

---

### 18. Get Blocked Users

**Endpoint:** `GET /api/v1/users/me/blocked`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get list of users you've blocked.

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "blocked-uuid",
      "username": "spammer",
      "blocked_at": "2025-11-12T10:00:00Z"
    }
  ]
}
```

---

## Search Endpoints

### 19. Search Users

**Endpoint:** `GET /api/v1/users/search`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Search users by username or display name.

**Query Parameters:**

- `q` - Search query (min 2 characters)
- `page` - Page number
- `limit` - Results per page

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "users": [
      {
        "id": "uuid",
        "username": "alice",
        "display_name": "Alice Anderson",
        "avatar_url": "https://...",
        "bio": "Rust developer from Denmark"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 5
    }
  }
}
```

---

### 20. Get User by Username

**Endpoint:** `GET /api/v1/users/username/:username`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get user profile by username.

**Response:** Same as Get User Profile

---

## Avatar Management

### 21. Upload Avatar

**Endpoint:** `POST /api/v1/profiles/me/avatar`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Upload user avatar image (calls ipfs-service).

**Request:** `multipart/form-data`

```
Content-Type: multipart/form-data
file: <image binary>
```

**Supported Formats:** JPG, PNG, GIF, WebP  
**Max Size:** 5MB  
**Recommended:** 500x500px

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "avatar_url": "https://ipfs.unityplatform.org/ipfs/QmXxx"
  }
}
```

---

### 22. Delete Avatar

**Endpoint:** `DELETE /api/v1/profiles/me/avatar`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Remove user avatar.

---

## Multi-User Operations

### 23. Get Multiple Users

**Endpoint:** `POST /api/v1/users/batch`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get multiple user profiles in one request.

**Request Body:**

```json
{
  "user_ids": [
    "uuid-1",
    "uuid-2",
    "uuid-3"
  ]
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid-1",
      "username": "alice",
      "display_name": "Alice Anderson",
      "avatar_url": "https://..."
    },
    {
      "id": "uuid-2",
      "username": "bob",
      "display_name": "Bob Builder",
      "avatar_url": "https://..."
    }
  ]
}
```

---

### 24. List All Users

**Endpoint:** `GET /api/v1/users`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** List all users (paginated).

**Query Parameters:**

- `page` - Page number
- `limit` - Results per page

---

### 25. Get User Statistics

**Endpoint:** `GET /api/v1/users/:id/stats`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Get user statistics.

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "followers_count": 42,
    "following_count": 15,
    "posts_count": 127,
    "badges_count": 5
  }
}
```

---

## GDPR Data Export

### 26. Request Data Export

**Endpoint:** `POST /api/v1/users/:id/data/export`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Request GDPR data export (Article 20).

**Response (202 Accepted):**

```json
{
  "success": true,
  "data": {
    "export_id": "export-uuid",
    "status": "processing",
    "requested_at": "2025-11-12T10:00:00Z"
  }
}
```

**Export includes:**

- Profile data
- Language proficiencies
- Profile links
- Connections (followers/following)
- Settings (from settings-service)
- All user-generated content

---

### 27. List Data Exports

**Endpoint:** `GET /api/v1/users/:id/data/export`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** List all data export requests.

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "export-uuid",
      "status": "completed",
      "requested_at": "2025-11-12T10:00:00Z",
      "completed_at": "2025-11-12T10:05:00Z",
      "download_url": "/api/v1/users/:id/data/export/export-uuid",
      "expires_at": "2025-11-19T10:05:00Z"
    }
  ]
}
```

**Status Values:**

- `processing` - Export in progress
- `completed` - Ready for download
- `failed` - Export failed

---

### 28. Download Data Export

**Endpoint:** `GET /api/v1/users/:id/data/export/:export_id`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Download data export file.

**Response:** JSON file download

```json
{
  "export_metadata": {
    "user_id": "uuid",
    "username": "alice",
    "exported_at": "2025-11-12T10:05:00Z",
    "gdpr_article": "Article 20 - Right to data portability"
  },
  "profile": { ... },
  "languages": [ ... ],
  "links": [ ... ],
  "connections": {
    "followers": [ ... ],
    "following": [ ... ]
  },
  "settings": { ... }
}
```

---

## Account Deletion (GDPR Article 17)

### 29. Request Account Deletion

**Endpoint:** `POST /api/v1/users/:id/account/delete`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Request account deletion with 30-day grace period.

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "deletion_id": "deletion-uuid",
    "status": "pending",
    "scheduled_for": "2025-12-12T10:00:00Z",
    "confirmation_token_sent": true
  }
}
```

**Actions:**

- Sends confirmation email with token
- Sets deletion date 30 days in future
- Account remains active during grace period

---

### 30. Confirm Account Deletion

**Endpoint:** `POST /api/v1/users/:id/account/delete/confirm`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Confirm account deletion with token from email.

**Request Body:**

```json
{
  "confirmation_token": "token-from-email"
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "message": "Account deletion confirmed. Your account will be deleted on 2025-12-12."
}
```

---

### 31. Get Deletion Status

**Endpoint:** `GET /api/v1/users/:id/account/delete`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Check account deletion status.

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "status": "confirmed",
    "scheduled_for": "2025-12-12T10:00:00Z",
    "can_cancel": true,
    "days_remaining": 30
  }
}
```

---

### 32. Cancel Account Deletion

**Endpoint:** `DELETE /api/v1/users/:id/account/delete`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Description:** Cancel pending account deletion.

**Response (200 OK):**

```json
{
  "success": true,
  "message": "Account deletion cancelled"
}
```

---

## Testing

```bash
# Get own profile
curl -X GET http://localhost:8002/api/v1/profiles/me \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

# Update profile
curl -X PUT http://localhost:8002/api/v1/profiles/me \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "display_name": "Alice Anderson",
    "bio": "Rust developer"
  }'

# Follow user
curl -X POST http://localhost:8002/api/v1/users/USER_ID/follow \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

# Search users
curl -X GET "http://localhost:8002/api/v1/users/search?q=alice" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

# Request data export
curl -X POST http://localhost:8002/api/v1/users/USER_ID/data/export \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

---

**Last Updated:** November 12, 2025  
**Service Version:** 0.1.0-alpha.1  
**Implementation Status:** 28/28 endpoints (100%)  
**Tests:** 22/22 passing  
**OpenAPI:** <http://localhost:8002/swagger-ui>
