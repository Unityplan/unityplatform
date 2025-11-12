# settings-service API Endpoints

**Base URL:** `http://localhost:8003`  
**Version:** v1  
**Status:** ⏳ Scaffolded (0/5 endpoints)

---

## Settings Endpoints

### 1. Get All Settings

**Endpoint:** `GET /api/v1/settings/{user_id}`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Description:** Get all user settings (appearance + language + privacy).

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "appearance": {
      "theme_mode": "dark",
      "color_scheme": "forest-green",
      "reduced_motion": false,
      "wide_content_view": true,
      "compact_mode": false
    },
    "language": {
      "preferred_language": "da",
      "timezone": "Europe/Copenhagen",
      "auto_translate": true,
      "translation_provider": "libre-translate",
      "contribute_translations": true,
      "fallback_to_english": true
    },
    "privacy": {
      "profile_visibility": "public",
      "show_email": false,
      "show_location": true,
      "show_connections": true,
      "show_activity": true,
      "allow_messages": "followers",
      "allow_followers": "everyone",
      "require_follow_approval": false,
      "allow_indexing": true,
      "share_analytics": false
    }
  }
}
```

---

### 2. Update Settings (Bulk)

**Endpoint:** `PUT /api/v1/settings/{user_id}`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**Description:** Update settings (bulk update - partial changes allowed).

**Request Body:**

```json
{
  "appearance": {
    "theme_mode": "dark",
    "color_scheme": "ocean-blue"
  },
  "language": {
    "preferred_language": "da"
  }
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "updated": true
  }
}
```

**NATS Event Published:** `settings.updated`

---

### 3. Get/Update Appearance Settings

**Endpoint:** `GET /api/v1/settings/{user_id}/appearance`  
**Endpoint:** `PATCH /api/v1/settings/{user_id}/appearance`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**GET Response:**

```json
{
  "success": true,
  "data": {
    "theme_mode": "dark",
    "color_scheme": "forest-green",
    "reduced_motion": false,
    "wide_content_view": true,
    "compact_mode": false,
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

**PATCH Request:**

```json
{
  "theme_mode": "light",
  "compact_mode": true
}
```

**Theme Options:**

- `light` - Light theme
- `dark` - Dark theme
- `system` - Follow system preference

**Color Schemes:**

- `forest-green` - Default green
- `ocean-blue` - Blue theme
- `royal-purple` - Purple theme
- `custom` - Custom colors

---

### 4. Get/Update Language Settings

**Endpoint:** `GET /api/v1/settings/{user_id}/language`  
**Endpoint:** `PATCH /api/v1/settings/{user_id}/language`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**GET Response:**

```json
{
  "success": true,
  "data": {
    "preferred_language": "da",
    "timezone": "Europe/Copenhagen",
    "auto_translate": true,
    "translation_provider": "libre-translate",
    "contribute_translations": true,
    "fallback_to_english": true,
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

**PATCH Request:**

```json
{
  "preferred_language": "no",
  "timezone": "Europe/Oslo",
  "auto_translate": false
}
```

**Translation Providers:**

- `libre-translate` - Self-hosted, free
- `deepl` - High quality (future)
- `google` - Google Translate (future)
- `microsoft` - Microsoft Translator (future)

---

### 5. Get/Update Privacy Settings

**Endpoint:** `GET /api/v1/settings/{user_id}/privacy`  
**Endpoint:** `PATCH /api/v1/settings/{user_id}/privacy`  
**Authentication:** Bearer token required  
**Status:** ⏳ Planned

**GET Response:**

```json
{
  "success": true,
  "data": {
    "profile_visibility": "followers",
    "show_email": false,
    "show_location": false,
    "show_connections": true,
    "show_activity": true,
    "allow_messages": "followers",
    "allow_followers": "everyone",
    "require_follow_approval": false,
    "allow_indexing": true,
    "share_analytics": false,
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

**PATCH Request:**

```json
{
  "profile_visibility": "private",
  "allow_messages": "nobody",
  "share_analytics": false
}
```

**Profile Visibility:**

- `public` - Anyone can see profile
- `followers` - Only followers see details
- `private` - Only user can see

**Message Permissions:**

- `everyone` - Anyone can message
- `followers` - Only followers
- `nobody` - No messages allowed

**Follower Permissions:**

- `everyone` - Anyone can follow
- `approved` - Requires approval
- `nobody` - No followers allowed

---

## Default Settings

New users automatically get these default settings:

```json
{
  "appearance": {
    "theme_mode": "system",
    "color_scheme": "forest-green",
    "reduced_motion": false,
    "wide_content_view": false,
    "compact_mode": false
  },
  "language": {
    "preferred_language": "en",
    "timezone": "UTC",
    "auto_translate": false,
    "translation_provider": "libre-translate",
    "contribute_translations": false,
    "fallback_to_english": true
  },
  "privacy": {
    "profile_visibility": "public",
    "show_email": false,
    "show_location": false,
    "show_connections": true,
    "show_activity": true,
    "allow_messages": "everyone",
    "allow_followers": "everyone",
    "require_follow_approval": false,
    "allow_indexing": true,
    "share_analytics": true
  }
}
```

---

## NATS Events

**Subscribed:**

- `user.registered` - Create default settings for new user

**Published:**

- `settings.updated` - Settings changed
- `privacy.updated` - Privacy settings changed (important for other services)

---

## Testing

```bash
# Get all settings
curl -X GET http://localhost:8003/api/v1/settings/USER_ID \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

# Update appearance
curl -X PATCH http://localhost:8003/api/v1/settings/USER_ID/appearance \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "theme_mode": "dark",
    "color_scheme": "ocean-blue"
  }'

# Update privacy
curl -X PATCH http://localhost:8003/api/v1/settings/USER_ID/privacy \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "profile_visibility": "private",
    "allow_messages": "nobody"
  }'
```

---

**Last Updated:** November 12, 2025  
**Service Version:** 0.1.0-alpha.1  
**Implementation Status:** 0/5 endpoints (0%)  
**Priority:** Week 1 of migration plan
