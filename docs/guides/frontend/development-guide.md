# Frontend Development Guide

Complete guide for building the Unity Platform React frontend application.

**Tech Stack:** React 18, Vite, TanStack Router/Query, Zustand, shadcn/ui  
**Status:** Stage 5 in progress  
**Last Updated:** November 9, 2025

---

## ⚠️ CRITICAL: Version Compatibility Warning

### TailwindCSS v4 & shadcn/ui Breaking Changes

**Our versions are NOT backward compatible with previous versions!**

We use:

- **TailwindCSS v4.1.17** (breaking changes from v3)
- **shadcn/ui latest** (OKLCH colors only, new theming system)

### Key Differences from Older Documentation

| Aspect | ❌ OLD (v3) | ✅ NEW (v4 - WE USE THIS) |
|--------|-------------|---------------------------|
| **Config File** | `tailwind.config.js` | **NO CONFIG FILE** - CSS-based |
| **CSS Import** | `@tailwind base; @tailwind components;` | `@import "tailwindcss";` |
| **Theming** | `@theme { }` in config | `@theme { }` in CSS + `:root` variables |
| **Colors** | Hex/RGB (`#646cff`) | **OKLCH** (`oklch(0.985 0 0)`) |
| **Vite Plugin** | `@tailwindcss/postcss@3` | `@tailwindcss/vite` |
| **Variable Exposure** | Manual | `@theme inline { }` directive |

### Official Documentation (ALWAYS CHECK THESE)

- **TailwindCSS v4:** <https://tailwindcss.com/docs>
- **shadcn/ui Vite:** <https://ui.shadcn.com/docs/installation/vite>
- **shadcn/ui Theming:** <https://ui.shadcn.com/docs/theming>

### DO NOT Use

- ❌ TailwindCSS v3 documentation
- ❌ Old Stack Overflow answers (pre-2024)
- ❌ ChatGPT/AI suggestions without verification
- ❌ shadcn examples with hex colors

**If something doesn't work, verify against official v4 docs first!**

