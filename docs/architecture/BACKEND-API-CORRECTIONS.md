# Backend API Documentation - Corrections Applied

**Date:** November 11, 2025  
**Status:** Documentation Updated  
**Changes:** Critical architecture clarifications

---

## Summary of Changes

Three critical corrections have been applied to the backend API documentation based on architectural review:

### 1. ✅ Email is Optional (Not Required)

**Correction:** Email is NOT used for authentication - it's optional and used only for external notifications.

**What Changed:**

- User entity: `email: string` → `email?: string | null`
- Registration: Email is now optional field
- Purpose: Email used only for: invitations, password reset, notifications
- Authentication: Users log in with **username + password**, NOT email

**Files Updated:**

- `backend-api-requirements.md` - User entity and registration flow
- `database-schema-design.md` - Users table schema and constraints

**Key Points:**

- Privacy-first: Users don't need email to use the system
- Invitation system works without email (QR codes, shareable links)
- Email verification only happens if user provides email
- Global email registry only tracks provided emails

### 2. ✅ Username is Primary Identifier (Not Matrix ID)

**Correction:** Username is the permanent, primary identifier - Matrix ID is just a federated representation.

**What Changed:**

- Clarified username as primary identifier (never changes)
- Matrix ID is derived from username@territory
- Territory migration: username stays same, Matrix ID changes (old becomes alias)

**Identity Hierarchy:**

1. **Username** (Primary) - `alice` - Never changes, globally unique
2. **Matrix ID** (Federated) - `@alice:unityplan.dk` - Changes with territory, old becomes alias
3. **UUID** (Internal) - Database primary key
4. **Public Key Hash** (Future) - Holochain cryptographic identity

**Territory Migration Example:**

```
Before:
- Username: alice (permanent)
- Matrix ID: @alice:unityplan.dk (primary)

After Migration:
- Username: alice (unchanged)
- Matrix ID: @alice:unityplan.no (new primary)
- Matrix Alias: @alice:unityplan.dk (still works, redirects to new)
```

**Files Updated:**

- `database-schema-design.md` - Added identity system integration section
- Clarified in user table documentation

### 3. ✅ Territory Schema Naming (Consistent Approach)

**Correction:** Schema names use territory codes consistently for all deployment types.

**What Changed:**

- **Old:** Suggested `territory_1`, `territory_2` for multi-territory pods
- **New:** Use territory codes (`territory_dk`, `territory_no`, `territory_se`) for both single and multi-territory
- **Single-territory pod:** `territory_dk`
- **Multi-territory pod:** `territory_dk`, `territory_no`, `territory_se`

**Migration Script Design:**

```bash
# Single-territory pod
./migrate.sh --schema territory_dk --territory-code dk

# Multi-territory pod
./migrate.sh --schema territory_dk --territory-code dk
./migrate.sh --schema territory_no --territory-code no
./migrate.sh --schema territory_se --territory-code se
```

./migrate.sh --schema territory_3 --territory-code se

```

**SQL Template Example:**

```sql
-- Parameterized migration
CREATE SCHEMA IF NOT EXISTS :schema_name;

CREATE TABLE :schema_name.users (
    territory_code VARCHAR(10) NOT NULL DEFAULT :'territory_code',
    -- ...
);
```

**Pod Configuration:**

```yaml
# Single-territory pod
database:
  mode: single_territory
  schema_name: territory_dk
  territory_code: dk

# Multi-territory pod
database:
  mode: multi_territory
  territories:
    - { schema_name: territory_dk, territory_code: dk }
    - { schema_name: territory_no, territory_code: no }
    - { schema_name: territory_se, territory_code: se }
```

**Files Updated:**

- `database-schema-design.md` - All table definitions use `{schema_name}`
- Added migration strategy section for single vs multi-territory
- Updated schema structure overview

---

## Impact on Implementation

### Database Migrations

**Before:**

```sql
CREATE TABLE territory_dk.users (...);
CREATE TABLE territory_no.users (...);
```

**After:**

```sql
-- Use parameterized scripts
CREATE TABLE :schema_name.users (...);
```

### Application Code

**Before:**

```rust
let schema = format!("territory_{}", territory_code);
```

**After:**

```rust
// Schema name matches territory code
let schema = format!("territory_{}", territory_code);
// Returns: "territory_dk" (single or multi-territory)
```

### Email Handling

**Before:**

```rust
// Registration required email
pub struct RegisterRequest {
    pub email: String,  // Required
    pub username: String,
    pub password: String,
}
```

**After:**

```rust
// Email is optional
pub struct RegisterRequest {
    pub username: String,  // Primary identifier
    pub password: String,
    pub email: Option<String>,  // Optional - for notifications
}
```

---

## Updated Documentation Files

All changes committed to:

1. **database-schema-design.md**
   - Schema naming convention explained
   - `{schema_name}` template in all SQL
   - Email marked as OPTIONAL in users table
   - Identity system integration section added
   - Migration strategy for single/multi-territory

2. **backend-api-requirements.md**
   - User entity: email is optional
   - Registration: email optional
   - Identity hierarchy explained
   - Territory migration clarified

3. **This file:** `BACKEND-API-CORRECTIONS.md`
   - Summary of all changes
   - Implementation impact
   - Examples for both deployment modes

---

## Key Takeaways

### 🔑 Core Architectural Principles

1. **Username is King**
   - Primary human-readable identifier
   - Globally unique across all pods
   - Never changes (even with territory migration)
   - Used for login, social identification

2. **Email is Optional**
   - Privacy-first design
   - Used only for external notifications
   - Not required for system use
   - Invitation system works without it

3. **Matrix ID is Derived**
   - Format: `@username:unityplan.{territory}`
   - Changes when user migrates
   - Old Matrix IDs become aliases
   - Username remains constant anchor

4. **Consistent Schema Naming**
   - Both single and multi-pod: Use territory code (`territory_dk`, `territory_no`, etc.)
   - All migrations parameterized
   - Schema name format: `territory_{code}`

---

## Next Steps

When implementing:

1. **Create Migration Scripts**
   - Use `:schema_name` and `:territory_code` parameters
   - Test both single and multi-territory modes
   - Ensure email constraints allow NULL

2. **Update Application Config**
   - Add deployment mode (single/multi-territory)
   - Add schema mapping configuration
   - Load at startup

3. **Implement Identity System**
   - Global username registry
   - Optional email registry
   - Matrix ID generation
   - Territory migration support

4. **Build Registration Flow**
   - Email optional in request
   - Send verification only if email provided
   - Username as primary identifier

---

**Documentation Status:** ✅ Complete and Consistent  
**Ready for Implementation:** Yes  
**Breaking Changes:** No (these are clarifications before implementation)
