# Frontend Stack Rationale

**Last Updated:** November 9, 2025  
**Decision Date:** November 9, 2025  
**Status:** Approved for MVP Phase 1

---

## 📋 Executive Summary

UnityPlan's frontend is built on a **production-grade, future-ready SPA stack** optimized for 2025:

**Core Stack:**

- **Vite 5.x** (build tool)
- **React 18.x** (UI framework) - *NOT React 19*
- **TailwindCSS 4.x + shadcn/ui 3.5** (styling)
- **TanStack Router v1.13x** (routing)
- **TanStack Query v5** (data layer)
- **Zustand** (auth/UI state only)
- **react-hook-form + zod** (forms)
- **TypeScript** (type safety)

**Key Decision:** React 18 chosen over React 19 for stable ecosystem during MVP phase, with incremental upgrade path for future Tauri migration.

---

## 🎯 Design Principles

### 1. **Separation of Concerns**

| Layer | Technology | Responsibility |
|-------|------------|----------------|
| **UI Components** | React 18 + shadcn/ui | Presentation, user interactions |
| **Data Fetching** | TanStack Query | Server state, caching, background refetching |
| **Local State** | Zustand | Auth tokens, UI state (sidebar, theme) |
| **Routing** | TanStack Router | Navigation, route guards, lazy loading |
| **Forms** | react-hook-form + zod | Validation, submission, error handling |
| **Styling** | TailwindCSS + CSS variables | Design system, theming |

**Philosophy:** Each tool does one thing exceptionally well. No overlap, no redundancy.

---

### 2. **Future-Proofing for Tauri Migration**

**Timeline:** ~1 year (Phase 3)  
**Target:** Cross-platform desktop and mobile applications

| Current (SPA) | Future (Tauri) | Migration Path |
|---------------|----------------|----------------|
| React 18 in browser | React 18 in Tauri webview | ✅ **Zero changes** - Vite officially supported |
| Browser routing (history API) | File-based routing | ✅ **Minimal change** - Switch to `createHashHistory()` |
| TailwindCSS | TailwindCSS | ✅ **Zero changes** - CSS-based, DOM-agnostic |
| shadcn/ui components | shadcn/ui components | ✅ **Zero changes** - No browser-specific APIs |
| localStorage (Zustand) | Tauri secure storage | ✅ **Easy swap** - Change storage adapter only |
| TanStack Query | TanStack Query | ✅ **Zero changes** - Runtime-agnostic |
| Axios HTTP client | Tauri HTTP client | ✅ **Adapter pattern** - Same interface, different implementation |

**Verdict:** Current stack requires minimal changes for Tauri migration. Strategic choice reduces future technical debt.

---

### 3. **Developer Experience**

| Metric | Target | Achieved By |
|--------|--------|-------------|
| **Dev Server Startup** | < 1 second | Vite's native ESM |
| **Hot Reload** | Instant | Vite HMR |
| **Type Safety** | 100% | TypeScript strict mode |
| **Bundle Size** | < 200KB gzipped | Code splitting, tree shaking |
| **Testing Speed** | < 5s for unit tests | Vitest (Vite-native) |
| **Learning Curve** | Low | shadcn (copy-paste), Zustand (minimal API) |

---

## 🔍 Technology Decisions

### React 18 vs React 19

#### **Why React 18 (NOT React 19)?**

| Factor | React 18 | React 19 | Winner |
|--------|----------|----------|--------|
| **Ecosystem Stability** | ✅ All libraries fully optimized | ⚠️ Some dependencies still catching up | **React 18** |
| **Server Components** | ❌ Not supported | ✅ Supported | N/A (not needed for SPA) |
| **Actions & Forms** | ❌ Manual handling | ✅ Built-in | N/A (react-hook-form sufficient) |
| **`use()` Hook** | ❌ Not available | ✅ Available | N/A (TanStack Query handles async) |
| **Production Readiness** | ✅ Battle-tested | ⚠️ Newer, fewer real-world deployments | **React 18** |
| **SSR Benefits** | ⚠️ Limited | ✅ Improved | N/A (pure client-side app) |

#### **The Verdict**

**React 19's biggest gains are for SSR, not SPAs.**

