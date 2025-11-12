# settings-service

**Port:** 8003  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Scaffolded (0/5 endpoints)  
**Bounded Context:** User Preferences & Settings

---

## 📋 Overview

The settings-service manages all user preferences and settings, including appearance, language, privacy, and cross-device synchronization.

### **Responsibilities**

- ⏳ Appearance settings (theme, colors, layout)
- ⏳ Language settings (locale, timezone, translation preferences)
- ⏳ Privacy settings (profile visibility, data sharing)
- ⏳ Settings synchronization across devices
- ⏳ Default settings initialization for new users

### **Not Responsible For**

- ❌ Notification preferences (handled by notification-service)
- ❌ User profile data (handled by user-service)
- ❌ Account settings (password, email - handled by auth-service)

---

## 🔐 Authentication

This service uses **JWT-based authentication** via shared middleware from `shared-lib`.

### **Validation Strategy**

- **All requests:** JWT signature validation only (~0.01ms, no database query)
- **No critical operations:** Settings updates don't require additional database checks

### **Middleware**

```rust
use shared_lib::middleware::jwt_auth_middleware;

HttpServer::new(|| {
    App::new()
        .wrap(jwt_auth_middleware)  // All routes protected
        .service(get_settings)
        .service(update_settings)
})
```

**See [shared-lib/AUTHENTICATION.md](../shared-lib/AUTHENTICATION.md) for complete authentication architecture.**

---

## 🗄️ Database Schema

### **Tables Owned by settings-service**

#### **1. users_settings**

```sql
CREATE TABLE territory_{code}.users_settings (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    
    -- Appearance
    theme_mode VARCHAR(20) DEFAULT 'system',  -- light/dark/system
    color_scheme VARCHAR(50) DEFAULT 'forest-green',  -- forest-green/ocean-blue/royal-purple/custom
    reduced_motion BOOLEAN DEFAULT false,
    wide_content_view BOOLEAN DEFAULT false,
    compact_mode BOOLEAN DEFAULT false,
    
    -- Language
    preferred_language VARCHAR(10) DEFAULT 'en',  -- ISO 639-1
    timezone VARCHAR(100) DEFAULT 'UTC',  -- IANA timezone
    
    -- Translation
    auto_translate BOOLEAN DEFAULT false,
    translation_provider VARCHAR(50) DEFAULT 'libre-translate',  -- libre-translate/deepl/google/microsoft
    contribute_translations BOOLEAN DEFAULT false,
    fallback_to_english BOOLEAN DEFAULT true,
    
    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT valid_theme_mode CHECK (theme_mode IN ('light', 'dark', 'system')),
    CONSTRAINT valid_color_scheme CHECK (
        color_scheme IN ('forest-green', 'ocean-blue', 'royal-purple', 'custom')
    ),
    CONSTRAINT valid_translation_provider CHECK (
        translation_provider IN ('libre-translate', 'deepl', 'google', 'microsoft')
    )
);

CREATE INDEX idx_users_settings_theme ON users_settings(theme_mode);
CREATE INDEX idx_users_settings_language ON users_settings(preferred_language);
```

**Purpose:** User appearance and language preferences  
**Holochain Entry Type:** `UserSettings` (private chain - user sovereignty)  
**Auto-created:** When user registers (default values)

#### **2. users_privacy_settings**

```sql
CREATE TABLE territory_{code}.users_privacy_settings (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    
    -- Profile Visibility
    profile_visibility VARCHAR(20) DEFAULT 'public',  -- public/followers/private
    show_email BOOLEAN DEFAULT false,
    show_location BOOLEAN DEFAULT false,
    show_connections BOOLEAN DEFAULT true,
    show_activity BOOLEAN DEFAULT true,
    
    -- Interaction Controls
    allow_messages VARCHAR(20) DEFAULT 'everyone',  -- everyone/followers/nobody
    allow_followers VARCHAR(20) DEFAULT 'everyone',  -- everyone/approved/nobody
    require_follow_approval BOOLEAN DEFAULT false,
    
    -- Data Sharing
    allow_indexing BOOLEAN DEFAULT true,
    share_analytics BOOLEAN DEFAULT true,
    
    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT valid_profile_visibility CHECK (
        profile_visibility IN ('public', 'followers', 'private')
    ),
    CONSTRAINT valid_message_permission CHECK (
        allow_messages IN ('everyone', 'followers', 'nobody')
    ),
    CONSTRAINT valid_follower_permission CHECK (
        allow_followers IN ('everyone', 'approved', 'nobody')
    )
);

CREATE INDEX idx_users_privacy_settings_visibility ON users_privacy_settings(profile_visibility);
```

**Purpose:** User privacy and visibility controls  
**Holochain Entry Type:** `PrivacySettings` (private chain)  
**Default:** Public profile, open to followers

