# invitation-service API Endpoints

**Base URL:** `http://localhost:8004`  
**Version:** v1  
**Status:** ⏳ Scaffolded (0/6 endpoints)

---

## Invitation Endpoints

### 1. Validate Invitation Token

**Endpoint:** `POST /api/v1/invitations/validate`  
**Authentication:** None (public, called by auth-service)  
**Status:** ⏳ Planned

**Description:** Validate if an invitation token is usable (called during registration).

**Request Body:**

```json
{
  "token": "A7K9-M2X4-P5W8-Q1Z3"
}
```

**Response (200 OK) - Valid:**

```json
{
  "success": true,
  "data": {
    "valid": true,
    "invitation_id": "uuid",
    "created_by": "uuid",
    "uses_remaining": 0,
    "expires_at": null,
    "metadata": {}
  }
}
```

**Response (400 Bad Request) - Invalid:**

```json
{
  "success": false,
  "error": {
    "code": "INVALID_INVITATION",
    "message": "Invitation token is invalid, expired, or fully used"
  }
}
```

**Validation Rules:**

- Token exists in database
- `is_active = true`
- `expires_at > NOW()` (or NULL)
- `uses_count < max_uses` (or `max_uses = 0` for unlimited)

---

### 2. Mark Invitation as Used

**Endpoint:** `POST /api/v1/invitations/use`  
**Authentication:** None (internal, called by auth-service)  
**Status:** ⏳ Planned

**Description:** Mark invitation as used after successful registration.

**Request Body:**

```json
{
  "token": "A7K9-M2X4-P5W8-Q1Z3",
  "used_by": "uuid",
  "ip_address": "192.168.1.100",
  "user_agent": "Mozilla/5.0..."
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "invitation_id": "uuid",
    "uses_remaining": 0,
    "fully_used": true
  }
}
```

**Actions:**

- Insert into `invitation_uses`
- Increment `invitation_tokens.uses_count`
- If `uses_count >= max_uses`, set `is_active = false`
- Publish `invitation.used` NATS event

---

### 3. Create Invitation

**Endpoint:** `POST /api/v1/invitations`  
**Authentication:** Bearer token required  
**Authorization:** Manager role required (territory or community manager)  
**Status:** ⏳ Planned

**Description:** Create a new invitation token. **Production: Only managers can create invitations.**

**Request Body:**

```json
{
  "max_uses": 1,
  "expires_in_days": 7,
  "metadata": {
    "purpose": "friend_invite",
    "community_id": "uuid"
  }
}
```

**Parameters:**

- `max_uses`: `1` = single-use, `0` = unlimited, `N` = N uses
- `expires_in_days`: `7` = expires in 7 days, `null` = never expires
- `metadata`: Optional custom data

**Response (201 Created):**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "token": "A7K9-M2X4-P5W8-Q1Z3",
    "created_by": "uuid",
    "max_uses": 1,
    "expires_at": "2025-11-19T10:00:00Z",
    "invite_url": "https://unityplatform.org/join?invite=A7K9-M2X4-P5W8-Q1Z3"
  }
}
```

**Token Format:**

- 16 random alphanumeric characters
- Formatted as `XXXX-XXXX-XXXX-XXXX`
- Globally unique (checked against global registry)
- Excludes confusing characters (0, O, I, 1)

**NATS Event Published:** `invitation.created`

---

### 4. List My Invitations

**Endpoint:** `GET /api/v1/invitations/me`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Description:** Get all invitations created by the authenticated user.

**Query Parameters:**

- `status` - Filter by status (`active`, `used`, `expired`, `revoked`)
- `page` - Page number (default: 1)
- `limit` - Results per page (default: 20, max: 100)

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "invitations": [
      {
        "id": "uuid",
        "token": "A7K9-M2X4-P5W8-Q1Z3",
        "max_uses": 1,
        "uses_count": 1,
        "is_active": false,
        "expires_at": null,
        "created_at": "2025-11-10T10:00:00Z",
        "status": "used",
        "uses": [
          {
            "used_by": "uuid",
            "username": "alice",
            "used_at": "2025-11-11T10:00:00Z"
          }
        ]
      },
      {
        "id": "uuid-2",
        "token": "B8K2-N3Y5-Q6X9-R2A4",
        "max_uses": 5,
        "uses_count": 2,
        "is_active": true,
        "expires_at": "2025-11-19T10:00:00Z",
        "created_at": "2025-11-12T10:00:00Z",
        "status": "active",
        "uses": []
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 5
    }
  }
}
```

