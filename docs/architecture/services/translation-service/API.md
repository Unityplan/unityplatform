# translation-service API Endpoints

**Base URL:** `http://localhost:8012`  
**Version:** v1  
**Status:** 📋 Planned (Future Phase - i18n)

---

## Translation Resources

### 1. Get Translations

**Endpoint:** `GET /api/v1/translations/{language}`  
**Status:** 📋 Planned

**Response:**

```json
{
  "success": true,
  "data": {
    "language": "da",
    "namespace": "common",
    "translations": {
      "navigation.home": "Hjem",
      "navigation.profile": "Profil",
      "buttons.save": "Gem",
      "errors.required": "Dette felt er påkrævet"
    },
    "version": "1.0.0",
    "updated_at": "2025-11-12T10:00:00Z"
  }
}
```

---

### 2. Contribute Translation

**Endpoint:** `POST /api/v1/translations/{language}/contribute`  
**Status:** 📋 Planned

**Request:**

```json
{
  "key": "navigation.settings",
  "value": "Indstillinger",
  "namespace": "common"
}
```

**Workflow:**

1. Community member submits translation
2. Moderators review and approve
3. Translation published to all users

---

### 3. Vote on Translation

**Endpoint:** `POST /api/v1/translations/{language}/vote`  
**Status:** 📋 Planned

**Request:**

```json
{
  "translation_id": "uuid",
  "vote": "upvote"
}
```

---

## Auto-Translation

### 4. Translate Text

**Endpoint:** `POST /api/v1/translations/translate`  
**Status:** 📋 Planned

**Request:**

```json
{
  "text": "Hello, how are you?",
  "from": "en",
  "to": "da",
  "provider": "libre-translate"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "translated_text": "Hej, hvordan har du det?",
    "provider": "libre-translate",
    "from": "en",
    "to": "da"
  }
}
```

---

### 5. Detect Language

**Endpoint:** `POST /api/v1/translations/detect`  
**Status:** 📋 Planned

---

## Translation Providers

- **LibreTranslate** (default, self-hosted, free)
- DeepL (optional, API key required)
- Google Translate (optional)
- Microsoft Translator (optional)

---

## Community Translation System

**Workflow:**

1. Developer adds English text with key
2. Translation service marks as "needs translation"
3. Community members contribute translations
4. Moderators review (native speakers)
5. Approved translations published
6. Auto-translation fills gaps temporarily

**Contributor Badges:**

- Translation Contributor (10 approved)
- Translation Expert (100 approved)
- Polyglot (5+ languages)

---

**Last Updated:** November 12, 2025  
**Implementation Status:** Future phase (i18n system)
