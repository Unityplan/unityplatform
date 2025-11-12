# invitation-service

**Port:** 8004  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Scaffolded (0/6 endpoints)  
**Bounded Context:** Invitation Management & Tracking

---

## 📋 Overview

The invitation-service manages the entire invitation lifecycle: creation, validation, tracking, and revocation. All user registrations require a valid invitation token.

### **Responsibilities**

- ⏳ Generate unique invitation tokens
- ⏳ Validate invitation tokens (during registration)
- ⏳ Track invitation usage (who invited whom)
- ⏳ Revoke invitations (admin or creator)
- ⏳ List user's sent invitations with status
- ⏳ Global invitation uniqueness enforcement

### **Not Responsible For**

- ❌ User registration (handled by auth-service)
- ❌ User profiles (handled by user-service)
- ❌ Invitation emails/notifications (handled by notification-service)
- ❌ Community invitations (handled by community-service)

---

## 🗄️ Database Schema

### **Tables Owned by invitation-service**

#### **1. invitation_tokens (territory-specific)**

```sql
CREATE TABLE territory_{code}.invitation_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(255) UNIQUE NOT NULL,  -- Unique invitation code
    
    -- Ownership
    created_by UUID REFERENCES users(id) ON DELETE CASCADE,  -- Who created this invitation
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Usage limits
    max_uses INT DEFAULT 1,  -- 1 = single-use, 0 = unlimited
    uses_count INT DEFAULT 0,
    
    -- Expiration
    expires_at TIMESTAMPTZ,  -- NULL = never expires
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    revoked_at TIMESTAMPTZ,
    revoked_by UUID REFERENCES users(id),
    
    -- Metadata
    metadata JSONB DEFAULT '{}'::jsonb,  -- Custom data (e.g., community_id, role)
    
    CONSTRAINT valid_max_uses CHECK (max_uses >= 0),
    CONSTRAINT valid_uses_count CHECK (uses_count >= 0 AND uses_count <= max_uses),
    CONSTRAINT revoked_has_timestamp CHECK (
        (is_active = false AND revoked_at IS NOT NULL) OR is_active = true
    )
);

CREATE INDEX idx_invitation_tokens_token ON invitation_tokens(token);
CREATE INDEX idx_invitation_tokens_created_by ON invitation_tokens(created_by);
CREATE INDEX idx_invitation_tokens_active ON invitation_tokens(is_active) WHERE is_active = true;
CREATE INDEX idx_invitation_tokens_expires ON invitation_tokens(expires_at) WHERE expires_at IS NOT NULL;
```

