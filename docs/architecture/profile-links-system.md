# Profile Links System

**Status:** Planned  
**Category:** Architecture / Database Schema  
**Version:** 0.1.0-alpha.1

## Overview

User profiles support custom external links (social media, websites, portfolios, etc.) through a flexible link management system. Instead of hardcoded fields for specific platforms, users can add any links they want with custom labels and icons.

## Problem Statement

The original design included hardcoded social link fields in the user profile:
- `website_url`
- `github_url`
- `linkedin_url`
- `twitter_handle`

**Issues with this approach:**
1. ❌ Not user-sovereign - users limited to predefined platforms
2. ❌ Inflexible - can't add other platforms (Mastodon, GitLab, YouTube, etc.)
3. ❌ Wasteful - creates null fields for links users don't use
4. ❌ Not scalable - requires schema changes to add new platforms

## Solution: Custom Profile Links Table

### Database Schema

#### New Table: `profile_links`

```sql
CREATE TABLE profile_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    label VARCHAR(100) NOT NULL,          -- User-defined label (e.g., "My GitHub", "Portfolio")
    url VARCHAR(500) NOT NULL,            -- Full URL to external resource
    icon VARCHAR(100),                    -- Icon identifier (e.g., "fa-github", "fa-linkedin")
    display_order INT NOT NULL DEFAULT 0, -- Order to display links (0 = first)
    is_visible BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT profile_links_user_id_fkey 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT profile_links_url_format 
        CHECK (url ~* '^https?://'),
    CONSTRAINT profile_links_label_length 
        CHECK (char_length(label) >= 1 AND char_length(label) <= 100),
    CONSTRAINT profile_links_display_order_positive 
        CHECK (display_order >= 0)
);

-- Indexes
CREATE INDEX idx_profile_links_user_id ON profile_links(user_id);
CREATE INDEX idx_profile_links_display_order ON profile_links(user_id, display_order);
CREATE INDEX idx_profile_links_visible ON profile_links(user_id, is_visible);
```

#### Migration Strategy

**Phase 1: Add new table** (Immediate)
- Create `profile_links` table
- Keep existing social link fields temporarily

**Phase 2: Data migration** (Before removing old fields)
```sql
-- Migrate existing social links to new table
INSERT INTO profile_links (user_id, label, url, icon, display_order, is_visible)
SELECT 
    id as user_id,
    'Website' as label,
    website_url as url,
    'fa-globe' as icon,
    0 as display_order,
    true as is_visible
FROM user_profiles
WHERE website_url IS NOT NULL;

INSERT INTO profile_links (user_id, label, url, icon, display_order, is_visible)
SELECT 
    id as user_id,
    'GitHub' as label,
    github_url as url,
    'fa-github' as icon,
    1 as display_order,
    true as is_visible
FROM user_profiles
WHERE github_url IS NOT NULL;

INSERT INTO profile_links (user_id, label, url, icon, display_order, is_visible)
SELECT 
    id as user_id,
    'LinkedIn' as label,
    linkedin_url as url,
    'fa-linkedin' as icon,
    2 as display_order,
    true as is_visible
FROM user_profiles
WHERE linkedin_url IS NOT NULL;

INSERT INTO profile_links (user_id, label, url, icon, display_order, is_visible)
SELECT 
    id as user_id,
    'Twitter' as label,
    CASE 
        WHEN twitter_handle LIKE 'http%' THEN twitter_handle
        WHEN twitter_handle LIKE '@%' THEN 'https://twitter.com/' || SUBSTRING(twitter_handle FROM 2)
        ELSE 'https://twitter.com/' || twitter_handle
    END as url,
    'fa-twitter' as icon,
    3 as display_order,
    true as is_visible
FROM user_profiles
WHERE twitter_handle IS NOT NULL;
```

**Phase 3: Remove old fields** (After migration verified)
```sql
ALTER TABLE user_profiles DROP COLUMN website_url;
ALTER TABLE user_profiles DROP COLUMN github_url;
ALTER TABLE user_profiles DROP COLUMN linkedin_url;
ALTER TABLE user_profiles DROP COLUMN twitter_handle;
```

## API Endpoints

### User Service Endpoints

#### List User's Profile Links
```http
GET /api/v1/profiles/{user_id}/links
Authorization: Bearer {token}
```

**Response:**
```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "user_id": "123e4567-e89b-12d3-a456-426614174000",
      "label": "My GitHub",
      "url": "https://github.com/username",
      "icon": "fa-github",
      "display_order": 0,
      "is_visible": true,
      "created_at": "2025-01-15T10:00:00Z",
      "updated_at": "2025-01-15T10:00:00Z"
    }
  ]
}
```

