# Compass-TS Template Application Guide

**Project**: UnityPlan Frontend  
**Template Source**: compass-ts (Next.js LMS template)  
**Target Framework**: Vite + React 18 + TanStack Router  
**Date**: November 9, 2025

---

## Overview

This guide provides a step-by-step process to port the compass-ts template's design system, layouts, and components to the UnityPlan frontend project.

### Compatibility Summary

| Feature | Compass-TS | UnityPlan | Status |
|---------|------------|-----------|--------|
| React | 19.x | 18.x | ⚠️ Minor (no breaking changes needed) |
| TailwindCSS | 4.1.11 | 4.1.17 | ✅ Fully compatible |
| TypeScript | 5.8.3 | 5.9.3 | ✅ Fully compatible |
| PostCSS | 8.5.6 | 8.5.6 | ✅ Identical |
| clsx | 2.1.1 | 2.1.1 | ✅ Identical |

### Key Adaptations Required

1. **Routing**: Next.js → TanStack Router
2. **UI Primitives**: Headless UI → Radix UI (already installed)
3. **Font Loading**: Next.js localFont → Vite static assets
4. **Dialogs/Dropdowns**: Port to Radix UI equivalents

---

## Phase 1: Preparation & Setup

### Step 1.1: Git Commit Current State

```bash
cd /home/henrik/projects/unityplan_platform/workspace
git add .
git commit -m "feat(frontend): prepare for compass-ts template integration

- All tests passing (6 passed, 6 skipped, 3 todo)
- Profile system working with DB triggers
- Ready for design system upgrade"
git push origin main
```

### Step 1.2: Install Required Dependencies

```bash
cd frontend

# Install Radix UI components needed for Headless UI replacements
npm install @radix-ui/react-dialog@latest
npm install @radix-ui/react-dropdown-menu@latest

# Optional: Install geist font (matches template)
npm install geist@latest
```

### Step 1.3: Create Directory Structure

```bash
# Create new directories for template components
mkdir -p src/components/layouts
mkdir -p src/components/ui
mkdir -p src/styles
mkdir -p public/fonts
```

---

## Phase 2: Typography & Core Styles

### Step 2.1: Port Typography CSS

**File**: `frontend/src/styles/typography.css`

Copy from: `temp/compass-ts/src/app/typography.css`

**Adaptations needed**:

- Replace `var(--color-gray-700)` with your theme colors from `index.css`
- Adjust spacing to use your existing spacing scale
- Keep the `.prose` class structure (excellent for content pages)

**Action**:

1. Create `frontend/src/styles/typography.css`
2. Copy content from template
3. Update color variables to match your theme:
   - `--color-gray-950` → `var(--foreground)`
   - `--color-gray-700` → `var(--muted-foreground)`
   - `--color-gray-400` → lighter variant
   - `--color-white` → `var(--background)` (in dark mode)

### Step 2.2: Update Main CSS File

**File**: `frontend/src/index.css`

**Add after TailwindCSS import**:

```css
@import "tailwindcss";
@import "./styles/typography.css";

/* Add compass-ts theme variables */
@theme inline {
  --font-sans: var(--font-inter);
  --font-sans--font-feature-settings: "cv11";
  --font-mono: var(--font-geist-mono);
}

/* Custom text sizes from compass-ts (optional - only if you want their scale) */
@theme {
  --text-xs: 0.75rem;
  --text-xs--line-height: calc(1 / 0.75);
  --text-sm: 0.875rem;
  --text-sm--line-height: calc(1.25 / 0.875);
  /* ... add more as needed */
}
```

### Step 2.3: Download & Add Fonts

**Fonts used in template**:

- Inter Variable (primary sans-serif)
- Geist Mono (code/monospace)

**Option A: Use geist npm package** (Recommended)

```bash
npm install geist
```

Then import in `main.tsx`:

```tsx
import { GeistSans } from 'geist/font/sans';
import { GeistMono } from 'geist/font/mono';
```

**Option B: Download Inter Variable manually**

1. Download from Google Fonts: <https://fonts.google.com/specimen/Inter>
2. Place `.woff2` files in `public/fonts/`
3. Add `@font-face` declarations to CSS

---

## Phase 3: Layout Components

### Step 3.1: Create Base Layout Component

