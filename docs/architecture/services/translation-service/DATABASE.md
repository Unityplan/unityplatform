# translation-service Database Schema

**Service:** translation-service  
**Port:** 8012  
**Database:** Global schema only

---

## Schema Distribution

### Global Schema (Translation Resources)

**Table:** `global.translation_resources` (Future)  
**Purpose:** Shared translations across all territories

```sql
CREATE TABLE global.translation_resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    language_code VARCHAR(10) NOT NULL,
    namespace VARCHAR(50) NOT NULL,
    translation_key VARCHAR(255) NOT NULL,
    
    value TEXT NOT NULL,
    
    status VARCHAR(20) NOT NULL DEFAULT 'approved',
    
    contributed_by UUID,
    contributor_territory VARCHAR(10),
    
    upvotes INT NOT NULL DEFAULT 0,
    downvotes INT NOT NULL DEFAULT 0,
    
    version INT NOT NULL DEFAULT 1,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (language_code, namespace, translation_key),
    CHECK (status IN ('pending', 'approved', 'rejected'))
);
```

**Table:** `global.translation_votes` (Future)

```sql
CREATE TABLE global.translation_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    translation_id UUID NOT NULL REFERENCES global.translation_resources(id) ON DELETE CASCADE,
    
    user_id UUID NOT NULL,
    user_territory VARCHAR(10) NOT NULL,
    
    vote_type VARCHAR(10) NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (translation_id, user_id),
    CHECK (vote_type IN ('upvote', 'downvote'))
);
```

**Why Global?** Translations are shared resources - all users benefit from community contributions.

---

## Translation Workflow

1. Developer adds English text: `navigation.home = "Home"`
2. Translation service marks as "needs translation"
3. Community members contribute translations
4. Native speakers vote on quality
5. Best translation (most upvotes) auto-approved
6. Published to all territories

---

## LibreTranslate Integration

**Auto-Translation:**  
If no human translation exists, LibreTranslate provides machine translation (temporary).

**Table:** `global.auto_translations` (Cache)

```sql
CREATE TABLE global.auto_translations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    source_text TEXT NOT NULL,
    source_lang VARCHAR(10) NOT NULL,
    target_lang VARCHAR(10) NOT NULL,
    
    translated_text TEXT NOT NULL,
    
    provider VARCHAR(50) NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (source_text, source_lang, target_lang)
);
```

**Providers:** libre-translate (self-hosted), deepl, google, microsoft

---

## Multi-Pod Strategy

**Translation Resources:** Global (shared knowledge)  
**No Territory Storage:** Translations benefit everyone

**Example:**
- Danish community contributes "navigation.home = "Hjem""
- Norwegian community uses same translation (or contributes better one)
- Best translation wins (crowdsourced quality)

---

## NATS Events

**Publishes:** `translation.contributed`, `translation.approved`, `translation.updated`

---

**Last Updated:** November 12, 2025
