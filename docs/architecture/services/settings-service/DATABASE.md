# settings-service Database Schema

**Service:** settings-service  
**Port:** 8003  
**Database:** Territory schema only

---

## Tables

### territory_{code}.users_settings

**Status:** ✅ Exists in 20251111000001_mvp_core_schema.sql

```sql
CREATE TABLE territory_{code}.users_settings (
    user_id UUID PRIMARY KEY REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    -- Appearance
    theme_mode VARCHAR(20) NOT NULL DEFAULT 'system',
    color_scheme VARCHAR(50) NOT NULL DEFAULT 'forest-green',
    reduced_motion BOOLEAN NOT NULL DEFAULT false,
    
    -- Language
    preferred_language VARCHAR(10) NOT NULL DEFAULT 'en',
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    
    -- Privacy
    profile_visibility VARCHAR(20) NOT NULL DEFAULT 'public',
    show_email BOOLEAN NOT NULL DEFAULT false,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (theme_mode IN ('light', 'dark', 'system')),
    CHECK (profile_visibility IN ('public', 'followers', 'private'))
);
```

### territory_{code}.users_notification_settings

**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.users_notification_settings (
    user_id UUID PRIMARY KEY REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    -- Email Notifications
    email_enabled BOOLEAN NOT NULL DEFAULT true,
    email_on_follower BOOLEAN NOT NULL DEFAULT true,
    email_on_message BOOLEAN NOT NULL DEFAULT true,
    
    -- In-App Notifications
    in_app_enabled BOOLEAN NOT NULL DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Data Sovereignty

**All settings are personal data** - stored in user's home territory pod.

## NATS Events

**Subscribes:** `user.registered` (create default settings)  
**Publishes:** `settings.updated`, `privacy.updated`

---

**Last Updated:** November 12, 2025
