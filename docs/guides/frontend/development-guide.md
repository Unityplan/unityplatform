# Frontend Development Guide

Complete guide for building the UnityPlan React frontend application.

**Tech Stack:** React 18, Vite, TanStack Router/Query, Zustand, shadcn/ui  
**Status:** Stage 5 in progress  
**Last Updated:** November 9, 2025

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

# Application runs on http://localhost:3000
```

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

### Step 3: Install UI Dependencies

```bash
# TailwindCSS
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
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
- Base color: Slate
- CSS variables: Yes
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
# 1. Start backend services (if not running)
cd ..
docker compose -f docker-compose.dev.yml up -d

# 2. Start frontend dev server
cd frontend
npm run dev

# 3. Open browser to http://localhost:3000

# 4. Make changes - HMR (Hot Module Replacement) updates automatically
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
├── vite.config.ts        # Vite configuration
├── tailwind.config.js    # TailwindCSS configuration
├── tsconfig.json         # TypeScript configuration
└── vitest.config.ts      # Vitest test configuration
```

---

## Configuration Files

### vite.config.ts

```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
});
```

### tailwind.config.js

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ['class'],
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        border: 'hsl(var(--border))',
        input: 'hsl(var(--input))',
        ring: 'hsl(var(--ring))',
        background: 'hsl(var(--background))',
        foreground: 'hsl(var(--foreground))',
        primary: {
          DEFAULT: 'hsl(var(--primary))',
          foreground: 'hsl(var(--primary-foreground))',
        },
        // ... more theme colors
      },
      borderRadius: {
        lg: 'var(--radius)',
        md: 'calc(var(--radius) - 2px)',
        sm: 'calc(var(--radius) - 4px)',
      },
    },
  },
  plugins: [require('tailwindcss-animate')],
};
```

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
VITE_AUTH_SERVICE_URL=https://auth.unityplan.com
VITE_USER_SERVICE_URL=https://api.unityplan.com
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
# Check port 3000
lsof -i :3000

# Kill process if needed
kill -9 <PID>
```

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
