# Frontend Development Guide

React 18 single-page application (SPA) with Vite, TanStack Router/Query, Zustand, and shadcn/ui.

**Status:** Stage 5 in progress (Authentication & Profile pages)

---

## ⚠️ CRITICAL VERSION COMPATIBILITY

**TailwindCSS v4 & shadcn/ui Version Breaking Changes**

Our frontend uses **TailwindCSS v4.1.17** and the latest **shadcn/ui**, which have **breaking changes** from previous versions:

### TailwindCSS v4 Changes (NOT Backward Compatible)

- ❌ **NO MORE** `tailwind.config.js` file
- ✅ **CSS-based configuration** using `@import "tailwindcss"` and `@theme {}` blocks
- ✅ **OKLCH color format** (not hex or RGB) - example: `oklch(0.985 0 0)`
- ✅ **Vite plugin** required: `@tailwindcss/vite` (not `@tailwindcss/postcss@3`)
- ✅ **CSS variable theming** via `:root` and `.dark` pseudo-classes

### shadcn/ui Latest Version Changes

- ✅ **CSS variables approach** (no utility classes fallback in older docs)
- ✅ **OKLCH colors only** in theming system
- ✅ **@theme inline** directive for exposing CSS variables to Tailwind
- ✅ **components.json** configuration file required

### Official Documentation

- **TailwindCSS v4:** <https://tailwindcss.com/docs> (v4 docs)
- **shadcn/ui Vite Setup:** <https://ui.shadcn.com/docs/installation/vite>
- **shadcn/ui Theming:** <https://ui.shadcn.com/docs/theming>

### DO NOT Use Old Documentation

- ❌ TailwindCSS v3 documentation (outdated config format)
- ❌ shadcn/ui examples using hex colors or utility classes
- ❌ Stack Overflow answers from before 2024 (likely v3)

**Always verify against official v4 documentation when troubleshooting!**

**📖 For complete migration guide, see:** [TailwindCSS v4 Migration Guide](./TAILWIND-V4-MIGRATION.md)

---

## 📚 Guides in This Section

### [TailwindCSS v4 Migration Guide](./TAILWIND-V4-MIGRATION.md) 🔥 READ FIRST FOR STYLING

**CRITICAL:** Complete guide to TailwindCSS v4 breaking changes and OKLCH color format.

**Contents:**

- Breaking changes summary (v3 vs v4 comparison table)
- OKLCH color format (syntax, examples, conversion tools)
- Configuration files (vite.config.ts, postcss.config.js, index.css)
- shadcn/ui integration (components.json, theming)
- Common migration issues (with solutions)
- Official resources and tools
- Verification checklist

**Use when:** Working with TailwindCSS, shadcn/ui, theming, or encountering styling issues. Read this before asking "why doesn't my Tailwind config work?"

---

### [Development Guide](./development-guide.md) ⭐ START HERE

Complete guide for setting up and building the frontend application.

```

**Contents:**

- Quick start (prerequisites, commands)
- Project setup (Vite, dependencies, shadcn/ui initialization)
- Development workflow (daily dev, commands, code quality)
- Project structure (folders, files, organization)
- Configuration files (Vite, TailwindCSS, TypeScript, Vitest)
- Development practices (code organization, component patterns, state rules)
- Build & deployment (optimization, performance targets)
- Troubleshooting (common issues)

**Use when:** Starting the frontend project, setting up development environment, or understanding the project structure.

---

### [Testing Guide](./testing-guide.md)

Comprehensive testing strategies for all frontend code.

**Contents:**

- Testing philosophy (testing pyramid, principles)
- Testing stack (Vitest, Testing Library, MSW, Playwright)
- Unit testing (pure functions, custom hooks)
- Component testing (basic tests, user interactions, providers)
- Integration testing (API mocking with MSW)
- E2E testing (Playwright user flows)
- Best practices (test behavior not implementation, accessible queries)
- Common patterns (async data, error states, form validation)
- Mocking (API, Zustand, TanStack Query)
- Coverage (targets, exclusions)
- CI/CD integration (GitHub Actions)

**Use when:** Writing tests for components, hooks, or user flows. Achieving 80%+ code coverage.

---

### [Component Patterns](./component-patterns.md)

Reusable component patterns and best practices.

**Contents:**

- Component types (presentation, container, compound)
- Composition patterns (render props, custom hooks, HOCs)
- Form components (react-hook-form + zod, shadcn forms, file upload)
- Data display (tables, infinite scroll)
- Layout components (responsive grid, sidebar)
- Reusable patterns (loading states, error boundaries, empty states)
- Performance (memoization, code splitting)
- Accessibility (semantic HTML, ARIA, keyboard navigation)

**Use when:** Building new UI components or understanding existing component patterns.

---

### [State Management](./state-management.md)

Data fetching and state management with TanStack Query and Zustand.

**Contents:**

- State management philosophy (separation of concerns: server vs client)
- TanStack Query setup (QueryClient, providers, devtools)
- Query hooks (single queries, infinite queries, dependent queries, parallel queries)
- Mutation patterns (optimistic updates, rollbacks, cache invalidation)
- Zustand setup (auth store, UI store, persistence)
- Auth state management (login flow, protected routes, token refresh)
- UI state management (theme, sidebar, preferences)
- Advanced patterns (cache invalidation, prefetching, polling)
- Performance optimization (selectors, store splitting)
- Common pitfalls (what NOT to do)

**Use when:** Implementing data fetching, managing state, or understanding the separation of concerns.

---

## 🚀 Quick Start

### Prerequisites

- Node.js 20+
- npm 10+
- Backend services running (auth-service on :8080, user-service on :8081)

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

**Port Info:** Frontend uses port 5173 (Vite default) to avoid conflicts with infrastructure:

- Port 3000: Forgejo (Git server)
- Port 3001: Grafana (monitoring)

### Development Commands

```bash
# Start dev server with HMR
npm run dev

