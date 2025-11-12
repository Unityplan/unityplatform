# badge-service API Endpoints

**Base URL:** `http://localhost:8007`  
**Version:** v1  
**Status:** ⏳ Scaffolded (0/6 endpoints) - Phase 2

---

## Badge Endpoints

### 1. List Badges

**Endpoint:** `GET /api/v1/badges`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "badges": [
      {
        "id": "uuid",
        "name": "Early Adopter",
        "slug": "early-adopter",
        "description": "Joined during the alpha phase",
        "icon": "🌟",
        "rarity": "epic",
        "earned": false,
        "progress": null
      },
      {
        "id": "uuid",
        "name": "Social Butterfly",
        "slug": "social-butterfly",
        "description": "Have 10 followers",
        "icon": "🦋",
        "rarity": "common",
        "earned": false,
        "progress": {
          "current": 7,
          "target": 10,
          "percentage": 70
        }
      }
    ]
  }
}
```

---

### 2. Get User Badges

**Endpoint:** `GET /api/v1/badges/users/{user_id}`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

---

### 3. Award Badge

**Endpoint:** `POST /api/v1/badges/award`  
**Authentication:** Admin only  
**Status:** ⏳ Planned

**Request:**

```json
{
  "user_id": "uuid",
  "badge_id": "uuid",
  "awarded_by": "uuid"
}
```

---

### 4. Update Progress

**Endpoint:** `POST /api/v1/badges/progress`  
**Authentication:** Internal (called by other services)  
**Status:** ⏳ Planned

**Request:**

```json
{
  "user_id": "uuid",
  "badge_slug": "social-butterfly",
  "current_value": 8
}
```

---

### 5. Toggle Featured Badge

**Endpoint:** `PATCH /api/v1/badges/users/me/{badge_id}/feature`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

---

### 6. Get Badge Statistics

**Endpoint:** `GET /api/v1/badges/{id}/stats`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

---

## Initial Badges

- Early Adopter 🌟 (manual)
- First Steps 👣 (complete profile)
- Social Butterfly 🦋 (10 followers)
- Popular ⭐ (100 followers)
- Invitation Champion 🎫 (5 invitations used)
- Community Builder 🏘️ (created 3 communities)

---

**Last Updated:** November 12, 2025  
**Implementation Status:** 0/6 endpoints (0%)  
**Priority:** Phase 2
