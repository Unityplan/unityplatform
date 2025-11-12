# ipfs-service

**Port:** 8013  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Planned - Phase 2  
**Bounded Context:** Decentralized File Storage

---

## 📋 Overview

The ipfs-service manages file uploads, storage, and retrieval using IPFS (InterPlanetary File System) for decentralized, content-addressed storage.

### **Responsibilities**

- ⏳ Upload files to IPFS
- ⏳ Retrieve files from IPFS (gateway)
- ⏳ Pin files (ensure availability)
- ⏳ Track file metadata (filename, size, owner)
- ⏳ File access control
- ⏳ Generate thumbnails (images)

### **Not Responsible For**

- ❌ Video transcoding (future: separate service)
- ❌ File scanning/virus detection (future: integration)
- ❌ CDN distribution (IPFS provides this natively)

---

## 🗄️ Database Schema (Planned)

```sql
CREATE TABLE territory_{code}.ipfs_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- IPFS
    cid VARCHAR(255) UNIQUE NOT NULL,  -- Content Identifier (IPFS hash)
    ipfs_url VARCHAR(500) NOT NULL,  -- ipfs://QmXxx or https://gateway.ipfs.io/ipfs/QmXxx
    
    -- Metadata
    filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size BIGINT NOT NULL,  -- Bytes
    
    -- Ownership
    uploaded_by UUID NOT NULL REFERENCES users(id),
    uploaded_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Access Control
    visibility VARCHAR(20) DEFAULT 'public',  -- public/private/community
    community_id UUID REFERENCES communities(id),  -- If community-scoped
    
    -- Pinning
    is_pinned BOOLEAN DEFAULT true,
    pin_expiry TIMESTAMPTZ  -- NULL = permanent
);

CREATE INDEX idx_ipfs_files_cid ON ipfs_files(cid);
CREATE INDEX idx_ipfs_files_uploaded_by ON ipfs_files(uploaded_by);
CREATE INDEX idx_ipfs_files_community ON ipfs_files(community_id) WHERE community_id IS NOT NULL;
CREATE INDEX idx_ipfs_files_pinned ON ipfs_files(is_pinned) WHERE is_pinned = true;
```

---

## 🔌 API Endpoints (Planned)

### **Upload File**

```http
POST /v1/ipfs/upload
Content-Type: multipart/form-data

file: <binary>
visibility: "public"
community_id: "uuid" (optional)
```

**Response:**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "cid": "QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco",
    "ipfs_url": "ipfs://QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco",
    "gateway_url": "https://ipfs.unityplatform.org/ipfs/QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco",
    "filename": "avatar.jpg",
    "mime_type": "image/jpeg",
    "file_size": 245760,
    "uploaded_at": "2025-11-12T10:00:00Z"
  }
}
```

---

### **Get File**

```http
GET /v1/ipfs/files/{cid}
```

**Response:** File metadata

```json
{
  "success": true,
  "data": {
    "cid": "QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco",
    "gateway_url": "https://ipfs.unityplatform.org/ipfs/QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco",
    "filename": "avatar.jpg",
    "mime_type": "image/jpeg",
    "file_size": 245760,
    "uploaded_by": "uuid",
    "visibility": "public"
  }
}
```

---

### **List User Files**

```http
GET /v1/ipfs/files/me
```

**Response:**

```json
{
  "success": true,
  "data": {
    "files": [
      {
        "id": "uuid",
        "cid": "QmXxx",
        "filename": "avatar.jpg",
        "mime_type": "image/jpeg",
        "file_size": 245760,
        "uploaded_at": "2025-11-12T10:00:00Z",
        "gateway_url": "https://ipfs.unityplatform.org/ipfs/QmXxx"
      }
    ],
    "total_size": 1048576,  // Total bytes used
    "file_count": 15
  }
}
```

---

### **Delete File (Unpin)**

```http
DELETE /v1/ipfs/files/{cid}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "unpinned": true,
    "cid": "QmXxx"
  }
}
```

**Note:** File remains on IPFS network (content-addressed), but no longer pinned by our node. If other nodes pin it, it stays available.

---

## 🔗 Service Dependencies

### **Outbound Calls**

#### **IPFS Node**

- **Connection:** Local IPFS node via HTTP API
- **Operations:** Add file, pin file, unpin file, get file
- **Endpoint:** <http://localhost:5001/api/v0>

---

### **Inbound Calls**

#### **user-service**

- **When:** User uploads avatar
- **Endpoint:** POST /v1/ipfs/upload
- **Purpose:** Store avatar on IPFS

#### **community-service**

- **When:** Community uploads banner/avatar
- **Purpose:** Store community images

#### **forum-service** (future)

- **When:** User attaches file to post
- **Purpose:** Store attachments

---

## 📡 NATS Events (Planned)

**Published:**

- file.uploaded
- file.deleted

**Subscribed:**

- user.deleted (unpin user's files)

---

## 🔮 IPFS Architecture

### **Content Addressing**

Traditional web:

- URL: <https://example.com/avatar.jpg> (location-based)
- Problem: If server goes down, file is lost

IPFS:

- CID: QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco (content-based)
- File is the same regardless of who hosts it
- If ANY node on IPFS has it, you can retrieve it

### **Pinning Strategy**

```
User uploads file
     ↓
