# territory-service

**Port:** 8008  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Scaffolded (0/4 endpoints) - Phase 2  
**Bounded Context:** Territory & Pod Management

---

## 📋 Overview

The territory-service manages territories (pods), their settings, statistics, and inter-pod federation.

### **Responsibilities**

- ⏳ List available territories (pods)
- ⏳ Territory details and settings
- ⏳ Territory statistics (users, communities, activity)
- ⏳ Inter-territory federation (future)
- ⏳ Territory health monitoring

### **Not Responsible For**

- ❌ User registration (handled by auth-service per territory)
- ❌ Content management (handled by respective services)
- ❌ Infrastructure management (handled by DevOps)

---

## 🗄️ Database Schema

### **Tables Owned by territory-service**

#### **1. territories (global table)**

```sql
CREATE TABLE global.territories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Identity
    code VARCHAR(10) UNIQUE NOT NULL,  -- dk, no, se, eu
    name VARCHAR(255) NOT NULL,  -- Denmark, Norway, Sweden, Europe
    display_name VARCHAR(255) NOT NULL,  -- Denmark 🇩🇰
    
    -- Contact
    domain VARCHAR(255),  -- denmark.unityplatform.org
    admin_email VARCHAR(255),
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    is_accepting_registrations BOOLEAN DEFAULT true,
    
    -- Metadata
    description TEXT,
    flag_emoji VARCHAR(10),  -- 🇩🇰
    timezone VARCHAR(100) DEFAULT 'UTC',
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_territories_code ON territories(code);
CREATE INDEX idx_territories_active ON territories(is_active) WHERE is_active = true;

-- Seed data
INSERT INTO territories (code, name, display_name, flag_emoji, domain, timezone) VALUES
('dk', 'Denmark', 'Denmark 🇩🇰', '🇩🇰', 'denmark.unityplatform.org', 'Europe/Copenhagen'),
('no', 'Norway', 'Norway 🇳🇴', '🇳🇴', 'norway.unityplatform.org', 'Europe/Oslo'),
('se', 'Sweden', 'Sweden 🇸🇪', '🇸🇪', 'sweden.unityplatform.org', 'Europe/Stockholm'),
('eu', 'Europe', 'Europe 🇪🇺', '🇪🇺', 'europe.unityplatform.org', 'Europe/Brussels');
```

**Purpose:** Global registry of all territories (pods)  
**Holochain Entry Type:** `Territory` (public DHT - federated network)

#### **2. territory_settings (per-territory)**

```sql
CREATE TABLE territory_{code}.territory_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
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
