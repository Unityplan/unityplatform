# Identity System Architecture

## Overview

UnityPlan implements a **privacy-first, sovereignty-focused identity system** that supports:

- Optional email (no forced external system identification)
- Globally unique usernames across all pods/territories
- Seamless territory migration without username changes
- Federation-compatible identifiers (Matrix protocol)
- Future migration path to Holochain cryptographic identity

## Core Principles

### 1. User Sovereignty

- Email is **optional** - users don't need external system accounts
- Username is chosen by user and globally unique
- Identity can migrate between territories without breaking connections
- Future: Cryptographic identity owned by user (Holochain)

### 2. Privacy-First Design

- No forced email requirement
- Invitation system works with or without email (QR codes, links)
- Personal data stored only in territory schemas (not global)
- Global identity contains only minimal identification data

### 3. Federation-Ready

- Username@territory format compatible with federated systems
- Matrix protocol integration: `@username:unityplan.{territory}`
- Territory migration preserves identity with alias support

## Database Schema

### Global Identity (Minimal, Cross-Territory)

```sql
CREATE TABLE global.user_identities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,  -- GLOBALLY UNIQUE across all pods
    public_key_hash VARCHAR(64) UNIQUE NOT NULL,  -- Placeholder for future crypto key
    territory_code VARCHAR(100) NOT NULL REFERENCES global.territories(code),
    territory_user_id UUID NOT NULL,  -- Points to current territory user record
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (territory_code, territory_user_id)
);
```

### Territory User Data (Personal, Territory-Specific)

```sql
CREATE TABLE territory_{code}.users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,  -- Matches global username
    email VARCHAR(255) UNIQUE NULL,  -- OPTIONAL - for notifications only
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    display_name VARCHAR(100),
    -- ... other personal data
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

## Identity Components

### 1. Global UUID (Permanent Identity)

- **Purpose**: Permanent, immutable identifier across system
- **Format**: UUID v4
- **Example**: `550e8400-e29b-41d4-a716-446655440000`
- **Never changes**: Remains same even if user moves territories
- **Used for**: Internal references, foreign keys, audit trails

### 2. Username (Human-Readable Identity)

- **Purpose**: Human-friendly, globally unique identifier
- **Format**: Lowercase alphanumeric, 3-50 characters
- **Example**: `alice`, `bob_jones`, `charlie123`
- **Global uniqueness**: First-come-first-served across ALL pods
- **Never changes**: Same username even when moving territories
- **Used for**: User lookup, login, social identification

### 3. Username@Territory (Federated Identity)

- **Purpose**: Federation-compatible address
- **Format**: `{username}@{territory}`
- **Example**: `alice@denmark`, `bob@norway`
- **Changes on migration**: Updates to new territory
- **Used for**: Matrix protocol, external federation, display

### 4. Public Key Hash (Cryptographic Identity Placeholder)

- **Purpose**: Future Holochain cryptographic identity
- **Current Format**: SHA-256 hash of username + territory + UUID
- **Example**: `a1b2c3d4...` (64 character hex string)
- **Generation**: `SHA-256(username + "::" + territory_code + "::" + uuid)`
- **Never changes**: Preserves original registration identity
- **Future**: Will become hash of actual agent public key (Holochain)

### 5. Email (Optional Contact)

- **Purpose**: Optional notification channel
- **Format**: Standard email address
- **Example**: `alice@protonmail.com` (or NULL)
- **Territory-specific**: Stored only in territory schema
- **Not used for identity**: Just a communication preference
- **Can change**: User can update or remove anytime

## Public Key Hash Generation

### Current Implementation (Pre-Holochain)

```rust
pub fn generate_public_key_hash(
    username: &str, 
    territory_code: &str, 
    user_id: &Uuid
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(username.as_bytes());
    hasher.update(b"::");
    hasher.update(territory_code.as_bytes());
    hasher.update(b"::");
    hasher.update(user_id.as_bytes());
    format!("{:x}", hasher.finalize())
}
```

**Why this approach:**

- **Uniqueness**: UUID guarantees no collisions
- **Original identity preservation**: Encodes registration username@territory
- **Reproducible**: Can regenerate from stored data
- **Migration-safe**: Hash never changes even when user moves territories

### Future Implementation (Holochain)

```rust
pub fn generate_public_key_hash(agent_public_key: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(agent_public_key);
    format!("{:x}", hasher.finalize())
}
```

## Registration Flow

### Step-by-Step Process

```
1. User selects username: "alice"
2. Check global.user_identities.username
   - If taken: Reject, suggest alternatives
   - If available: Proceed
   
