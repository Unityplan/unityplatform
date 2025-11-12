# badge-service

**Port:** 8007  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Scaffolded (0/6 endpoints) - Phase 2  
**Bounded Context:** Gamification & Achievements

---

## 📋 Overview

The badge-service manages the gamification system: badges, achievements, user progress, and badge awards.

### **Responsibilities**

- ⏳ Define badges (name, icon, criteria)
- ⏳ Award badges to users
- ⏳ Track badge progress
- ⏳ List available badges
- ⏳ Display user's earned badges
- ⏳ Badge rarity and statistics

### **Not Responsible For**

- ❌ Leaderboards (future: separate service)
- ❌ Points/XP system (not planned for MVP)
- ❌ Competitions (not planned for MVP)

---

## 🗄️ Database Schema

### **Tables Owned by badge-service**

#### **1. badges**

```sql
CREATE TABLE territory_{code}.badges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Basic Info
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    icon VARCHAR(255) NOT NULL,  -- Emoji or URL
    
    -- Criteria
    criteria_type VARCHAR(50) NOT NULL,  -- manual/follower_count/invitation_count/community_count
    criteria_value INT,  -- e.g., 10 followers
    
    -- Rarity
    rarity VARCHAR(20) DEFAULT 'common',  -- common/rare/epic/legendary
    
    -- Visibility
    is_active BOOLEAN DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT valid_rarity CHECK (rarity IN ('common', 'rare', 'epic', 'legendary'))
);

CREATE INDEX idx_badges_slug ON badges(slug);
CREATE INDEX idx_badges_active ON badges(is_active) WHERE is_active = true;
CREATE INDEX idx_badges_rarity ON badges(rarity);
```

**Purpose:** Badge definitions  
**Holochain Entry Type:** `Badge` (public DHT)  
**Example Badges:**
- Early Adopter 🌟 (manual award)
- Social Butterfly 🦋 (10 followers)
- Invitation Champion 🎫 (5 people joined via your invitation)
- Community Builder 🏘️ (created 3 communities)

#### **2. user_badges**

```sql
CREATE TABLE territory_{code}.user_badges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Relationship
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    badge_id UUID NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
    
    -- Award Info
    awarded_at TIMESTAMPTZ DEFAULT NOW(),
    awarded_by UUID REFERENCES users(id),  -- NULL = auto-awarded
    
    -- Display
    is_featured BOOLEAN DEFAULT false,  -- Show on profile
    
    UNIQUE(user_id, badge_id)  -- Can only earn each badge once
);

CREATE INDEX idx_user_badges_user ON user_badges(user_id);
CREATE INDEX idx_user_badges_badge ON user_badges(badge_id);
CREATE INDEX idx_user_badges_featured ON user_badges(user_id, is_featured) WHERE is_featured = true;
```

**Purpose:** Track which users earned which badges  
**Holochain Entry Type:** `UserBadge` (link: User → Badge)

#### **3. badge_progress**

```sql
CREATE TABLE territory_{code}.badge_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Relationship
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    badge_id UUID NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
    
    -- Progress
    current_value INT DEFAULT 0,
    target_value INT NOT NULL,
    
    -- Timestamps
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, badge_id)
);

CREATE INDEX idx_badge_progress_user ON badge_progress(user_id);
CREATE INDEX idx_badge_progress_badge ON badge_progress(badge_id);
```

**Purpose:** Track progress towards earning badges  
**Example:** User has 7/10 followers for "Social Butterfly" badge

---

## 🔌 API Endpoints

### **1. List Badges**

#### **GET /v1/badges**

Get all available badges

**Response:**

```json
{
  "success": true,
  "data": {
    "badges": [
      {
        "id": "uuid",
        "name": "Early Adopter",
        "slug": "early-adopter",
        "description": "Joined during the alpha phase",
        "icon": "🌟",
        "rarity": "epic",
        "earned": false,
        "progress": null
      },
      {
        "id": "uuid",
        "name": "Social Butterfly",
        "slug": "social-butterfly",
        "description": "Have 10 followers",
        "icon": "🦋",
        "rarity": "common",
        "earned": false,
        "progress": {
          "current": 7,
          "target": 10,
          "percentage": 70
        }
      }
    ]
  }
}
```

---

### **2. Get User Badges**

#### **GET /v1/badges/users/{user_id}**

Get badges earned by a user

**Response:**

```json
{
  "success": true,
  "data": {
    "badges": [
      {
        "id": "uuid",
        "name": "Early Adopter",
        "slug": "early-adopter",
        "icon": "🌟",
        "rarity": "epic",
        "awarded_at": "2025-11-10T10:00:00Z",
        "is_featured": true
      }
    ],
    "total_count": 5,
    "rarity_counts": {
      "common": 2,
      "rare": 2,
      "epic": 1,
      "legendary": 0
    }
  }
}
```

---

### **3. Award Badge**

#### **POST /v1/badges/award**

Award a badge to a user (admin only or auto-award system)

**Request:**

