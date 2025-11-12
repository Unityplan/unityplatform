# Inter-Service Communication - Unity Platform

**Version:** 0.1.0-alpha.1  
**Date:** November 12, 2025  
**Status:** Implementation Planning

---

## Overview

This document defines how services communicate with each other in the Unity Platform across different deployment phases.

---

## Phase 1: Single Pod Communication (Development)

### Service Communication Patterns

#### 1. **Same-Pod Service Communication**

All services run in the same pod (Denmark) without a gateway.

**JWT Validation (No API Calls):**

```rust
// Services validate JWTs independently using shared secret
// No need to call auth-service for every request

use shared_lib::auth::validate_jwt;

async fn protected_handler(req: HttpRequest) -> Result<HttpResponse> {
    let token = extract_token_from_header(&req)?;
    let claims = validate_jwt(&token)?;  // Validates using shared secret
    
    // Use claims.user_id for authorization
    Ok(HttpResponse::Ok().finish())
}
```

**Direct Database Queries (Rare):**

```rust
// Generally avoid cross-service database queries
// Use NATS events instead for loose coupling

// Only in exceptional cases:
async fn check_user_exists(pool: &PgPool, user_id: Uuid) -> Result<bool> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1 AND deleted_at IS NULL)",
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(exists.unwrap_or(false))
}
```

**NATS Events (Preferred):**

```rust
// Publish event when user is deleted
await nats.publish("user.deleted", UserDeletedEvent {
    user_id,
    deleted_by,
    territory_code: "DK",
}).await?;

// Other services subscribe and react
let mut sub = nats.subscribe("user.deleted").await?;
while let Some(msg) = sub.next().await {
    let event: UserDeletedEvent = serde_json::from_slice(&msg.payload)?;
    
    // Clean up user data in this service
    cleanup_user_data(event.user_id).await?;
}
```

#### 2. **Event-Driven Patterns**

**User Lifecycle Events:**

```rust
// user-service publishes
"user.created"   → badge-service, notification-service subscribe
"user.updated"   → cache invalidation across services
"user.deleted"   → cleanup in all services
```

**Community Events:**

```rust
// community-service publishes
"community.created"       → notification-service notifies members
"user.joined_community"   → badge-service checks achievement
"user.left_community"     → update member counts
```

**Best Practices:**

- Events are fire-and-forget (async)
- No response expected from subscribers
- Events include all necessary data (avoid follow-up queries)
- Event ordering is best-effort in Phase 1

---

## Phase 2: Multi-Pod Communication (Production)

### Cross-Pod Communication Patterns

#### 1. **Cross-Pod HTTP Calls**

When users from different territories interact:

```rust
// Alice (DK pod) wants to view Bob's profile (NO pod)
async fn get_cross_pod_user_profile(
    user_id: Uuid,
    territory_code: &str,
) -> Result<UserProfile> {
    // Determine which pod hosts the user
    let pod_url = match territory_code {
        "NO" => "https://no.unityplatform.org",
        "SE" => "https://se.unityplatform.org",
        "EU" => "https://eu.unityplatform.org",
        _ => return Err(Error::UnknownTerritory),
    };
    
    // Make HTTP call with circuit breaker
    CROSS_POD_CIRCUIT.call(async {
        let client = reqwest::Client::new();
        client.get(&format!("{}/api/v1/users/{}", pod_url, user_id))
            .header("X-Request-ID", request_id)
            .send()
            .await?
            .json::<UserProfile>()
            .await
    }).await
}
```

#### 2. **API Gateway Routing**

**Traefik Configuration:**

```yaml
# Route requests based on subdomain to correct pod
http:
  routers:
    denmark-router:
      rule: "Host(`dk.unityplatform.org`)"
      service: denmark-pod
      
    norway-router:
      rule: "Host(`no.unityplatform.org`)"
      service: norway-pod
      
  services:
    denmark-pod:
      loadBalancer:
        servers:
          - url: "http://denmark-user-service:8002"
          - url: "http://denmark-user-service-replica:8002"
```

#### 3. **Circuit Breaker Implementation**

```rust
use lazy_static::lazy_static;

lazy_static! {
    static ref CROSS_POD_CIRCUIT: CircuitBreaker = CircuitBreaker::new(
        5,                          // 5 consecutive failures
        Duration::from_secs(30)     // 30 second timeout
    );
}

async fn resilient_cross_pod_call() -> Result<UserProfile> {
    CROSS_POD_CIRCUIT.call(async {
        // Make HTTP call
        fetch_from_remote_pod().await
    }).await
    .or_else(|e| match e {
        CircuitBreakerError::CircuitOpen => {
            // Fallback to cached data
            get_cached_user_profile().await
        }
        CircuitBreakerError::RequestFailed(e) => Err(e),
    })
}
```

