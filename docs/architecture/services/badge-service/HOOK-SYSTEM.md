# Badge Criteria Hook System Design

**Version:** 0.1.0-alpha.1  
**Status:** Design Proposal  
**Author:** AI Assistant  
**Date:** January 16, 2025

---

## Overview

The Badge Criteria Hook System enables services to trigger badge awards through generic NATS events without direct coupling to badge-service. This maintains service independence while allowing flexible, criteria-based badge awarding.

## Core Principles

1. **Service Independence**: Course-service (or any service) never calls badge-service directly
2. **Event-Driven**: Badge-service subscribes to generic progress events
3. **Flexible Criteria**: Badge criteria can be evaluated server-side without client knowledge
4. **Retroactive Support**: New badges can be added without modifying source services

## Architecture

### Event Flow

```
┌─────────────────┐
│ Course Service  │
│                 │
│ User completes  │
│ course module   │
└────────┬────────┘
         │
         │ Publishes: badge.progress.course-completion-10
         │ Payload: { user_id, value: 8 }
         ▼
┌─────────────────┐
│   NATS Broker   │
└────────┬────────┘
         │
         │ Subscribed: badge.progress.*
         ▼
┌─────────────────┐
│  Badge Service  │
│                 │
│ 1. Receive evt  │
│ 2. Query badges │
│    with criteria│
│ 3. Evaluate     │
│ 4. Auto-award   │
└─────────────────┘
```

### Event Subject Pattern

**Pattern:** `badge.progress.{badge-slug}`

**Examples:**

- `badge.progress.course-completion-10` - User completed courses
- `badge.progress.forum-contributor` - User forum posts
- `badge.progress.territory-advocate` - User territory participation

### Event Payload

```json
{
  "user_id": "uuid",
  "value": 8,                    // Current progress value
  "territory_code": "dk",        // Optional: territory context
  "metadata": {                  // Optional: additional context
    "course_id": "uuid",
    "score": 95,
    "completed_at": "2025-01-16T10:00:00Z"
  }
}
```

## Database Schema Enhancement

### Badge Registry Changes

Add `criteria_event_subject` field to `global.badge_registry`:

```sql
ALTER TABLE global.badge_registry
ADD COLUMN criteria_event_subject VARCHAR(255);

COMMENT ON COLUMN global.badge_registry.criteria_event_subject IS 
'NATS subject pattern for event-driven criteria evaluation (e.g., badge.progress.course-completion-10)';
```

### Example Badge Configuration

```sql
INSERT INTO global.badge_registry (
    name, 
    slug, 
    description,
    criteria_type,
    criteria_value,
    criteria_event_subject,
    rarity
) VALUES (
    'Course Completion Master',
    'course-completion-10',
    'Complete 10 courses with passing scores',
    'count',
    10,
    'badge.progress.course-completion-10',
    'rare'
);
```

## Implementation Pattern

### 1. Source Service (e.g., Course Service)

```rust
// When user completes a course
async fn on_course_completed(
    nats: &NatsClient,
    user_id: Uuid,
    course_count: i32,
) -> Result<()> {
    let event = serde_json::json!({
        "user_id": user_id,
        "value": course_count,
        "metadata": {
            "latest_course_id": course_id,
            "completed_at": chrono::Utc::now(),
        }
    });

    let payload = serde_json::to_vec(&event)?;
    nats.publish("badge.progress.course-completion-10", payload).await?;
    
    Ok(())
}
```

### 2. Badge Service Subscription

