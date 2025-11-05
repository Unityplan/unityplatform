# Territory Management Standard

**Document Version:** 1.0  
**Last Updated:** November 5, 2025  
**Status:** **CRITICAL - DO NOT MODIFY FORMAT**

---

## ⚠️ CRITICAL NOTICE

This document defines the **Territory ID Format Standard** for UnityPlan. This format is **FUNDAMENTAL** to the platform's sovereignty model and **MUST NOT** be changed without comprehensive review of:

- Database schemas
- API contracts
- Permission systems
- Pod routing logic
- Frontend territory selection
- Documentation references

Any changes to this format will require:
1. Architecture review board approval
2. Migration plan for existing data
3. Backwards compatibility strategy
4. Updated documentation across all project files

---

## 🆔 Territory ID Format

### **Countries** (ISO 3166-1 Alpha-2)

Uses standard two-letter country codes:

| ID | Name | Notes |
|----|------|-------|
| `US` | United States | ISO 3166-1 |
| `CA` | Canada | ISO 3166-1 |
| `AU` | Australia | ISO 3166-1 |
| `NZ` | New Zealand | ISO 3166-1 |
| `MX` | Mexico | ISO 3166-1 |
| `GB` | United Kingdom | ISO 3166-1 |
| `FR` | France | ISO 3166-1 |
| `DK` | Denmark | ISO 3166-1 |
| `NO` | Norway | ISO 3166-1 |
| `SE` | Sweden | ISO 3166-1 |
| `DE` | Germany | ISO 3166-1 |
| `IT` | Italy | ISO 3166-1 |
| `ES` | Spain | ISO 3166-1 |
| `PT` | Portugal | ISO 3166-1 |
| `NL` | Netherlands | ISO 3166-1 |
| `BE` | Belgium | ISO 3166-1 |
| `CH` | Switzerland | ISO 3166-1 |
| `AT` | Austria | ISO 3166-1 |
| `PL` | Poland | ISO 3166-1 |
| `CZ` | Czech Republic | ISO 3166-1 |
| `FI` | Finland | ISO 3166-1 |
| `IE` | Ireland | ISO 3166-1 |
| `JP` | Japan | ISO 3166-1 |
| `KR` | South Korea | ISO 3166-1 |
| `CN` | China | ISO 3166-1 |
| `IN` | India | ISO 3166-1 |
| `BR` | Brazil | ISO 3166-1 |
| `AR` | Argentina | ISO 3166-1 |
| `CL` | Chile | ISO 3166-1 |
| `ZA` | South Africa | ISO 3166-1 |
| `EG` | Egypt | ISO 3166-1 |
| `KE` | Kenya | ISO 3166-1 |
| `NG` | Nigeria | ISO 3166-1 |

**Total**: 249 countries (ISO 3166-1 standard)

**Reference**: [ISO 3166-1 Country Codes](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)

---

### **First Nations** ({NAME}-FN-{COUNTRY})

Format prioritizes First Nation name, followed by FN marker, followed by country code for **geographic context only**:

**Format**: `{NAME}-FN-{COUNTRY}`

**CRITICAL**: The country code in a First Nation ID is **GEOGRAPHIC METADATA ONLY** and does **NOT** create a parent-child relationship. First Nations are **top-level sovereign territories** with `parent_territory = None`.

**Examples**:

| ID | Name | Country Context | parent_territory | Notes |
|----|------|-----------------|------------------|-------|
| `HAIDA-FN-CA` | Haida Nation | Canada (geographic) | `None` | Independent, Pacific Northwest |
| `NAVAJO-FN-US` | Navajo Nation | United States (geographic) | `None` | Independent, largest US tribe |
| `CREE-FN-CA` | Cree Nation | Canada (geographic) | `None` | Independent, largest FN in Canada |
| `CHEROKEE-FN-US` | Cherokee Nation | United States (geographic) | `None` | Independent, second largest US tribe |
| `YOLNGU-FN-AU` | Yolngu people | Australia (geographic) | `None` | Independent, Northern Territory |
| `MAORI-FN-NZ` | Māori | New Zealand (geographic) | `None` | Independent, Indigenous Polynesian |
| `ZAPOTEC-FN-MX` | Zapotec people | Mexico (geographic) | `None` | Independent, Oaxaca region |
| `INUIT-FN-CA` | Inuit | Canada (geographic) | `None` | Independent, Arctic regions |
| `SAMI-FN-NO` | Sámi people | Norway (geographic) | `None` | Independent (also in SE, FI, RU) |

