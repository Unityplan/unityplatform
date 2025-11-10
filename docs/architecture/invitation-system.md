# Invitation-Based Registration System

**Status:** Implemented (Phase 1) ✅  
**Last Updated:** November 10, 2025  
**Security Model:** Database-enforced territory binding

---

## Overview

UnityPlan uses an **invitation-only registration system** to maintain community quality, prevent spam, and align with the user sovereignty model where communities control their membership.

### Key Security Feature (November 2025 Update)

**Territory Binding:** Invitation tokens are bound to specific territories via a **global registry table**. This prevents users from accidentally or maliciously registering in the wrong territory. The territory is determined by the database, not client input.

## Philosophy

- **No open registration** - Users cannot self-register without an invitation
- **Community gatekeeping** - Existing members control who joins
- **Territory management** - Territory managers can invite groups of users
- **Trust network** - Growth happens through trusted connections
- **Security-first** - Territory binding enforced in database, not client-provided ✅

---

## Invitation Token Types

### 1. **Single-Use Token** (Personal Invitation)

- Valid for **one registration only**
- Sent to a **specific email address**
- Created by any user with `invite_user` permission
- Expires after a set period (default: 7 days)
- Automatically revoked after use

**Use Case:** Invite a specific person you know

**Example:**

```json
{
  "token_type": "single_use",
  "email": "alice@example.com",
  "created_by": "user-uuid",
  "expires_at": "2025-11-13T12:00:00Z",
  "max_uses": 1,
  "used_count": 0
}
```

### 2. **Group Token** (Multi-Use Invitation)

- Valid for **multiple registrations** (configurable limit)
- Not tied to specific email addresses
- Created by territory managers or users with `invite_group` permission
- Expires after a set period OR usage limit reached
- Useful for onboarding groups, workshops, or courses

**Use Case:** Invite a class of students, workshop participants, or community group

**Example:**

```json
{
  "token_type": "group",
  "max_uses": 50,
  "used_count": 23,
  "created_by": "manager-uuid",
  "expires_at": "2025-12-31T23:59:59Z",
  "metadata": {
    "purpose": "Winter 2025 Permaculture Course",
    "group_name": "Permaculture Students"
  }
}
```

---

## Database Schema

### **1. Territory Invitation Tokens** (territory_*.invitation_tokens)

Stores invitation tokens within each territory schema. Each token is created by a territory user.

```sql
CREATE TABLE territory_dk.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token VARCHAR(64) UNIQUE NOT NULL,  -- Cryptographically random
    token_type VARCHAR(20) NOT NULL CHECK (token_type IN ('single_use', 'group')),
    
    -- Restrictions
    email VARCHAR(255),  -- NULL for group tokens, specific for single_use
    max_uses INT NOT NULL DEFAULT 1,
    used_count INT NOT NULL DEFAULT 0,
    
    -- Metadata
    created_by_user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    community_id UUID REFERENCES territory_dk.communities(id),  -- ⭐ Optional community assignment
    purpose TEXT,  -- Optional description
    metadata JSONB,  -- Additional data (group name, course info, etc.)
    
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

### **2. Global Token Registry** (global.invitation_token_registry) ⭐ NEW

**Purpose:** Secure token-to-territory mapping. Prevents users from selecting wrong territory.

**Security Model:** Territory binding is **database-enforced**, not client-provided.

```sql
CREATE TABLE global.invitation_token_registry (
    token VARCHAR(255) PRIMARY KEY,  -- Same token as in territory schema
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code) ON DELETE CASCADE,
    territory_token_id UUID NOT NULL,  -- FK to territory_X.invitation_tokens.id
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Ensure token is globally unique across all territories
    CONSTRAINT uq_global_invitation_token UNIQUE (token)
);

CREATE INDEX idx_global_invitation_registry_token ON global.invitation_token_registry(token);
CREATE INDEX idx_global_invitation_registry_territory ON global.invitation_token_registry(territory_code);
```

**How it works:**

1. When invitation token is created in `territory_dk.invitation_tokens`, it's also registered in `global.invitation_token_registry` with `territory_code = 'dk'`
2. When user enters token during registration, backend queries `global.invitation_token_registry` to determine territory
3. Client **cannot manipulate** territory selection (database-enforced)
4. Backend then validates full token details in the appropriate territory schema

### **3. Territory Invitation Uses** (territory_*.invitation_uses)

```sql
-- Track who used which invitation (audit trail)
CREATE TABLE territory_dk.invitation_uses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    invitation_token_id UUID NOT NULL REFERENCES territory_dk.invitation_tokens(id),
    user_id UUID NOT NULL REFERENCES territory_dk.users(id),
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,  -- Optional: track IP for security
    user_agent TEXT   -- Optional: track browser/device
);

