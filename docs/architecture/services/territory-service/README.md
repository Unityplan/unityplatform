# territory-service

**Port:** 8008  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Not Started (0/7 endpoints) - Phase 1  
**Bounded Context:** Territory & Pod Management

---

## 📋 Overview

The territory-service manages the global territory registry, per-territory settings, statistics, and territory manager assignments. It serves both authenticated users, territory managers, and platform managers.

### **Responsibilities**

- ✅ Manage global territory registry (`global.territories_registry`)
- ✅ List available territories for authenticated users
- ✅ Territory details and settings (per-territory schemas)
- ✅ Territory statistics aggregation
- ✅ Territory manager assignment tracking
- ✅ Settings replication from pod to global registry
- ✅ Platform manager infrastructure management
- ⏳ Inter-territory federation (Phase 2)

### **Not Responsible For**

- ❌ User registration (handled by auth-service per territory)
- ❌ Content management (handled by respective services)
- ❌ Infrastructure deployment (handled by DevOps)
- ❌ Assigning "Territory Manager" badges (handled by badge-service)
- ❌ Assigning "Platform Manager" badges (handled by badge-service)
- ❌ User authentication (handled by auth-service)
- ❌ Calculating statistics (aggregated from other services)

---

## 🗄️ Database Schema

### **Tables Owned by territory-service**

#### **1. territories_registry (global table)**

**Column Ownership Model:**

**Platform Manager Columns (Infrastructure - Platform Manager Only):**

