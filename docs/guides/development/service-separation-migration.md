# Service Separation Migration Plan

**Date:** November 12, 2025  
**Status:** Active Migration Plan  
**Goal:** Properly separate consolidated services into true microservices architecture

---

## 🎯 Migration Overview

### **Current State (Consolidated)**

```
auth-service (8001)
  ✅ Authentication
  ⚠️  Invitation logic (should be in invitation-service)

user-service (8002)
  ✅ User profiles
  ✅ Profile links
  ✅ Language proficiency
  ✅ User connections
  ✅ GDPR compliance
  ⚠️  User settings (should be in settings-service)
  ⚠️  Notification settings (should be in notification-service)
```

### **Target State (Microservices)**

```
auth-service (8001)
  ✅ Authentication only

user-service (8002)
  ✅ Profiles, links, languages, connections, GDPR

settings-service (8003)
  ✅ User settings, privacy settings

invitation-service (8004)
  ✅ Invitation management

notification-service (8005)
  ✅ Notifications, notification settings
```

---

## 📋 Migration Phases

### **Phase 1: settings-service (Week 1)**

**Goal:** Move all settings logic from user-service to settings-service

#### **Step 1.1: Database Migration**

Create migration to transfer ownership documentation:

```sql
-- Migration: 20251113000001_document_settings_service_ownership.sql
-- Note: Tables already exist, just documenting ownership transfer

COMMENT ON TABLE territory_dk.users_settings IS 
  'Owned by: settings-service (Port 8003). User appearance and language preferences.';

COMMENT ON TABLE territory_dk.users_privacy_settings IS 
  'Owned by: settings-service (Port 8003). Profile visibility and privacy controls.';
```

#### **Step 1.2: Implement settings-service Endpoints**

```rust
// services/settings-service/src/handlers/settings.rs

// User Settings
GET    /v1/settings/{user_id}
PUT    /v1/settings/{user_id}
PATCH  /v1/settings/{user_id}/appearance
PATCH  /v1/settings/{user_id}/language
PATCH  /v1/settings/{user_id}/privacy
```

**Files to create:**

- `services/settings-service/src/handlers/settings.rs`
- `services/settings-service/src/handlers/privacy.rs`
- `services/settings-service/src/models/settings.rs`

**Copy from user-service:**

- Models: `UserSettings`, `PrivacySettings`, update requests
- Validation logic
- Database queries

#### **Step 1.3: Update user-service**

**Remove settings endpoints:**

```rust
// services/user-service/src/main.rs
// DELETE these routes:
.route("/v1/users/{id}/settings", web::get().to(...))
.route("/v1/users/{id}/settings", web::put().to(...))
```

**Update GDPR export to call settings-service:**

```rust
// services/user-service/src/handlers/data_export.rs

async fn collect_user_data(user_id: Uuid, pool: &PgPool) -> Result<ExportedUserData> {
    // ... existing code ...
    
    // NEW: Call settings-service API instead of direct DB query
    let settings = reqwest::get(
        format!("http://settings-service:8003/v1/settings/{}", user_id)
    )
    .await?
    .json::<UserSettings>()
    .await
    .ok(); // Optional - don't fail export if settings unavailable
    
    Ok(ExportedUserData {
        // ... existing fields ...
        settings,
    })
}
```

#### **Step 1.4: Update Frontend**

```typescript
// frontend/src/api/settings.ts (NEW FILE)

export const settingsApi = {
  getSettings: (userId: string) =>
    apiClient.get<UserSettings>(`/v1/settings/${userId}`),
  
  updateSettings: (userId: string, data: Partial<UserSettings>) =>
    apiClient.put<UserSettings>(`/v1/settings/${userId}`, data),
  
  updateAppearance: (userId: string, data: Partial<AppearanceSettings>) =>
    apiClient.patch(`/v1/settings/${userId}/appearance`, data),
};

// frontend/src/pages/SettingsPage.tsx
// Update to use settingsApi instead of userApi
```

#### **Step 1.5: Docker Compose Update**

```yaml
# docker-compose.dev.yml

services:
  settings-service:
    build:
      context: ./services/settings-service
    ports:
      - "8003:8003"
    environment:
      - DATABASE_URL=postgresql://...
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8003
    depends_on:
      - postgres
      - nats
```

#### **Step 1.6: Testing**

```bash
# Test settings-service independently
cd services/settings-service
cargo test

# Test user-service still works (without settings endpoints)
cd services/user-service
cargo test

# Integration test: GDPR export calls settings-service
curl -X POST http://localhost:8002/v1/users/{id}/data/export
# Verify export includes settings from settings-service
```

**Checklist:**

