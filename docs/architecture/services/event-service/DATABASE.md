# event-service Database Schema

**Service:** event-service  
**Port:** 8009  
**Database:** Territory schema only

---

## Tables

### territory_{code}.community_events

**Status:** ✅ Exists in 20251111000001_mvp_core_schema.sql

```sql
CREATE TABLE territory_{code}.community_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    community_id UUID NOT NULL REFERENCES territory_{code}.communities(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    title VARCHAR(255) NOT NULL,
    description TEXT,
    
    event_type VARCHAR(20) NOT NULL,
    
    location VARCHAR(500),
    online_url VARCHAR(500),
    
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    
    max_attendees INT,
    
    is_published BOOLEAN NOT NULL DEFAULT false,
    is_cancelled BOOLEAN NOT NULL DEFAULT false,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (event_type IN ('online', 'in_person', 'hybrid')),
    CHECK (end_time > start_time)
);
```

### territory_{code}.event_rsvps

**Status:** ✅ Exists in core migration

```sql
CREATE TABLE territory_{code}.event_rsvps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    event_id UUID NOT NULL REFERENCES territory_{code}.community_events(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    rsvp_status VARCHAR(20) NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (event_id, user_id),
    CHECK (rsvp_status IN ('going', 'interested', 'not_going'))
);
```

## Data Sovereignty

Events are **territory-specific**. Users can only attend events in their territory (for now).

## Future: Cross-Territory Events

Global events (conferences) require federation - event stored in global schema, RSVPs in territory schemas.

## NATS Events

**Publishes:** `event.created`, `event.updated`, `rsvp.submitted`

---

**Last Updated:** November 12, 2025
