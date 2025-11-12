# community-service

**Port:** 8006  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Scaffolded (0/8 endpoints) - Phase 2  
**Bounded Context:** Communities & Groups

---

## 📋 Overview

The community-service manages communities (groups), memberships, roles, and community settings. Communities are the foundation of collaboration in the platform.

### **Responsibilities**

- ⏳ Create and manage communities
- ⏳ Community membership management
- ⏳ Role assignment (admin, moderator, member)
- ⏳ Community settings and visibility
- ⏳ Community discovery (public communities)
- ⏳ Invitation to communities

### **Not Responsible For**

- ❌ Forum posts/discussions (handled by forum-service)
- ❌ Events (handled by event-service)
- ❌ Courses within communities (handled by course-service)
- ❌ File sharing (handled by ipfs-service)

---

## 🔐 Authentication

This service uses **JWT-based authentication** via shared middleware from `shared-lib`.

### **Validation Strategy**

- **Public routes:** No authentication (list public communities)
- **Standard requests:** JWT signature validation only (~0.01ms)
- **Admin operations:** Role verification (JWT + database check for community roles)

### **Middleware**

```rust
use shared_lib::middleware::jwt_auth_middleware;

HttpServer::new(|| {
    App::new()
        .service(list_public_communities)  // No auth
        .wrap(jwt_auth_middleware)  // Protects all other routes
        .service(create_community)
        .service(join_community)
        .service(manage_roles)  // Additional role check in handler
})
```

**See [shared-lib/AUTHENTICATION.md](../shared-lib/AUTHENTICATION.md) for complete authentication architecture.**

---

## 🗄️ Database Schema

### **Tables Owned by community-service**

#### **1. communities**

```sql
CREATE TABLE territory_{code}.communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Basic Info
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,  -- URL-friendly name
    description TEXT,
    avatar_url VARCHAR(500),
    banner_url VARCHAR(500),
    
    -- Ownership
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Visibility
    visibility VARCHAR(20) DEFAULT 'public',  -- public/private/hidden
    require_approval BOOLEAN DEFAULT false,
    
    -- Stats
    member_count INT DEFAULT 1,
    
    -- Territory
    territory_code VARCHAR(10) NOT NULL,
    
    CONSTRAINT valid_visibility CHECK (visibility IN ('public', 'private', 'hidden'))
);

CREATE INDEX idx_communities_slug ON communities(slug);
CREATE INDEX idx_communities_created_by ON communities(created_by);
CREATE INDEX idx_communities_visibility ON communities(visibility) WHERE visibility = 'public';
CREATE INDEX idx_communities_territory ON communities(territory_code);
CREATE INDEX idx_communities_member_count ON communities(member_count DESC);
```

**Purpose:** Community definitions  
**Holochain Entry Type:** `Community` (public DHT)  
**Visibility:**

- `public` - Listed in directory, anyone can join
- `private` - Listed in directory, requires approval
- `hidden` - Not listed, invitation only

#### **2. community_members**

```sql
CREATE TABLE territory_{code}.community_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Relationship
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Role
    role VARCHAR(20) DEFAULT 'member',  -- owner/admin/moderator/member
    
    -- Timestamps
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(community_id, user_id),
    CONSTRAINT valid_role CHECK (role IN ('owner', 'admin', 'moderator', 'member'))
);

CREATE INDEX idx_community_members_community ON community_members(community_id);
CREATE INDEX idx_community_members_user ON community_members(user_id);
CREATE INDEX idx_community_members_role ON community_members(community_id, role);
```

**Purpose:** Community membership tracking  
**Holochain Entry Type:** `CommunityMember` (link: Community → Member)  
**Roles:**

- `owner` - Community creator, full control
- `admin` - Can manage members, settings
- `moderator` - Can moderate content
- `member` - Regular member

#### **3. community_settings**

```sql
CREATE TABLE territory_{code}.community_settings (
    community_id UUID PRIMARY KEY REFERENCES communities(id) ON DELETE CASCADE,
    
    -- Member Permissions
    members_can_invite BOOLEAN DEFAULT true,
    members_can_post BOOLEAN DEFAULT true,
    members_can_create_events BOOLEAN DEFAULT false,
    
    -- Moderation
    auto_approve_posts BOOLEAN DEFAULT true,
    auto_approve_members BOOLEAN DEFAULT true,
    
    -- Notifications
    notify_on_new_member BOOLEAN DEFAULT true,
    notify_on_new_post BOOLEAN DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Purpose:** Community-specific settings  
**Holochain Entry Type:** `CommunitySettings` (private to admins)

---

## 🔌 API Endpoints

### **1. Create Community**

#### **POST /v1/communities**

Create a new community (authenticated users only)

**Request:**

```json
{
  "name": "Rust Developers Denmark",
  "slug": "rust-developers-dk",
  "description": "A community for Rust developers in Denmark",
  "visibility": "public",
  "require_approval": false
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "Rust Developers Denmark",
    "slug": "rust-developers-dk",
    "description": "A community for Rust developers in Denmark",
    "visibility": "public",
    "created_by": "uuid",
    "member_count": 1,
    "created_at": "2025-11-12T10:00:00Z"
  }
}
```

**Actions:**

- Create community
- Add creator as `owner` in `community_members`
- Publish `community.created` NATS event

---

### **2. Get Community**

#### **GET /v1/communities/{slug}**

Get community details

**Response:**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "Rust Developers Denmark",
    "slug": "rust-developers-dk",
    "description": "A community for Rust developers in Denmark",
    "avatar_url": "https://cdn.unityplatform.org/avatars/community-uuid.jpg",
    "visibility": "public",
    "member_count": 42,
    "created_by": "uuid",
    "created_at": "2025-11-12T10:00:00Z",
    "current_user_role": "member"  // null if not a member
  }
}
```

