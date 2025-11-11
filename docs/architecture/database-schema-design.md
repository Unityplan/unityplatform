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

### 4. Privacy Settings Table

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
    
    -- Locale
    language VARCHAR(10) NOT NULL DEFAULT 'en',
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

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
