-- Migration: Add invitation system for registration
-- Version: 20251106000003
-- Description: Create invitation_tokens and invitation_uses tables for invitation-based registration

-- Create invitation_tokens table in territory schema
CREATE TABLE IF NOT EXISTS territory_dk.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token VARCHAR(64) UNIQUE NOT NULL,  -- Cryptographically random (e.g., inv_a7bd3632957845479)
    token_type VARCHAR(20) NOT NULL CHECK (token_type IN ('single_use', 'group')),
    
    -- Restrictions
    email VARCHAR(255),  -- NULL for group tokens, specific email for single_use
    max_uses INT NOT NULL DEFAULT 1,
    used_count INT NOT NULL DEFAULT 0,
    
    -- Metadata
    created_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    purpose TEXT,  -- Optional description of invitation purpose
    metadata JSONB,  -- Additional data (group name, course info, etc.)
    
    -- Lifecycle
    expires_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    revoked_at TIMESTAMPTZ,
    revoked_by_user_id UUID REFERENCES territory_dk.users(id) ON DELETE SET NULL,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT check_token_type_constraints CHECK (
        (token_type = 'single_use' AND email IS NOT NULL AND max_uses = 1) OR
        (token_type = 'group' AND email IS NULL AND max_uses > 1)
    ),
    CONSTRAINT check_uses_within_limit CHECK (used_count <= max_uses),
    CONSTRAINT check_expires_in_future CHECK (expires_at > created_at)
);

-- Create invitation_uses table (audit trail)
CREATE TABLE IF NOT EXISTS territory_dk.invitation_uses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    invitation_token_id UUID NOT NULL REFERENCES territory_dk.invitation_tokens(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,  -- Optional: track IP for security
    user_agent TEXT   -- Optional: track browser/device
);

-- Indexes for invitation_tokens
CREATE INDEX idx_invitation_tokens_token ON territory_dk.invitation_tokens(token);
CREATE INDEX idx_invitation_tokens_email ON territory_dk.invitation_tokens(email) WHERE email IS NOT NULL;
CREATE INDEX idx_invitation_tokens_created_by ON territory_dk.invitation_tokens(created_by_user_id);
CREATE INDEX idx_invitation_tokens_active ON territory_dk.invitation_tokens(is_active, expires_at) WHERE is_active = true;
CREATE INDEX idx_invitation_tokens_type ON territory_dk.invitation_tokens(token_type);

-- Indexes for invitation_uses
CREATE INDEX idx_invitation_uses_token ON territory_dk.invitation_uses(invitation_token_id);
CREATE INDEX idx_invitation_uses_user ON territory_dk.invitation_uses(user_id);
CREATE INDEX idx_invitation_uses_used_at ON territory_dk.invitation_uses(used_at);

-- Trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_invitation_token_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_invitation_token_updated_at
    BEFORE UPDATE ON territory_dk.invitation_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_invitation_token_updated_at();

-- Add column to users table to track which invitation was used
ALTER TABLE territory_dk.users 
    ADD COLUMN IF NOT EXISTS invited_by_token_id UUID REFERENCES territory_dk.invitation_tokens(id) ON DELETE SET NULL;

CREATE INDEX idx_users_invited_by_token ON territory_dk.users(invited_by_token_id) WHERE invited_by_token_id IS NOT NULL;

-- Comments
COMMENT ON TABLE territory_dk.invitation_tokens IS 'Invitation tokens for controlled user registration';
COMMENT ON COLUMN territory_dk.invitation_tokens.token IS 'Cryptographically random token (e.g., inv_a7bd3632957845479)';
COMMENT ON COLUMN territory_dk.invitation_tokens.token_type IS 'Type: single_use (one person) or group (multiple people)';
COMMENT ON COLUMN territory_dk.invitation_tokens.email IS 'For single_use tokens: specific email address; for group: NULL';
COMMENT ON COLUMN territory_dk.invitation_tokens.max_uses IS 'Maximum number of times token can be used (1 for single_use, N for group)';
COMMENT ON COLUMN territory_dk.invitation_tokens.used_count IS 'Number of times token has been used';
COMMENT ON COLUMN territory_dk.invitation_tokens.is_active IS 'Whether token is currently active (can be manually deactivated)';

COMMENT ON TABLE territory_dk.invitation_uses IS 'Audit trail of invitation token usage';
COMMENT ON COLUMN territory_dk.users.invited_by_token_id IS 'Which invitation token was used to create this account';
