# notification-service Database Schema

**Service:** notification-service  
**Port:** 8005  
**Database:** Territory schema only

---

## Table

### territory_{code}.notifications

**Status:** ✅ Exists in 20251111000001_mvp_core_schema.sql

```sql
CREATE TABLE territory_{code}.notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    notification_type VARCHAR(50) NOT NULL,
    
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    
    link VARCHAR(500),
    
    data JSONB,
    
    is_read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON territory_{code}.notifications(user_id, created_at DESC);
CREATE INDEX idx_notifications_unread ON territory_{code}.notifications(user_id, is_read) WHERE is_read = false;
CREATE INDEX idx_notifications_type ON territory_{code}.notifications(notification_type);
```

## Notification Types

- `follower` - New follower
- `message` - New message
- `badge` - Badge awarded
- `invitation_used` - Invitation was used
- `community_invite` - Invited to community
- `event_reminder` - Event starting soon
- `course_update` - Course content updated

## Data Sovereignty

Notifications are personal - stored in user's territory pod.

## NATS Events

**Subscribes:**
- `user.followed` → Create follower notification
- `badge.awarded` → Create badge notification
- `invitation.used` → Create invitation used notification
- `community.invited` → Create community invite notification

**Publishes:** None (terminal service)

---

**Last Updated:** November 12, 2025