**Purpose:** Primary invitation token storage  
**Holochain Entry Type:** `InvitationToken` (public chain - inviter's reputation)  
**Token Format:** 16-character alphanumeric (e.g., `A7K9-M2X4-P5W8-Q1Z3`)

#### **2. invitation_uses (territory-specific)**

```sql
CREATE TABLE territory_{code}.invitation_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invitation_id UUID NOT NULL REFERENCES invitation_tokens(id) ON DELETE CASCADE,
    
    -- Who used it
    used_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    used_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Context
    ip_address INET,
    user_agent TEXT,
    
    UNIQUE(invitation_id, used_by)  -- Prevent duplicate use by same user
);

CREATE INDEX idx_invitation_uses_invitation ON invitation_uses(invitation_id);
CREATE INDEX idx_invitation_uses_user ON invitation_uses(used_by);
CREATE INDEX idx_invitation_uses_timestamp ON invitation_uses(used_at);
```

**Purpose:** Track who used which invitation and when  
**Holochain Entry Type:** `InvitationUse` (public - builds trust graph)  
**Use Case:** Invitation tree visualization, spam prevention

#### **3. invitation_token_registry (global - uniqueness enforcement)**

```sql
CREATE TABLE global.invitation_token_registry (
    token VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(token)
);

CREATE INDEX idx_invitation_token_registry_territory ON invitation_token_registry(territory_code);
```

**Purpose:** Ensure invitation codes are globally unique across all territories  
**Example:** `A7K9-M2X4-P5W8-Q1Z3` can only exist once across all pods  
**Cleanup:** Delete entry when token is fully used or expires

---

## 🔌 API Endpoints

### **1. Validate Invitation Token**

#### **POST /v1/invitations/validate**

Validate if an invitation token is usable (called by auth-service during registration)

**Request:**

```json
{
  "token": "A7K9-M2X4-P5W8-Q1Z3"
}
```

**Response (Valid):**

```json
{
  "success": true,
  "data": {
    "valid": true,
    "invitation_id": "uuid",
    "created_by": "uuid",
    "uses_remaining": 0,  // 0 = unlimited, N = number left
    "expires_at": null,
    "metadata": {}
  }
}
```

**Response (Invalid):**

```json
{
  "success": false,
  "error": {
    "code": "INVALID_INVITATION",
    "message": "Invitation token is invalid, expired, or fully used"
  }
}
```

**Validation Rules:**
- Token exists in database
- `is_active = true`
- `expires_at > NOW()` (or NULL)
- `uses_count < max_uses` (or `max_uses = 0`)

---

### **2. Mark Invitation as Used**

#### **POST /v1/invitations/use**

Mark invitation as used after successful registration (called by auth-service)

**Request:**

```json
{
  "token": "A7K9-M2X4-P5W8-Q1Z3",
  "used_by": "uuid",
  "ip_address": "192.168.1.100",
  "user_agent": "Mozilla/5.0..."
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "invitation_id": "uuid",
    "uses_remaining": 0,
    "fully_used": true
  }
}
```

**Actions:**
- Insert into `invitation_uses`
- Increment `invitation_tokens.uses_count`
- If `uses_count >= max_uses`, set `is_active = false`
- Publish `invitation.used` NATS event

---

### **3. Create Invitation**

#### **POST /v1/invitations**

Create a new invitation token (requires authentication)

**Request:**

```json
{
  "max_uses": 1,  // 1 = single-use, 0 = unlimited
  "expires_in_days": 7,  // NULL = never expires
  "metadata": {
    "purpose": "friend_invite",
    "community_id": "uuid"
  }
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "token": "A7K9-M2X4-P5W8-Q1Z3",
    "created_by": "uuid",
    "max_uses": 1,
    "expires_at": "2025-11-19T10:00:00Z",
    "invite_url": "https://unityplan.org/join?invite=A7K9-M2X4-P5W8-Q1Z3"
  }
}
```

**Token Generation:**
- 16 random alphanumeric characters
- Formatted as `XXXX-XXXX-XXXX-XXXX`
- Checked against global registry for uniqueness
- Retry if collision (extremely rare)

---

### **4. List My Invitations**

#### **GET /v1/invitations/me**

Get all invitations created by the authenticated user

**Query Parameters:**
- `status` - Filter by status (active/used/expired/revoked)
- `page` - Page number
- `limit` - Results per page (max 100)

**Response:**

```json
{
  "success": true,
  "data": {
    "invitations": [
      {
        "id": "uuid",
        "token": "A7K9-M2X4-P5W8-Q1Z3",
        "max_uses": 1,
        "uses_count": 1,
        "is_active": false,
        "expires_at": null,
        "created_at": "2025-11-10T10:00:00Z",
        "status": "used",
        "uses": [
          {
            "used_by": "uuid",
            "username": "alice",
            "used_at": "2025-11-11T10:00:00Z"
          }
        ]
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 5
    }
  }
}
```

---

### **5. Get Invitation Uses**

#### **GET /v1/invitations/{id}/uses**

Get all users who used a specific invitation (own invitations only)

**Response:**

```json
{
  "success": true,
  "data": {
    "invitation_id": "uuid",
    "token": "A7K9-M2X4-P5W8-Q1Z3",
    "uses": [
      {
        "user_id": "uuid",
        "username": "alice",
        "used_at": "2025-11-11T10:00:00Z",
        "ip_address": "192.168.1.100"
      }
    ],
    "total_uses": 1,
    "max_uses": 1
  }
}
```

---

### **6. Revoke Invitation**

#### **DELETE /v1/invitations/{id}**

Revoke an invitation (creator can revoke own, admins can revoke any)

**Response:**

```json
{
  "success": true,
  "data": {
    "invitation_id": "uuid",
    "revoked_at": "2025-11-12T10:00:00Z"
  }
}
```

**Actions:**
- Set `is_active = false`
- Set `revoked_at = NOW()`
- Set `revoked_by = current_user_id`
- Publish `invitation.revoked` NATS event

---

## 🔗 Service Dependencies

### **Outbound Calls (Services invitation-service depends on)**

None - invitation-service is a **leaf service**

---

### **Inbound Calls (Services that call invitation-service)**

#### **auth-service**

- **When:** User registration
- **Endpoints:**
  - `POST /v1/invitations/validate` (check if token is valid)
  - `POST /v1/invitations/use` (mark as used after successful registration)
- **Purpose:** Enforce invitation-only registration
- **Fallback:** Registration fails if service unavailable (critical dependency)

#### **Frontend**

- **When:** User wants to invite friends
- **Endpoints:**
  - `POST /v1/invitations` (create new invitation)
  - `GET /v1/invitations/me` (list my invitations)
  - `GET /v1/invitations/{id}/uses` (see who joined)
  - `DELETE /v1/invitations/{id}` (revoke invitation)
- **Purpose:** Invitation management UI

#### **notification-service**

- **When:** Invitation created or used
- **Purpose:** Send email with invitation link, notify inviter when someone joins
- **Communication:** NATS events

---

## 📡 NATS Events

### **Published Events**

```typescript
// Invitation created
{
  event: "invitation.created",
  invitation_id: "uuid",
  created_by: "uuid",
  token: "A7K9-M2X4-P5W8-Q1Z3",
  max_uses: 1,
  expires_at: "ISO8601",
  timestamp: "ISO8601"
}

// Invitation used
{
  event: "invitation.used",
  invitation_id: "uuid",
  token: "A7K9-M2X4-P5W8-Q1Z3",
  used_by: "uuid",
  uses_remaining: 0,
  fully_used: true,
  timestamp: "ISO8601"
}

// Invitation revoked
{
  event: "invitation.revoked",
  invitation_id: "uuid",
  revoked_by: "uuid",
  reason: "admin_action",
  timestamp: "ISO8601"
}
```

### **Subscribed Events**

None - invitation-service doesn't subscribe to events

---

## 🔮 Holochain Migration

### **DNA Design: invitations.happ**

#### **Entry Types**

**1. InvitationToken** (Public - builds reputation)

```rust
#[hdk_entry_helper]
struct InvitationToken {
    token: String,
    created_by: AgentPubKey,
    created_at: Timestamp,
    max_uses: u32,  // 0 = unlimited
    expires_at: Option<Timestamp>,
    metadata: BTreeMap<String, String>,
}
```

**2. InvitationUse** (Public - trust graph)

```rust
#[hdk_entry_helper]
struct InvitationUse {
    invitation_hash: EntryHash,
    used_by: AgentPubKey,
    used_at: Timestamp,
}
```

#### **Links**

```rust
// Inviter → Invitations they created
Path::from("inviter").join(agent_pub_key) → InvitationToken

// Invitation → Who used it
InvitationToken → InvitationUse

// User → How they joined (who invited them)
AgentPubKey → InvitationUse
```

#### **Validation Rules**

```rust
// Invitation creation
validate_create_invitation_token(token: InvitationToken) {
    - Token must be unique (global DHT check)
    - max_uses >= 0
    - created_by must be caller's agent key
}

// Invitation use
validate_create_invitation_use(use: InvitationUse) {
    - Invitation must exist
    - Not expired
    - Uses < max_uses (query existing uses)
    - used_by must be caller's agent key
}
```

#### **Trust Graph Visualization**

```
Alice (inviter)
  ├─ Bob (invited by Alice)
  │   ├─ Charlie (invited by Bob)
  │   └─ Diana (invited by Bob)
  └─ Eve (invited by Alice)
```

This creates a **decentralized reputation system**:
- If Bob's invitees spam, Bob's reputation decreases
- Trust flows through invitation chains
- No central authority needed

#### **Migration Strategy**

**Phase 1:** PostgreSQL storage
- Current implementation with PostgreSQL
- Invitation tokens stored in database

**Phase 2:** Dual storage (PostgreSQL + Holochain)
- Write to both PostgreSQL and Holochain
- Read from PostgreSQL (fast queries)
- Holochain builds trust graph

**Phase 3:** Holochain native
- Read/write directly from Holochain DHT
- PostgreSQL removed
- Pure decentralized trust graph

---

## ✅ Implementation Status

### **Completed**

- ✅ Database schema designed
- ✅ Models defined (Rust structs)
- ✅ Service scaffolded

### **Pending** (Week 2-3 of migration plan)

- ⏳ Implement POST /v1/invitations/validate
- ⏳ Implement POST /v1/invitations/use
- ⏳ Implement POST /v1/invitations
- ⏳ Implement GET /v1/invitations/me
- ⏳ Implement GET /v1/invitations/{id}/uses
- ⏳ Implement DELETE /v1/invitations/{id}
- ⏳ Token generation with global uniqueness check
- ⏳ Update auth-service to call invitation-service
- ⏳ Frontend integration (invite friends UI)
- ⏳ Tests (unit + integration)
- ⏳ OpenAPI documentation

---

## 🧪 Testing

### **Test Plan**

```bash
cd services/invitation-service
cargo test

# Test cases to implement:
# - Create invitation (single-use, multi-use, unlimited)
# - Validate invitation (valid, expired, fully used, revoked)
# - Mark invitation as used
# - Prevent duplicate use by same user
# - List user's invitations
# - Get invitation uses
# - Revoke invitation
# - Global token uniqueness enforcement
# - Invitation expiration
# - Rate limiting (prevent invitation spam)
```

---

## 📊 Token Generation

### **Algorithm**

```rust
use rand::Rng;

fn generate_invitation_token() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = rand::thread_rng();
    
    let token: String = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    // Format as XXXX-XXXX-XXXX-XXXX
    format!(
        "{}-{}-{}-{}",
        &token[0..4],
        &token[4..8],
        &token[8..12],
        &token[12..16]
    )
}
```

**Character Set:** Excludes confusing characters (0, O, I, 1)  
**Length:** 16 characters (33^16 = ~10^24 combinations)  
**Collision Probability:** Negligible (checked against global registry)

---

## 🔒 Security Considerations

### **Rate Limiting**

- Max 10 invitation creations per user per day
- Prevents invitation spam
- Admin-created invitations exempt from limit

### **Invitation Abuse Prevention**

- Track IP addresses in `invitation_uses`
- Flag users who create many invitations with no uses
- Flag users whose invitees are banned (spam indicator)

### **Global Uniqueness**

- `invitation_token_registry` prevents cross-territory duplicates
- Tokens checked before creation
- Cleanup: Delete from registry when token expires or is fully used

---

## 📈 Metrics & Monitoring

### **Key Metrics**

- Invitations created per day
- Invitation usage rate (% of invitations that get used)
- Average time between creation and use
- Invitation trees (depth and breadth)
- Revoked invitations (spam indicator)

### **Alerts**

- Spike in invitation creation (potential spam)
- Low usage rate (invitations not working?)
- High revocation rate (abuse?)

---

## 🚀 Implementation Priority

**Week 2-3 of Migration Plan:**

1. **Day 1-2:** Implement validation endpoints
   - POST /v1/invitations/validate
   - POST /v1/invitations/use
   - Token generation with global uniqueness

2. **Day 3:** Implement management endpoints
   - POST /v1/invitations (create)
   - GET /v1/invitations/me (list)
   - DELETE /v1/invitations/{id} (revoke)

3. **Day 4:** Update auth-service integration
   - Replace built-in invitation logic with API calls
   - Migration script for existing tokens
   - Testing

4. **Day 5:** Frontend & testing
   - Invite friends UI
   - Invitation tree visualization
   - End-to-end tests

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Ready for Implementation (Week 2-3)  
**Migration Plan:** [Service Separation Migration](../../guides/development/service-separation-migration.md)