**Sovereignty Principles**:
- ✅ First Nation name comes **first** (respects sovereignty)
- ✅ `FN` marker clearly identifies as First Nation
- ✅ Country code provides **geographic context** (prevents name collisions)
- ✅ **`parent_territory = None`** (top-level, equal to countries)
- ✅ First Nations control their own communities (e.g., `HAIDA-FN-CA-MASSETT`)
- ✅ Self-identification respected (registered name is authoritative)

**Why Keep Country Code?**
1. **Prevents name collisions**: `EAGLE-FN-CA` vs `EAGLE-FN-US` are distinct
2. **Geographic context**: Helps users understand location
3. **No power implication**: Code is metadata, NOT hierarchy
4. **Practical**: Matches how First Nations often identify themselves internationally

**What This Means for Governance**:
- TeacherRegistrar for `CA` **CANNOT** manage `HAIDA-FN-CA` (separate hierarchies)
- TeacherRegistrar for `HAIDA-FN-CA` **CANNOT** manage `CA` (separate hierarchies)
- Each is sovereign within their own hierarchy
- Platform respects Indigenous self-determination

---

### **Communities** ({PARENT}-{NAME})

Communities are nested within countries or First Nations:

**Format**: `{PARENT_ID}-{COMMUNITY_NAME}`

**Examples**:

| ID | Name | Parent | Type |
|----|------|--------|------|
| `US-CA-SF` | San Francisco | United States → California | City |
| `HAIDA-FN-CA-MASSETT` | Massett | Haida Nation (Canada) | Village |
| `CA-BC-VANCOUVER` | Vancouver | Canada → British Columbia | City |
| `NAVAJO-FN-US-WINDOW-ROCK` | Window Rock | Navajo Nation (US) | Capital |
| `AU-NSW-SYDNEY` | Sydney | Australia → New South Wales | City |
| `DK-COPENHAGEN` | Copenhagen | Denmark | City |
| `NO-OSLO` | Oslo | Norway | City |
| `SE-STOCKHOLM` | Stockholm | Sweden | City |

**Hierarchy Depth**: Unlimited (communities can nest within communities)

---

## 🗄️ Database Schema

### Territories Table

