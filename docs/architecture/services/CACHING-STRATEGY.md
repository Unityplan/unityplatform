# Caching Strategy - Unity Platform

**Version:** 0.1.0-alpha.1  
**Date:** November 12, 2025  
**Status:** Implementation Planning

---

## Overview

This document defines the caching strategy for Unity Platform services across different deployment phases.

---

## Phase 1: Single Pod Caching (Development)

### Cache Technologies

#### 1. **In-Memory Cache (Service-Local)**

Use `moka` crate for hot data within each service:

```rust
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

lazy_static! {
    static ref USER_CACHE: Cache<Uuid, User> = Cache::builder()
        .max_capacity(10_000)
        .time_to_live(Duration::from_secs(300))  // 5 minutes
        .build();
}

async fn get_user_cached(user_id: Uuid, pool: &PgPool) -> Result<User> {
    // Try cache first
    if let Some(user) = USER_CACHE.get(&user_id) {
        return Ok(user);
    }
    
    // Cache miss - query database
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await?;
    
    // Update cache
    USER_CACHE.insert(user_id, user.clone()).await;
    
    Ok(user)
}
```

#### 2. **Distributed Cache (Redis)**

Use Redis for data shared across services:

```rust
use redis::AsyncCommands;

async fn get_community_members_cached(
    community_id: Uuid,
    redis: &mut redis::aio::Connection,
    pool: &PgPool,
) -> Result<Vec<Uuid>> {
    let key = format!("community:{}:members", community_id);
    
    // Try Redis first
    if let Ok(members_json) = redis.get::<_, String>(&key).await {
        return Ok(serde_json::from_str(&members_json)?);
    }
    
    // Cache miss - query database
    let members = sqlx::query_scalar!(
        "SELECT user_id FROM community_members WHERE community_id = $1",
        community_id
    )
    .fetch_all(pool)
    .await?;
    
    // Update Redis with 1 minute TTL
    let members_json = serde_json::to_string(&members)?;
    redis.set_ex(&key, members_json, 60).await?;
    
    Ok(members)
}
```

### What to Cache

| Data Type | TTL | Cache Layer | Invalidation |
|-----------|-----|-------------|--------------|
| **User existence** | 60 seconds | In-memory | user.deleted event |
| **User profiles** | 5 minutes | In-memory | user.updated event |
| **Community memberships** | 1 minute | Redis | user.joined_community, user.left_community events |
| **Public community list** | 10 minutes | Redis | community.created, community.updated events |
| **Territory information** | 1 hour | In-memory | Rarely changes, manual invalidation |
| **Badge definitions** | 24 hours | In-memory | Manual invalidation only |

### Cache Keys

**Naming Convention:**

```
{service}:{territory}:{resource}:{id}:{field}
```

**Examples:**

```rust
// User profile
"user:DK:profile:550e8400-e29b-41d4-a716-446655440000"

// Community members list
"community:DK:members:123e4567-e89b-12d3-a456-426614174000"

// User's communities
"user:DK:communities:550e8400-e29b-41d4-a716-446655440000"
```

### Cache Invalidation

#### NATS Event-Based Invalidation

```rust
// Subscribe to user.updated events
let mut sub = nats.subscribe("user.updated").await?;

tokio::spawn(async move {
    while let Some(msg) = sub.next().await {
        if let Ok(event) = serde_json::from_slice::<UserUpdatedEvent>(&msg.payload) {
            // Invalidate in-memory cache
            USER_CACHE.invalidate(&event.user_id).await;
            
            // Invalidate Redis cache
            let key = format!("user:{}:profile:{}", event.territory_code, event.user_id);
            let _ = redis.del::<_, ()>(&key).await;
            
            tracing::info!("Cache invalidated for user {}", event.user_id);
        }
    }
});
```

#### TTL Expiration

```rust
// Automatic expiration after TTL
Cache::builder()
    .time_to_live(Duration::from_secs(300))  // 5 minutes
    .build();
```

---

## Phase 2: Multi-Pod Caching (Production)

### Extended Cache Technologies

#### 1. **In-Memory Cache (Pod-Local)**

Same as Phase 1, but optimized for high traffic:

```rust
lazy_static! {
    static ref USER_CACHE: Cache<Uuid, User> = Cache::builder()
        .max_capacity(100_000)  // Increased capacity
        .time_to_live(Duration::from_secs(600))  // 10 minutes
        .time_to_idle(Duration::from_secs(300))  // 5 minutes idle
        .build();
}
```