```json
{
  "user_id": "uuid",
  "badge_id": "uuid",
  "awarded_by": "uuid"  // Optional (null = auto-awarded)
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "user_badge_id": "uuid",
    "badge_name": "Early Adopter",
    "awarded_at": "2025-11-12T10:00:00Z"
  }
}
```

**Actions:**

- Insert into `user_badges`
- Publish `badge.awarded` NATS event
- Trigger notification to user

---

### **4. Update Progress**

#### **POST /v1/badges/progress**

Update badge progress (internal API, called by other services)

**Request:**

```json
{
  "user_id": "uuid",
  "badge_slug": "social-butterfly",
  "current_value": 8
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "progress": {
      "current": 8,
      "target": 10,
      "percentage": 80
    },
    "badge_earned": false
  }
}
```

**Auto-Award Logic:**

If `current_value >= target_value`, automatically award badge

---

### **5. Toggle Featured Badge**

#### **PATCH /v1/badges/users/me/{badge_id}/feature**

Feature/un-feature a badge on your profile

**Request:**

```json
{
  "is_featured": true
}
```

---

### **6. Get Badge Statistics**

#### **GET /v1/badges/{id}/stats**

Get badge statistics (how many users earned it)

**Response:**

```json
{
  "success": true,
  "data": {
    "badge_id": "uuid",
    "badge_name": "Early Adopter",
    "total_awarded": 150,
    "rarity": "epic",
    "percentage_of_users": 3.5
  }
}
```

---

## 🔗 Service Dependencies

### **Outbound Calls**

#### **user-service**

- **When:** Getting user details for badge award notification
- **Purpose:** Enrich badge award event

#### **notification-service**

- **Communication:** NATS events
- **Purpose:** Notify user when badge is earned

---

### **Inbound Calls**

#### **Frontend**

- **Endpoints:** All badge endpoints
- **Purpose:** Display badges, track progress

#### **Internal Services** (via NATS)

- **user-service:** Follower count updates → update badge progress
- **invitation-service:** Invitation used → update badge progress
- **community-service:** Community created → update badge progress

---

## 📡 NATS Events

### **Published Events**

```typescript
// Badge awarded
{
  event: "badge.awarded",
  user_id: "uuid",
  badge_id: "uuid",
  badge_name: "Early Adopter",
  badge_icon: "🌟",
  awarded_by: "uuid",  // null = auto-awarded
  timestamp: "ISO8601"
}
```

### **Subscribed Events**

```typescript
// User followed (update Social Butterfly progress)
{
  event: "user.followed",
  followed_id: "uuid"
}
// Action: Increment follower_count for "Social Butterfly" badge

// Invitation used (update Invitation Champion progress)
{
  event: "invitation.used",
  invited_by: "uuid"
}
// Action: Increment invitation count

// Community created (update Community Builder progress)
{
  event: "community.created",
  created_by: "uuid"
}
// Action: Increment community count
```

---

## 🔮 Holochain Migration

### **DNA Design: badges.happ**

#### **Entry Types**

**1. Badge** (Public DHT)

```rust
#[hdk_entry_helper]
struct Badge {
    name: String,
    slug: String,
    description: String,
    icon: String,
    criteria_type: CriteriaType,
    criteria_value: Option<u32>,
    rarity: BadgeRarity,
}
```

**2. UserBadge** (Link)

```rust
// Link: User → Badge (with timestamp)
create_link(
    agent_pub_key,
    badge_hash,
    LinkTag::from(timestamp.to_string()),
    LinkType::UserBadge
)?;
```

#### **Validation**

```rust
validate_create_link_user_badge(link: Link) {
    - Badge must exist
    - User hasn't already earned this badge
    - If auto-award, verify criteria met
}
```

---

## ✅ Implementation Status

### **Completed**

- ✅ Database schema designed
- ✅ Service scaffolded

### **Pending** (Phase 2)

- ⏳ All 6 endpoints
- ⏳ Badge definitions (seed data)
- ⏳ Auto-award logic
- ⏳ Progress tracking
- ⏳ NATS integration
- ⏳ Tests

---

## 🎖️ Initial Badges

### **Seed Data**

```sql
INSERT INTO badges (name, slug, description, icon, criteria_type, criteria_value, rarity) VALUES
('Early Adopter', 'early-adopter', 'Joined during alpha', '🌟', 'manual', NULL, 'epic'),
('First Steps', 'first-steps', 'Complete your profile', '👣', 'manual', NULL, 'common'),
('Social Butterfly', 'social-butterfly', 'Have 10 followers', '🦋', 'follower_count', 10, 'common'),
('Popular', 'popular', 'Have 100 followers', '⭐', 'follower_count', 100, 'rare'),
('Invitation Champion', 'invitation-champion', '5 users joined via your invitation', '🎫', 'invitation_count', 5, 'rare'),
('Community Builder', 'community-builder', 'Created 3 communities', '🏘️', 'community_count', 3, 'epic');
```

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Phase 2 (Gamification layer)  
**Dependencies:** user-service, notification-service