#### 4. **Retry Strategies**

```rust
use tokio::time::{sleep, Duration};

async fn retry_with_backoff<F, T>(
    f: F,
    max_retries: u32,
) -> Result<T>
where
    F: Fn() -> Pin<Box<dyn Future<Output = Result<T>>>>,
{
    let mut attempt = 0;
    
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_retries => {
                attempt += 1;
                let delay = Duration::from_millis(100 * 2u64.pow(attempt));
                tracing::warn!(
                    "Request failed (attempt {}), retrying in {:?}",
                    attempt,
                    delay
                );
                sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

#### 5. **Request ID Propagation**

```rust
// Extract request ID from incoming request
let request_id = req.headers()
    .get("X-Request-ID")
    .and_then(|h| h.to_str().ok())
    .map(|s| s.to_string())
    .unwrap_or_else(|| Uuid::new_v4().to_string());

// Propagate to cross-pod calls
client.get(&remote_url)
    .header("X-Request-ID", &request_id)
    .send()
    .await?;

// Also propagate in NATS events
let event = UserDeletedEvent {
    event_id: Uuid::new_v4(),
    request_id: Some(request_id),
    // ... rest of event
};
```

#### 6. **Cross-Pod Event Distribution**

```rust
// NATS JetStream for persistent, ordered events
let js = nats.jetstream();

// Create stream for cross-pod events
js.create_stream(&StreamConfig {
    name: "USER_EVENTS".to_string(),
    subjects: vec!["user.*".to_string()],
    retention: RetentionPolicy::WorkQueue,
    max_age: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
    storage: StorageType::File,
    ..Default::default()
}).await?;

// Publish with acknowledgment
js.publish("user.deleted", event_bytes).await?.await?;

// Subscribe with consumer
let consumer = js.create_consumer(&ConsumerConfig {
    durable_name: Some("norway-pod-consumer".to_string()),
    ack_policy: AckPolicy::Explicit,
    ..Default::default()
}).await?;
```

---

## Event Schema Standards

### Event Structure

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event<T> {
    pub event_id: Uuid,
    pub event_type: String,
    pub version: String,
    pub timestamp: i64,
    pub territory_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,  // For distributed tracing
    pub payload: T,
}
```

### Event Types

| Event Type | Publisher | Subscribers | Purpose |
|------------|-----------|-------------|---------|
| `user.created` | user-service | badge-service, notification-service | Initialize user in dependent services |
| `user.updated` | user-service | All services | Cache invalidation |
| `user.deleted` | user-service | All services | Cleanup user data |
| `community.created` | community-service | notification-service | Notify members |
| `user.joined_community` | community-service | badge-service | Check achievements |
| `invitation.created` | invitation-service | notification-service | Send invitation notification |
| `invitation.accepted` | invitation-service | user-service, badge-service | Create user, award badge |

### Event Publishing Best Practices

1. **Include all necessary data** in the event payload
2. **Use versioning** for schema evolution
3. **Add request_id** for distributed tracing (Phase 2)
4. **Territory isolation** - include territory_code
5. **Idempotent handlers** - events may be delivered multiple times

---

## Synchronous vs Asynchronous Communication

### Use HTTP (Synchronous) When

- ✅ Cross-pod user lookups (Alice DK → Bob NO)
- ✅ Real-time data required
- ✅ Client needs immediate response
- ✅ Strong consistency required

**Example:**

```rust
// User wants to view another user's profile
GET /api/v1/users/{id}  // May require cross-pod call
```

### Use NATS Events (Asynchronous) When

- ✅ Internal pod notifications
- ✅ Cache invalidation
- ✅ Eventual consistency acceptable
- ✅ Fire-and-forget operations
- ✅ Multiple services need the same data

**Example:**

```rust
// User profile updated → invalidate caches everywhere
publish("user.updated", event);  // Multiple services subscribe
```

### Trade-offs

| Aspect | HTTP (Sync) | NATS Events (Async) |
|--------|-------------|---------------------|
| **Latency** | Higher (network + processing) | Lower (fire-and-forget) |
| **Consistency** | Strong (immediate) | Eventual (delayed) |
| **Coupling** | Tight (caller knows callee) | Loose (pub/sub) |
| **Reliability** | Requires retry logic | JetStream guarantees delivery |
| **Error Handling** | Immediate (circuit breaker) | Deferred (dead letter queue) |
| **Scalability** | Limited by network | High (message queue) |