```rust
pub async fn subscribe_badge_progress(nats: NatsClient, db: Database) {
    let subscriber = nats.subscribe("badge.progress.*").await.unwrap();
    
    tokio::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            // Extract badge slug from subject
            let parts: Vec<&str> = msg.subject.split('.').collect();
            if parts.len() != 3 {
                continue;
            }
            let badge_slug = parts[2];
            
            // Parse event
            let event: BadgeProgressEvent = match serde_json::from_slice(&msg.payload) {
                Ok(e) => e,
                Err(_) => continue,
            };
            
            // Evaluate criteria
            match evaluate_badge_criteria(&db, &nats, event.user_id, badge_slug, event.value).await {
                Ok(awarded) => {
                    if awarded {
                        tracing::info!(
                            "Auto-awarded badge '{}' to user {} via progress event",
                            badge_slug,
                            event.user_id
                        );
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to evaluate badge criteria for {}: {}",
                        badge_slug,
                        e
                    );
                }
            }
        }
    });
}
```

### 3. Criteria Evaluation

```rust
async fn evaluate_badge_criteria(
    db: &Database,
    nats: &NatsClient,
    user_id: Uuid,
    badge_slug: &str,
    current_value: i32,
) -> Result<bool> {
    // Get badge configuration
    let badge = sqlx::query!(
        r#"
        SELECT id, name, criteria_type, criteria_value
        FROM global.badge_registry
        WHERE slug = $1 AND is_active = true AND criteria_event_subject IS NOT NULL
        "#,
        badge_slug
    )
    .fetch_optional(db.pool())
    .await?;
    
    let badge = match badge {
        Some(b) => b,
        None => return Ok(false), // Badge not found or not event-driven
    };
    
    // Check if user already has badge
    let territory = "dk"; // TODO: Get from context
    let has_badge = sqlx::query_scalar!(
        &format!(
            "SELECT EXISTS(SELECT 1 FROM territory_{}.user_badges WHERE user_id = $1 AND badge_id = $2)",
            territory
        ),
        user_id,
        badge.id
    )
    .fetch_one(db.pool())
    .await?
    .unwrap_or(false);
    
    if has_badge {
        return Ok(false); // Already has badge
    }
    
    // Evaluate criteria
    let should_award = match badge.criteria_type.as_str() {
        "count" => {
            let target = badge.criteria_value.unwrap_or(1);
            current_value >= target
        }
        "boolean" => current_value > 0,
        _ => false,
    };
    
    if should_award {
        // Auto-award the badge
        award_badge(
            db,
            nats,
            user_id,
            badge_slug,
            None, // System-awarded
            Some("Auto-awarded via progress event".to_string()),
        )
        .await?;
        
        return Ok(true);
    }
    
    Ok(false)
}
```

## Badge Progress Event Model

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeProgressEvent {
    pub user_id: Uuid,
    pub value: i32,
    #[serde(default)]
    pub territory_code: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}