```sql
-- Global territories table (replicated to all pods)
CREATE TABLE global.territories (
  id VARCHAR(100) PRIMARY KEY,           -- e.g., 'DK', 'HAIDA-FN-CA', 'DK-COPENHAGEN'
  name VARCHAR(255) NOT NULL,            -- Display name
  type VARCHAR(50) NOT NULL,             -- 'country', 'first_nation', 'community'
  parent_territory VARCHAR(100),         -- NULL for top-level (countries & First Nations)
  pod_id VARCHAR(50),                    -- Which pod serves this territory
  timezone VARCHAR(100),                 -- IANA timezone (e.g., 'Europe/Copenhagen')
  locale VARCHAR(10),                    -- Default locale (e.g., 'da_DK')
  default_language VARCHAR(10),          -- ISO 639-1 language code (e.g., 'da')
  metadata JSONB,                        -- Additional territory-specific data
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  
  FOREIGN KEY (parent_territory) REFERENCES global.territories(id),
  
  -- Sovereignty constraint: countries and First Nations have no parent
  CHECK (
    (type IN ('country', 'first_nation') AND parent_territory IS NULL)
    OR
    (type = 'community' AND parent_territory IS NOT NULL)
  )
);

-- Indexes for efficient queries
CREATE INDEX idx_territories_type ON global.territories(type);
CREATE INDEX idx_territories_parent ON global.territories(parent_territory);
CREATE INDEX idx_territories_pod ON global.territories(pod_id);

-- Example data
INSERT INTO global.territories (id, name, type, parent_territory, pod_id, timezone, locale, default_language) VALUES
  -- Countries
  ('DK', 'Denmark', 'country', NULL, 'dk', 'Europe/Copenhagen', 'da_DK', 'da'),
  ('NO', 'Norway', 'country', NULL, 'no', 'Europe/Oslo', 'no_NO', 'no'),
  ('SE', 'Sweden', 'country', NULL, 'se', 'Europe/Stockholm', 'sv_SE', 'sv'),
  ('CA', 'Canada', 'country', NULL, 'ca', 'America/Toronto', 'en_CA', 'en'),
  ('US', 'United States', 'country', NULL, 'us', 'America/New_York', 'en_US', 'en'),
  
  -- First Nations (top-level, equal to countries)
  ('HAIDA-FN-CA', 'Haida Nation', 'first_nation', NULL, 'haida-fn-ca', 'America/Vancouver', 'en_CA', 'en'),
  ('NAVAJO-FN-US', 'Navajo Nation', 'first_nation', NULL, 'navajo-fn-us', 'America/Phoenix', 'en_US', 'nv'),
  ('CREE-FN-CA', 'Cree Nation', 'first_nation', NULL, 'cree-fn-ca', 'America/Winnipeg', 'en_CA', 'cr'),
  ('SAMI-FN-NO', 'Sámi people', 'first_nation', NULL, 'sami-fn-no', 'Europe/Oslo', 'se_NO', 'se'),
  
  -- Communities (nested)
  ('DK-COPENHAGEN', 'Copenhagen', 'community', 'DK', 'dk', 'Europe/Copenhagen', 'da_DK', 'da'),
  ('NO-OSLO', 'Oslo', 'community', 'NO', 'no', 'Europe/Oslo', 'no_NO', 'no'),
  ('HAIDA-FN-CA-MASSETT', 'Massett', 'community', 'HAIDA-FN-CA', 'haida-fn-ca', 'America/Vancouver', 'en_CA', 'en'),
  ('NAVAJO-FN-US-WINDOW-ROCK', 'Window Rock', 'community', 'NAVAJO-FN-US', 'navajo-fn-us', 'America/Phoenix', 'en_US', 'nv');
```

---

## 🔒 Permission & Governance Implications

### Separate Hierarchies

```
Country Hierarchy:                First Nation Hierarchy:
CA (Canada)                       HAIDA-FN-CA (Haida Nation)
├── CA-BC                         ├── HAIDA-FN-CA-MASSETT
│   └── CA-BC-VANCOUVER           └── HAIDA-FN-CA-SKIDEGATE
└── CA-ON
    └── CA-ON-TORONTO

TeacherRegistrar[CA] can manage:  TeacherRegistrar[HAIDA-FN-CA] can manage:
✅ CA-BC                          ✅ HAIDA-FN-CA-MASSETT
✅ CA-ON                          ✅ HAIDA-FN-CA-SKIDEGATE
❌ HAIDA-FN-CA (separate)         ❌ CA (separate)
```

### Role Scope Examples

```sql
-- Territory Manager for Canada (cannot access Haida Nation)
INSERT INTO roles (user_id, role_type, territory_id) VALUES
  ('user-123', 'territory_manager', 'CA');

-- This user CAN manage:
SELECT * FROM territories WHERE id LIKE 'CA-%';
-- CA-BC, CA-ON, CA-BC-VANCOUVER, etc.

-- This user CANNOT manage:
SELECT * FROM territories WHERE id LIKE 'HAIDA-FN-CA%';
-- HAIDA-FN-CA, HAIDA-FN-CA-MASSETT (separate hierarchy)

-- Territory Manager for Haida Nation (cannot access Canada)
INSERT INTO roles (user_id, role_type, territory_id) VALUES
  ('user-456', 'territory_manager', 'HAIDA-FN-CA');

-- This user CAN manage:
SELECT * FROM territories WHERE id LIKE 'HAIDA-FN-CA-%';
-- HAIDA-FN-CA-MASSETT, HAIDA-FN-CA-SKIDEGATE

-- This user CANNOT manage:
SELECT * FROM territories WHERE id LIKE 'CA%' AND id != 'HAIDA-FN-CA%';
-- CA, CA-BC, CA-ON (separate hierarchy)
```

---

## 🧭 Frontend Implementation

### Territory Selector Component

