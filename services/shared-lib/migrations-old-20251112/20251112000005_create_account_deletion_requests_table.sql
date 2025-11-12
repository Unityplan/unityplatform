-- Migration: Create account_deletion_requests table for GDPR Article 17 compliance
-- Created: 2025-11-12
-- Description: Implements GDPR Right to Erasure (Article 17) with 30-day grace period,
--              email confirmation, and cancellation support.

-- Create the account_deletion_requests table
CREATE TABLE IF NOT EXISTS territory_dk.account_deletion_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    confirmation_token VARCHAR(255),
    token_expires_at TIMESTAMPTZ,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ,
    scheduled_deletion_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    cancellation_reason TEXT,
    ip_address TEXT,
    user_agent TEXT,
    
    -- Constraints
    CONSTRAINT account_deletion_requests_status_check CHECK (
        status IN ('pending', 'confirmed', 'cancelled', 'completed')
    ),
    CONSTRAINT account_deletion_requests_check CHECK (
        scheduled_deletion_at IS NULL OR confirmed_at IS NOT NULL
    ),
    
    -- Foreign keys
    CONSTRAINT account_deletion_requests_user_id_fkey 
        FOREIGN KEY (user_id) 
        REFERENCES territory_dk.users(id) 
        ON DELETE CASCADE
);

-- Create indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_account_deletion_requests_user 
    ON territory_dk.account_deletion_requests(user_id);

CREATE INDEX IF NOT EXISTS idx_account_deletion_requests_status 
    ON territory_dk.account_deletion_requests(status);

CREATE INDEX IF NOT EXISTS idx_account_deletion_requests_scheduled 
    ON territory_dk.account_deletion_requests(scheduled_deletion_at) 
    WHERE status = 'confirmed' AND scheduled_deletion_at IS NOT NULL;

-- Create function to automatically set scheduled deletion date (30 days from confirmation)
CREATE OR REPLACE FUNCTION territory_dk.set_deletion_schedule()
RETURNS TRIGGER AS $$
BEGIN
    -- When deletion is confirmed, schedule it for 30 days from confirmation
    IF NEW.status = 'confirmed' AND NEW.confirmed_at IS NOT NULL AND (OLD.status IS NULL OR OLD.status != 'confirmed') THEN
        NEW.scheduled_deletion_at = NEW.confirmed_at + INTERVAL '30 days';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to invoke the deletion schedule function
CREATE TRIGGER trigger_set_deletion_schedule
    BEFORE INSERT OR UPDATE ON territory_dk.account_deletion_requests
    FOR EACH ROW
    EXECUTE FUNCTION territory_dk.set_deletion_schedule();

-- Add comments for documentation
COMMENT ON TABLE territory_dk.account_deletion_requests IS 
    'GDPR Article 17 (Right to Erasure): Tracks account deletion requests with 30-day grace period and email confirmation.';

COMMENT ON COLUMN territory_dk.account_deletion_requests.id IS 
    'Unique identifier for the deletion request';

COMMENT ON COLUMN territory_dk.account_deletion_requests.user_id IS 
    'User who requested account deletion';

COMMENT ON COLUMN territory_dk.account_deletion_requests.status IS 
    'Deletion status: pending, confirmed, cancelled, or completed';

COMMENT ON COLUMN territory_dk.account_deletion_requests.confirmation_token IS 
    'Email confirmation token (UUID format)';

COMMENT ON COLUMN territory_dk.account_deletion_requests.token_expires_at IS 
    'Token expiry time (24 hours after request)';

COMMENT ON COLUMN territory_dk.account_deletion_requests.requested_at IS 
    'Timestamp when deletion was requested';

COMMENT ON COLUMN territory_dk.account_deletion_requests.confirmed_at IS 
    'Timestamp when user confirmed deletion via email';

COMMENT ON COLUMN territory_dk.account_deletion_requests.scheduled_deletion_at IS 
    'Automatic deletion date (30 days after confirmation) - set by trigger';

COMMENT ON COLUMN territory_dk.account_deletion_requests.cancelled_at IS 
    'Timestamp when deletion was cancelled';

COMMENT ON COLUMN territory_dk.account_deletion_requests.cancellation_reason IS 
    'User-provided reason for cancellation';

COMMENT ON COLUMN territory_dk.account_deletion_requests.ip_address IS 
    'IP address of deletion request (for audit)';

COMMENT ON COLUMN territory_dk.account_deletion_requests.user_agent IS 
    'User agent of deletion request (for audit)';

COMMENT ON FUNCTION territory_dk.set_deletion_schedule() IS 
    'Automatically sets scheduled_deletion_at to 30 days after confirmed_at when deletion is confirmed';
