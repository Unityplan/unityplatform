# Language Proficiency & Translation System

**Status:** Planned  
**Category:** Architecture / User Profile  
**Priority:** High (Core feature for multilingual platform)

## Overview

The platform's language proficiency system enables true multilingual collaboration with automatic translation. Users define their language capabilities with detailed proficiency levels, allowing the platform to make intelligent translation decisions.

## User Language Preferences Structure

### Current Simple Implementation (MVP)

**Field:** `languages` (string array)  
**Format:** Comma-separated list  
**Example:** `["English (Native)", "Danish (Fluent)", "Swedish (Intermediate)"]`

### Future Enhanced Implementation

```typescript
interface LanguagePreferences {
  // Primary language for UI and content
  preferred_language: string; // ISO 639-1 code (e.g., "en", "da", "sv")
  
  // Ordered list of secondary languages with proficiency levels
  secondary_languages: LanguageProficiency[];
  
  // Fallback to English if preferred languages unavailable
  fallback_to_english: boolean; // Default: true
  
  // Auto-translate content not in preferred languages
  auto_translate: boolean; // Default: true
  translation_provider: TranslationProvider;
}

interface LanguageProficiency {
  language_code: string; // ISO 639-1 (e.g., "da", "sv", "no")
  language_name: string; // Display name ("Danish", "Svenska", etc.)
  
  // Four skill dimensions (CEFR-inspired)
  spoken_level: ProficiencyLevel;
  written_level: ProficiencyLevel;
  listening_level: ProficiencyLevel;
  reading_level: ProficiencyLevel;
}

enum ProficiencyLevel {
  Native = "native",
  Fluent = "fluent",           // C2 - Full professional proficiency
  Advanced = "advanced",       // C1 - Advanced
  Intermediate = "intermediate", // B1-B2 - Intermediate
  Basic = "basic",             // A2 - Elementary
  Learning = "learning",       // A1 - Beginner
}

enum TranslationProvider {
  DeepL = "deepl",                    // Premium quality
  LibreTranslate = "libretranslate",  // Self-hosted, privacy-focused (preferred)
  Google = "google",
  Microsoft = "microsoft",
}
```

## Database Schema (Future)

### Table: `user_languages`

```sql
CREATE TABLE territory.user_languages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES territory.users(id) ON DELETE CASCADE,
    language_code VARCHAR(10) NOT NULL,  -- ISO 639-1 or 639-3
    language_name VARCHAR(100) NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    display_order INT NOT NULL DEFAULT 0,  -- Order in list
    
    -- Proficiency levels
    spoken_level VARCHAR(20),    -- native, fluent, advanced, intermediate, basic, learning
    written_level VARCHAR(20),
    listening_level VARCHAR(20),
    reading_level VARCHAR(20),
    
    -- Metadata
    learned_from DATE,           -- When they started learning
    notes TEXT,                  -- Personal notes about their proficiency
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT user_languages_user_id_lang_code_unique 
        UNIQUE (user_id, language_code),
    CONSTRAINT user_languages_proficiency_check 
        CHECK (spoken_level IN ('native', 'fluent', 'advanced', 'intermediate', 'basic', 'learning')),
    CONSTRAINT user_languages_display_order_positive 
        CHECK (display_order >= 0)
);

-- Indexes
CREATE INDEX idx_user_languages_user_id ON territory.user_languages(user_id);
CREATE INDEX idx_user_languages_primary ON territory.user_languages(user_id, is_primary);
CREATE INDEX idx_user_languages_order ON territory.user_languages(user_id, display_order);
```

## How Translation Works

### Content Display Logic

1. **User views content** (forum post, course material, etc.)
2. **Platform checks content language** against user's language preferences
3. **Translation decision tree:**
   - ✅ Content in user's primary language → Show original
   - ✅ Content in secondary language with Fluent/Native reading level → Show original
   - ✅ Content in secondary language with Advanced reading level → Show original with translation option
   - ⚠️ Content in secondary language with Intermediate/Basic reading level → Auto-translate, show original button
   - ❌ Content in unknown language → Auto-translate

### Translation Quality Matching

The system matches translation quality to user proficiency:

```typescript
function needsTranslation(
  contentLanguage: string,
  userPreferences: LanguagePreferences
): boolean {
  // Primary language - never translate
  if (contentLanguage === userPreferences.preferred_language) {
    return false;
  }
  
  // Check secondary languages
  const proficiency = userPreferences.secondary_languages.find(
    lang => lang.language_code === contentLanguage
  );
  
  if (!proficiency) {
    // Unknown language - translate
    return true;
  }
  
  // Translate based on reading level
  return proficiency.reading_level < ProficiencyLevel.Advanced;
}
```

## Frontend UI Components

### Language Selector Component (Future)

```tsx
// Language proficiency editor for profile
<LanguageProficiencyEditor
  languages={user.language_preferences}
  onChange={updateLanguages}
/>

// Features:
// - Add/remove languages
// - Set primary language
// - Drag-and-drop reorder
// - Proficiency sliders for each skill (spoken, written, listening, reading)
// - Language search with autocomplete (ISO 639 languages)
// - Visual proficiency indicators (Native → Learning)
```