3. Optional: User provides email
   - If provided: Store in territory_dk.users.email
   - If not: Leave NULL
   
4. Create territory user:
   INSERT INTO territory_dk.users 
   (username, email, password_hash, ...)
   VALUES ('alice', 'alice@pm.me', '$argon2...', ...)
   RETURNING id;  -- Returns: dk-user-456
   
5. Create global identity:
   INSERT INTO global.user_identities
   (username, public_key_hash, territory_code, territory_user_id)
   VALUES (
     'alice',
     SHA-256('alice::dk::' + uuid),  -- Generated server-side
     'dk',
     'dk-user-456'
   )
   RETURNING id;  -- Returns: global-uuid-123
   
6. User is registered:
   - Global identity: uuid-123
   - Username: "alice" (globally reserved)
   - Territory user: dk-user-456
   - Federated ID: alice@denmark
   - Matrix ID: @alice:unityplan.dk
```

## Territory Migration

### Use Case: User Moves from Denmark to Norway

**Before Migration:**

```
global.user_identities:
  - id: uuid-123 (permanent)
  - username: "alice" (permanent)
  - public_key_hash: SHA-256("alice::dk::uuid-123") (permanent)
  - territory_code: "dk"
  - territory_user_id: dk-user-456

territory_dk.users:
  - id: dk-user-456
  - username: "alice"
  - email: "alice@protonmail.com"
  - is_active: true
```

**Migration Process:**

```
1. User requests migration to Norway
2. Create new territory user:
   INSERT INTO territory_no.users
   (username, email, password_hash, ...)
   VALUES ('alice', NULL, '$argon2...', ...)
   RETURNING id;  -- Returns: no-user-789
   
3. Update global identity:
   UPDATE global.user_identities
   SET territory_code = 'no',
       territory_user_id = 'no-user-789',
       updated_at = NOW()
   WHERE id = 'uuid-123';
   
4. Deactivate old territory user (optional - for single citizenship):
   UPDATE territory_dk.users
   SET is_active = false
   WHERE id = 'dk-user-456';
   
   OR keep active for dual citizenship
```

**After Migration:**

```
global.user_identities:
  - id: uuid-123 (unchanged)
  - username: "alice" (unchanged)
  - public_key_hash: SHA-256("alice::dk::uuid-123") (unchanged - preserves origin)
  - territory_code: "no" (updated)
  - territory_user_id: no-user-789 (updated)

territory_dk.users:
  - id: dk-user-456
  - username: "alice"
  - is_active: false (or true for dual citizenship)

territory_no.users:
  - id: no-user-789
  - username: "alice"
  - is_active: true
```

### Multi-Territory Citizenship

Users can be active in **multiple territories simultaneously**:

```
global.user_identities:
  - id: uuid-123
  - username: "alice"
  - territory_code: "no" (primary territory)

territory_dk.users:
  - id: dk-user-456
  - username: "alice"
  - is_active: true (active in Denmark communities)

territory_no.users:
  - id: no-user-789
  - username: "alice"
  - is_active: true (active in Norway communities)
```

**User can:**

- Participate in Denmark communities as `alice@denmark`
- Participate in Norway communities as `alice@norway`
- Same global identity (uuid-123) everywhere
- Old contacts in Denmark can still find them

## Matrix Protocol Integration

### Matrix ID Format

```
Primary Matrix ID: @{username}:unityplan.{territory}
```

**Examples:**

- User in Denmark: `@alice:unityplan.dk`
- User in Norway: `@alice:unityplan.no`
- User in Sweden: `@alice:unityplan.se`

### Territory Migration with Matrix

**Migration maintains conversation continuity:**

```
1. User starts in Denmark:
   Matrix ID: @alice:unityplan.dk
   
2. User moves to Norway:
   New primary ID: @alice:unityplan.no
   Alias (old conversations): @alice:unityplan.dk
   
3. Old contacts continue working:
   - Messages to @alice:unityplan.dk route to @alice:unityplan.no
   - User can respond from either ID
   - Gradual migration of contacts to new ID
```

### Implementation Mapping

```rust
// Generate Matrix ID from user identity
fn generate_matrix_id(username: &str, territory_code: &str) -> String {
    // Map territory code to domain
    let domain = match territory_code {
        "dk" => "unityplan.dk",
        "no" => "unityplan.no",
        "se" => "unityplan.se",
        _ => "unityplan.org", // Fallback
    };
    
    format!("@{}:{}", username, domain)
}

