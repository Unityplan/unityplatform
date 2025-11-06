# User Data Sovereignty Architecture

**Version:** 0.1.0-alpha.1  
**Migration:** 20251106000001  
**Status:** Critical architectural correction

---

## 🌸 Core Principle: Users Belong to Their Pods

Using the natural ecosystem metaphor:
- **Flowers (users)** bloom in their specific pod (territory)
- **Seeds (user data)** stay in that pod's soil (territory schema)
- **Pollen (cryptographic hash)** can travel via mycorrhizal network (global schema)
- **No flower is uprooted** - users stay rooted in their territory

## 🚨 The Problem (Old Architecture)

**WRONG:** `global.users` table containing personal data

```sql
-- ❌ INCORRECT: Personal data in global schema
CREATE TABLE global.users (
    id UUID PRIMARY KEY,
    email VARCHAR(255),      -- Personal data!
    username VARCHAR(50),    -- Personal data!
    password_hash VARCHAR,   -- Personal data!
    display_name VARCHAR,    -- Personal data!
    bio TEXT,                -- Personal data!
    ...
);
```

**Why this violates data sovereignty:**
- Users are flowers that bloom in specific pods (territories)
- Personal data leaving territory schema violates user sovereignty
- Global schema should only coordinate, not store personal information
- Incompatible with future Holochain migration (user-controlled data)

## ✅ The Solution (Correct Architecture)

### Global Schema: Cryptographic Identities ONLY

```sql
-- ✅ CORRECT: Only cryptographic hashes, NO personal data
CREATE TABLE global.user_identities (
    public_key_hash VARCHAR(64) PRIMARY KEY,  -- Cryptographic identity
    territory_code VARCHAR(100) NOT NULL,     -- Where user data lives
    territory_user_id UUID NOT NULL,          -- ID within territory schema
    created_at TIMESTAMPTZ,
    last_seen_at TIMESTAMPTZ
);
```

**What this stores:**
- ✅ Cryptographic public key hash (future: Holochain agent ID)
- ✅ Territory code (which pod owns this user)
- ✅ Territory-local user ID (reference to actual user record)
- ❌ NO email, username, name, or any personal data

### Territory Schema: ALL Personal Data

```sql
-- ✅ CORRECT: All personal data in territory schema
CREATE TABLE territory_dk.users (
    id UUID PRIMARY KEY,
    
    -- Future Holochain identity
    public_key_hash VARCHAR(64) UNIQUE,
    
    -- Current auth (temporary)
    email VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255),
    
    -- Profile data (STAYS IN TERRITORY)
    username VARCHAR(50) UNIQUE NOT NULL,
    full_name VARCHAR(255),
    display_name VARCHAR(100),
    avatar_url TEXT,
    bio TEXT,
    
    -- Privacy controls
    email_visible BOOLEAN DEFAULT FALSE,
    profile_public BOOLEAN DEFAULT TRUE,
    
    -- Status
    is_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    last_login_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**What this stores:**
- ✅ ALL personal data (email, username, profile)
- ✅ User-controlled privacy settings
- ✅ Authentication credentials (current: password, future: optional)
- ✅ Cryptographic identity hash (links to global.user_identities)

## 🔐 Authentication Flow

### Current (Email/Password)

1. **User Registration:**
   ```
   POST /auth/register { email, username, password, territory_code }
   
   → Create territory_dk.users record
   → Hash password with Argon2
   → Generate public_key_hash (Blake2b of email+username)
   → Insert into global.user_identities (hash, territory_code, user_id)
   → Return JWT with { public_key_hash, territory_code }
   ```

2. **User Login:**
   ```
   POST /auth/login { email, password, territory_code }
   
   → Query territory_{code}.users WHERE email = ?
   → Verify password hash
   → Lookup public_key_hash
   → Return JWT with { public_key_hash, territory_code }
   ```

3. **Protected Endpoints:**
   ```
   JWT contains: { public_key_hash, territory_code }
   
   → Extract territory_code from JWT
   → SET search_path = territory_{code}
   → Query users WHERE public_key_hash = ?
   → Return user data from territory schema
   ```

### Future (WebAuthn/Holochain)

1. **WebAuthn (Phase 2 - Beta):**
   ```
   - Email becomes optional
   - User registers passkey (cryptographic keypair)
   - public_key_hash = hash of WebAuthn public key
   - Password recovery via backup codes (not email)
   ```

2. **Holochain (Phase 3):**
   ```
   - User controls their own cryptographic keypair
   - public_key_hash = Holochain agent public key
   - Email fully optional (contact method only)
   - User data stored in their Holochain DNA (pod)
   - Cross-pod coordination via global.user_identities
   ```

## 🔄 Data Flow Examples

### Cross-Territory User Lookup

**Scenario:** Forum post from user in Denmark, viewed in Sweden

```sql
-- Step 1: Get cryptographic identity from post
SELECT public_key_hash FROM territory_se.forum_posts WHERE id = ?;

-- Step 2: Lookup which territory owns this user
SELECT territory_code FROM global.user_identities 
WHERE public_key_hash = ?;
-- Returns: 'DK'

-- Step 3: Query user's public profile from their territory
SET search_path = territory_dk;
SELECT username, display_name, avatar_url 
FROM users 
WHERE public_key_hash = ?
AND profile_public = true;  -- Respect privacy settings
```

### User Data Export (GDPR Compliance)

**Scenario:** User requests their data

```sql
-- Step 1: Authenticate user, get territory_code from JWT
SET search_path = territory_dk;