#### Create Profile Link
```http
POST /api/v1/profiles/{user_id}/links
Authorization: Bearer {token}
Content-Type: application/json

{
  "label": "My Portfolio",
  "url": "https://example.com",
  "icon": "fa-briefcase",
  "display_order": 0,
  "is_visible": true
}
```

**Response:** `201 Created` with link object

#### Update Profile Link
```http
PUT /api/v1/profiles/{user_id}/links/{link_id}
Authorization: Bearer {token}
Content-Type: application/json

{
  "label": "Updated Label",
  "url": "https://new-url.com",
  "icon": "fa-star",
  "display_order": 1,
  "is_visible": false
}
```

**Response:** `200 OK` with updated link object

#### Delete Profile Link
```http
DELETE /api/v1/profiles/{user_id}/links/{link_id}
Authorization: Bearer {token}
```

**Response:** `204 No Content`

#### Reorder Profile Links
```http
PATCH /api/v1/profiles/{user_id}/links/reorder
Authorization: Bearer {token}
Content-Type: application/json

{
  "link_ids": [
    "uuid-1",  // Will become display_order 0
    "uuid-2",  // Will become display_order 1
    "uuid-3"   // Will become display_order 2
  ]
}
```

**Response:** `200 OK`

## Frontend Implementation

### Profile Edit Page - Links Section

**Component Design:**

```tsx
// ProfileLinksManager component
<Card>
  <CardHeader>
    <CardTitle>External Links</CardTitle>
    <CardDescription>
      Add links to your social profiles, websites, or portfolios
    </CardDescription>
  </CardHeader>
  <CardContent>
    {/* Existing Links */}
    <div className="space-y-2">
      {links.map((link, index) => (
        <div key={link.id} className="flex items-center gap-2 p-3 border rounded-lg">
          {/* Icon Selector */}
          <IconPicker value={link.icon} onChange={(icon) => updateLink(link.id, { icon })} />
          
          {/* Label Input */}
          <Input 
            value={link.label} 
            onChange={(e) => updateLink(link.id, { label: e.target.value })}
            placeholder="Label (e.g., GitHub)"
          />
          
          {/* URL Input */}
          <Input 
            type="url"
            value={link.url} 
            onChange={(e) => updateLink(link.id, { url: e.target.value })}
            placeholder="https://..."
          />
          
          {/* Reorder Buttons */}
          <Button size="sm" onClick={() => moveUp(index)}>↑</Button>
          <Button size="sm" onClick={() => moveDown(index)}>↓</Button>
          
          {/* Delete Button */}
          <Button size="sm" variant="destructive" onClick={() => deleteLink(link.id)}>
            <Trash2 />
          </Button>
        </div>
      ))}
    </div>
    
    {/* Add New Link Button */}
    <Button onClick={addNewLink} variant="outline" className="w-full mt-4">
      <Plus className="mr-2" />
      Add Link
    </Button>
  </CardContent>
</Card>
```

### Icon Library

**Font Awesome Free Brands Icons:**
https://fontawesome.com/v6/search?ip=brands&ic=free&o=r

**Common Icons:**
- `fa-github` - GitHub
- `fa-gitlab` - GitLab
- `fa-linkedin` - LinkedIn
- `fa-twitter` / `fa-x-twitter` - Twitter/X
- `fa-mastodon` - Mastodon
- `fa-youtube` - YouTube
- `fa-facebook` - Facebook
- `fa-instagram` - Instagram
- `fa-discord` - Discord
- `fa-telegram` - Telegram
- `fa-reddit` - Reddit
- `fa-stack-overflow` - Stack Overflow
- `fa-codepen` - CodePen
- `fa-dev` - Dev.to
- `fa-medium` - Medium
- `fa-globe` - Website
- `fa-envelope` - Email
- `fa-briefcase` - Portfolio

**Implementation Options:**

1. **React Icons** (recommended for UnityPlan)
   - Package: `react-icons`
   - Import: `import { FaGithub, FaLinkedin } from 'react-icons/fa6';`
   - Pros: Tree-shakeable, TypeScript support, includes Font Awesome

2. **Font Awesome React**
   - Package: `@fortawesome/react-fontawesome`
   - More heavyweight but official

3. **Lucide React** (already in use)
   - For generic icons (Globe, Mail, Briefcase)
   - Combine with react-icons for brand icons

### Icon Picker Component

