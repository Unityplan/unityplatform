# invitation-service Database Schema

**Service:** invitation-service  
**Port:** 8004  
**Database:** Global + Territory schemas

---

## Schema Distribution

### Global Schema

**Table:** `global.registry_invitation`  
**Purpose:** Prevent duplicate tokens across all pods

```sql
CREATE TABLE global.registry_invitation (
    token VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    territory_token_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_invitation_token UNIQUE (token)
);

CREATE INDEX idx_registry_invitation_territory ON global.registry_invitation(territory_code);
```

**Why Global?** Invitation tokens must be globally unique (XXXX-XXXX-XXXX-XXXX format).

**Naming Convention:** Follows `registry_{resource}` pattern for global uniqueness enforcement.

---

### Territory Schema

**Table:** `territory_{code}.invitation_invitations_tokens`  
**Status:** ⏳ Pending migration creation

```sql
CREATE TABLE territory_{code}.invitation_invitations_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    token VARCHAR(255) NOT NULL UNIQUE,
    created_by UUID NOT NULL,  -- References global.registry_username(user_id) - no FK for service independence
    
    max_uses INT NOT NULL DEFAULT 1,
    uses_count INT NOT NULL DEFAULT 0,
    
    expires_at TIMESTAMPTZ,
    
    is_active BOOLEAN NOT NULL DEFAULT true,
    revoked_at TIMESTAMPTZ,
    revoked_by UUID,  -- References global.registry_username(user_id) - no FK for service independence
    
    metadata JSONB,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (max_uses >= 0),
    CHECK (uses_count <= max_uses OR max_uses = 0)
);

CREATE INDEX idx_invitation_invitations_tokens_token ON invitation_invitations_tokens(token);
CREATE INDEX idx_invitation_invitations_tokens_created_by ON invitation_invitations_tokens(created_by);
CREATE INDEX idx_invitation_invitations_tokens_active ON invitation_invitations_tokens(is_active) WHERE is_active = true;
CREATE INDEX idx_invitation_invitations_tokens_expires ON invitation_invitations_tokens(expires_at) WHERE expires_at IS NOT NULL;

COMMENT ON TABLE invitation_invitations_tokens IS 'Invitation tokens - owned by invitation-service';
COMMENT ON COLUMN invitation_invitations_tokens.created_by IS 'References global.registry_username(user_id) - validated via JWT, no FK for service independence';
COMMENT ON COLUMN invitation_invitations_tokens.revoked_by IS 'References global.registry_username(user_id) - validated via JWT, no FK for service independence';
```

**Naming Convention:** Follows `{service}_{entity}_{data}` pattern:

- `invitation_` = service prefix (invitation-service)
- `invitations_` = entity (what this service manages)
- `tokens` = data type (token storage)

**Table:** `territory_{code}.invitation_invitations_uses`

```sql
CREATE TABLE territory_{code}.invitation_invitations_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    invitation_id UUID NOT NULL,  -- References invitation_invitations_tokens(id) - FK within same service is OK
    used_by UUID NOT NULL,  -- References territory_{code}.auth_users_core(id) - no FK for service independence
    
    ip_address INET,
    user_agent TEXT,
    
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (invitation_id, used_by)
);

CREATE INDEX idx_invitation_invitations_uses_invitation ON invitation_invitations_uses(invitation_id);
CREATE INDEX idx_invitation_invitations_uses_user ON invitation_invitations_uses(used_by);
CREATE INDEX idx_invitation_invitations_uses_timestamp ON invitation_invitations_uses(used_at);

-- Foreign key within same service is allowed
ALTER TABLE invitation_invitations_uses 
    ADD CONSTRAINT fk_invitation_invitations_uses_invitation 
    FOREIGN KEY (invitation_id) 
    REFERENCES invitation_invitations_tokens(id) 
    ON DELETE CASCADE;

COMMENT ON TABLE invitation_invitations_uses IS 'Invitation usage tracking - owned by invitation-service';
COMMENT ON COLUMN invitation_invitations_uses.used_by IS 'References auth_users_core(id) - validated via JWT, no FK for service independence';
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