```

## Migration Path

### Phase 1: Infrastructure (Current Sprint)

1. ✅ Add NATS event publishing to badge-service (completed)
2. ⏳ Add `criteria_event_subject` column to badge registry
3. ⏳ Implement `badge.progress.*` subscription
4. ⏳ Implement criteria evaluation function

### Phase 2: Course Service Integration (Future)

1. Publish `badge.progress.course-completion-10` events
2. Publish `badge.progress.course-instructor` events
3. Update course completion counter

### Phase 3: Forum Service Integration (Future)

1. Publish `badge.progress.forum-contributor` events
2. Publish `badge.progress.forum-moderator` events

### Phase 4: Territory Service Integration (Future)

1. Publish `badge.progress.territory-advocate` events
2. Publish `badge.progress.territory-organizer` events

## Benefits

### Service Independence

- Course-service doesn't know about badge-service
- Badge-service doesn't depend on course-service APIs
- Services communicate through events only

### Flexibility

- Add new badges without changing source services
- Update criteria without redeploying services
- Support multiple services for same badge

### Retroactive Support

- New badges can evaluate historical progress events
- Event replay can award badges retroactively

### Scalability

- NATS handles event distribution
- Badge-service scales independently
- No synchronous API dependencies

## Example Use Cases

### Course Completion Badge

**Source:** Course Service  
**Event:** `badge.progress.course-completion-10`  
**Criteria:** User completed 10 courses  
**Trigger:** When user completes any course

### Forum Contributor Badge

**Source:** Forum Service  
**Event:** `badge.progress.forum-contributor`  
**Criteria:** User made 50 helpful forum posts  
**Trigger:** When user creates a post

### Territory Advocate Badge

**Source:** Territory Service  
**Event:** `badge.progress.territory-advocate`  
**Criteria:** User participated in 5 territory events  
**Trigger:** When user RSVPs to event

## Testing Strategy

### Unit Tests

- Criteria evaluation logic
- Event parsing and validation
- Badge awarding conditions

### Integration Tests

- NATS event subscription
- Database queries for criteria
- Auto-award flow

### E2E Tests

1. Publish progress event
2. Verify badge auto-awarded
3. Verify badge.awarded event published
4. Verify idempotency (duplicate events)

## Security Implementation

The event-driven hook system requires careful security design to prevent abuse while maintaining service independence. This section details a comprehensive security architecture using cryptographic signatures.

### Security Architecture Overview

**Hybrid Approach: API Registration + Cryptographic Event Signing**

1. **Service Registration** (One-time, via HTTP API)
   - Service authenticates with badge-service API
   - Registers public key and service identity
   - Receives confirmation and service_id

2. **Event Publishing** (Runtime, via NATS)
   - Service signs events with private key
   - Publishes signed events to NATS
   - No authentication headers needed

3. **Event Verification** (Badge-service, on receipt)
   - Verifies signature using registered public key
   - Rejects events from unregistered services
   - Logs all verification attempts

### Why This Approach?

| Aspect | Direct API Calls | Pure NATS Events | Signed NATS Events (Our Choice) |
|--------|-----------------|------------------|----------------------------------|
| **Coupling** | High (sync HTTP) | Low (async events) | ✅ Low (async events) |
| **Authentication** | JWT per request | None | ✅ Crypto signature per event |
| **Performance** | Slower (HTTP) | Fast (NATS) | ✅ Fast (NATS) |
| **Audit Trail** | Service identity | No identity | ✅ Service identity + signature |
| **Trust Model** | API auth | Trust network | ✅ Zero-trust cryptographic |
| **Abuse Prevention** | Rate limiting | Limited | ✅ Service accountability |

### Database Schema for Service Registry

```sql
-- Add to migration: Create service registry for event publishers
CREATE TABLE IF NOT EXISTS global.service_publisher_registry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_name VARCHAR(100) NOT NULL UNIQUE,
    service_type VARCHAR(50) NOT NULL, -- 'course', 'forum', 'territory', etc.
    public_key TEXT NOT NULL,
    registered_by UUID REFERENCES global.username_registry(user_id),
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_event_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    metadata JSONB,
    
    CONSTRAINT service_name_format CHECK (service_name ~ '^[a-z][a-z0-9-]*$')
);

CREATE INDEX idx_service_publisher_active ON global.service_publisher_registry(service_name) 
WHERE is_active = true;

COMMENT ON TABLE global.service_publisher_registry IS 
'Registry of services authorized to publish badge progress events with their public keys';

-- Track event verification attempts for security monitoring
CREATE TABLE IF NOT EXISTS global.event_verification_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_name VARCHAR(100),
    event_subject VARCHAR(255) NOT NULL,
    verified BOOLEAN NOT NULL,
    failure_reason TEXT,
    user_id UUID,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB
);