#### 2. **Distributed Cache (Redis Cluster)**

Redis cluster for cross-pod shared state:

```rust
// Connect to Redis cluster
let client = redis::cluster::ClusterClient::new(vec![
    "redis://dk-redis-1:6379",
    "redis://dk-redis-2:6379",
    "redis://dk-redis-3:6379",
])?;

let mut conn = client.get_async_connection().await?;
```

#### 3. **CDN Caching (Static Assets)**

CloudFlare/Bunny CDN for static assets:

- User avatars
- Community images
- Course thumbnails
- Badge icons

**Cache-Control Headers:**

```rust
// In IPFS service for avatar uploads
HttpResponse::Ok()
    .insert_header(("Cache-Control", "public, max-age=31536000, immutable"))
    .insert_header(("ETag", etag))
    .body(file_bytes)
```

### Extended Cache Data

| Data Type | TTL | Cache Layer | Invalidation | Phase 2 Addition |
|-----------|-----|-------------|--------------|------------------|
| **Cross-pod user lookups** | 10 minutes | Redis Cluster | Cross-pod events | ✅ New |
| **Territory routing** | 1 hour | In-memory | Manual/config change | ✅ New |
| **Badge definitions** | 24 hours | In-memory | Manual invalidation | Same as Phase 1 |
| **Course metadata** | 1 hour | Redis Cluster | course.updated event | ✅ New |
| **Static assets** | 1 year | CDN | URL versioning | ✅ New |

### Cross-Pod Cache Invalidation

```rust
// Norway pod publishes user.updated event
nats.publish("user.updated", UserUpdatedEvent {
    user_id,
    territory_code: "NO",
    fields_changed: vec!["display_name".to_string()],
}).await?;

// Denmark pod subscribes and invalidates cross-pod cache
let mut sub = nats.subscribe("user.updated").await?;

tokio::spawn(async move {
    while let Some(msg) = sub.next().await {
        if let Ok(event) = serde_json::from_slice::<UserUpdatedEvent>(&msg.payload) {
            // Invalidate cross-pod user cache
            let key = format!("user:{}:profile:{}", event.territory_code, event.user_id);
            
            // Remove from Redis cluster (visible to all pods)
            redis.del::<_, ()>(&key).await.ok();
            
            // Remove from local in-memory cache
            USER_CACHE.invalidate(&event.user_id).await;
            
            tracing::info!(
                territory = %event.territory_code,
                user_id = %event.user_id,
                "Cross-pod cache invalidated"
            );
        }
    }
});
```

### Cache Stampede Prevention

**Problem:** Multiple requests for the same data hit the database simultaneously when cache expires.

**Solution:** Lock-based recomputation

```rust
use tokio::sync::RwLock;
use std::sync::Arc;

lazy_static! {
    static ref RECOMPUTE_LOCKS: Arc<RwLock<HashMap<String, Arc<Mutex<()>>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
}

async fn get_with_stampede_prevention<T, F>(
    key: &str,
    cache: &Cache<String, T>,
    recompute_fn: F,
) -> Result<T>
where
    T: Clone + Send + Sync + 'static,
    F: Future<Output = Result<T>>,
{
    // Try cache first
    if let Some(value) = cache.get(key) {
        return Ok(value);
    }
    
    // Get or create lock for this key
    let lock = {
        let mut locks = RECOMPUTE_LOCKS.write().await;
        locks.entry(key.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    };
    
    // Only one request recomputes, others wait
    let _guard = lock.lock().await;
    
    // Check cache again (might have been populated by another request)
    if let Some(value) = cache.get(key) {
        return Ok(value);
    }
    
    // Recompute and cache
    let value = recompute_fn.await?;
    cache.insert(key.to_string(), value.clone()).await;
    
    Ok(value)
}
```

### Cache Warming

**Pre-populate frequently accessed data on service startup:**

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... initialize services ...
    
    // Warm up cache on startup
    tokio::spawn(async move {
        tracing::info!("Warming up cache...");
        
        // Load territory information
        let territories = fetch_all_territories(&pool).await?;
        for territory in territories {
            TERRITORY_CACHE.insert(territory.code.clone(), territory).await;
        }
        
        // Load badge definitions
        let badges = fetch_all_badges(&pool).await?;
        for badge in badges {
            BADGE_CACHE.insert(badge.id, badge).await;
        }
        
        tracing::info!("Cache warming complete");
        Ok::<_, Error>(())
    });
    
    // ... start server ...
}
```

---

## Cache Versioning

### Schema Changes

Include version in cache keys to invalidate all caches on schema changes:

```rust
const CACHE_VERSION: &str = "v2";

