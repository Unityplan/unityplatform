# user-service Database Schema

**Service:** user-service  
**Port:** 8002  
**Database Access:** Territory schema only

---

## Overview

The user-service owns all user profile data, connections, and GDPR compliance features. All data is stored in **territory schema only** (data sovereignty).

**Data Sovereignty Principle:**
- ALL user data stays in their home territory pod
- No global schema access needed (auth-service owns username/email registries)
- Cross-territory user lookups via API Gateway (federation)

---

## Schema Distribution

### Territory Schema Tables (9 tables)

| Table | Purpose | Rows (est.) |
|-------|---------|-------------|
| `users_profiles` | Extended profile data | 1:1 with users |
| `users_profile_links` | Social links (GitHub, LinkedIn) | 0-10 per user |
| `users_language_proficiency` | Language skills | 1-5 per user |
| `user_connections` | Followers/following/blocks | N:M relationships |
| `data_exports` | GDPR data export requests | 0-5 per user |
| `account_deletion_requests` | GDPR deletion requests | 0-1 per user |
| `file_uploads` | Avatar/image metadata | 1-10 per user |
| `activities` | User activity feed | 100-1000 per user |
| `audit_log` | Security audit trail | 100-1000 per user |

**All Personal Data:**  
Every table contains personal information that belongs to the user - stays in their territory.

---

## Table Specifications

### territory_{code}.users_profiles

**Purpose:** Extended user profile data (bio, avatar, interests, skills)

**Ownership:** user-service  
**Schema:** Territory only  
**Status:** ✅ Exists in 20251111000001_mvp_core_schema.sql

```sql
CREATE TABLE territory_{code}.users_profiles (
    user_id UUID PRIMARY KEY REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    -- Display Info
    display_name VARCHAR(100),
    avatar_url VARCHAR(500),
    
    -- Rich Profile
    bio VARCHAR(280),
    about TEXT,
    
    -- Tags (PostgreSQL arrays)
    interests TEXT[],
    skills TEXT[],
    languages VARCHAR(10)[],
    
    -- Location (privacy-aware: "[lat,lng]Display Name")
    location VARCHAR(500),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_profiles_display_name ON territory_{code}.users_profiles(display_name);
CREATE INDEX idx_users_profiles_interests ON territory_{code}.users_profiles USING GIN(interests);
CREATE INDEX idx_users_profiles_skills ON territory_{code}.users_profiles USING GIN(skills);

-- Full-text search
CREATE INDEX idx_users_profiles_search ON territory_{code}.users_profiles USING GIN(
    to_tsvector('english', 
        COALESCE(display_name, '') || ' ' || 
        COALESCE(bio, '') || ' ' || 
        COALESCE(about, '')
    )
);
```

**Data Sovereignty:**  
Profile data (bio, interests, location) is personal - NEVER leaves territory pod.

**Cross-Territory Access:**  
Other users fetch profiles via API: `GET https://denmark.unityplan.org/api/v1/profiles/{user_id}`

---

### territory_{code}.users_profile_links

**Purpose:** External links (GitHub, LinkedIn, portfolio, etc.)

**Ownership:** user-service  
**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.users_profile_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    label VARCHAR(100) NOT NULL,
    url VARCHAR(500) NOT NULL,
    icon VARCHAR(50),
    
    display_order INT NOT NULL DEFAULT 0,
    is_visible BOOLEAN NOT NULL DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (url ~* '^https?://'),
    UNIQUE (user_id, display_order)
);

CREATE INDEX idx_users_profile_links_user ON territory_{code}.users_profile_links(user_id);
```

**Link Types:** github, gitlab, twitter, linkedin, youtube, instagram, facebook, website, blog, other

---

### territory_{code}.users_language_proficiency

**Purpose:** Language skills with proficiency levels

**Ownership:** user-service  
**Status:** ✅ Exists in core migration + updated schema

```sql
CREATE TABLE territory_{code}.users_language_proficiency (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    language_code VARCHAR(10) NOT NULL,
    proficiency_level VARCHAR(20) NOT NULL,
    
    display_order INT NOT NULL DEFAULT 0,
    is_preferred BOOLEAN NOT NULL DEFAULT false,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (user_id, language_code),
    CHECK (proficiency_level IN ('basic', 'intermediate', 'advanced', 'fluent', 'native'))
);

