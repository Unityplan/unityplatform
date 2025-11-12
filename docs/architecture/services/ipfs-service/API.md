# ipfs-service API Endpoints

**Base URL:** `http://localhost:8013`  
**Version:** v1  
**Status:** 📋 Planned (Future Phase - Decentralized Storage)

---

## File Management

### 1. Upload File

**Endpoint:** `POST /api/v1/ipfs/upload`  
**Authentication:** Bearer token required  
**Status:** 📋 Planned

**Request:**

```
POST /api/v1/ipfs/upload
Content-Type: multipart/form-data

file=@avatar.png
```

**Response:**

```json
{
  "success": true,
  "data": {
    "cid": "QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG",
    "url": "ipfs://QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG",
    "gateway_url": "https://ipfs.unityplatform.org/ipfs/QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG",
    "size_bytes": 45678,
    "mime_type": "image/png",
    "uploaded_at": "2025-11-12T10:00:00Z"
  }
}
```

---

### 2. Get File

**Endpoint:** `GET /api/v1/ipfs/{cid}`  
**Authentication:** None (public gateway)  
**Status:** 📋 Planned

**Response:** Binary file data

---

### 3. List User Files

**Endpoint:** `GET /api/v1/ipfs/files`  
**Authentication:** Bearer token required  
**Status:** 📋 Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "files": [
      {
        "cid": "QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG",
        "filename": "avatar.png",
        "size_bytes": 45678,
        "mime_type": "image/png",
        "uploaded_at": "2025-11-12T10:00:00Z",
        "pinned": true
      }
    ],
    "total_size_bytes": 45678,
    "quota_bytes": 104857600,
    "usage_percentage": 0.04
  }
}
```

---

### 4. Delete File (Unpin)

**Endpoint:** `DELETE /api/v1/ipfs/{cid}`  
**Authentication:** Bearer token required  
**Status:** 📋 Planned

**Note:** Unpins file from IPFS node. File remains on IPFS network if pinned elsewhere.

---

### 5. Get File Metadata

**Endpoint:** `GET /api/v1/ipfs/{cid}/metadata`  
**Status:** 📋 Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "cid": "QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG",
    "size_bytes": 45678,
    "mime_type": "image/png",
    "filename": "avatar.png",
    "uploaded_by": "uuid",
    "uploaded_at": "2025-11-12T10:00:00Z",
    "pin_count": 1,
    "is_pinned": true
  }
}
```

---

## IPFS Configuration

**IPFS Node:**

- Local IPFS daemon (go-ipfs)
- Data directory: `docker/ipfs-data/`
- API port: 5001
- Gateway port: 8080

**Storage Quotas:**

- Default: 100MB per user
- Upgradable: 1GB (future premium)

**Supported File Types:**

**Images:**

- JPEG, PNG, GIF, WebP
- Max size: 5MB
- Recommended: 500x500px for avatars

**Documents:**

- PDF, Markdown
- Max size: 10MB

**Archives:**

- ZIP, TAR
- Max size: 50MB

---

## Content Addressing

**CID (Content Identifier):**

- Example: `QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG`
- Hash-based (SHA-256)
- Same content = same CID (deduplication)
- Verifiable (integrity check)

**URI Format:**

- `ipfs://{cid}` - Protocol URI
- `https://ipfs.unityplatform.org/ipfs/{cid}` - Gateway URL
- `https://gateway.ipfs.io/ipfs/{cid}` - Public gateway

---

## Pinning Strategy

**Auto-Pin:**

- User avatars
- Profile images
- Course certificates
- Community logos

**Garbage Collection:**

- Unpinned files deleted after 30 days
- Users can manually unpin files

---

**Last Updated:** November 12, 2025  
**Implementation Status:** Future phase (decentralized storage)