---

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Project Setup](#project-setup)
- [Development Workflow](#development-workflow)
- [Project Structure](#project-structure)
- [Configuration Files](#configuration-files)
- [Development Practices](#development-practices)
- [Build & Deployment](#build--deployment)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

### Prerequisites

- Node.js 20+ and npm 10+
- Backend services running (auth-service on :8080, user-service on :8081)
- PostgreSQL, NATS, Redis running via Docker

### Initial Setup

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev

# Application runs on http://localhost:5173
```

**Note:** Port 5173 is Vite's default. We use this to avoid conflicts with infrastructure services (Forgejo on 3000, Grafana on 3001).

---

## Project Setup

### Step 1: Create Vite Project

```bash
# Create new Vite project with React + TypeScript
npm create vite@latest frontend -- --template react-ts

cd frontend
npm install
```

### Step 2: Install Core Dependencies

```bash
# Routing and data fetching
npm install @tanstack/react-router @tanstack/react-query

# State management
npm install zustand

# HTTP client
npm install axios

# Forms and validation
npm install react-hook-form @hookform/resolvers zod
```

### Step 3: Install UI Dependencies (TailwindCSS v4)

```bash
# TailwindCSS v4 with Vite plugin
npm install -D tailwindcss @tailwindcss/vite

# IMPORTANT: NO `npx tailwindcss init` needed for v4!
# Configuration is done in CSS files, not JS config
```

### Step 4: Install Testing Dependencies

```bash
# Vitest and Testing Library
npm install -D vitest @vitest/ui jsdom
npm install -D @testing-library/react @testing-library/jest-dom @testing-library/user-event
```

### Step 5: Initialize shadcn/ui

```bash
npx shadcn@latest init
```

Follow prompts:

- Style: Default
- Base color: Neutral (uses OKLCH colors)
- CSS variables: Yes (required for v4)
- React Server Components: No
- TypeScript: Yes
- Path aliases: @/*→ ./src/*

### Step 6: Install Base Components

```bash
npx shadcn@latest add button input card form label select checkbox textarea avatar tabs toast
```

---

## Development Workflow

### Daily Development

```bash
# 1. Start dev server
npm run dev

# 2. Server starts on http://localhost:5173

# 3. Open browser to http://localhost:5173
```

### Development Commands

```bash
# Development server with HMR
npm run dev

# Type checking
npm run type-check

# Linting
npm run lint

# Fix linting issues
npm run lint:fix

# Run tests
npm run test

# Run tests with UI
npm run test:ui

# Run tests with coverage
npm run test:coverage

# Build for production
npm run build

# Preview production build locally
npm run preview
```

### Code Quality Workflow

```bash
# Before committing
npm run lint        # Check for linting errors
npm run type-check  # Check TypeScript errors
npm run test        # Run all tests

# Fix issues
npm run lint:fix    # Auto-fix linting issues
```

---

## Project Structure

```
frontend/
├── src/
│   ├── api/              # API client functions
│   │   ├── auth.ts       # Authentication endpoints
│   │   └── users.ts      # User management endpoints
│   │
│   ├── components/       # Reusable React components
│   │   ├── ui/          # shadcn/ui components
│   │   │   ├── button.tsx
│   │   │   ├── input.tsx
│   │   │   ├── card.tsx
│   │   │   └── ...
│   │   ├── Avatar.tsx
│   │   ├── UserCard.tsx
│   │   ├── ProfileHeader.tsx
│   │   └── ProtectedRoute.tsx
│   │
│   ├── lib/
│   │   ├── api-client.ts # Axios instance with interceptors
│   │   ├── queries/      # TanStack Query hooks
│   │   │   ├── useAuth.ts
│   │   │   └── useUser.ts
│   │   └── utils.ts      # Utility functions
│   │
│   ├── pages/            # Page components
│   │   ├── auth/
│   │   │   ├── LoginPage.tsx
│   │   │   ├── RegisterPage.tsx
│   │   │   └── ResetPasswordPage.tsx
│   │   ├── profile/
│   │   │   ├── ProfileViewPage.tsx
│   │   │   └── ProfileEditPage.tsx
│   │   └── DashboardPage.tsx
│   │
│   ├── stores/           # Zustand stores
│   │   ├── authStore.ts  # Auth tokens (persisted)
│   │   └── uiStore.ts    # UI state (theme, sidebar)
│   │
│   ├── types/            # TypeScript type definitions
│   │   ├── auth.ts
│   │   └── user.ts
│   │
│   ├── App.tsx           # Root component
│   ├── main.tsx          # Entry point
│   └── index.css         # Global styles + theme variables
│
├── tests/                # Test files
│   ├── unit/
│   ├── integration/
│   └── e2e/
│
├── public/               # Static assets
│
├── .env.development      # Development environment variables
├── .env.production       # Production environment variables
├── package.json          # Dependencies and scripts
├── vite.config.ts        # Vite configuration (includes TailwindCSS v4 plugin)
├── postcss.config.js     # PostCSS configuration (for TailwindCSS v4)
├── tsconfig.json         # TypeScript configuration
└── vitest.config.ts      # Vitest test configuration
```

---

## Configuration Files

### vite.config.ts (TailwindCSS v4)

```typescript
import path from 'path';
import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [react(), tailwindcss()], // TailwindCSS v4 Vite plugin
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 5173, // Vite default - avoids conflicts with Forgejo (3000) and Grafana (3001)
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
});
```

### postcss.config.js (TailwindCSS v4)

```javascript
// PostCSS configuration for TailwindCSS v4
export default {
  plugins: {
    '@tailwindcss/postcss': {},
  },
};
```

### src/index.css (TailwindCSS v4 Theming)

```css
/* Import TailwindCSS v4 - replaces old @tailwind directives */
@import "tailwindcss";

/* Define theme CSS variables using OKLCH color format */
:root {
  --radius: 0.625rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --primary: oklch(0.205 0 0);
  --primary-foreground: oklch(0.985 0 0);
  --secondary: oklch(0.97 0 0);
  --secondary-foreground: oklch(0.205 0 0);
  --muted: oklch(0.97 0 0);
  --muted-foreground: oklch(0.556 0 0);
  --border: oklch(0.922 0 0);
  --input: oklch(0.922 0 0);
  --ring: oklch(0.708 0 0);
  /* Add more variables as needed */
}

/* Dark mode theme */
.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --primary: oklch(0.922 0 0);
  --primary-foreground: oklch(0.205 0 0);
  --secondary: oklch(0.269 0 0);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.269 0 0);
  --muted-foreground: oklch(0.708 0 0);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.556 0 0);
  /* Add more dark mode variables as needed */
}

/* Expose CSS variables to TailwindCSS using @theme inline */
@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-ring: var(--ring);
  /* Add more color mappings as needed */
}
```

**IMPORTANT:** TailwindCSS v4 does NOT use `tailwind.config.js`! All configuration is in CSS files using `@theme` blocks and CSS variables in OKLCH format.

### tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,

    /* Path aliases */
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

### vitest.config.ts

```typescript
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
    css: true,
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
```

### package.json Scripts

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "lint:fix": "eslint . --ext ts,tsx --fix",
    "type-check": "tsc --noEmit",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage"
  }
}
```

### Environment Variables

**.env.development:**

```bash
VITE_AUTH_SERVICE_URL=http://localhost:8080
VITE_USER_SERVICE_URL=http://localhost:8081
VITE_API_TIMEOUT=30000
```

**.env.production:**

```bash
VITE_AUTH_SERVICE_URL=https://auth.unityplatform.com
VITE_USER_SERVICE_URL=https://api.unityplatform.com
VITE_API_TIMEOUT=30000
```

---

## Development Practices

### Code Organization

**Follow Single Responsibility Principle:**

