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
    email VARCHAR(255),                        -- NULL for group, specific for single_use
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
        (token_type = 'single_use' AND email IS NOT NULL AND max_uses = 1) OR
        (token_type = 'group' AND email IS NULL AND max_uses > 1)
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
| `profiles` | `Profile` | Public/Private | Agent → Profile, Profile → ProfileLink |
| `profile_links` | `ProfileLink` | Public | Profile → ProfileLink |
| `privacy_settings` | `PrivacySettings` | Private | Agent → PrivacySettings |
| `user_settings` | Local storage | Private | N/A (not on DHT) |
| `notification_settings` | Local storage | Private | N/A |
| `invitation_tokens` | `InvitationToken` | Private | Creator → Invitation, Invitation → Redeemer |
| `invitation_uses` | Source chain entry | Immutable | Invitation → User |

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