**Status Values:**

- `active` - Can still be used
- `used` - Fully used (single-use invitations)
- `expired` - Past expiration date
- `revoked` - Manually revoked

---

### 5. Get Invitation Uses

**Endpoint:** `GET /api/v1/invitations/{id}/uses`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Description:** Get all users who used a specific invitation (own invitations only).

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "invitation_id": "uuid",
    "token": "A7K9-M2X4-P5W8-Q1Z3",
    "uses": [
      {
        "user_id": "uuid",
        "username": "alice",
        "used_at": "2025-11-11T10:00:00Z",
        "ip_address": "192.168.1.100"
      },
      {
        "user_id": "uuid-2",
        "username": "bob",
        "used_at": "2025-11-12T10:00:00Z",
        "ip_address": "192.168.1.101"
      }
    ],
    "total_uses": 2,
    "max_uses": 5
  }
}
```

**Use Case:** See who joined using your invitation (invitation tree visualization).

---

### 6. Revoke Invitation

**Endpoint:** `DELETE /api/v1/invitations/{id}`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Description:** Revoke an invitation (creator can revoke own, admins can revoke any).

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "invitation_id": "uuid",
    "revoked_at": "2025-11-12T10:00:00Z"
  }
}
```

**Actions:**

- Set `is_active = false`
- Set `revoked_at = NOW()`
- Set `revoked_by = current_user_id`
- Publish `invitation.revoked` NATS event

**Error (403 Forbidden):**

```json
{
  "success": false,
  "error": {
    "code": "FORBIDDEN",
    "message": "You can only revoke your own invitations"
  }
}
```

---

## Invitation Tree Visualization

**Future Endpoint:** `GET /api/v1/invitations/tree/{user_id}`

Visualize the invitation chain:

```
Alice (root)
  ├─ Bob (invited by Alice)
  │   ├─ Charlie (invited by Bob)
  │   └─ Diana (invited by Bob)
  └─ Eve (invited by Alice)
```

**Use Case:**

- Trust graph visualization
- Reputation system (if Bob's invitees spam, Bob's reputation decreases)
- Community growth tracking

---

## NATS Events

**Published:**

- `invitation.created` - New invitation created
- `invitation.used` - Invitation used (someone registered)
- `invitation.revoked` - Invitation revoked

**Subscribed:**

- None

---

## Rate Limiting

**Invitation Creation:**

- Max 10 invitations per user per day
- Prevents invitation spam
- Admin-created invitations exempt from limit

---

## Testing

```bash
# Validate invitation (public endpoint)
curl -X POST http://localhost:8004/api/v1/invitations/validate \
  -H "Content-Type: application/json" \
  -d '{
    "token": "A7K9-M2X4-P5W8-Q1Z3"
  }'

# Create invitation
curl -X POST http://localhost:8004/api/v1/invitations \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "max_uses": 1,
    "expires_in_days": 7
  }'

# List my invitations
curl -X GET http://localhost:8004/api/v1/invitations/me \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

# Get invitation uses
curl -X GET http://localhost:8004/api/v1/invitations/INVITATION_ID/uses \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

# Revoke invitation
curl -X DELETE http://localhost:8004/api/v1/invitations/INVITATION_ID \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

---

**Last Updated:** November 12, 2025  
**Service Version:** 0.1.0-alpha.1  
**Implementation Status:** 0/6 endpoints (0%)  
**Priority:** Week 2-3 of migration plan