-- Step 2: Export ALL data from territory schema
SELECT * FROM users WHERE id = ?;
SELECT * FROM user_profiles WHERE user_id = ?;
SELECT * FROM refresh_tokens WHERE user_id = ?;
-- ... all territory-specific user data

-- Step 3: Include global identity hash
SELECT public_key_hash, created_at, last_seen_at
FROM global.user_identities
WHERE territory_code = 'DK' AND territory_user_id = ?;

-- Result: Complete data export, all in one JSON file
```

## 🛡️ Security & Privacy Benefits

1. **Data Sovereignty:**
   - User data never leaves their territory
   - Each pod controls its own users
   - GDPR compliance: data stays in EU if user chooses EU territory

2. **Privacy by Design:**
   - Global schema only has cryptographic hashes
   - Personal data not exposed in cross-territory queries
   - Users control privacy settings in their territory

3. **Future-Proof:**
   - Compatible with Holochain agent identities
   - Supports WebAuthn passwordless authentication
   - Email becomes optional (user-controlled contact method)

4. **Audit Trail:**
   - All actions logged with public_key_hash (pseudonymous)
   - Personal data not in audit logs
   - Users can request audit of their actions

## 📊 Database Schema Diagram

```
┌─────────────────────────────────────────────────────────────┐
│ GLOBAL SCHEMA (Cross-territory coordination)               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  user_identities                                            │
│  ┌─────────────────────────────┐                           │
│  │ public_key_hash (PK)        │ ← Cryptographic identity  │
│  │ territory_code              │ ← Which pod owns user     │
│  │ territory_user_id           │ ← ID in territory schema  │
│  │ created_at, last_seen_at    │                           │
│  └─────────────────────────────┘                           │
│                                                             │
│  territories                                                │
│  ┌─────────────────────────────┐                           │
│  │ code (PK)                   │                           │
│  │ name, type, parent_code     │                           │
│  └─────────────────────────────┘                           │
│                                                             │
│  sessions, audit_log, territory_managers, role_assignments │
│  (all use public_key_hash, NO personal data)               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ References
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ TERRITORY SCHEMA (e.g., territory_dk)                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  users (ALL PERSONAL DATA)                                  │
│  ┌─────────────────────────────┐                           │
│  │ id (PK)                     │                           │
│  │ public_key_hash (links up)  │ ─┐                        │
│  │ email, password_hash        │  │                        │
│  │ username, full_name         │  │ Link to global         │
│  │ display_name, avatar_url    │  │ user_identities        │
│  │ bio, privacy_settings       │  │                        │
│  │ is_verified, is_active      │ ─┘                        │
│  └─────────────────────────────┘                           │
│                                                             │
│  refresh_tokens, user_profiles, user_badges, etc.          │
│  (all territory-specific user data)                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Migration Impact

### What Changes

**Before Migration:**
- ❌ global.users with personal data
- ❌ Direct references to global.users(id)

**After Migration:**
- ✅ global.user_identities with cryptographic hashes only
- ✅ territory_*.users with ALL personal data
- ✅ All references use public_key_hash

### Breaking Changes

1. **Auth Service:**
   - Must query territory schema, not global
   - JWT contains public_key_hash + territory_code
   - Registration creates territory user + global identity

2. **User Lookups:**
   - Old: `SELECT * FROM global.users WHERE id = ?`
   - New: `SET search_path = territory_{code}; SELECT * FROM users WHERE public_key_hash = ?`

3. **Foreign Keys:**
   - Old: `user_id UUID REFERENCES global.users(id)`
   - New: `public_key_hash VARCHAR(64) REFERENCES global.user_identities(public_key_hash)`
   - **OR** (for territory-specific data): `user_id UUID REFERENCES territory_{code}.users(id)`

## 🧪 Testing the Migration

```bash
# Apply migration
cd services/shared-lib
sqlx migrate run

# Verify structure
psql -U postgres -d unityplan_dk -c "
  \d global.user_identities;
  \d territory_dk.users;
"

# Test user creation
psql -U postgres -d unityplan_dk -c "
  -- Create user in territory
  INSERT INTO territory_dk.users (username, email, password_hash, public_key_hash)
  VALUES ('testuser', 'test@example.com', 'hash', 'test_hash_123');
  
  -- Verify global identity created (via trigger)
  SELECT * FROM global.user_identities WHERE public_key_hash = 'test_hash_123';
"

# Rollback if needed
sqlx migrate revert
```

## 📚 Related Documentation

- `docs/project/overview.md` - Natural ecosystem metaphor
- `docs/architecture/territory-management-standard.md` - Territory structure
- `docs/guides/development/versioning-strategy.md` - Version management
- `.github/copilot-instructions.md` - AI context (includes ecosystem metaphor)

## 🚀 Next Steps

1. **Update auth-service implementation:**
   - Query territory schemas for user data
   - Generate public_key_hash on registration
   - Include territory_code in JWT

2. **Update documentation:**
   - Rust backend development plan
   - API endpoint documentation
   - Authentication flow diagrams

3. **Future enhancements:**
   - WebAuthn support (Phase 2)
   - Holochain migration (Phase 3)
   - Cross-territory user discovery (privacy-preserving)

---

**Remember:** Users are flowers that bloom in their pods. Their data stays in their territory, while their cryptographic identity (pollen) can travel through the mycorrhizal network for cross-territory coordination.
