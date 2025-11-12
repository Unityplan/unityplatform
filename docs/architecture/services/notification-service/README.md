# notification-service

**Port:** 8005  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Scaffolded (0/7 endpoints)  
**Bounded Context:** Notifications & User Preferences

---

## 📋 Overview

The notification-service manages all user notifications across the platform, including in-app notifications, email notifications, and user notification preferences.

### **Responsibilities**

- ⏳ Create and store notifications
- ⏳ Mark notifications as read/unread
- ⏳ Delete notifications
- ⏳ Manage user notification preferences
- ⏳ Subscribe to platform events and create notifications
- ⏳ Send email notifications (via SMTP or external service)
- ⏳ Push notifications (future: WebPush, mobile)

### **Not Responsible For**

- ❌ Email template rendering (uses simple templates)
- ❌ SMS notifications (not planned for MVP)
- ❌ Push notification infrastructure (future phase)
- ❌ Real-time websocket delivery (frontend polls or uses SSE)

---

## 🗄️ Database Schema

### **Tables Owned by notification-service**

#### **1. notifications**

```sql
CREATE TABLE territory_{code}.notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Recipient
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Content
    type VARCHAR(50) NOT NULL,  -- follower, message, badge, invitation_used, etc.
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    
    -- Metadata
    data JSONB DEFAULT '{}'::jsonb,  -- Type-specific data (user_id, badge_id, etc.)
    link VARCHAR(500),  -- Deep link to relevant page
    
    -- Status
    is_read BOOLEAN DEFAULT false,
    read_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON notifications(user_id);
CREATE INDEX idx_notifications_unread ON notifications(user_id, is_read) WHERE is_read = false;
CREATE INDEX idx_notifications_type ON notifications(type);
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);
```

