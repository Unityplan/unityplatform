# forum-service API Endpoints

**Base URL:** `http://localhost:8011`  
**Version:** v1  
**Status:** 📋 Planned (Future Phase - Matrix Integration)

---

## Matrix-Based Forum

**Architecture:** Hybrid Matrix + Holochain

- **Matrix:** Real-time messaging, federation
- **Holochain:** Permanent storage, cryptographic verification

---

## Room Management

### 1. Create Forum Room

**Endpoint:** `POST /api/v1/forums/rooms`  
**Status:** 📋 Planned

**Request:**

```json
{
  "name": "Rust Help",
  "description": "Ask questions about Rust",
  "visibility": "public",
  "community_id": "uuid"
}
```

**Actions:**

- Creates Matrix room
- Stores metadata in Holochain DHT

---

### 2. List Rooms

**Endpoint:** `GET /api/v1/forums/rooms`  
**Status:** 📋 Planned

---

## Post Management

### 3. Create Post

**Endpoint:** `POST /api/v1/forums/rooms/{room_id}/posts`  
**Status:** 📋 Planned

**Request:**

```json
{
  "title": "How to handle errors in Rust?",
  "content": "I'm new to Rust and confused about Result and Option types...",
  "tags": ["rust", "error-handling"]
}
```

**Actions:**

- Send Matrix message
- Store in Holochain with cryptographic signature

---

### 4. Reply to Post

**Endpoint:** `POST /api/v1/forums/posts/{post_id}/replies`  
**Status:** 📋 Planned

---

### 5. Get Post Thread

**Endpoint:** `GET /api/v1/forums/posts/{post_id}`  
**Status:** 📋 Planned  
**Source:** Holochain DHT (permanent storage)

---

### 6. Search Posts

**Endpoint:** `GET /api/v1/forums/search?q=rust+error`  
**Status:** 📋 Planned

---

## Moderation

### 7. Report Post

**Endpoint:** `POST /api/v1/forums/posts/{post_id}/report`  
**Status:** 📋 Planned

---

### 8. Delete Post

**Endpoint:** `DELETE /api/v1/forums/posts/{post_id}`  
**Status:** 📋 Planned  
**Note:** Soft delete (marks as deleted, preserves history)

---

## Matrix Integration

**Matrix SDK:** `matrix-sdk` (Rust)

**Matrix Homeserver:**

- Runs on `matrix.unityplatform.org`
- Federation enabled (connect with other Matrix servers)
- Single Sign-On with auth-service

**Real-time Features:**

- Typing indicators
- Read receipts
- Reactions (emoji)
- File attachments

---

**Last Updated:** November 12, 2025  
**Implementation Status:** Future phase (Matrix + Holochain hybrid)