ipfs-service adds to local IPFS node
     ↓
File is automatically pinned (won't be garbage collected)
     ↓
File propagates to IPFS network (DHT)
     ↓
Other users can retrieve from ANY node
     ↓
If we unpin, file stays if others pin it
```

### **IPFS Gateway**

```
User requests file: https://ipfs.unityplatform.org/ipfs/QmXxx
     ↓
Gateway queries IPFS network
     ↓
Retrieves file from nearest node
     ↓
Serves via HTTP
```

---

## 🛠️ IPFS Node Setup

### **Docker Compose**

```yaml
services:
  ipfs:
    image: ipfs/kubo:latest
    ports:
      - "5001:5001"  # API
      - "8080:8080"  # Gateway
      - "4001:4001"  # Swarm
    volumes:
      - ./docker/ipfs-data:/data/ipfs
    environment:
      - IPFS_PROFILE=server
```

### **Rust SDK**

```rust
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};

async fn upload_to_ipfs(file_bytes: Vec<u8>) -> Result<String> {
    let client = IpfsClient::default();
    let response = client.add(file_bytes).await?;
    Ok(response.hash)  // Returns CID
}

async fn pin_file(cid: &str) -> Result<()> {
    let client = IpfsClient::default();
    client.pin_add(cid, true).await?;
    Ok(())
}
```

---

## 🌍 Holochain Integration (Future)

### **Hybrid Approach: IPFS + Holochain**

- **IPFS:** Store large files (images, videos, documents)
- **Holochain:** Store file metadata, access control, ownership

```rust
#[hdk_entry_helper]
struct FileMetadata {
    cid: String,  // IPFS Content Identifier
    filename: String,
    mime_type: String,
    file_size: u64,
    uploaded_by: AgentPubKey,
    visibility: FileVisibility,
}
```

**Flow:**

1. Upload file to IPFS → get CID
2. Store CID + metadata in Holochain
3. Holochain validates ownership and access
4. User retrieves file from IPFS using CID

**Benefits:**

- Decentralized storage (IPFS)
- Sovereign ownership (Holochain)
- No central server (both are P2P)

---

## 📊 Storage Limits (Future)

### **Per-User Quotas**

```sql
CREATE TABLE territory_{code}.storage_quotas (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    max_storage_mb INT DEFAULT 100,  -- 100MB per user
    used_storage_mb BIGINT DEFAULT 0,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### **Quota Enforcement**

Before upload:

1. Check user's current storage usage
2. Check file size
3. If `used_storage_mb + file_size > max_storage_mb`, reject upload
4. Otherwise, allow and increment `used_storage_mb`

---

## 🔒 Access Control

### **Visibility Levels**

- **public** - Anyone can access (listed in IPFS network)
- **private** - Only uploader can access (requires authentication)
- **community** - Only community members can access

### **Private Files**

Private files are still on IPFS (public network), but:

1. CID is NOT shared publicly
2. Only authorized users get the CID from our API
3. Without CID, file is undiscoverable on IPFS

**Better privacy (future):** Encrypt files before uploading to IPFS

---

## 📈 Metrics & Monitoring

### **Key Metrics**

- Total files stored
- Total storage used (GB)
- IPFS node health (peers connected, DHT size)
- Upload success rate
- Gateway response time

### **Alerts**

- IPFS node offline
- Storage quota exceeded (territory-wide)
- High gateway latency

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Phase 2 - Not Yet Started  
**Dependencies:** user-service, community-service  
**Technology:** IPFS (Kubo node), ipfs-api crate  
**Future:** Holochain metadata + IPFS storage = decentralized sovereign file system