- **Server Components**: Not applicable to client-only apps
- **Actions**: `react-hook-form` + `zod` already provides superior form handling
- **`use()` hook**: TanStack Query handles async data fetching better
- **Bleeding-edge instability**: React 19 too new for MVP phase

**React 18 provides:**

- ✅ Fully stable ecosystem (all dependencies optimized)
- ✅ Fewer breaking edges during development
- ✅ Proven production deployments
- ✅ Incremental upgrade path to React 19 when needed (Tauri phase)

---

### TanStack Query vs Manual State Management

#### **Before TanStack Query (❌ Anti-pattern)**

```tsx
// Manual useEffect + useState (DON'T DO THIS)
function CourseList() {
  const [courses, setCourses] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  useEffect(() => {
    fetch('/api/courses')
      .then(res => res.json())
      .then(data => {
        setCourses(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err);
        setLoading(false);
      });
  }, []);
  
  // No caching, no refetching, no deduplication
  // Every component re-fetches data
  // Manual error handling, no retries
  // Background updates require custom logic
}
```

#### **After TanStack Query (✅ Best Practice)**

```tsx
// TanStack Query (DO THIS)
function useCourses() {
  return useQuery({
    queryKey: ['courses'],
    queryFn: fetchCourses,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

function CourseList() {
  const { data: courses, isLoading, error } = useCourses();
  
  // Automatic caching across components
  // Background refetching when stale
  // Request deduplication
  // Automatic retries on failure
  // DevTools for debugging
}
```

#### **Benefits**

| Feature | Manual State | TanStack Query |
|---------|-------------|----------------|
| **Caching** | ❌ Manual | ✅ Automatic |
| **Refetching** | ❌ Custom logic | ✅ Background, configurable |
| **Deduplication** | ❌ Multiple requests | ✅ Single request |
| **Error Handling** | ❌ Boilerplate | ✅ Built-in with retries |
| **Loading States** | ❌ Manual flags | ✅ Automatic |
| **DevTools** | ❌ None | ✅ React Query DevTools |
| **Optimistic Updates** | ❌ Complex | ✅ Simple API |
| **Pagination** | ❌ Custom | ✅ Built-in |
| **Infinite Scroll** | ❌ Complex | ✅ `useInfiniteQuery` |

**Verdict:** TanStack Query eliminates 80% of data fetching boilerplate and prevents common bugs.

---

### Zustand vs Redux/Context API

#### **State Management Philosophy**

**Rule:** Use the right tool for the right job.

| State Type | Tool | Reason |
|------------|------|--------|
| **Server Data** (courses, users, profiles) | TanStack Query | Designed for async data, caching, refetching |
| **Auth Tokens** (JWT, refresh token) | Zustand | Needs persistence, simple read/write |
| **UI State** (sidebar open, theme, modal state) | Zustand | Ephemeral, no server sync |
| **Form State** (input values, validation) | react-hook-form | Optimized for forms, minimal re-renders |
| **Component-Local State** (toggle, counter) | useState | No global access needed |

#### **Why Zustand (Not Redux/Context)?**

| Feature | Zustand | Redux | Context API |
|---------|---------|-------|-------------|
| **Boilerplate** | ✅ Minimal | ❌ Heavy | ⚠️ Medium |
| **Performance** | ✅ Selective subscription | ⚠️ Requires middleware | ❌ Re-renders all consumers |
| **DevTools** | ✅ Available | ✅ Built-in | ❌ None |
| **Learning Curve** | ✅ Minutes | ❌ Days | ⚠️ Medium |
| **Persistence** | ✅ Built-in middleware | ⚠️ Manual | ❌ Manual |
| **TypeScript** | ✅ Excellent | ⚠️ Verbose | ⚠️ Tricky |

**Example: Auth Store**

```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AuthState {
  token: string | null;
  userId: string | null;
  isAuthenticated: boolean;
  login: (token: string, userId: string) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      token: null,
      userId: null,
      isAuthenticated: false,
      login: (token, userId) => set({ token, userId, isAuthenticated: true }),
      logout: () => set({ token: null, userId: null, isAuthenticated: false }),
    }),
    { name: 'auth-storage' }
  )
);

// Usage (1 line)
const { isAuthenticated, login, logout } = useAuthStore();
```