```typescript
interface Territory {
  id: string;
  name: string;
  type: 'country' | 'first_nation' | 'community';
  parent_territory: string | null;
  pod_id: string;
}

// Group territories by type for display
const groupedTerritories = {
  countries: territories.filter(t => t.type === 'country'),
  firstNations: territories.filter(t => t.type === 'first_nation'),
  communities: territories.filter(t => t.type === 'community')
};

// Display order: Countries, First Nations, then Communities
<TerritorySelect>
  <optgroup label="Countries">
    {groupedTerritories.countries.map(t => 
      <option value={t.id}>{t.name}</option>
    )}
  </optgroup>
  
  <optgroup label="First Nations">
    {groupedTerritories.firstNations.map(t => 
      <option value={t.id}>{t.name}</option>
    )}
  </optgroup>
  
  <optgroup label="Communities">
    {groupedTerritories.communities.map(t => 
      <option value={t.id}>{t.name}</option>
    )}
  </optgroup>
</TerritorySelect>
```

---

## 🚀 API Endpoints

### Get Territory by ID

```http
GET /api/v1/territories/{id}

Example:
GET /api/v1/territories/HAIDA-FN-CA

Response:
{
  "id": "HAIDA-FN-CA",
  "name": "Haida Nation",
  "type": "first_nation",
  "parent_territory": null,
  "pod_id": "haida-fn-ca",
  "timezone": "America/Vancouver",
  "locale": "en_CA",
  "default_language": "en"
}
```

### List Territories

```http
GET /api/v1/territories?type=first_nation

Response:
[
  {
    "id": "HAIDA-FN-CA",
    "name": "Haida Nation",
    "type": "first_nation",
    "parent_territory": null
  },
  {
    "id": "NAVAJO-FN-US",
    "name": "Navajo Nation",
    "type": "first_nation",
    "parent_territory": null
  }
]
```

### Get Territory Hierarchy

```http
GET /api/v1/territories/{id}/hierarchy

Example:
GET /api/v1/territories/HAIDA-FN-CA-MASSETT/hierarchy

Response:
{
  "id": "HAIDA-FN-CA-MASSETT",
  "name": "Massett",
  "path": [
    {
      "id": "HAIDA-FN-CA",
      "name": "Haida Nation",
      "type": "first_nation"
    },
    {
      "id": "HAIDA-FN-CA-MASSETT",
      "name": "Massett",
      "type": "community"
    }
  ]
}
```

---

## 📚 References

- **Project Overview**: `project_docs/2-project-overview.md` (Territory Definition section)
- **Multi-Pod Architecture**: `project_docs/5-multi-pod-architecture.md` (Territory ID Format section)
- **ISO 3166-1 Standard**: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
- **Indigenous Self-Determination**: Platform design principle

---

## ✅ Validation Rules

### Territory ID Validation

```typescript
function validateTerritoryId(id: string): boolean {
  // Country: 2 uppercase letters
  const countryPattern = /^[A-Z]{2}$/;
  
  // First Nation: {NAME}-FN-{COUNTRY}
  const firstNationPattern = /^[A-Z][A-Z0-9-]+-FN-[A-Z]{2}$/;
  
  // Community: {PARENT}-{NAME} (recursive)
  const communityPattern = /^([A-Z]{2}|[A-Z][A-Z0-9-]+-FN-[A-Z]{2})(-[A-Z][A-Z0-9-]*)+$/;
  
  return countryPattern.test(id) || 
         firstNationPattern.test(id) || 
         communityPattern.test(id);
}

// Examples
validateTerritoryId('DK')                        // ✅ true (country)
validateTerritoryId('HAIDA-FN-CA')               // ✅ true (First Nation)
validateTerritoryId('DK-COPENHAGEN')             // ✅ true (community)
validateTerritoryId('HAIDA-FN-CA-MASSETT')       // ✅ true (community)
validateTerritoryId('haida-fn-ca')               // ❌ false (lowercase)
validateTerritoryId('HAIDA-CA')                  // ❌ false (missing FN marker)
validateTerritoryId('CA-HAIDA-FN-CA')            // ❌ false (wrong format)
```

---

**Document Status:** **NORMATIVE** - This is the authoritative reference for territory IDs  
**Change Control:** Architecture Review Board approval required  
**Last Review:** November 5, 2025