**File**: `frontend/src/components/layouts/BaseLayout.tsx`

**Purpose**: Provides font classes and basic structure

```tsx
import { clsx } from 'clsx';
import React from 'react';

interface BaseLayoutProps {
  children: React.ReactNode;
  className?: string;
}

export function BaseLayout({ children, className }: BaseLayoutProps) {
  return (
    <div className={clsx('min-h-screen font-sans antialiased', className)}>
      <div className="isolate">{children}</div>
    </div>
  );
}
```

### Step 3.2: Port Navbar Component

**File**: `frontend/src/components/layouts/Navbar.tsx`

**Source**: `temp/compass-ts/src/components/navbar.tsx`

**Adaptations**:

1. Replace `@headlessui/react` Dialog with Radix Dialog:

   ```tsx
   import * as Dialog from '@radix-ui/react-dialog';
   ```

2. Replace Next.js `Link` with TanStack Router `Link`:

   ```tsx
   import { Link } from '@tanstack/react-router';
   ```

3. Replace `useState` for navigation state (same)

4. Update navigation links to your routes:
   - `["/", "Dashboard"]`
   - `["/profile", "Profile"]`
   - etc.

**Steps**:

1. Copy `navbar.tsx` → `frontend/src/components/layouts/Navbar.tsx`
2. Update imports (Dialog, Link, icons)
3. Update navigation structure to match your app
4. Test mobile menu functionality

### Step 3.3: Port Centered Layout

**File**: `frontend/src/components/layouts/CenteredLayout.tsx`

**Source**: `temp/compass-ts/src/components/centered-layout.tsx`

**Usage**: Perfect for login, register, and simple pages

**Adaptations**: Minimal - just verify Navbar import

```tsx
import React from 'react';
import { Navbar } from './Navbar';

export function CenteredLayout({
  breadcrumbs,
  children,
}: {
  breadcrumbs?: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <div className="pb-30">
      <Navbar>
        {breadcrumbs && <div className="min-w-0">{breadcrumbs}</div>}
      </Navbar>
      <div className="px-4 sm:px-6">
        <div className="mx-auto max-w-6xl">{children}</div>
      </div>
    </div>
  );
}
```

### Step 3.4: Port Sidebar Layout

**File**: `frontend/src/components/layouts/SidebarLayout.tsx`

**Source**: `temp/compass-ts/src/components/sidebar-layout.tsx`

**Usage**: Perfect for dashboard, profile, settings pages

**Major Adaptations**:

1. Replace Dialog with Radix UI Dialog
2. Replace `usePathname()` with TanStack Router:

   ```tsx
   import { useMatchRoute, Link } from '@tanstack/react-router';
   
   const matchRoute = useMatchRoute();
   const isActive = matchRoute({ to: lesson.path });
   ```

3. Update navigation structure to match your app modules:

   ```tsx
   const modules = [
     {
       id: 'account',
       title: 'Account',
       pages: [
         { id: 'dashboard', title: 'Dashboard', path: '/dashboard' },
         { id: 'profile', title: 'Profile', path: '/profile' },
       ]
     }
   ];
   ```

**Steps**:

1. Copy `sidebar-layout.tsx` → `frontend/src/components/layouts/SidebarLayout.tsx`
2. Replace Headless UI Dialog with Radix Dialog
3. Update routing logic for TanStack Router
4. Update navigation modules for your app
5. Test sidebar toggle and mobile dialog

---

## Phase 4: UI Components

### Step 4.1: Port Icon Button

**File**: `frontend/src/components/ui/IconButton.tsx`

**Source**: `temp/compass-ts/src/components/icon-button.tsx`

Simple copy - should work as-is.

### Step 4.2: Port Button Component

**File**: `frontend/src/components/ui/Button.tsx`

**Source**: `temp/compass-ts/src/components/button.tsx`

Compare with your existing shadcn Button - merge best features.

### Step 4.3: Port Dropdown Component

**File**: `frontend/src/components/ui/Dropdown.tsx`

**Source**: `temp/compass-ts/src/components/dropdown.tsx`

**Adaptation**: Replace Headless UI Menu with Radix DropdownMenu