**Verdict:** Zustand perfect for auth/UI state. Let TanStack Query handle data.

---

### shadcn/ui vs MUI/Chakra/Ant Design

#### **Why shadcn/ui?**

| Feature | shadcn/ui | MUI/Chakra/Ant Design |
|---------|-----------|----------------------|
| **Ownership** | ✅ Copy-paste into codebase | ❌ npm dependency |
| **Customization** | ✅ Full control (edit source) | ⚠️ Theme overrides |
| **Bundle Size** | ✅ Only what you use | ❌ Entire library |
| **Accessibility** | ✅ Radix UI primitives (WAI-ARIA) | ⚠️ Varies |
| **Tailwind Integration** | ✅ Native | ⚠️ Requires wrappers |
| **Dark Mode** | ✅ CSS variables | ⚠️ Theme system |
| **Version Lock-in** | ✅ None (you own code) | ❌ Breaking changes |

**Philosophy:** You copy components into `src/components/ui/`, then modify as needed. No dependency on npm package lifecycle.

**Example:**

```bash
# Install button component
npx shadcn-ui@latest add button

# Creates: src/components/ui/button.tsx
# You own this file, modify freely
```

**Verdict:** shadcn/ui gives maximum flexibility with zero lock-in.

---

## 🏗️ Architecture Patterns

### 1. **Route-Based Code Splitting**

```tsx
import { lazy } from 'react';
import { createFileRoute } from '@tanstack/react-router';

// Lazy load components
const Home = lazy(() => import('./pages/Home'));
const Dashboard = lazy(() => import('./pages/Dashboard'));
const CourseDetail = lazy(() => import('./pages/CourseDetail'));

// TanStack Router with lazy loading
export const Route = createFileRoute('/courses/$courseId')({
  component: CourseDetail, // Loaded only when route accessed
});
```

**Benefits:**

- Initial bundle < 200KB
- Fast first page load
- Subsequent routes load on-demand

---

### 2. **API Client Pattern**

```typescript
// src/lib/api-client.ts
import axios from 'axios';
import { useAuthStore } from '@/stores/authStore';

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
});

// Request interceptor: Add auth token
apiClient.interceptors.request.use((config) => {
  const { token } = useAuthStore.getState();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Response interceptor: Handle token refresh
apiClient.interceptors.response.use(
  (response) => response,
  async (error) => {
    if (error.response?.status === 401) {
      // Attempt token refresh
      const newToken = await refreshToken();
      if (newToken) {
        error.config.headers.Authorization = `Bearer ${newToken}`;
        return apiClient.request(error.config);
      }
      // Redirect to login
      useAuthStore.getState().logout();
    }
    return Promise.reject(error);
  }
);

export default apiClient;
```

---

### 3. **Query Hooks Pattern**

```typescript
// src/lib/queries/useCourses.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import apiClient from '@/lib/api-client';

// Fetch courses
export function useCourses() {
  return useQuery({
    queryKey: ['courses'],
    queryFn: () => apiClient.get('/api/v1/courses').then(res => res.data),
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// Enroll in course
export function useEnrollCourse() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (courseId: string) => 
      apiClient.post(`/api/v1/courses/${courseId}/enroll`),
    onSuccess: () => {
      // Invalidate courses list to refetch
      queryClient.invalidateQueries({ queryKey: ['courses'] });
    },
  });
}
```

---

### 4. **Form Pattern with Validation**

```tsx
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

// Zod schema (shareable with backend if using TypeScript/tRPC)
const profileSchema = z.object({
  username: z.string().min(3).max(20),
  email: z.string().email(),
  bio: z.string().max(500).optional(),
});

type ProfileForm = z.infer<typeof profileSchema>;

export function ProfileForm() {
  const { register, handleSubmit, formState: { errors } } = useForm<ProfileForm>({
    resolver: zodResolver(profileSchema),
  });
  
  const updateProfile = useUpdateProfile();
  
  const onSubmit = (data: ProfileForm) => {
    updateProfile.mutate(data);
  };
  
  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div>
        <Input {...register('username')} placeholder="Username" />
        {errors.username && <p className="text-sm text-red-500">{errors.username.message}</p>}
      </div>
      <div>
        <Input {...register('email')} placeholder="Email" />
        {errors.email && <p className="text-sm text-red-500">{errors.email.message}</p>}
      </div>
      <Button type="submit" disabled={updateProfile.isPending}>
        {updateProfile.isPending ? 'Saving...' : 'Save Profile'}
      </Button>
    </form>
  );
}
```

