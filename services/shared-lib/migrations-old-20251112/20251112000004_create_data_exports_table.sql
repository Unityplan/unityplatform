-- Migration: Create data_exports table for GDPR Article 20 compliance
-- Created: 2025-11-12
-- Description: Implements GDPR data portability rights (Article 20) by allowing users to export their personal data.
--              Exports automatically expire after 7 days and support rate limiting.

-- Create the data_exports table
CREATE TABLE IF NOT EXISTS territory_dk.data_exports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    file_path TEXT,
    file_size BIGINT,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    downloaded_at TIMESTAMPTZ,
    error_message TEXT,
    
    -- Constraints
    CONSTRAINT data_exports_status_check CHECK (
        status IN ('pending', 'processing', 'completed', 'failed', 'expired')
    ),
    CONSTRAINT data_exports_check CHECK (
        expires_at IS NULL OR completed_at IS NOT NULL
    ),
    
    -- Foreign keys
    CONSTRAINT data_exports_user_id_fkey 
        FOREIGN KEY (user_id) 
        REFERENCES territory_dk.users(id) 
        ON DELETE CASCADE
);

-- Create indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_data_exports_user 
    ON territory_dk.data_exports(user_id);

CREATE INDEX IF NOT EXISTS idx_data_exports_status 
    ON territory_dk.data_exports(status);

CREATE INDEX IF NOT EXISTS idx_data_exports_expires 
    ON territory_dk.data_exports(expires_at) 
    WHERE status = 'completed' AND expires_at IS NOT NULL;

-- Create function to automatically set expiry date
CREATE OR REPLACE FUNCTION territory_dk.set_export_expiry()
RETURNS TRIGGER AS $$
BEGIN
    -- When export completes, set expiry to 7 days from completion
    IF NEW.status = 'completed' AND NEW.completed_at IS NOT NULL THEN
        NEW.expires_at = NEW.completed_at + INTERVAL '7 days';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to invoke the expiry function
CREATE TRIGGER trigger_set_export_expiry
    BEFORE INSERT OR UPDATE ON territory_dk.data_exports
    FOR EACH ROW
    EXECUTE FUNCTION territory_dk.set_export_expiry();

-- Add comments for documentation
COMMENT ON TABLE territory_dk.data_exports IS 
    'GDPR Article 20 (Right to Data Portability): Stores user data export requests and metadata. Exports expire after 7 days.';

COMMENT ON COLUMN territory_dk.data_exports.id IS 
    'Unique identifier for the export request';

COMMENT ON COLUMN territory_dk.data_exports.user_id IS 
    'User who requested the data export';

COMMENT ON COLUMN territory_dk.data_exports.status IS 
    'Export status: pending, processing, completed, failed, or expired';

COMMENT ON COLUMN territory_dk.data_exports.file_path IS 
    'Path to the exported data file (null for in-memory exports)';

COMMENT ON COLUMN territory_dk.data_exports.file_size IS 
    'Size of the exported data in bytes';

COMMENT ON COLUMN territory_dk.data_exports.requested_at IS 
    'Timestamp when the export was requested';

COMMENT ON COLUMN territory_dk.data_exports.started_at IS 
    'Timestamp when processing started';

COMMENT ON COLUMN territory_dk.data_exports.completed_at IS 
    'Timestamp when processing completed';

COMMENT ON COLUMN territory_dk.data_exports.expires_at IS 
    'Automatic expiry date (7 days after completion) - set by trigger';

COMMENT ON COLUMN territory_dk.data_exports.downloaded_at IS 
    'Timestamp when user downloaded the export (for tracking)';

COMMENT ON COLUMN territory_dk.data_exports.error_message IS 
    'Error message if export failed';

COMMENT ON FUNCTION territory_dk.set_export_expiry() IS 
    'Automatically sets expires_at to 7 days after completed_at when export completes';