---

### **3. List Communities**

#### **GET /v1/communities**

List public communities (discovery)

**Query Parameters:**

- `search` - Search by name/description
- `territory` - Filter by territory code
- `sort` - Sort by (members/newest)
- `page` - Page number
- `limit` - Results per page

**Response:**

```json
{
  "success": true,
  "data": {
    "communities": [
      {
        "id": "uuid",
        "name": "Rust Developers Denmark",
        "slug": "rust-developers-dk",
        "description": "A community for Rust developers in Denmark",
        "avatar_url": "https://...",
        "member_count": 42,
        "visibility": "public"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 15
    }
  }
}
```

---

### **4. Update Community**

#### **PATCH /v1/communities/{id}**

Update community details (admin/owner only)

**Request:**

```json
{
  "description": "Updated description",
  "visibility": "private",
  "require_approval": true
}
```

---

### **5. Delete Community**

#### **DELETE /v1/communities/{id}**

Delete community (owner only)

**Response:**

```json
{
  "success": true,
  "data": {
    "deleted": true
  }
}
```

---

### **6. Join Community**

#### **POST /v1/communities/{id}/join**

Join a community

**Response:**

```json
{
  "success": true,
  "data": {
    "membership": {
      "community_id": "uuid",
      "user_id": "uuid",
      "role": "member",
      "joined_at": "2025-11-12T10:00:00Z",
      "pending_approval": false
    }
  }
}
```

**Actions:**

- If `require_approval = false`, join immediately
- If `require_approval = true`, create pending membership
- Increment `member_count`
- Publish `community.member_joined` NATS event
- Notify admins if pending approval

---

### **7. Leave Community**

#### **POST /v1/communities/{id}/leave**

Leave a community

**Response:**

```json
{
  "success": true,
  "data": {
    "left": true
  }
}
```

---

### **8. Manage Members**

#### **GET /v1/communities/{id}/members**

List community members (members can see list)

**Query Parameters:**

- `role` - Filter by role
- `page` - Page number
- `limit` - Results per page

**Response:**

```json
{
  "success": true,
  "data": {
    "members": [
      {
        "user_id": "uuid",
        "username": "alice",
        "avatar_url": "https://...",
        "role": "admin",
        "joined_at": "2025-11-10T10:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 50,
      "total": 42
    }
  }
}
```

#### **PATCH /v1/communities/{id}/members/{user_id}**

Update member role (admin/owner only)

**Request:**

```json
{
  "role": "moderator"
}
```

#### **DELETE /v1/communities/{id}/members/{user_id}**

Remove member (admin/owner only)

---

## 🔗 Service Dependencies

### **Outbound Calls**

#### **user-service**

- **When:** Getting member details for community member list
- **Endpoint:** `GET /v1/users/{id}` (batch)
- **Purpose:** Enrich member list with user profiles

#### **notification-service**

- **When:** Community events (new member, approval needed)
- **Communication:** NATS events
- **Purpose:** Notify admins/members

---

### **Inbound Calls**

#### **Frontend**

- **Endpoints:** All community endpoints
- **Purpose:** Community discovery, management, membership

#### **forum-service** (future)

- **When:** Creating forum post
- **Purpose:** Verify user is community member

#### **event-service** (future)

- **When:** Creating community event
- **Purpose:** Verify user has permission

---

## 📡 NATS Events

### **Published Events**

```typescript
// Community created
{
  event: "community.created",
  community_id: "uuid",
  community_name: "Rust Developers Denmark",
  created_by: "uuid",
  visibility: "public",
  timestamp: "ISO8601"
}

// Member joined
{
  event: "community.member_joined",
  community_id: "uuid",
  user_id: "uuid",
  role: "member",
  timestamp: "ISO8601"
}

// Member left
{
  event: "community.member_left",
  community_id: "uuid",
  user_id: "uuid",
  timestamp: "ISO8601"
}
```

### **Subscribed Events**

```typescript
// User deleted (remove from communities)
{
  event: "user.deleted",
  user_id: "uuid"
}
// Action: Remove user from all communities
```

---

## 🔮 Holochain Migration

### **DNA Design: communities.happ**

#### **Entry Types**

**1. Community** (Public DHT)

```rust
#[hdk_entry_helper]
struct Community {
    name: String,
    slug: String,
    description: String,
    visibility: CommunityVisibility,
    created_by: AgentPubKey,
    created_at: Timestamp,
}
```

**2. CommunityMember** (Link)

```rust
// Link: Community → Member
create_link(
    community_hash,
    agent_pub_key,
    LinkTag::from("member"),
    LinkType::CommunityMember
)?;
```

#### **Validation**

```rust
validate_create_community(community: Community) {
    - Name must be 3-100 characters
    - Slug must be unique (DHT query)
    - created_by must be caller
}

validate_create_link_community_member(link: Link) {
    - Community must exist
    - Agent not already a member
    - If private, check invitation or approval
}
```

---

## ✅ Implementation Status

### **Completed**

- ✅ Database schema designed
- ✅ Service scaffolded

### **Pending** (Phase 2)

- ⏳ All 8 endpoints
- ⏳ Membership management
- ⏳ Role-based permissions
- ⏳ Community discovery
- ⏳ NATS integration
- ⏳ Tests

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Phase 2 (After user-service, auth-service, settings, invitations, notifications)  
**Dependencies:** user-service, notification-service
