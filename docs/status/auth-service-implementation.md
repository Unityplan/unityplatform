# Auth Service Implementation Plan

**Last Updated:** November 6, 2025  
**Status:** In Progress

---

## ✅ Completed (Option 1: Testing)

- [x] Service compiles and runs on port 8001
- [x] Health endpoint working
- [x] User registration (DK territory)
- [x] User login with JWT tokens
- [x] Password hashing (Argon2)
- [x] Dynamic schema routing (multi-territory support)
- [x] Data sovereignty validated (territory_dk.users + global.user_identities)
- [x] JWT token structure validated

---

## 🎯 In Progress (Option 2: Complete Auth Service)

### **Phase A: Invitation System** ⭐ CURRENT PRIORITY

#### Migration
- [x] Created migration 20251106000003_add_invitation_system
- [x] Applied to database (invitation_tokens + invitation_uses tables)
- [x] Added invited_by_token_id to users table

#### Models (To Do)
- [ ] InvitationToken model
- [ ] InvitationUse model
- [ ] CreateInvitationRequest
- [ ] InvitationResponse
- [ ] Update RegisterRequest (add invitation_token field)

#### Handlers (To Do)
- [ ] POST /api/auth/invitations - Create invitation token
- [ ] GET /api/auth/invitations - List user's invitations
- [ ] DELETE /api/auth/invitations/{id} - Revoke invitation
- [ ] GET /api/auth/invitations/validate/{token} - Validate token (public)
- [ ] Update POST /api/auth/register - Validate invitation before registration

#### Utilities (To Do)
- [ ] generate_invitation_token() - Cryptographically secure token generation
- [ ] validate_invitation() - Check token validity, expiration, uses

#### Bootstrap (To Do)
- [ ] Create initial group invitation for territory managers
- [ ] Document how to use invitations

---

### **Phase B: Token Management**

#### Refresh Token Endpoint
- [ ] POST /api/auth/refresh handler
- [ ] Validate refresh token in database
- [ ] Check expiration
- [ ] Token rotation (generate new refresh token)
- [ ] Update refresh_tokens table
- [ ] Return new access + refresh tokens

#### Logout Endpoint
- [ ] POST /api/auth/logout handler
- [ ] Validate refresh token
- [ ] Delete from refresh_tokens table
- [ ] Return success message

---

### **Phase C: Protected Routes**

#### JWT Middleware
- [ ] Create middleware module
- [ ] Extract Authorization header
- [ ] Validate JWT signature
- [ ] Decode claims
- [ ] Query user from territory schema
- [ ] Make AuthenticatedUser available to handlers
- [ ] Handle errors (expired, invalid, missing token)

#### Get Current User Endpoint
- [ ] GET /api/auth/me handler
- [ ] Use JWT middleware
- [ ] Return full user profile (respecting privacy)
- [ ] Include territory information

---

### **Phase D: Testing & Documentation**

#### Unit Tests
- [ ] Password hashing tests (already exists)
- [ ] Token generation tests (already exists)
- [ ] Invitation token generation tests
- [ ] Invitation validation logic tests
- [ ] Token expiration tests

#### Integration Tests
- [ ] Full registration flow (with invitation)
- [ ] Login flow
- [ ] Refresh token flow
- [ ] Logout flow
- [ ] Protected endpoint access
- [ ] Invitation creation/usage flow

#### Documentation
- [x] Invitation system architecture
- [ ] API endpoint documentation
- [ ] Frontend integration guide
- [ ] Territory manager guide (how to invite users)

---

## 📋 Implementation Steps (Detailed)

### **Step 1: Invitation Models** (NEXT)

Create invitation-related models in `services/auth-service/src/models/invitation.rs`:

```rust
// InvitationToken model
pub struct InvitationToken {
    pub id: Uuid,
    pub token: String,
    pub token_type: String, // "single_use" or "group"
    pub email: Option<String>,
    pub max_uses: i32,
    pub used_count: i32,
    pub created_by_user_id: Uuid,
    pub purpose: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by_user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request/Response models
pub struct CreateInvitationRequest {
    pub token_type: String,
    pub email: Option<String>,
    pub max_uses: i32,
    pub expires_in_days: Option<i64>,
    pub purpose: Option<String>,
}

pub struct InvitationResponse {
    pub id: Uuid,
    pub token: String,
    pub token_type: String,
    pub email: Option<String>,
    pub max_uses: i32,
    pub used_count: i32,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
```

