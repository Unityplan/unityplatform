# Backend API Requirements for Frontend Features

**Status:** Requirements Documentation  
**Created:** November 11, 2025  
**Purpose:** Define backend API requirements based on implemented frontend features  
**Next Step:** Database schema design for multi-tenant Holochain migration readiness

---

## Table of Contents

1. [User Authentication & Registration](#1-user-authentication--registration)
2. [User Profile Management](#2-user-profile-management)
3. [User Settings Management](#3-user-settings-management)
4. [Invitation System](#4-invitation-system)
5. [Data Sovereignty & GDPR](#5-data-sovereignty--gdpr)

---

## 1. User Authentication & Registration

### 1.1 Core User Entity

**Data Model:**

```typescript
interface User {
  id: string;                    // UUID - Primary identifier
  username: string;              // UNIQUE - Primary identifier (never changes, even with territory migration)
  email?: string | null;         // OPTIONAL - For external notifications only (invitations, password reset)
  full_name: string | null;      // Optional display name
  territory_code: string;        // FK to territory (e.g., 'dk', 'no', 'se', 'eu')
  is_active: boolean;            // Account status
  created_at: string;            // ISO 8601 timestamp
  updated_at: string;            // ISO 8601 timestamp
}
```

**Identity System:**

- **Username:** Primary human-readable identifier (globally unique across all pods/territories)
- **Never changes:** Username remains the same even when user migrates between territories
- **Matrix ID:** Derived from username@territory (e.g., `@alice:unityplan.dk`)
- **Territory Migration:** Matrix ID changes, but username stays constant
  - Before: `@alice:unityplan.dk` (primary Matrix ID)
  - After: `@alice:unityplan.no` (new primary), `@alice:unityplan.dk` (alias - still works)
- **Email:** Optional field used only for external notifications, NOT for authentication
- See [Identity System Architecture](identity-system.md) for complete details

**Database Requirements:**

- Multi-tenant architecture with territory schemas (naming: `territory_{code}`, e.g., `territory_dk`)
- User table in each territory schema
- Global registry for cross-territory lookups (username uniqueness - email uniqueness if provided)
- Indexes on: `username`, `email` (where not null), `territory_code`

**Security Requirements:**

- Password hashing (bcrypt/argon2)
- Email verification workflow (only if email provided)
- Account activation/deactivation
- Soft delete support (retain data for GDPR export)

### 1.2 Authentication Flow

**Login Request:**

```typescript
interface LoginRequest {
  username: string;              // Privacy-first (NOT email)
  password: string;
  territory_code: string;        // Which pod/territory to authenticate against
}
```

**Authentication Response:**

```typescript
interface AuthResponse {
  access_token: string;          // JWT - Short-lived (15 minutes)
  refresh_token: string;         // Long-lived (7 days)
  token_type: 'Bearer';
  expires_in: number;            // Seconds until access_token expires
  user: User;                    // Full user object
}
```

**Required Endpoints:**

```
POST   /api/v1/auth/login           - Authenticate user
POST   /api/v1/auth/logout          - Invalidate tokens
POST   /api/v1/auth/refresh         - Refresh access token
GET    /api/v1/auth/me              - Get current user
POST   /api/v1/auth/verify-email    - Verify email address
```

**Token Management:**

- JWT with claims: `user_id`, `username`, `territory_code`, `roles`, `exp`, `iat`
- Refresh token rotation (invalidate old token on refresh)
- Token blacklist for logout (Redis/database)
- Rate limiting on login attempts (5 attempts per 15 minutes)

### 1.3 Registration Flow

**Registration Request:**

```typescript
interface RegisterRequest {
  username: string;              // Required - Must be unique globally (primary identifier)
  password: string;              // Required - Min 8 chars, complexity requirements
  email?: string;                // Optional - If provided, will receive verification email
  full_name?: string;            // Optional - Can be added later in profile
  invitation_token: string;      // Required - Determines territory automatically
}
```

**Backend Processing:**

1. Validate invitation token via `global.invitation_token_registry`
2. Determine territory from token (NOT client-provided)
3. Verify username uniqueness (global check across all territories)
4. If email provided: Verify email uniqueness (global check)
5. Validate password complexity
6. Hash password
7. Create user in `{territory_schema}.users` table
8. Register username in `global.user_identities`
9. If email provided: Send verification email
10. If invitation has `community_id`, assign user to community
11. Mark invitation token as used
12. Return auth tokens

**Required Endpoints:**

```
POST   /api/v1/auth/register        - Register new user (requires invitation)
GET    /api/v1/auth/check-username  - Check username availability
GET    /api/v1/auth/check-email     - Check email availability
```

---

## 2. User Profile Management

### 2.1 User Profile Entity

**Data Model:**

```typescript
interface UserProfile {
  user_id: string;               // FK to users.id (Primary Key)
  username: string;              // Denormalized for easy display
  email?: string | null;         // Only shown if privacy settings allow
  
  // Basic Info
  full_name?: string | null;
  display_name?: string | null;  // Preferred display name (fallback: username)
  avatar_url?: string | null;    // S3/IPFS URL
  
  // Rich Profile
  bio?: string | null;           // Short tagline (280 chars max)
  about?: string | null;         // Long-form description (Markdown supported)
  
  // Interests & Skills
  interests?: string[] | null;   // Array of interest tags
  skills?: string[] | null;      // Array of skill tags
  
  // Language Proficiency (detailed skills visible to others)
  language_proficiencies?: LanguageProficiency[];  // Ordered by user preference
  
  // Location (privacy-aware)
  location?: string | null;      // Encoded: "[55.6761,12.5683]Copenhagen, Denmark"
  
  // Privacy Settings
  privacy?: PrivacySettings;
  
  // Timestamps
  created_at?: string;
  updated_at?: string;
}
```

**Database Requirements:**

- Profile table in each territory schema (`{schema_name}.users_profiles`)
- One-to-one relationship with users table
- JSON/JSONB column for interests, skills, languages arrays
- Full-text search on: `display_name`, `bio`, `about`, `interests`, `skills`
- Separate table for profile links (flexible system)

### 2.2 Profile Privacy Settings

**Data Model:**

```typescript
interface PrivacySettings {
  profile_visibility: 'public' | 'connections_only' | 'private';
  show_email: boolean;           // Show email on public profile
  show_full_name: boolean;       // Show full name (vs display_name only)
  show_location: boolean;        // Show location on profile
  show_connections: boolean;     // Show followers/following count
  allow_messages_from: 'everyone' | 'connections_only' | 'nobody';
}
```

**Default Privacy Settings:**

```typescript
{
  profile_visibility: 'public',
  show_email: false,
  show_full_name: true,
  show_location: true,
  show_connections: true,
  allow_messages_from: 'everyone'
}
```

**Privacy Enforcement:**

- Backend must filter profile data based on viewer's relationship to profile owner
- Relationships: `self`, `connection`, `stranger`
- Apply privacy rules at query level (database views or application logic)

### 2.3 Profile Links System (Flexible External Links)

**Data Model:**

```typescript
interface ProfileLink {
  id: string;                    // UUID
  user_id: string;               // FK to users.id
  label: string;                 // Display text (e.g., "My Portfolio")
  url: string;                   // Full URL (validated)
  icon?: string | null;          // Icon name or emoji (e.g., "github", "🌐")
  display_order: number;         // Sort order (0-indexed)
  is_visible: boolean;           // Show/hide individual links
  created_at: string;
  updated_at: string;
}
```

**Database Requirements:**

- Separate table: `{schema_name}.users_profile_links`
- Index on: `user_id`, `display_order`
- Constraint: Max 10 links per user
- URL validation (must be valid HTTPS)

**CRUD Operations:**

```
GET    /api/v1/profiles/{userId}/links           - List user's links
POST   /api/v1/profiles/{userId}/links           - Create link
PUT    /api/v1/profiles/{userId}/links/{linkId}  - Update link
DELETE /api/v1/profiles/{userId}/links/{linkId}  - Delete link
PATCH  /api/v1/profiles/{userId}/links/reorder   - Reorder links
```

### 2.4 Avatar Management

**Requirements:**

- File upload to S3-compatible storage or IPFS
- Image processing: resize to 256x256, 512x512, 1024x1024
- Supported formats: JPEG, PNG, WebP
- Max file size: 5 MB
- Default avatar: Gravatar or generated avatar (DiceBear/Boring Avatars)

**Endpoints:**

```
POST   /api/v1/avatars/{userId}     - Upload avatar (multipart/form-data)
DELETE /api/v1/avatars/{userId}     - Delete avatar (restore default)
```

### 2.5 Location Encoding

The frontend uses an encoded location format that combines coordinates with display name:

**Format:** `[latitude,longitude]Display Name`

**Example:** `[55.6761,12.5683]Havndal, Randers Municipality, Denmark`

**Parsing Functions (Backend):**

```typescript
// Extract display name for public display
function getLocationDisplayName(location: string): string {
  const match = location.match(/\](.+)$/);
  return match ? match[1] : location;
}

// Extract coordinates for geospatial queries
function parseLocationCoordinates(location: string): [number, number] | null {
  const match = location.match(/\[(-?\d+\.?\d*),(-?\d+\.?\d*)\]/);
  if (!match) return null;
  return [parseFloat(match[1]), parseFloat(match[2])];
}
```

**Database Storage:**

- Store full encoded string in `location` column (TEXT)
- Optional: Extract coordinates to PostGIS POINT column for geospatial queries
- Use PostGIS for location-based searches (nearby users, communities)

### 2.6 Profile API Endpoints

```
GET    /api/v1/profiles/{userId}           - Get user profile (privacy-aware)
GET    /api/v1/profiles/{userId}/full      - Get full profile (own profile only)
PUT    /api/v1/profiles/{userId}           - Update profile
PATCH  /api/v1/profiles/{userId}/privacy   - Update privacy settings
```

**Example Response (GET /api/v1/profiles/{userId}):**

```json
{
  "user_id": "uuid",
  "username": "alice_dk",
  "display_name": "Alice",
  "avatar_url": "https://cdn.unityplan.dk/avatars/uuid.jpg",
  "bio": "Permaculture enthusiast & community builder",
  "location": "Copenhagen, Denmark",
  "interests": ["permaculture", "community building", "education"],
  "skills": ["teaching", "gardening", "facilitation"],
  "languages": ["da", "en"],
  "created_at": "2025-01-15T10:00:00Z"
}
```

---

## 3. User Settings Management

### 3.1 Appearance Settings

**Data Model:**

```typescript
interface AppearanceSettings {
  theme_mode: 'light' | 'dark' | 'system';     // Theme mode preference
  color_scheme: 'forest-green' | 'ocean-blue' | 'royal-purple' | 'custom';  // Color palette (future feature)
  reduced_motion: boolean;                     // Accessibility: reduce animations
  wide_content_view: boolean;                  // Use full width for content
  compact_mode: boolean;                       // Collapse sidebar by default
}

interface LanguageSettings {
  preferred_language: string;                  // ISO 639-1 code (UI language)
  timezone: string;                            // IANA timezone
  auto_translate: boolean;                     // Auto-translate foreign content
  translation_provider: 'libre-translate' | 'deepl' | 'google' | 'microsoft';
  contribute_translations: boolean;            // Share translations with community
  fallback_to_english: boolean;                // Fallback if preferred unavailable
}

interface LanguageProficiency {
  id: string;
  language_code: string;                       // ISO 639-1
  language_name: string;                       // Display name
  spoken_level: ProficiencyLevel;
  written_level: ProficiencyLevel;
  reading_level: ProficiencyLevel;
  listening_level: ProficiencyLevel;
  display_order: number;                       // User's preference order
  is_preferred: boolean;                       // Primary language
  show_on_profile: boolean;
  created_at: string;
  updated_at: string;
}

type ProficiencyLevel = 'native' | 'fluent' | 'advanced' | 'intermediate' | 'basic' | 'learning';
```

**Storage:**

- Frontend: localStorage + cookies (for immediate UX)
- Backend: `{schema_name}.users_settings` table (sync across devices)
- Language proficiency: `{schema_name}.users_language_proficiency` table (public profile data)

### 3.2 Privacy Settings

See [2.2 Profile Privacy Settings](#22-profile-privacy-settings)

**Endpoint:**

```
GET    /api/v1/settings/privacy    - Get privacy settings
PATCH  /api/v1/settings/privacy    - Update privacy settings
```

### 3.3 Notification Settings

**Data Model:**

```typescript
interface NotificationSettings {
  // Email Notifications
  email_digest: boolean;           // Daily digest
  email_messages: boolean;         // New direct messages
  email_followers: boolean;        // New followers
  email_community: boolean;        // Community announcements
  email_updates: boolean;          // Platform updates
  
  // In-App Notifications
  inapp_messages: boolean;         // Direct messages
  inapp_followers: boolean;        // New followers
  inapp_community: boolean;        // Community activity
  inapp_mentions: boolean;         // Mentions in posts
  inapp_likes: boolean;            // Likes on posts
  
  // Push Notifications (Future)
  push_enabled: boolean;           // Master switch
  push_messages: boolean;
  push_followers: boolean;
  push_community: boolean;
}
```

**Database:**

- Table: `{schema_name}.users_notification_settings`
- One-to-one with users table

**Endpoints:**

```
GET    /api/v1/settings/notifications    - Get notification settings
PATCH  /api/v1/settings/notifications    - Update notification settings
```

### 3.4 Account Settings

**Email Change:**

```
POST   /api/v1/account/email/change      - Request email change
POST   /api/v1/account/email/verify      - Verify new email
```

**Password Change:**

```
POST   /api/v1/account/password/change   - Change password
```

**Two-Factor Authentication (TOTP):**

```
POST   /api/v1/account/totp/enable       - Enable TOTP (returns QR code)
POST   /api/v1/account/totp/verify       - Verify TOTP setup
POST   /api/v1/account/totp/disable      - Disable TOTP
GET    /api/v1/account/totp/recovery     - Get recovery codes
```

**Data Model (TOTP):**

```typescript
interface TOTPSettings {
  user_id: string;
  secret: string;                  // Encrypted TOTP secret
  enabled: boolean;
  verified_at: string | null;
  recovery_codes: string[];        // Hashed recovery codes
  created_at: string;
}
```

---

## 4. Invitation System

### 4.1 Invitation Token Entity

See [docs/architecture/invitation-system.md](invitation-system.md) for complete specification.

**Summary:**

- **Territory Tokens:** Stored in `territory_dk.invitation_tokens`
- **Global Registry:** `global.invitation_token_registry` for territory binding
- **Token Types:** `single_use` (personal) and `group` (multi-use)
- **Security:** Database-enforced territory binding (client cannot manipulate)

### 4.2 Invitation API Endpoints

```
GET    /api/v1/invitations/validate/{token}  - Validate token & get territory info
POST   /api/v1/invitations/create            - Create invitation (requires permission)
GET    /api/v1/invitations/my-invitations    - List user's created invitations
DELETE /api/v1/invitations/{tokenId}         - Revoke invitation
GET    /api/v1/invitations/{tokenId}/uses    - View invitation usage (audit)
```

### 4.3 Invitation Creation

**Request:**

```typescript
interface CreateInvitationRequest {
  token_type: 'single_use' | 'group';
  email?: string;                  // Required for single_use
  max_uses?: number;               // Required for group (default: 1)
  expires_at?: string;             // ISO 8601 (default: 7 days)
  community_id?: string;           // Optional: auto-assign to community
  purpose?: string;                // Optional: description
}
```

**Response:**

```typescript
interface InvitationToken {
  id: string;
  token: string;                   // Share this with invitee
  token_type: string;
  email?: string | null;
  max_uses: number;
  used_count: number;
  remaining_uses: number;
  expires_at: string;
  created_at: string;
  invitation_url: string;          // Full registration URL with token
}
```

---

## 5. Data Sovereignty & GDPR

### 5.1 Data Export (GDPR Article 20)

**Requirements:**

- Export all user data in machine-readable format (JSON)
- Include: profile, settings, posts, messages, community memberships, activity logs
- Asynchronous processing (background job)
- Email download link when ready
- Retention: 7 days after generation

**Endpoint:**

```
POST   /api/v1/data/export          - Request data export
GET    /api/v1/data/export/status   - Check export status
GET    /api/v1/data/export/download - Download export file
```

**Export Format:**

```json
{
  "export_date": "2025-11-11T12:00:00Z",
  "user": { "id": "...", "username": "...", ... },
  "profile": { ... },
  "settings": { ... },
  "posts": [ ... ],
  "messages": [ ... ],
  "communities": [ ... ],
  "activity_log": [ ... ]
}
```

### 5.2 Account Deletion (GDPR Article 17)

**Requirements:**

- Soft delete (mark as deleted, retain for 30 days)
- Hard delete after 30 days (background job)
- Anonymize content instead of deleting (preserve community discussions)
- Email confirmation required
- Export data before deletion

**Endpoint:**

```
POST   /api/v1/account/delete           - Request account deletion
POST   /api/v1/account/delete/confirm   - Confirm deletion (email token)
POST   /api/v1/account/delete/cancel    - Cancel pending deletion
```

**Anonymization Strategy:**

- Posts/comments: Replace `user_id` with `[deleted]`, keep content
- Messages: Delete content, keep metadata for recipient
- Profile: Delete all data
- Audit logs: Keep for legal compliance (90 days)

---

## Database Schema Considerations for Holochain Migration

### Multi-Tenancy Requirements

**Current Architecture:**

- PostgreSQL with territory schemas (`territory_dk`, `territory_no`, etc.)
- Global schema for cross-territory data (`global.territories`, `global.invitation_token_registry`)

**Holochain Migration Readiness:**

1. **Territory Isolation:**
   - Each territory = separate Holochain DNA instance
   - Data sovereignty maintained at territory level
   - Cross-territory lookups via DHT queries

2. **Data Structure Mapping:**
   - PostgreSQL tables → Holochain Entry Types
   - Foreign keys → Links in Holochain
   - JSONB columns → Nested entries or serialized data

3. **Schema Design Principles:**
   - **Immutable audit trails:** Use append-only logs (maps to Holochain's source chain)
   - **Agent-centric data:** User owns their profile (maps to Holochain agent entries)
   - **Linked data:** Use FKs that translate to Holochain links
   - **Event sourcing:** Store state changes as events (Holochain is event-sourced by default)

4. **Key Entities to Map:**

   ```
   PostgreSQL                            →  Holochain
   ──────────────────────────────────────────────────────────
   {schema_name}.users                   →  Agent Entry (built-in)
   {schema_name}.users_profiles          →  Profile Entry Type
   {schema_name}.users_profile_links     →  ProfileLink Entry Type
   {schema_name}.posts                   →  Post Entry Type
   {schema_name}.communities             →  Community Entry Type
   global.invitation_tokens              →  Bridge between DNAs
   ```

5. **Validation Rules:**
   - PostgreSQL constraints → Holochain validation functions
   - Unique constraints → DHT collision detection
   - Foreign key checks → Link validation

6. **Migration Strategy:**
   - Phase 1: PostgreSQL (current)
   - Phase 2: Dual-write (PostgreSQL + Holochain)
   - Phase 3: Read from Holochain, fallback to PostgreSQL
   - Phase 4: Full Holochain (PostgreSQL archive only)

---

## Next Steps

1. **Database Schema Design:**
   - Create detailed SQL schemas for all entities
   - Design indexes and constraints
   - Plan migration scripts from current state

2. **API Implementation Priority:**
   - Priority 1: Authentication & Registration (blocks all other work)
   - Priority 2: Profile Management (user-facing features)
   - Priority 3: Settings & Privacy (essential for user sovereignty)
   - Priority 4: Invitations (community growth)
   - Priority 5: Data Export & GDPR (compliance)

3. **Holochain Preparation:**
   - Design Entry Type schemas
   - Plan validation logic
   - Map relationships to Links
   - Design DHT query patterns

---

**Related Documents:**

- [Invitation System Architecture](invitation-system.md)
- [Multi-Pod Architecture](multi-pod-architecture.md)
- [User Data Sovereignty](user-data-sovereignty.md)
- [Territory Management Standard](territory-management-standard.md)