- [ ] settings-service endpoints implemented
- [ ] Tests passing (settings-service)
- [ ] user-service updated to call settings-service API
- [ ] GDPR export includes settings
- [ ] Frontend updated
- [ ] Docker Compose configured
- [ ] Integration tests passing

---

### **Phase 2: notification-service (Week 2)**

**Goal:** Move notification settings from user-service, implement notification system

#### **Step 2.1: Database Migration**

```sql
-- Migration: 20251114000001_create_notifications_tables.sql

-- Move ownership of notification settings
COMMENT ON TABLE territory_dk.users_notification_settings IS 
  'Owned by: notification-service (Port 8005). User notification preferences.';

-- Create notifications table
CREATE TABLE territory_dk.notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    action_url TEXT,
    is_read BOOLEAN DEFAULT false,
    read_at TIMESTAMPTZ,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    
    CONSTRAINT valid_notification_type CHECK (
        type IN ('message', 'follower', 'community', 'mention', 'like', 'system')
    )
);

CREATE INDEX idx_notifications_user ON notifications(user_id, created_at DESC);
CREATE INDEX idx_notifications_unread ON notifications(user_id, is_read) WHERE is_read = false;

-- Create notification templates
CREATE TABLE territory_dk.notification_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_key VARCHAR(100) NOT NULL,
    language_code VARCHAR(10) NOT NULL,
    subject_template TEXT,
    body_template TEXT NOT NULL,
    notification_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT unique_template_per_language UNIQUE(template_key, language_code)
);
```

#### **Step 2.2: Implement notification-service**

```rust
// services/notification-service/src/handlers/notifications.rs

GET    /v1/notifications                    // List (paginated)
GET    /v1/notifications/unread-count       // Badge count
POST   /v1/notifications/{id}/read          // Mark read
POST   /v1/notifications/read-all           // Mark all read
DELETE /v1/notifications/{id}               // Delete

// Notification Settings
GET    /v1/notifications/settings/{user_id}
PUT    /v1/notifications/settings/{user_id}

// Internal API (called by other services)
POST   /v1/notifications/send               // Create notification
```

#### **Step 2.3: NATS Event Subscriptions**

```rust
// services/notification-service/src/events/subscriber.rs

async fn subscribe_to_events(nats: &NatsClient) {
    // Subscribe to user events
    nats.subscribe("user.followed").await;
    nats.subscribe("user.blocked").await;
    nats.subscribe("profile.updated").await;
    
    // Subscribe to community events (future)
    nats.subscribe("community.member.joined").await;
    nats.subscribe("community.post.created").await;
}

// When event received, create notification
async fn handle_user_followed(event: UserFollowedEvent) {
    // Get user preferences
    let settings = get_notification_settings(event.target_user_id).await?;
    
    if settings.inapp_followers {
        create_notification(Notification {
            user_id: event.target_user_id,
            type: "follower",
            title: "New Follower",
            message: format!("{} started following you", event.username),
            action_url: Some(format!("/profiles/{}", event.user_id)),
        }).await?;
    }
    
    if settings.email_followers {
        send_email_notification(...).await?;
    }
}
```

#### **Step 2.4: Update user-service to Publish Events**

```rust
// services/user-service/src/handlers/connections.rs

pub async fn follow_user(...) -> Result<HttpResponse, AppError> {
    // ... existing follow logic ...
    
    // NEW: Publish NATS event
    let event = UserFollowedEvent {
        event: "user.followed".to_string(),
        user_id: path.user_id,
        target_user_id: path.target_id,
        username: user.username,
        timestamp: Utc::now(),
    };
    
    nats_client.publish("user.followed", &event).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(connection)))
}
```

#### **Step 2.5: Remove Notification Settings from user-service**

```rust
// services/user-service/src/main.rs
// DELETE these routes:
.route("/v1/users/{id}/settings/notifications", web::get().to(...))
.route("/v1/users/{id}/settings/notifications", web::put().to(...))
```

**Update GDPR export:**

```rust
// Call notification-service API for notification settings
let notification_settings = reqwest::get(
    format!("http://notification-service:8005/v1/notifications/settings/{}", user_id)
)
.await?
.json()
.await.ok();
```

**Checklist:**

- [ ] notification-service endpoints implemented
- [ ] NATS event subscriptions working
- [ ] user-service publishes events
- [ ] Notification creation from events verified
- [ ] Frontend notification bell working
- [ ] Email notifications configured
- [ ] Integration tests passing

---

### **Phase 3: invitation-service (Week 2-3)**

**Goal:** Move invitation logic from auth-service to invitation-service

#### **Step 3.1: Implement invitation-service**

