# invitation-service Database Schema

**Service:** invitation-service  
**Port:** 8004  
**Database:** Global + Territory schemas

---

## Schema Distribution

### Global Schema

**Table:** `global.invitation_token_registry`  
**Purpose:** Prevent duplicate tokens across all pods

```sql
CREATE TABLE global.invitation_token_registry (
    token VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    territory_token_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_invitation_token UNIQUE (token)
);
```

**Why Global?** Invitation tokens must be globally unique (XXXX-XXXX-XXXX-XXXX format).

---

### Territory Schema

**Table:** `territory_{code}.invitation_tokens`  
**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    token VARCHAR(255) NOT NULL UNIQUE,
    created_by UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    max_uses INT NOT NULL DEFAULT 1,
    uses_count INT NOT NULL DEFAULT 0,
    
    expires_at TIMESTAMPTZ,
    
    is_active BOOLEAN NOT NULL DEFAULT true,
    revoked_at TIMESTAMPTZ,
    
    metadata JSONB,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (max_uses >= 0),
    CHECK (uses_count <= max_uses OR max_uses = 0)
);
```

**Table:** `territory_{code}.invitation_uses`

```sql
CREATE TABLE territory_{code}.invitation_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    invitation_id UUID NOT NULL REFERENCES territory_{code}.invitation_tokens(id) ON DELETE CASCADE,
    used_by UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    ip_address INET,
    user_agent TEXT,
    
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (invitation_id, used_by)
);
```

## Multi-Pod Considerations

**Token Generation:**

1. Generate token in territory schema
2. Register in global schema (ensure uniqueness)
3. Transaction: both inserts succeed or rollback

**Token Validation:**

1. Check global registry (exists?)
2. Fetch from territory schema (active? not expired?)
3. Return validation result to auth-service

## Invitation Tree

Users can see who they invited and who those users invited (trust graph).

## NATS Events

**Publishes:** `invitation.created`, `invitation.used`, `invitation.revoked`

---

**Last Updated:** November 12, 2025
