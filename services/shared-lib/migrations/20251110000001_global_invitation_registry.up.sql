-- Add global invitation token registry for secure territory lookup
-- This ensures tokens are bound to territories in the DATABASE, not client-provided parameters
-- Prevents security issue where user could manipulate territory selection

-- Global registry table - Maps tokens to their territories
CREATE TABLE global.invitation_token_registry (
    token VARCHAR(255) PRIMARY KEY,  -- Same as territory.invitation_tokens.token
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    territory_token_id UUID NOT NULL,  -- References territory_X.invitation_tokens.id
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Ensure token is globally unique across all territories
    CONSTRAINT uq_global_invitation_token UNIQUE (token)
);

-- Index for fast territory lookup by token
CREATE INDEX idx_global_invitation_registry_token ON global.invitation_token_registry(token);
CREATE INDEX idx_global_invitation_registry_territory ON global.invitation_token_registry(territory_code);

COMMENT ON TABLE global.invitation_token_registry IS 'Global registry mapping invitation tokens to their territories. Ensures tokens cannot be used across territories (security).';
COMMENT ON COLUMN global.invitation_token_registry.token IS 'The invitation token string. Must match token in territory_X.invitation_tokens table.';
COMMENT ON COLUMN global.invitation_token_registry.territory_code IS 'Territory this token belongs to. Client cannot manipulate this - it is database-enforced.';
COMMENT ON COLUMN global.invitation_token_registry.territory_token_id IS 'UUID of the token record in territory_X.invitation_tokens table.';

-- Grant permissions
GRANT SELECT, INSERT, DELETE ON global.invitation_token_registry TO unityplan;