fn user_cache_key(user_id: Uuid) -> String {
    format!("{}:user:profile:{}", CACHE_VERSION, user_id)
}

// When schema changes, increment CACHE_VERSION
// Old cached data becomes unreachable automatically
```

### URL Versioning for Static Assets

```rust
// Include content hash in URL
let avatar_url = format!(
    "https://cdn.unityplatform.org/avatars/{}/{}.jpg",
    user_id,
    content_hash  // Changes when file changes
);

// CDN caches indefinitely, URL changes invalidate cache
```

---

## Monitoring Cache Performance

### Metrics

```rust
use prometheus::{IntCounter, Histogram};

lazy_static! {
    static ref CACHE_HITS: IntCounter = IntCounter::new(
        "cache_hits_total",
        "Total cache hits"
    ).unwrap();
    
    static ref CACHE_MISSES: IntCounter = IntCounter::new(
        "cache_misses_total",
        "Total cache misses"
    ).unwrap();
    
    static ref CACHE_LATENCY: Histogram = Histogram::new(
        "cache_latency_seconds",
        "Cache operation latency"
    ).unwrap();
}

async fn get_user_with_metrics(user_id: Uuid, pool: &PgPool) -> Result<User> {
    let start = Instant::now();
    
    let result = if let Some(user) = USER_CACHE.get(&user_id) {
        CACHE_HITS.inc();
        Ok(user)
    } else {
        CACHE_MISSES.inc();
        let user = fetch_user_from_db(user_id, pool).await?;
        USER_CACHE.insert(user_id, user.clone()).await;
        Ok(user)
    };
    
    CACHE_LATENCY.observe(start.elapsed().as_secs_f64());
    result
}
```

### Grafana Dashboard

Monitor:

- Cache hit ratio (hits / (hits + misses))
- Cache size (number of entries)
- Cache memory usage
- Invalidation rate
- Average cache latency

---

## Best Practices

### 1. **Cache Invalidation Patterns**

```rust
// ✅ Good: Event-based invalidation
nats.publish("user.updated", event).await?;

// ❌ Bad: Manual invalidation (error-prone)
USER_CACHE.invalidate(&user_id).await;
redis.del(&key).await?;
```

### 2. **TTL Selection**

```rust
// ✅ Good: Short TTL for frequently changing data
Cache::builder().time_to_live(Duration::from_secs(60))  // 1 minute

// ✅ Good: Long TTL for static data
Cache::builder().time_to_live(Duration::from_secs(86400))  // 24 hours

// ❌ Bad: No TTL (cache grows forever)
Cache::builder().build();  // No TTL!
```

### 3. **Cache Key Design**

```rust
// ✅ Good: Specific, hierarchical keys
"user:DK:profile:550e8400"
"community:NO:members:123e4567"

// ❌ Bad: Generic keys (collision risk)
"user_550e8400"
"members_123e4567"
```

### 4. **Error Handling**

```rust
// ✅ Good: Fallback to database on cache failure
match USER_CACHE.get(&user_id) {
    Some(user) => Ok(user),
    None => fetch_user_from_db(user_id, pool).await,
}

// ❌ Bad: Return error on cache miss
match USER_CACHE.get(&user_id) {
    Some(user) => Ok(user),
    None => Err(Error::CacheMiss),  // Wrong!
}
```

---

## Summary

### Phase 1 (Development)

- **In-memory cache** for hot data (`moka` crate)
- **Redis** for shared state across services
- **Event-based invalidation** via NATS
- **Simple TTL** expiration
- **Manual cache warming** not needed (low traffic)

### Phase 2 (Production)

- **Redis Cluster** for cross-pod shared state
- **CDN caching** for static assets
- **Cross-pod invalidation** via NATS JetStream
- **Cache stampede prevention** with locks
- **Cache warming** on service startup
- **Versioned cache keys** for schema changes
- **Metrics and monitoring** (Prometheus + Grafana)

---

**Related Documentation:**

- [Inter-Service Communication](./INTER-SERVICE-COMMUNICATION.md)
- [Middleware Guide](./shared-lib/MIDDLEWARE.md)
- [Error Handling](./ERROR-HANDLING.md)
- [Observability](./OBSERVABILITY.md)

---

**Last Updated:** November 12, 2025  
**Status:** Ready for implementation