# Run tests
npm run test

# Run tests with UI
npm run test:ui

# Run tests with coverage
npm run test:coverage

# Lint code
npm run lint

# Build for production
npm run build

# Preview production build
npm run preview
```

---

## 🏗️ Architecture Overview

### Technology Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Build Tool** | Vite 5.x | Fast dev server, native ESM, HMR |
| **UI Library** | React 18.3 | Component-based UI (stable ecosystem) |
| **Routing** | TanStack Router 1.134 | Type-safe client-side routing |
| **Data Fetching** | TanStack Query v5 | Server state management, caching |
| **State Management** | Zustand | Auth tokens + UI state only |
| **Styling** | TailwindCSS 4.1 | Utility-first CSS framework |
| **Components** | shadcn/ui 3.5 | Accessible, customizable components |
| **Forms** | react-hook-form + zod | Form handling and validation |
| **HTTP Client** | Axios | HTTP requests with interceptors |
| **Testing** | Vitest + Testing Library | Unit and component testing |
| **E2E Testing** | Playwright | End-to-end user flow testing |

### Design Principles

**1. Separation of Concerns**

- **TanStack Query** handles ALL server data (fetching, caching, refetching)
- **Zustand** handles ONLY auth tokens + UI state (theme, sidebar)
- **Never** store server data in Zustand

**2. Automatic Token Refresh**

- Axios interceptor catches 401 errors
- Automatically refreshes access token
- Retries failed request with new token
- Redirects to login if refresh fails

**3. Type Safety**

- TypeScript strict mode
- Zod schemas for validation
- TanStack Router type-safe routes
- API response types

**4. Performance**

- Code splitting with React.lazy()
- Route-based chunking
- Optimized bundle size (<200KB gzipped)
- Automatic tree-shaking

---

## 📁 Project Structure

```
frontend/
├── src/
│   ├── api/              # API client functions
│   │   ├── auth.ts       # Authentication API (login, register, logout)
│   │   └── users.ts      # User API (profile, avatar, privacy, connections)
│   │
│   ├── components/       # Reusable components
│   │   ├── ui/          # shadcn/ui components
│   │   ├── Avatar.tsx
│   │   ├── UserCard.tsx
│   │   └── ProtectedRoute.tsx
│   │
│   ├── lib/
│   │   ├── api-client.ts # Axios instance (with interceptors)
│   │   ├── queries/      # TanStack Query hooks
│   │   │   └── useUser.ts
│   │   └── utils.ts
│   │
│   ├── pages/
│   │   ├── auth/        # Authentication pages
│   │   │   ├── LoginPage.tsx
│   │   │   └── RegisterPage.tsx
│   │   └── profile/     # Profile pages
│   │       ├── ProfileViewPage.tsx
│   │       └── ProfileEditPage.tsx
│   │
│   ├── stores/          # Zustand stores
│   │   ├── authStore.ts # Auth tokens (persist to localStorage)
│   │   └── uiStore.ts   # Theme, sidebar state
│   │
│   ├── types/           # TypeScript type definitions
│   │   ├── auth.ts
│   │   └── user.ts
│   │
│   ├── App.tsx          # Root component
│   ├── main.tsx         # Entry point
│   └── index.css        # Global styles + theme variables
│
├── tests/               # Test files
│   ├── unit/
│   ├── integration/
│   └── e2e/
│
├── public/              # Static assets
├── package.json
├── vite.config.ts       # Vite configuration
├── tailwind.config.js   # TailwindCSS configuration
├── tsconfig.json        # TypeScript configuration
└── vitest.config.ts     # Vitest configuration
```

---

## 🎨 Key Features

### Implemented (Stage 5)

- ✅ Vite project with React 18 + TypeScript
- ✅ TailwindCSS with dark/light/system theming
- ✅ shadcn/ui component library
- ✅ Zustand auth store with localStorage persistence
- ✅ Axios instance with automatic token refresh
- ✅ TanStack Query hooks for all API operations
- ✅ Login page with form validation
- ✅ Registration page with territory selection
- ✅ Profile view page with tabs
- ✅ Profile edit page with avatar upload
- ✅ Follow/unfollow functionality

### Planned (Future Stages)

- ⏳ Territory management UI
- ⏳ Badge display and progress tracking
- ⏳ Course browsing and enrollment
- ⏳ Forum discussions
- ⏳ Matrix protocol integration
- ⏳ PWA support (offline mode)
- ⏳ Tauri desktop/mobile apps

---

## 🔧 Common Tasks

### Add New Page

1. Create page component in `src/pages/`
2. Define route in router configuration
3. Add navigation link

```tsx
// src/pages/MyPage.tsx
export function MyPage() {
  return <div>My Page</div>;
}