- `code` - Territory ID (Primary Key)
- `pod_url` - Infrastructure endpoint (e.g., <https://denmark.unityplatform.dk>)
- `api_url` - API endpoint (e.g., <https://api.denmark.unityplatform.dk>)
- `status` - active/maintenance/inactive

**Territory Manager Columns (Sovereignty - Replicated from Pod Settings):**

- `name` - Territory name (replicated from `territory_{code}.territory_settings.name`)
- `display_name` - Display name (replicated from `territory_{code}.territory_settings.display_name`)
- `description` - Description (replicated from `territory_{code}.territory_settings.description`)
- `language_code` - Default language (replicated from `territory_{code}.territory_settings.language_code`)
- `timezone` - Default timezone (replicated from `territory_{code}.territory_settings.timezone`)
- `currency_code` - Default currency (replicated from `territory_{code}.territory_settings.currency_code`)

```sql
CREATE TABLE global.territories_registry (
    -- Primary Key (Platform Manager)
    code VARCHAR(10) PRIMARY KEY,
    
    -- Infrastructure (Platform Manager Only)
    pod_url VARCHAR(255) NOT NULL,
    api_url VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    
    -- Territory Identity (Replicated from Pod)
    name VARCHAR(100) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    
    -- Localization (Replicated from Pod)
    language_code VARCHAR(10) NOT NULL,
    timezone VARCHAR(50) NOT NULL,
    currency_code VARCHAR(3),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (status IN ('active', 'maintenance', 'inactive')),
    CHECK (char_length(code) >= 2 AND char_length(code) <= 10)
);

CREATE INDEX idx_territories_registry_status ON global.territories_registry(status);

-- Seed data (Denmark only for MVP)
INSERT INTO global.territories_registry (
    code, name, display_name, description,
    pod_url, api_url, status,
    language_code, timezone, currency_code
) VALUES (
    'dk',
    'Denmark',
    'Denmark Territory',
    'Primary territory for Denmark-based users',
    'https://denmark.unityplatform.dk',
    'https://api.denmark.unityplatform.dk',
    'active',
    'da',
    'Europe/Copenhagen',
    'DKK'
) ON CONFLICT (code) DO NOTHING;
```

**Purpose:** Global registry of all territories across all pods  
**Access:** Read-only for most services, read-write for territory-service  
**Replication:** Pod settings changes replicate to this table via NATS events

---

#### **2. territory_settings (per-territory)**

**Source of Truth for Territory-Specific Configuration**

```sql
CREATE TABLE territory_{code}.territory_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Identity (SOURCE OF TRUTH - replicates to global.territories_registry)
    name VARCHAR(100) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    
    -- Localization (SOURCE OF TRUTH - replicates to global.territories_registry)
    language_code VARCHAR(10) NOT NULL,
    timezone VARCHAR(50) NOT NULL,
    currency_code VARCHAR(3),
    
    -- Registration
    registration_enabled BOOLEAN DEFAULT true,
    invitation_required BOOLEAN DEFAULT true,
    max_users INT DEFAULT 0,  -- 0 = unlimited
    
    -- Features
    features_enabled JSONB DEFAULT '{
        "communities": true,
        "badges": true,
        "events": false,
        "courses": false,
        "forum": false
    }'::jsonb,
    
    -- Branding
    primary_color VARCHAR(7) DEFAULT '#2E7D32',  -- Forest green
    logo_url VARCHAR(500),
    
    -- Timestamps
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Seed default settings for Denmark
INSERT INTO territory_dk.territory_settings (
    name, display_name, description,
    language_code, timezone, currency_code
) VALUES (
    'Denmark',
    'Denmark Territory',
    'Primary territory for Denmark-based users',
    'da',
    'Europe/Copenhagen',
    'DKK'
) ON CONFLICT DO NOTHING;
```

**Purpose:** Per-territory configuration (source of truth)  
**Managed By:** Territory managers (users with active "Territory Manager" badge)  
**Replication:** Changes propagate to `global.territories_registry`

---

#### **3. territory_managers (per-territory)**

**Territory Manager Assignment Tracking**

```sql
CREATE TABLE territory_{code}.territory_managers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Assignment
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id),
    territory_code VARCHAR(10) NOT NULL,
    
    -- Audit Trail
    assigned_by UUID,  -- Platform manager who assigned this user
    assigned_at TIMESTAMPTZ DEFAULT NOW(),
    notes TEXT,
    
    -- Constraints
    UNIQUE(user_id, territory_code)
);

CREATE INDEX idx_territory_managers_user ON territory_{code}.territory_managers(user_id);
CREATE INDEX idx_territory_managers_territory ON territory_{code}.territory_managers(territory_code);
```

**Purpose:** Track which users are assigned as territory managers  
**Managed By:** Platform managers (users with active "Platform Manager" badge)  
**Authorization:** User must have BOTH:

  1. Active "Territory Manager" badge (from badge-service)
  2. Assignment in this table for the specific territory

**Note:** If badge is removed/expired, permissions are DISABLED but assignment remains

---

#### **4. territory_stats (per-territory)**

**Aggregated Territory Statistics**

```sql
CREATE TABLE territory_{code}.territory_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- User Metrics
    total_users INT DEFAULT 0,
    active_users_7d INT DEFAULT 0,
    active_users_30d INT DEFAULT 0,
    
    -- Community Metrics
    total_communities INT DEFAULT 0,
    total_posts INT DEFAULT 0,
    
    -- Storage Metrics
    storage_used_mb BIGINT DEFAULT 0,
    
    -- Timestamp
    calculated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Seed initial stats for Denmark
INSERT INTO territory_dk.territory_stats (
    total_users, active_users_7d, total_communities
) VALUES (0, 0, 0) ON CONFLICT DO NOTHING;
```

**Purpose:** Territory statistics aggregation  
**Managed By:** Territory-service (aggregates from other services)  
**Updated:** Daily via background job or on-demand calculation  
**Access:** Territory managers can view their territory stats

---

## 🔌 API Endpoints

### **Public Endpoints (Authenticated Users)**

All users with valid JWT tokens can access these endpoints.

#### **1. List All Active Territories**

**Endpoint:** `GET /api/v1/territories`  
**Authentication:** Bearer token required  
**Authorization:** Any authenticated user  
**Status:** ⏳ Not Started

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

**Use Case:** Users viewing available territories they can interact with

---

#### **2. Get Territory Details**

**Endpoint:** `GET /api/v1/territories/{code}`  
**Authentication:** Bearer token required  
**Authorization:** Any authenticated user  
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

### **Territory Manager Endpoints**

Requires active "Territory Manager" badge AND assignment to the territory.

#### **3. Get Territory Statistics**

**Endpoint:** `GET /api/v1/territories/{code}/manage/stats`  
**Authentication:** Bearer token required  
**Authorization:** Territory manager for this territory  
**Status:** ⏳ Not Started

**Authorization Check:**

1. Extract user_id from JWT
2. Check if user has active "Territory Manager" badge
3. Check if user is assigned to this territory in `territory_{code}.territory_managers`

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

**Use Case:** Territory manager dashboard showing detailed metrics

---

#### **4. Update Territory Settings**

**Endpoint:** `PATCH /api/v1/territories/{code}/manage/settings`  
**Authentication:** Bearer token required  
**Authorization:** Territory manager for this territory  
**Status:** ⏳ Not Started

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

1. Updates `territory_{code}.territory_settings` (source of truth)
2. Replicates sovereignty fields to `global.territories_registry`:
   - name, display_name, description
   - language_code, timezone, currency_code
3. Publishes NATS event: `territory.settings.updated`

**Use Case:** Territory manager updating their territory configuration

---

### **Platform Manager Endpoints**

Requires active "Platform Manager" badge.

#### **5. Create New Territory**

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

1. Inserts into `global.territories_registry`
2. Publishes NATS event: `territory.created`
3. **Note:** Does NOT create schema (DevOps handles pod deployment)

**Use Case:** Platform manager adding new territory to the registry

---

#### **6. Update Territory Infrastructure**

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

1. Updates infrastructure fields in `global.territories_registry`
2. Publishes NATS event: `territory.infrastructure.updated`

**Use Case:** Platform manager updating pod URLs or setting maintenance mode

---

#### **7. Delete Territory**

**Endpoint:** `DELETE /api/v1/territories/{code}/admin`  
**Authentication:** Bearer token required  
**Authorization:** Platform manager only  
**Status:** ⏳ Not Started

**Validation:**

- ❌ Cannot delete if communities exist in this territory
- ❌ Cannot delete if territory managers are assigned
- ✅ Can delete if created by mistake (no data, no managers)

**Response (Success):**

```json
{
  "success": true,
  "data": {
    "message": "Territory 'test' deleted successfully"
  }
}
```

**Response (Validation Error):**

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

1. Deletes from `global.territories_registry`
2. Publishes NATS event: `territory.deleted`

**Use Case:** Platform manager removing mistakenly created territory

---

## 🔐 Authorization System

### Territory Manager Authorization

**Requirements:** User must have BOTH:

1. **Active "Territory Manager" badge** (checked via badge-service)
2. **Assignment to territory** (checked in `territory_{code}.territory_managers`)

**Flow:**

```rust
async fn verify_territory_manager(
    user_id: Uuid,
    territory_code: &str,
    db: &Database
) -> Result<bool> {
    // 1. Check badge: Does user have active "Territory Manager" badge?
    let has_badge = check_territory_manager_badge(user_id, db).await?;
    
    if !has_badge {
        return Ok(false); // Badge removed/expired - permissions disabled
    }
    
    // 2. Check assignment: Is user assigned to this territory?
    let query = format!(
        "SELECT EXISTS(
            SELECT 1 FROM territory_{}.territory_managers
            WHERE user_id = $1 AND territory_code = $2
        )",
        territory_code
    );
    
    let assigned: bool = sqlx::query_scalar(&query)
        .bind(user_id)
        .bind(territory_code)
        .fetch_one(db.pool())
        .await?;
    
    Ok(assigned)
}

async fn check_territory_manager_badge(user_id: Uuid, db: &Database) -> Result<bool> {
    // Query badge-service tables to verify active badge
    // This will be implemented when badge-service is created
    // For now, assume badge exists if assignment exists
    Ok(true)
}
```

**Badge States:**

- ✅ **Badge Active + Assignment Exists** → Full territory manager permissions
- ❌ **Badge Removed + Assignment Exists** → Permissions DISABLED (assignment retained)
- ✅ **Badge Reinstated + Assignment Exists** → Permissions REACTIVATED
- ❌ **Badge Active + No Assignment** → No territory manager permissions

### Platform Manager Authorization

**Requirements:**

1. **Active "Platform Manager" badge** (checked via badge-service)

**Flow:**

```rust
async fn verify_platform_manager(user_id: Uuid, db: &Database) -> Result<bool> {
    // Check badge service: Does user have active "Platform Manager" badge?
    check_platform_manager_badge(user_id, db).await
}

async fn check_platform_manager_badge(user_id: Uuid, db: &Database) -> Result<bool> {
    // Query badge-service tables to verify active badge
    // This will be implemented when badge-service is created
    Ok(true) // Placeholder
}
```

---

## 📡 NATS Events

### Published Events

**1. territory.created**

```json
{
  "event": "territory.created",
  "timestamp": "2025-11-13T10:00:00Z",
  "data": {
    "code": "no",
    "name": "Norway",
    "pod_url": "https://norway.unityplatform.no",
    "api_url": "https://api.norway.unityplatform.no"
  }
}
```

**2. territory.settings.updated**

```json
{
  "event": "territory.settings.updated",
  "timestamp": "2025-11-13T10:30:00Z",
  "data": {
    "code": "dk",
    "name": "Denmark",
    "display_name": "Kingdom of Denmark",
    "description": "Updated description",
    "language_code": "da",
    "timezone": "Europe/Copenhagen",
    "currency_code": "DKK"
  }
}
```

**3. territory.infrastructure.updated**

```json
{
  "event": "territory.infrastructure.updated",
  "timestamp": "2025-11-13T11:00:00Z",
  "data": {
    "code": "dk",
    "pod_url": "https://denmark.unityplatform.org",
    "api_url": "https://api.denmark.unityplatform.org",
    "status": "maintenance"
  }
}
```

**4. territory.deleted**

```json
{
  "event": "territory.deleted",
  "timestamp": "2025-11-13T12:00:00Z",
  "data": {
    "code": "test"
  }
}
```

---

## 🔄 Data Replication Flow

### Settings Update Flow (Territory Manager → Global Registry)

**When territory manager updates settings:**

1. **Update pod settings** (source of truth):

   ```sql
   UPDATE territory_dk.territory_settings
   SET name = $1, display_name = $2, description = $3,
       language_code = $4, timezone = $5, currency_code = $6,
       updated_at = NOW()
   WHERE id = (SELECT id FROM territory_dk.territory_settings LIMIT 1)
   ```

2. **Replicate to global registry**:

   ```sql
   UPDATE global.territories_registry
   SET name = $1, display_name = $2, description = $3,
       language_code = $4, timezone = $5, currency_code = $6,
       updated_at = NOW()
   WHERE code = 'dk'
   ```

3. **Publish NATS event** for federation:

   ```rust
   nats.publish("territory.settings.updated", TerritorySettingsUpdated {
       code: "dk",
       name, display_name, description,
       language_code, timezone, currency_code,
   }).await?;
   ```

4. **Other pods receive event** and update their local copy of `global.territories_registry`

**Consistency Model:**

- Pod settings table = **Source of Truth**
- Global registry = **Replicated Cache** (updated synchronously on same pod)
- Other pods = **Eventually Consistent** (updated via NATS events)

---

## 🚀 Implementation Checklist

### Database

- [ ] Migration: Rename `global.territories` → `global.territories_registry`
- [ ] Migration: Create `territory_{code}.territory_settings`
- [ ] Migration: Create `territory_{code}.territory_managers`
- [ ] Migration: Create `territory_{code}.territory_stats`
- [ ] Migration: Seed default settings for Denmark

### Service Structure

- [ ] Scaffold `services/territory-service`
- [ ] Create models (requests/responses)
- [ ] Create handlers (7 endpoints)
- [ ] Create services (business logic)
- [ ] Add to Docker Compose

### Endpoints

**Public (2):**

- [ ] `GET /api/v1/territories` - List active territories
- [ ] `GET /api/v1/territories/{code}` - Get territory details

**Territory Manager (2):**

- [ ] `GET /api/v1/territories/{code}/manage/stats` - Get stats
- [ ] `PATCH /api/v1/territories/{code}/manage/settings` - Update settings

**Platform Manager (3):**

- [ ] `POST /api/v1/territories/admin` - Create territory
- [ ] `PATCH /api/v1/territories/{code}/admin` - Update infrastructure
- [ ] `DELETE /api/v1/territories/{code}/admin` - Delete territory

### Authorization

- [ ] Implement territory manager badge verification
- [ ] Implement territory manager assignment verification
- [ ] Implement platform manager badge verification
- [ ] Add middleware for role-based access

### Testing

- [ ] Test public endpoints (authenticated users)
- [ ] Test territory manager endpoints (with/without badge)
- [ ] Test platform manager endpoints (with/without badge)
- [ ] Test settings replication (pod → global registry)
- [ ] Test NATS event publishing

---

**Last Updated:** November 13, 2025  
**Implementation Status:** 0/7 endpoints (0%)  
**Priority:** Phase 1 - Critical for frontend territory info pages

**Managed By:** Platform managers (users with active "Platform Manager" badge)  
**Authorization:** User must have BOTH:

  1. Active "Territory Manager" badge (from badge-service)
  2. Assignment in this table for the specific territory

**Note:** If badge is removed/expired, permissions are DISABLED but assignment remains

---

#### **4. territory_stats (per-territory)**

**Aggregated Territory Statistics**
    -- Timestamps
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

```

**Purpose:** Territory-specific settings and feature flags  
**Holochain:** Not migrated (territory configuration stays on pod)

#### **3. territory_stats (per-territory)**

```sql
CREATE TABLE territory_{code}.territory_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Stats
    total_users INT DEFAULT 0,
    active_users_7d INT DEFAULT 0,  -- Active in last 7 days
    total_communities INT DEFAULT 0,
    total_posts INT DEFAULT 0,
    
    -- Storage
    storage_used_mb BIGINT DEFAULT 0,
    
    -- Timestamp
    calculated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Purpose:** Territory statistics (updated daily via cron job)  
**Holochain:** Not migrated (local metrics)

---

## 🔌 API Endpoints

### **1. List Territories**

#### **GET /v1/territories**

Get all active territories (public endpoint, no auth required)

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
        "domain": "denmark.unityplatform.org",
        "is_accepting_registrations": true,
        "flag_emoji": "🇩🇰",
        "timezone": "Europe/Copenhagen"
      },
      {
        "code": "no",
        "name": "Norway",
        "display_name": "Norway 🇳🇴",
        "domain": "norway.unityplatform.org",
        "is_accepting_registrations": true,
        "flag_emoji": "🇳🇴",
        "timezone": "Europe/Oslo"
      }
    ]
  }
}
```

**Use Case:** Frontend territory selector during registration

---

### **2. Get Territory Details**

#### **GET /v1/territories/{code}**

Get detailed information about a territory

**Response:**

```json
{
  "success": true,
  "data": {
    "territory": {
      "code": "dk",
      "name": "Denmark",
      "display_name": "Denmark 🇩🇰",
      "description": "Unity Platform pod for Denmark",
      "domain": "denmark.unityplatform.org",
      "admin_email": "admin@denmark.unityplatform.org",
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

### **3. Update Territory Settings**

#### **PATCH /v1/territories/{code}/settings**

Update territory settings (admin only)

**Request:**

```json
{
  "registration_enabled": false,
  "features_enabled": {
    "events": true
  }
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "updated": true
  }
}
```

---

### **4. Get Territory Stats**

#### **GET /v1/territories/{code}/stats**

Get current territory statistics

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

## 🔗 Service Dependencies

### **Outbound Calls**

None - territory-service is a **leaf service** (reads global data)

---

### **Inbound Calls**

#### **Frontend**

- **When:** Registration, territory selection
- **Endpoints:** GET /v1/territories (public)
- **Purpose:** Display available territories

#### **Admin Dashboard**

- **Endpoints:** All territory endpoints
- **Purpose:** Territory management, monitoring

#### **Other Services** (future)

- **When:** Cross-territory operations (federation)
- **Purpose:** Verify territory exists, check federation rules

---

## 📡 NATS Events

### **Published Events**

```typescript
// Territory stats updated
{
  event: "territory.stats_updated",
  territory_code: "dk",
  stats: {
    total_users: 150,
    total_communities: 12
  },
  timestamp: "ISO8601"
}
```

### **Subscribed Events**

```typescript
// User registered (increment territory user count)
{
  event: "user.registered",
  user_id: "uuid",
  territory_code: "dk"
}

// Community created (increment territory community count)
{
  event: "community.created",
  community_id: "uuid",
  territory_code: "dk"
}
```

---

## 🔮 Holochain Migration

### **DNA Design: federation.happ**

#### **Entry Types**

**1. Territory** (Public DHT - federated)

```rust
#[hdk_entry_helper]
struct Territory {
    code: String,
    name: String,
    display_name: String,
    domain: String,
    admin_pub_key: AgentPubKey,
    is_active: bool,
    is_accepting_registrations: bool,
    created_at: Timestamp,
}
```

#### **Links**

```rust
// Global territory registry
Path::from("territories") → Territory

// Territory → Admin
Territory → AgentPubKey (admin)
```

#### **Federation Logic**

```rust
// Territories announce themselves to the DHT
// Other territories discover them via DHT queries
// Users can move between territories with their data
// Content can be federated across territories
```

#### **Key Concept: Multi-Pod DHT**

Each territory runs its own pod (separate PostgreSQL, services), but they all connect to the same Holochain DHT. This creates a **federated network** where:

- Users are sovereign (control their data)
- Territories are autonomous (independent infrastructure)
- Network is unified (shared DHT for discovery and federation)

#### **Migration Strategy**

**Phase 1:** PostgreSQL storage

- Current implementation with PostgreSQL
- Territories managed in global table

**Phase 2:** Dual storage (PostgreSQL + Holochain)

- Territories announce to DHT
- PostgreSQL for fast queries
- Holochain for federation

**Phase 3:** Holochain federation

- Full inter-territory federation
- User migration between territories
- Content sharing across territories

---

## ✅ Implementation Status

### **Completed**

- ✅ Database schema designed
- ✅ Service scaffolded
- ✅ Seed data prepared

### **Pending** (Phase 2)

- ⏳ All 4 endpoints
- ⏳ Stats calculation (cron job)
- ⏳ Territory health monitoring
- ⏳ Admin dashboard integration
- ⏳ Tests

---

## 🌍 Territory Architecture

### **Current Territories**

```
┌─────────────────────────────────────────┐
│ GLOBAL LAYER (shared across all pods)  │
│ - territories table                     │
│ - username_registry                     │
│ - email_registry                        │
│ - invitation_token_registry             │
└─────────────────────────────────────────┘

┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│ Denmark  │  │ Norway   │  │ Sweden   │  │ Europe   │
│ Pod (dk) │  │ Pod (no) │  │ Pod (se) │  │ Pod (eu) │
│          │  │          │  │          │  │          │
│ Users    │  │ Users    │  │ Users    │  │ Users    │
│ Services │  │ Services │  │ Services │  │ Services │
│ Data     │  │ Data     │  │ Data     │  │ Data     │
└──────────┘  └──────────┘  └──────────┘  └──────────┘
```

### **Future: Holochain Federation**

```
┌─────────────────────────────────────────┐
│ HOLOCHAIN DHT (global, decentralized)   │
│ - Territory registry                    │
│ - User public keys                      │
│ - Federation rules                      │
└─────────────────────────────────────────┘
         ↓         ↓         ↓         ↓
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│ Denmark  │  │ Norway   │  │ Sweden   │  │ Anyone   │
│ Pod      │←→│ Pod      │←→│ Pod      │←→│ Can Run  │
│          │  │          │  │          │  │ A Pod!   │
└──────────┘  └──────────┘  └──────────┘  └──────────┘
```

---

## 📊 Stats Calculation

### **Daily Cron Job**

```rust
// Calculate territory stats (runs daily at 00:00 UTC)
async fn calculate_territory_stats(territory_code: &str) {
    let total_users = query_count("users", territory_code).await;
    let active_users_7d = query_active_users(7, territory_code).await;
    let total_communities = query_count("communities", territory_code).await;
    let total_posts = query_count("forum_posts", territory_code).await;
    let storage_used_mb = calculate_storage(territory_code).await;
    
    // Insert into territory_stats
    insert_stats(territory_code, stats).await;
    
    // Publish NATS event
    publish_event("territory.stats_updated", stats).await;
}
```

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Phase 2 (Infrastructure layer)  
**Dependencies:** None (reads global data)  
**Future:** Holochain federation enables anyone to run a pod