```rust
// services/invitation-service/src/handlers/invitations.rs

// Public API (no auth required - used by registration page)
GET    /v1/invitations/validate/{token}

// Authenticated API
POST   /v1/invitations                      // Create (with rate limiting)
GET    /v1/invitations                      // List my invitations
GET    /v1/invitations/{id}                 // Get details
DELETE /v1/invitations/{id}                 // Revoke
GET    /v1/invitations/{id}/uses            // Usage audit

// Internal API (called by auth-service)
POST   /v1/invitations/{token}/use          // Mark as used
```

**Rate Limiting:**

```rust
// Max 10 invitations per day per user
async fn create_invitation(...) -> Result<HttpResponse, AppError> {
    let today_count = sqlx::query!(
        "SELECT COUNT(*) FROM invitation_tokens 
         WHERE created_by = $1 
         AND created_at > NOW() - INTERVAL '24 hours'",
        user_id
    )
    .fetch_one(pool)
    .await?
    .count;
    
    if today_count >= 10 {
        return Err(AppError::RateLimitExceeded(
            "Maximum 10 invitations per day".to_string()
        ));
    }
    
    // ... create invitation ...
}
```

#### **Step 3.2: Create invitation_uses Table**

```sql
-- Migration: 20251115000001_create_invitation_uses.sql

CREATE TABLE territory_dk.invitation_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_id UUID NOT NULL REFERENCES invitation_tokens(id) ON DELETE CASCADE,
    used_by_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    used_at TIMESTAMPTZ DEFAULT NOW(),
    ip_address TEXT,
    user_agent TEXT,
    
    CONSTRAINT one_use_per_user_per_token UNIQUE(token_id, used_by_user_id)
);

CREATE INDEX idx_invitation_uses_token ON invitation_uses(token_id);
CREATE INDEX idx_invitation_uses_user ON invitation_uses(used_by_user_id);
```

#### **Step 3.3: Update auth-service**

**Remove invitation logic, call invitation-service API:**

```rust
// services/auth-service/src/handlers/register.rs

pub async fn register(...) -> Result<HttpResponse, AppError> {
    // Validate invitation token via invitation-service API
    let invitation = reqwest::get(
        format!("http://invitation-service:8004/v1/invitations/validate/{}", 
                req.invitation_token)
    )
    .await?
    .json::<InvitationToken>()
    .await
    .map_err(|_| AppError::InvalidInvitation)?;
    
    // Verify token is valid and not expired
    if !invitation.is_valid {
        return Err(AppError::InvalidInvitation);
    }
    
    // ... create user ...
    
    // Mark invitation as used
    reqwest::post(
        format!("http://invitation-service:8004/v1/invitations/{}/use", 
                req.invitation_token)
    )
    .json(&json!({
        "user_id": user.id,
        "ip_address": ip_address,
        "user_agent": user_agent
    }))
    .send()
    .await?;
    
    Ok(...)
}
```

**Checklist:**

- [ ] invitation-service endpoints implemented
- [ ] Rate limiting working (10/day)
- [ ] Invitation validation API working
- [ ] auth-service calls invitation-service
- [ ] Usage tracking in invitation_uses table
- [ ] Admin panel can create/revoke invitations
- [ ] Integration tests passing

---

### **Phase 4: Integration & Testing (Week 3)**

#### **Step 4.1: Service Communication Testing**

Test all inter-service API calls:

```bash
# Test 1: GDPR export calls settings-service and notification-service
curl -X POST http://localhost:8002/v1/users/{id}/data/export
# Verify export includes data from both services

# Test 2: User follows → notification created
curl -X POST http://localhost:8002/v1/users/{id}/connections/follow/{target_id}
# Check notification-service for new notification

# Test 3: Registration uses invitation-service
curl -X POST http://localhost:8001/api/v1/auth/register \
  -d '{"invitation_token": "...", "username": "test", "password": "..."}'
# Verify invitation marked as used
```

#### **Step 4.2: NATS Event Flow Testing**

```bash
# Subscribe to NATS events
nats sub "user.>"

# Trigger user action (follow)
curl -X POST http://localhost:8002/v1/users/{id}/connections/follow/{target_id}

# Verify event published:
# Subject: user.followed
# Data: {"event": "user.followed", "user_id": "...", ...}

# Check notification created in notification-service
curl http://localhost:8005/v1/notifications?user_id={target_id}
```

#### **Step 4.3: Multi-Pod Testing**

Test service independence for multi-pod deployment:

```bash
# Start Denmark pod
docker-compose -f docker-compose.pod.yml --env-file pods/denmark/.env up

# Start Norway pod
docker-compose -f docker-compose.pod.yml --env-file pods/norway/.env up

# Test cross-pod follow
# DK user follows NO user
curl -X POST http://dk.local:8002/v1/users/{dk_user}/connections/follow/{no_user}

# Verify:
# 1. Connection stored in territory_dk.users_connections
# 2. Event published to DK NATS
# 3. NO user gets notification (via global event bus - future)
```