// Add to router
```

### Add New API Endpoint

1. Add function to appropriate API file (`src/api/`)
2. Create TanStack Query hook in `src/lib/queries/`
3. Use hook in component

```typescript
// src/api/users.ts
export async function getSomething(id: string) {
  const response = await apiClient.get(`/api/users/${id}/something`);
  return response.data;
}

// src/lib/queries/useUser.ts
export function useSomething(id: string) {
  return useQuery({
    queryKey: ['something', id],
    queryFn: () => userApi.getSomething(id),
  });
}
```

### Add New shadcn Component

```bash
npx shadcn@latest add dialog
npx shadcn@latest add dropdown-menu
```

### Update Theme Colors

Edit `src/index.css` CSS variables:

```css
:root {
  --primary: 222.2 47.4% 11.2%;
  --primary-foreground: 210 40% 98%;
  /* ... */
}
```

---

## 🧪 Testing

### Unit Tests

Test individual functions and hooks:

```bash
npm run test -- Avatar.test.tsx
```

### Component Tests

Test component rendering and interactions:

```tsx
import { render, screen } from '@testing-library/react';
import { Avatar } from './Avatar';

test('renders avatar with fallback', () => {
  render(<Avatar name="John Doe" />);
  expect(screen.getByText('JD')).toBeInTheDocument();
});
```

### E2E Tests

Test complete user flows:

```bash
npx playwright test
```

---

## 🐛 Troubleshooting

### Module Not Found

```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Vite Dev Server Won't Start

```bash
# Check port 5173 is free (Vite default)
lsof -i :5173
# Kill if needed
kill -9 <PID>
# Or let Vite auto-select next available port
```

**Remember:** Ports 3000-3001 are used by infrastructure (Forgejo, Grafana). Frontend uses 5173.

### API Calls Fail with CORS

1. Check Vite proxy config in `vite.config.ts`
2. Verify backend services are running
3. Check backend CORS settings

### Token Refresh Infinite Loop

Check Axios interceptor has retry prevention flag.

### shadcn Components Not Working

```bash
# Reinitialize shadcn
npx shadcn@latest init
```

---

## 📚 Additional Resources

### Internal Documentation

- [Frontend Stack Rationale](../../architecture/frontend-stack-rationale.md) - Why React 18, TanStack Query, etc.
- [Tech Stack Reference](../../project/tech-stack.md) - Complete technology listing
- [Versioning Strategy](../shared/versioning-strategy.md) - SemVer 2.0.0 guidelines

### External Resources

- [React Documentation](https://react.dev)
- [Vite Documentation](https://vitejs.dev)
- [TanStack Router](https://tanstack.com/router)
- [TanStack Query](https://tanstack.com/query)
- [Zustand](https://zustand-demo.pmnd.rs)
- [shadcn/ui](https://ui.shadcn.com)
- [TailwindCSS](https://tailwindcss.com)
- [Vitest](https://vitest.dev)

### Learning Resources

- [Practical React Query](https://tkdodo.eu/blog/practical-react-query) - Essential reading
- [React Hook Form with Zod](https://react-hook-form.com/get-started#SchemaValidation)
- [TailwindCSS Best Practices](https://tailwindcss.com/docs/reusing-styles)

---

## 🎯 Next Steps

1. **Start with Development Guide:** Complete project setup
2. **Build Auth Pages:** Login and registration
3. **Add Profile Pages:** View and edit user profiles
4. **Implement Testing:** Write unit and E2E tests
5. **Deploy:** Build and deploy to production

---

**For implementation details, see the Stage 5 guides in `temp/` directory.**
