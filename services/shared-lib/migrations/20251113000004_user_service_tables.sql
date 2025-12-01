-- Migration: User Service Core Tables
-- Service: user-service
-- Description: Creates user profile, connections, and GDPR compliance tables in territory schema
-- Dependencies: 20251112000003_auth_core_tables.sql (requires users table)

-- ============================================================================
-- users_profiles - Extended user profile data
-- ============================================================================
CREATE TABLE IF NOT EXISTS territory_dk.user_users_profiles (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    
    -- Display Info
    display_name VARCHAR(100),
    avatar_url VARCHAR(500),
    
    -- Rich Profile
    bio VARCHAR(280),
    about TEXT,
    location VARCHAR(500),
    website VARCHAR(500),
    
    -- Tags (PostgreSQL arrays)
    interests TEXT[],
    skills TEXT[],
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_profiles_display_name ON territory_dk.user_users_profiles(display_name);
CREATE INDEX IF NOT EXISTS idx_users_profiles_interests ON territory_dk.user_users_profiles USING GIN(interests);
CREATE INDEX IF NOT EXISTS idx_users_profiles_skills ON territory_dk.user_users_profiles USING GIN(skills);

-- Full-text search
CREATE INDEX IF NOT EXISTS idx_users_profiles_search ON territory_dk.user_users_profiles USING GIN(
    to_tsvector('english', 
        COALESCE(display_name, '') || ' ' || 
        COALESCE(bio, '') || ' ' || 
        COALESCE(about, '')
    )
);

COMMENT ON TABLE territory_dk.user_users_profiles IS 'Extended user profile data (bio, avatar, interests, skills)';
COMMENT ON COLUMN territory_dk.user_users_profiles.display_name IS 'Public display name (different from username)';
COMMENT ON COLUMN territory_dk.user_users_profiles.bio IS 'Short bio (280 chars, like Twitter)';
COMMENT ON COLUMN territory_dk.user_users_profiles.about IS 'Long-form about section (Markdown)';

-- ============================================================================
-- users_profile_links - External links (GitHub, LinkedIn, etc.)
-- ============================================================================
CREATE TABLE IF NOT EXISTS territory_dk.user_users_profile_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    
    label VARCHAR(100) NOT NULL,
    url VARCHAR(500) NOT NULL,
    icon VARCHAR(50),
    
    display_order INT NOT NULL DEFAULT 0,
    is_visible BOOLEAN NOT NULL DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_url CHECK (url ~* '^https?://'),
    CONSTRAINT unique_user_order UNIQUE (user_id, display_order)
);

CREATE INDEX idx_users_profile_links_user ON territory_dk.user_users_profile_links(user_id);

COMMENT ON TABLE territory_dk.user_users_profile_links IS 'External profile links (max 10 per user)';
COMMENT ON COLUMN territory_dk.user_users_profile_links.label IS 'Link label (e.g., "GitHub", "LinkedIn")';
COMMENT ON COLUMN territory_dk.user_users_profile_links.icon IS 'Icon identifier (e.g., "github", "linkedin")';

-- ============================================================================
-- users_language_proficiency - Language skills with proficiency levels
-- ============================================================================
CREATE TABLE IF NOT EXISTS territory_dk.user_users_profile_language_proficiency (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    
    language_code VARCHAR(10) NOT NULL,  -- ISO 639-1 (e.g., 'en', 'da', 'no')
    language_name VARCHAR(100) NOT NULL,
    
    -- Four skill dimensions
    spoken_level VARCHAR(20) NOT NULL,
    written_level VARCHAR(20) NOT NULL,
    reading_level VARCHAR(20) NOT NULL,
    listening_level VARCHAR(20) NOT NULL,
    
    display_order INT NOT NULL DEFAULT 0,
    is_preferred BOOLEAN NOT NULL DEFAULT false,
    show_on_profile BOOLEAN NOT NULL DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_user_language UNIQUE (user_id, language_code),
    CONSTRAINT valid_proficiency_levels CHECK (
        spoken_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning') AND
        written_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning') AND
        reading_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning') AND
        listening_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')
    )
);

CREATE INDEX idx_users_language_proficiency_user ON territory_dk.user_users_profile_language_proficiency(user_id);
CREATE INDEX idx_users_language_proficiency_order ON territory_dk.user_users_profile_language_proficiency(user_id, display_order);
CREATE INDEX idx_users_language_proficiency_preferred ON territory_dk.user_users_profile_language_proficiency(user_id, is_preferred) 
    WHERE is_preferred = true;

COMMENT ON TABLE territory_dk.user_users_profile_language_proficiency IS 'User language skills with 4 dimensions (spoken/written/reading/listening)';
COMMENT ON COLUMN territory_dk.user_users_profile_language_proficiency.language_code IS 'ISO 639-1 language code (e.g., en, da, no, sv)';

