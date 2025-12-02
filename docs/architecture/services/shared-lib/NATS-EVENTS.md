# NATS Events Reference - Unity Platform

**Version:** 0.1.0-alpha.1  
**Status:** ✅ Active Standard  
**Last Updated:** November 14, 2025

## Overview

Unity Platform uses NATS for event-driven inter-service communication. This document defines the standard event naming conventions, event types, and implementation patterns for both territory-scoped and global events.

## Table of Contents

1. [Event Naming Convention](#event-naming-convention)
2. [Event Scopes](#event-scopes)
3. [Standard Event Types](#standard-event-types)
4. [Event Payload Format](#event-payload-format)
5. [Publishing Events](#publishing-events)
6. [Subscribing to Events](#subscribing-to-events)
7. [JetStream Configuration](#jetstream-configuration)
8. [Security Considerations](#security-considerations) ⚠️ **CRITICAL**
9. [Best Practices](#best-practices)
10. [Monitoring](#monitoring)
11. [Migration Strategy](#migration-strategy)

---

## Event Naming Convention

### Format

```
{scope}.{domain}.{action}
```

### Components

- **scope**: Event visibility level (`territory.{code}`, `global`, `cross.{from}.{to}`)
- **domain**: Service or feature area (`user`, `auth`, `badge`, `course`, `forum`, `group`)
- **action**: Past-tense verb describing what happened (`registered`, `updated`, `deleted`, `awarded`)

### Examples

```
territory.dk.user.registered
territory.dk.user.profile_updated
territory.dk.badge.awarded
territory.dk.course.completed
territory.dk.forum.post_created

global.user.suspended
global.content.flagged
global.policy.updated
global.system.maintenance_scheduled

cross.dk.no.message.sent
cross.dk.se.collaboration.invited
cross.*.*.content.shared
```

---

## Event Scopes

### 1. **Territory-Scoped Events** (`territory.{code}.*`)

**Purpose:** Events that only matter within a single territory/pod  
**Replication:** R1 (single replica, no cross-pod replication)  
**Retention:** 30 days  
**Use Cases:** Local user actions, territory-specific operations

**Pattern:**

```
territory.{territory_code}.{domain}.{action}
```

**Examples:**

```
territory.dk.user.login              # User logged in (Denmark)
territory.dk.user.logout             # User logged out
territory.dk.session.expired         # Session expired
territory.dk.group.created           # Community group created
territory.dk.group.member_added      # Member joined group
territory.dk.post.published          # Forum post created
territory.dk.course.enrolled         # User enrolled in course
territory.dk.badge.earned            # Badge earned by user
```

**When to use:**

- User login/logout events
- Local group/community changes
- Territory-specific content creation
- Local notifications
- Territory administrative actions

### 2. **Global Events** (`global.*`)

**Purpose:** Events that need to be visible across ALL territories/pods  
**Replication:** R3 (replicated across all NATS nodes)  
**Retention:** 7 days  
**Use Cases:** Platform-wide changes, security events, global policies

**Pattern:**

```
global.{domain}.{action}
```

**Examples:**

```
global.user.registered               # User registered (global registry)
global.user.suspended                # User suspended platform-wide
global.user.deleted                  # User account deleted
global.auth.token_revoked            # JWT token invalidated globally
global.content.flagged               # Content flagged for moderation
global.content.removed               # Content removed by admin
global.policy.updated                # Platform policy changed
global.system.maintenance            # System maintenance scheduled
global.badge.type_created            # New badge type available
global.territory.created             # New territory added
```

**When to use:**

- Username/email registry updates (prevent duplicates)
- Security events (suspensions, token revocations)
- Platform-wide policy changes
- Global content moderation
- New territory/service deployments
- System-wide announcements

### 3. **Cross-Territory Events** (`cross.{from}.{to}.*`)

**Purpose:** Events between users/content in different territories  
**Replication:** R3 (needs to reach both territories)  
**Retention:** 14 days  
**Use Cases:** Cross-territory collaboration, messaging, content sharing

**Pattern:**

```
cross.{source_territory}.{target_territory}.{domain}.{action}
```

**Examples:**

```
cross.dk.no.message.sent             # DK user → NO user message
cross.dk.no.collaboration.invited    # DK user invites NO user
cross.se.dk.content.shared           # SE user shares content with DK user
cross.*.*.search.query               # Search across all territories
cross.dk.*.announcement.broadcast    # DK admin → all territories
```

**When to use:**

- Direct messages between users in different territories
- Cross-territory collaboration invitations
- Content sharing across territories
- Multi-territory search queries
- Territory-to-all announcements

---

## Standard Event Types

### User Domain Events

#### `user.registered`

**Scope:** Global (username/email must be unique globally)

```json
{
  "event_type": "user.registered",
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2025-11-14T12:30:00Z",
  "payload": {
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "johndoe",
    "email": "john@example.com",
    "territory": "dk",
    "invitation_token": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

**Publishers:** auth-service  
**Subscribers:** user-service, invitation-service, notification-service

#### `user.profile_updated`

**Scope:** Territory-local

```json
{
  "event_type": "user.profile_updated",
  "event_id": "550e8400-e29b-41d4-a716-446655440001",
  "timestamp": "2025-11-14T12:35:00Z",
  "payload": {
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "territory": "dk",
    "fields_updated": ["display_name", "avatar_url"],
    "updated_by": "123e4567-e89b-12d3-a456-426614174000"
  }
}
```

**Publishers:** user-service  
**Subscribers:** cache-invalidation, search-indexer

#### `user.suspended`

**Scope:** Global (prevent login across all territories)

```json
{
  "event_type": "user.suspended",
  "event_id": "550e8400-e29b-41d4-a716-446655440002",
  "timestamp": "2025-11-14T12:40:00Z",
  "payload": {
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "territory": "dk",
    "reason": "Terms of Service violation",
    "suspended_by": "admin-user-id",
    "duration_days": 30,
    "expires_at": "2025-12-14T12:40:00Z"
  }
}
```

**Publishers:** user-service, admin-service  
**Subscribers:** auth-service (revoke tokens), notification-service

#### `user.deleted` ✅ **IMPLEMENTED**

**Scope:** Global (GDPR compliance, cross-territory deletion)  
**Implementation:** `shared-lib/src/events/user.rs`

```json
{
  "user_id": "123e4567-e89b-12d3-a456-426614174000",
  "territory": "dk",
  "deleted_at": "2025-12-02T10:30:00Z"
}
```

**Publishers:** auth-service (when user requests account deletion)  
**Subscribers:** All services, task-scheduler-service (audit logging)

**Deletion Flow:**

1. **Day 0 - User Requests Deletion** (`DELETE /api/v1/users/me`)
   - auth-service soft-deletes user in `auth_users_core`
   - Sets `deleted_at` timestamp in `global.registry_username`
   - Publishes `user.deleted` event to NATS
   - User immediately cannot login

2. **Day 0 - Services React to Event**
   - user-service: Soft-deletes profiles, settings
   - badge-service: Soft-deletes badges, progress
   - community-service: Soft-deletes memberships, managers
   - All services set `deleted_at` on their user data

3. **Days 1-29 - Waiting Period (GDPR Right to Erasure)**
   - Data marked deleted but retained
   - User cannot access account
   - Admin can restore if requested within 30 days

4. **Day 30+ - Hard Deletion (Automated)**
   - task-scheduler-service cron job runs (Sunday 2:00 AM UTC)
   - Queries `global.registry_username WHERE deleted_at < NOW() - INTERVAL '30 days'`
   - Permanently deletes from ALL tables in transaction
   - Removes from global registries

**Event Struct:**
```rust
use shared_lib::UserDeletedEvent;

// Publishing (auth-service)
let event = UserDeletedEvent::new(user_id, territory);
nats_client.publish(
    UserDeletedEvent::SUBJECT, // "user.deleted"
    &serde_json::to_vec(&event)?
).await?;

// Subscribing (all services)
let mut sub = nats_client.subscribe(UserDeletedEvent::SUBJECT).await?;
while let Some(msg) = sub.next().await {
    let event: UserDeletedEvent = serde_json::from_slice(&msg.payload)?;
    // Soft-delete user data
    sqlx::query("UPDATE users SET deleted_at = $1 WHERE user_id = $2")
        .bind(event.deleted_at)
        .bind(event.user_id)
        .execute(pool).await?;
}
```

**Hard Deletion Details:**
- **Trigger:** Cron job in task-scheduler-service
- **Schedule:** Weekly (Sunday 2:00 AM UTC)
- **Criteria:** `deleted_at > 30 days ago`
- **Scope:** All services, all territories
- **Safety:** Transaction-safe, all-or-nothing deletion
- **Tables:** 18+ tables across 5 services

**Admin Endpoints:**
- `GET /api/v1/cleanup/stats` - View pending deletions (dry-run)
- `POST /api/v1/cleanup/users` - Manual trigger (requires `task-admin` badge)

**Monitoring:**
- task-scheduler logs all deletion events
- Statistics tracked: soft_deleted, eligible, deleted, territories
- Audit trail maintained for compliance

### Auth Domain Events

#### `auth.login`

**Scope:** Territory-local

```json
{
  "event_type": "auth.login",
  "event_id": "550e8400-e29b-41d4-a716-446655440003",
  "timestamp": "2025-11-14T12:45:00Z",
  "payload": {
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "territory": "dk",
    "ip_address": "192.168.1.100",
    "user_agent": "Mozilla/5.0...",
    "session_id": "session-uuid"
  }
}
```

**Publishers:** auth-service  
**Subscribers:** analytics-service, security-monitor

#### `auth.token_revoked`

**Scope:** Global (security event)

```json
{
  "event_type": "auth.token_revoked",
  "event_id": "550e8400-e29b-41d4-a716-446655440004",
  "timestamp": "2025-11-14T12:50:00Z",
  "payload": {
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "territory": "dk",
    "token_id": "token-uuid",
    "reason": "password_changed",
    "revoked_by": "user_id or admin_id"
  }
}
```

**Publishers:** auth-service  
**Subscribers:** All services with JWT validation, session-service

### Badge Domain Events

#### `badge.awarded`

**Scope:** Territory-local

```json
{
  "event_type": "badge.awarded",
  "event_id": "550e8400-e29b-41d4-a716-446655440005",
  "timestamp": "2025-11-14T13:00:00Z",
  "payload": {
    "badge_id": "badge-uuid",
    "badge_slug": "course-master-rust",
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "territory": "dk",
    "awarded_by": "system",
    "reason": "Completed all Rust courses",
    "metadata": {
      "courses_completed": 5,
      "avg_score": 95.5
    }
  }
}
```

**Publishers:** badge-service  
**Subscribers:** user-service (update permissions), notification-service, achievement-tracker

#### `badge.type_created`

**Scope:** Global (new badge available platform-wide)

```json
{
  "event_type": "badge.type_created",
  "event_id": "550e8400-e29b-41d4-a716-446655440006",
  "timestamp": "2025-11-14T13:05:00Z",
  "payload": {
    "badge_slug": "community-champion-2025",
    "name": "Community Champion 2025",
    "description": "Awarded for exceptional community contributions",
    "icon_url": "https://cdn.unityplatform.dk/badges/champion-2025.svg",
    "criteria": {
      "min_posts": 100,
      "min_helpful_votes": 500
    },
    "created_by": "admin-user-id"
  }
}
```

**Publishers:** badge-service  
**Subscribers:** All badge-service instances, cache-invalidation

### Course Domain Events

#### `course.completed`

**Scope:** Territory-local

```json
{
  "event_type": "course.completed",
  "event_id": "550e8400-e29b-41d4-a716-446655440007",
  "timestamp": "2025-11-14T13:10:00Z",
  "payload": {
    "course_id": "course-uuid",
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "territory": "dk",
    "score": 95.5,
    "completion_time_minutes": 240,
    "certificate_url": "https://cdn.unityplatform.dk/certificates/..."
  }
}
```

**Publishers:** course-service  
**Subscribers:** badge-service (trigger badge awards), analytics-service, notification-service

#### `course.enrollment`

**Scope:** Territory-local

```json
{
  "event_type": "course.enrollment",
  "event_id": "550e8400-e29b-41d4-a716-446655440008",
  "timestamp": "2025-11-14T13:15:00Z",
  "payload": {
    "course_id": "course-uuid",
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "territory": "dk",
    "enrollment_type": "self_enrolled",
    "deadline": "2025-12-31T23:59:59Z"
  }
}
```

**Publishers:** course-service  
**Subscribers:** notification-service, analytics-service

### Forum Domain Events

#### `forum.post_created`

**Scope:** Territory-local

```json
{
  "event_type": "forum.post_created",
  "event_id": "550e8400-e29b-41d4-a716-446655440009",
  "timestamp": "2025-11-14T13:20:00Z",
  "payload": {
    "post_id": "post-uuid",
    "thread_id": "thread-uuid",
    "forum_id": "forum-uuid",
    "author_id": "123e4567-e89b-12d3-a456-426614174000",
    "territory": "dk",
    "content_preview": "First 100 characters...",
    "mentions": ["user-uuid-1", "user-uuid-2"]
  }
}
```

**Publishers:** forum-service  
**Subscribers:** notification-service (for mentions), search-indexer, analytics-service

#### `forum.content_flagged`

**Scope:** Global (moderation event)

```json
{
  "event_type": "forum.content_flagged",
  "event_id": "550e8400-e29b-41d4-a716-446655440010",
  "timestamp": "2025-11-14T13:25:00Z",
  "payload": {
    "content_id": "post-uuid or comment-uuid",
    "content_type": "post",
    "flagged_by": "user-uuid",
    "territory": "dk",
    "reason": "spam",
    "moderator_queue": "global_moderation"
  }
}
```

**Publishers:** forum-service  
**Subscribers:** moderation-service, admin-dashboard

### Territory Domain Events

#### `territory.created`

**Scope:** Global (affects routing and service configuration)

```json
{
  "event_type": "territory.created",
  "event_id": "550e8400-e29b-41d4-a716-446655440011",
  "timestamp": "2025-11-14T13:30:00Z",
  "payload": {
    "territory_code": "fi",
    "territory_name": "Finland",
    "database_url": "postgresql://...",
    "nats_subjects": ["territory.fi.*"],
    "pod_id": "pod-nordic",
    "status": "active",
    "created_by": "admin-uuid"
  }
}
```

**Publishers:** territory-service  
**Subscribers:** All services (update routing tables), deployment-service, monitoring

### System Domain Events

#### `system.maintenance_scheduled`

**Scope:** Global

```json
{
  "event_type": "system.maintenance_scheduled",
  "event_id": "550e8400-e29b-41d4-a716-446655440012",
  "timestamp": "2025-11-14T13:35:00Z",
  "payload": {
    "maintenance_id": "maint-uuid",
    "scheduled_start": "2025-11-15T02:00:00Z",
    "scheduled_end": "2025-11-15T04:00:00Z",
    "affected_services": ["user-service", "auth-service"],
    "description": "Database schema migration",
    "notify_users": true
  }
}
```

**Publishers:** admin-service  
**Subscribers:** notification-service, status-page, all affected services

---

## Event Payload Format

### Standard Envelope

All events MUST follow this envelope format:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformEvent<T> {
    /// Unique event identifier
    pub event_id: Uuid,
    
    /// Event type (matches NATS subject last segment)
    pub event_type: String,
    
    /// ISO 8601 timestamp when event occurred
    pub timestamp: DateTime<Utc>,
    
    /// Territory where event originated (for routing)
    pub territory: Option<String>,
    
    /// Event-specific payload
    pub payload: T,
    
    /// Optional metadata for debugging/tracing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EventMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// Request ID for tracing
    pub request_id: Option<String>,
    
    /// Service that published the event
    pub publisher: String,
    
    /// Publisher version
    pub publisher_version: String,
    
    /// Correlation ID for event chains
    pub correlation_id: Option<Uuid>,
}
```

### Example Usage

```rust
use shared_lib::events::{PlatformEvent, EventMetadata};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisteredPayload {
    pub user_id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub territory: String,
}

// Create event
let event = PlatformEvent {
    event_id: Uuid::new_v4(),
    event_type: "user.registered".to_string(),
    timestamp: Utc::now(),
    territory: Some("dk".to_string()),
    payload: UserRegisteredPayload {
        user_id: Uuid::new_v4(),
        username: "johndoe".to_string(),
        email: Some("john@example.com".to_string()),
        territory: "dk".to_string(),
    },
    metadata: Some(EventMetadata {
        request_id: Some(request_id.clone()),
        publisher: "auth-service".to_string(),
        publisher_version: env!("CARGO_PKG_VERSION").to_string(),
        correlation_id: None,
    }),
};
```

---

## Publishing Events

### Basic Publishing

```rust
use shared_lib::NatsClient;
use serde_json;

// Initialize NATS client (in main.rs)
let nats_client = NatsClient::new(
    &env::var("NATS_URL").unwrap_or("nats://nats:4222".to_string()),
    "unityplan-global".to_string()
).await?;

// Make available to handlers
.app_data(web::Data::new(nats_client.clone()))

// In handler
async fn register_user(
    body: ValidatedJson<RegisterRequest>,
    db: web::Data<Database>,
    nats: web::Data<NatsClient>,
) -> Result<HttpResponse> {
    // ... register user logic ...
    
    // Publish global event
    let event = PlatformEvent {
        event_id: Uuid::new_v4(),
        event_type: "user.registered".to_string(),
        timestamp: Utc::now(),
        territory: Some(territory.clone()),
        payload: UserRegisteredPayload {
            user_id,
            username: username.clone(),
            email: email.clone(),
            territory: territory.clone(),
        },
        metadata: Some(EventMetadata {
            request_id: Some(request_id),
            publisher: "auth-service".to_string(),
            publisher_version: env!("CARGO_PKG_VERSION").to_string(),
            correlation_id: None,
        }),
    };
    
    let subject = "global.user.registered";
    let payload = serde_json::to_vec(&event)?;
    
    nats.publish(subject, payload).await?;
    
    tracing::info!(
        user_id = %user_id,
        subject = %subject,
        "Published user.registered event"
    );
    
    Ok(HttpResponse::Created().json(response))
}
```

### Territory-Scoped Events

```rust
// Publish to territory-specific subject
let subject = format!("territory.{}.badge.awarded", territory);
let payload = serde_json::to_vec(&event)?;

nats.publish(&subject, payload).await?;
```

### Cross-Territory Events

```rust
// DK user sends message to NO user
let subject = format!("cross.{}.{}.message.sent", from_territory, to_territory);
let payload = serde_json::to_vec(&event)?;

nats.publish(&subject, payload).await?;
```

---

## Subscribing to Events

### Basic Subscription

```rust
use shared_lib::NatsClient;
use futures::StreamExt;

async fn start_event_subscriber(nats: NatsClient) {
    // Subscribe to specific subject
    let mut subscriber = nats.client()
        .subscribe("global.user.registered".to_string())
        .await
        .expect("Failed to subscribe");
    
    tracing::info!("Subscribed to global.user.registered");
    
    // Process events
    while let Some(message) = subscriber.next().await {
        match serde_json::from_slice::<PlatformEvent<UserRegisteredPayload>>(&message.payload) {
            Ok(event) => {
                tracing::info!(
                    event_id = %event.event_id,
                    user_id = %event.payload.user_id,
                    "Received user.registered event"
                );
                
                // Handle event
                if let Err(e) = handle_user_registered(event).await {
                    tracing::error!(error = ?e, "Failed to handle event");
                }
            }
            Err(e) => {
                tracing::error!(error = ?e, "Failed to deserialize event");
            }
        }
    }
}

async fn handle_user_registered(event: PlatformEvent<UserRegisteredPayload>) -> Result<()> {
    // Business logic here
    Ok(())
}
```

### Wildcard Subscriptions

```rust
// Subscribe to all territory events for DK
let mut subscriber = nats.client()
    .subscribe("territory.dk.*".to_string())
    .await?;

// Subscribe to all user events across all territories
let mut subscriber = nats.client()
    .subscribe("*.user.*".to_string())
    .await?;

// Subscribe to all cross-territory events from DK
let mut subscriber = nats.client()
    .subscribe("cross.dk.>".to_string())
    .await?;
```

### Queue Groups (Load Balancing)

```rust
// Multiple instances process events in round-robin
let mut subscriber = nats.client()
    .queue_subscribe("global.user.registered".to_string(), "user-processor".to_string())
    .await?;
```

---

## JetStream Configuration

### Creating Streams

```bash
# Global events stream (R3 replication)
nats stream add GLOBAL_EVENTS \
  --subjects="global.*" \
  --storage=file \
  --replicas=3 \
  --retention=limits \
  --max-age=7d \
  --max-msgs=-1 \
  --max-bytes=-1 \
  --discard=old

# Territory stream (R1 replication)
nats stream add TERRITORY_DK \
  --subjects="territory.dk.*" \
  --storage=file \
  --replicas=1 \
  --retention=limits \
  --max-age=30d \
  --discard=old

# Cross-territory stream (R3 replication)
nats stream add CROSS_TERRITORY \
  --subjects="cross.*.*.*" \
  --storage=file \
  --replicas=3 \
  --retention=limits \
  --max-age=14d \
  --discard=old
```

### Creating Consumers

```bash
# Durable consumer for user events
nats consumer add GLOBAL_EVENTS user-event-processor \
  --filter="global.user.*" \
  --ack=explicit \
  --replay=instant \
  --deliver=all \
  --max-deliver=-1 \
  --wait=30s

# Territory-specific consumer
nats consumer add TERRITORY_DK dk-badge-processor \
  --filter="territory.dk.badge.*" \
  --ack=explicit \
  --replay=instant \
  --deliver=all
```

---

## Security Considerations

### 🔒 Critical Security Requirements

Event-driven architectures introduce security risks that MUST be addressed in production:

### 1. **Encryption**

#### Transport Security (REQUIRED in Production)

✅ **Enable TLS for NATS connections:**

```yaml
# docker-compose.yml - Production NATS configuration
service-nats:
  command:
    - "--tls"
    - "--tlscert=/certs/server-cert.pem"
    - "--tlskey=/certs/server-key.pem"
    - "--tlscacert=/certs/ca-cert.pem"
```

```rust
// Rust client with TLS
let nats_client = async_nats::ConnectOptions::new()
    .require_tls(true)
    .tls_client_config(tls_config)
    .connect(&nats_url)
    .await?;
```

#### Data at Rest (RECOMMENDED)

- JetStream file storage should be encrypted at filesystem level
- Use encrypted volumes for `/data` mount points
- Consider database-level encryption for sensitive event archives

### 2. **Authentication & Authorization**

#### NATS Authorization (REQUIRED in Production)

**Problem:** Without authorization, any service can publish/subscribe to any subject.

**Solution:** Use NATS account-based authorization:

```conf
# nats-server.conf
authorization {
  users = [
    # Auth service - can publish user events
    {
      user: "auth-service"
      password: $AUTH_SERVICE_PASSWORD
      permissions: {
        publish: ["global.user.*", "global.auth.*"]
        subscribe: []
      }
    }
    # Badge service - can publish badge events, subscribe to user events
    {
      user: "badge-service"
      password: $BADGE_SERVICE_PASSWORD
      permissions: {
        publish: ["territory.*.badge.*"]
        subscribe: ["global.user.*", "territory.*.course.*"]
      }
    }
    # Admin service - full access
    {
      user: "admin-service"
      password: $ADMIN_SERVICE_PASSWORD
      permissions: {
        publish: [">"]
        subscribe: [">"]
      }
    }
  ]
}
```

**Rust implementation:**

```rust
let nats_client = async_nats::ConnectOptions::new()
    .user_and_password(
        env::var("NATS_USER")?,
        env::var("NATS_PASSWORD")?
    )
    .connect(&nats_url)
    .await?;
```

#### Territory Isolation (CRITICAL)

**Enforce territory boundaries:**

```rust
// Validate territory in event matches service's allowed territories
async fn publish_event<T>(
    nats: &NatsClient,
    subject: &str,
    event: &PlatformEvent<T>,
    allowed_territories: &[String],
) -> Result<()> 
where
    T: Serialize,
{
    // Security: Verify territory in subject matches event payload
    if let Some(ref territory) = event.territory {
        if !allowed_territories.contains(territory) {
            return Err(AppError::Authorization(
                format!("Service not authorized for territory: {}", territory)
            ));
        }
        
        // Verify subject includes correct territory
        if subject.starts_with("territory.") && !subject.starts_with(&format!("territory.{}", territory)) {
            return Err(AppError::Validation(
                "Subject territory mismatch".to_string()
            ));
        }
    }
    
    let payload = serde_json::to_vec(&event)?;
    nats.publish(subject, payload).await
}
```

### 3. **Sensitive Data Handling**

#### ⚠️ NEVER Include in Events

**Prohibited data:**

- ❌ Passwords (even hashed)
- ❌ Password reset tokens
- ❌ JWT tokens (full tokens)
- ❌ API keys
- ❌ Session cookies
- ❌ Credit card numbers
- ❌ Social security numbers
- ❌ Full medical records

#### 🔶 Minimize PII (Personal Identifiable Information)

**Use with caution:**

- ⚠️ Email addresses (only when necessary, consider hashing)
- ⚠️ IP addresses (anonymize or exclude in production)
- ⚠️ User agents (truncate or exclude)
- ⚠️ Physical addresses
- ⚠️ Phone numbers
- ⚠️ Birth dates

**Best practice - Use references:**

```rust
// ❌ BAD - Includes sensitive data
#[derive(Serialize)]
struct UserRegisteredPayload {
    user_id: Uuid,
    username: String,
    email: String,          // PII
    full_name: String,      // PII
    date_of_birth: String,  // PII
    ip_address: String,     // PII
}

// ✅ GOOD - Minimal data, use IDs
#[derive(Serialize)]
struct UserRegisteredPayload {
    user_id: Uuid,
    username: String,       // Public identifier
    territory: String,
    // Services that need email can fetch from user-service using user_id
}
```

#### Redact PII in Logs

```rust
tracing::info!(
    user_id = %event.payload.user_id,
    // ❌ DON'T: email = %event.payload.email,
    territory = %event.payload.territory,
    "User registered event published"
);
```

### 4. **Event Signing & Verification**

#### Problem: Event Tampering

Without signatures, malicious services can:

- Forge events pretending to be from other services
- Modify events in transit
- Replay old events

#### Solution: HMAC Signatures

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

// Add signature to event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub request_id: Option<String>,
    pub publisher: String,
    pub publisher_version: String,
    pub correlation_id: Option<Uuid>,
    
    // Security: Event signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    
    // Timestamp for replay protection
    pub signed_at: DateTime<Utc>,
}

// Sign event before publishing
fn sign_event<T: Serialize>(event: &mut PlatformEvent<T>, secret: &[u8]) -> Result<()> {
    // Serialize payload without signature
    let payload_json = serde_json::to_string(&event.payload)?;
    
    // Create HMAC
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|e| AppError::Internal(format!("HMAC error: {}", e)))?;
    
    mac.update(payload_json.as_bytes());
    mac.update(event.event_id.as_bytes());
    mac.update(event.timestamp.to_rfc3339().as_bytes());
    
    let signature = hex::encode(mac.finalize().into_bytes());
    
    if let Some(ref mut metadata) = event.metadata {
        metadata.signature = Some(signature);
        metadata.signed_at = Utc::now();
    }
    
    Ok(())
}

// Verify event signature on consumption
fn verify_event<T: Serialize>(event: &PlatformEvent<T>, secret: &[u8]) -> Result<bool> {
    let metadata = event.metadata.as_ref()
        .ok_or(AppError::Validation("Missing event metadata".to_string()))?;
    
    let signature = metadata.signature.as_ref()
        .ok_or(AppError::Validation("Missing event signature".to_string()))?;
    
    // Check replay protection (event not older than 5 minutes)
    let age = Utc::now().signed_duration_since(metadata.signed_at);
    if age.num_minutes() > 5 {
        return Err(AppError::Validation("Event too old, possible replay attack".to_string()));
    }
    
    // Verify signature
    let payload_json = serde_json::to_string(&event.payload)?;
    
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|e| AppError::Internal(format!("HMAC error: {}", e)))?;
    
    mac.update(payload_json.as_bytes());
    mac.update(event.event_id.as_bytes());
    mac.update(event.timestamp.to_rfc3339().as_bytes());
    
    let expected = hex::encode(mac.finalize().into_bytes());
    
    Ok(signature == &expected)
}
```

### 5. **Rate Limiting**

#### Per-Service Event Publishing Limits

```rust
// Shared-lib rate limiter for event publishing
pub struct EventRateLimiter {
    limits: HashMap<String, RateLimiter>,
}

impl EventRateLimiter {
    pub async fn check_limit(&self, service: &str, subject: &str) -> Result<()> {
        let key = format!("{}:{}", service, subject);
        
        if let Some(limiter) = self.limits.get(&key) {
            if !limiter.check() {
                return Err(AppError::RateLimitExceeded(
                    format!("Event publishing rate limit exceeded for {}", key)
                ));
            }
        }
        
        Ok(())
    }
}

// Usage
rate_limiter.check_limit("auth-service", "global.user.registered").await?;
nats.publish(subject, payload).await?;
```

#### JetStream Consumer Rate Limits

```bash
# Limit consumer message delivery rate
nats consumer add GLOBAL_EVENTS rate-limited-processor \
  --filter="global.*" \
  --rate-limit=100  # Max 100 msgs/second
```

### 6. **Input Validation**

#### Validate Event Payloads on Consumption

```rust
async fn handle_user_registered(message: Message) -> Result<()> {
    // Deserialize and validate
    let event: PlatformEvent<UserRegisteredPayload> = 
        serde_json::from_slice(&message.payload)?;
    
    // Security: Verify event signature
    verify_event(&event, EVENT_SIGNING_SECRET.as_bytes())?;
    
    // Validate payload fields
    if event.payload.username.is_empty() {
        return Err(AppError::Validation("Empty username".to_string()));
    }
    
    if event.payload.username.len() > 50 {
        return Err(AppError::Validation("Username too long".to_string()));
    }
    
    // Sanitize for SQL injection, XSS, etc.
    let safe_username = sanitize(&event.payload.username);
    
    // Process event
    process_registration(event.payload.user_id, &safe_username).await
}

fn sanitize(input: &str) -> String {
    // Remove SQL injection attempts
    input.replace("'", "")
         .replace("\"", "")
         .replace(";", "")
         .replace("--", "")
         // Remove XSS attempts
         .replace("<", "&lt;")
         .replace(">", "&gt;")
}
```

### 7. **Audit Logging**

#### Log All Event Publishing

```rust
async fn publish_with_audit<T: Serialize>(
    nats: &NatsClient,
    subject: &str,
    event: &PlatformEvent<T>,
    request_id: &str,
) -> Result<()> {
    // Audit log BEFORE publishing
    tracing::info!(
        event_id = %event.event_id,
        event_type = %event.event_type,
        subject = %subject,
        publisher = ?event.metadata.as_ref().map(|m| &m.publisher),
        request_id = %request_id,
        "Publishing NATS event"
    );
    
    let payload = serde_json::to_vec(&event)?;
    
    match nats.publish(subject, payload).await {
        Ok(_) => {
            tracing::info!(
                event_id = %event.event_id,
                subject = %subject,
                "Event published successfully"
            );
            Ok(())
        }
        Err(e) => {
            tracing::error!(
                event_id = %event.event_id,
                subject = %subject,
                error = ?e,
                "Failed to publish event"
            );
            Err(e)
        }
    }
}
```

### 8. **GDPR Compliance**

#### Data Retention Policies

```bash
# Automatic deletion after retention period
nats stream add GLOBAL_EVENTS \
  --subjects="global.*" \
  --retention=limits \
  --max-age=7d    # Auto-delete after 7 days (GDPR compliance)
  
nats stream add TERRITORY_DK \
  --subjects="territory.dk.*" \
  --retention=limits \
  --max-age=30d   # Auto-delete after 30 days
```

#### Right to Be Forgotten

**Problem:** Events contain user data that must be deleted on request.

**Solution 1:** Don't store sensitive PII in events (use references)

**Solution 2:** Implement event scrubbing:

```rust
// Anonymize events for deleted users
async fn handle_user_deletion(user_id: Uuid) -> Result<()> {
    // Publish deletion event
    nats.publish(
        "global.user.deleted",
        PlatformEvent {
            event_type: "user.deleted".to_string(),
            payload: UserDeletedPayload { user_id },
            // ...
        }
    ).await?;
    
    // All services listening should:
    // 1. Delete user-specific data
    // 2. Anonymize or delete cached events
    // 3. Mark user as [deleted] in search indexes
    
    Ok(())
}
```

### 9. **Security Checklist**

Before publishing events to production:

- [ ] ✅ TLS enabled for NATS connections
- [ ] ✅ NATS user authentication configured
- [ ] ✅ Subject-level authorization enforced
- [ ] ✅ Territory isolation validated
- [ ] ✅ No passwords/tokens in event payloads
- [ ] ✅ PII minimized or excluded
- [ ] ✅ Event signatures implemented (if required)
- [ ] ✅ Rate limiting configured
- [ ] ✅ Input validation on event consumption
- [ ] ✅ Audit logging enabled
- [ ] ✅ GDPR retention policies configured
- [ ] ✅ Sensitive data redacted from logs
- [ ] ✅ JetStream storage encrypted
- [ ] ✅ Regular security audits scheduled

### 10. **Common Security Anti-Patterns**

❌ **DON'T DO THIS:**

```rust
// ❌ Including password in event
nats.publish("user.registered", json!({
    "user_id": user_id,
    "password": password  // NEVER!
}));

// ❌ No validation before processing
async fn handle_event(msg: Message) {
    let event: Event = serde_json::from_slice(&msg.payload)?;
    database.execute(&event.sql_query).await?;  // SQL injection!
}

// ❌ Logging sensitive data
tracing::info!(
    "User registered: email={}, password_hash={}",  // PII in logs!
    email, hash
);

// ❌ No authorization check
nats.publish("admin.delete_all_users", json!({}));  // Any service can publish!

// ❌ Trusting event content without verification
async fn award_badge(event: BadgeEvent) {
    // No signature check - could be forged!
    database.award_badge(event.user_id, event.badge_id).await?;
}
```

✅ **DO THIS INSTEAD:**

```rust
// ✅ Use references, not sensitive data
nats.publish("user.registered", json!({
    "user_id": user_id,
    // Services fetch email from user-service if needed
}));

// ✅ Validate and sanitize
async fn handle_event(msg: Message) {
    let event: Event = serde_json::from_slice(&msg.payload)?;
    verify_event_signature(&event)?;
    let safe_input = sanitize(&event.username);
    database.insert_user(safe_input).await?;
}

// ✅ Redact PII in logs
tracing::info!(
    user_id = %user_id,
    "User registered"  // No PII
);

// ✅ Check permissions before publishing
if !is_admin_service() {
    return Err(AppError::Authorization("Not authorized"));
}
nats.publish("admin.delete_all_users", payload).await?;

// ✅ Verify event authenticity
async fn award_badge(event: BadgeEvent) {
    verify_event_signature(&event, SECRET.as_bytes())?;
    check_event_age(&event)?;  // Replay protection
    database.award_badge(event.user_id, event.badge_id).await?;
}
```

---

## Best Practices

### 1. **Event Naming**

✅ **DO:**

- Use past tense for actions (`registered`, `updated`, `deleted`)
- Be specific (`user.profile_updated` not `user.changed`)
- Use lowercase with dots as separators
- Follow the scope.domain.action pattern

❌ **DON'T:**

- Use present tense (`registering`, `updating`)
- Be vague (`user.event`, `thing.happened`)
- Mix naming conventions
- Include version numbers in subject names

### 2. **Event Payload**

✅ **DO:**

- Keep payloads small (< 1KB if possible)
- Include all necessary identifiers (user_id, territory, etc.)
- Use ISO 8601 for timestamps
- Include request_id for tracing
- Version your payload structs

❌ **DON'T:**

- Include large binary data (use URLs to external storage)
- Embed entire objects (use IDs and let subscribers fetch if needed)
- Send sensitive data unencrypted
- Make breaking changes to payload structure

### 3. **Error Handling**

✅ **DO:**

- Log failed event processing with context
- Use dead letter queues for unprocessable events
- Implement exponential backoff for retries
- Monitor event processing lag

❌ **DON'T:**

- Silently drop failed events
- Retry indefinitely without backoff
- Block event processing on failures
- Ignore duplicate events

### 4. **Performance**

✅ **DO:**

- Use queue groups for load balancing
- Batch process events when possible
- Monitor message rates and adjust replicas
- Use appropriate retention policies

❌ **DON'T:**

- Subscribe with broad wildcards unnecessarily
- Process events synchronously in request handlers
- Store events indefinitely
- Publish events in tight loops without batching

### 5. **Testing**

✅ **DO:**

- Write integration tests for event publishing
- Mock NATS in unit tests
- Test event serialization/deserialization
- Verify idempotency of event handlers

❌ **DON'T:**

- Test against production NATS
- Assume events arrive in order
- Skip testing error scenarios
- Forget to test wildcard subscriptions

---

## Example: Complete Event Flow

### Scenario: User Registration Triggers Badge Award

**1. Auth-service publishes registration event:**

```rust
// auth-service/src/handlers/auth.rs
nats.publish(
    "global.user.registered",
    serde_json::to_vec(&PlatformEvent {
        event_id: Uuid::new_v4(),
        event_type: "user.registered".to_string(),
        timestamp: Utc::now(),
        territory: Some("dk".to_string()),
        payload: UserRegisteredPayload {
            user_id,
            username: "johndoe".to_string(),
            email: Some("john@example.com".to_string()),
            territory: "dk".to_string(),
        },
        metadata: Some(EventMetadata {
            request_id: Some(request_id),
            publisher: "auth-service".to_string(),
            publisher_version: env!("CARGO_PKG_VERSION").to_string(),
            correlation_id: None,
        }),
    })?
).await?;
```

**2. Badge-service subscribes and processes:**

```rust
// badge-service/src/subscribers/user_events.rs
async fn handle_user_registered(event: PlatformEvent<UserRegisteredPayload>) -> Result<()> {
    tracing::info!(
        user_id = %event.payload.user_id,
        "Processing user.registered event for badge award"
    );
    
    // Award "Welcome" badge to new user
    let badge_awarded = award_badge(
        &event.payload.user_id,
        "welcome-badge",
        &event.payload.territory,
        "Awarded automatically on registration"
    ).await?;
    
    // Publish badge.awarded event
    nats.publish(
        &format!("territory.{}.badge.awarded", event.payload.territory),
        serde_json::to_vec(&PlatformEvent {
            event_id: Uuid::new_v4(),
            event_type: "badge.awarded".to_string(),
            timestamp: Utc::now(),
            territory: Some(event.payload.territory.clone()),
            payload: BadgeAwardedPayload {
                badge_id: badge_awarded.id,
                badge_slug: "welcome-badge".to_string(),
                user_id: event.payload.user_id,
                territory: event.payload.territory.clone(),
                awarded_by: "system".to_string(),
                reason: "Welcome to Unity Platform!".to_string(),
            },
            metadata: Some(EventMetadata {
                request_id: event.metadata.as_ref().and_then(|m| m.request_id.clone()),
                publisher: "badge-service".to_string(),
                publisher_version: env!("CARGO_PKG_VERSION").to_string(),
                correlation_id: Some(event.event_id), // Chain events
            }),
        })?
    ).await?;
    
    Ok(())
}
```

**3. Notification-service sends welcome email:**

```rust
// notification-service/src/subscribers/badge_events.rs
async fn handle_badge_awarded(event: PlatformEvent<BadgeAwardedPayload>) -> Result<()> {
    // Send congratulations email
    send_email(
        &event.payload.user_id,
        "Congratulations on your first badge!",
        &format!("You've earned the {} badge!", event.payload.badge_slug)
    ).await?;
    
    Ok(())
}
```

---

## Monitoring

### Key Metrics

```bash
# Message rate
nats_events_published_total{service="auth-service",subject="global.user.registered"}
nats_events_consumed_total{service="badge-service",subject="global.user.registered"}

# Processing lag
nats_consumer_lag_seconds{consumer="badge-processor",stream="GLOBAL_EVENTS"}

# Errors
nats_event_processing_errors_total{service="badge-service",event_type="user.registered"}
```

### Health Checks

```rust
// Verify NATS connectivity in /ready endpoint
async fn ready(nats: web::Data<NatsClient>) -> Result<HttpResponse> {
    // Test NATS publish
    nats.publish("health.check", b"ping").await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "status": "ready",
        "nats": "connected"
    })))
}
```

---

## Migration Strategy

### Adding New Event Types

1. **Define event in shared-lib:**

```rust
// shared-lib/src/events.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct NewEventPayload {
    // fields
}
```

2. **Document in this file**
3. **Publish from service**
4. **Add subscribers in consuming services**
5. **Monitor adoption**

### Deprecating Events

1. Mark as deprecated in documentation
2. Add warning logs to publishers
3. Give consumers 2 releases to migrate
4. Remove publisher code
5. Remove from documentation

---

**See Also:**

- [NATS Clustering Configuration](../../../guides/deployment/nats-clustering.md)
- [Inter-Service Communication](./INTER-SERVICE-COMMUNICATION.md)
- [Error Handling](./ERROR-HANDLING.md)