CREATE INDEX idx_users_language_proficiency_user ON territory_{code}.users_language_proficiency(user_id);
```

**Proficiency Levels:**
- `basic` - A1/A2 (CEFR)
- `intermediate` - B1/B2
- `advanced` - C1/C2
- `fluent` - Near-native
- `native` - Mother tongue

---

### territory_{code}.user_connections

**Purpose:** Social connections (follow, block)

**Ownership:** user-service  
**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.user_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    follower_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    connection_type VARCHAR(20) NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (follower_id, following_id, connection_type),
    CHECK (connection_type IN ('follow', 'block')),
    CHECK (follower_id != following_id)
);

CREATE INDEX idx_user_connections_follower ON territory_{code}.user_connections(follower_id, connection_type);
CREATE INDEX idx_user_connections_following ON territory_{code}.user_connections(following_id, connection_type);
```

**Connection Types:**
- `follow` - User A follows User B (can see posts, get notifications)
- `block` - User A blocks User B (hides content, prevents interaction)

**Business Rules:**
- Blocking removes mutual follows
- Blocking prevents future follows
- Unblocking does NOT restore follows (user must re-follow)

**Cross-Territory Connections:**  
Users can follow users from other territories. The connection is stored in the follower's home pod.

**Example:**
- Alice (Denmark) follows Bob (Norway)
- Connection stored in `territory_dk.user_connections` (Alice's home pod)
- Bob's follower count fetched via API from his home pod (Norway)

---

### territory_{code}.data_exports

**Purpose:** GDPR Article 20 (Right to Data Portability)

**Ownership:** user-service  
**Status:** ✅ Exists in migration 20251112000004

```sql
CREATE TABLE territory_{code}.data_exports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    status VARCHAR(20) NOT NULL DEFAULT 'processing',
    
    export_data JSONB,
    file_path VARCHAR(500),
    file_size_bytes BIGINT,
    
    expires_at TIMESTAMPTZ NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    downloaded_at TIMESTAMPTZ,
    
    CHECK (status IN ('processing', 'completed', 'failed', 'expired')),
    CHECK (expires_at > created_at)
);

CREATE INDEX idx_data_exports_user ON territory_{code}.data_exports(user_id);
CREATE INDEX idx_data_exports_status ON territory_{code}.data_exports(status);
CREATE INDEX idx_data_exports_expires ON territory_{code}.data_exports(expires_at);
```

**Export Contents:**
```json
{
  "user": { "username": "alice", "email": "alice@example.com" },
  "profile": { "bio": "...", "interests": [...], "skills": [...] },
  "profile_links": [...],
  "languages": [...],
  "connections": { "followers": [...], "following": [...] },
  "settings": {...},
  "activity": [...],
  "export_date": "2025-11-12T10:00:00Z"
}
```

**Lifecycle:**
1. User requests export
2. Background job collects all user data
3. Export saved as JSON (GDPR requirement: machine-readable)
4. User downloads within 7 days
5. Export auto-deletes after expiration

---

### territory_{code}.account_deletion_requests

**Purpose:** GDPR Article 17 (Right to Erasure)

**Ownership:** user-service  
**Status:** ✅ Exists in migration 20251112000005

```sql
CREATE TABLE territory_{code}.account_deletion_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    
    confirmation_token VARCHAR(255) UNIQUE,
    token_expires_at TIMESTAMPTZ,
    
    scheduled_deletion_at TIMESTAMPTZ NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    
    CHECK (status IN ('pending', 'confirmed', 'cancelled', 'completed')),
    UNIQUE (user_id, status)
);

CREATE INDEX idx_account_deletion_requests_user ON territory_{code}.account_deletion_requests(user_id);
CREATE INDEX idx_account_deletion_requests_status ON territory_{code}.account_deletion_requests(status);
CREATE INDEX idx_account_deletion_requests_token ON territory_{code}.account_deletion_requests(confirmation_token);
```

**Deletion Flow:**
1. User requests deletion
2. Confirmation email sent with token
3. User confirms via email link (or expires in 24h)
4. 30-day grace period (can cancel)
5. After 30 days: hard delete user + all data

**Data Cleanup:**
- User profile → DELETED
- Profile links → DELETED
- Connections → DELETED (follower/following)
- Data exports → DELETED
- Activity logs → ANONYMIZED (user_id → NULL)
- Global registries → username/email freed

---

### territory_{code}.file_uploads

**Purpose:** Track uploaded files (avatars, images) via ipfs-service

**Ownership:** Shared (user-service tracks, ipfs-service stores)  
**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.file_uploads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    -- File Info
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    
    -- Storage (IPFS or S3)
    storage_type VARCHAR(20) NOT NULL,
    storage_path VARCHAR(500) NOT NULL,
    ipfs_hash VARCHAR(128),
    
    -- Upload Context
    upload_context VARCHAR(50) NOT NULL,
    
    -- Metadata
    metadata JSONB,
    
    -- Soft Delete
    deleted_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (storage_type IN ('ipfs', 's3', 'local')),
    CHECK (upload_context IN ('avatar', 'profile_image', 'community_logo', 'event_image'))
);