-- ============================================================================
-- user_connections - Social connections (follow, block)
-- ============================================================================
CREATE TABLE IF NOT EXISTS territory_dk.user_users_connections (
    user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    target_user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    
    connection_type VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (user_id, target_user_id, connection_type),
    
    CONSTRAINT no_self_connection CHECK (user_id != target_user_id),
    CONSTRAINT valid_connection_type CHECK (connection_type IN ('follow', 'block')),
    CONSTRAINT valid_status CHECK (status IN ('active', 'pending', 'rejected'))
);

CREATE INDEX idx_user_connections_user ON territory_dk.user_users_connections(user_id, connection_type);
CREATE INDEX idx_user_connections_target ON territory_dk.user_users_connections(target_user_id, connection_type);
CREATE INDEX idx_user_connections_blocks ON territory_dk.user_users_connections(user_id, connection_type) 
    WHERE connection_type = 'block';

COMMENT ON TABLE territory_dk.user_users_connections IS 'User social connections (follow/block)';
COMMENT ON COLUMN territory_dk.user_users_connections.connection_type IS 'follow = user follows target, block = user blocks target';
COMMENT ON COLUMN territory_dk.user_users_connections.status IS 'active = current connection, pending = friend request, rejected = declined';

-- ============================================================================
-- data_exports - GDPR Article 20 (Right to Data Portability)
-- ============================================================================
CREATE TABLE IF NOT EXISTS territory_dk.user_users_data_exports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    
    file_path VARCHAR(500),
    file_size_bytes BIGINT,
    
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    downloaded_at TIMESTAMPTZ,
    
    error_message TEXT,
    
    CONSTRAINT valid_export_status CHECK (
        status IN ('pending', 'processing', 'completed', 'failed', 'expired')
    )
);

CREATE INDEX idx_data_exports_user ON territory_dk.user_users_data_exports(user_id);
CREATE INDEX idx_data_exports_status ON territory_dk.user_users_data_exports(status);
CREATE INDEX idx_data_exports_expires ON territory_dk.user_users_data_exports(expires_at) WHERE status = 'completed';

-- Auto-expire exports after 7 days
CREATE OR REPLACE FUNCTION expire_old_exports()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'completed' AND NEW.completed_at IS NOT NULL THEN
        NEW.expires_at = NEW.completed_at + INTERVAL '7 days';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_set_export_expiry
    BEFORE INSERT OR UPDATE ON territory_dk.user_users_data_exports
    FOR EACH ROW
    EXECUTE FUNCTION expire_old_exports();

COMMENT ON TABLE territory_dk.user_users_data_exports IS 'GDPR Article 20: User data export requests (7-day expiration)';
COMMENT ON COLUMN territory_dk.user_users_data_exports.file_path IS 'Path to generated export file (JSON format)';

-- ============================================================================
-- account_deletion_requests - GDPR Article 17 (Right to Erasure)
-- ============================================================================
CREATE TABLE IF NOT EXISTS territory_dk.user_users_account_deletion_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.auth_users_core(id) ON DELETE CASCADE,
    
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    
    confirmation_token VARCHAR(255) UNIQUE,
    token_expires_at TIMESTAMPTZ,
    
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ,
    scheduled_deletion_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    cancellation_reason TEXT,
    
    ip_address VARCHAR(45),
    user_agent TEXT,
    
    CONSTRAINT valid_deletion_status CHECK (
        status IN ('pending', 'confirmed', 'cancelled', 'completed')
    ),
    CONSTRAINT deletion_schedule_check CHECK (
        scheduled_deletion_at IS NULL OR confirmed_at IS NOT NULL
    )
);

CREATE INDEX idx_account_deletion_requests_user ON territory_dk.user_users_account_deletion_requests(user_id);
CREATE INDEX idx_account_deletion_requests_status ON territory_dk.user_users_account_deletion_requests(status);
CREATE INDEX idx_account_deletion_requests_scheduled ON territory_dk.user_users_account_deletion_requests(scheduled_deletion_at)
    WHERE status = 'confirmed' AND scheduled_deletion_at IS NOT NULL;

-- Auto-schedule deletion for 30 days after confirmation
CREATE OR REPLACE FUNCTION set_deletion_schedule()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'confirmed' AND NEW.confirmed_at IS NOT NULL 
       AND (OLD.status IS NULL OR OLD.status != 'confirmed') THEN
        NEW.scheduled_deletion_at = NEW.confirmed_at + INTERVAL '30 days';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_set_deletion_schedule
    BEFORE INSERT OR UPDATE ON territory_dk.user_users_account_deletion_requests
    FOR EACH ROW
    EXECUTE FUNCTION set_deletion_schedule();

COMMENT ON TABLE territory_dk.user_users_account_deletion_requests IS 'GDPR Article 17: Account deletion with 30-day grace period';
COMMENT ON COLUMN territory_dk.user_users_account_deletion_requests.confirmation_token IS 'Email confirmation token (24h expiry)';
COMMENT ON COLUMN territory_dk.user_users_account_deletion_requests.scheduled_deletion_at IS 'Hard delete scheduled 30 days after confirmation';