---

## 🧪 Testing Strategy

### Unit Tests (Vitest)

```typescript
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { Button } from '@/components/ui/button';

describe('Button', () => {
  it('renders children', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });
  
  it('handles click events', async () => {
    const handleClick = vi.fn();
    render(<Button onClick={handleClick}>Click me</Button>);
    
    await userEvent.click(screen.getByText('Click me'));
    expect(handleClick).toHaveBeenCalledOnce();
  });
});
```

### Integration Tests (TanStack Query)

```typescript
import { renderHook, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useCourses } from '@/lib/queries/useCourses';

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return ({ children }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
};

describe('useCourses', () => {
  it('fetches courses successfully', async () => {
    const { result } = renderHook(() => useCourses(), { wrapper: createWrapper() });
    
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data).toHaveLength(3);
  });
});
```

---

## 📦 Build Optimization

### Vite Configuration

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { visualizer } from 'rollup-plugin-visualizer';

export default defineConfig({
  plugins: [
    react(),
    visualizer({ open: true, gzipSize: true }),
  ],
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          'react-vendor': ['react', 'react-dom'],
          'router': ['@tanstack/react-router'],
          'query': ['@tanstack/react-query'],
          'ui': ['@radix-ui/react-dialog', '@radix-ui/react-dropdown-menu'],
        },
      },
    },
  },
});
```

**Target Bundle Sizes:**

- Initial bundle: < 200KB gzipped
- Route chunks: < 50KB gzipped each
- Vendor chunks: Cached separately, loaded once

---

## 🚀 Migration Path to React 19

**When:** After MVP (Phase 2 or Phase 3), or when Tauri migration happens

**Steps:**

1. **Upgrade React** (1-2 days)

   ```bash
   npm install react@19 react-dom@19
   ```

2. **Update Dependencies** (1 week)
   - Wait for TanStack Router React 19 support
   - Wait for shadcn/ui React 19 compatibility
   - Test all third-party libraries

3. **Adopt React 19 Features** (optional, 2-4 weeks)
   - Migrate forms to React 19 Actions (if beneficial)
   - Explore Server Components for Tauri SSR (if applicable)
   - Use `use()` hook where it simplifies code

**Risk:** Low - React 18 → 19 migration well-documented by React team

---

## 💡 Optional Enhancements

### When Ready (Post-MVP)

| Enhancement | Purpose | When |
|-------------|---------|------|
| **tRPC** | Type-safe RPC | If backend moves to TypeScript |
| **Jotai/Recoil** | Fine-grained reactivity | If Zustand insufficient |
| **Vite PWA Plugin** | Offline support | Before Tauri migration |
| **Framer Motion** | Animations | Polish phase |
| **Playwright** | E2E tests | After core features stable |

---

## ✅ Conclusion

### Stack Summary

> **Vite 5 + React 18 + TailwindCSS 4 + shadcn/ui 3.5 + TanStack Router + TanStack Query + Zustand + react-hook-form + zod**

### Why This Stack?

✅ **Production-grade** - Battle-tested, mature ecosystem  
✅ **Future-ready** - Minimal changes for Tauri migration  
✅ **Developer-friendly** - Fast dev server, minimal boilerplate  
✅ **Scalable** - Clear separation of concerns, code splitting  
✅ **Type-safe** - TypeScript strict mode, zod schemas  
✅ **Performant** - < 200KB bundles, automatic caching  

### Strategic Decision

**React 18 over React 19:**

- React 19's benefits (Server Components, Actions) don't apply to pure SPAs
- Stable ecosystem reduces risk during MVP phase
- Incremental upgrade path when needed
- Zero technical debt for current requirements

---

**Approved By:** Architecture Team  
**Next Review:** After MVP completion (Phase 1 end)  
**Related Documents:**

- [Technology Stack](tech-stack.md)
- [Phase 1 Checklist](../status/current/phase-1-checklist.md)
- [Versioning Strategy](../guides/development/versioning-strategy.md)