```tsx
import * as DropdownMenu from '@radix-ui/react-dropdown-menu';

export function Dropdown({ children }: { children: React.ReactNode }) {
  return <DropdownMenu.Root>{children}</DropdownMenu.Root>;
}

export function DropdownButton({ children, ...props }: any) {
  return <DropdownMenu.Trigger {...props}>{children}</DropdownMenu.Trigger>;
}

export function DropdownMenu({ children }: { children: React.ReactNode }) {
  return (
    <DropdownMenu.Portal>
      <DropdownMenu.Content className="...styles...">
        {children}
      </DropdownMenu.Content>
    </DropdownMenu.Portal>
  );
}

export function DropdownItem({ children, ...props }: any) {
  return <DropdownMenu.Item {...props}>{children}</DropdownMenu.Item>;
}
```

### Step 4.4: Port Input Component

**File**: `frontend/src/components/ui/Input.tsx`

**Source**: `temp/compass-ts/src/components/input.tsx`

Compare with your existing shadcn Input - merge or replace.

### Step 4.5: Copy Icon Components

**Directory**: `frontend/src/components/icons/`

**Source**: `temp/compass-ts/src/icons/`

Copy all icon components - they're framework-agnostic SVG components.

---

## Phase 5: Integration with Existing Pages

### Step 5.1: Update Login Page

**File**: `frontend/src/pages/auth/LoginPage.tsx`

**Wrap with CenteredLayout**:

```tsx
import { CenteredLayout } from '@/components/layouts/CenteredLayout';

export function LoginPage() {
  return (
    <CenteredLayout>
      {/* Existing login form */}
    </CenteredLayout>
  );
}
```

### Step 5.2: Update Dashboard Page

**File**: `frontend/src/pages/dashboard/DashboardPage.tsx` (create if needed)

**Wrap with SidebarLayout**:

```tsx
import { SidebarLayout } from '@/components/layouts/SidebarLayout';

const modules = [
  {
    id: 'main',
    title: 'Main',
    pages: [
      { id: 'dashboard', title: 'Dashboard', path: '/dashboard' },
      { id: 'profile', title: 'Profile', path: '/profile' },
    ]
  }
];

export function DashboardPage() {
  return (
    <SidebarLayout modules={modules}>
      {/* Existing dashboard content */}
    </SidebarLayout>
  );
}
```

### Step 5.3: Update Profile Pages

**Files**:

- `frontend/src/pages/profile/ProfileViewPage.tsx`
- `frontend/src/pages/profile/ProfileEditPage.tsx`

**Wrap with SidebarLayout** (same as dashboard)

### Step 5.4: Add Typography to Content

**For any content-heavy pages**, wrap content in `.prose` class:

```tsx
<div className="prose max-w-none">
  <h1>Your Title</h1>
  <p>Your content with beautiful typography...</p>
</div>
```

---

## Phase 6: Testing & Refinement

### Step 6.1: Visual Testing Checklist

Test each page:

- [ ] Login page (CenteredLayout)
- [ ] Dashboard (SidebarLayout)
- [ ] Profile view (SidebarLayout)
- [ ] Profile edit (SidebarLayout)

Test responsive behavior:

- [ ] Mobile menu (< 768px)
- [ ] Sidebar toggle (desktop)
- [ ] Typography scales correctly
- [ ] Dark mode works

### Step 6.2: Run Existing Tests

```bash
npm test -- --run
```

Ensure no regressions:

- [ ] All previous tests still pass
- [ ] No new console errors
- [ ] Layouts render correctly

### Step 6.3: Browser Testing

Test in:

- [ ] Chrome/Edge
- [ ] Firefox
- [ ] Safari (if available)
- [ ] Mobile viewport (DevTools)

---

## Phase 7: Cleanup & Documentation

### Step 7.1: Remove Unused Code

If replacing existing layouts:

- Remove old layout components
- Update all route imports
- Clean up unused CSS

### Step 7.2: Update Documentation

Update `frontend/README.md`:

```markdown
## Design System

This project uses a design system adapted from the Compass template:
- Typography: Inter Variable + Geist Mono
- Layouts: Centered (auth) + Sidebar (app)
- Components: Radix UI primitives
- Styling: TailwindCSS 4 + custom theme
```

### Step 7.3: Git Commit

