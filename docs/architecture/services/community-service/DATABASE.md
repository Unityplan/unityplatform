# community-service Database Schema

**Service:** community-service  
**Port:** 8006  
**Database:** Territory schema only

---

## Tables

### territory_{code}.communities

**Status:** ✅ Exists in 20251111000001_mvp_core_schema.sql

```sql
CREATE TABLE territory_{code}.communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    
    created_by UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    visibility VARCHAR(20) NOT NULL DEFAULT 'public',
    require_approval BOOLEAN NOT NULL DEFAULT false,
    
    member_count INT NOT NULL DEFAULT 0,
    
    avatar_url VARCHAR(500),
    banner_url VARCHAR(500),
    
    tags TEXT[],
    
    is_active BOOLEAN NOT NULL DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (visibility IN ('public', 'private', 'hidden'))
);
```

### territory_{code}.community_members

**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.community_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    community_id UUID NOT NULL REFERENCES territory_{code}.communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    role VARCHAR(20) NOT NULL DEFAULT 'member',
    
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    pending_approval BOOLEAN NOT NULL DEFAULT false,
    
    UNIQUE (community_id, user_id),
    CHECK (role IN ('owner', 'admin', 'moderator', 'member'))
);
```

## Data Sovereignty

Communities are **territory-specific**. Cross-territory communities require federation (future).

## NATS Events

**Publishes:** `community.created`, `member_joined`, `member_left`

---

**Last Updated:** November 12, 2025
