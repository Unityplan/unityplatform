# territory-service Database Schema

**Service:** territory-service  
**Port:** 8008  
**Version:** 0.1.0-alpha.1  
**Database:** Global + Per-Territory Schemas

---

## 📋 Overview

Territory-service owns and manages:

1. **global.territories_registry** - Global territory registry (all pods)
2. **territory_{code}.territory_settings** - Per-territory configuration (pod-local)
3. **territory_{code}.territory_managers** - Territory manager assignments (pod-local)
4. **territory_{code}.territory_stats** - Territory statistics (pod-local, read-only)

---

## 🗄️ Global Tables

### global.territories_registry

**Status:** ⏳ Rename from global.territories in Migration 20251113000005

**Column Ownership:**

**Platform Manager Columns (Infrastructure):**

- `code` - Territory ID (Primary Key)
- `pod_url` - Infrastructure endpoint
- `api_url` - API endpoint
- `status` - active/maintenance/inactive

**Territory Manager Columns (Sovereignty - Replicated from Pod):**

- `name` - Territory name
- `display_name` - Display name
- `description` - Description
- `language_code` - Default language
- `timezone` - Default timezone
- `currency_code` - Default currency

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

CREATE INDEX idx_territories_registry_status 
    ON global.territories_registry(status);

COMMENT ON TABLE global.territories_registry IS 
    'Global territory registry. Platform managers manage infrastructure columns, territory managers manage sovereignty columns (replicated from pod).';
```

**Why Global:**
Territory registry is global metadata for federation - all pods need to know all territories for routing and discovery.

---

## 🗄️ Per-Territory Tables

### territory_{code}.territory_settings

**Status:** ⏳ Create in Migration 20251113000005

**Purpose:** Source of truth for territory-specific configuration

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
    max_users INT DEFAULT 0,
    
    -- Features
    features_enabled JSONB DEFAULT '{
        "communities": true,
        "badges": true,
        "events": false,
        "courses": false,
        "forum": false
    }'::jsonb,
    
    -- Branding
    primary_color VARCHAR(7) DEFAULT '#2E7D32',
    logo_url VARCHAR(500),
    
    -- Timestamps
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Trigger: Replicate to global registry
CREATE OR REPLACE FUNCTION territory_{code}.replicate_settings_to_global()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE global.territories_registry
    SET 
        name = NEW.name,
        display_name = NEW.display_name,
        description = NEW.description,
        language_code = NEW.language_code,
        timezone = NEW.timezone,
        currency_code = NEW.currency_code,
        updated_at = NOW()
    WHERE code = '{code}';
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_replicate_settings
AFTER UPDATE ON territory_{code}.territory_settings
FOR EACH ROW
EXECUTE FUNCTION territory_{code}.replicate_settings_to_global();
```

---

### territory_{code}.territory_managers

**Status:** ⏳ Create in Migration 20251113000005

**Purpose:** Track territory manager assignments

```sql
CREATE TABLE territory_{code}.territory_managers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Assignment
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id),
    territory_code VARCHAR(10) NOT NULL,
    
    -- Audit Trail
    assigned_by UUID,
    assigned_at TIMESTAMPTZ DEFAULT NOW(),
    notes TEXT,
    
    UNIQUE(user_id, territory_code)
);

CREATE INDEX idx_territory_managers_user 
    ON territory_{code}.territory_managers(user_id);
CREATE INDEX idx_territory_managers_territory 
    ON territory_{code}.territory_managers(territory_code);
```

**Authorization:** User needs BOTH:

1. Active "Territory Manager" badge (badge-service)
2. Assignment in this table

---

### territory_{code}.territory_stats

**Status:** ⏳ Create in Migration 20251113000005

**Purpose:** Aggregated statistics

```sql
CREATE TABLE territory_{code}.territory_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    total_users INT DEFAULT 0,
    active_users_7d INT DEFAULT 0,
    active_users_30d INT DEFAULT 0,
    total_communities INT DEFAULT 0,
    total_posts INT DEFAULT 0,
    storage_used_mb BIGINT DEFAULT 0,
    
    calculated_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## 📡 NATS Events

**Published by territory-service:**

- `territory.created`
- `territory.settings.updated`
- `territory.infrastructure.updated`
- `territory.deleted`

---

**Last Updated:** November 13, 2025  
**Migration:** 20251113000005 (pending)