CREATE INDEX idx_file_uploads_user ON territory_{code}.file_uploads(user_id);
CREATE INDEX idx_file_uploads_storage ON territory_{code}.file_uploads(storage_path);
CREATE INDEX idx_file_uploads_ipfs ON territory_{code}.file_uploads(ipfs_hash) WHERE ipfs_hash IS NOT NULL;
```

**Why in user-service?**  
User owns the file metadata. IPFS-service handles storage, user-service tracks ownership.

---

### territory_{code}.activities

**Purpose:** User activity feed (posts, follows, badges, etc.)

**Ownership:** user-service (aggregates events from all services)  
**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    activity_type VARCHAR(50) NOT NULL,
    
    title VARCHAR(255) NOT NULL,
    description TEXT,
    
    target_type VARCHAR(50),
    target_id UUID,
    
    metadata JSONB,
    
    is_public BOOLEAN NOT NULL DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_activities_user ON territory_{code}.activities(user_id, created_at DESC);
CREATE INDEX idx_activities_type ON territory_{code}.activities(activity_type);
CREATE INDEX idx_activities_public ON territory_{code}.activities(is_public, created_at DESC) WHERE is_public = true;
```

**Activity Types:**
- `user.registered` - User joined platform
- `user.followed` - User followed someone
- `badge.earned` - User earned badge
- `community.joined` - User joined community
- `course.completed` - User completed course
- `event.attended` - User attended event

**Privacy:**  
`is_public` controls visibility. Non-public activities only shown to user.

---

### territory_{code}.audit_log

**Purpose:** Security audit trail (login, password change, etc.)

**Ownership:** Shared (multiple services write audit logs)  
**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    user_id UUID REFERENCES territory_{code}.users(id) ON DELETE SET NULL,
    
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50) NOT NULL,
    resource_id UUID,
    
    ip_address INET,
    user_agent TEXT,
    
    old_values JSONB,
    new_values JSONB,
    
    metadata JSONB,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_log_user ON territory_{code}.audit_log(user_id, created_at DESC);