---

## Caching Cross-Pod Data

### Cache Strategy

```rust
// Try cache first, then cross-pod call
async fn get_user_profile_with_cache(
    user_id: Uuid,
    territory_code: &str,
) -> Result<UserProfile> {
    // 1. Try in-memory cache
    if let Some(profile) = MEMORY_CACHE.get(&user_id).await {
        return Ok(profile);
    }
    
    // 2. Try Redis cache
    if let Some(profile) = redis_get_user(&user_id).await? {
        MEMORY_CACHE.insert(user_id, profile.clone()).await;
        return Ok(profile);
    }
    
    // 3. Cross-pod HTTP call
    let profile = get_cross_pod_user_profile(user_id, territory_code).await?;
    
    // 4. Update caches
    redis_set_user(&user_id, &profile, 600).await?;  // 10 min TTL
    MEMORY_CACHE.insert(user_id, profile.clone()).await;
    
    Ok(profile)
}
```

### Cache Invalidation via Events

```rust
// Subscribe to user.updated events
let mut sub = nats.subscribe("user.updated").await?;

while let Some(msg) = sub.next().await {
    let event: UserUpdatedEvent = serde_json::from_slice(&msg.payload)?;
    
    // Invalidate cache
    MEMORY_CACHE.remove(&event.user_id).await;
    redis_del_user(&event.user_id).await?;
    
    tracing::info!("Cache invalidated for user {}", event.user_id);
}
```

---

## Service Discovery

### Phase 1: Hardcoded Endpoints

```rust
// Services know each other's localhost ports
const USER_SERVICE_URL: &str = "http://localhost:8002";
const NOTIFICATION_SERVICE_URL: &str = "http://localhost:8007";
```

### Phase 2: Environment-Based Discovery

```rust
// Environment variables for pod URLs
let pod_urls = HashMap::from([
    ("DK", env::var("DK_POD_URL").unwrap_or("https://dk.unityplatform.org".to_string())),
    ("NO", env::var("NO_POD_URL").unwrap_or("https://no.unityplatform.org".to_string())),
    ("SE", env::var("SE_POD_URL").unwrap_or("https://se.unityplatform.org".to_string())),
    ("EU", env::var("EU_POD_URL").unwrap_or("https://eu.unityplatform.org".to_string())),
]);
```

### Phase 2: Kubernetes Service Discovery (Future)

```yaml
# Kubernetes services provide automatic DNS
http://user-service.denmark.svc.cluster.local:8002
http://user-service.norway.svc.cluster.local:8002
```

---

## Error Handling

### Cross-Pod Call Failures

```rust
match get_cross_pod_user_profile(user_id, territory_code).await {
    Ok(profile) => Ok(profile),
    Err(CircuitBreakerError::CircuitOpen) => {
        // Circuit is open - use cached data or return error
        get_cached_profile(user_id)
            .await
            .or(Err(Error::ServiceUnavailable))
    }
    Err(CircuitBreakerError::RequestFailed(e)) => {
        tracing::error!("Cross-pod call failed: {}", e);
        Err(Error::RemoteServiceError(e.to_string()))
    }
}
```

### Event Processing Failures

```rust
// Handle event processing errors gracefully
match process_user_deleted_event(&event).await {
    Ok(_) => {
        // Acknowledge event
        msg.ack().await?;
    }
    Err(e) => {
        tracing::error!("Failed to process event: {}", e);
        // Negative acknowledge - will be redelivered
        msg.nak().await?;
    }
}
```

---

## Summary

### Phase 1 (Development)

- **No cross-pod calls** (single Denmark pod)
- **JWT validation** using shared secret
- **NATS events** for service communication
- **Direct database** queries avoided (prefer events)
- **Best-effort** event ordering

### Phase 2 (Production)

- **Cross-pod HTTP calls** with circuit breakers
- **Request ID propagation** for distributed tracing
- **Retry logic** with exponential backoff
- **NATS JetStream** for persistent events
- **Cache invalidation** via events
- **API Gateway** (Traefik) for routing

---

**Related Documentation:**

- [Middleware Guide](./shared-lib/MIDDLEWARE.md)
- [Caching Strategy](./CACHING-STRATEGY.md)
- [Error Handling](./ERROR-HANDLING.md)
- [Observability](./OBSERVABILITY.md)

---

**Last Updated:** November 12, 2025  
**Status:** Ready for implementation
