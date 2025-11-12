# ipfs-service Database Schema

**Service:** ipfs-service  
**Port:** 8013  
**Database:** Territory schema only

---

## Table

### territory_{code}.file_uploads

**Status:** ✅ Exists in 20251111000001_mvp_core_schema.sql (shared with user-service)

```sql
CREATE TABLE territory_{code}.file_uploads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    user_id UUID NOT NULL REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    -- File Info
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    
    -- Storage
    storage_type VARCHAR(20) NOT NULL,
    storage_path VARCHAR(500) NOT NULL,
    ipfs_hash VARCHAR(128),
    
    -- Context
    upload_context VARCHAR(50) NOT NULL,
    
    -- Metadata
    metadata JSONB,
    
    -- Soft Delete
    deleted_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (storage_type IN ('ipfs', 's3', 'local')),
    CHECK (upload_context IN ('avatar', 'profile_image', 'community_logo', 'event_image', 'course_material'))
);

CREATE INDEX idx_file_uploads_user ON territory_{code}.file_uploads(user_id);
CREATE INDEX idx_file_uploads_ipfs ON territory_{code}.file_uploads(ipfs_hash) WHERE ipfs_hash IS NOT NULL;
CREATE INDEX idx_file_uploads_context ON territory_{code}.file_uploads(upload_context);
```

## Data Sovereignty

**File Ownership:** Files are owned by users - metadata stored in their territory.

**IPFS Storage:**

- Content-addressed (CID)
- Pinned on territory's IPFS node
- Public gateway: `https://ipfs.unityplan.org/ipfs/{cid}`

---

## Storage Quotas (Future)

**Table:** `territory_{code}.user_storage_quotas`

```sql
CREATE TABLE territory_{code}.user_storage_quotas (
    user_id UUID PRIMARY KEY REFERENCES territory_{code}.users(id) ON DELETE CASCADE,
    
    quota_bytes BIGINT NOT NULL DEFAULT 104857600,
    used_bytes BIGINT NOT NULL DEFAULT 0,
    
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CHECK (used_bytes <= quota_bytes)
);
```

**Default Quota:** 100MB per user  
**Future:** Upgradable quotas (premium)

---

## Multi-Pod Strategy

**IPFS Node:** Each territory has its own IPFS node  
**Pinning:** Files pinned on user's home territory  
**Access:** Any territory can fetch via IPFS CID (decentralized)

**Cross-Territory File Access:**

- Alice (Denmark) uploads avatar → Pinned on Denmark IPFS node
- Bob (Norway) views Alice's profile → Fetches avatar via CID from Denmark IPFS node
- IPFS handles peer discovery and content routing

---

## Garbage Collection

**Unpinning Strategy:**

- User deletes file → Mark `deleted_at`
- After 30 days → Unpin from IPFS node
- Content may still exist on IPFS network (if others pinned it)

---

## NATS Events

**Publishes:** `file.uploaded`, `file.deleted`

---

**Last Updated:** November 12, 2025