- One component per file
- Keep components focused and small (<200 lines)
- Extract reusable logic into custom hooks
- Separate business logic from presentation

**Example:**

```tsx
// ❌ Bad: Everything in one component
function UserProfile() {
  // 500 lines of mixed logic and UI
}

// ✅ Good: Separated concerns
function UserProfile() {
  const { user } = useUser(userId);
  return <ProfileView user={user} />;
}
```

### Component Patterns

**Use composition over prop drilling:**

```tsx
// ✅ Good: Compound components
<Card>
  <CardHeader>
    <CardTitle>Profile</CardTitle>
  </CardHeader>
  <CardContent>
    {/* content */}
  </CardContent>
</Card>
```

**Keep components pure when possible:**

```tsx
// ✅ Pure component - easier to test
function UserCard({ user }: { user: User }) {
  return (
    <div>
      <h3>{user.name}</h3>
      <p>{user.email}</p>
    </div>
  );
}
```

### State Management Rules

**TanStack Query for server data:**

```tsx
// ✅ All server data through TanStack Query
function UserProfile() {
  const { data: user } = useUser(userId);
  const updateMutation = useUpdateUser();
  
  // Never store server data in Zustand!
}
```

**Zustand for auth and UI state only:**

```tsx
// ✅ Auth tokens and UI state
const authStore = useAuthStore();
const uiStore = useUIStore();

// ❌ Never store server data in Zustand
```

### Form Handling

**Always use react-hook-form + zod:**

```tsx
const schema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
});

type FormData = z.infer<typeof schema>;

function LoginForm() {
  const { register, handleSubmit, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
  });
  
  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <Input {...register('email')} />
      {errors.email && <span>{errors.email.message}</span>}
    </form>
  );
}
```

### Error Handling

**Handle errors gracefully:**

```tsx
function UserProfile() {
  const { data, isLoading, error } = useUser(userId);
  
  if (isLoading) return <Spinner />;
  if (error) return <ErrorMessage error={error} />;
  if (!data) return <NotFound />;
  
  return <ProfileView user={data} />;
}
```

### TypeScript Best Practices

**Define types explicitly:**

```tsx
// ✅ Explicit types
interface User {
  id: string;
  email: string;
  username: string;
}

function UserCard({ user }: { user: User }) {
  // ...
}

// ❌ Avoid 'any'
function UserCard({ user }: { user: any }) {
  // ...
}
```

---

## Build & Deployment

### Production Build

```bash
# Type check
npm run type-check

# Run tests
npm run test

# Build
npm run build

# Output in dist/ directory
```

### Build Optimization

**Code splitting:**

```tsx
// Lazy load routes
const ProfilePage = lazy(() => import('./pages/ProfilePage'));

// Use in router
<Route path="/profile" element={<ProfilePage />} />
```

**Bundle analysis:**

```bash
npm install -D rollup-plugin-visualizer

# Add to vite.config.ts
import { visualizer } from 'rollup-plugin-visualizer';

plugins: [
  react(),
  visualizer({ open: true }),
]
```

### Performance Targets

- **Initial bundle:** <200KB gzipped
- **First Contentful Paint:** <1.5s
- **Time to Interactive:** <3s
- **Lighthouse score:** >90

---

## Troubleshooting

### Common Issues

**Module not found:**

```bash
# Clear cache
rm -rf node_modules package-lock.json
npm install
```

**Vite dev server won't start:**

```bash
# Check port 5173 (Vite default)
lsof -i :5173

# Kill process if needed
kill -9 <PID>

# Or let Vite auto-select next available port
```

**Note:** Infrastructure services reserve ports 3000 (Forgejo), 3001 (Grafana), 8080 (auth-service), 8081 (user-service). Frontend uses 5173 to avoid conflicts.

**API calls fail with CORS:**

1. Check Vite proxy config
2. Verify backend CORS settings
3. Check backend is running

**Token refresh loop:**

- Check Axios interceptor has retry prevention
- Verify refresh endpoint works
- Check token expiration times

**shadcn components not styled:**

```bash
# Reinitialize
npx shadcn@latest init
```

**TypeScript errors:**

```bash
# Regenerate types
npm run type-check
```

---

## Additional Resources

- [React Documentation](https://react.dev)
- [Vite Documentation](https://vitejs.dev)
- [TanStack Query](https://tanstack.com/query)
- [TanStack Router](https://tanstack.com/router)
- [Zustand](https://zustand-demo.pmnd.rs)
- [shadcn/ui](https://ui.shadcn.com)
- [TailwindCSS](https://tailwindcss.com)

**Internal Guides:**

- [Testing Guide](./testing-guide.md)
- [Component Patterns](./component-patterns.md)
- [State Management](./state-management.md)
- [Frontend Stack Rationale](../../architecture/frontend-stack-rationale.md)

---

**Next:** Follow the Stage 5 implementation guide in `temp/stage-5-implementation-guide.md` for step-by-step setup.
