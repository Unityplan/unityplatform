# forum-service Database Schema

**Service:** forum-service  
**Port:** 8011  
**Database:** Global + Territory schemas (Hybrid: Matrix + PostgreSQL metadata)

---

## Architecture

**Matrix Protocol:** Real-time messaging and federation  
**PostgreSQL:** Metadata, indexing, search

**Data Flow:**

1. Messages stored in Matrix homeserver (Synapse)
2. Metadata (room info, participants) stored in PostgreSQL
3. Search index built from Matrix data

---

## Schema Distribution

### Global Schema (Forum Catalog)

**Table:** `global.forum_rooms` (Future)  
**Purpose:** Global forum room registry

```sql
CREATE TABLE global.forum_rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    matrix_room_id VARCHAR(255) NOT NULL UNIQUE,
    
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    
    visibility VARCHAR(20) NOT NULL,
    
    community_id UUID,
    community_territory VARCHAR(10),
    
    created_by UUID NOT NULL,
    creator_territory VARCHAR(10) NOT NULL,
    
    member_count INT NOT NULL DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (visibility IN ('public', 'private', 'hidden'))
);
```

**Why Global?** Forum rooms can be accessed across territories (federated discussions).

---

### Territory Schema (User Participation)

**Table:** `territory_{code}.forum_memberships` (Future)

```sql
CREATE TABLE territory_{code}.forum_memberships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    forum_room_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_read_at TIMESTAMPTZ,
    
    UNIQUE (forum_room_id, user_id)
);
```

**Why Territory?** User's forum participation is personal data.

---

## Matrix Integration

**Matrix Homeserver:** Runs on `matrix.unityplatform.org`  
**Database:** Separate Synapse PostgreSQL database

**PostgreSQL Tables (Synapse):**

- `rooms` - Room metadata
- `events` - Messages (JSON)
- `room_memberships` - Participants
- `room_aliases` - Room names

**Unity Platform Tables:**

- Metadata index (for fast search)
- User participation tracking
- Cross-territory room registry

---

## Holochain Migration

**Current:** Matrix (real-time) + PostgreSQL (metadata)  
**Future:** Matrix (real-time) + Holochain (permanent storage)

**Benefits:**

- Messages cryptographically signed (Holochain)
- Permanent archive (DHT storage)
- User owns message history (source chain)

---

## Multi-Pod Considerations

**Forum Room Location:**

- Public rooms → global schema (accessible to all)
- Community rooms → territory schema (local to community)

**Message Storage:**

- Matrix homeserver (federated)
- Holochain DHT (permanent archive)

**Cross-Territory Participation:**
Users from any territory can join global forums. Matrix handles federation.

---

## NATS Events

**Publishes:** `forum.room_created`, `forum.message_posted`, `forum.user_joined`

---

**Last Updated:** November 12, 2025