CREATE INDEX idx_invitation_uses_token ON territory_dk.invitation_uses(invitation_token_id);
CREATE INDEX idx_invitation_uses_user ON territory_dk.invitation_uses(user_id);
```

---

## Registration Flow

### **Secure Flow (Invitation-Based with Territory Binding)** ✅

```
User Flow:
  1. User receives invitation: "Join Denmark Territory" + token inv_abc123
  2. User opens registration page
  3. User enters invitation token (NO territory selection)
  4. Frontend validates token via GET /api/auth/invitations/validate/{token}
  5. Backend looks up territory from global.invitation_token_registry
  6. Backend returns: { territory: "Denmark", community: "Copenhagen Guild" }
  7. Frontend displays: "You're joining Denmark Territory → Copenhagen Guild"
  8. User completes registration (username, password, etc.)
  9. Backend derives territory from token (client cannot manipulate)
  10. User created in correct territory schema with community assignment

Backend Processing:
  1. Query global.invitation_token_registry for territory_code
  2. Validate token exists in territory_{code}.invitation_tokens
  3. Check token is active and not expired
  4. For single_use: Verify email matches token.email (if provided)
  5. For group: Check used_count < max_uses
  6. Create user account in territory_{code}.users
  7. If token has community_id: assign user to that community
  8. Increment token.used_count
  9. Record invitation use in invitation_uses table
  10. If single_use OR group token reached max_uses: mark token as inactive
  11. Return access/refresh tokens
```

**Security Benefits:**

- ✅ Territory binding enforced in database (client cannot manipulate)
- ✅ Community context preserved automatically
- ✅ Prevents accidental registration in wrong territory
- ✅ Prevents malicious territory manipulation
- ✅ Clear UX: user sees exactly where they're joining

---

## API Endpoints

### **Validate Invitation (Updated)** ⭐

```http
GET /api/auth/invitations/validate/{token}
# ⭐ NO territory_code parameter - backend looks it up

# Response:
{
  "valid": true,
  "token_type": "single_use",
  "territory": {
    "code": "dk",
    "name": "Denmark"
  },
  "community": {                    # ⭐ If invitation has community
    "id": "uuid",
    "name": "Copenhagen Permaculture Guild"
  },
  "email": "alice@example.com",     # Only for single_use tokens
  "expires_at": "2025-11-13T12:00:00Z",
  "remaining_uses": 1
}
```

### **Registration (Updated)** ⭐

```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "alice@example.com",
  "username": "alice_dk",
  "password": "SecurePass123!",
  "full_name": "Alice Denmark",
  # ⭐ REMOVED: "territory_code" - backend derives from token
  "invitation_token": "inv_a7bd3632957845479"  # ⭐ REQUIRED - territory looked up from this
}
```

**Response (Success):**

```json
{
  "user": { "id": "...", "username": "alice_dk", "territory_code": "dk", ... },
  "access_token": "eyJ0eXAi...",
  "refresh_token": "414426c9...",
  "expires_in": 900
}
```

**Response (Invalid Token):**

```json
{
  "error": "Invalid or expired invitation token"
}
```

**Response (Token Already Used - Single Use):**

```json
{
  "error": "Invitation token has already been used"
}
```

**Response (Email Mismatch - Single Use):**

```json
{
  "error": "This invitation is for a different email address"
}
```

**Response (Success):**

```json
{
  "user": { "id": "...", "username": "alice_dk", ... },
  "access_token": "eyJ0eXAi...",
  "refresh_token": "414426c9...",
  "expires_in": 900
}
```

**Response (Invalid Token):**

```json
{
  "error": "Invalid or expired invitation token"
}
```

**Response (Token Already Used - Single Use):**

```json
{
  "error": "Invitation token has already been used"
}
```

**Response (Email Mismatch - Single Use):**

```json
{
  "error": "This invitation is for a different email address"
}
```

### **Create Invitation Token (New Endpoint)**

```http
POST /api/auth/invitations
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "token_type": "single_use",  // or "group"
  "email": "alice@example.com",  // Required for single_use, null for group
  "max_uses": 1,  // 1 for single_use, N for group
  "expires_in_days": 7,  // Optional, default: 7
  "purpose": "Invite Alice to join our community"  // Optional
}
```

**Response:**

```json
{
  "invitation": {
    "id": "uuid",
    "token": "inv_a7bd3632957845479",
    "token_type": "single_use",
    "email": "alice@example.com",
    "max_uses": 1,
    "used_count": 0,
    "expires_at": "2025-11-13T12:00:00Z",
    "created_at": "2025-11-06T12:00:00Z"
  }
}
```

### **List Invitations**

```http
GET /api/auth/invitations
Authorization: Bearer <access_token>

# Response: List of invitations created by current user
```

### **Revoke Invitation**

```http
DELETE /api/auth/invitations/{token_id}
Authorization: Bearer <access_token>

# Response: 204 No Content
```

### **Validate Invitation (Public)**

```http
GET /api/auth/invitations/validate/{token}

