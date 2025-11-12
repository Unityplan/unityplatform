# territory-service Database Schema

**Service:** territory-service  
**Port:** 8008  
**Database:** Global schema only

---

## Table

### global.territories

**Status:** ✅ Exists in 20251111000001_mvp_core_schema.sql

```sql
CREATE TABLE global.territories (
    code VARCHAR(10) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    
    -- Pod Configuration
    pod_url VARCHAR(255) NOT NULL,
    api_url VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    
    -- Metadata
    language_code VARCHAR(10) NOT NULL,
    timezone VARCHAR(50) NOT NULL,
    currency_code VARCHAR(3),
    
    -- Holochain DNA (Future)
    dna_hash VARCHAR(128),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (status IN ('active', 'maintenance', 'inactive'))
);
```

## Why Global Schema?

Territory registry is **global metadata** - defines available pods. All pods need to know about all other pods for federation.

## Territory Stats (Future Table)

```sql
CREATE TABLE global.territory_stats (
    territory_code VARCHAR(10) PRIMARY KEY REFERENCES global.territories(code),
    
    total_users BIGINT NOT NULL DEFAULT 0,
    active_users_7d BIGINT NOT NULL DEFAULT 0,
    active_users_30d BIGINT NOT NULL DEFAULT 0,
    
    total_communities BIGINT NOT NULL DEFAULT 0,
    total_posts BIGINT NOT NULL DEFAULT 0,
    
    storage_used_mb BIGINT NOT NULL DEFAULT 0,
    
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## NATS Events

**Publishes:** `territory.created`, `territory.updated`, `territory.deactivated`

---

**Last Updated:** November 12, 2025
