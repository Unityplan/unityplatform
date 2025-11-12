# forum-service

**Port:** 8011  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Planned - Phase 2  
**Bounded Context:** Forum & Discussions (Matrix Integration)

---

## 📋 Overview

The forum-service integrates with Matrix protocol to provide decentralized forum functionality for communities.

### **Responsibilities**

- ⏳ Create Matrix rooms for communities
- ⏳ Bridge Matrix rooms to web interface
- ⏳ Forum post creation and replies
- ⏳ Thread moderation
- ⏳ Search and discovery
- ⏳ Matrix user mapping

### **Not Responsible For**

- ❌ Real-time chat (Matrix handles this natively)
- ❌ Matrix server management (Synapse/Dendrite)
- ❌ E2E encryption (Matrix handles this)

---

## 🗄️ Database Schema (Planned)

```sql
CREATE TABLE territory_{code}.forum_rooms (
    id UUID PRIMARY KEY,
    community_id UUID REFERENCES communities(id),
    matrix_room_id VARCHAR(255) UNIQUE NOT NULL,  -- !abc123:matrix.org
    room_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE territory_{code}.forum_posts (
    id UUID PRIMARY KEY,
    room_id UUID REFERENCES forum_rooms(id),
    matrix_event_id VARCHAR(255) UNIQUE NOT NULL,
    author_id UUID REFERENCES users(id),
    content TEXT NOT NULL,
    parent_post_id UUID REFERENCES forum_posts(id),  -- For replies
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE territory_{code}.matrix_user_mapping (
    user_id UUID REFERENCES users(id),
    matrix_user_id VARCHAR(255) UNIQUE NOT NULL,  -- @alice:matrix.org
    access_token TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id)
);
```

---

## 🔌 API Endpoints (Planned)

- POST /v1/forum/rooms - Create forum room (for community)
- GET /v1/forum/rooms/{community_id} - Get community forum room
- POST /v1/forum/posts - Create post (sends to Matrix)
- GET /v1/forum/posts - List posts (reads from Matrix)
- POST /v1/forum/posts/{id}/reply - Reply to post
- DELETE /v1/forum/posts/{id} - Delete post (moderation)
- GET /v1/forum/search - Search posts

---

## 🔗 Matrix Integration

### **Architecture**

```
Frontend (Web UI)
       ↓
forum-service (REST API)
       ↓
Matrix Homeserver (Synapse/Dendrite)
       ↓
Matrix Federation Network
```

### **User Flow**

1. User creates account on UnityPlan
2. forum-service creates Matrix account (same username)
3. User joins community
4. forum-service creates/joins Matrix room for community
5. User posts to forum via web UI
6. forum-service sends Matrix message
7. Matrix syncs to all federated servers
8. Other users see post (via Matrix or web UI)

### **Matrix SDK Integration**

```rust
use matrix_sdk::{Client, Room, RoomId};

async fn create_forum_room(community_name: &str) -> Result<Room> {
    let client = get_matrix_client().await?;
    let room = client.create_room(create_room::Request::new())
        .name(community_name)
        .visibility(Visibility::Public)
        .await?;
    Ok(room)
}

async fn send_forum_post(room_id: &RoomId, content: &str) -> Result<()> {
    let client = get_matrix_client().await?;
    let room = client.get_room(room_id).unwrap();
    room.send(RoomMessageEventContent::text_plain(content)).await?;
    Ok(())
}
```

---

## 📡 NATS Events (Planned)

**Published:**

- forum.room_created
- forum.post_created
- forum.post_deleted

**Subscribed:**

- community.created (create forum room automatically)
- community.deleted (archive forum room)

---

## 🔮 Holochain Migration (Future)

### **Key Concept: Dual Protocol**

Matrix provides real-time federation, Holochain provides sovereignty:

1. **Matrix:** Real-time messaging, federation, E2E encryption
2. **Holochain:** Permanent storage, user sovereignty, cryptographic signatures

```rust
#[hdk_entry_helper]
struct ForumPost {
    content: String,
    author: AgentPubKey,
    community_hash: EntryHash,
    parent_post: Option<EntryHash>,
    created_at: Timestamp,
    matrix_event_id: Option<String>,  // Bridge to Matrix
}
```

**Hybrid Approach:**

- Ephemeral chat → Matrix
- Permanent posts → Holochain + Matrix
- User data → Holochain
- Real-time sync → Matrix

---

## 🛠️ Implementation Notes

### **Matrix Homeserver**

Options:

1. **Synapse** (Python, mature, resource-heavy)
2. **Dendrite** (Go, lighter, still beta)
3. **Conduit** (Rust, experimental)

Recommendation: Start with Synapse, migrate to Conduit when stable

### **Authentication Bridge**

```rust
// SSO integration: UnityPlan login → Matrix login
async fn provision_matrix_user(username: &str, password: &str) -> Result<String> {
    let client = Client::builder()
        .homeserver_url("https://matrix.unityplan.org")
        .build()
        .await?;
    
    let response = client.register(username, password).await?;
    Ok(response.access_token)
}
```

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Phase 2 - Not Yet Started  
**Dependencies:** community-service, user-service  
**Technology:** Matrix Protocol (Synapse/Dendrite homeserver)  
**Future:** Holochain + Matrix hybrid for sovereignty + real-time