# Response:
{
  "valid": true,
  "token_type": "single_use",
  "email": "alice@example.com",  // Only for single_use
  "expires_at": "2025-11-13T12:00:00Z",
  "uses_remaining": 1
}
```

---

## Permissions

### **Who Can Invite?**

| Permission          | Token Type  | Description                           |
|---------------------|-------------|---------------------------------------|
| `invite_user`       | single_use  | Regular users can invite individuals  |
| `invite_group`      | group       | Community leaders can invite groups   |
| `manage_territory`  | both        | Territory managers have full access   |

**Default Settings:**

- New users: No invitation permissions by default
- Community moderators: `invite_user` permission
- Territory managers: All invitation permissions

---

## Security Considerations

### **Token Generation**

```rust
// Generate cryptographically secure random token
use rand::Rng;
use sha2::{Digest, Sha256};

fn generate_invitation_token() -> String {
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
    let hash = Sha256::digest(&random_bytes);
    format!("inv_{}", hex::encode(&hash[..16]))  // 32 hex chars + prefix
}
```

### **Rate Limiting**

- Max 10 invitations per user per day (configurable)
- Max 100 group invitations per territory manager per month
- Prevent invitation spam

### **Audit Trail**

- Log all invitation creation (who, when, type)
- Log all invitation uses (who used, when, IP)
- Track revocations (who revoked, when, why)

### **Token Expiration**

- Single-use: 7 days default (configurable)
- Group: 30 days default (configurable)
- Auto-cleanup: Delete expired tokens after 90 days

---

## Migration Strategy

### **Phase 1: Add Invitation System (Current)**

1. Create invitation_tokens and invitation_uses tables
2. Update /api/auth/register to require invitation_token
3. Implement invitation CRUD endpoints
4. Add invitation management to auth-service

### **Phase 2: Seed Initial Invitations**

1. Territory managers get default invitation permissions
2. Create initial group tokens for early adopters
3. Document invitation workflow for community leaders

### **Phase 3: Frontend Integration**

1. Invitation management UI (create, view, revoke)
2. Registration form with token input
3. Token validation on frontend before submission

---

## Example Workflows

### **Workflow 1: Territory Manager Invites Workshop Participants**

```
1. Territory manager logs in
2. Creates group token:
   - Type: group
   - Max uses: 25
   - Expires: 30 days
   - Purpose: "Spring 2025 Gardening Workshop"
3. Shares token link with workshop participants
4. Participants register using the group token
5. Manager monitors invitation usage
6. After workshop, manager can revoke unused invitations
```

### **Workflow 2: User Invites a Friend**

```
1. User logs in
2. Creates single-use token:
   - Type: single_use
   - Email: friend@example.com
   - Expires: 7 days
3. System sends invitation email to friend (Phase 2)
4. Friend clicks link, registers with pre-filled email
5. Token automatically invalidated after use
```

---

## Future Enhancements (Phase 2+)

### **Badge-Based Invitations** ⭐ NEW

- **Attach badges to invitation tokens** - Pre-grant permissions and access rights
- **Automatic course enrollment** - Grant access to specific courses upon registration
- **Forum permissions** - Provide forum access rights through invitation
- **Conditional activation** - Badges activate after completing Code of Conduct course
- **Use Cases:**
  - Workshop invitation grants access to workshop materials course
  - Community invitation grants forum posting rights after onboarding
  - Teacher invitation grants course creation permissions
  - Student invitation auto-enrolls in semester courses

**Implementation Concept:**

```json
{
  "token_type": "group",
  "max_uses": 30,
  "purpose": "Spring 2025 Permaculture Course",
  "badges": [
    {
      "badge_id": "course-access-permaculture-101",
      "auto_grant": true,
      "requires_conduct_course": true
    },
    {
      "badge_id": "forum-category-gardening",
      "auto_grant": true,
      "requires_conduct_course": true
    }
  ]
}
```

**Flow:**

1. User accepts invitation and registers
2. User completes Code of Conduct course (mandatory)
3. System automatically grants badges from invitation
4. User gains immediate access to pre-approved courses and forums
5. Audit trail tracks badge grants via invitation

### **Other Future Features**

- **Email Integration**: Automatic invitation emails with magic links
- **Invitation Templates**: Pre-defined invitation messages
- **Invitation Analytics**: Track conversion rates, popular sources
- **Referral Rewards**: Gamification for inviting active members
- **Invitation Chains**: Track how communities grow (social graph)
- **Conditional Invitations**: Require profile completion, agreements, etc.
- **Bulk Import**: Upload CSV of emails for mass invitations
- **Invitation Webhooks**: Notify external systems when invitations are used

---

## Related Documentation

- [User Data Sovereignty](./user-data-sovereignty.md)
- [Territory Management Standard](./territory-management-standard.md)
- [Authentication Flow](./authentication-flow.md) (to be created)
- [Badge System](./badge-system.md) (to be created)
