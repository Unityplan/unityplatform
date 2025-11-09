# Component Registry Guide

Guide for managing UI components in UnityPlan frontend.

**Status:** Using shadcn/ui (copy-paste components)  
**Future:** May create local registry for custom components  
**Last Updated:** November 9, 2025

---

## 📋 Table of Contents

- [Current Approach: shadcn/ui](#current-approach-shadcnui)
- [shadcn/ui MCP Integration](#shadcnui-mcp-integration)
- [Component Organization](#component-organization)
- [Future: Local Component Registry](#future-local-component-registry)

---

## Current Approach: shadcn/ui

We use **shadcn/ui** - a collection of re-usable components that you copy into your project.

**Philosophy:**

- ✅ Copy components into your codebase (you own the code)
- ✅ Customize freely without forking
- ✅ Built on Radix UI primitives
- ✅ Styled with TailwindCSS
- ✅ Fully accessible (ARIA)

**Why shadcn/ui?**

- Full control over component code
- No package version conflicts
- Easy to customize for our needs
- Great TypeScript support
- Excellent accessibility out of the box

---

## shadcn/ui MCP Integration

The shadcn/ui Model Context Protocol (MCP) server is now configured in VS Code.

**What it provides:**

- Component browsing directly in GitHub Copilot Chat
- Installation commands
- Component documentation
- Usage examples

**How to use:**

1. **In Copilot Chat, ask about shadcn components:**

   ```
   @shadcn-ui Show me available button components
   @shadcn-ui How do I install the dialog component?
   @shadcn-ui What props does the card component accept?
   ```

2. **Install components:**

   ```bash
   # The MCP will suggest commands like:
   npx shadcn@latest add button
   npx shadcn@latest add card
   npx shadcn@latest add dialog
   ```

3. **Get component examples:**
   - Ask for usage examples
   - Get prop documentation
   - See variant options

**Configuration (already set in `.vscode/settings.json`):**

```json
{
  "github.copilot.chat.mcp.enabled": true,
  "github.copilot.chat.mcp.servers": {
    "shadcn-ui": {
      "command": "npx",
      "args": ["-y", "@shadcn/mcp"]
    }
  }
}
```

---

## Component Organization

### Current Structure

```
frontend/src/
├── components/
│   ├── ui/                 # shadcn/ui components (installed)
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── input.tsx
│   │   ├── dialog.tsx
│   │   └── ...
│   │
│   ├── Avatar.tsx          # Custom components
│   ├── UserCard.tsx        # Compositions using ui components
│   ├── ProfileHeader.tsx
│   └── ...
```

### Component Categories

**1. Base UI Components (`components/ui/`)**

- From shadcn/ui
- Low-level, reusable
- Examples: Button, Input, Card, Dialog

**2. Custom Components (`components/`)**

- Built using base UI components
- Business logic specific
- Examples: UserCard, ProfileHeader, PostCard

**3. Page Components (`pages/`)**

- Full page layouts
- Compose multiple components
- Examples: LoginPage, ProfilePage

### Adding shadcn Components

```bash
# Browse available components
npx shadcn@latest list

# Add a component
npx shadcn@latest add button

# Add multiple components
npx shadcn@latest add button card input dialog

# Component is copied to src/components/ui/
# You can now customize it freely
```

### Creating Custom Components

```tsx
// src/components/UserCard.tsx
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';

export function UserCard({ user }) {
  return (
    <Card>
      <CardHeader>
        <Avatar>
          <AvatarImage src={user.avatar_url} />
          <AvatarFallback>{user.username[0]}</AvatarFallback>
        </Avatar>
        <h3>{user.username}</h3>
      </CardHeader>
      <CardContent>
        <p>{user.email}</p>
        <Button>View Profile</Button>
      </CardContent>
    </Card>
  );
}
```

---

## Future: Local Component Registry

### When to Consider a Local Registry

**Consider if:**

- ✅ We have 20+ custom components used across multiple projects
- ✅ We need version control for internal components
- ✅ Multiple teams need shared component library
- ✅ We want to publish components internally

**Don't need if:**

- ❌ Only one frontend project (current state)
- ❌ Components are project-specific
- ❌ Team is small (< 5 developers)

### Potential Options

#### Option 1: Internal npm Registry (Verdaccio)

**Pros:**

- Standard npm workflow
- Version management
- Private packages
- Easy to set up

**Cons:**

- Requires hosting
- Package management overhead
- May be overkill for small team

**Setup:**

```bash
# Install Verdaccio
npm install -g verdaccio

# Run local registry
verdaccio

# Configure npm to use local registry
npm set registry http://localhost:4873/
```

#### Option 2: Git Submodules

**Pros:**

- No additional infrastructure
- Version control built-in
- Simple for small teams

**Cons:**

- Submodule management complexity
- Not ideal for frequent changes
- Manual synchronization

#### Option 3: Monorepo with Turborepo/Nx

**Pros:**

- Single repo for all projects
- Shared components automatically synced
- Great developer experience
- Built-in caching and build optimization

**Cons:**

- Larger repo size
- Requires different CI/CD setup
- More complex initial setup

**Example structure:**

```
unityplan-monorepo/
├── apps/
│   ├── web/              # Main frontend
│   ├── mobile/           # Future mobile app
│   └── admin/            # Admin dashboard
├── packages/
│   ├── ui/               # Shared UI components
│   ├── utils/            # Shared utilities
│   └── types/            # Shared TypeScript types
└── package.json
```

### Recommendation for Now

**Stick with shadcn/ui approach:**

1. We have only one frontend project
2. Team is small
3. Can easily copy components between projects if needed later
4. shadcn/ui provides excellent foundation

**When to revisit:**

- When we start a second frontend project (mobile app, admin dashboard)
- When we have 30+ custom components to share
- When we have 5+ frontend developers

### Migration Path

If we decide to create a local registry later:

**Step 1:** Extract custom components to separate package

```bash
# Create packages/ui
mkdir -p packages/ui/src/components
mv frontend/src/components/UserCard.tsx packages/ui/src/components/
```

**Step 2:** Set up build process

```json
// packages/ui/package.json
{
  "name": "@unityplan/ui",
  "version": "0.1.0",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsup src/index.ts --format esm,cjs --dts"
  }
}
```

**Step 3:** Publish to internal registry or use as local package

```bash
# Publish to Verdaccio
npm publish --registry http://localhost:4873/

# Or use as local package
npm install ../packages/ui
```

---

## Best Practices

### Component Guidelines

**Do:**

- ✅ Keep components focused (single responsibility)
- ✅ Use TypeScript for all components
- ✅ Document props with JSDoc
- ✅ Follow accessibility guidelines
- ✅ Write tests for custom components

**Don't:**

- ❌ Mix business logic in UI components
- ❌ Create overly generic "god components"
- ❌ Skip TypeScript types
- ❌ Forget keyboard navigation
- ❌ Skip testing

### Naming Conventions

```tsx
// ✅ Good: Clear, descriptive names
UserProfileCard.tsx
PostListItem.tsx
CommentThread.tsx

// ❌ Bad: Vague or too generic
Card.tsx (conflicts with shadcn/ui)
Item.tsx
Thing.tsx
```

### Documentation

Document custom components with JSDoc:

```tsx
/**
 * Displays user information in a card format
 * 
 * @param user - User object with id, username, email, avatar_url
 * @param onEdit - Callback when edit button is clicked
 * 
 * @example
 * ```tsx
 * <UserCard 
 *   user={currentUser} 
 *   onEdit={() => navigate('/edit')} 
 * />
 * ```
 */
export function UserCard({ user, onEdit }: UserCardProps) {
  // ...
}
```

---

## Related Documentation

- [Development Guide](./development-guide.md) - Project setup
- [Component Patterns](./component-patterns.md) - Component best practices
- [shadcn/ui Docs](https://ui.shadcn.com) - Official documentation
- [Radix UI Docs](https://www.radix-ui.com) - Primitive components

---

**Current Status:** Using shadcn/ui with MCP integration for easy component discovery and installation. Local registry deferred until we have multiple frontend projects or larger team.
