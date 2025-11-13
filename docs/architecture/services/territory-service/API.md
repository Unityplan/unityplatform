# territory-service API Endpoints

**Base URL:** `http://localhost:8008`  
**Version:** v1  
**Status:** 🚧 In Progress (4/7 endpoints) - Phase 2

---

## Public Endpoints (Authenticated Users)

### 1. List All Active Territories

**Endpoint:** `GET /api/v1/territories`  
**Authentication:** Bearer token required  
**Authorization:** Any authenticated user  
**Status:** ✅ Implemented

**Response:**

```json
{
  "success": true,
  "data": {
    "territories": [
      {
        "code": "dk",
        "name": "Denmark",
        "display_name": "Denmark Territory",
        "description": "Primary territory for Denmark-based users",
        "pod_url": "https://denmark.unityplatform.dk",
        "api_url": "https://api.denmark.unityplatform.dk",
        "status": "active",
        "language_code": "da",
        "timezone": "Europe/Copenhagen",
        "currency_code": "DKK"
      }
    ]
  }
}
```

**Use Case:** Users viewing available territories

---

### 2. Get Territory Details

**Endpoint:** `GET /api/v1/territories/{code}`  
**Authentication:** Bearer token required  
**Authorization:** Any authenticated user  
**Status:** ✅ Implemented
**Status:** ⏳ Not Started

**Response:**

```json
{
  "success": true,
  "data": {
    "territory": {
      "code": "dk",
      "name": "Denmark",
      "display_name": "Denmark Territory",
      "description": "Primary territory for Denmark-based users",
      "pod_url": "https://denmark.unityplatform.dk",
      "api_url": "https://api.denmark.unityplatform.dk",
      "status": "active",
      "language_code": "da",
      "timezone": "Europe/Copenhagen",
      "currency_code": "DKK",
      "created_at": "2025-11-01T00:00:00Z",
      "updated_at": "2025-11-13T00:00:00Z"
    },
    "settings": {
      "registration_enabled": true,
      "invitation_required": true,
      "max_users": 0,
      "features_enabled": {
        "communities": true,
        "badges": true,
        "events": false,
        "courses": false,
        "forum": false
      },
      "primary_color": "#2E7D32",
      "logo_url": null
    },
    "stats": {
      "total_users": 5,
      "active_users_7d": 3,
      "total_communities": 2,
      "calculated_at": "2025-11-13T00:00:00Z"
    }
  }
}
```

**Use Case:** Frontend territory info pages for logged-in users

---

## Territory Manager Endpoints

Requires active "Territory Manager" badge AND assignment to territory.

### 3. Get Territory Statistics