// alice in Denmark → @alice:unityplan.dk
// alice in Norway → @alice:unityplan.no
```

**Alias Management:**

- When user migrates, old Matrix ID becomes alias
- Matrix server maintains alias → primary ID mapping
- Transparent for existing conversations
- New conversations use new primary ID

## Invitation System

### With Email (Traditional)

```
1. Admin creates invitation:
   - invited_email: "bob@example.com"
   - Send email with invitation link/token
   
2. Bob clicks link:
   - Validates token
   - Pre-fills email (optional - can change or remove)
   - Chooses username
   - Completes registration
```

### Without Email (Privacy-Focused)

```
1. Admin creates invitation:
   - invited_email: NULL
   - Generate shareable token/link
   
2. Share invitation via:
   - QR code (print, scan with phone)
   - Messaging app (Signal, WhatsApp)
   - In-person code entry
   - Physical invitation card
   
3. User accesses invitation:
   - Validates token (bearer token - anyone with code can use)
   - Chooses username
   - Optional: Provides email for notifications
   - Completes registration
```

### Invitation Schema

```sql
CREATE TABLE territory_{code}.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token VARCHAR(255) UNIQUE NOT NULL,
    token_type VARCHAR(20) NOT NULL,  -- 'single_use', 'group'
    
    created_by_user_id UUID REFERENCES territory_{code}.users(id),
    invited_email VARCHAR(255) NULL,  -- OPTIONAL - for email invitations
    invited_username VARCHAR(50) NULL,  -- OPTIONAL - for targeted invitations
    
    max_uses INTEGER NULL,  -- NULL = unlimited
    current_uses INTEGER DEFAULT 0 NOT NULL,
    expires_at TIMESTAMPTZ NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## Holochain Migration Path

### Phase 1: Current (Username-Based Identity)

```
global.user_identities:
  - id UUID (permanent identity)
  - username VARCHAR (globally unique)
  - public_key_hash VARCHAR (SHA-256 of username::territory::uuid)
  - territory_code VARCHAR
  - territory_user_id UUID

Identity: Username-based
Authentication: Password + JWT
```

### Phase 2: Add Cryptographic Keys

```
global.user_identities:
  - id UUID (legacy, maintained)
  - username VARCHAR (alias for human lookup)
  - agent_public_key VARCHAR (ed25519 public key) -- NEW
  - public_key_hash VARCHAR (SHA-256 of agent_public_key) -- UPDATED
  - territory_code VARCHAR
  - territory_user_id UUID

Identity: Username OR cryptographic key
Authentication: Password + JWT OR signature challenge
```

**Key Generation:**

```rust
// Generate ed25519 keypair
let keypair = Ed25519KeyPair::generate();
let public_key = keypair.public_key();
let public_key_hash = sha256(public_key);

// Store in database
INSERT INTO global.user_identities
(id, username, agent_public_key, public_key_hash, ...)
VALUES (uuid, 'alice', public_key, public_key_hash, ...);

// User stores private key locally (encrypted)
```

### Phase 3: Holochain Integration

```
global.user_identities:
  - id UUID (legacy)
  - username VARCHAR (alias)
  - agent_public_key VARCHAR (PRIMARY identity)
  - public_key_hash VARCHAR (agent address in Holochain)
  - holochain_agent_address VARCHAR (native Holochain ID) -- NEW
  - territory_code VARCHAR
  - territory_user_id UUID

Identity: Cryptographic (agent_public_key)
Authentication: Signature challenge (sign with private key)
Username: Optional lookup alias
```

**Identity Verification:**

```rust
// Challenge-response authentication
fn verify_identity(challenge: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    ed25519::verify(public_key, challenge, signature)
}

// User proves ownership by signing challenge
let challenge = generate_random_challenge();
let signature = user_signs_with_private_key(challenge);
let verified = verify_identity(challenge, signature, user.agent_public_key);
```

### Phase 4: Full Holochain (Future)

```
Holochain DNA:
  - Agent public key: PRIMARY identity
  - Agent address: Hash of public key
  - Username: Optional alias in profile
  
Federation Bridge:
  - global.user_identities maps username → agent_public_key
  - Username lookup resolves to Holochain agent
  - Legacy systems use username, native Holochain uses agent key

Identity: Fully cryptographic, user-sovereign
Authentication: Cryptographic signatures only
Username: Display alias, not identity
```

**Benefits:**

- User owns their private key (true sovereignty)
- Can prove identity cryptographically
- No server can impersonate user
- Identity portable across systems
- Username just convenience layer (like contacts app)

## Identity Lookup and Resolution

### By Username (Human-Friendly)

```sql
-- Find user by globally unique username
SELECT 
    ui.id AS global_user_id,
    ui.username,
    ui.territory_code,
    ui.public_key_hash,
    t.code AS territory_code,
    t.name AS territory_name
FROM global.user_identities ui
JOIN global.territories t ON ui.territory_code = t.code
WHERE ui.username = 'alice';
```