**Purpose:** Store all user notifications  
**Holochain Entry Type:** `Notification` (private - recipient's chain only)  
**Retention:** Auto-delete after 90 days (configurable)

#### **2. users_notification_settings**

```sql
CREATE TABLE territory_{code}.users_notification_settings (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    
    -- Email Notifications
    email_on_follower BOOLEAN DEFAULT true,
    email_on_message BOOLEAN DEFAULT true,
    email_on_badge BOOLEAN DEFAULT true,
    email_on_invitation_used BOOLEAN DEFAULT true,
    email_on_community_invite BOOLEAN DEFAULT true,
    
    -- In-App Notifications
    notify_on_follower BOOLEAN DEFAULT true,
    notify_on_message BOOLEAN DEFAULT true,
    notify_on_badge BOOLEAN DEFAULT true,
    notify_on_invitation_used BOOLEAN DEFAULT true,
    notify_on_community_invite BOOLEAN DEFAULT true,
    
    -- Digest Settings
    email_digest_frequency VARCHAR(20) DEFAULT 'daily',  -- never/daily/weekly
    email_digest_time TIME DEFAULT '09:00:00',  -- Time to send digest
    
    -- Global Controls
    email_enabled BOOLEAN DEFAULT true,
    notification_enabled BOOLEAN DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT valid_digest_frequency CHECK (
        email_digest_frequency IN ('never', 'daily', 'weekly')
    )
);

CREATE INDEX idx_notification_settings_digest ON users_notification_settings(email_digest_frequency)
    WHERE email_digest_frequency != 'never';
```

**Purpose:** User notification preferences  
**Holochain Entry Type:** `NotificationSettings` (private)  
**Default:** All notifications enabled

#### **3. notification_templates**

```sql
CREATE TABLE territory_{code}.notification_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Template Identity
    type VARCHAR(50) UNIQUE NOT NULL,  -- follower, message, badge, etc.
    language VARCHAR(10) DEFAULT 'en',  -- ISO 639-1
    
    -- Email Template
    email_subject VARCHAR(255) NOT NULL,
    email_body TEXT NOT NULL,  -- Plain text or simple HTML
    
    -- In-App Template
    title_template VARCHAR(255) NOT NULL,  -- "{{username}} started following you"
    message_template TEXT NOT NULL,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(type, language)
);

CREATE INDEX idx_notification_templates_type ON notification_templates(type);
CREATE INDEX idx_notification_templates_language ON notification_templates(language);
```

**Purpose:** Notification templates for different event types  
**Example Types:** follower, message, badge, invitation_used, community_invite  
**Variables:** `{{username}}`, `{{badge_name}}`, `{{link}}`

---

## 🔌 API Endpoints

### **1. List Notifications**

#### **GET /v1/notifications**

Get user's notifications (paginated, newest first)

**Query Parameters:**

- `unread_only` - Boolean (default: false)
- `type` - Filter by notification type
- `page` - Page number (default: 1)
- `limit` - Results per page (default: 20, max: 100)

**Response:**

```json
{
  "success": true,
  "data": {
    "notifications": [
      {
        "id": "uuid",
        "type": "follower",
        "title": "New Follower",
        "message": "alice started following you",
        "data": {
          "user_id": "uuid",
          "username": "alice"
        },
        "link": "/users/alice",
        "is_read": false,
        "created_at": "2025-11-12T10:00:00Z"
      }
    ],
    "unread_count": 5,
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 42
    }
  }
}
```

---

### **2. Get Unread Count**

#### **GET /v1/notifications/unread/count**

Get count of unread notifications (lightweight endpoint for badges)

**Response:**

```json
{
  "success": true,
  "data": {
    "count": 5
  }
}
```

---

### **3. Mark as Read**

#### **PATCH /v1/notifications/{id}/read**

Mark a notification as read

**Response:**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "is_read": true,
    "read_at": "2025-11-12T10:05:00Z"
  }
}
```

#### **PATCH /v1/notifications/read-all**

Mark all notifications as read

**Response:**

```json
{
  "success": true,
  "data": {
    "updated_count": 5
  }
}
```

---

### **4. Delete Notification**

#### **DELETE /v1/notifications/{id}**

Delete a notification

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

### **5. Get Notification Settings**

#### **GET /v1/notifications/settings**

Get user's notification preferences

**Response:**

```json
{
  "success": true,
  "data": {
    "email": {
      "enabled": true,
      "on_follower": true,
      "on_message": true,
      "on_badge": true,
      "on_invitation_used": true,
      "on_community_invite": true
    },
    "in_app": {
      "enabled": true,
      "on_follower": true,
      "on_message": true,
      "on_badge": true,
      "on_invitation_used": true,
      "on_community_invite": true
    },
    "digest": {
      "frequency": "daily",
      "time": "09:00:00"
    }
  }
}
```

---

### **6. Update Notification Settings**

#### **PATCH /v1/notifications/settings**

Update notification preferences (partial update)

**Request:**

```json
{
  "email": {
    "on_follower": false,
    "on_message": false
  },
  "digest": {
    "frequency": "weekly"
  }
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "updated": true
  }
}
```

---

### **7. Send Notification (Internal)**

#### **POST /v1/notifications/send**

Create and send a notification (internal API, not exposed to frontend)

**Request:**

```json
{
  "user_id": "uuid",
  "type": "follower",
  "data": {
    "follower_id": "uuid",
    "username": "alice"
  }
}
```

**Actions:**

1. Check user's notification settings
2. If `notify_on_follower = true`, create in-app notification
3. If `email_on_follower = true`, send email
4. Render template with data
5. Store notification in database

**Response:**

```json
{
  "success": true,
  "data": {
    "notification_id": "uuid",
    "email_sent": true
  }
}
```

---

## 🔗 Service Dependencies

### **Outbound Calls (Services notification-service depends on)**

#### **settings-service**

- **When:** Sending email notification
- **Endpoint:** `GET /v1/settings/{user_id}/language`
- **Purpose:** Get user's preferred language for notification template
- **Fallback:** Use English template if service unavailable

#### **Email Service (SMTP or external API)**

- **When:** Sending email notifications
- **Provider:** SMTP (self-hosted) or SendGrid/Mailgun (future)
- **Purpose:** Deliver email notifications

---

### **Inbound Calls (Services that call notification-service)**

#### **Frontend**

- **Endpoints:** All notification endpoints (list, count, mark read, settings)
- **Purpose:** Display notifications, manage preferences

#### **Internal Services (via NATS events)**

- **Communication:** Event-driven (no direct API calls)
- **Services:** All services publish events that trigger notifications

---

## 📡 NATS Events

### **Subscribed Events (triggers notification creation)**

```typescript
// User followed
{
  event: "user.followed",
  follower_id: "uuid",
  followed_id: "uuid",
  username: "alice"
}
// → Notification to followed_id: "alice started following you"