```bash
git add .
git commit -m "feat(frontend): integrate compass-ts design system

- Add typography system with .prose styles
- Port CenteredLayout for auth pages
- Port SidebarLayout for app pages
- Replace Headless UI with Radix UI components
- Add Inter Variable and Geist Mono fonts
- Update all pages to use new layouts
- Maintain existing functionality and tests"
git push origin main
```

---

## Quick Reference: File Mapping

### Files to Create

| Template File | UnityPlan Target | Priority |
|---------------|------------------|----------|
| `app/typography.css` | `styles/typography.css` | HIGH |
| `components/centered-layout.tsx` | `components/layouts/CenteredLayout.tsx` | HIGH |
| `components/sidebar-layout.tsx` | `components/layouts/SidebarLayout.tsx` | HIGH |
| `components/navbar.tsx` | `components/layouts/Navbar.tsx` | HIGH |
| `components/icon-button.tsx` | `components/ui/IconButton.tsx` | MEDIUM |
| `components/dropdown.tsx` | `components/ui/Dropdown.tsx` | MEDIUM |
| `components/button.tsx` | `components/ui/Button.tsx` | LOW (have shadcn) |
| `components/input.tsx` | `components/ui/Input.tsx` | LOW (have shadcn) |
| `icons/*.tsx` | `components/icons/*.tsx` | MEDIUM |

### Package Adaptations

| Compass-TS Package | UnityPlan Replacement | Notes |
|--------------------|----------------------|-------|
| `@headlessui/react` (Dialog) | `@radix-ui/react-dialog` | Already have Radix |
| `@headlessui/react` (Menu) | `@radix-ui/react-dropdown-menu` | Need to install |
| `next/link` | `@tanstack/react-router` Link | Direct replacement |
| `next/navigation` usePathname | `@tanstack/react-router` useMatchRoute | Different API |
| `next/font` localFont | Manual @font-face or geist package | Static import |

---

## Troubleshooting

### Issue: Dialog not rendering

**Solution**: Check Radix Dialog is properly installed and imported:

```tsx
import * as Dialog from '@radix-ui/react-dialog';
```

### Issue: Fonts not loading

**Solution**: Verify font files are in `public/fonts/` and CSS paths are correct:

```css
@font-face {
  font-family: 'Inter Variable';
  src: url('/fonts/InterVariable.woff2') format('woff2');
}
```

### Issue: Routes not matching

**Solution**: Update TanStack Router match logic:

```tsx
const matchRoute = useMatchRoute();
const isActive = matchRoute({ to: '/dashboard' });
```

### Issue: Dark mode not working

**Solution**: Ensure dark mode class is on `<html>` element (check your theme setup)

---

## Estimated Timeline

| Phase | Time Estimate | Can Pause After? |
|-------|---------------|------------------|
| Phase 1: Preparation | 15 min | ✅ Yes |
| Phase 2: Typography | 30 min | ✅ Yes |
| Phase 3: Layouts | 2 hours | ✅ Yes |
| Phase 4: UI Components | 1.5 hours | ✅ Yes |
| Phase 5: Integration | 1.5 hours | ⚠️ Finish section |
| Phase 6: Testing | 1 hour | ✅ Yes |
| Phase 7: Cleanup | 30 min | ✅ Yes |
| **Total** | **~7.5 hours** | |

**Recommended approach**: Work in phases, commit after each phase, test incrementally.

---

## Success Criteria

✅ **Typography**: Content pages use `.prose` class with beautiful styling  
✅ **Layouts**: All pages use either CenteredLayout or SidebarLayout  
✅ **Navigation**: Sidebar navigation works with route highlighting  
✅ **Responsive**: Mobile menu and sidebar toggle work correctly  
✅ **Dark Mode**: Design system supports dark mode throughout  
✅ **Tests**: All existing tests pass without modification  
✅ **No Regressions**: All existing functionality still works  

---

## Notes

- **React 18 vs 19**: No compatibility issues expected. Template uses standard React patterns.
- **TailwindCSS 4**: Both projects use same major version - full compatibility.
- **Incremental Migration**: You can port layouts one page at a time.
- **Rollback**: Each phase is git-committed for easy rollback if needed.

---

**Next Step**: Commit current state, then begin Phase 1 preparation.
