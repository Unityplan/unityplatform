# community-service API Endpoints

**Base URL:** `http://localhost:8006`  
**Version:** v1  
**Status:** ⏳ Scaffolded (0/8 endpoints) - Phase 2

---

## Community Endpoints

### 1. Create Community

**Endpoint:** `POST /api/v1/communities`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Request:**

```json
{
  "name": "Rust Developers Denmark",
  "slug": "rust-developers-dk",
  "description": "A community for Rust developers in Denmark",
  "visibility": "public",
  "require_approval": false
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "Rust Developers Denmark",
    "slug": "rust-developers-dk",
    "description": "A community for Rust developers in Denmark",
    "visibility": "public",
    "created_by": "uuid",
    "member_count": 1,
    "created_at": "2025-11-12T10:00:00Z"
  }
}
```

---

### 2. Get Community

**Endpoint:** `GET /api/v1/communities/{slug}`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

---

### 3. List Communities

**Endpoint:** `GET /api/v1/communities`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Query Parameters:**

- `search` - Search by name/description
- `territory` - Filter by territory code
- `sort` - Sort by (members/newest)
- `page`, `limit` - Pagination

---

### 4. Update Community

**Endpoint:** `PATCH /api/v1/communities/{id}`  
**Authentication:** Bearer token required (admin/owner only)  
**Status:** ⏳ Planned

---

### 5. Delete Community

**Endpoint:** `DELETE /api/v1/communities/{id}`  
**Authentication:** Bearer token required (owner only)  
**Status:** ⏳ Planned

---

### 6. Join Community

**Endpoint:** `POST /api/v1/communities/{id}/join`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "membership": {
      "community_id": "uuid",
      "user_id": "uuid",
      "role": "member",
      "joined_at": "2025-11-12T10:00:00Z",
      "pending_approval": false
    }
  }
}
```

---

### 7. Leave Community

**Endpoint:** `POST /api/v1/communities/{id}/leave`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

---

### 8. Manage Members

**Endpoint:** `GET /api/v1/communities/{id}/members`  
**Endpoint:** `PATCH /api/v1/communities/{id}/members/{user_id}`  
**Endpoint:** `DELETE /api/v1/communities/{id}/members/{user_id}`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

---

**Last Updated:** November 12, 2025  
**Implementation Status:** 0/8 endpoints (0%)  
**Priority:** Phase 2