// Badge awarded
{
  event: "badge.awarded",
  user_id: "uuid",
  badge_id: "uuid",
  badge_name: "Early Adopter",
  badge_icon: "🌟"
}
// → Notification: "You earned the Early Adopter badge!"

// Invitation used
{
  event: "invitation.used",
  invitation_id: "uuid",
  invited_by: "uuid",
  new_user_id: "uuid",
  new_username: "bob"
}
// → Notification to invited_by: "bob joined using your invitation"

// Message received (future)
{
  event: "message.received",
  recipient_id: "uuid",
  sender_id: "uuid",
  sender_username: "charlie",
  message_preview: "Hey, check this out..."
}
// → Notification: "New message from charlie"

// Community invitation (future)
{
  event: "community.invited",
  user_id: "uuid",
  community_id: "uuid",
  community_name: "Rust Developers",
  invited_by: "uuid"
}
// → Notification: "You've been invited to join Rust Developers"

// User registered (create default settings)
{
  event: "user.registered",
  user_id: "uuid",
  username: "alice"
}
// → Action: Create default notification settings
```

### **Published Events**

```typescript
// Notification sent
{
  event: "notification.sent",
  notification_id: "uuid",
  user_id: "uuid",
  type: "follower",
  email_sent: true,
  timestamp: "ISO8601"
}
```

---

## 🔮 Holochain Migration

### **DNA Design: notifications.happ**

#### **Entry Types**

**1. Notification** (Private - recipient's chain only)

```rust
#[hdk_entry_helper]
struct Notification {
    notification_type: NotificationType,
    title: String,
    message: String,
    data: BTreeMap<String, String>,
    link: Option<String>,
    created_at: Timestamp,
}

#[derive(Serialize, Deserialize, Debug)]
enum NotificationType {
    Follower,
    Message,
    Badge,
    InvitationUsed,
    CommunityInvite,
}
```

**2. NotificationSettings** (Private)

```rust
#[hdk_entry_helper]
struct NotificationSettings {
    email_preferences: EmailPreferences,
    in_app_preferences: InAppPreferences,
    digest_settings: DigestSettings,
}
```

#### **Key Principle: User Sovereignty**

Notifications in Holochain are stored ONLY in the recipient's private source chain. No one else can see or access them. The sender cannot track if notifications were read.

#### **Cross-Device Sync**

```
Device A creates notification → commits to user's chain
Device B reads from same agent's chain → sees notification
Automatic synchronization via agent's source chain
```

#### **Migration Strategy**

**Phase 1:** PostgreSQL storage

- Current implementation with PostgreSQL
- Notifications stored in database

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

### **Pending** (Week 2 of migration plan)

- ⏳ Implement GET /v1/notifications
- ⏳ Implement GET /v1/notifications/unread/count
- ⏳ Implement PATCH /v1/notifications/{id}/read
- ⏳ Implement PATCH /v1/notifications/read-all
- ⏳ Implement DELETE /v1/notifications/{id}
- ⏳ Implement GET /v1/notifications/settings
- ⏳ Implement PATCH /v1/notifications/settings
- ⏳ Implement POST /v1/notifications/send (internal)
- ⏳ NATS event subscribers (user.followed, badge.awarded, etc.)
- ⏳ Email sending integration (SMTP)
- ⏳ Notification templates (seed database)
- ⏳ Auto-create default settings on user registration
- ⏳ Frontend integration
- ⏳ Tests (unit + integration)

---

## 🧪 Testing

### **Test Plan**

```bash
cd services/notification-service
cargo test