#### **Step 4.4: Performance Testing**

Test latency of multi-service calls:

```bash
# Benchmark GDPR export (calls settings + notification services)
ab -n 100 -c 10 \
  -H "Authorization: Bearer $TOKEN" \
  http://localhost:8002/v1/users/{id}/data/export

# Measure latency:
# - Direct DB query: ~50ms
# - With 2 service calls: ~150ms (acceptable)
```

#### **Step 4.5: Failure Testing**

Test graceful degradation when services unavailable:

```bash
# Stop settings-service
docker stop settings-service

# Test GDPR export
curl -X POST http://localhost:8002/v1/users/{id}/data/export

# Expected: Export succeeds but without settings section
# Verify: Export includes profile, links, connections (not settings)
```

**Checklist:**

- [ ] All inter-service calls working
- [ ] NATS events publishing/subscribing
- [ ] Multi-pod deployment tested
- [ ] Performance acceptable (<200ms for multi-service calls)
- [ ] Graceful degradation when services down
- [ ] All services have health checks
- [ ] Monitoring dashboards updated

---

## 🔧 Infrastructure Updates

### **Traefik Routing**

```yaml
# docker/traefik/traefik.yml

http:
  routers:
    settings-service:
      rule: "PathPrefix(`/v1/settings`)"
      service: settings-service
      middlewares:
        - jwt-auth
    
    invitation-service:
      rule: "PathPrefix(`/v1/invitations`)"
      service: invitation-service
      # Note: /validate endpoint is public (no auth middleware)
    
    notification-service:
      rule: "PathPrefix(`/v1/notifications`)"
      service: notification-service
      middlewares:
        - jwt-auth
  
  services:
    settings-service:
      loadBalancer:
        servers:
          - url: "http://settings-service:8003"
    
    invitation-service:
      loadBalancer:
        servers:
          - url: "http://invitation-service:8004"
    
    notification-service:
      loadBalancer:
        servers:
          - url: "http://notification-service:8005"
```

### **NATS Configuration**

```yaml
# docker/nats/nats.conf

jetstream {
  store_dir: /data/jetstream
  max_memory_store: 1GB
  max_file_store: 10GB
}

# Streams for event persistence
stream {
  name: USER_EVENTS
  subjects: ["user.>"]
  retention: limits
  max_age: 7d
  storage: file
}

stream {
  name: NOTIFICATION_EVENTS
  subjects: ["notification.>"]
  retention: limits
  max_age: 30d
  storage: file
}
```

### **Monitoring**

```yaml
# docker/prometheus/prometheus.yml

scrape_configs:
  - job_name: 'settings-service'
    static_configs:
      - targets: ['settings-service:8003']
  
  - job_name: 'invitation-service'
    static_configs:
      - targets: ['invitation-service:8004']
  
  - job_name: 'notification-service'
    static_configs:
      - targets: ['notification-service:8005']
```

**Grafana Dashboards:**

- Service health (all 5 services)
- Request latency per service
- Inter-service call latency
- NATS event throughput
- Database connections per service

---

## 📊 Success Metrics

### **Migration Complete When:**

- [ ] All 5 services running independently
- [ ] No direct database access across services
- [ ] All inter-service communication via APIs or NATS
- [ ] GDPR export includes data from all services
- [ ] Frontend calls correct service for each feature
- [ ] Multi-pod deployment tested
- [ ] All tests passing (unit + integration)
- [ ] Documentation updated
- [ ] Monitoring dashboards showing all services

### **Performance Targets:**

- API response time: < 200ms (95th percentile)
- NATS event delivery: < 50ms
- Database query time: < 50ms (95th percentile)
- Service startup time: < 10s
- Multi-service call overhead: < 100ms

### **Reliability Targets:**

- Service uptime: 99.9%
- Graceful degradation when dependency down
- Circuit breaker for failing services
- Retry logic with exponential backoff
- Health checks every 30s

---

## 🚀 Deployment Strategy

### **Rolling Migration (Zero Downtime)**

**Week 1:**

1. Deploy settings-service (parallel to user-service)
2. Test both services serving settings
3. Switch frontend to settings-service
4. Remove settings from user-service

**Week 2:**

1. Deploy notification-service
2. Start NATS event publishing
3. Test notification creation
4. Switch frontend to notification-service
5. Remove notification settings from user-service

**Week 3:**

1. Deploy invitation-service
2. Test invitation validation API
3. Update auth-service to call invitation-service
4. Verify registration flow
5. Deploy admin panel for invitation management

**Week 4:**

1. Full integration testing
2. Performance testing
3. Multi-pod deployment
4. Production rollout

---

**Migration Lead:** Core Team  
**Status:** Ready to Execute  
**Timeline:** 3-4 weeks  
**Risk Level:** Medium (thorough testing required)