```tsx
import { useState } from 'react';
import { FaGithub, FaLinkedin, FaTwitter, FaGlobe, FaBriefcase, /* ... */ } from 'react-icons/fa6';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Button } from '@/components/ui/button';

const ICON_OPTIONS = [
  { id: 'fa-github', icon: FaGithub, label: 'GitHub' },
  { id: 'fa-linkedin', icon: FaLinkedin, label: 'LinkedIn' },
  { id: 'fa-twitter', icon: FaTwitter, label: 'Twitter/X' },
  { id: 'fa-globe', icon: FaGlobe, label: 'Website' },
  { id: 'fa-briefcase', icon: FaBriefcase, label: 'Portfolio' },
  // ... more icons
];

export function IconPicker({ value, onChange }: { value: string; onChange: (icon: string) => void }) {
  const selectedIcon = ICON_OPTIONS.find(opt => opt.id === value);
  const IconComponent = selectedIcon?.icon || FaGlobe;
  
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline" size="sm" className="w-12 h-12">
          <IconComponent className="size-5" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80">
        <div className="grid grid-cols-5 gap-2">
          {ICON_OPTIONS.map((option) => (
            <Button
              key={option.id}
              variant={value === option.id ? 'default' : 'outline'}
              size="sm"
              onClick={() => onChange(option.id)}
              title={option.label}
            >
              <option.icon className="size-5" />
            </Button>
          ))}
        </div>
      </PopoverContent>
    </Popover>
  );
}
```

## Profile View Display

**Profile View Page - Links Section:**

```tsx
{/* External Links */}
{links.length > 0 && (
  <div className="flex flex-wrap gap-2">
    {links
      .filter(link => link.is_visible)
      .sort((a, b) => a.display_order - b.display_order)
      .map((link) => {
        const IconComponent = getIconComponent(link.icon);
        return (
          <a
            key={link.id}
            href={link.url}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 px-3 py-1.5 rounded-md border border-input bg-background hover:bg-accent hover:text-accent-foreground transition-colors"
          >
            <IconComponent className="size-4" />
            <span className="text-sm">{link.label}</span>
          </a>
        );
      })}
  </div>
)}
```

## Backend Implementation

### Rust Data Structures

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProfileLink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub label: String,
    pub url: String,
    pub icon: Option<String>,
    pub display_order: i32,
    pub is_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileLinkRequest {
    pub label: String,
    pub url: String,
    pub icon: Option<String>,
    pub display_order: Option<i32>,
    pub is_visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileLinkRequest {
    pub label: Option<String>,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub display_order: Option<i32>,
    pub is_visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderLinksRequest {
    pub link_ids: Vec<Uuid>,
}
```

### Validation Rules

```rust
impl CreateProfileLinkRequest {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Label validation
        if self.label.is_empty() || self.label.len() > 100 {
            return Err(ValidationError::new("label must be 1-100 characters"));
        }
        
        // URL validation
        if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
            return Err(ValidationError::new("url must start with http:// or https://"));
        }
        if self.url.len() > 500 {
            return Err(ValidationError::new("url must be less than 500 characters"));
        }
        
        // Icon validation (optional)
        if let Some(icon) = &self.icon {
            if icon.len() > 100 {
                return Err(ValidationError::new("icon must be less than 100 characters"));
            }
        }
        
        // Display order validation
        if let Some(order) = self.display_order {
            if order < 0 {
                return Err(ValidationError::new("display_order must be non-negative"));
            }
        }
        
        Ok(())
    }
}
```

## Security Considerations

1. **Authorization**: Only the profile owner can manage their links
2. **URL Validation**: Ensure URLs use http/https protocols
3. **Rate Limiting**: Limit link creation (e.g., max 20 links per user)
4. **XSS Prevention**: Sanitize labels before display
5. **Link Verification**: Optional - verify URLs are accessible before saving

## Benefits

✅ **User Sovereignty**: Users control what links to share  
✅ **Flexibility**: Add any platform, not just predefined ones  
✅ **Scalability**: No schema changes needed for new platforms  
✅ **Efficiency**: Only store links users actually use  
✅ **Customization**: Users choose labels and order  
✅ **Privacy**: Users can hide links without deleting them  

## Implementation Timeline

1. **Phase 1** (Current Sprint): Create database schema and migration
2. **Phase 2** (Next Sprint): Backend API endpoints
3. **Phase 3** (Future Sprint): Frontend UI components
4. **Phase 4** (Future Sprint): Migrate existing data and remove old fields

## Related Documentation

- `docs/architecture/infrastructure.md` - Database architecture
- `docs/guides/development/database-migrations.md` - Migration procedures
- `services/user-service/README.md` - User service documentation
- `frontend/src/api/users.ts` - Frontend API client

## References

- Font Awesome Icons: https://fontawesome.com/v6/search?ip=brands&ic=free&o=r
- React Icons Library: https://react-icons.github.io/react-icons/