# Test cases to implement:
# - Create notification
# - List notifications (with pagination)
# - Get unread count
# - Mark notification as read
# - Mark all as read
# - Delete notification
# - Get notification settings
# - Update notification settings
# - Send email notification
# - NATS event subscribers (mock events)
# - Template rendering with variables
# - Default settings creation for new user
# - Notification auto-deletion after 90 days
```

---

## 📧 Email Integration

### **SMTP Configuration**

```rust
// config.toml
[email]
enabled = true
provider = "smtp"  # smtp/sendgrid/mailgun
smtp_host = "smtp.gmail.com"
smtp_port = 587
smtp_username = "notifications@unityplan.org"
smtp_password = "secure_password"
from_email = "notifications@unityplan.org"
from_name = "UnityPlan"
```

### **Email Template Example**

```html
Subject: {{username}} started following you

Hi there,

{{username}} started following you on UnityPlan!

View their profile: {{link}}

---
UnityPlan - User Sovereignty First
Manage your notification settings: {{settings_link}}
```

### **Variables Available**

- `{{username}}` - Actor's username
- `{{link}}` - Deep link to relevant page
- `{{badge_name}}` - Badge name (for badge notifications)
- `{{community_name}}` - Community name
- `{{settings_link}}` - Link to notification settings

---

## 🔔 Notification Types

### **Defined Types**

```rust
pub enum NotificationType {
    Follower,           // Someone followed you
    Message,            // New message (future)
    Badge,              // Badge awarded
    InvitationUsed,     // Your invitation was used
    CommunityInvite,    // Invited to community (future)
    CommunityPost,      // New post in community (future)
    CourseUpdate,       // Course content updated (future)
    ForumReply,         // Forum reply (future)
}
```

### **Adding New Types**

1. Add to `NotificationType` enum
2. Create template in `notification_templates` table
3. Subscribe to relevant NATS event
4. Update notification settings table

---

## 🗑️ Notification Retention

### **Auto-Cleanup**

```sql
-- Cron job (runs daily)
DELETE FROM notifications
WHERE created_at < NOW() - INTERVAL '90 days';
```

**Retention:** 90 days (configurable)  
**User Action:** Users can manually delete notifications anytime

---

## 📊 Metrics & Monitoring

### **Key Metrics**

- Notifications created per day (by type)
- Email delivery rate
- Notification read rate (% of notifications read)
- Average time to read
- Unsubscribe rate (users disabling notifications)

### **Alerts**

- Email delivery failures (SMTP errors)
- High unread rate (users ignoring notifications?)
- Spike in notification creation (system issue?)

---

## 🚀 Implementation Priority

**Week 2 of Migration Plan:**

1. **Day 1-2:** Implement core endpoints
   - GET /v1/notifications
   - GET /v1/notifications/unread/count
   - PATCH /v1/notifications/{id}/read
   - DELETE /v1/notifications/{id}

2. **Day 3:** Settings endpoints
   - GET /v1/notifications/settings
   - PATCH /v1/notifications/settings
   - Default settings creation

3. **Day 4:** NATS integration
   - Subscribe to user.followed, badge.awarded, invitation.used
   - POST /v1/notifications/send (internal)
   - Template rendering

4. **Day 5:** Email & testing
   - SMTP integration
   - Email templates
   - Frontend integration
   - End-to-end tests

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Ready for Implementation (Week 2)  
**Migration Plan:** [Service Separation Migration](../../guides/development/service-separation-migration.md)
