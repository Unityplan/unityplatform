-- ============================================================================
-- Migration: 20251118000009_invitation_service_tables.sql
-- Level: 2 (Service Tables)
-- Service: invitation-service
-- Description: Create invitation token management tables
-- Dependencies: 20251112000003_auth_core_tables.sql (requires auth_users_core)
-- ============================================================================

-- ============================================================================
-- Global: Invitation Token Registry (Global Uniqueness)
-- ============================================================================
-- Purpose: Ensure invitation tokens are globally unique across all territories
-- Owner: invitation-service
-- Strategy: Check this table before creating invitation tokens
-- Format: XXXX-XXXX-XXXX-XXXX (16 alphanumeric characters)
-- ============================================================================

CREATE TABLE IF NOT EXISTS global.registry_invitation (
    token VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL,
    territory_token_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Reference to territory (OK - global table)
    CONSTRAINT fk_invitation_territory 
        FOREIGN KEY (territory_code) 
        REFERENCES global.registry_territories(code) 
        ON DELETE CASCADE,
    
    -- Ensure token format (basic validation)
    CONSTRAINT uq_global_invitation_token UNIQUE (token),
    CHECK (char_length(token) >= 16 AND char_length(token) <= 255)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_registry_invitation_territory 
    ON global.registry_invitation(territory_code);
CREATE INDEX IF NOT EXISTS idx_registry_invitation_created 
    ON global.registry_invitation(created_at);

-- Comments
COMMENT ON TABLE global.registry_invitation IS 
    'Global invitation token registry ensuring tokens are unique across all territories and pods.';
COMMENT ON COLUMN global.registry_invitation.token IS 
    'Globally unique invitation code (format: XXXX-XXXX-XXXX-XXXX)';
COMMENT ON COLUMN global.registry_invitation.territory_token_id IS 
    'References invitation_invitations_tokens(id) in the territory schema';

-- ============================================================================
-- Territory: Invitation Tokens
-- ============================================================================
-- Purpose: Store invitation tokens with usage limits and expiration
-- Owner: invitation-service
-- Scope: Territory-specific (tokens belong to territory managers)
-- Naming: {service}_{entity}_{data} = invitation_invitations_tokens
-- Production: Only territory/community managers can create invitations
-- ============================================================================

CREATE TABLE IF NOT EXISTS territory_dk.invitation_invitations_tokens (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Token
    token VARCHAR(255) NOT NULL UNIQUE,
    
    -- Ownership (no FK for service independence - validated via JWT)
    created_by UUID NOT NULL,
    
    -- Usage limits
    max_uses INT NOT NULL DEFAULT 1,      -- 1 = single-use, 0 = unlimited, N = N uses
    uses_count INT NOT NULL DEFAULT 0,     -- Current usage count
    
    -- Expiration
    expires_at TIMESTAMPTZ,                -- NULL = never expires
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    revoked_at TIMESTAMPTZ,
    revoked_by UUID,                       -- References auth_users_core(id) - no FK for service independence
    
    -- Metadata (extensible JSON for community_id, role, etc.)
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (max_uses >= 0),
    CHECK (uses_count >= 0),
    CHECK (uses_count <= max_uses OR max_uses = 0),  -- 0 = unlimited
    CHECK (
        (is_active = TRUE AND revoked_at IS NULL) OR 
        (is_active = FALSE)  -- Can be false for revocation OR fully used
    )
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_invitation_invitations_tokens_token 
    ON territory_dk.invitation_invitations_tokens(token);
CREATE INDEX IF NOT EXISTS idx_invitation_invitations_tokens_created_by 
    ON territory_dk.invitation_invitations_tokens(created_by);
CREATE INDEX IF NOT EXISTS idx_invitation_invitations_tokens_active 
    ON territory_dk.invitation_invitations_tokens(is_active) 
    WHERE is_active = TRUE;
CREATE INDEX IF NOT EXISTS idx_invitation_invitations_tokens_expires 
    ON territory_dk.invitation_invitations_tokens(expires_at) 
    WHERE expires_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_invitation_invitations_tokens_created 
    ON territory_dk.invitation_invitations_tokens(created_at);

-- Comments
COMMENT ON TABLE territory_dk.invitation_invitations_tokens IS 
    'Invitation tokens for user registration. Owned by invitation-service. Production: Manager-only creation.';
COMMENT ON COLUMN territory_dk.invitation_invitations_tokens.created_by IS 
    'References auth_users_core(id) - validated via JWT middleware. No FK constraint for service independence.';
COMMENT ON COLUMN territory_dk.invitation_invitations_tokens.revoked_by IS 
    'References auth_users_core(id) - user who revoked the invitation. No FK constraint for service independence.';
COMMENT ON COLUMN territory_dk.invitation_invitations_tokens.max_uses IS 
    'Maximum number of times this invitation can be used. 1 = single-use, 0 = unlimited, N = N uses.';
COMMENT ON COLUMN territory_dk.invitation_invitations_tokens.metadata IS 
    'Extensible JSON for additional context (e.g., {"community_id": "uuid", "role": "member", "purpose": "friend_invite"})';

-- ============================================================================
-- Territory: Invitation Uses (Tracking)
-- ============================================================================
-- Purpose: Track who used which invitation token (trust graph / invitation tree)
-- Owner: invitation-service
-- Scope: Territory-specific
-- Naming: {service}_{entity}_{data} = invitation_invitations_uses
-- ============================================================================

CREATE TABLE IF NOT EXISTS territory_dk.invitation_invitations_uses (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Foreign key to invitation token (OK - same service)
    invitation_id UUID NOT NULL,
    
    -- User who used the invitation (no FK for service independence)
    used_by UUID NOT NULL,
    
    -- Context
    ip_address INET,
    user_agent TEXT,
    
    -- Timestamp
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Prevent duplicate usage by same user
    UNIQUE (invitation_id, used_by)
);

-- Foreign key within same service is allowed
ALTER TABLE territory_dk.invitation_invitations_uses 
    ADD CONSTRAINT fk_invitation_invitations_uses_invitation 
    FOREIGN KEY (invitation_id) 
    REFERENCES territory_dk.invitation_invitations_tokens(id) 
    ON DELETE CASCADE;

-- Indexes
CREATE INDEX IF NOT EXISTS idx_invitation_invitations_uses_invitation 
    ON territory_dk.invitation_invitations_uses(invitation_id);
CREATE INDEX IF NOT EXISTS idx_invitation_invitations_uses_user 
    ON territory_dk.invitation_invitations_uses(used_by);
CREATE INDEX IF NOT EXISTS idx_invitation_invitations_uses_timestamp 
    ON territory_dk.invitation_invitations_uses(used_at);

-- Comments
COMMENT ON TABLE territory_dk.invitation_invitations_uses IS 
    'Tracks who used which invitation token. Builds invitation tree / trust graph.';
COMMENT ON COLUMN territory_dk.invitation_invitations_uses.invitation_id IS 
    'References invitation_invitations_tokens(id) - FK allowed within same service.';
COMMENT ON COLUMN territory_dk.invitation_invitations_uses.used_by IS 
    'References auth_users_core(id) - validated via JWT. No FK constraint for service independence.';
COMMENT ON COLUMN territory_dk.invitation_invitations_uses.ip_address IS 
    'IP address of registration for spam detection and security analysis.';

-- ============================================================================
-- Trigger: Update updated_at timestamp
-- ============================================================================

CREATE OR REPLACE FUNCTION update_invitation_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_invitation_tokens_updated_at
    BEFORE UPDATE ON territory_dk.invitation_invitations_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_invitation_updated_at();

-- ============================================================================
-- Migration Complete
-- ============================================================================
-- Tables Created:
-- - global.registry_invitation (global uniqueness)
-- - territory_dk.invitation_invitations_tokens (invitation storage)
-- - territory_dk.invitation_invitations_uses (usage tracking)
--
-- Service Independence:
-- - No FK to auth_users_core (validated via JWT middleware)
-- - FK within same service (invitation_uses -> invitation_tokens) is OK
--
-- Production Requirements:
-- - Only managers can create invitations (enforced by invitation-service)
-- - Registration requires valid invitation token (enforced by auth-service)
-- - Dev mode: Optional invitation validation for testing
-- ============================================================================
