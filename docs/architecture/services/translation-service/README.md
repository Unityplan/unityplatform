# translation-service

**Port:** 8012  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Planned - Phase 2  
**Bounded Context:** Internationalization (i18n) & Translation

---

## 📋 Overview

The translation-service manages translations, language resources, and community-contributed translations.

### **Responsibilities**

- ⏳ Store UI translations (strings)
- ⏳ Translate content (auto-translation)
- ⏳ Community translation contributions
- ⏳ Translation validation and voting
- ⏳ Language detection
- ⏳ Translation cache

### **Not Responsible For**

- ❌ User language preferences (handled by settings-service)
- ❌ Professional translation services (future: integration)
- ❌ OCR translation (not planned)

---

## 🗄️ Database Schema (Planned)

```sql
CREATE TABLE territory_{code}.translation_keys (
    id UUID PRIMARY KEY,
    key VARCHAR(255) UNIQUE NOT NULL,  -- e.g., "auth.login.title"
    context TEXT,  -- Description for translators
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE territory_{code}.translations (
    id UUID PRIMARY KEY,
    key_id UUID REFERENCES translation_keys(id),
    language VARCHAR(10) NOT NULL,  -- ISO 639-1 (en, da, no, se)
    value TEXT NOT NULL,
    is_official BOOLEAN DEFAULT false,  -- Official vs community translation
    votes INT DEFAULT 0,
    contributed_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(key_id, language)
);

CREATE TABLE territory_{code}.translation_cache (
    source_text TEXT NOT NULL,
    source_language VARCHAR(10) NOT NULL,
    target_language VARCHAR(10) NOT NULL,
    translated_text TEXT NOT NULL,
    provider VARCHAR(50) DEFAULT 'libre-translate',  -- libre-translate/deepl/google
    cached_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    PRIMARY KEY (source_text, source_language, target_language)
);

CREATE TABLE territory_{code}.translation_votes (
    translation_id UUID REFERENCES translations(id),
    user_id UUID REFERENCES users(id),
    vote INT NOT NULL,  -- +1 or -1
    voted_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (translation_id, user_id)
);
```

---

## 🔌 API Endpoints (Planned)

### **Translation Resources**

- GET /v1/translations/{language} - Get all translations for language
- GET /v1/translations/{language}/{key} - Get specific translation
- POST /v1/translations - Contribute translation
- PATCH /v1/translations/{id} - Update translation
- POST /v1/translations/{id}/vote - Vote on translation quality

### **Auto-Translation**

- POST /v1/translate - Translate text
- POST /v1/translate/detect - Detect language
- GET /v1/translate/supported-languages - List supported languages

---

## 🌍 Supported Languages (Initial)

### **Phase 1 (Core)**

- English (en) - Default
- Danish (da) - Denmark pod
- Norwegian (no) - Norway pod
- Swedish (se) - Sweden pod

### **Phase 2 (Expansion)**

- German (de)
- French (fr)
- Spanish (es)
- Italian (it)

### **Community-Driven**

Users can contribute translations for any language

---

## 🔌 API Examples

### **Get Translations**

```http
GET /v1/translations/da
```

**Response:**

```json
{
  "success": true,
  "data": {
    "language": "da",
    "translations": {
      "auth.login.title": "Log ind",
      "auth.login.username": "Brugernavn",
      "auth.login.password": "Adgangskode",
      "auth.register.title": "Opret konto",
      "common.save": "Gem",
      "common.cancel": "Annuller"
    }
  }
}
```

### **Auto-Translate Text**

```http
POST /v1/translate
{
  "text": "Hello, how are you?",
  "source_language": "en",
  "target_language": "da"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "translated_text": "Hej, hvordan har du det?",
    "source_language": "en",
    "target_language": "da",
    "provider": "libre-translate",
    "cached": false
  }
}
```

### **Contribute Translation**

```http
POST /v1/translations
{
  "key": "auth.login.title",
  "language": "da",
  "value": "Log ind",
  "context": "Login page title"
}
```

---

## 🔗 Service Dependencies

### **Outbound Calls**

#### **Translation API (LibreTranslate)**

- **When:** Auto-translate content
- **Provider:** Self-hosted LibreTranslate or API service
- **Fallback:** Cache previous translations

---

### **Inbound Calls**

#### **Frontend**

- **Endpoints:** GET /v1/translations/{language}
- **Purpose:** Load translations on app start

#### **notification-service**

- **When:** Sending notifications in user's language
- **Endpoint:** POST /v1/translate
- **Purpose:** Translate notification content

---

## 📡 NATS Events (Planned)

**Published:**

- translation.contributed
- translation.approved
- translation.language_added

**Subscribed:**

- user.registered (preload user's language translations)

---

## 🤝 Community Translation Workflow

1. **User contributes translation** - POST /v1/translations
2. **Other users vote** - POST /v1/translations/{id}/vote
3. **High-voted translations become official** - Auto-promotion at 10+ votes
4. **Official translations used in UI** - is_official = true

---

## 🔮 Holochain Migration (Future)

```rust
#[hdk_entry_helper]
struct Translation {
    key: String,
    language: String,
    value: String,
    contributed_by: AgentPubKey,
    created_at: Timestamp,
}

#[hdk_entry_helper]
struct TranslationVote {
    translation_hash: EntryHash,
    voter: AgentPubKey,
    vote: i32,  // +1 or -1
}
```

**Key Concept:** Decentralized translation contributions

- Anyone can contribute
- Community votes determine quality
- No central authority approving translations
- Reputation-based (inviter graph)

---

## 🛠️ Translation Providers

### **LibreTranslate** (Recommended)

- Open-source, self-hosted
- Free API (no usage limits)
- Docker deployment

```yaml
services:
  libretranslate:
    image: libretranslate/libretranslate:latest
    ports:
      - "5000:5000"
    environment:
      - LT_DISABLE_WEB_UI=true
```

### **Future: DeepL/Google Translate API**

- Better quality for professional content
- API key required
- Usage limits and costs

---

## 📊 Translation Coverage

### **Metrics**

- Translation completion per language (% of keys translated)
- Community contribution rate
- Auto-translation usage
- Most requested languages

### **Dashboard**

```
English (en):     100% (1234/1234) - Official
Danish (da):       95% (1172/1234) - Community
Norwegian (no):    90% (1110/1234) - Community
Swedish (se):      85% (1048/1234) - Community
German (de):       30% (370/1234)  - Community
```

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Phase 2 - Not Yet Started  
**Dependencies:** settings-service (user language preference)  
**Technology:** LibreTranslate (self-hosted), community contributions  
**Future:** Holochain-based decentralized translation curation