### By Global UUID (System References)

```sql
-- Find user by permanent global ID
SELECT 
    ui.id,
    ui.username,
    ui.territory_code,
    tu.email,
    tu.display_name,
    tu.is_active
FROM global.user_identities ui
JOIN territory_dk.users tu ON ui.territory_user_id = tu.id
WHERE ui.id = '550e8400-e29b-41d4-a716-446655440000';
```

### By Public Key Hash (Future Cryptographic)

```sql
-- Find user by cryptographic identity
SELECT 
    ui.id,
    ui.username,
    ui.agent_public_key,
    ui.territory_code
FROM global.user_identities ui
WHERE ui.public_key_hash = 'a1b2c3d4...';
```

### By Username@Territory (Federated)

```sql
-- Resolve federated ID (alice@denmark)
SELECT 
    ui.id,
    ui.username,
    ui.territory_code,
    tu.id AS territory_user_id
FROM global.user_identities ui
JOIN territory_dk.users tu ON ui.territory_user_id = tu.id
WHERE ui.username = 'alice' 
  AND ui.territory_code = 'dk';
```

## Security Considerations

### Username Uniqueness Enforcement

**Problem**: Race condition when two users register same username simultaneously

**Solution**: Database-level UNIQUE constraint + application-level check

```sql
-- Database enforces uniqueness
CREATE UNIQUE INDEX idx_global_user_identities_username 
ON global.user_identities(LOWER(username));
```

```rust
// Application checks availability before registration
async fn check_username_available(pool: &PgPool, username: &str) -> Result<bool> {
    let exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM global.user_identities WHERE LOWER(username) = LOWER($1))",
        username.to_lowercase()
    )
    .fetch_one(pool)
    .await?
    .exists
    .unwrap_or(false);
    
    Ok(!exists)
}
```

### Public Key Hash Collisions

**Probability**: SHA-256 collision is cryptographically infeasible (2^128 attempts)

**Safety**: UUID uniqueness + SHA-256 guarantees no practical collision risk

**Verification**: Database UNIQUE constraint on public_key_hash column

### Email Privacy

**Risk**: Email addresses identify users across systems

**Mitigation**:

- Email is optional (can be NULL)
- Stored only in territory schema (not global)
- Not used for identity or authentication
- User can remove/change anytime
- Invitation system works without email

### Territory Migration Security

**Attack**: Malicious actor tries to hijack user identity during migration

**Prevention**:

- Migration requires authenticated session
- Only current user can migrate their own identity
- Old territory user can be deactivated (prevents dual access)
- Audit log records all migration events

```rust
// Only allow user to migrate their own account
async fn migrate_territory(
    user_id: Uuid,  // From authenticated session
    target_territory: &str,
    pool: &PgPool
) -> Result<()> {
    // Verify user owns this identity
    let identity = get_user_identity(pool, user_id).await?;
    
    // Create new territory user
    let new_territory_user = create_territory_user(target_territory, &identity).await?;
    
    // Update global identity (atomic transaction)
    let result = sqlx::query!(
        "UPDATE global.user_identities 
         SET territory_code = $1, 
             territory_user_id = $2,
             updated_at = NOW()
         WHERE id = $3",
        target_territory,
        new_territory_user.id,
        user_id
    )
    .execute(pool)
    .await?;
    
    // Audit log
    log_territory_migration(user_id, identity.territory_code, target_territory).await?;
    
    Ok(())
}
```

## Summary

**Current Implementation (MVP):**

- ✅ Email optional (privacy-first)
- ✅ Globally unique usernames
- ✅ Username@territory federation
- ✅ Public key hash placeholder (SHA-256 of username::territory::uuid)
- ✅ Territory migration support
- ✅ Matrix protocol compatible (`@username:unityplan.{territory}`)
- ✅ Invitation system with/without email

**Future Holochain Migration:**

- 🔄 Add cryptographic keypair generation
- 🔄 Replace placeholder hash with real public key hash
- 🔄 Username becomes alias (not primary identity)
- 🔄 Signature-based authentication
- 🔄 Full user-sovereign identity

**Benefits:**

- User sovereignty (own your identity)
- Privacy-first (no forced email)
- Future-proof (Holochain ready)
- Federation-compatible (Matrix, ActivityPub)
- Seamless migration (username preserved)
- Human-friendly (readable usernames)
- Cryptographically secure (future)

This architecture balances **practical usability today** with a **clear migration path to decentralized, cryptographic identity** in the future.