CREATE INDEX idx_event_verification_failures ON global.event_verification_log(verified, timestamp)
WHERE NOT verified;
```

### Service Registration Endpoint

**API Endpoint:** `POST /api/v1/badges/register-publisher`

**Authentication Required:** Platform Manager badge (high-trust admin)

```rust
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPublisherRequest {
    #[validate(regex = "^[a-z][a-z0-9-]*$")]
    pub service_name: String,
    
    #[validate(length(min = 1, max = 50))]
    pub service_type: String, // 'course', 'forum', 'territory'
    
    #[validate(length(min = 200, max = 2000))]
    pub public_key: String, // PEM-encoded Ed25519 public key
    
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPublisherResponse {
    pub service_id: Uuid,
    pub service_name: String,
    pub registered_at: DateTime<Utc>,
    pub status: String,
}
```

**Handler Implementation:**

```rust
use shared_lib::{AuthUser, RequirePermission, PermissionChecker};

pub async fn register_publisher(
    auth: AuthUser,
    body: ValidatedJson<RegisterPublisherRequest>,
    db: web::Data<Database>,
    permission_checker: web::Data<PermissionChecker>,
) -> Result<HttpResponse> {
    // Verify caller has platform:manage permission
    if !permission_checker
        .has_permission(auth.id, "platform:manage")
        .await?
    {
        return Err(AppError::Forbidden(
            "Requires Platform Manager badge".to_string(),
        ));
    }

    // Validate public key format (Ed25519)
    validate_ed25519_public_key(&body.public_key)?;

    // Register or update service
    let service_id = sqlx::query_scalar!(
        r#"
        INSERT INTO global.service_publisher_registry 
        (service_name, service_type, public_key, registered_by, metadata)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (service_name) 
        DO UPDATE SET 
            public_key = EXCLUDED.public_key,
            registered_by = EXCLUDED.registered_by,
            metadata = EXCLUDED.metadata,
            is_active = true
        RETURNING id
        "#,
        body.service_name,
        body.service_type,
        body.public_key,
        auth.id,
        body.metadata
    )
    .fetch_one(db.pool())
    .await?;

    tracing::info!(
        "Service publisher '{}' registered by {} (service_id: {})",
        body.service_name,
        auth.username,
        service_id
    );

    Ok(HttpResponse::Created().json(RegisterPublisherResponse {
        service_id,
        service_name: body.service_name.clone(),
        registered_at: Utc::now(),
        status: "active".to_string(),
    }))
}

fn validate_ed25519_public_key(key_pem: &str) -> Result<()> {
    use ed25519_dalek::VerifyingKey;
    
    // Parse PEM and verify it's valid Ed25519
    let key_bytes = pem::parse(key_pem)
        .map_err(|e| AppError::Validation(format!("Invalid PEM format: {}", e)))?;
    
    VerifyingKey::from_bytes(&key_bytes.contents.try_into().map_err(|_| {
        AppError::Validation("Invalid key length for Ed25519".to_string())
    })?)
    .map_err(|e| AppError::Validation(format!("Invalid Ed25519 key: {}", e)))?;
    
    Ok(())
}
```

### Enhanced Event Payload with Signature

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedBadgeProgressEvent {
    // Core event data
    pub user_id: Uuid,
    pub value: i32,
    #[serde(default)]
    pub territory_code: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    
    // Security fields
    pub service_name: String,
    pub event_id: Uuid, // Unique per event, prevents replay
    pub timestamp: DateTime<Utc>,
    pub signature: String, // Base64-encoded Ed25519 signature
}

impl SignedBadgeProgressEvent {
    /// Create canonical representation for signing
    fn canonical_representation(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.event_id,
            self.service_name,
            self.user_id,
            self.value,
            self.timestamp.to_rfc3339(),
            self.territory_code.as_deref().unwrap_or(""),
        )
    }
}
```

### Service-Side: Signing Events (Course Service Example)

```rust
use ed25519_dalek::{Signer, SigningKey};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub struct BadgeEventPublisher {
    nats: NatsClient,
    service_name: String,
    signing_key: SigningKey,
}

impl BadgeEventPublisher {
    pub fn new(nats: NatsClient, service_name: String, private_key_pem: &str) -> Result<Self> {
        let key_bytes = pem::parse(private_key_pem)?.contents;
        let signing_key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());
        
        Ok(Self {
            nats,
            service_name,
            signing_key,
        })
    }
    
    pub async fn publish_course_completion(
        &self,
        user_id: Uuid,
        course_count: i32,
        territory_code: &str,
    ) -> Result<()> {
        let event = SignedBadgeProgressEvent {
            user_id,
            value: course_count,
            territory_code: Some(territory_code.to_string()),
            metadata: None,
            service_name: self.service_name.clone(),
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            signature: String::new(), // Will be filled below
        };
        
        // Create signature
        let canonical = event.canonical_representation();
        let signature = self.signing_key.sign(canonical.as_bytes());
        
        let mut signed_event = event;
        signed_event.signature = BASE64.encode(signature.to_bytes());
        
        // Publish to NATS
        let payload = serde_json::to_vec(&signed_event)?;
        self.nats
            .publish("badge.progress.course-completion-10", payload)
            .await?;
        
        tracing::debug!(
            "Published signed badge progress event for user {} (event_id: {})",
            user_id,
            signed_event.event_id
        );
        
        Ok(())
    }
}
```

### Badge Service: Event Verification

```rust
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub async fn subscribe_badge_progress(nats: NatsClient, db: Database) {
    let subscriber = nats.subscribe("badge.progress.*").await.unwrap();
    
    tokio::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            // Extract badge slug from subject
            let parts: Vec<&str> = msg.subject.split('.').collect();
            if parts.len() != 3 {
                continue;
            }
            let badge_slug = parts[2];
            
            // Parse and verify event
            match verify_and_process_event(&db, &nats, badge_slug, &msg.payload).await {
                Ok(Some(user_id)) => {
                    tracing::info!(
                        "Successfully processed badge progress event for user {} (badge: {})",
                        user_id,
                        badge_slug
                    );
                }
                Ok(None) => {
                    // Event verified but no badge awarded (criteria not met)
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to verify badge progress event for {}: {}",
                        badge_slug,
                        e
                    );
                }
            }
        }
    });
}

async fn verify_and_process_event(
    db: &Database,
    nats: &NatsClient,
    badge_slug: &str,
    payload: &[u8],
) -> Result<Option<Uuid>> {
    // Parse event
    let event: SignedBadgeProgressEvent = serde_json::from_slice(payload)
        .map_err(|e| AppError::Validation(format!("Invalid event format: {}", e)))?;
    
    // Get service public key
    let service = sqlx::query!(
        r#"
        SELECT public_key
        FROM global.service_publisher_registry
        WHERE service_name = $1 AND is_active = true
        "#,
        event.service_name
    )
    .fetch_optional(db.pool())
    .await?;
    
    let service = match service {
        Some(s) => s,
        None => {
            log_verification_failure(
                db,
                &event.service_name,
                badge_slug,
                "Service not registered",
                event.user_id,
            )
            .await;
            return Err(AppError::Forbidden(format!(
                "Service '{}' not registered",
                event.service_name
            )));
        }
    };
    
    // Verify signature
    if let Err(e) = verify_event_signature(&event, &service.public_key) {
        log_verification_failure(
            db,
            &event.service_name,
            badge_slug,
            &format!("Signature verification failed: {}", e),
            event.user_id,
        )
        .await;
        return Err(e);
    }
    
    // Check event freshness (prevent replay attacks)
    let event_age = Utc::now().signed_duration_since(event.timestamp);
    if event_age.num_minutes() > 5 {
        log_verification_failure(
            db,
            &event.service_name,
            badge_slug,
            "Event too old (replay attack?)",
            event.user_id,
        )
        .await;
        return Err(AppError::Validation("Event timestamp too old".to_string()));
    }
    
    // Check for duplicate event_id (deduplication)
    if is_duplicate_event(db, &event.event_id).await? {
        tracing::warn!(
            "Duplicate event_id {} from service {}",
            event.event_id,
            event.service_name
        );
        return Ok(None);
    }
    
    // Rate limiting: max 100 events per user per hour
    if exceeds_rate_limit(db, event.user_id, 100, 3600).await? {
        log_verification_failure(
            db,
            &event.service_name,
            badge_slug,
            "Rate limit exceeded",
            event.user_id,
        )
        .await;
        return Err(AppError::TooManyRequests(
            "Badge progress rate limit exceeded".to_string(),
        ));
    }
    
    // Update last_event_at for service
    update_service_activity(db, &event.service_name).await?;
    
    // Evaluate badge criteria
    let awarded = evaluate_badge_criteria(
        db,
        nats,
        event.user_id,
        badge_slug,
        event.value,
    )
    .await?;
    
    // Log successful verification
    log_verification_success(db, &event.service_name, badge_slug, event.user_id).await;
    
    if awarded {
        Ok(Some(event.user_id))
    } else {
        Ok(None)
    }
}

fn verify_event_signature(event: &SignedBadgeProgressEvent, public_key_pem: &str) -> Result<()> {
    // Parse public key
    let key_bytes = pem::parse(public_key_pem)
        .map_err(|e| AppError::Internal(format!("Invalid PEM: {}", e)))?;
    
    let verifying_key = VerifyingKey::from_bytes(&key_bytes.contents.try_into().unwrap())
        .map_err(|e| AppError::Internal(format!("Invalid key: {}", e)))?;
    
    // Decode signature
    let signature_bytes = BASE64
        .decode(&event.signature)
        .map_err(|e| AppError::Validation(format!("Invalid signature encoding: {}", e)))?;
    
    let signature = Signature::from_bytes(&signature_bytes.try_into().unwrap());
    
    // Verify signature
    let canonical = event.canonical_representation();
    verifying_key
        .verify(canonical.as_bytes(), &signature)
        .map_err(|_| AppError::Forbidden("Invalid event signature".to_string()))?;
    
    Ok(())
}

async fn is_duplicate_event(db: &Database, event_id: &Uuid) -> Result<bool> {
    // Could use Redis for better performance
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM global.event_verification_log WHERE metadata->>'event_id' = $1)",
        event_id.to_string()
    )
    .fetch_one(db.pool())
    .await?
    .unwrap_or(false);
    
    Ok(exists)
}

async fn exceeds_rate_limit(
    db: &Database,
    user_id: Uuid,
    max_events: i32,
    window_seconds: i32,
) -> Result<bool> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)::int as count
        FROM global.event_verification_log
        WHERE user_id = $1 
        AND verified = true
        AND timestamp > NOW() - INTERVAL '1 second' * $2
        "#,
        user_id,
        window_seconds
    )
    .fetch_one(db.pool())
    .await?
    .unwrap_or(0);
    
    Ok(count >= max_events)
}

async fn update_service_activity(db: &Database, service_name: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE global.service_publisher_registry SET last_event_at = NOW() WHERE service_name = $1",
        service_name
    )
    .execute(db.pool())
    .await?;
    
    Ok(())
}

async fn log_verification_success(
    db: &Database,
    service_name: &str,
    event_subject: &str,
    user_id: Uuid,
) {
    let _ = sqlx::query!(
        r#"
        INSERT INTO global.event_verification_log
        (service_name, event_subject, verified, user_id)
        VALUES ($1, $2, true, $3)
        "#,
        service_name,
        event_subject,
        user_id
    )
    .execute(db.pool())
    .await;
}

async fn log_verification_failure(
    db: &Database,
    service_name: &str,
    event_subject: &str,
    reason: &str,
    user_id: Uuid,
) {
    let _ = sqlx::query!(
        r#"
        INSERT INTO global.event_verification_log
        (service_name, event_subject, verified, failure_reason, user_id)
        VALUES ($1, $2, false, $3, $4)
        "#,
        service_name,
        event_subject,
        reason,
        user_id
    )
    .execute(db.pool())
    .await;
}
```

### Multi-Pod Considerations

In the Unity Platform's multi-pod architecture:

1. **Local Service Registration**
   - Denmark's course-service registers with Denmark's badge-service
   - Norway's course-service registers with Norway's badge-service
   - Each pod maintains independent service registry

2. **Cross-Pod Courses**
   - User in Denmark takes course from global registry
   - Denmark's course-service provides course locally
   - Denmark's course-service publishes progress events
   - Denmark's badge-service awards badges
   - **No cross-pod event publishing needed**

3. **Service Identity**
   - Service name includes territory: `course-service-dk`, `course-service-no`
   - Each territory instance has its own key pair
   - Prevents confusion in federated scenarios

### Defense-in-Depth with NATS ACLs

While cryptographic signatures provide strong authentication, NATS ACLs add an additional security layer:

```conf
# nats-server.conf
authorization {
    users = [
        {
            user: "course-service"
            password: "course_service_secret"
            permissions {
                publish = ["badge.progress.course-*"]
                subscribe = []
            }
        },
        {
            user: "forum-service"
            password: "forum_service_secret"
            permissions {
                publish = ["badge.progress.forum-*"]
                subscribe = []
            }
        },
        {
            user: "badge-service"
            password: "badge_service_secret"
            permissions {
                publish = ["badge.awarded", "badge.revoked"]
                subscribe = ["badge.progress.*"]
            }
        }
    ]
}
```

### Security Monitoring & Alerts

```rust
// Monitor for suspicious patterns
pub async fn security_monitoring_task(db: Database) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    
    loop {
        interval.tick().await;
        
        // Check for high failure rates
        let failure_rate = sqlx::query_scalar!(
            r#"
            SELECT 
                COUNT(*) FILTER (WHERE NOT verified)::float / 
                NULLIF(COUNT(*)::float, 0) as rate
            FROM global.event_verification_log
            WHERE timestamp > NOW() - INTERVAL '5 minutes'
            "#
        )
        .fetch_one(db.pool())
        .await
        .unwrap_or(Some(0.0));
        
        if let Some(rate) = failure_rate {
            if rate > 0.1 {
                tracing::error!(
                    "High event verification failure rate: {:.1}%",
                    rate * 100.0
                );
                // Send alert to monitoring system
            }
        }
        
        // Check for unusual event volumes per user
        let suspicious_users = sqlx::query!(
            r#"
            SELECT user_id, COUNT(*) as event_count
            FROM global.event_verification_log
            WHERE timestamp > NOW() - INTERVAL '1 hour'
            GROUP BY user_id
            HAVING COUNT(*) > 1000
            "#
        )
        .fetch_all(db.pool())
        .await
        .unwrap_or_default();
        
        for user in suspicious_users {
            tracing::warn!(
                "Suspicious event volume for user {}: {} events/hour",
                user.user_id,
                user.event_count
            );
        }
    }
}
```

### Service Registration Setup Guide

**1. Generate Key Pair (Service Setup)**

```bash
# Generate Ed25519 key pair
openssl genpkey -algorithm Ed25519 -out course-service-dk-private.pem
openssl pkey -in course-service-dk-private.pem -pubout -out course-service-dk-public.pem

# Store private key securely (environment variable or secrets manager)
export BADGE_EVENT_PRIVATE_KEY=$(cat course-service-dk-private.pem)
```

**2. Register with Badge Service (One-time Admin Action)**

```bash
PUBLIC_KEY=$(cat course-service-dk-public.pem)

curl -X POST https://dk.unityplan.org/api/v1/badges/register-publisher \
  -H "Authorization: Bearer $PLATFORM_MANAGER_JWT" \
  -H "Content-Type: application/json" \
  -d "{
    \"serviceName\": \"course-service-dk\",
    \"serviceType\": \"course\",
    \"publicKey\": \"$PUBLIC_KEY\"
  }"
```

**3. Configure Service (Environment Variables)**

```bash
# course-service .env
BADGE_EVENT_PUBLISHER_NAME=course-service-dk
BADGE_EVENT_PRIVATE_KEY_PATH=/run/secrets/course-service-dk-private.pem
NATS_URL=nats://localhost:4222
```

### Comparison: Security Levels

| Security Measure | Without Signatures | With Signatures |
|-----------------|-------------------|-----------------|
| **Malicious Service** | ✅ Can publish fake events | ❌ Signature fails |
| **Compromised NATS** | ✅ Can inject events | ❌ Signature fails |
| **Replay Attack** | ✅ Can replay old events | ❌ Timestamp check fails |
| **Service Impersonation** | ✅ Easy to impersonate | ❌ No private key |
| **Event Tampering** | ✅ Can modify in transit | ❌ Signature invalidated |
| **Audit Trail** | ❌ No source identity | ✅ Service name verified |
| **Rate Limiting** | Per user only | ✅ Per user + per service |
| **Revocation** | Not possible | ✅ Disable service in registry |

### Migration Path

**Phase 1: Add Registry (Non-Breaking)**

1. Create service_publisher_registry table
2. Create event_verification_log table
3. Add register-publisher endpoint
4. Services can optionally register

**Phase 2: Accept Both (Transition)**

1. Badge-service accepts both signed and unsigned events
2. Log warnings for unsigned events
3. Monitor adoption rate

**Phase 3: Require Signatures (Secure)**

1. Reject unsigned events
2. All services must be registered
3. Full security enforcement

### Best Practices Summary

1. ✅ **Use Ed25519 signatures** - Fast, secure, industry standard
2. ✅ **Timestamp all events** - Prevents replay attacks (5-minute window)
3. ✅ **Unique event_id** - Enables deduplication
4. ✅ **Rate limit by user** - Prevents abuse (100 events/hour)
5. ✅ **Log all verifications** - Security audit trail
6. ✅ **Monitor failures** - Alert on suspicious patterns
7. ✅ **NATS ACLs** - Defense-in-depth network security
8. ✅ **Service registry** - Centralized trust management
9. ✅ **Key rotation** - Update public keys as needed
10. ✅ **Separate keys per pod** - Independent trust domains

## Future Enhancements

### Complex Criteria

```sql
-- Support multiple event sources for one badge
ALTER TABLE global.badge_registry
ADD COLUMN criteria_config JSONB;

-- Example: Badge requires both course completion AND forum participation
{
  "requires_all": [
    {
      "event": "badge.progress.course-completion-10",
      "value": 10
    },
    {
      "event": "badge.progress.forum-contributor",
      "value": 50
    }
  ]
}
```

### Time-Based Criteria

```json
{
  "criteria": {
    "event": "badge.progress.streak-learner",
    "value": 7,
    "window": "7d",
    "description": "Complete 1 course per day for 7 consecutive days"
  }
}
```

### Leaderboard Integration

```json
{
  "criteria": {
    "event": "badge.progress.top-contributor",
    "rank": "top-10",
    "period": "monthly",
    "description": "Top 10 forum contributors this month"
  }
}
```

## Implementation Checklist

### Database Changes

- [ ] Add `criteria_event_subject` column to badge registry
- [ ] Create migration script
- [ ] Update seed badges with event subjects

### Code Changes

- [ ] Create `BadgeProgressEvent` model
- [ ] Implement `subscribe_badge_progress()` function
- [ ] Implement `evaluate_badge_criteria()` function
- [ ] Add subscription to `initialize_subscriptions()`
- [ ] Update award_badge() to handle system awards

### Testing

- [ ] Unit tests for criteria evaluation
- [ ] Integration tests for event subscription
- [ ] E2E test for complete flow

### Documentation

- [ ] Update API.md with event patterns
- [ ] Document badge configuration examples
- [ ] Add service integration guide

---

## Summary

The Badge Criteria Hook System provides a clean, event-driven approach to badge awarding that:

1. **Maintains Service Independence**: No direct service-to-service calls
2. **Enables Flexibility**: New badges without code changes
3. **Supports Scalability**: Async event processing via NATS
4. **Preserves Simplicity**: Clear event patterns and criteria evaluation

This design aligns with the platform's microservices architecture and user sovereignty principles by keeping services loosely coupled while enabling rich badge ecosystems.

---

**Next Steps:**

1. Review and approve design
2. Implement database migration
3. Implement subscription and evaluation logic
4. Create comprehensive tests
5. Document for service integrators
