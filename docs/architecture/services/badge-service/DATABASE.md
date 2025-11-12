# badge-service Database Schema

**Service:** badge-service  
**Port:** 8007  
**Database:** Global + Territory schemas

---

## Schema Distribution

### Global Schema

**Table:** `global.badge_definitions` (Future)  
**Purpose:** Shared badge catalog across all pods

```sql
CREATE TABLE global.badge_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    
    icon VARCHAR(50),
    rarity VARCHAR(20) NOT NULL,
    
    criteria JSONB NOT NULL,
    
    is_active BOOLEAN NOT NULL DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (rarity IN ('common', 'rare', 'epic', 'legendary'))
);
```

**Why Global?** Badge definitions (types) shared across territories.

---

### Territory Schema

**Table:** `territory_{code}.badge_awards`  
**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.badge_awards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    badge_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    awarded_by UUID REFERENCES territory_{code}.users(id) ON DELETE SET NULL,
    
    reason TEXT,
    
    is_featured BOOLEAN NOT NULL DEFAULT false,
    
    awarded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (badge_id, user_id)
);
```

**Table:** `territory_{code}.badge_progress`

```sql
CREATE TABLE territory_{code}.badge_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    badge_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    current_value INT NOT NULL DEFAULT 0,
    target_value INT NOT NULL,
    
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (badge_id, user_id)
);
```

## Multi-Pod Strategy

**Badge Definitions:** Global (shared catalog)  
**Badge Awards:** Territory (user's personal achievements)

## NATS Events

**Publishes:** `badge.awarded`, `badge.progress_updated`

---

**Last Updated:** November 12, 2025