CREATE INDEX idx_audit_log_action ON territory_{code}.audit_log(action);
CREATE INDEX idx_audit_log_time ON territory_{code}.audit_log(created_at DESC);
```

**Audit Actions:**
- `user.login` - User logged in
- `user.logout` - User logged out
- `user.password_changed` - Password updated
- `user.profile_updated` - Profile edited
- `user.settings_changed` - Settings modified
- `user.data_exported` - GDPR export requested
- `user.deletion_requested` - Account deletion requested

**Retention:**  
Keep 2 years for security compliance.

---

## Multi-Pod Considerations

### Cross-Territory User Discovery

**Scenario:** User in Norway searches for "Alice"

```sql
-- Local search (territory_no)
SELECT user_id, username, display_name, avatar_url, bio
FROM territory_no.users u
JOIN territory_no.users_profiles p ON u.id = p.user_id
WHERE username ILIKE '%alice%' OR display_name ILIKE '%alice%'
LIMIT 20;
```

**Result:** Only finds users in Norway pod.

**For Global Search:**  
API Gateway aggregates results from all pods:

```
GET /api/v1/users/search?q=alice
→ Queries all pods in parallel
→ Aggregates results
→ Returns combined list
```

---

### Cross-Territory Connections

**Scenario:** Alice (Denmark) follows Bob (Norway)

**Storage:**
```sql
-- Stored in Alice's home pod (territory_dk)
INSERT INTO territory_dk.user_connections (follower_id, following_id, connection_type)
VALUES (alice_id, bob_id, 'follow');
```

**Bob's Follower Count:**
```
GET https://norway.unityplan.org/api/v1/profiles/{bob_id}/followers
→ Queries all pods for connections where following_id = bob_id
→ Aggregates count
```

**Performance:**  
Follower/following counts cached in user profile (updated via NATS events).

---

### Cross-Territory Profile Access

**Scenario:** User in Sweden views Alice's profile (Denmark)

```
GET https://denmark.unityplan.org/api/v1/profiles/{alice_id}
→ Routed to Denmark pod (Alice's home)
→ Returns profile data
→ Privacy settings enforced (public/followers/private)
```

**Privacy Enforcement:**
```rust
async fn get_profile(user_id: Uuid, viewer_id: Option<Uuid>) -> Result<Profile> {
    let profile = fetch_profile(user_id).await?;
    
    match profile.visibility {
        Visibility::Public => Ok(profile),
        Visibility::Followers => {
            if let Some(viewer) = viewer_id {
                if is_following(viewer, user_id).await? {
                    Ok(profile)
                } else {
                    Err(Error::Forbidden)
                }
            } else {
                Err(Error::Unauthorized)
            }
        },
        Visibility::Private => Err(Error::Forbidden),
    }
}
```

---

## Performance Optimizations

### Denormalization Strategy

**Problem:**  
Follower count requires expensive aggregation.

**Solution:**  
Cache counts in profile:

```sql
-- Add to users_profiles
ALTER TABLE territory_dk.users_profiles
ADD COLUMN followers_count INT NOT NULL DEFAULT 0,
ADD COLUMN following_count INT NOT NULL DEFAULT 0;

-- Update via trigger or NATS event
CREATE OR REPLACE FUNCTION update_follower_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' AND NEW.connection_type = 'follow' THEN
        UPDATE users_profiles SET followers_count = followers_count + 1 WHERE user_id = NEW.following_id;
        UPDATE users_profiles SET following_count = following_count + 1 WHERE user_id = NEW.follower_id;
    ELSIF TG_OP = 'DELETE' AND OLD.connection_type = 'follow' THEN
        UPDATE users_profiles SET followers_count = followers_count - 1 WHERE user_id = OLD.following_id;
        UPDATE users_profiles SET following_count = following_count - 1 WHERE user_id = OLD.follower_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_update_follower_count
AFTER INSERT OR DELETE ON user_connections
FOR EACH ROW EXECUTE FUNCTION update_follower_count();
```

**Trade-off:**  
Slightly stale counts (eventually consistent) for fast reads.

---

## GDPR Compliance

### Data Export (Article 20)

**User Request:** "Download my data"

**Process:**
1. User clicks "Export Data"
2. Background job collects:
   - User account info
   - Profile data
   - Profile links
   - Languages
   - Connections (followers/following)
   - Settings
   - Activity history
3. JSON file generated
4. User downloads within 7 days

**Export Format:**
```json
{
  "export_version": "1.0",
  "export_date": "2025-11-12T10:00:00Z",
  "user": {...},
  "profile": {...},
  "connections": {...},
  "settings": {...},
  "activity": [...]
}
```

---

### Account Deletion (Article 17)

**User Request:** "Delete my account"

**Process:**
1. User requests deletion
2. Confirmation email sent
3. User confirms (24h timeout)
4. 30-day grace period (soft delete: `deleted_at` set)
5. After 30 days: hard delete

**Data Cleanup:**
```sql
-- Hard delete after 30 days
DELETE FROM territory_dk.users WHERE id = ? AND deleted_at < NOW() - INTERVAL '30 days';

-- Cascades to:
-- - users_profiles
-- - users_profile_links
-- - users_language_proficiency
-- - user_connections
-- - data_exports
-- - account_deletion_requests
-- - file_uploads (marks for IPFS unpinning)
-- - refresh_tokens (auth-service)

-- Global cleanup:
DELETE FROM global.username_registry WHERE user_id = ?;
DELETE FROM global.email_registry WHERE user_id = ?;
```

---

## Service Dependencies

### Depends On:
- **auth-service** - User authentication (creates user record)
- **ipfs-service** - Avatar storage

### Used By:
- **All services** - User profile data (display_name, avatar_url)

### NATS Events Subscribed:
- `user.registered` - Create default profile

### NATS Events Published:
- `user.followed` - User followed someone
- `user.unfollowed` - User unfollowed someone
- `profile.updated` - Profile data changed
- `avatar.updated` - Avatar changed

---

**Last Updated:** November 12, 2025  
**Schema Version:** 20251112000005 (with GDPR tables)  
**Implementation Status:** 28/28 endpoints complete
