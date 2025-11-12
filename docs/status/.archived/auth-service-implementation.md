# Auth Service Implementation Plan

**Last Updated:** November 6, 2025  
**Status:** In Progress - Phase A Complete, Phase B Next

---

## ✅ Completed

### Testing & Core Authentication
- [x] Service compiles and runs on port 8001
- [x] Health endpoint working
- [x] User registration (DK territory)
- [x] User login with JWT tokens
- [x] Password hashing (Argon2)
- [x] Dynamic schema routing (multi-territory support)
- [x] Data sovereignty validated (territory_dk.users + global.user_identities)
- [x] JWT token structure validated

### Phase A: Invitation System ✅ COMPLETE
- [x] Created migration 20251106000003_add_invitation_system
- [x] Applied to database (invitation_tokens + invitation_uses tables)
- [x] Added invited_by_token_id to users table
- [x] InvitationToken model
- [x] InvitationUse model
- [x] CreateInvitationRequest
- [x] InvitationResponse
- [x] Updated RegisterRequest (added invitation_token field - BREAKING CHANGE)
- [x] POST /api/auth/invitations - Create invitation token
- [x] GET /api/auth/invitations - List user's invitations
- [x] DELETE /api/auth/invitations/{id} - Revoke invitation
- [x] GET /api/auth/invitations/{id}/uses - Get invitation usage audit trail
- [x] GET /api/auth/invitations/validate/{token} - Validate token (public endpoint)
- [x] Updated POST /api/auth/register - Validates invitation before registration
- [x] generate_invitation_token() - Cryptographically secure token generation
- [x] validate_invitation_token() - Check token validity, expiration, uses, email matching
- [x] use_invitation_token() - Mark token as used, create audit record
- [x] Bootstrap script (create-bootstrap-invitation.sh) for territory admins
- [x] JWT middleware protecting invitation management endpoints
- [x] User model updated with invited_by_token_id field
- [x] All SQL queries updated to include invited_by_token_id column

**Key Features:**
- Two token types: `single_use` (email-specific) and `group` (multi-use)
- Token format: `inv_` + 32 hex characters (cryptographically secure)
- Automatic deactivation when max_uses reached
- Audit trail tracks: user_id, timestamp, ip_address, user_agent
- Email validation for single_use tokens
- Territory-aware (tokens are territory-specific)

**Endpoints Tested:**
- ✅ POST /api/auth/invitations (protected)
- ✅ GET /api/auth/invitations (protected)
- ✅ DELETE /api/auth/invitations/{id} (protected)
- ✅ GET /api/auth/invitations/{id}/uses (protected)
- ✅ GET /api/auth/invitations/validate/{token} (public)

---

## 🎯 In Progress

### **Phase B: Token Management** ⭐ NEXT PRIORITY

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

#### JWT Middleware ✅ COMPLETE
- [x] Create middleware module
- [x] Extract Authorization header
- [x] Validate JWT signature
- [x] Decode claims
- [x] Query user from territory schema
- [x] Make AuthenticatedUser available to handlers
- [x] Handle errors (expired, invalid, missing token)

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
- [x] Invitation token generation tests (in invitation.rs service)
- [ ] Invitation validation logic tests
- [ ] Token expiration tests

#### Integration Tests
- [ ] Full registration flow (with invitation)
- [ ] Login flow
- [ ] Refresh token flow
- [ ] Logout flow
- [ ] Protected endpoint access
- [ ] Invitation creation/usage flow (manual testing complete)

#### Documentation
- [x] Invitation system architecture (docs/architecture/invitation-system.md)
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
---

## 📋 Next Steps

### Immediate (Phase B: Token Management)

1. **POST /api/auth/refresh** - Token rotation
   - Validate refresh token from database
   - Check expiration
   - Generate new access + refresh tokens
   - Update refresh_tokens table
   - Delete old refresh token

2. **POST /api/auth/logout** - Revoke session
   - Validate refresh token
   - Delete from refresh_tokens table
   - Return success message

3. **GET /api/auth/me** - Current user info
   - Use JWT middleware
   - Return user profile (respecting privacy settings)
   - Include territory information

### Future (Phase D: Testing & Documentation)

1. **Integration Tests**
   - Complete registration flow with invitation
   - Token refresh and rotation
   - Logout and session management
   - Invitation lifecycle (create → validate → use → revoke)

2. **API Documentation**
   - OpenAPI/Swagger specification
   - Request/response examples
   - Error handling guide
   - Rate limiting documentation

3. **User Guides**
   - Territory manager guide (invitation management)
   - Frontend integration examples
   - Best practices for token handling

---

## 🔮 Future Features (Phase 2+)

### **Badge-Based Invitations**
Extend invitation system to automatically grant badges and permissions upon registration.

**Features:**
- Attach one or more badges to invitation tokens
- Auto-grant course access permissions
- Auto-grant forum category permissions
- Conditional activation after Code of Conduct course completion

**Database Extension (Conceptual):**
```sql
-- Future: Link badges to invitations
CREATE TABLE territory_dk.invitation_badges (
    id UUID PRIMARY KEY,
    invitation_token_id UUID NOT NULL REFERENCES invitation_tokens(id),
    badge_id UUID NOT NULL,  -- Reference to badge system
    auto_grant BOOLEAN DEFAULT true,
    requires_conduct_course BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**API Extension (Conceptual):**
```json
POST /api/auth/invitations
{
  "token_type": "group",
  "max_uses": 30,
  "expires_in_days": 60,
  "purpose": "Spring 2025 Permaculture Course",
  "badges": [
    {
      "badge_type": "course_access",
      "course_id": "permaculture-101",
      "requires_conduct": true
    },
    {
      "badge_type": "forum_access",
      "category_id": "gardening-discussion",
      "requires_conduct": true
    }
  ]
}
```

**Use Cases:**
- Workshop invitations grant access to workshop materials
- Student invitations auto-enroll in semester courses
- Community invitations grant forum posting rights after onboarding
- Teacher invitations grant course creation permissions

**Implementation Priority:** Phase 2 (after Badge Service and Course Service are implemented)

---

## 📝 Notes

- **Breaking Change:** `/api/auth/register` now requires `invitation_token` field
- **Migration Path:** Bootstrap script creates initial admin invitations
- **Testing:** Manual testing complete for all invitation endpoints
- **Security:** Tokens are cryptographically secure (32 hex characters)
- **Audit:** Full audit trail for invitation usage
- **Territory Isolation:** All invitation data stored in territory schemas

---

## � Issues Fixed

1. **User model missing invited_by_token_id** - Added to model and all SQL queries
2. **JWT middleware user_id type mismatch** - Parse String to UUID before database query
3. **IP address column type** - Changed from inet to text for flexibility
4. **Bootstrap token foreign key** - Made created_by_user_id nullable for bootstrap scenarios

---

## 🎯 Current Task

**START HERE:** Step 1 - Create invitation models in `models/invitation.rs`

**Next:** Step 2 - Update RegisterRequest with invitation_token field

Should we proceed with implementing the invitation models?
