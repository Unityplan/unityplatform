# territory-service API Endpoints

**Base URL:** `http://localhost:8008`  
**Version:** v1  
**Status:** ⏳ Scaffolded (0/4 endpoints) - Phase 2

---

## Territory Endpoints

### 1. List Territories

**Endpoint:** `GET /api/v1/territories`  
**Authentication:** None (public)  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "territories": [
      {
        "code": "dk",
        "name": "Denmark",
        "display_name": "Denmark 🇩🇰",
        "domain": "denmark.unityplan.org",
        "is_accepting_registrations": true,
        "flag_emoji": "🇩🇰",
        "timezone": "Europe/Copenhagen"
      },
      {
        "code": "no",
        "name": "Norway",
        "display_name": "Norway 🇳🇴",
        "domain": "norway.unityplan.org",
        "is_accepting_registrations": true,
        "flag_emoji": "🇳🇴",
        "timezone": "Europe/Oslo"
      }
    ]
  }
}
```

---

### 2. Get Territory Details

**Endpoint:** `GET /api/v1/territories/{code}`  
**Authentication:** None (public)  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "territory": {
      "code": "dk",
      "name": "Denmark",
      "display_name": "Denmark 🇩🇰",
      "description": "UnityPlan pod for Denmark",
      "domain": "denmark.unityplan.org",
      "admin_email": "admin@denmark.unityplan.org",
      "flag_emoji": "🇩🇰",
      "timezone": "Europe/Copenhagen",
      "is_active": true,
      "is_accepting_registrations": true,
      "created_at": "2025-11-01T00:00:00Z"
    },
    "settings": {
      "invitation_required": true,
      "max_users": 0,
      "features_enabled": {
        "communities": true,
        "badges": true,
        "events": false,
        "courses": false,
        "forum": false
      }
    },
    "stats": {
      "total_users": 150,
      "active_users_7d": 42,
      "total_communities": 12,
      "calculated_at": "2025-11-12T00:00:00Z"
    }
  }
}
```

---

### 3. Update Territory Settings

**Endpoint:** `PATCH /api/v1/territories/{code}/settings`  
**Authentication:** Admin only  
**Status:** ⏳ Planned

**Request:**

```json
{
  "registration_enabled": false,
  "features_enabled": {
    "events": true
  }
}
```

---

### 4. Get Territory Stats

**Endpoint:** `GET /api/v1/territories/{code}/stats`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "stats": {
      "total_users": 150,
      "active_users_7d": 42,
      "active_users_30d": 98,
      "total_communities": 12,
      "total_posts": 345,
      "storage_used_mb": 1250,
      "calculated_at": "2025-11-12T00:00:00Z"
    },
    "growth": {
      "users_last_7d": 8,
      "communities_last_7d": 2
    }
  }
}
```

---

## Current Territories

- `dk` - Denmark 🇩🇰
- `no` - Norway 🇳🇴
- `se` - Sweden 🇸🇪
- `eu` - Europe 🇪🇺

---

**Last Updated:** November 12, 2025  
**Implementation Status:** 0/4 endpoints (0%)  
**Priority:** Phase 2
