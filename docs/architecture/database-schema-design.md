# Database Schema Design - PostgreSQL to Holochain Migration

**Status:** Planning Phase  
**Created:** November 11, 2025  
**Purpose:** Design PostgreSQL schema with Holochain migration in mind  
**Target:** Multi-tenant, agent-centric, event-sourced architecture

---

## Table of Contents

1. [Design Principles](#design-principles)
2. [Schema Structure Overview](#schema-structure-overview)
3. [Global Schema](#global-schema)
4. [Territory Schema](#territory-schema)
5. [Holochain Entry Type Mapping](#holochain-entry-type-mapping)
6. [Migration Strategy](#migration-strategy)

---

## Design Principles

### PostgreSQL Phase (Current)

1. **Multi-Tenancy:** Territory-based schema isolation (`territory_dk`, `territory_no`, etc.)
2. **ACID Compliance:** Strong consistency for critical operations
3. **Relational Integrity:** Foreign keys, constraints, cascades
4. **Performance:** Indexes on frequently queried columns
5. **Auditability:** Timestamps, soft deletes, audit logs

### Holochain Readiness (Future)

1. **Agent-Centric:** Users own their data (agent entries)
2. **Event-Sourced:** Append-only logs, immutable history
3. **Distributed:** No central authority, DHT-based storage
4. **Linked Data:** Relationships via cryptographic links
5. **Validation:** Schema validation in zome functions

### Design for Both Worlds

- **Immutable Audit Trails:** Use separate audit tables (maps to Holochain source chain)
- **Avoid Complex Joins:** Denormalize where appropriate (DHT queries are expensive)
- **User Ownership:** User data should be queryable by user_id (agent in Holochain)
- **Event Logs:** Store state changes as events (natural fit for Holochain)
- **Soft Deletes:** Never hard delete (Holochain is append-only)

---

## Schema Structure Overview

## Schema Structure Overview

**Schema Naming Convention:**

- **Single-territory pod:** Schema named after territory code
  - Example: `territory_dk` (Denmark only)
  - One territory per database
  
- **Multi-territory pod:** Schema named after territory code
  - Example: `territory_dk`, `territory_no`, `territory_se`
  - Multiple territories per database
  
- **In documentation:** We use `{schema_name}` as placeholder in SQL examples

```
unityplan_db
├── global (Global/shared data across territories)
│   ├── territories
│   ├── invitation_token_registry
│   └── global_username_registry
│
├── territory_dk (Single-pod example: Denmark)
│   ├── users
│   ├── users_profiles
│   ├── users_profile_links
│   ├── users_language_proficiency
│   ├── users_settings
│   ├── users_notification_settings
│   ├── invitation_tokens
│   ├── invitation_uses
│   ├── communities
│   ├── community_members
│   ├── roles
│   ├── role_assignments
│   ├── community_role_elections
│   ├── community_role_election_votes
│   ├── badge_definitions
│   ├── badge_awards
│   ├── badge_progress
│   ├── groups (access bubbles)
│   ├── group_communities
│   ├── group_forums (future extension)
│   ├── group_courses (future extension)
│   ├── notifications
│   ├── user_connections
│   ├── community_events
│   ├── event_rsvps
│   ├── file_uploads
│   ├── content_reports
│   ├── moderation_actions
│   ├── activities
│   ├── posts
│   ├── messages
│   └── users_audit_logs
│
├── territory_dk (Multi-pod example: Denmark on shared pod)
│   └── (same structure as above)
│
├── territory_no (Multi-pod example: Norway on shared pod)
│   └── (same structure as above)
│
└── territory_se (Multi-pod example: Sweden on shared pod)
    └── (same structure as above)
```

---

## Global Schema

### 1. Territories

**Purpose:** Registry of all territory pods in the network

```sql
CREATE SCHEMA IF NOT EXISTS global;

CREATE TABLE global.territories (
    code VARCHAR(10) PRIMARY KEY,              -- 'dk', 'no', 'se', 'eu'
    name VARCHAR(100) NOT NULL,                -- 'Denmark', 'Norway', etc.
    display_name VARCHAR(100) NOT NULL,        -- 'Denmark Territory'
    description TEXT,
    
    -- Pod Configuration
    pod_url VARCHAR(255) NOT NULL,             -- https://denmark.unityplan.org
    api_url VARCHAR(255) NOT NULL,             -- https://api.denmark.unityplan.org
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- 'active', 'maintenance', 'inactive'
    
    -- Metadata
    language_code VARCHAR(10) NOT NULL,        -- 'da', 'no', 'sv', 'en'
    timezone VARCHAR(50) NOT NULL,             -- 'Europe/Copenhagen'
    currency_code VARCHAR(3),                  -- 'DKK', 'NOK', 'SEK'
    
    -- Holochain DNA Info (Future)
    dna_hash VARCHAR(128),                     -- Holochain DNA identifier
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_territories_status ON global.territories(status);
```

**Holochain Mapping:**

- This becomes a **Bridge** configuration
- Each territory = separate DNA instance
- Cross-territory queries via Bridge calls

### 2. Global Username Registry

**Purpose:** Ensure username uniqueness across all territories

```sql
CREATE TABLE global.username_registry (
    username VARCHAR(50) PRIMARY KEY,          -- Globally unique username
    user_id UUID NOT NULL,                     -- FK to territory_X.users.id
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Composite unique constraint (redundant but explicit)
    CONSTRAINT uq_global_username UNIQUE (username)
);

CREATE INDEX idx_username_registry_territory ON global.username_registry(territory_code);
CREATE INDEX idx_username_registry_user ON global.username_registry(user_id);
```

**Holochain Mapping:**

- Bridge query across DNAs
- DHT anchor pattern: `anchor("username", "alice_dk") -> AgentPubKey`

### 3. Global Email Registry

**Purpose:** Ensure email uniqueness across all territories

```sql
CREATE TABLE global.email_registry (
    email VARCHAR(255) PRIMARY KEY,            -- Globally unique email (lowercased)
    user_id UUID NOT NULL,                     -- FK to territory_X.users.id
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code),
    
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_email UNIQUE (email)
);

CREATE INDEX idx_email_registry_territory ON global.email_registry(territory_code);
CREATE INDEX idx_email_registry_verified ON global.email_registry(is_verified);
```

**Holochain Mapping:**

- Bridge query for email uniqueness check
- Privacy: Emails may be stored off-chain (GDPR considerations)

### 4. Global Invitation Token Registry

**Purpose:** Map invitation tokens to territories (security)

See [backend-api-requirements.md](backend-api-requirements.md#2-global-token-registry-globalinvitation_token_registry--new) for details.

```sql
CREATE TABLE global.invitation_token_registry (
    token VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    territory_token_id UUID NOT NULL,          -- FK to territory_X.invitation_tokens.id
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_global_invitation_token UNIQUE (token)
);

CREATE INDEX idx_global_invitation_registry_token ON global.invitation_token_registry(token);
CREATE INDEX idx_global_invitation_registry_territory ON global.invitation_token_registry(territory_code);
```

**Holochain Mapping:**

- Bridge query for token validation
- Token creator (agent) signs invitation entry

---

## Territory Schema

### Design Template

**Important:** Schema names use territory codes for all deployments:

- **Single-territory pod:** Use territory code directly (e.g., `territory_dk`)
- **Multi-territory pod:** Use territory codes for each territory (e.g., `territory_dk`, `territory_no`, `territory_se`)
- **Template:** All examples below use `{schema_name}` as placeholder

Each territory schema has identical structure. Migration scripts should accept `schema_name` as parameter.

```sql
-- Single-territory pod example (Denmark)
CREATE SCHEMA IF NOT EXISTS territory_dk;

-- Multi-territory pod example (hosting dk, no, se)
CREATE SCHEMA IF NOT EXISTS territory_dk;  -- Denmark
CREATE SCHEMA IF NOT EXISTS territory_no;  -- Norway  
CREATE SCHEMA IF NOT EXISTS territory_se;  -- Sweden
```

### 1. Users Table

**Purpose:** Core user authentication and identity

**Note:** Email is **OPTIONAL** - used only for external notifications (invitations, password reset), not authentication.

```sql
CREATE TABLE {schema_name}.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Authentication (Username is primary identifier)
    username VARCHAR(50) NOT NULL UNIQUE,      -- Territory-local unique (matches global)
    email VARCHAR(255) UNIQUE,                 -- OPTIONAL - for notifications only
    password_hash VARCHAR(255) NOT NULL,       -- bcrypt/argon2
    
    -- Profile
    full_name VARCHAR(255),
    
    -- Territory Binding
    territory_code VARCHAR(10) NOT NULL DEFAULT 'dk', -- Denormalized for queries
    
    -- Account Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    
    -- Two-Factor Auth
    totp_secret VARCHAR(255),                  -- Encrypted TOTP secret
    totp_enabled BOOLEAN NOT NULL DEFAULT false,
    
    -- Soft Delete
    deleted_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (email IS NULL OR email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$'),
    CHECK (char_length(username) >= 3 AND char_length(username) <= 50),
    CHECK (deleted_at IS NULL OR is_active = false)
);

CREATE INDEX idx_users_username ON {schema_name}.users(username);
CREATE INDEX idx_users_email ON {schema_name}.users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_users_active ON {schema_name}.users(is_active) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_verified ON {schema_name}.users(is_verified);
```

**Holochain Mapping:**

```rust
// Agent Entry (built-in Holochain type)
// username stored in Agent's source chain
// email: NOT stored on-chain (privacy - used only for external notifications)
// Password hash: Stored in separate auth service (not on DHT)
```

**Migration Notes:**

- `id` (UUID) → `AgentPubKey` (Holochain agent identifier)
- `username` → **Primary identifier** (never changes, globally unique across all pods/territories)
- `email` → **Optional** (stored off-chain for privacy, used only for external notifications: invitations, password reset)
- `password_hash` → Stored in separate auth service (not on DHT)
- `is_active`, `is_verified` → Validation rules in zome

**Identity System Integration:**

- **Username:** Primary human-readable identifier (globally unique, never changes even with territory migration)
- **Matrix ID:** Derived from `username@territory` (e.g., `@alice:unityplan.dk`)
- **Territory Migration:** Username stays same, Matrix ID changes (old ID becomes alias)
  - Before migration: `@alice:unityplan.dk` (primary)
  - After migration: `@alice:unityplan.no` (new primary), `@alice:unityplan.dk` (alias - still works)
- **Key Point:** Username is the anchor - Matrix ID is just a federated representation
- See [Identity System Architecture](identity-system.md) for complete details

### 2. Profiles Table

**Purpose:** User profile data (public & private)

```sql
CREATE TABLE {schema_name}.users_profiles (
    user_id UUID PRIMARY KEY REFERENCES {schema_name}.users(id) ON DELETE CASCADE,
    
    -- Display Info
    display_name VARCHAR(100),
    avatar_url VARCHAR(500),                   -- S3/IPFS URL
    
    -- Rich Profile
    bio VARCHAR(280),                          -- Short tagline
    about TEXT,                                -- Long-form (Markdown)
    
    -- Arrays (PostgreSQL native, JSON in Holochain)
    interests TEXT[],                          -- Array of tags
    skills TEXT[],                             -- Array of tags
    languages VARCHAR(10)[],                   -- Array of ISO 639-1 codes
    
    -- Location (privacy-aware encoded format)
    location VARCHAR(500),                     -- "[lat,lng]Display Name"
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_profiles_display_name ON {schema_name}.users_profiles(display_name);
CREATE INDEX idx_users_profiles_interests ON {schema_name}.users_profiles USING GIN(interests);
CREATE INDEX idx_users_profiles_skills ON {schema_name}.users_profiles USING GIN(skills);

-- Full-text search
CREATE INDEX idx_users_profiles_search ON {schema_name}.users_profiles USING GIN(
    to_tsvector('english', 
        COALESCE(display_name, '') || ' ' || 
        COALESCE(bio, '') || ' ' || 
        COALESCE(about, '')
    )
);
```

**Skills & Interests Search Examples:**

GIN (Generalized Inverted Index) enables fast array-based searches for finding users by skills or interests.

```sql
-- Find all users with "beekeeping" skill
SELECT u.username, p.display_name, p.location, p.skills
FROM {schema_name}.users u
JOIN {schema_name}.users_profiles p ON u.id = p.user_id
WHERE p.skills @> ARRAY['beekeeping'];

-- Find users with ANY of multiple skills (OR)
SELECT u.username, p.display_name, p.skills
FROM {schema_name}.users u
JOIN {schema_name}.users_profiles p ON u.id = p.user_id
WHERE p.skills && ARRAY['beekeeping', 'permaculture', 'carpentry'];

-- Find users with ALL of multiple skills (AND)
SELECT u.username, p.display_name, p.skills
FROM {schema_name}.users u
JOIN {schema_name}.users_profiles p ON u.id = p.user_id
WHERE p.skills @> ARRAY['beekeeping', 'permaculture'];

-- Find closest user with specific skill (sorted by distance)
WITH user_locations AS (
    SELECT 
        u.username,
        p.display_name,
        p.skills,
        p.location,
        -- Extract coordinates from "[lat,lng]Display Name" format
        CAST(substring(p.location from '\[([0-9.-]+),') AS FLOAT) AS lat,
        CAST(substring(p.location from ',([0-9.-]+)\]') AS FLOAT) AS lng
    FROM {schema_name}.users u
    JOIN {schema_name}.users_profiles p ON u.id = p.user_id
    WHERE p.skills @> ARRAY['beekeeping']
      AND p.location IS NOT NULL
)
SELECT 
    username,
    display_name,
    skills,
    location,
    -- Calculate distance in km using Haversine formula
    (6371 * acos(
        cos(radians(:user_lat)) * 
        cos(radians(lat)) * 
        cos(radians(lng) - radians(:user_lng)) + 
        sin(radians(:user_lat)) * 
        sin(radians(lat))
    )) AS distance_km
FROM user_locations
ORDER BY distance_km
LIMIT 10;

-- Case-insensitive skill search
SELECT u.username, p.display_name, p.skills
FROM {schema_name}.users u
JOIN {schema_name}.users_profiles p ON u.id = p.user_id
WHERE EXISTS (
    SELECT 1 FROM unnest(p.skills) skill 
    WHERE LOWER(skill) = LOWER('Beekeeping')
);
```

**Tag Format Best Practices:**

- Use lowercase: "beekeeping" not "Beekeeping"
- Hyphenate compound words: "web-development" not "web development"
- Use singular form: "beekeeping" not "beekeepings"
- Implement autocomplete in frontend to suggest standardized tags

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct Profile {
    pub agent: AgentPubKey,          // Owner of this profile
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub about: Option<String>,
    pub interests: Vec<String>,
    pub skills: Vec<String>,
    pub languages: Vec<String>,
    pub location: Option<String>,
}

// Entry type definition
entry_defs![Profile::entry_def()];

// Link from Agent to Profile
create_link(agent_pub_key, profile_hash, "profile")?;
```

### 3. Profile Links Table

**Purpose:** Flexible external links (replaces deprecated social link columns)

```sql
CREATE TABLE {schema_name}.users_profile_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES {schema_name}.users(id) ON DELETE CASCADE,
    
    -- Link Data
    label VARCHAR(100) NOT NULL,               -- "My Portfolio", "GitHub"
    url VARCHAR(500) NOT NULL,                 -- Full URL
    icon VARCHAR(50),                          -- Icon name or emoji
    
    -- Display Control
    display_order INT NOT NULL DEFAULT 0,      -- Sort order
    is_visible BOOLEAN NOT NULL DEFAULT true,  -- Show/hide
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (url ~* '^https?://'),               -- Must be valid URL
    UNIQUE (user_id, display_order)            -- Unique order per user
);

CREATE INDEX idx_users_profile_links_user ON {schema_name}.users_profile_links(user_id);
CREATE INDEX idx_users_profile_links_order ON {schema_name}.users_profile_links(user_id, display_order);

-- Limit to 10 links per user (trigger or application logic)
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct ProfileLink {
    pub agent: AgentPubKey,
    pub label: String,
    pub url: String,
    pub icon: Option<String>,
    pub display_order: u32,
    pub is_visible: bool,
}

// Link from Profile to ProfileLink entries
create_link(profile_hash, link_hash, "profile_link")?;
```

### 4. Language Proficiency Table

**Purpose:** User's language skills visible to other users (public profile info)

```sql
CREATE TABLE {schema_name}.users_language_proficiency (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES {schema_name}.users(id) ON DELETE CASCADE,
    
    -- Language Details
    language_code VARCHAR(10) NOT NULL,        -- ISO 639-1 (e.g., "en", "da", "es")
    language_name VARCHAR(100) NOT NULL,       -- "English", "Dansk", "Español"
    
    -- Proficiency Levels
    spoken_level VARCHAR(20) NOT NULL DEFAULT 'basic',
        CHECK (spoken_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')),
    written_level VARCHAR(20) NOT NULL DEFAULT 'basic',
        CHECK (written_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')),
    reading_level VARCHAR(20) NOT NULL DEFAULT 'basic',
        CHECK (reading_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')),
    listening_level VARCHAR(20) NOT NULL DEFAULT 'basic',
        CHECK (listening_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')),
    
    -- Display Control
    display_order INT NOT NULL DEFAULT 0,      -- User's preference order
    is_preferred BOOLEAN NOT NULL DEFAULT false, -- Primary/preferred language
    show_on_profile BOOLEAN NOT NULL DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE (user_id, language_code)             -- One entry per language per user
);

CREATE INDEX idx_users_language_proficiency_user ON {schema_name}.users_language_proficiency(user_id);
CREATE INDEX idx_users_language_proficiency_order ON {schema_name}.users_language_proficiency(user_id, display_order);
CREATE INDEX idx_users_language_proficiency_preferred ON {schema_name}.users_language_proficiency(user_id, is_preferred) WHERE is_preferred = true;
```

**Usage:**

- User's preferred language marked with `is_preferred = true`
- Secondary languages ordered by `display_order`
- Different proficiency for each skill (speaking/writing/reading/listening)
- Public visibility controlled by `show_on_profile`

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct LanguageProficiency {
    pub agent: AgentPubKey,
    pub language_code: String,
    pub language_name: String,
    pub spoken_level: ProficiencyLevel,
    pub written_level: ProficiencyLevel,
    pub reading_level: ProficiencyLevel,
    pub listening_level: ProficiencyLevel,
    pub display_order: u32,
    pub is_preferred: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProficiencyLevel {
    Native,
    Fluent,
    Advanced,
    Intermediate,
    Basic,
    Learning,
}
```

### 5. Privacy Settings Table

**Purpose:** User privacy preferences

```sql
CREATE TABLE territory_dk.privacy_settings (
    user_id UUID PRIMARY KEY REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Profile Visibility
    profile_visibility VARCHAR(20) NOT NULL DEFAULT 'public',
        CHECK (profile_visibility IN ('public', 'connections_only', 'private')),
    
    -- Field Visibility
    show_email BOOLEAN NOT NULL DEFAULT false,
    show_full_name BOOLEAN NOT NULL DEFAULT true,
    show_location BOOLEAN NOT NULL DEFAULT true,
    show_connections BOOLEAN NOT NULL DEFAULT true,
    
    -- Interaction Permissions
    allow_messages_from VARCHAR(20) NOT NULL DEFAULT 'everyone',
        CHECK (allow_messages_from IN ('everyone', 'connections_only', 'nobody')),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct PrivacySettings {
    pub agent: AgentPubKey,
    pub profile_visibility: ProfileVisibility,
    pub show_email: bool,
    pub show_full_name: bool,
    pub show_location: bool,
    pub show_connections: bool,
    pub allow_messages_from: MessagePermission,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProfileVisibility {
    Public,
    ConnectionsOnly,
    Private,
}
```

### 5. User Settings Table

**Purpose:** App preferences (appearance, notifications, etc.)

```sql
CREATE TABLE {schema_name}.users_settings (
    user_id UUID PRIMARY KEY REFERENCES {schema_name}.users(id) ON DELETE CASCADE,
    
    -- Appearance
    theme_mode VARCHAR(20) NOT NULL DEFAULT 'system',
        CHECK (theme_mode IN ('light', 'dark', 'system')),
    color_scheme VARCHAR(50) NOT NULL DEFAULT 'forest-green',
        CHECK (color_scheme IN ('forest-green', 'ocean-blue', 'royal-purple', 'custom')),
    reduced_motion BOOLEAN NOT NULL DEFAULT false,
    wide_content_view BOOLEAN NOT NULL DEFAULT false,
    compact_mode BOOLEAN NOT NULL DEFAULT false,
    
    -- Locale & Language Preferences
    preferred_language VARCHAR(10) NOT NULL DEFAULT 'en',  -- ISO 639-1 code (UI language)
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    
    -- Translation Settings (for consuming content in other languages)
    auto_translate BOOLEAN NOT NULL DEFAULT true,         -- Auto-translate content not in preferred language
    translation_provider VARCHAR(30) NOT NULL DEFAULT 'libre-translate',
        CHECK (translation_provider IN ('libre-translate', 'deepl', 'google', 'microsoft')),
    contribute_translations BOOLEAN NOT NULL DEFAULT true, -- Share local translations with community
    fallback_to_english BOOLEAN NOT NULL DEFAULT true,    -- Use English if preferred language unavailable
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Translation Flow:**

1. User views content in foreign language
2. Client-side translation (privacy-first, on user's device)
3. If `contribute_translations = true`, translated content pushed to translation memory
4. Other users with same language pair benefit from cached translation
5. Collaborative translation effort reduces API costs and improves privacy

**Holochain Mapping:**

- Store in agent's source chain (private data)
- Not replicated to DHT (local preferences)

### 6. Notification Settings Table

**Purpose:** Notification delivery preferences

```sql
CREATE TABLE {schema_name}.users_notification_settings (
    user_id UUID PRIMARY KEY REFERENCES {schema_name}.users(id) ON DELETE CASCADE,
    
    -- Email Notifications
    email_digest BOOLEAN NOT NULL DEFAULT true,
    email_messages BOOLEAN NOT NULL DEFAULT true,
    email_followers BOOLEAN NOT NULL DEFAULT true,
    email_community BOOLEAN NOT NULL DEFAULT false,
    email_updates BOOLEAN NOT NULL DEFAULT true,
    
    -- In-App Notifications
    inapp_messages BOOLEAN NOT NULL DEFAULT true,
    inapp_followers BOOLEAN NOT NULL DEFAULT true,
    inapp_community BOOLEAN NOT NULL DEFAULT true,
    inapp_mentions BOOLEAN NOT NULL DEFAULT true,
    inapp_likes BOOLEAN NOT NULL DEFAULT false,
    
    -- Push Notifications
    push_enabled BOOLEAN NOT NULL DEFAULT false,
    push_messages BOOLEAN NOT NULL DEFAULT false,
    push_followers BOOLEAN NOT NULL DEFAULT false,
    push_community BOOLEAN NOT NULL DEFAULT false,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 7. Invitation Tokens Table

**Purpose:** Territory-local invitation tokens

See [invitation-system.md](invitation-system.md) for complete specification.

```sql
CREATE TABLE territory_dk.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(64) UNIQUE NOT NULL,         -- Cryptographically random
    token_type VARCHAR(20) NOT NULL CHECK (token_type IN ('single_use', 'group')),
    
    -- Restrictions
    email VARCHAR(255),                        -- Optional: for email delivery, can be NULL for QR/link sharing
    max_uses INT NOT NULL DEFAULT 1,
    used_count INT NOT NULL DEFAULT 0,
    
    -- Metadata
    created_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    community_id UUID REFERENCES territory_dk.communities(id),
    purpose TEXT,
    metadata JSONB,
    
    -- Lifecycle
    expires_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    revoked_at TIMESTAMPTZ,
    revoked_by_user_id UUID REFERENCES territory_dk.users(id),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (
        (token_type = 'single_use' AND max_uses = 1) OR
        (token_type = 'group' AND max_uses > 1)
    ),
    CHECK (used_count <= max_uses)
);

CREATE INDEX idx_invitation_tokens_token ON territory_dk.invitation_tokens(token);
CREATE INDEX idx_invitation_tokens_email ON territory_dk.invitation_tokens(email) WHERE email IS NOT NULL;
CREATE INDEX idx_invitation_tokens_created_by ON territory_dk.invitation_tokens(created_by_user_id);
CREATE INDEX idx_invitation_tokens_active ON territory_dk.invitation_tokens(is_active, expires_at);
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct InvitationToken {
    pub token: String,
    pub token_type: TokenType,
    pub created_by: AgentPubKey,
    pub email: Option<String>,
    pub max_uses: u32,
    pub used_count: u32,
    pub expires_at: Timestamp,
    pub is_active: bool,
}

// Link from creator to invitation
create_link(creator_agent, invitation_hash, "created_invitation")?;
```

### 8. Invitation Uses Table

**Purpose:** Audit trail of invitation token usage

```sql
CREATE TABLE territory_dk.invitation_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invitation_token_id UUID NOT NULL REFERENCES territory_dk.invitation_tokens(id),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    
    -- Audit Data
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,                           -- Optional: security tracking
    user_agent TEXT                            -- Optional: device tracking
);

CREATE INDEX idx_invitation_uses_token ON territory_dk.invitation_uses(invitation_token_id);
CREATE INDEX idx_invitation_uses_user ON territory_dk.invitation_uses(user_id);
```

**Holochain Mapping:**

- Immutable source chain entry
- Link from invitation to user who redeemed it

---

### 9. Communities Table

**Purpose:** Communities within territories (guilds, learning circles, local chapters, study groups)

**Design Decision:** Communities use **UUID primary keys** instead of hierarchical string IDs to support unlimited parent/child nesting. The hierarchy is tracked via `parent_community_id`, and the full path can be reconstructed by traversing parent relationships.

**Access Model:**

- **ALL users can VIEW all communities** (read-only by default) - for inspiration and discovery
- **Territory Communities** (geographic/place-based): Membership via invitation only
- **Guild Communities** (interest-based): Membership via badge OR invitation OR public join
- Members can actively participate (post, comment, join events), non-members can only observe

```sql
CREATE TABLE territory_dk.communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,                -- Simple name (e.g., "Copenhagen", "Beekeepers Guild")
    slug VARCHAR(255) NOT NULL,                -- URL-friendly identifier
    description TEXT,
    
    -- Type and Access Control
    community_type VARCHAR(50) NOT NULL,       -- 'territory' (place-based) or 'guild' (interest-based)
    access_badge_id UUID REFERENCES territory_dk.badge_definitions(id), -- Badge required to join (guilds only, optional)
    
    -- Hierarchy
    parent_community_id UUID REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    territory_code VARCHAR(10) NOT NULL,       -- Which territory this community belongs to
    depth INTEGER NOT NULL DEFAULT 0,          -- Hierarchy depth (0 = top-level, 1 = child, etc.)
    
    -- Membership Settings
    join_policy VARCHAR(50) NOT NULL DEFAULT 'open', -- 'open', 'invite_only', 'badge_required', 'approval_required'
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE, -- Requires admin approval to join
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,        -- Flexible storage (tags, location, etc.)
    
    -- Lifecycle
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    
    -- Constraints
    CONSTRAINT uq_community_slug_territory UNIQUE (slug, territory_code),
    CHECK (depth >= 0),
    CHECK (community_type IN ('territory', 'guild')),
    CHECK (join_policy IN ('open', 'invite_only', 'badge_required', 'approval_required')),
    -- Territory communities are invitation-based, guilds can require badges
    CHECK (
        (community_type = 'territory' AND join_policy = 'invite_only') OR
        (community_type = 'guild' AND join_policy IN ('open', 'badge_required', 'approval_required', 'invite_only'))
    ),
    -- Badge required only for guilds with badge_required policy
    CHECK (
        (join_policy = 'badge_required' AND access_badge_id IS NOT NULL) OR
        (join_policy != 'badge_required')
    )
);

CREATE INDEX idx_communities_parent ON territory_dk.communities(parent_community_id);
CREATE INDEX idx_communities_territory ON territory_dk.communities(territory_code);
CREATE INDEX idx_communities_slug ON territory_dk.communities(slug);
CREATE INDEX idx_communities_active ON territory_dk.communities(is_active);
CREATE INDEX idx_communities_depth ON territory_dk.communities(depth);
CREATE INDEX idx_communities_type ON territory_dk.communities(community_type);
CREATE INDEX idx_communities_badge ON territory_dk.communities(access_badge_id) WHERE access_badge_id IS NOT NULL;
```

**Example Communities:**

```sql
-- Territory community (place-based, invitation only)
INSERT INTO territory_dk.communities (
    id, name, slug, community_type, join_policy, territory_code, depth, created_by_user_id
)
VALUES (
    'uuid-copenhagen', 
    'Copenhagen', 
    'copenhagen', 
    'territory',      -- Place-based community
    'invite_only',    -- Can only join via invitation
    'DK', 
    0, 
    'creator-uuid'
);

-- Guild community (interest-based, open to all)
INSERT INTO territory_dk.communities (
    id, name, slug, community_type, join_policy, territory_code, depth, created_by_user_id
)
VALUES (
    'uuid-beekeepers-guild', 
    'Beekeepers Guild', 
    'beekeepers-guild', 
    'guild',          -- Interest-based community
    'open',           -- Anyone can join
    'DK', 
    0, 
    'creator-uuid'
);

-- Guild community (interest-based, badge required)
INSERT INTO territory_dk.communities (
    id, name, slug, community_type, join_policy, access_badge_id, territory_code, depth, created_by_user_id
)
VALUES (
    'uuid-advanced-permaculture', 
    'Advanced Permaculture Guild', 
    'advanced-permaculture', 
    'guild',                           -- Interest-based community
    'badge_required',                  -- Requires badge to join
    'uuid-permaculture-cert-badge',    -- Badge ID required
    'DK', 
    0, 
    'creator-uuid'
);

-- Nested territory community (sub-community of Copenhagen)
INSERT INTO territory_dk.communities (
    id, name, slug, parent_community_id, community_type, join_policy, territory_code, depth, created_by_user_id
)
VALUES (
    'uuid-valby', 
    'Valby', 
    'valby', 
    'uuid-copenhagen',  -- Parent community
    'territory',        -- Place-based
    'invite_only',      -- Invitation required
    'DK', 
    1,                  -- Child depth
    'creator-uuid'
);
```

**Access Control Examples:**

```sql
-- Check if user can JOIN a community (not just view)
CREATE OR REPLACE FUNCTION territory_dk.user_can_join_community(
    p_user_id UUID,
    p_community_id UUID
) RETURNS BOOLEAN AS $$
DECLARE
    v_community_type VARCHAR(50);
    v_join_policy VARCHAR(50);
    v_access_badge_id UUID;
    v_has_badge BOOLEAN;
    v_is_member BOOLEAN;
BEGIN
    -- Get community details
    SELECT community_type, join_policy, access_badge_id
    INTO v_community_type, v_join_policy, v_access_badge_id
    FROM territory_dk.communities
    WHERE id = p_community_id AND is_active = TRUE;
    
    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;
    
    -- Check if already a member
    SELECT EXISTS(
        SELECT 1
        FROM territory_dk.community_members
        WHERE user_id = p_user_id
          AND community_id = p_community_id
          AND left_at IS NULL
    ) INTO v_is_member;
    
    IF v_is_member THEN
        RETURN TRUE;  -- Already a member
    END IF;
    
    -- Check join policy
    IF v_join_policy = 'open' THEN
        RETURN TRUE;  -- Anyone can join
    ELSIF v_join_policy = 'invite_only' THEN
        RETURN FALSE;  -- Requires invitation (handled separately)
    ELSIF v_join_policy = 'badge_required' THEN
        -- Check if user has the required badge
        SELECT EXISTS(
            SELECT 1
            FROM territory_dk.badge_awards
            WHERE user_id = p_user_id
              AND badge_id = v_access_badge_id
              AND revoked_at IS NULL
              AND (expires_at IS NULL OR expires_at > NOW())
        ) INTO v_has_badge;
        RETURN v_has_badge;
    ELSIF v_join_policy = 'approval_required' THEN
        RETURN TRUE;  -- Can request to join (pending approval)
    END IF;
    
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;

-- Note: ALL users can VIEW all communities regardless of membership or badges
-- This function only controls who can JOIN/PARTICIPATE actively
```

**Community Types Explained:**

1. **Territory Communities** (`community_type = 'territory'`):
   - **Purpose**: Geographic/place-based communities (cities, neighborhoods, regions)
   - **Join Policy**: ALWAYS `invite_only` (enforced by CHECK constraint)
   - **Membership**: Via invitation tokens only
   - **Example**: Copenhagen, Valby, Aarhus
   - **Rationale**: You can't join a place-based community unless you're invited (typically because you live there or have connection)

2. **Guild Communities** (`community_type = 'guild'`):
   - **Purpose**: Interest-based communities (beekeepers, developers, artists)
   - **Join Policies**:
     - `open`: Anyone can join freely
     - `badge_required`: Must have specific badge to join
     - `approval_required`: Can request to join, admin approves
     - `invite_only`: Only via invitation
   - **Example**: Beekeepers Guild (open), Advanced Permaculture (badge required)
   - **Rationale**: Interest-based groups can set their own entry requirements

**Universal View Access:**

ALL users can VIEW all communities regardless of:

- Community type
- Join policy
- Badge requirements
- Membership status

**Why?**

- **Discovery**: Users see what's happening and get inspired
- **Transparency**: Territory communities visible before moving to a place
- **Motivation**: Guild activities visible to encourage learning and participation
- **Roadmap**: Users see what they could unlock by earning badges

**Participation vs Viewing:**

| Action | Non-Members | Members |
|--------|-------------|---------|
| View community info | ✅ All users | ✅ Members |
| View community posts/events | ✅ All users | ✅ Members |
| Comment/post/participate | ❌ No | ✅ Yes |
| Join events | ❌ No (view only) | ✅ Yes |
| Vote in community decisions | ❌ No | ✅ Yes |

```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct Community {
    pub name: String,
    pub description: String,
    pub community_type: CommunityType,
    pub join_policy: JoinPolicy,
    pub access_badge: Option<ActionHash>,  // Badge required to join (if badge_required)
    pub parent_community: Option<ActionHash>, // Link to parent community entry
    pub created_by: AgentPubKey,
}

enum CommunityType {
    Territory,  // Place-based (cities, neighborhoods)
    Guild,      // Interest-based (beekeepers, developers)
}

enum JoinPolicy {
    Open,              // Anyone can join freely
    InviteOnly,        // Requires invitation
    BadgeRequired,     // Must have specific badge
    ApprovalRequired,  // Request to join, admin approves
}

// Links:
// - Creator → Community
// - Parent Community → Child Community (if parent exists)
// - Territory → Community (via anchor)
// - Badge → Community (if badge required for access)

// Note: All communities are publicly readable in Holochain DHT
// Access control only applies to participation (posting, voting, etc.)
```

---

### 10. Community Members Table

**Purpose:** Track user membership in communities

```sql
CREATE TABLE territory_dk.community_members (
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    -- Membership status
    role VARCHAR(50) NOT NULL DEFAULT 'member',  -- 'member', 'moderator', 'admin'
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    approved_at TIMESTAMPTZ,                     -- NULL if requires_approval and not yet approved
    approved_by_user_id UUID REFERENCES territory_dk.users(id),
    
    -- Lifecycle
    left_at TIMESTAMPTZ,                         -- NULL if currently a member
    
    PRIMARY KEY (user_id, community_id)
);

CREATE INDEX idx_community_members_user ON territory_dk.community_members(user_id);
CREATE INDEX idx_community_members_community ON territory_dk.community_members(community_id);
CREATE INDEX idx_community_members_role ON territory_dk.community_members(role);
CREATE INDEX idx_community_members_active ON territory_dk.community_members(user_id, community_id) 
    WHERE left_at IS NULL;
```

**Holochain Mapping:**

```rust
// Link from Community to Member
create_link(community_hash, user_agent_pub_key, "member", LinkTag::new("role:member"))?;

// Link from User to Community (reverse lookup)
create_link(user_agent_pub_key, community_hash, "joined_community")?;
```

---

### 11. Roles Table

**Purpose:** Define roles that can be assigned to users (territory-level and community-level)

**Note:** LMS and Forum-specific roles will be added later as extensions

```sql
CREATE TABLE territory_dk.roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,                -- 'territory_manager', 'community_manager', etc.
    scope VARCHAR(50) NOT NULL,                -- 'territory', 'community', 'user'
    description TEXT,
    
    -- Role metadata
    is_electable BOOLEAN NOT NULL DEFAULT FALSE, -- Can this role be elected democratically?
    requires_election BOOLEAN NOT NULL DEFAULT FALSE, -- Must this role be elected?
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Lifecycle
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uq_role_name_scope UNIQUE (name, scope)
);

CREATE INDEX idx_roles_scope ON territory_dk.roles(scope);
CREATE INDEX idx_roles_electable ON territory_dk.roles(is_electable);

-- Seed core platform and territory roles
-- Platform roles have LIMITED permissions due to inverted pyramid (users > communities > territories > global)
INSERT INTO territory_dk.roles (name, scope, description, is_electable, requires_election) VALUES
    -- Platform/Global roles (Limited by inverted pyramid - infrastructure only)
    ('platform_admin', 'global', 'System maintenance and oversight - READ access to all data, LIMITED write access', FALSE, FALSE),
    ('database_admin', 'global', 'Database maintenance, backups, performance tuning', FALSE, FALSE),
    ('security_engineer', 'global', 'Security audits, vulnerability management, incident response', FALSE, FALSE),
    ('devops_engineer', 'global', 'CI/CD pipelines, container orchestration, deployment automation', FALSE, FALSE),
    ('monitoring_specialist', 'global', 'Observability setup, alerting, incident detection', FALSE, FALSE),
    ('network_engineer', 'global', 'Network architecture, VPNs, firewalls', FALSE, FALSE),
    ('technical_support', 'global', 'Initial troubleshooting of infrastructure and user issues', FALSE, FALSE),
    
    -- Territory roles
    ('territory_manager', 'territory', 'Manages territory infrastructure and users', FALSE, FALSE),
    
    -- Community roles (Democratic elections)
    ('community_manager', 'community', 'Manages a specific community', TRUE, TRUE),
    ('community_moderator', 'community', 'Moderates community discussions', TRUE, TRUE),
    ('member', 'community', 'Standard community member', FALSE, FALSE);
```

**Platform Role Permissions (Inverted Pyramid):**

```sql
-- Example: Platform Admin has LIMITED permissions
-- ✅ CAN: Read all data (monitoring, troubleshooting)
-- ✅ CAN: Server management, database optimization, security updates
-- ✅ CAN: Analytics, performance metrics, system health monitoring
-- ❌ CANNOT: Modify user content, courses, or governance decisions
-- ❌ CANNOT: Override territory sovereignty or user data ownership
-- All actions logged in immutable audit trail
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct Role {
    pub name: String,
    pub scope: RoleScope,
    pub description: String,
    pub is_electable: bool,
}

enum RoleScope {
    Territory,
    Community,
    User,
}
```

---

### 12. Role Assignments Table

**Purpose:** Assign roles to users at territory or community level

```sql
CREATE TABLE territory_dk.role_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES territory_dk.roles(id) ON DELETE CASCADE,
    
    -- Scope
    community_id UUID REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    -- community_id is NULL for territory-level roles, NOT NULL for community-level roles
    
    -- Assignment metadata
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assigned_by_user_id UUID REFERENCES territory_dk.users(id), -- NULL if elected
    assigned_via_election_id UUID,                    -- NULL if assigned manually
    
    -- Expiration (optional)
    expires_at TIMESTAMPTZ,                           -- NULL = no expiration
    
    -- Lifecycle
    revoked_at TIMESTAMPTZ,
    revoked_by_user_id UUID REFERENCES territory_dk.users(id),
    revoked_reason TEXT,
    
    -- Constraints
    CONSTRAINT chk_community_scope CHECK (
        (community_id IS NULL AND role_id IN (
            SELECT id FROM territory_dk.roles WHERE scope IN ('territory', 'user')
        ))
        OR
        (community_id IS NOT NULL AND role_id IN (
            SELECT id FROM territory_dk.roles WHERE scope = 'community'
        ))
    ),
    
    -- Prevent duplicate active assignments
    CONSTRAINT uq_active_role_assignment UNIQUE (user_id, role_id, community_id, revoked_at)
);

CREATE INDEX idx_role_assignments_user ON territory_dk.role_assignments(user_id);
CREATE INDEX idx_role_assignments_role ON territory_dk.role_assignments(role_id);
CREATE INDEX idx_role_assignments_community ON territory_dk.role_assignments(community_id);
CREATE INDEX idx_role_assignments_active ON territory_dk.role_assignments(user_id, role_id, community_id)
    WHERE revoked_at IS NULL;
```

**Holochain Mapping:**

```rust
// Link from User to Role Assignment
create_link(user_agent_pub_key, role_assignment_hash, "has_role")?;

// Link from Community to Role Assignment (for community-scoped roles)
create_link(community_hash, role_assignment_hash, "role_holder")?;
```

---

### 13. Community Role Elections Table

**Purpose:** Democratic elections for community roles (100% unanimous vote required)

See [overview.md](../project/overview.md#community-level-democratic-role-elections) for complete election process.

```sql
CREATE TABLE territory_dk.community_role_elections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES territory_dk.roles(id),
    
    -- Election type
    election_type VARCHAR(20) NOT NULL CHECK (election_type IN ('elect', 'remove')),
    
    -- Nominee and nominator
    nominee_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    nominated_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Voting period
    voting_period_starts TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    voting_period_ends TIMESTAMPTZ NOT NULL,
    
    -- Eligible voters (snapshot at election creation)
    eligible_voter_count INTEGER NOT NULL,     -- Total eligible voters
    eligible_voter_ids UUID[] NOT NULL,        -- Array of eligible voter user IDs
    
    -- Election status
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'passed', 'failed', 'expired')),
    
    -- Results (populated when finalized)
    votes_for INTEGER DEFAULT 0,
    votes_against INTEGER DEFAULT 0,
    votes_total INTEGER DEFAULT 0,
    finalized_at TIMESTAMPTZ,
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,        -- Purpose, reasoning, etc.
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (voting_period_ends > voting_period_starts),
    CHECK (eligible_voter_count > 0),
    CHECK (votes_total <= eligible_voter_count)
);

CREATE INDEX idx_elections_community ON territory_dk.community_role_elections(community_id);
CREATE INDEX idx_elections_nominee ON territory_dk.community_role_elections(nominee_user_id);
CREATE INDEX idx_elections_status ON territory_dk.community_role_elections(status);
CREATE INDEX idx_elections_active ON territory_dk.community_role_elections(community_id, status)
    WHERE status = 'active';
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct CommunityRoleElection {
    pub community_hash: ActionHash,
    pub role_name: String,
    pub election_type: ElectionType,
    pub nominee: AgentPubKey,
    pub nominated_by: AgentPubKey,
    pub voting_period_ends: Timestamp,
    pub eligible_voters: Vec<AgentPubKey>,
}

enum ElectionType {
    Elect,
    Remove,
}

// Links:
// - Community → Election
// - Election → Nominee
```

---

### 14. Community Role Election Votes Table

**Purpose:** Track individual votes in community role elections

```sql
CREATE TABLE territory_dk.community_role_election_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    election_id UUID NOT NULL REFERENCES territory_dk.community_role_elections(id) ON DELETE CASCADE,
    voter_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Vote
    approved BOOLEAN NOT NULL,                 -- TRUE = yes, FALSE = no
    
    -- Timestamp
    voted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Metadata
    comment TEXT,                              -- Optional reasoning
    
    -- Prevent duplicate votes
    CONSTRAINT uq_election_vote UNIQUE (election_id, voter_user_id)
);

CREATE INDEX idx_election_votes_election ON territory_dk.community_role_election_votes(election_id);
CREATE INDEX idx_election_votes_voter ON territory_dk.community_role_election_votes(voter_user_id);
```

**Election Finalization Logic:**

When all eligible voters have voted (or voting period ends):

1. Count votes: `votes_for`, `votes_against`, `votes_total`
2. Check for **100% unanimous approval**: `votes_for = eligible_voter_count`
3. If unanimous:
   - `election_type = 'elect'`: Create `role_assignment` for nominee
   - `election_type = 'remove'`: Revoke existing `role_assignment`
4. Update election `status` to 'passed' or 'failed'
5. Set `finalized_at` timestamp

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct ElectionVote {
    pub election_hash: ActionHash,
    pub voter: AgentPubKey,
    pub approved: bool,
    pub voted_at: Timestamp,
}

// Links:
// - Election → Vote
// - Voter → Vote (for audit)

// Validation:
// - Voter must be in eligible_voters list
// - Only one vote per voter per election
// - Election must be active
```

---

### 15. Badge Definitions Table

**Purpose:** Define badges that grant permissions and can be earned through courses or assigned

**Note:** Badges are the **cornerstone of the permission system**. All access to courses, forums, and administrative functions is controlled by badges.

```sql
CREATE TABLE territory_dk.badge_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(100) UNIQUE NOT NULL,         -- Unique identifier (e.g., 'code_of_conduct', 'forum_moderator')
    name VARCHAR(255) NOT NULL,                -- Display name
    description TEXT NOT NULL,
    category VARCHAR(50) NOT NULL,             -- 'governance', 'learning', 'community', 'admin'
    
    -- Visual
    icon_url TEXT,                             -- Badge icon/image URL
    color VARCHAR(7),                          -- Hex color code (#FF5733)
    
    -- Permissions granted by this badge
    permissions JSONB DEFAULT '[]'::jsonb,     -- Array of permission strings
    
    -- Prerequisites
    prerequisite_badge_ids UUID[],             -- Must have these badges first
    
    -- Expiration
    requires_renewal BOOLEAN NOT NULL DEFAULT FALSE,
    renewal_period_days INTEGER,              -- NULL = never expires, 365 = annual renewal
    
    -- Criteria for earning (if course-based)
    earn_criteria JSONB DEFAULT '{}'::jsonb,   -- Course completion, achievements, etc.
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Lifecycle
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_user_id UUID REFERENCES territory_dk.users(id),
    
    CHECK (category IN ('governance', 'learning', 'community', 'admin'))
);

CREATE INDEX idx_badge_definitions_code ON territory_dk.badge_definitions(code);
CREATE INDEX idx_badge_definitions_category ON territory_dk.badge_definitions(category);
CREATE INDEX idx_badge_definitions_active ON territory_dk.badge_definitions(is_active);

-- Seed core mandatory badge
INSERT INTO territory_dk.badge_definitions (
    code, 
    name, 
    description, 
    category, 
    permissions, 
    requires_renewal, 
    renewal_period_days,
    is_active
) VALUES (
    'code_of_conduct',
    'Code of Conduct',
    'Mandatory badge - required for all platform participation. Must be renewed annually.',
    'governance',
    '["enroll_in_courses", "view_forums", "comment_on_topics", "join_communities"]'::jsonb,
    TRUE,
    365,  -- Annual renewal
    TRUE
);
```

**Special Badge: Code of Conduct**

- **REQUIRED** for all platform participation
- **Annual renewal** mandatory (365 days)
- **Automated notifications** at 30, 14, 7 days before expiration
- **Automatic lockout** on expiration:
  - ❌ Cannot enroll in courses
  - ❌ Cannot view/comment on forums
  - ❌ Cannot join communities
  - ✅ Can view/edit profile
  - ✅ Can delete account
  - ✅ Can retake Code of Conduct course

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct BadgeDefinition {
    pub code: String,
    pub name: String,
    pub description: String,
    pub category: BadgeCategory,
    pub permissions: Vec<String>,
    pub prerequisite_badges: Vec<ActionHash>,
    pub requires_renewal: bool,
    pub renewal_period_days: Option<u32>,
}

enum BadgeCategory {
    Governance,
    Learning,
    Community,
    Admin,
}
```

---

### 16. Badge Awards Table

**Purpose:** Track badge assignments to users (earned or manually assigned)

```sql
CREATE TABLE territory_dk.badge_awards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    badge_id UUID NOT NULL REFERENCES territory_dk.badge_definitions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Award source
    award_source VARCHAR(50) NOT NULL,         -- 'course_completion', 'manual_assignment', 'achievement'
    source_id UUID,                            -- Course ID, achievement ID, etc.
    
    -- Assignment metadata
    awarded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    awarded_by_user_id UUID REFERENCES territory_dk.users(id),
    awarded_reason TEXT,
    
    -- Expiration (calculated from badge.renewal_period_days)
    expires_at TIMESTAMPTZ,                    -- NULL = never expires
    
    -- Renewal tracking
    renewal_reminder_sent BOOLEAN NOT NULL DEFAULT FALSE,
    renewal_reminder_sent_at TIMESTAMPTZ,
    
    -- Revocation
    revoked_at TIMESTAMPTZ,
    revoked_by_user_id UUID REFERENCES territory_dk.users(id),
    revoked_reason TEXT,
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Constraints
    CHECK (award_source IN ('course_completion', 'manual_assignment', 'achievement')),
    
    -- Allow multiple awards of same badge (for renewal tracking)
    CONSTRAINT uq_active_badge_award UNIQUE (badge_id, user_id, revoked_at)
);

CREATE INDEX idx_badge_awards_user ON territory_dk.badge_awards(user_id);
CREATE INDEX idx_badge_awards_badge ON territory_dk.badge_awards(badge_id);
CREATE INDEX idx_badge_awards_expires ON territory_dk.badge_awards(expires_at) WHERE revoked_at IS NULL;
CREATE INDEX idx_badge_awards_active ON territory_dk.badge_awards(user_id, badge_id) 
    WHERE revoked_at IS NULL;
CREATE INDEX idx_badge_awards_expiring_soon ON territory_dk.badge_awards(expires_at)
    WHERE revoked_at IS NULL AND expires_at IS NOT NULL;
```

**Expiration Checking:**

```sql
-- Find badges expiring in the next 30 days (for reminders)
SELECT 
    ba.id,
    ba.user_id,
    bd.name,
    ba.expires_at,
    EXTRACT(DAY FROM (ba.expires_at - NOW())) as days_until_expiry
FROM territory_dk.badge_awards ba
JOIN territory_dk.badge_definitions bd ON bd.id = ba.badge_id
WHERE ba.revoked_at IS NULL
    AND ba.expires_at IS NOT NULL
    AND ba.expires_at <= NOW() + INTERVAL '30 days'
    AND ba.renewal_reminder_sent = FALSE
ORDER BY ba.expires_at ASC;
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct BadgeAward {
    pub badge_hash: ActionHash,
    pub user: AgentPubKey,
    pub award_source: AwardSource,
    pub awarded_at: Timestamp,
    pub expires_at: Option<Timestamp>,
}

enum AwardSource {
    CourseCompletion(ActionHash),
    ManualAssignment(AgentPubKey),
    Achievement(ActionHash),
}

// Links:
// - User → BadgeAward
// - Badge → BadgeAward
// - Course → BadgeAward (if course completion)
```

---

### 17. Badge Progress Table

**Purpose:** Track progress toward earning multi-step badges (achievements, milestones)

```sql
CREATE TABLE territory_dk.badge_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    badge_id UUID NOT NULL REFERENCES territory_dk.badge_definitions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Progress tracking
    progress JSONB NOT NULL DEFAULT '{}'::jsonb,  -- Flexible progress data
    current_step INTEGER NOT NULL DEFAULT 0,
    total_steps INTEGER NOT NULL,
    completion_percentage DECIMAL(5,2) GENERATED ALWAYS AS ((current_step::DECIMAL / NULLIF(total_steps, 0)) * 100) STORED,
    
    -- Timestamps
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    
    -- Constraints
    CHECK (current_step >= 0),
    CHECK (total_steps > 0),
    CHECK (current_step <= total_steps),
    
    CONSTRAINT uq_badge_progress UNIQUE (badge_id, user_id)
);

CREATE INDEX idx_badge_progress_user ON territory_dk.badge_progress(user_id);
CREATE INDEX idx_badge_progress_badge ON territory_dk.badge_progress(badge_id);
CREATE INDEX idx_badge_progress_incomplete ON territory_dk.badge_progress(user_id, badge_id)
    WHERE completed_at IS NULL;
```

**Example Usage:**

```sql
-- Track "100 Forum Comments" badge progress
INSERT INTO territory_dk.badge_progress (badge_id, user_id, progress, current_step, total_steps)
VALUES (
    '<forum_100_comments_badge_id>',
    '<user_id>',
    '{"comment_count": 45}'::jsonb,
    45,
    100
);

-- When user reaches 100 comments, award badge
UPDATE territory_dk.badge_progress
SET current_step = 100,
    completed_at = NOW()
WHERE id = '<progress_id>';

INSERT INTO territory_dk.badge_awards (badge_id, user_id, award_source)
VALUES ('<forum_100_comments_badge_id>', '<user_id>', 'achievement');
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct BadgeProgress {
    pub badge_hash: ActionHash,
    pub user: AgentPubKey,
    pub progress: ProgressData,
    pub current_step: u32,
    pub total_steps: u32,
    pub started_at: Timestamp,
}

// Links:
// - User → BadgeProgress
// - Badge → BadgeProgress
```

---

### 18. Groups Table (Access Bubbles)

**Purpose:** Groups are logical containers ("bubbles") that bundle together communities, forums (future extension), and courses (future extension). A badge acts as a "key" to unlock access to the entire group.

**Key Concept:** Instead of granting access to individual resources, users earn badges that unlock entire groups of related content. This creates cohesive learning/collaboration spaces.

**Example Use Cases:**

- "Platform Management" group → "Platform Management Access" badge → General Platform Management forum + Territory Management course
- "Platform Developer" group → "Platform Developer" badge → Development communities + Dev forums + Advanced courses
- "Beekeepers Network" group → "Beekeeping Basics" badge → Local beekeeping communities + Forums + Courses

**Scope Rules:**

- **Global scope**: Available to all users across all territories (if they have the badge)
- **Territory scope**: Available only to users in that territory (and child communities)
- **Community scope**: Available only from that community level and up in the hierarchy

```sql
CREATE TABLE territory_dk.groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(100) UNIQUE NOT NULL,         -- Unique identifier (e.g., 'platform_management', 'beekeeping_network')
    name VARCHAR(255) NOT NULL,                -- Display name
    description TEXT NOT NULL,
    
    -- Access control
    scope VARCHAR(50) NOT NULL,                -- 'global', 'territory', 'community'
    scope_id UUID,                             -- NULL for global, territory/community UUID for local
    access_badge_id UUID NOT NULL REFERENCES territory_dk.badge_definitions(id), -- Badge required to access this group
    
    -- Visual
    icon_url TEXT,
    color VARCHAR(7),                          -- Hex color code
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Lifecycle
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_user_id UUID REFERENCES territory_dk.users(id),
    
    CHECK (scope IN ('global', 'territory', 'community')),
    -- Global groups don't need scope_id, local groups require it
    CHECK (
        (scope = 'global' AND scope_id IS NULL) OR
        (scope IN ('territory', 'community') AND scope_id IS NOT NULL)
    )
);

CREATE INDEX idx_groups_code ON territory_dk.groups(code);
CREATE INDEX idx_groups_scope ON territory_dk.groups(scope);
CREATE INDEX idx_groups_scope_id ON territory_dk.groups(scope_id);
CREATE INDEX idx_groups_access_badge ON territory_dk.groups(access_badge_id);
CREATE INDEX idx_groups_active ON territory_dk.groups(is_active);

-- Example: Platform Management group
INSERT INTO territory_dk.groups (code, name, description, scope, access_badge_id, created_by_user_id)
VALUES (
    'platform_management',
    'Platform Management',
    'Access to platform administration resources, forums, and courses',
    'global',
    '<platform_management_access_badge_id>',
    '<platform_admin_user_id>'
);
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct Group {
    pub code: String,
    pub name: String,
    pub description: String,
    pub scope: GroupScope,
    pub access_badge: ActionHash,  // Badge required to access
    pub created_by: AgentPubKey,
}

enum GroupScope {
    Global,
    Territory(ActionHash),
    Community(ActionHash),
}

// Links:
// - Badge → Group (groups requiring this badge)
// - Territory → Group (if territory-scoped)
// - Community → Group (if community-scoped)
// - Creator → Group
```

---

### 19. Group Communities Table

**Purpose:** Link groups to communities. A group can contain multiple communities, creating logical groupings.

**Example:** "Beekeeping Network" group might include communities like "Urban Beekeepers", "Organic Beekeepers", "Commercial Beekeepers"

```sql
CREATE TABLE territory_dk.group_communities (
    group_id UUID NOT NULL REFERENCES territory_dk.groups(id) ON DELETE CASCADE,
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    -- Metadata
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by_user_id UUID REFERENCES territory_dk.users(id),
    
    PRIMARY KEY (group_id, community_id)
);

CREATE INDEX idx_group_communities_group ON territory_dk.group_communities(group_id);
CREATE INDEX idx_group_communities_community ON territory_dk.group_communities(community_id);
```

**Holochain Mapping:**

```rust
// Links:
// - Group → Community (communities in this group)
// - Community → Group (groups this community belongs to)
```

---

### 20. Group Forums Table (Future Extension)

**Purpose:** Link groups to forums when forum extension is added. A group can contain multiple forums for discussion.

**Note:** This table structure is defined now for completeness, but will only be populated when the forum extension is implemented.

```sql
CREATE TABLE territory_dk.group_forums (
    group_id UUID NOT NULL REFERENCES territory_dk.groups(id) ON DELETE CASCADE,
    forum_id UUID NOT NULL,  -- References forum table (when extension added)
    
    -- Metadata
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by_user_id UUID REFERENCES territory_dk.users(id),
    
    PRIMARY KEY (group_id, forum_id)
);

CREATE INDEX idx_group_forums_group ON territory_dk.group_forums(group_id);
CREATE INDEX idx_group_forums_forum ON territory_dk.group_forums(forum_id);

COMMENT ON TABLE territory_dk.group_forums IS 'Links groups to forums. Populated when forum extension is added.';
```

**Holochain Mapping:**

```rust
// Links:
// - Group → Forum (forums in this group)
// - Forum → Group (groups this forum belongs to)
```

---

### 21. Group Courses Table (Future Extension)

**Purpose:** Link groups to courses when LMS extension is added. A group can contain multiple courses.

**Note:** This table structure is defined now for completeness, but will only be populated when the LMS extension is implemented.

**Example:** "Territory Management" group → "Creating Territories" course + "Managing Territory Settings" course + "Territory Best Practices" course

```sql
CREATE TABLE territory_dk.group_courses (
    group_id UUID NOT NULL REFERENCES territory_dk.groups(id) ON DELETE CASCADE,
    course_id UUID NOT NULL,  -- References course table (when extension added)
    
    -- Metadata
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by_user_id UUID REFERENCES territory_dk.users(id),
    
    PRIMARY KEY (group_id, course_id)
);

CREATE INDEX idx_group_courses_group ON territory_dk.group_courses(group_id);
CREATE INDEX idx_group_courses_course ON territory_dk.group_courses(course_id);

COMMENT ON TABLE territory_dk.group_courses IS 'Links groups to courses. Populated when LMS extension is added.';
```

**Holochain Mapping:**

```rust
// Links:
// - Group → Course (courses in this group)
// - Course → Group (groups this course belongs to)
```

---

## Groups Access Control Flow

**How Groups Work:**

1. **Platform Admin creates a group** (e.g., "Platform Management")
2. **Creates/assigns access badge** (e.g., "Platform Management Access")
3. **Adds resources to group**:
   - Communities (immediately)
   - Forums (when extension added)
   - Courses (when extension added)
4. **Users gain access** by earning/receiving the badge:
   - Complete a course that awards the badge
   - Manually assigned by authorized user
5. **Access check**: When user tries to participate in group resources:
   - Is this resource in a group?
   - Does user have the required badge?
   - Is badge still valid (not expired/revoked)?

**Important: Groups Control PARTICIPATION, Not VIEWING**

- **Viewing**: ALL users can VIEW all communities/forums/courses regardless of group membership
- **Participation**: Groups control who can ACTIVELY PARTICIPATE (post, comment, enroll, etc.)
- **Purpose**: Users see what's available and are motivated to earn badges to unlock participation

**Community Access Layers:**

1. **Universal View Access** (ALL users):
   - View all communities (territory & guild)
   - Read community descriptions
   - See community posts/events (read-only)
   - View community members

2. **Group-Based Participation** (Badge holders):
   - If community is in a group: Badge required to join and participate
   - If community NOT in a group: Follow community's own join_policy

3. **Community-Level Access** (Individual community settings):
   - Territory communities: Always invite_only (membership)
   - Guild communities: Can be open, badge_required, approval_required, or invite_only
   - Community badge (if set) overrides group badge for that specific community

**Example Scenarios:**

1. **Territory Community (Copenhagen)**:
   - Type: `territory`
   - Join Policy: `invite_only`
   - In Group: No
   - Result: All users can VIEW activity, only invited users can JOIN and participate

2. **Guild Community (Beekeepers)**:
   - Type: `guild`
   - Join Policy: `open`
   - In Group: No
   - Result: All users can VIEW, anyone can JOIN and participate

3. **Guild Community (Advanced Permaculture)**:
   - Type: `guild`
   - Join Policy: `badge_required`
   - Badge: "Permaculture Basics Certificate"
   - In Group: Yes ("Permaculture Network" group)
   - Result: All users can VIEW, only badge holders can JOIN and participate

4. **Platform Management Community**:
   - Type: `guild`
   - Join Policy: `badge_required`
   - Badge: "Platform Admin"
   - In Group: Yes ("Platform Management" group)
   - Group Badge: "Platform Management Access"
   - Result: All users can VIEW, only Platform Management Access badge holders can participate

**Scope Visibility:**

```sql
-- Check if user can access a group
CREATE OR REPLACE FUNCTION territory_dk.user_can_access_group(
    p_user_id UUID,
    p_group_id UUID
) RETURNS BOOLEAN AS $$
DECLARE
    v_access_badge_id UUID;
    v_has_badge BOOLEAN;
    v_scope VARCHAR(50);
    v_scope_id UUID;
BEGIN
    -- Get group details
    SELECT access_badge_id, scope, scope_id
    INTO v_access_badge_id, v_scope, v_scope_id
    FROM territory_dk.groups
    WHERE id = p_group_id AND is_active = TRUE;
    
    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;
    
    -- Check if user has the required badge (and it's not expired/revoked)
    SELECT EXISTS(
        SELECT 1
        FROM territory_dk.badge_awards
        WHERE user_id = p_user_id
          AND badge_id = v_access_badge_id
          AND revoked_at IS NULL
          AND (expires_at IS NULL OR expires_at > NOW())
    ) INTO v_has_badge;
    
    IF NOT v_has_badge THEN
        RETURN FALSE;
    END IF;
    
    -- Check scope visibility
    IF v_scope = 'global' THEN
        -- Global groups accessible to all (with badge)
        RETURN TRUE;
    ELSIF v_scope = 'territory' THEN
        -- Territory groups: user must be in that territory
        -- (This would check user's territory membership)
        RETURN TRUE; -- Simplified for now
    ELSIF v_scope = 'community' THEN
        -- Community groups: user must be in that community or parent communities
        -- (This would check community hierarchy)
        RETURN TRUE; -- Simplified for now
    END IF;
    
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;
```

**Example Workflow: Platform Management**

1. Platform Admin bootstrapped into system
2. Admin creates "Platform Management" group (global scope)
3. Admin creates "Platform Management Access" badge
4. Admin assigns badge to themselves
5. Admin creates "General Platform Management" forum → adds to group
6. Admin creates "Territory Management" course → adds to group
7. Course completion grants "Territory Manager" badge (different badge with different permissions)
8. New user takes "Territory Management" course → earns Territory Manager badge → can now create territories

---

### 22. Notifications Table

**Purpose:** Store notifications for users about system events, badge expirations, elections, invitations, etc.

**Note:** This is the notification queue/history. User preferences are in `users_notification_settings`.

```sql
CREATE TABLE territory_dk.notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- Notification type and content
    notification_type VARCHAR(50) NOT NULL,    -- 'badge_expiring', 'badge_awarded', 'election_vote_request', 'community_invitation', 'role_assigned', 'system_announcement'
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    
    -- Related entities (polymorphic)
    related_entity_type VARCHAR(50),           -- 'badge', 'community', 'election', 'role', 'invitation', 'event'
    related_entity_id UUID,                    -- ID of the related entity
    
    -- Action links
    action_url TEXT,                           -- Deep link to take action
    action_label VARCHAR(100),                 -- "View Badge", "Vote Now", "Accept Invitation"
    
    -- Delivery tracking
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    read_at TIMESTAMPTZ,
    is_delivered BOOLEAN NOT NULL DEFAULT FALSE,
    delivered_at TIMESTAMPTZ,
    delivery_method VARCHAR(50),               -- 'in_app', 'email', 'push'
    
    -- Priority and expiration
    priority VARCHAR(20) NOT NULL DEFAULT 'normal', -- 'low', 'normal', 'high', 'urgent'
    expires_at TIMESTAMPTZ,                    -- Auto-delete after this date
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (notification_type IN ('badge_expiring', 'badge_awarded', 'badge_revoked', 'election_vote_request', 'election_completed', 'community_invitation', 'community_joined', 'role_assigned', 'role_removed', 'system_announcement', 'event_reminder', 'event_invitation')),
    CHECK (priority IN ('low', 'normal', 'high', 'urgent'))
);

CREATE INDEX idx_notifications_user ON territory_dk.notifications(user_id);
CREATE INDEX idx_notifications_unread ON territory_dk.notifications(user_id, is_read) WHERE is_read = FALSE;
CREATE INDEX idx_notifications_type ON territory_dk.notifications(notification_type);
CREATE INDEX idx_notifications_created ON territory_dk.notifications(created_at DESC);
CREATE INDEX idx_notifications_related ON territory_dk.notifications(related_entity_type, related_entity_id);
CREATE INDEX idx_notifications_expires ON territory_dk.notifications(expires_at) WHERE expires_at IS NOT NULL;
```

**Common Notification Types:**

```sql
-- Badge expiration warning (30, 14, 7 days before)
INSERT INTO territory_dk.notifications (user_id, notification_type, title, message, related_entity_type, related_entity_id, priority, action_url, action_label)
VALUES (
    '<user_id>',
    'badge_expiring',
    'Code of Conduct Badge Expiring Soon',
    'Your Code of Conduct badge will expire in 7 days. Please retake the course to maintain platform access.',
    'badge',
    '<badge_id>',
    'high',
    '/courses/code-of-conduct',
    'Renew Now'
);

-- Election vote request
INSERT INTO territory_dk.notifications (user_id, notification_type, title, message, related_entity_type, related_entity_id, priority, action_url, action_label)
VALUES (
    '<user_id>',
    'election_vote_request',
    'Your Vote Needed: Community Manager Election',
    'The Copenhagen community is voting to elect a new Community Manager. Your vote is required (100% participation).',
    'election',
    '<election_id>',
    'urgent',
    '/elections/<election_id>',
    'Vote Now'
);

-- Community invitation
INSERT INTO territory_dk.notifications (user_id, notification_type, title, message, related_entity_type, related_entity_id, action_url, action_label)
VALUES (
    '<user_id>',
    'community_invitation',
    'Invitation to Join Beekeepers Guild',
    'You have been invited to join the Beekeepers Guild community.',
    'community',
    '<community_id>',
    '/communities/<community_id>/join',
    'Accept Invitation'
);
```

**Auto-cleanup Expired Notifications:**

```sql
-- Scheduled job to delete expired notifications
DELETE FROM territory_dk.notifications
WHERE expires_at IS NOT NULL AND expires_at < NOW();
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct Notification {
    pub recipient: AgentPubKey,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub related_entity: Option<ActionHash>,
    pub action_url: Option<String>,
    pub priority: NotificationPriority,
    pub created_at: Timestamp,
}

enum NotificationType {
    BadgeExpiring,
    BadgeAwarded,
    ElectionVoteRequest,
    CommunityInvitation,
    RoleAssigned,
    SystemAnnouncement,
    EventReminder,
}

enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

// Links:
// - User → Notification (recipient's notifications)
// - Related Entity → Notification (notifications about this entity)
```

---

### 23. User Connections Table

**Purpose:** Track social connections between users (following, friends, blocks)

```sql
CREATE TABLE territory_dk.user_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,      -- The user initiating the connection
    target_user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE, -- The user being connected to
    
    -- Connection type
    connection_type VARCHAR(50) NOT NULL,      -- 'follow', 'friend', 'block'
    
    -- Status (for friend requests)
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- 'pending', 'active', 'rejected'
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (connection_type IN ('follow', 'friend', 'block')),
    CHECK (status IN ('pending', 'active', 'rejected')),
    CHECK (user_id != target_user_id),         -- Can't connect to yourself
    
    -- Prevent duplicate connections
    CONSTRAINT uq_user_connection UNIQUE (user_id, target_user_id, connection_type)
);

CREATE INDEX idx_user_connections_user ON territory_dk.user_connections(user_id);
CREATE INDEX idx_user_connections_target ON territory_dk.user_connections(target_user_id);
CREATE INDEX idx_user_connections_type ON territory_dk.user_connections(connection_type);
CREATE INDEX idx_user_connections_pending ON territory_dk.user_connections(target_user_id, status) 
    WHERE connection_type = 'friend' AND status = 'pending';
```

**Connection Types:**

1. **Follow** (one-way):
   - User follows another user to see their activity
   - Status: Always 'active' (no approval needed)
   - Example: Alice follows Bob

2. **Friend** (two-way, requires approval):
   - User sends friend request
   - Status: 'pending' until accepted
   - Creates reciprocal connection when accepted
   - Example: Alice sends friend request to Bob → Bob accepts → both are friends

3. **Block** (one-way):
   - User blocks another user
   - Prevents all interactions
   - Status: Always 'active'
   - Example: Alice blocks Bob → Bob can't see Alice's content or message her

**Example Operations:**

```sql
-- User follows another user
INSERT INTO territory_dk.user_connections (user_id, target_user_id, connection_type, status)
VALUES ('<alice_id>', '<bob_id>', 'follow', 'active');

-- Send friend request
INSERT INTO territory_dk.user_connections (user_id, target_user_id, connection_type, status)
VALUES ('<alice_id>', '<bob_id>', 'friend', 'pending');

-- Accept friend request (creates reciprocal connection)
UPDATE territory_dk.user_connections
SET status = 'active', updated_at = NOW()
WHERE user_id = '<alice_id>' AND target_user_id = '<bob_id>' AND connection_type = 'friend';

INSERT INTO territory_dk.user_connections (user_id, target_user_id, connection_type, status)
VALUES ('<bob_id>', '<alice_id>', 'friend', 'active');

-- Block user
INSERT INTO territory_dk.user_connections (user_id, target_user_id, connection_type, status)
VALUES ('<alice_id>', '<spam_user_id>', 'block', 'active');

-- Check if user is blocked
SELECT EXISTS(
    SELECT 1
    FROM territory_dk.user_connections
    WHERE target_user_id = '<alice_id>'
      AND user_id = '<bob_id>'
      AND connection_type = 'block'
      AND status = 'active'
) AS is_blocked;

-- Get user's followers
SELECT u.id, u.username, up.display_name, up.avatar_url
FROM territory_dk.user_connections uc
JOIN territory_dk.users u ON u.id = uc.user_id
LEFT JOIN territory_dk.users_profiles up ON up.user_id = u.id
WHERE uc.target_user_id = '<user_id>'
  AND uc.connection_type = 'follow'
  AND uc.status = 'active';

-- Get users that user is following
SELECT u.id, u.username, up.display_name, up.avatar_url
FROM territory_dk.user_connections uc
JOIN territory_dk.users u ON u.id = uc.target_user_id
LEFT JOIN territory_dk.users_profiles up ON up.user_id = u.id
WHERE uc.user_id = '<user_id>'
  AND uc.connection_type = 'follow'
  AND uc.status = 'active';

-- Get user's friends (mutual connections)
SELECT u.id, u.username, up.display_name, up.avatar_url
FROM territory_dk.user_connections uc
JOIN territory_dk.users u ON u.id = uc.target_user_id
LEFT JOIN territory_dk.users_profiles up ON up.user_id = u.id
WHERE uc.user_id = '<user_id>'
  AND uc.connection_type = 'friend'
  AND uc.status = 'active';
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct UserConnection {
    pub user: AgentPubKey,
    pub target_user: AgentPubKey,
    pub connection_type: ConnectionType,
    pub status: ConnectionStatus,
    pub created_at: Timestamp,
}

enum ConnectionType {
    Follow,
    Friend,
    Block,
}

enum ConnectionStatus {
    Pending,
    Active,
    Rejected,
}

// Links:
// - User → Target User (following, friends)
// - Target User → User (reverse lookups)
// - Blocks stored as private entries
```

---

### 24. Community Events Table

**Purpose:** Community events/calendar with RSVP tracking

```sql
CREATE TABLE territory_dk.community_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES territory_dk.communities(id) ON DELETE CASCADE,
    
    -- Event details
    title VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Event type
    event_type VARCHAR(50) NOT NULL,           -- 'meeting', 'workshop', 'social', 'learning', 'ceremony', 'other'
    
    -- Scheduling
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) NOT NULL,             -- 'Europe/Copenhagen', 'America/New_York'
    is_all_day BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Recurrence (for recurring events)
    recurrence_rule TEXT,                      -- RRULE format (RFC 5545)
    parent_event_id UUID REFERENCES territory_dk.community_events(id), -- For recurring event instances
    
    -- Location
    location_type VARCHAR(50) NOT NULL,        -- 'physical', 'virtual', 'hybrid'
    location_address TEXT,                     -- Physical address
    location_url TEXT,                         -- Video conference link
    location_coordinates POINT,                -- Lat/Long for mapping
    
    -- Capacity
    max_participants INTEGER,
    requires_rsvp BOOLEAN NOT NULL DEFAULT TRUE,
    rsvp_deadline TIMESTAMPTZ,
    
    -- Visibility
    is_public BOOLEAN NOT NULL DEFAULT TRUE,   -- Public events visible to all
    requires_badge_id UUID REFERENCES territory_dk.badge_definitions(id), -- Badge required to attend
    
    -- Organizer
    created_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'scheduled', -- 'draft', 'scheduled', 'ongoing', 'completed', 'cancelled'
    cancellation_reason TEXT,
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,        -- Tags, custom fields, etc.
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (event_type IN ('meeting', 'workshop', 'social', 'learning', 'ceremony', 'other')),
    CHECK (location_type IN ('physical', 'virtual', 'hybrid')),
    CHECK (status IN ('draft', 'scheduled', 'ongoing', 'completed', 'cancelled')),
    CHECK (ends_at > starts_at)
);

CREATE INDEX idx_community_events_community ON territory_dk.community_events(community_id);
CREATE INDEX idx_community_events_starts ON territory_dk.community_events(starts_at);
CREATE INDEX idx_community_events_status ON territory_dk.community_events(status);
CREATE INDEX idx_community_events_creator ON territory_dk.community_events(created_by_user_id);
CREATE INDEX idx_community_events_upcoming ON territory_dk.community_events(starts_at) 
    WHERE status = 'scheduled' AND starts_at > NOW();
```

**Event RSVPs Table:**

```sql
CREATE TABLE territory_dk.event_rsvps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES territory_dk.community_events(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_dk.users(id) ON DELETE CASCADE,
    
    -- RSVP status
    response VARCHAR(50) NOT NULL,             -- 'going', 'maybe', 'not_going', 'invited'
    
    -- Additional info
    guest_count INTEGER NOT NULL DEFAULT 0,    -- Number of additional guests
    dietary_requirements TEXT,
    notes TEXT,                                -- User notes
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (response IN ('going', 'maybe', 'not_going', 'invited')),
    CHECK (guest_count >= 0),
    
    CONSTRAINT uq_event_rsvp UNIQUE (event_id, user_id)
);

CREATE INDEX idx_event_rsvps_event ON territory_dk.event_rsvps(event_id);
CREATE INDEX idx_event_rsvps_user ON territory_dk.event_rsvps(user_id);
CREATE INDEX idx_event_rsvps_response ON territory_dk.event_rsvps(event_id, response);
CREATE INDEX idx_event_rsvps_going ON territory_dk.event_rsvps(event_id) 
    WHERE response = 'going';
```

**Example Operations:**

```sql
-- Create community event
INSERT INTO territory_dk.community_events (
    community_id, title, description, event_type, starts_at, ends_at, 
    timezone, location_type, location_address, max_participants, created_by_user_id
)
VALUES (
    '<community_id>',
    'Monthly Beekeeping Workshop',
    'Learn about queen rearing and hive management techniques.',
    'workshop',
    '2025-12-15 14:00:00+01',
    '2025-12-15 17:00:00+01',
    'Europe/Copenhagen',
    'physical',
    'Community Garden, Valby, Copenhagen',
    20,
    '<organizer_id>'
);

-- RSVP to event
INSERT INTO territory_dk.event_rsvps (event_id, user_id, response, guest_count)
VALUES ('<event_id>', '<user_id>', 'going', 1)
ON CONFLICT (event_id, user_id) 
DO UPDATE SET response = EXCLUDED.response, guest_count = EXCLUDED.guest_count, updated_at = NOW();

-- Get upcoming events for a community
SELECT e.*, 
       COUNT(r.id) FILTER (WHERE r.response = 'going') AS going_count,
       COUNT(r.id) FILTER (WHERE r.response = 'maybe') AS maybe_count
FROM territory_dk.community_events e
LEFT JOIN territory_dk.event_rsvps r ON r.event_id = e.id
WHERE e.community_id = '<community_id>'
  AND e.status = 'scheduled'
  AND e.starts_at > NOW()
GROUP BY e.id
ORDER BY e.starts_at ASC;

-- Get user's upcoming events
SELECT e.*, c.name AS community_name, r.response
FROM territory_dk.community_events e
JOIN territory_dk.communities c ON c.id = e.community_id
JOIN territory_dk.event_rsvps r ON r.event_id = e.id
WHERE r.user_id = '<user_id>'
  AND e.starts_at > NOW()
  AND e.status = 'scheduled'
ORDER BY e.starts_at ASC;

-- Check event capacity
SELECT 
    e.title,
    e.max_participants,
    COUNT(r.id) FILTER (WHERE r.response = 'going') + COALESCE(SUM(r.guest_count) FILTER (WHERE r.response = 'going'), 0) AS total_attendees,
    e.max_participants - (COUNT(r.id) FILTER (WHERE r.response = 'going') + COALESCE(SUM(r.guest_count) FILTER (WHERE r.response = 'going'), 0)) AS spots_remaining
FROM territory_dk.community_events e
LEFT JOIN territory_dk.event_rsvps r ON r.event_id = e.id
WHERE e.id = '<event_id>'
GROUP BY e.id;
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct CommunityEvent {
    pub community: ActionHash,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub starts_at: Timestamp,
    pub ends_at: Timestamp,
    pub location_type: LocationType,
    pub location_details: LocationDetails,
    pub max_participants: Option<u32>,
    pub created_by: AgentPubKey,
}

#[hdk_entry_helper]
pub struct EventRSVP {
    pub event: ActionHash,
    pub user: AgentPubKey,
    pub response: RSVPResponse,
    pub guest_count: u32,
}

enum EventType {
    Meeting,
    Workshop,
    Social,
    Learning,
    Ceremony,
    Other,
}

enum LocationType {
    Physical,
    Virtual,
    Hybrid,
}

enum RSVPResponse {
    Going,
    Maybe,
    NotGoing,
    Invited,
}

// Links:
// - Community → Event (community's events)
// - Event → RSVP (event attendees)
// - User → RSVP (user's RSVPs)
```

---

### 25. File Uploads Table

**Purpose:** Track file uploads and IPFS metadata (avatars, community logos, attachments)

```sql
CREATE TABLE territory_dk.file_uploads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Owner
    uploaded_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    
    -- File details
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,                 -- Size in bytes
    mime_type VARCHAR(100) NOT NULL,           -- 'image/jpeg', 'application/pdf'
    file_extension VARCHAR(10),                -- 'jpg', 'pdf', 'png'
    
    -- Storage
    storage_type VARCHAR(50) NOT NULL,         -- 'ipfs', 's3', 'local'
    storage_path TEXT NOT NULL,                -- IPFS hash or S3 path or local path
    ipfs_hash VARCHAR(100),                    -- CID for IPFS files
    
    -- Usage tracking
    entity_type VARCHAR(50),                   -- 'user_avatar', 'community_logo', 'event_image', 'post_attachment'
    entity_id UUID,                            -- ID of the related entity
    
    -- Image metadata (if applicable)
    image_width INTEGER,
    image_height INTEGER,
    has_thumbnail BOOLEAN DEFAULT FALSE,
    thumbnail_path TEXT,
    
    -- Status
    upload_status VARCHAR(50) NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed'
    virus_scan_status VARCHAR(50),             -- 'pending', 'clean', 'infected'
    
    -- Access control
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_accessed_at TIMESTAMPTZ,
    
    CHECK (storage_type IN ('ipfs', 's3', 'local')),
    CHECK (upload_status IN ('pending', 'processing', 'completed', 'failed')),
    CHECK (virus_scan_status IN ('pending', 'clean', 'infected', 'skipped'))
);

CREATE INDEX idx_file_uploads_user ON territory_dk.file_uploads(uploaded_by_user_id);
CREATE INDEX idx_file_uploads_entity ON territory_dk.file_uploads(entity_type, entity_id);
CREATE INDEX idx_file_uploads_ipfs ON territory_dk.file_uploads(ipfs_hash) WHERE ipfs_hash IS NOT NULL;
CREATE INDEX idx_file_uploads_status ON territory_dk.file_uploads(upload_status);
CREATE INDEX idx_file_uploads_created ON territory_dk.file_uploads(created_at DESC);
```

**Example Operations:**

```sql
-- Upload avatar to IPFS
INSERT INTO territory_dk.file_uploads (
    uploaded_by_user_id, file_name, file_size, mime_type, file_extension,
    storage_type, storage_path, ipfs_hash, entity_type, entity_id,
    image_width, image_height, upload_status
)
VALUES (
    '<user_id>',
    'avatar.jpg',
    153600,
    'image/jpeg',
    'jpg',
    'ipfs',
    'https://ipfs.io/ipfs/QmXyz123...',
    'QmXyz123...',
    'user_avatar',
    '<user_id>',
    512,
    512,
    'completed'
);

-- Update user's avatar URL
UPDATE territory_dk.users_profiles
SET avatar_url = (SELECT storage_path FROM territory_dk.file_uploads WHERE id = '<file_id>')
WHERE user_id = '<user_id>';

-- Upload community logo
INSERT INTO territory_dk.file_uploads (
    uploaded_by_user_id, file_name, file_size, mime_type, storage_type, 
    storage_path, ipfs_hash, entity_type, entity_id, upload_status
)
VALUES (
    '<user_id>',
    'beekeepers-logo.png',
    89432,
    'image/png',
    'ipfs',
    'https://ipfs.io/ipfs/QmAbc456...',
    'QmAbc456...',
    'community_logo',
    '<community_id>',
    'completed'
);

-- Get user's recent uploads
SELECT id, file_name, file_size, mime_type, storage_path, created_at
FROM territory_dk.file_uploads
WHERE uploaded_by_user_id = '<user_id>'
ORDER BY created_at DESC
LIMIT 20;

-- Clean up old unused files (scheduled job)
DELETE FROM territory_dk.file_uploads
WHERE created_at < NOW() - INTERVAL '90 days'
  AND entity_type IS NULL
  AND upload_status = 'completed';
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct FileUpload {
    pub uploaded_by: AgentPubKey,
    pub file_name: String,
    pub file_size: u64,
    pub mime_type: String,
    pub ipfs_hash: String,
    pub entity_type: Option<EntityType>,
    pub entity_id: Option<ActionHash>,
    pub is_public: bool,
    pub created_at: Timestamp,
}

enum EntityType {
    UserAvatar,
    CommunityLogo,
    EventImage,
    PostAttachment,
}

// Links:
// - User → FileUpload (user's uploads)
// - Entity → FileUpload (files for this entity)
// - IPFS hash stored for DHT retrieval
```

---

### 26. Content Reports Table

**Purpose:** User-generated reports for inappropriate content or behavior

```sql
CREATE TABLE territory_dk.content_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Reporter
    reported_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    
    -- Reported content/user
    report_type VARCHAR(50) NOT NULL,          -- 'user', 'community', 'event', 'profile', 'comment' (future)
    reported_entity_type VARCHAR(50) NOT NULL, -- 'user', 'community', 'event', 'post', 'comment'
    reported_entity_id UUID NOT NULL,
    reported_user_id UUID REFERENCES territory_dk.users(id), -- If reporting a user or user's content
    
    -- Report details
    reason VARCHAR(100) NOT NULL,              -- 'spam', 'harassment', 'inappropriate_content', 'misinformation', 'other'
    description TEXT NOT NULL,
    severity VARCHAR(50) NOT NULL DEFAULT 'normal', -- 'low', 'normal', 'high', 'critical'
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- 'pending', 'under_review', 'resolved', 'dismissed'
    resolution TEXT,
    resolved_by_user_id UUID REFERENCES territory_dk.users(id),
    resolved_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (report_type IN ('user', 'community', 'event', 'profile', 'comment', 'post')),
    CHECK (reported_entity_type IN ('user', 'community', 'event', 'post', 'comment', 'profile')),
    CHECK (reason IN ('spam', 'harassment', 'inappropriate_content', 'hate_speech', 'misinformation', 'violence', 'other')),
    CHECK (severity IN ('low', 'normal', 'high', 'critical')),
    CHECK (status IN ('pending', 'under_review', 'resolved', 'dismissed'))
);

CREATE INDEX idx_content_reports_reporter ON territory_dk.content_reports(reported_by_user_id);
CREATE INDEX idx_content_reports_entity ON territory_dk.content_reports(reported_entity_type, reported_entity_id);
CREATE INDEX idx_content_reports_user ON territory_dk.content_reports(reported_user_id);
CREATE INDEX idx_content_reports_status ON territory_dk.content_reports(status);
CREATE INDEX idx_content_reports_pending ON territory_dk.content_reports(status, created_at) 
    WHERE status = 'pending';
```

**Moderation Actions Table:**

```sql
CREATE TABLE territory_dk.moderation_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Related report (optional - can take action without report)
    content_report_id UUID REFERENCES territory_dk.content_reports(id),
    
    -- Moderator
    moderator_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    moderator_role_id UUID REFERENCES territory_dk.roles(id),
    
    -- Action target
    target_user_id UUID REFERENCES territory_dk.users(id),
    target_entity_type VARCHAR(50),            -- 'user', 'community', 'event', 'post', 'comment'
    target_entity_id UUID,
    
    -- Action details
    action_type VARCHAR(50) NOT NULL,          -- 'warning', 'content_removal', 'temporary_restriction', 'permanent_ban', 'badge_revocation'
    action_reason TEXT NOT NULL,
    action_duration INTERVAL,                  -- For temporary restrictions
    
    -- Warning tracking (3-strike system)
    is_strike BOOLEAN NOT NULL DEFAULT FALSE,
    strike_number INTEGER,                     -- 1, 2, 3
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,                    -- For temporary actions
    
    CHECK (action_type IN ('warning', 'content_removal', 'temporary_restriction', 'permanent_ban', 'badge_revocation', 'account_suspension'))
);

CREATE INDEX idx_moderation_actions_moderator ON territory_dk.moderation_actions(moderator_user_id);
CREATE INDEX idx_moderation_actions_target_user ON territory_dk.moderation_actions(target_user_id);
CREATE INDEX idx_moderation_actions_report ON territory_dk.moderation_actions(content_report_id);
CREATE INDEX idx_moderation_actions_strikes ON territory_dk.moderation_actions(target_user_id, is_strike) 
    WHERE is_strike = TRUE;
CREATE INDEX idx_moderation_actions_active ON territory_dk.moderation_actions(target_user_id, expires_at)
    WHERE expires_at IS NOT NULL AND expires_at > NOW();
```

**Example Operations:**

```sql
-- Report inappropriate profile
INSERT INTO territory_dk.content_reports (
    reported_by_user_id, report_type, reported_entity_type, reported_entity_id,
    reported_user_id, reason, description, severity
)
VALUES (
    '<reporter_id>',
    'user',
    'profile',
    '<profile_user_id>',
    '<profile_user_id>',
    'inappropriate_content',
    'Profile contains offensive images and hate speech in bio.',
    'high'
);

-- Moderator reviews and issues warning (strike 1)
INSERT INTO territory_dk.moderation_actions (
    content_report_id, moderator_user_id, target_user_id, action_type,
    action_reason, is_strike, strike_number
)
VALUES (
    '<report_id>',
    '<moderator_id>',
    '<offending_user_id>',
    'warning',
    'Inappropriate profile content removed. This is your first warning.',
    TRUE,
    1
);

-- Update report status
UPDATE territory_dk.content_reports
SET status = 'resolved',
    resolution = 'User warned and content removed.',
    resolved_by_user_id = '<moderator_id>',
    resolved_at = NOW()
WHERE id = '<report_id>';

-- Check user's strike count
SELECT COUNT(*) AS strike_count
FROM territory_dk.moderation_actions
WHERE target_user_id = '<user_id>'
  AND is_strike = TRUE;

-- Get pending reports for moderation queue
SELECT r.*, 
       reporter.username AS reporter_username,
       reported.username AS reported_username
FROM territory_dk.content_reports r
LEFT JOIN territory_dk.users reporter ON reporter.id = r.reported_by_user_id
LEFT JOIN territory_dk.users reported ON reported.id = r.reported_user_id
WHERE r.status = 'pending'
ORDER BY r.severity DESC, r.created_at ASC;
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct ContentReport {
    pub reported_by: AgentPubKey,
    pub report_type: ReportType,
    pub reported_entity: ActionHash,
    pub reported_user: Option<AgentPubKey>,
    pub reason: ReportReason,
    pub description: String,
    pub severity: ReportSeverity,
    pub created_at: Timestamp,
}

#[hdk_entry_helper]
pub struct ModerationAction {
    pub moderator: AgentPubKey,
    pub target_user: AgentPubKey,
    pub action_type: ActionType,
    pub reason: String,
    pub is_strike: bool,
    pub strike_number: Option<u8>,
    pub created_at: Timestamp,
}

// Reports and moderation actions stored as signed entries
// Links:
// - Reported Entity → Report
// - Target User → ModerationAction (user's moderation history)
```

---

### 27. Activity Feed Table

**Purpose:** Track user and community activities for timeline/feed display

```sql
CREATE TABLE territory_dk.activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Actor (who did the action)
    actor_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    
    -- Activity type
    activity_type VARCHAR(50) NOT NULL,        -- 'user_joined', 'badge_earned', 'community_joined', 'event_created', 'role_assigned'
    
    -- Target/Object (what was acted upon)
    object_type VARCHAR(50),                   -- 'user', 'badge', 'community', 'event', 'role'
    object_id UUID,
    object_name VARCHAR(255),                  -- Denormalized for performance
    
    -- Context
    community_id UUID REFERENCES territory_dk.communities(id), -- If activity is within a community
    
    -- Visibility
    visibility VARCHAR(50) NOT NULL DEFAULT 'public', -- 'public', 'community', 'friends', 'private'
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,        -- Additional context, rich content
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (activity_type IN ('user_joined', 'profile_updated', 'badge_earned', 'badge_renewed', 'community_joined', 'community_created', 'event_created', 'event_rsvp', 'role_assigned', 'election_completed', 'connection_made')),
    CHECK (visibility IN ('public', 'community', 'friends', 'private'))
);

CREATE INDEX idx_activities_actor ON territory_dk.activities(actor_user_id);
CREATE INDEX idx_activities_community ON territory_dk.activities(community_id);
CREATE INDEX idx_activities_object ON territory_dk.activities(object_type, object_id);
CREATE INDEX idx_activities_created ON territory_dk.activities(created_at DESC);
CREATE INDEX idx_activities_public ON territory_dk.activities(created_at DESC) WHERE visibility = 'public';
CREATE INDEX idx_activities_community_feed ON territory_dk.activities(community_id, created_at DESC)
    WHERE community_id IS NOT NULL;
```

**Example Activities:**

```sql
-- User joined platform
INSERT INTO territory_dk.activities (actor_user_id, activity_type, visibility)
VALUES ('<user_id>', 'user_joined', 'public');

-- User earned badge
INSERT INTO territory_dk.activities (
    actor_user_id, activity_type, object_type, object_id, object_name, visibility, metadata
)
VALUES (
    '<user_id>',
    'badge_earned',
    'badge',
    '<badge_id>',
    'Code of Conduct',
    'public',
    '{"badge_category": "governance"}'::jsonb
);

-- User joined community
INSERT INTO territory_dk.activities (
    actor_user_id, activity_type, object_type, object_id, object_name, 
    community_id, visibility
)
VALUES (
    '<user_id>',
    'community_joined',
    'community',
    '<community_id>',
    'Beekeepers Guild',
    '<community_id>',
    'community'
);

-- User created event
INSERT INTO territory_dk.activities (
    actor_user_id, activity_type, object_type, object_id, object_name,
    community_id, visibility, metadata
)
VALUES (
    '<user_id>',
    'event_created',
    'event',
    '<event_id>',
    'Monthly Beekeeping Workshop',
    '<community_id>',
    'public',
    '{"event_date": "2025-12-15T14:00:00Z"}'::jsonb
);

-- Get user's activity feed
SELECT a.*,
       u.username AS actor_username,
       up.display_name AS actor_display_name,
       up.avatar_url AS actor_avatar
FROM territory_dk.activities a
JOIN territory_dk.users u ON u.id = a.actor_user_id
LEFT JOIN territory_dk.users_profiles up ON up.user_id = a.actor_user_id
WHERE a.actor_user_id = '<user_id>'
ORDER BY a.created_at DESC
LIMIT 50;

-- Get community activity feed
SELECT a.*,
       u.username AS actor_username,
       up.display_name AS actor_display_name
FROM territory_dk.activities a
JOIN territory_dk.users u ON u.id = a.actor_user_id
LEFT JOIN territory_dk.users_profiles up ON up.user_id = a.actor_user_id
WHERE a.community_id = '<community_id>'
  AND a.visibility IN ('public', 'community')
ORDER BY a.created_at DESC
LIMIT 50;

-- Get global public activity feed
SELECT a.*,
       u.username AS actor_username,
       up.display_name AS actor_display_name
FROM territory_dk.activities a
JOIN territory_dk.users u ON u.id = a.actor_user_id
LEFT JOIN territory_dk.users_profiles up ON up.user_id = a.actor_user_id
WHERE a.visibility = 'public'
ORDER BY a.created_at DESC
LIMIT 100;
```

**Holochain Mapping:**

```rust
#[hdk_entry_helper]
pub struct Activity {
    pub actor: AgentPubKey,
    pub activity_type: ActivityType,
    pub object_type: Option<ObjectType>,
    pub object_hash: Option<ActionHash>,
    pub community: Option<ActionHash>,
    pub visibility: Visibility,
    pub metadata: HashMap<String, String>,
    pub created_at: Timestamp,
}

enum ActivityType {
    UserJoined,
    ProfileUpdated,
    BadgeEarned,
    CommunityJoined,
    EventCreated,
    RoleAssigned,
}

enum Visibility {
    Public,
    Community,
    Friends,
    Private,
}

// Links:
// - Actor → Activity (user's activities)
// - Community → Activity (community feed)
// - Public activities indexed for global feed
```

---

## User Initialization Workflow

### What Happens When a New User is Created?

When a new user registers on the platform, the system performs a **multi-table initialization** to ensure all necessary user-related tables have proper default entries. This can be implemented using **database triggers** (automatic, guaranteed consistency) or **application logic** (flexible, easier to test).

#### Recommended Approach: Database Triggers

Database triggers ensure atomic initialization - if any part fails, the entire user creation rolls back. This prevents orphaned user records without corresponding profiles/settings.

**Tables Automatically Created:**

1. **`users`** - Created by registration endpoint (primary table)
2. **`users_profiles`** - Auto-created by trigger with default values
3. **`users_settings`** - Auto-created by trigger with default preferences
4. **`users_notification_settings`** - Auto-created by trigger with default notification preferences

**Tables Created On-Demand (not required for registration):**

- **`users_profile_links`** - Added when user adds external links
- **`users_language_proficiency`** - Added when user declares language skills
- **`users_audit_logs`** - Created as user performs auditable actions

#### Initialization Sequence

```sql
-- Step 1: Registration endpoint creates user (application logic)
-- This happens in territory_dk schema (or whichever territory user registers in)
INSERT INTO territory_dk.users (
    username,
    matrix_id,
    email, -- optional
    password_hash,
    territory_code
) VALUES (
    'john_doe',
    '@john_doe:unityplan.dk',
    'john@example.com', -- optional
    '$argon2id$v=19$m=...',
    'dk'
);

-- Step 2: Trigger automatically creates profile with defaults
-- (Trigger fires on users INSERT)
INSERT INTO territory_dk.users_profiles (
    user_id,
    display_name,
    bio,
    avatar_url,
    header_image_url,
    location,
    website,
    skills,
    interests,
    is_public
) VALUES (
    <new_user_id>,
    'john_doe', -- defaults to username
    '',         -- empty bio
    NULL,       -- no avatar yet
    NULL,       -- no header image yet
    NULL,       -- no location yet
    '',         -- empty website
    ARRAY[]::TEXT[], -- empty skills array
    ARRAY[]::TEXT[], -- empty interests array
    TRUE        -- public by default
);

-- Step 3: Trigger automatically creates settings with defaults
INSERT INTO territory_dk.users_settings (
    user_id,
    preferred_language,
    theme_mode,
    color_scheme,
    auto_translate,
    translation_provider,
    contribute_translations,
    data_collection_consent
) VALUES (
    <new_user_id>,
    'en',              -- English default
    'system',          -- Follow system theme
    'default',         -- Default color scheme
    FALSE,             -- No auto-translate
    'libre-translate', -- Privacy-first provider
    TRUE,              -- Contribute to translation memory
    FALSE              -- Explicit consent required
);

-- Step 4: Trigger automatically creates notification settings
INSERT INTO territory_dk.users_notification_settings (
    user_id,
    email_notifications,
    push_notifications,
    matrix_notifications,
    notification_frequency,
    notify_mentions,
    notify_replies,
    notify_follows,
    notify_forum_activity,
    notify_course_updates,
    notify_badge_awards,
    notify_system_announcements
) VALUES (
    <new_user_id>,
    FALSE,      -- Email off by default (privacy-first)
    TRUE,       -- Push on (in-app)
    TRUE,       -- Matrix on (primary communication)
    'realtime', -- Immediate notifications
    TRUE,       -- Notify on mentions
    TRUE,       -- Notify on replies
    TRUE,       -- Notify on follows
    FALSE,      -- Forum activity off by default
    TRUE,       -- Course updates on
    TRUE,       -- Badge awards on
    TRUE        -- System announcements on
);

-- Step 5: Register username globally (application logic)
-- This happens in global schema to prevent duplicates across territories
INSERT INTO global.username_registry (
    username,
    territory_code,
    user_id
) VALUES (
    'john_doe',
    'dk',
    <new_user_id>
);

-- Step 6: Register email globally if provided (application logic)
-- Only if user provided email during registration
INSERT INTO global.email_registry (
    email,
    territory_code,
    user_id
) VALUES (
    'john@example.com',
    'dk',
    <new_user_id>
);

-- Step 7: Record invitation use (if registration via invitation)
INSERT INTO territory_dk.invitation_uses (
    invitation_token_id,
    user_id,
    used_at
) VALUES (
    <invitation_token_id>,
    <new_user_id>,
    NOW()
);
```

#### SQL Trigger Implementation

```sql
-- Trigger function to auto-create profile
CREATE OR REPLACE FUNCTION territory_dk.create_user_profile()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO territory_dk.users_profiles (
        user_id,
        display_name,
        bio,
        avatar_url,
        header_image_url,
        location,
        website,
        skills,
        interests,
        is_public
    ) VALUES (
        NEW.id,
        NEW.username, -- Default display_name to username
        '',
        NULL,
        NULL,
        NULL,
        '',
        ARRAY[]::TEXT[],
        ARRAY[]::TEXT[],
        TRUE
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach trigger to users table
CREATE TRIGGER trigger_create_user_profile
    AFTER INSERT ON territory_dk.users
    FOR EACH ROW
    EXECUTE FUNCTION territory_dk.create_user_profile();

-- Trigger function to auto-create settings
CREATE OR REPLACE FUNCTION territory_dk.create_user_settings()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO territory_dk.users_settings (
        user_id,
        preferred_language,
        theme_mode,
        color_scheme,
        auto_translate,
        translation_provider,
        contribute_translations,
        data_collection_consent
    ) VALUES (
        NEW.id,
        'en',
        'system',
        'default',
        FALSE,
        'libre-translate',
        TRUE,
        FALSE
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach trigger to users table
CREATE TRIGGER trigger_create_user_settings
    AFTER INSERT ON territory_dk.users
    FOR EACH ROW
    EXECUTE FUNCTION territory_dk.create_user_settings();

-- Trigger function to auto-create notification settings
CREATE OR REPLACE FUNCTION territory_dk.create_user_notification_settings()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO territory_dk.users_notification_settings (
        user_id,
        email_notifications,
        push_notifications,
        matrix_notifications,
        notification_frequency,
        notify_mentions,
        notify_replies,
        notify_follows,
        notify_forum_activity,
        notify_course_updates,
        notify_badge_awards,
        notify_system_announcements
    ) VALUES (
        NEW.id,
        FALSE,
        TRUE,
        TRUE,
        'realtime',
        TRUE,
        TRUE,
        TRUE,
        FALSE,
        TRUE,
        TRUE,
        TRUE
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach trigger to users table
CREATE TRIGGER trigger_create_user_notification_settings
    AFTER INSERT ON territory_dk.users
    FOR EACH ROW
    EXECUTE FUNCTION territory_dk.create_user_notification_settings();
```

#### Application Logic Alternative

If you prefer **application-level initialization** (instead of database triggers):

**Advantages:**

- Easier to test and mock
- More flexible (can conditionally skip tables)
- Clearer error handling in API responses
- No trigger maintenance across multiple territory schemas

**Implementation:**

```rust
// Pseudo-Rust code for registration endpoint
async fn register_user(
    territory: &str,
    registration: UserRegistration
) -> Result<UserCreated, RegistrationError> {
    // Start database transaction
    let mut tx = db.begin().await?;
    
    // 1. Create user
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO territory_{territory}.users 
        (username, matrix_id, email, password_hash, territory_code)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        registration.username,
        format!("@{}:unityplan.{}", registration.username, territory),
        registration.email,
        password_hash,
        territory
    )
    .fetch_one(&mut *tx)
    .await?;
    
    // 2. Create profile with defaults
    sqlx::query!(
        r#"
        INSERT INTO territory_{territory}.users_profiles
        (user_id, display_name, bio, website, skills, interests, is_public)
        VALUES ($1, $2, '', '', ARRAY[]::TEXT[], ARRAY[]::TEXT[], TRUE)
        "#,
        user.id,
        registration.username
    )
    .execute(&mut *tx)
    .await?;
    
    // 3. Create settings with defaults
    sqlx::query!(
        r#"
        INSERT INTO territory_{territory}.users_settings
        (user_id, preferred_language, theme_mode, color_scheme, 
         auto_translate, translation_provider, contribute_translations, 
         data_collection_consent)
        VALUES ($1, 'en', 'system', 'default', FALSE, 'libre-translate', TRUE, FALSE)
        "#,
        user.id
    )
    .execute(&mut *tx)
    .await?;
    
    // 4. Create notification settings with defaults
    sqlx::query!(
        r#"
        INSERT INTO territory_{territory}.users_notification_settings
        (user_id, email_notifications, push_notifications, matrix_notifications,
         notification_frequency, notify_mentions, notify_replies, notify_follows,
         notify_forum_activity, notify_course_updates, notify_badge_awards,
         notify_system_announcements)
        VALUES ($1, FALSE, TRUE, TRUE, 'realtime', TRUE, TRUE, TRUE, FALSE, TRUE, TRUE, TRUE)
        "#,
        user.id
    )
    .execute(&mut *tx)
    .await?;
    
    // 5. Register username globally
    sqlx::query!(
        r#"
        INSERT INTO global.username_registry (username, territory_code, user_id)
        VALUES ($1, $2, $3)
        "#,
        registration.username,
        territory,
        user.id
    )
    .execute(&mut *tx)
    .await?;
    
    // 6. Register email globally (if provided)
    if let Some(email) = registration.email {
        sqlx::query!(
            r#"
            INSERT INTO global.email_registry (email, territory_code, user_id)
            VALUES ($1, $2, $3)
            "#,
            email,
            territory,
            user.id
        )
        .execute(&mut *tx)
        .await?;
    }
    
    // 7. Record invitation use (if via invitation)
    if let Some(invitation_token_id) = registration.invitation_token_id {
        sqlx::query!(
            r#"
            INSERT INTO territory_{territory}.invitation_uses
            (invitation_token_id, user_id, used_at)
            VALUES ($1, $2, NOW())
            "#,
            invitation_token_id,
            user.id
        )
        .execute(&mut *tx)
        .await?;
    }
    
    // Commit transaction
    tx.commit().await?;
    
    Ok(UserCreated {
        user_id: user.id,
        username: user.username,
        matrix_id: user.matrix_id,
    })
}
```

#### Error Handling

**Transaction Rollback Scenarios:**

- Username already exists (global.username_registry unique constraint)
- Email already registered (global.email_registry unique constraint)
- Invalid invitation token
- Invitation token already used or expired
- Database constraint violation

**Error Response:**

```json
{
  "error": "registration_failed",
  "message": "Username already exists",
  "field": "username"
}
```

#### Summary

**Required Tables on User Creation:**

1. ✅ `users` - Always created (primary table)
2. ✅ `users_profiles` - Always created (via trigger or app logic)
3. ✅ `users_settings` - Always created (via trigger or app logic)
4. ✅ `users_notification_settings` - Always created (via trigger or app logic)
5. ✅ `username_registry` (global) - Always created
6. ⚠️ `email_registry` (global) - Only if email provided
7. ⚠️ `invitation_uses` - Only if registration via invitation

**Optional Tables (Created Later):**

- `users_profile_links` - When user adds external links
- `users_language_proficiency` - When user declares language skills
- `users_audit_logs` - As user performs auditable actions

**Recommendation:** Use **database triggers** for guaranteed atomic initialization of core user tables (profiles, settings, notification_settings). Use **application logic** for global registries and conditional tables (email_registry, invitation_uses).

---

## Holochain Entry Type Mapping

### Entry Types Summary

| PostgreSQL Table | Holochain Entry Type | Visibility | Links |
|------------------|---------------------|------------|-------|
| `users` | Agent (built-in) | Private | → Profile |
| `users_profiles` | `Profile` | Public/Private | Agent → Profile, Profile → ProfileLink |
| `users_profile_links` | `ProfileLink` | Public | Profile → ProfileLink |
| `users_language_proficiency` | `LanguageProficiency` | Public | Agent → LanguageProficiency |
| `users_settings` | Local storage | Private | N/A (not on DHT) |
| `users_notification_settings` | Local storage | Private | N/A |
| `invitation_tokens` | `InvitationToken` | Private | Creator → Invitation, Invitation → Redeemer |
| `invitation_uses` | Source chain entry | Immutable | Invitation → User |
| `communities` | `Community` | Public | Creator → Community, Parent → Child |
| `community_members` | Link | Public | Community → Member, Member → Community |
| `roles` | `Role` | Public | N/A (global definition) |
| `role_assignments` | Link + Entry | Public/Private | User → Role, Community → RoleHolder |
| `community_role_elections` | `Election` | Public | Community → Election, Election → Nominee |
| `community_role_election_votes` | `Vote` | Private (signed) | Election → Vote (encrypted) |
| `badge_definitions` | `BadgeDefinition` | Public | N/A (template) |
| `badge_awards` | `BadgeAward` | Public | User → Badge, Badge → User, Course → Badge |
| `badge_progress` | `BadgeProgress` | Private | User → Progress, Badge → Progress |
| `groups` | `Group` | Public | Badge → Group, Territory → Group, Community → Group |
| `group_communities` | Link | Public | Group → Community, Community → Group |
| `group_forums` | Link | Public | Group → Forum, Forum → Group (future) |
| `group_courses` | Link | Public | Group → Course, Course → Group (future) |
| `notifications` | `Notification` | Private | User → Notification, Entity → Notification |
| `user_connections` | `Connection` | Public/Private | User → Target (follow/friend), blocks private |
| `community_events` | `Event` | Public | Community → Event, Creator → Event |
| `event_rsvps` | `RSVP` | Public | Event → RSVP, User → RSVP |
| `file_uploads` | `FileMetadata` | Public/Private | User → File, Entity → File, IPFS hash |
| `content_reports` | `Report` | Private (mods) | Reporter → Report, Entity → Report |
| `moderation_actions` | `ModerationAction` | Private (mods) | Moderator → Action, User → Action |
| `activities` | `Activity` | Public/Private | Actor → Activity, Community → Activity |

**Notes:**

- Community role election votes are **private entries** but cryptographically signed to prove authenticity. The election tally is public, but individual votes remain private to preserve democratic integrity.
- Badge awards are **public** to enable verification of permissions and credentials.
- Badge progress is **private** to protect user learning journey privacy.

### Validation Functions

Each entry type needs validation:

```rust
// Example: Profile validation
pub fn validate_create_profile(
    _action: EntryCreationAction,
    profile: Profile,
) -> ExternResult<ValidateCallbackResult> {
    // Validate agent owns this profile
    if profile.agent != action.author() {
        return Ok(ValidateCallbackResult::Invalid(
            "Profile must be created by the agent it represents".to_string()
        ));
    }
    
    // Validate display_name length
    if let Some(name) = &profile.display_name {
        if name.len() > 100 {
            return Ok(ValidateCallbackResult::Invalid(
                "Display name too long".to_string()
            ));
        }
    }
    
    Ok(ValidateCallbackResult::Valid)
}
```

---

## Migration Strategy

### Database Migration: Single vs Multi-Territory Pods

**Migration Scripts Design:**

All migration scripts should accept `schema_name` as a parameter to support both deployment types:

```bash
# Single-territory pod deployment
./migrate.sh --schema territory_dk

# Multi-territory pod deployment
./migrate.sh --schema territory_dk --territory-code dk
./migrate.sh --schema territory_no --territory-code no
./migrate.sh --schema territory_se --territory-code se
```

**Migration Script Structure:**

```sql
-- Migration template (parameterized)
-- Usage: psql -v schema_name=territory_dk -v territory_code=dk -f migration.sql

-- Create schema
CREATE SCHEMA IF NOT EXISTS :schema_name;

-- Create tables
CREATE TABLE :schema_name.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE,  -- OPTIONAL
    territory_code VARCHAR(10) NOT NULL DEFAULT :'territory_code',
    -- ... rest of columns
);

-- Create indexes
CREATE INDEX idx_users_username ON :schema_name.users(username);
-- ... rest of indexes
```

**Pod Configuration:**

```yaml
# Single-territory pod (config.yml)
database:
  mode: single_territory
  schema_name: territory_dk
  territory_code: dk

# Multi-territory pod (config.yml)
database:
  mode: multi_territory
  territories:
    - schema_name: territory_dk
      territory_code: dk
    - schema_name: territory_no
      territory_code: no
    - schema_name: territory_se
      territory_code: se
```

### Holochain Migration Phases

### Phase 1: PostgreSQL Foundation (Current)

- Implement full PostgreSQL schema with parameterized scripts
- Support both single and multi-territory deployments
- Build REST APIs
- Test with frontend
- Optimize queries and indexes

### Phase 2: Dual-Write

- Set up Holochain DNAs (one per territory)
- Implement write replication: PostgreSQL → Holochain
- Validate data consistency
- Monitor performance

### Phase 3: Dual-Read

- Read from Holochain with PostgreSQL fallback
- Gradual traffic shift: 10% → 50% → 90%
- Compare results, tune validation logic

### Phase 4: Holochain Primary

- Read from Holochain by default
- PostgreSQL as archive/backup only
- Migrate historical data to DHT

### Phase 5: Full Holochain

- Sunset PostgreSQL write path
- Use PostgreSQL for analytics only
- Celebrate decentralization! 🎉

---

## Next Steps

1. **Implement SQL Scripts:**
   - Create migration files for all schemas
   - Add seed data for testing
   - Write rollback scripts

2. **Build Backend APIs:**
   - Implement CRUD operations
   - Add authentication middleware
   - Test with frontend

3. **Holochain Prototyping:**
   - Create proof-of-concept DNA
   - Test entry types and validation
   - Benchmark DHT performance

4. **Documentation:**
   - API documentation (OpenAPI/Swagger)
   - Database ER diagrams
   - Holochain architecture docs

---

**Related Documents:**

- [Backend API Requirements](backend-api-requirements.md)
- [Invitation System](invitation-system.md)
- [Multi-Pod Architecture](multi-pod-architecture.md)
- [User Data Sovereignty](user-data-sovereignty.md)