### **Step 2: Update RegisterRequest**

Add `invitation_token` field to `RegisterRequest`:

```rust
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub full_name: Option<String>,
    pub territory_code: String,
    pub invitation_token: String,  // ⭐ NEW - REQUIRED
}
```

### **Step 3: Invitation Validation Logic**

Create `services/auth-service/src/services/invitation.rs`:

```rust
pub async fn validate_invitation_token(
    pool: &PgPool,
    schema_name: &str,
    token: &str,
    email: Option<&str>,
) -> Result<InvitationToken> {
    // 1. Query token from database
    // 2. Check is_active = true
    // 3. Check expires_at > now
    // 4. Check used_count < max_uses
    // 5. For single_use: verify email matches
    // 6. Return token if valid
}

pub async fn use_invitation_token(
    pool: &PgPool,
    schema_name: &str,
    token_id: Uuid,
    user_id: Uuid,
    ip_address: Option<String>,
) -> Result<()> {
    // 1. Increment used_count
    // 2. Create invitation_uses record
    // 3. If single_use OR max_uses reached: set is_active = false
}

pub fn generate_invitation_token() -> String {
    // Generate cryptographically secure random token
    // Format: "inv_" + 32 hex characters
}
```

### **Step 4: Update Register Handler**

Modify `handlers/auth.rs` register function:

```rust
pub async fn register(...) -> Result {
    // 1. Validate request (including invitation_token)
    // 2. Verify territory exists
    // 3. Validate invitation token ⭐ NEW
    // 4. Check email/username not taken
    // 5. Hash password
    // 6. Create user (with invited_by_token_id) ⭐ NEW
    // 7. Record invitation use ⭐ NEW
    // 8. Generate JWT tokens
    // 9. Return response
}
```

### **Step 5: Create Bootstrap Invitation**

Create a script or SQL to generate initial invitation for territory managers:

```sql
-- Bootstrap invitation for Denmark territory manager
INSERT INTO territory_dk.invitation_tokens (
    token,
    token_type,
    email,
    max_uses,
    created_by_user_id,
    purpose,
    expires_at,
    is_active
) VALUES (
    'inv_bootstrap_dk_manager_2025',  -- Known token for setup
    'single_use',
    'admin@denmark-territory.com',  -- Initial admin email
    1,
    (SELECT id FROM territory_dk.users LIMIT 1),  -- System user
    'Bootstrap invitation for Denmark territory manager',
    NOW() + INTERVAL '30 days',
    true
);
```

---

## 🔄 Implementation Flow

```
┌─────────────────────────────────────────────────────┐
│ Phase A: Invitation System (Current Priority)      │
│  1. Create models                                   │
│  2. Update RegisterRequest                          │
│  3. Implement invitation validation                 │
│  4. Create invitation CRUD endpoints                │
│  5. Update register handler                         │
│  6. Create bootstrap invitation                     │
└─────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────┐
│ Phase B: Token Management                          │
│  1. POST /api/auth/refresh                          │
│  2. POST /api/auth/logout                           │
└─────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────┐
│ Phase C: Protected Routes                          │
│  1. JWT middleware                                  │
│  2. GET /api/auth/me                                │
└─────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────┐
│ Phase D: Testing & Documentation                   │
│  1. Integration tests                               │
│  2. API documentation                               │
│  3. User guides                                     │
└─────────────────────────────────────────────────────┘
```

---

## 📝 Notes

- **Breaking Change:** Existing `/api/auth/register` endpoint will now require `invitation_token`
- **Migration Path:** Existing users (if any) are grandfathered in
- **Bootstrap:** Need initial invitation token for first territory manager
- **Testing:** Will need to create test invitation tokens for development

---

## 🎯 Current Task

**START HERE:** Step 1 - Create invitation models in `models/invitation.rs`

**Next:** Step 2 - Update RegisterRequest with invitation_token field

Should we proceed with implementing the invitation models?