**Endpoint:** `GET /api/v1/territories/{code}/manage/stats`  
**Authentication:** Bearer token required  
**Authorization:** Territory manager for this territory  
**Status:** ✅ Implemented

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
      "calculated_at": "2025-11-13T00:00:00Z"
    },
    "growth": {
      "users_last_7d": 8,
      "users_last_30d": 25,
      "communities_last_7d": 2
    }
  }
}
```

**Use Case:** Territory manager dashboard

---

### 4. Update Territory Settings

**Endpoint:** `PATCH /api/v1/territories/{code}/manage/settings`  
**Authentication:** Bearer token required  
**Authorization:** Territory manager for this territory  
**Status:** ✅ Implemented

**Request:**

```json
{
  "name": "Denmark",
  "display_name": "Kingdom of Denmark",
  "description": "Updated description",
  "language_code": "da",
  "timezone": "Europe/Copenhagen",
  "currency_code": "DKK",
  "registration_enabled": false,
  "features_enabled": {
    "events": true
  },
  "primary_color": "#1B5E20"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "settings": {
      "name": "Denmark",
      "display_name": "Kingdom of Denmark",
      "description": "Updated description",
      "language_code": "da",
      "timezone": "Europe/Copenhagen",
      "currency_code": "DKK",
      "registration_enabled": false,
      "invitation_required": true,
      "max_users": 0,
      "features_enabled": {
        "communities": true,
        "badges": true,
        "events": true,
        "courses": false,
        "forum": false
      },
      "primary_color": "#1B5E20",
      "updated_at": "2025-11-13T10:30:00Z"
    }
  }
}
```

**Side Effects:**

- Updates `territory_{code}.territory_settings`
- Replicates to `global.territories_registry`
- Publishes NATS event: `territory.settings.updated`

**Use Case:** Territory manager updating territory configuration

---

## Platform Manager Endpoints

Requires active "Platform Manager" badge.

### 5. Create New Territory

**Endpoint:** `POST /api/v1/territories/admin`  
**Authentication:** Bearer token required  
**Authorization:** Platform manager only  
**Status:** ⏳ Not Started

**Request:**

```json
{
  "code": "no",
  "name": "Norway",
  "display_name": "Norway Territory",
  "description": "Territory for Norway-based users",
  "pod_url": "https://norway.unityplatform.no",
  "api_url": "https://api.norway.unityplatform.no",
  "status": "active",
  "language_code": "no",
  "timezone": "Europe/Oslo",
  "currency_code": "NOK"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "territory": {
      "code": "no",
      "name": "Norway",
      "display_name": "Norway Territory",
      "description": "Territory for Norway-based users",
      "pod_url": "https://norway.unityplatform.no",
      "api_url": "https://api.norway.unityplatform.no",
      "status": "active",
      "language_code": "no",
      "timezone": "Europe/Oslo",
      "currency_code": "NOK",
      "created_at": "2025-11-13T10:00:00Z"
    }
  }
}
```

**Side Effects:**

- Inserts into `global.territories_registry`
- Publishes NATS event: `territory.created`

**Use Case:** Platform manager adding new territory

---

### 6. Update Territory Infrastructure

**Endpoint:** `PATCH /api/v1/territories/{code}/admin`  
**Authentication:** Bearer token required  
**Authorization:** Platform manager only  
**Status:** ⏳ Not Started

**Request:**

```json
{
  "pod_url": "https://denmark.unityplatform.org",
  "api_url": "https://api.denmark.unityplatform.org",
  "status": "maintenance"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "territory": {
      "code": "dk",
      "pod_url": "https://denmark.unityplatform.org",
      "api_url": "https://api.denmark.unityplatform.org",
      "status": "maintenance",
      "updated_at": "2025-11-13T11:00:00Z"
    }
  }
}
```

**Side Effects:**

- Updates `global.territories_registry` infrastructure fields
- Publishes NATS event: `territory.infrastructure.updated`

**Use Case:** Platform manager updating pod URLs or maintenance status

---

### 7. Delete Territory

**Endpoint:** `DELETE /api/v1/territories/{code}/admin`  
**Authentication:** Bearer token required  
**Authorization:** Platform manager only  
**Status:** ⏳ Not Started

**Validation:**

- Cannot delete if communities exist
- Cannot delete if territory managers assigned
- Can only delete if created by mistake

**Response (Success):**

```json
{
  "success": true,
  "data": {
    "message": "Territory 'test' deleted successfully"
  }
}
```

**Response (Error):**

```json
{
  "success": false,
  "error": {
    "code": "TERRITORY_HAS_DEPENDENCIES",
    "message": "Cannot delete territory: 2 communities exist and 3 managers assigned",
    "details": {
      "communities_count": 2,
      "managers_count": 3
    }
  }
}
```

**Side Effects:**

- Deletes from `global.territories_registry`
- Publishes NATS event: `territory.deleted`

**Use Case:** Platform manager removing mistakenly created territory

---

## Current Territories

- `dk` - Denmark 🇩🇰 (MVP - Active)

**Future Territories (Platform Manager Can Add):**

- Countries: NO, SE, US, CA, etc. (ISO 3166-1 Alpha-2)
- First Nations: HAIDA-FN-CA, NAVAJO-FN-US, SAMI-FN-NO (Sovereign)
- Communities: DK-COPENHAGEN, HAIDA-FN-CA-MASSETT (Nested)

---

**Last Updated:** November 13, 2025  
**Implementation Status:** 0/7 endpoints (0%)  
**Priority:** Phase 1 - Critical

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