### Current MVP Implementation

```tsx
// Simple comma-separated input (current)
<Input
  id="languages"
  value="English (Native), Danish (Fluent), Swedish (Intermediate)"
  placeholder="e.g., English (Native), Danish (Fluent), Swedish (Intermediate)"
/>

// Helper text
<p className="text-sm text-muted-foreground">
  Primary language first, then secondary languages with proficiency levels.
  This helps with translation features.
</p>
```

## Migration Path

### Phase 1: MVP (Current)

- Simple string array in `user_profiles.languages`
- Comma-separated input field
- Manual proficiency indication in parentheses
- No automatic translation yet

### Phase 2: Structured Data

- Create `user_languages` table
- Migrate existing language strings to structured format
- Parse proficiency levels from strings
- Build language selector UI component

### Phase 3: Translation Integration

- Integrate translation service (LibreTranslate preferred)
- Implement content language detection
- Add translation caching (translation memory)
- Smart translation based on user proficiency

### Phase 4: Advanced Features

- Language learning mode (side-by-side original + translation)
- Community-driven translation corrections
- Context-aware translation (preserve technical terms)
- Translation quality feedback loop

## ISO 639 Language Codes

Common languages for Nordic/European context:

| Code | Language | Native Name |
|------|----------|-------------|
| `da` | Danish | Dansk |
| `sv` | Swedish | Svenska |
| `no` | Norwegian | Norsk |
| `fi` | Finnish | Suomi |
| `is` | Icelandic | Íslenska |
| `en` | English | English |
| `de` | German | Deutsch |
| `fr` | French | Français |
| `es` | Spanish | Español |
| `it` | Italian | Italiano |
| `pl` | Polish | Polski |
| `ru` | Russian | Русский |

Full list: <https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes>

## Privacy & Data Sovereignty

### Language Data Privacy

- ✅ Language preferences stored in **territory schema** (data sovereignty)
- ✅ Never shared without explicit user consent
- ✅ Translation happens **server-side** (LibreTranslate self-hosted)
- ✅ Translation memory cached per territory
- ✅ No data sent to external translation APIs (when using LibreTranslate)
- ✅ User can opt-out of auto-translation

### Translation Memory

```sql
-- Territory-scoped translation cache
CREATE TABLE territory.translation_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_text_hash VARCHAR(64) NOT NULL,  -- SHA-256 of source text
    source_language VARCHAR(10) NOT NULL,
    target_language VARCHAR(10) NOT NULL,
    translated_text TEXT NOT NULL,
    translation_provider VARCHAR(50),
    quality_score FLOAT,  -- 0.0 - 1.0 (from provider or community rating)
    
    -- Metadata
    usage_count INT DEFAULT 1,
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT translation_cache_unique 
        UNIQUE (source_text_hash, source_language, target_language)
);

CREATE INDEX idx_translation_cache_hash ON territory.translation_cache(source_text_hash);
CREATE INDEX idx_translation_cache_langs ON territory.translation_cache(source_language, target_language);
```

## Related Documentation

- `docs/project/overview.md` - User profile system architecture
- `docs/architecture/infrastructure.md` - Translation service design
- `docs/status/current/phase-1-status.md` - Stage 11: Translation Service
- `services/translation-service/` - Translation service implementation (future)

## API Endpoints (Future)

### User Language Management

```http
GET /api/v1/profiles/{user_id}/languages
POST /api/v1/profiles/{user_id}/languages
PUT /api/v1/profiles/{user_id}/languages/{lang_id}
DELETE /api/v1/profiles/{user_id}/languages/{lang_id}
PATCH /api/v1/profiles/{user_id}/languages/reorder
```

### Translation Service

```http
POST /api/v1/translate
{
  "text": "Hello, world!",
  "source_language": "en",
  "target_language": "da",
  "context": "forum_post"  // Helps preserve technical terms
}

Response:
{
  "translated_text": "Hej, verden!",
  "source_language_detected": "en",
  "confidence": 0.98,
  "cached": false,
  "provider": "libretranslate"
}
```

## Benefits

✅ **User Sovereignty**: Users control their language data  
✅ **Inclusive**: Barrier-free multilingual collaboration  
✅ **Smart**: Translations only when needed  
✅ **Privacy-First**: Self-hosted translation (no data leaks)  
✅ **Context-Aware**: Preserves technical terms and nuances  
✅ **Community-Driven**: Users improve translations together  
✅ **Educational**: Optional language learning mode  
✅ **Efficient**: Translation memory reduces redundant work  

## References

- **LibreTranslate**: <https://libretranslate.com/> (self-hosted translation)
- **ISO 639**: <https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes> (language codes)
- **CEFR Levels**: <https://en.wikipedia.org/wiki/Common_European_Framework_of_Reference_for_Languages>
- **DeepL API**: <https://www.deepl.com/pro-api> (premium option)
