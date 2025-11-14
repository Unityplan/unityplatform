# badge-service API Endpoints

**Base URL:** `http://localhost:8007`  
**Version:** v1  
**Status:** ✅ Implemented (7/7 core endpoints) - Phase 1 Complete

---

## Badge Endpoints

### 1. List Badges

**Endpoint:** `GET /api/v1/badges`  
**Authentication:** Optional (shows user progress if authenticated)  
**Status:** ✅ Implemented

**Response:**

```json
[
  {
    "id": "uuid",
    "name": "Code of Conduct",
    "slug": "code-of-conduct",
    "description": "Agreed to platform code of conduct",
    "icon": "📜",
    "criteriaType": "manual",
    "criteriaValue": null,
    "rarity": "common",
    "isActive": true,
    "createdAt": "2025-11-14T10:00:00Z",
    "updatedAt": "2025-11-14T10:00:00Z",
    "userHasBadge": true,
    "userProgress": null,
    "userTarget": null
  }
]
```

**Notes:**

- Returns all active badges from `global.badge_registry`
- If authenticated, includes `userHasBadge`, `userProgress`, `userTarget` fields
- Sorted by rarity (legendary → epic → rare → common) then name

---

### 2. Get User Badges

**Endpoint:** `GET /api/v1/badges/users/{user_id}`  
**Authentication:** Bearer token required  
**Status:** ✅ Implemented

**Response:**

```json
[
  {
    "id": "uuid",
    "badgeId": "uuid",
    "badgeName": "Territory Manager",
    "badgeSlug": "territory-manager",
    "badgeIcon": "🌍",
    "badgeRarity": "epic",
    "awardedAt": "2025-11-14T10:00:00Z",
    "awardedBy": "uuid",
    "isFeatured": false
  }
]
```

---

### 3. Award Badge

**Endpoint:** `POST /api/v1/badges/award`  
**Authentication:** Admin/Platform Manager  
**Status:** ✅ Implemented

**Request:**

```json
{
  "userId": "uuid",
  "badgeSlug": "portal-manager",
  "reason": "Promoted to portal manager role"
}
```

**Response:**

```json
{
  "success": true,
  "message": "Badge awarded successfully"
}
```

**Errors:**

- `404` - Badge or user not found
- `409` - User already has this badge

---

### 4. Revoke Badge

**Endpoint:** `POST /api/v1/badges/revoke`  
**Authentication:** Admin/Platform Manager  
**Status:** ✅ Implemented

**Request:**

```json
{
  "userId": "uuid",
  "badgeSlug": "portal-manager",
  "reason": "Role change"
}
```

---

### 5. Update Progress

**Endpoint:** `POST /api/v1/badges/progress`  
**Authentication:** Internal (service-to-service)  
**Status:** ✅ Implemented

**Request:**

```json
{
  "userId": "uuid",
  "badgeSlug": "course-completion-10",
  "currentValue": 8
}
```

**Notes:**

- Auto-awards badge when currentValue reaches criteriaValue
- Used by course-service, forum-service, etc. to track progress

---

### 6. Toggle Featured Badge

**Endpoint:** `PATCH /api/v1/badges/featured`  
**Authentication:** Bearer token required (own badges only)  
**Status:** ✅ Implemented

**Request:**

```json
{
  "badgeId": "uuid",
  "isFeatured": true
}
```

**Notes:**

- Users can feature/unfeature their own badges
- Featured badges displayed prominently on profile

---

### 7. Register Badge (Service-to-Service)

**Endpoint:** `POST /api/v1/badges/register`  
**Authentication:** Service-to-service (no auth required)  
**Status:** ✅ Implemented

**Request:**

```json
{
  "slug": "territory-manager",
  "name": "Territory Manager",
  "description": "Grants full management access to territory",
  "icon": "🌍",
  "category": "role",
  "criteriaType": "manual",
  "criteriaValue": null,
  "rarity": "epic",
  "isRenewable": false,
  "renewalDays": null,
  "grantsPermissions": [
    "territory:manage",
    "territory:settings:manage"
  ]
}
```

**Response:**

```json
{
  "id": "uuid",
  "slug": "territory-manager",
  "name": "Territory Manager",
  "description": "Grants full management access to territory",
  "icon": "�",
  "criteriaType": "manual",
  "criteriaValue": null,
  "rarity": "epic",
  "isActive": true,
  "createdAt": "2025-11-14T10:00:00Z",
  "updatedAt": "2025-11-14T10:00:00Z"
}
```

**Notes:**

- Idempotent - returns existing badge if already registered
- Used by services to register their role badges on startup
- `grantsPermissions` array defines permissions granted by badge
- See `docs/guides/development/permission-system-usage.md` for details

---

## Badge Categories

- **role** - Management and access roles (Territory Manager, Portal Manager)
- **achievement** - User accomplishments (Course completion, badges earned)
- **code_of_conduct** - Platform agreements (Code of Conduct acceptance)
- **special** - Limited/event badges (Early Adopter, Beta Tester)

## Permission-Granting Badges

Badges in the `role` category can grant permissions used by the permission middleware:

**Portal Service Badges:**

- 🏛️ Portal Manager - `portal:*` (all portal permissions)
- 💻 Portal Developer - `portal:services:develop`, `portal:services:deploy`
- 🌐 Portal Translator - `portal:translations:edit`
- ⚙️ Portal Infrastructure Manager - `portal:infrastructure:manage`
- 🧪 Portal Tester - `portal:testing:access`

**Territory Service Badge:**

- 🌍 Territory Manager - `territory:manage`, `territory:settings:manage`

**See:** `docs/examples/portal-service-badges.json` for complete definitions

---

## Initial Badges (Seeded)

- Platform Manager 👑 (manual, epic)
- Territory Manager 🌍 (manual, epic)
- Code of Conduct 📜 (manual, common)
- Early Adopter 🌟 (manual, rare)
- Community Builder 🏘️ (manual, rare)
- Course Instructor 👨‍🏫 (manual, rare)
- Beta Tester 🧪 (manual, common)

---

**Last Updated:** November 14, 2025  
**Implementation Status:** 7/7 endpoints (100%) ✅  
**Priority:** Phase 1 Complete
