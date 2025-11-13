# user-service Implemented API Endpoints

**Base URL:** `http://localhost:8002`  
**Version:** v1  
**Last Updated:** 2025-11-13  
**Status:** ✅ Phase 1 Complete (18/18 endpoints tested)

> **API Restructure (2025-11-13):** All user-related endpoints moved under `/api/v1/user/*` for semantic clarity. Service endpoints under `/api/v1/service/*`.

> **Field Naming:** All request/response bodies use `camelCase` (e.g., `displayName`, `isVisible`)

---

## Authentication

All endpoints require JWT authentication unless marked as **Public**.

**Authentication Header:**

```http
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJh...
```

**Swagger UI:** `http://localhost:8002/swagger-ui/`

---

## Table of Contents

1. [Service Endpoints](#service-endpoints) - 1 endpoint
2. [Profile Endpoints](#profile-endpoints) - 3 endpoints
3. [Profile Links](#profile-links) - 4 endpoints
4. [Language Proficiency](#language-proficiency) - 4 endpoints
5. [Connections](#connections) - 6 endpoints

**Total:** 18 endpoints implemented ✅

---

## Service Endpoints

### Health Check

```http
GET /api/v1/service/health
```

**Auth:** Public  
**Response:**

```json
{
  "service": "user-service",
  "status": "healthy",
  "version": "0.1.0-alpha.1"
}
```

---

## Profile Endpoints

### 1. Get Own Profile

```http
GET /api/v1/user/profile
```

**Auth:** Required  
**Response:**

```json
{
  "id": "21a7c1f4-0dc8-47f1-aa26-c9848fedb3a1",
  "username": "alice_admin",
  "displayName": null,
  "avatarUrl": null,
  "bio": "Platform administrator",
  "about": null,
  "location": null,
  "website": null,
  "interests": null,
  "skills": null,
  "createdAt": "2025-11-13T19:17:45.817709Z",
  "updatedAt": "2025-11-13T19:31:50.013976Z"
}
```

### 2. Update Own Profile

```http
PUT /api/v1/user/profile
```

**Auth:** Required  
**Request Body:**

```json
{
  "displayName": "Alice Anderson",
  "bio": "Rust developer from Denmark",
  "location": "Copenhagen",
  "website": "https://alice.dev"
}
```

**Response:** Same as Get Own Profile

### 3. Get Profile by ID

```http
GET /api/v1/user/profile/{id}
```

**Auth:** Required  
**Response:** Same as Get Own Profile

---

## Profile Links

### 4. List Links

```http
GET /api/v1/user/profile/links
```

**Auth:** Required  
**Response:**

```json
[
  {
    "id": "ea9d0291-0a10-410a-9e9f-2835a318a091",
    "label": "My GitHub",
    "url": "https://github.com/alice",
    "icon": "github",
    "displayOrder": 0,
    "isVisible": true,
    "createdAt": "2025-11-13T19:32:26.409477Z",
    "updatedAt": "2025-11-13T19:32:31.729363Z"
  }
]
```

### 5. Create Link

```http
POST /api/v1/user/profile/links
```

**Auth:** Required  
**Request Body:**

```json
{
  "label": "GitHub Profile",
  "url": "https://github.com/alice",
  "icon": "github",
  "displayOrder": 0,
  "isVisible": true
}
```

**Response:** Same as List Links (single object)

### 6. Update Link

```http
PUT /api/v1/user/profile/links/{id}
```

**Auth:** Required  
**Request Body:** (all fields optional)

```json
{
  "label": "My GitHub",
  "isVisible": false
}
```

**Response:** Same as List Links (single object)

### 7. Delete Link

```http
DELETE /api/v1/user/profile/links/{id}
```

**Auth:** Required  
**Response:** `204 No Content`

---

## Language Proficiency

**Proficiency Levels:** `none`, `basic`, `intermediate`, `fluent`, `native`

### 8. List Languages

```http
GET /api/v1/user/profile/languages
```

**Auth:** Required  
**Response:**

```json
[
  {
    "id": "7f56bf9b-e76c-471c-a742-ad36aa3a5fe2",
    "languageCode": "en",
    "languageName": "English",
    "spokenLevel": "native",
    "writtenLevel": "native",
    "readingLevel": "native",
    "listeningLevel": "native",
    "displayOrder": 0,
    "isPreferred": true,
    "showOnProfile": true,
    "createdAt": "2025-11-13T19:33:10.039998Z",
    "updatedAt": "2025-11-13T19:33:18.031614Z"
  }
]
```

### 9. Create Language

```http
POST /api/v1/user/profile/languages
```

**Auth:** Required  
**Request Body:**

```json
{
  "languageCode": "en",
  "languageName": "English",
  "spokenLevel": "native",
  "writtenLevel": "native",
  "readingLevel": "native",
  "listeningLevel": "native",
  "displayOrder": 0,
  "isPreferred": false,
  "showOnProfile": true
}
```

**Response:** Same as List Languages (single object)

### 10. Update Language

```http
PUT /api/v1/user/profile/languages/{id}
```

**Auth:** Required  
**Request Body:** (all fields optional)

```json
{
  "isPreferred": true,
  "spokenLevel": "fluent"
}
```

**Response:** Same as List Languages (single object)

### 11. Delete Language

```http
DELETE /api/v1/user/profile/languages/{id}
```

**Auth:** Required  
**Response:** `204 No Content`

---

## Connections

### 12. Follow User

```http
POST /api/v1/user/connections/{id}/follow
```

**Auth:** Required  
**Response:** `204 No Content`

### 13. Unfollow User

```http
DELETE /api/v1/user/connections/{id}/follow
```

**Auth:** Required  
**Response:** `204 No Content`

### 14. Get Followers

```http
GET /api/v1/user/connections/{id}/followers?limit=20&offset=0
```

**Auth:** Required  
**Query Params:**

- `limit` (optional): Max results (default: 20, max: 100)
- `offset` (optional): Pagination offset (default: 0)

**Response:**

```json
{
  "connections": [
    {
      "userId": "21a7c1f4-0dc8-47f1-aa26-c9848fedb3a1",
      "username": "alice_admin",
      "connectionType": "follow",
      "status": "active",
      "createdAt": "2025-11-13T19:33:37.374686Z"
    }
  ],
  "total": 1,
  "offset": 0,
  "limit": 20
}
```

### 15. Get Following

```http
GET /api/v1/user/connections/{id}/following?limit=20&offset=0
```

**Auth:** Required  
**Query Params:** Same as Get Followers  
**Response:** Same as Get Followers

### 16. Block User

```http
POST /api/v1/user/connections/{id}/block
```

**Auth:** Required  
**Response:** `204 No Content`

### 17. Unblock User

```http
DELETE /api/v1/user/connections/{id}/block
```

**Auth:** Required  
**Response:** `204 No Content`

### 18. Search Users

```http
GET /api/v1/user/connections/search?q=bob&limit=20&offset=0
```

**Auth:** Required  
**Query Params:**

- `q` (required): Search query (username or display name)
- `limit` (optional): Max results (default: 20, max: 100)
- `offset` (optional): Pagination offset (default: 0)

**Response:**

```json
{
  "users": [
    {
      "id": "ad684771-d98f-4078-9b96-fc4d03dcd7b2",
      "username": "bob_manager",
      "displayName": null,
      "avatarUrl": null,
      "bio": null,
      "isFollowing": false,
      "isFollower": false,
      "isBlocked": false
    }
  ],
  "total": 1,
  "offset": 0,
  "limit": 20
}
```

---

## Testing Examples

```bash
# Get token
TOKEN=$(curl -s -X POST http://localhost:8001/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice_admin", "password": "SecurePass123!", "territory": "dk"}' \
  | jq -r '.access_token')

# Get own profile
curl -s "http://localhost:8002/api/v1/user/profile" \
  -H "Authorization: Bearer $TOKEN" | jq

# Update profile
curl -s -X PUT "http://localhost:8002/api/v1/user/profile" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"displayName": "Alice", "bio": "Platform admin"}' | jq

# Create profile link
curl -s -X POST "http://localhost:8002/api/v1/user/profile/links" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"label": "GitHub", "url": "https://github.com/alice", "icon": "github"}' | jq

# Search users
curl -s "http://localhost:8002/api/v1/user/connections/search?q=bob" \
  -H "Authorization: Bearer $TOKEN" | jq
```

---

## Notes

- All timestamps are in ISO 8601 format (UTC)
- All request/response bodies use camelCase field naming
- Optional fields in responses may be `null`
- Empty arrays returned for empty lists (not null)
- Search query parameter is `q` (not `query`)
- Route registration order matters: specific routes before greedy `/{id}` routes

## Swagger Tags

The OpenAPI documentation uses these tags to organize endpoints:

- `service` - Service health and metadata
- `profile` - User profile management
- `profile-links` - External profile links
- `language-proficiency` - Language skills
- `connections` - User connections (follow/block/search)