---

## 🔌 API Endpoints

### **Bulk Settings**

#### **GET /v1/settings/{user_id}**

Get all user settings (appearance + language + privacy)

**Response:**

```json
{
  "success": true,
  "data": {
    "appearance": {
      "theme_mode": "dark",
      "color_scheme": "forest-green",
      "reduced_motion": false,
      "wide_content_view": true,
      "compact_mode": false
    },
    "language": {
      "preferred_language": "da",
      "timezone": "Europe/Copenhagen",
      "auto_translate": true,
      "translation_provider": "libre-translate",
      "contribute_translations": true,
      "fallback_to_english": true
    },
    "privacy": {
      "profile_visibility": "public",
      "show_email": false,
      "show_location": true,
      "allow_messages": "followers",
      "allow_followers": "everyone"
    }
  }
}
```

#### **PUT /v1/settings/{user_id}**

Update settings (bulk update - partial changes allowed)

**Request:**

```json
{
  "appearance": {
    "theme_mode": "dark",
    "color_scheme": "ocean-blue"
  },
  "language": {
    "preferred_language": "da"
  }
}
```

---

### **Appearance Settings**

#### **GET /v1/settings/{user_id}/appearance**

Get appearance settings only

**Response:**

```json
{
  "success": true,
  "data": {
    "theme_mode": "dark",
    "color_scheme": "forest-green",
    "reduced_motion": false,
    "wide_content_view": true,
    "compact_mode": false,
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

#### **PATCH /v1/settings/{user_id}/appearance**

Update appearance settings (partial update)

**Request:**

```json
{
  "theme_mode": "light",
  "compact_mode": true
}
```

---

### **Language Settings**

#### **GET /v1/settings/{user_id}/language**

Get language and translation settings

**Response:**

```json
{
  "success": true,
  "data": {
    "preferred_language": "da",
    "timezone": "Europe/Copenhagen",
    "auto_translate": true,
    "translation_provider": "libre-translate",
    "contribute_translations": true,
    "fallback_to_english": true,
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

#### **PATCH /v1/settings/{user_id}/language**

Update language settings

**Request:**

```json
{
  "preferred_language": "no",
  "timezone": "Europe/Oslo",
  "auto_translate": false
}
```

---

### **Privacy Settings**

#### **GET /v1/settings/{user_id}/privacy**

Get privacy settings

**Response:**

```json
{
  "success": true,
  "data": {
    "profile_visibility": "followers",
    "show_email": false,
    "show_location": false,
    "show_connections": true,
    "show_activity": true,
    "allow_messages": "followers",
    "allow_followers": "everyone",
    "require_follow_approval": false,
    "allow_indexing": true,
    "share_analytics": false,
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

#### **PATCH /v1/settings/{user_id}/privacy**

Update privacy settings

**Request:**

```json
{
  "profile_visibility": "private",
  "allow_messages": "nobody",
  "share_analytics": false
}
```

---

## 🔗 Service Dependencies

### **Outbound Calls (Services settings-service depends on)**

None - settings-service is a **leaf service**

---

### **Inbound Calls (Services that call settings-service)**

#### **user-service**

- **When:** GDPR data export
- **Endpoint:** `GET /v1/settings/{user_id}`
- **Purpose:** Include settings in user data export
- **Fallback:** Export without settings if service unavailable

#### **Frontend**

- **When:** Settings page, user preferences
- **Endpoints:** All settings endpoints
- **Purpose:** Display and update user preferences

#### **notification-service**

- **When:** Determining notification language
- **Endpoint:** `GET /v1/settings/{user_id}/language`
- **Purpose:** Send notifications in user's preferred language
- **Fallback:** Use English if service unavailable

---

## 📡 NATS Events

### **Published Events**

```typescript
// Settings updated
{
  event: "settings.updated",
  user_id: "uuid",
  category: "appearance" | "language" | "privacy",
  changes: {
    "theme_mode": "dark",
    "color_scheme": "ocean-blue"
  },
  timestamp: "ISO8601"
}

// Privacy settings changed (important for other services)
{
  event: "privacy.updated",
  user_id: "uuid",
  profile_visibility: "private",
  allow_messages: "nobody",
  timestamp: "ISO8601"
}
```

### **Subscribed Events**

```typescript
// User registered (create default settings)
{
  event: "user.registered",
  user_id: "uuid",
  username: "alice",
  territory_code: "dk"
}
// Action: Create default settings for new user
```

---

## 🔮 Holochain Migration

### **DNA Design: settings.happ**

#### **Entry Types**

**1. UserSettings** (Private - user's source chain only)

```rust
#[hdk_entry_helper]
struct UserSettings {
    theme_mode: ThemeMode,
    color_scheme: ColorScheme,
    reduced_motion: bool,
    wide_content_view: bool,
    compact_mode: bool,
    preferred_language: String,
    timezone: String,
    auto_translate: bool,
    translation_provider: TranslationProvider,
    contribute_translations: bool,
    fallback_to_english: bool,
}
```

**2. PrivacySettings** (Private)

```rust
#[hdk_entry_helper]
struct PrivacySettings {
    profile_visibility: ProfileVisibility,
    show_email: bool,
    show_location: bool,
    show_connections: bool,
    show_activity: bool,
    allow_messages: MessagePermission,
    allow_followers: FollowerPermission,
    require_follow_approval: bool,
    allow_indexing: bool,
    share_analytics: bool,
}
```

#### **Key Principle: User Sovereignty**

Settings in Holochain are stored ONLY in the user's private source chain. No one else can see or access them. This aligns perfectly with user data sovereignty.

#### **Cross-Device Sync**

```rust
// Settings sync via agent's source chain
// Device A updates settings → commits to chain
// Device B reads from same agent's chain → gets latest settings
// Automatic synchronization without central server
```

#### **Migration Strategy**

**Phase 1:** PostgreSQL storage, prepare for Holochain

- Current implementation with PostgreSQL
- Settings stored per user

**Phase 2:** Dual storage (PostgreSQL + Holochain)

- Write to both PostgreSQL and Holochain
- Read from PostgreSQL (fast queries)
- Holochain as source of truth

**Phase 3:** Holochain native

- Read/write directly from Holochain
- PostgreSQL removed
- Pure agent sovereignty

---

## ✅ Implementation Status

### **Completed**

- ✅ Database schema designed
- ✅ Models defined (Rust structs)
- ✅ Service scaffolded

### **Pending** (Week 1 of migration plan)

- ⏳ Implement GET /v1/settings/{user_id}
- ⏳ Implement PUT /v1/settings/{user_id}
- ⏳ Implement PATCH /v1/settings/{user_id}/appearance
- ⏳ Implement PATCH /v1/settings/{user_id}/language
- ⏳ Implement PATCH /v1/settings/{user_id}/privacy
- ⏳ Auto-create defaults on user registration (NATS subscriber)
- ⏳ Update user-service GDPR export to call settings-service
- ⏳ Frontend integration
- ⏳ Tests (unit + integration)
- ⏳ OpenAPI documentation

---

## 🧪 Testing

### **Test Plan**

```bash
cd services/settings-service
cargo test

# Test cases to implement:
# - Default settings creation for new user
# - Get all settings
# - Update appearance settings
# - Update language settings
# - Update privacy settings
# - Partial updates (PATCH)
# - Validation (invalid values rejected)
# - Multi-device sync (last-write-wins)
# - GDPR export integration
```

---

## 📝 Data Models

### **Rust Enums**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "kebab-case")]
pub enum ColorScheme {
    ForestGreen,
    OceanBlue,
    RoyalPurple,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ProfileVisibility {
    Public,     // Anyone can see profile
    Followers,  // Only followers can see details
    Private,    // Only user can see
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum MessagePermission {
    Everyone,   // Anyone can message
    Followers,  // Only followers can message
    Nobody,     // No messages allowed
}
```

---

## 🚀 Implementation Priority

**Week 1 of Migration Plan:**

1. **Day 1-2:** Implement core handlers
   - GET /v1/settings/{user_id}
   - PUT /v1/settings/{user_id}
   - Database queries

2. **Day 3:** Implement granular endpoints
   - PATCH /v1/settings/{user_id}/appearance
   - PATCH /v1/settings/{user_id}/language
   - PATCH /v1/settings/{user_id}/privacy

3. **Day 4:** NATS integration
   - Subscribe to user.registered
   - Auto-create default settings
   - Publish settings.updated events

4. **Day 5:** Integration & testing
   - Update user-service GDPR export
   - Frontend integration
   - End-to-end tests

---

## 📊 Default Values

When a new user registers, create settings with these defaults:

```json
{
  "appearance": {
    "theme_mode": "system",
    "color_scheme": "forest-green",
    "reduced_motion": false,
    "wide_content_view": false,
    "compact_mode": false
  },
  "language": {
    "preferred_language": "en",
    "timezone": "UTC",
    "auto_translate": false,
    "translation_provider": "libre-translate",
    "contribute_translations": false,
    "fallback_to_english": true
  },
  "privacy": {
    "profile_visibility": "public",
    "show_email": false,
    "show_location": false,
    "show_connections": true,
    "show_activity": true,
    "allow_messages": "everyone",
    "allow_followers": "everyone",
    "require_follow_approval": false,
    "allow_indexing": true,
    "share_analytics": true
  }
}
```

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Ready for Implementation (Week 1)  
**Migration Plan:** [Service Separation Migration](../../guides/development/service-separation-migration.md)
