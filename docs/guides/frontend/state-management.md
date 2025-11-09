# State Management Guide

Complete guide to state management in UnityPlan using TanStack Query and Zustand.

**Philosophy:** TanStack Query for server data, Zustand for client state  
**Last Updated:** November 9, 2025

---

## 📋 Table of Contents

- [State Management Philosophy](#state-management-philosophy)
- [TanStack Query (Server State)](#tanstack-query-server-state)
- [Zustand (Client State)](#zustand-client-state)
- [Auth State Management](#auth-state-management)
- [UI State Management](#ui-state-management)
- [Advanced Patterns](#advanced-patterns)
- [Performance Optimization](#performance-optimization)
- [Common Pitfalls](#common-pitfalls)

---

## State Management Philosophy

### Separation of Concerns

**Two types of state:**

1. **Server State** (TanStack Query):
   - User data, posts, comments, etc.
   - Fetched from backend APIs
   - Cached, synchronized, and refreshed
   - Source of truth: Backend database

2. **Client State** (Zustand):
   - Auth tokens (persisted)
   - UI preferences (theme, sidebar state)
   - Form state (temporary, use react-hook-form)
   - Source of truth: Browser/local storage

### The Golden Rule

```tsx
// ✅ CORRECT: Server data in TanStack Query
function UserProfile() {
  const { data: user } = useUser(userId);  // ← Server data
  const { theme } = useUIStore();          // ← Client state
  
  return <div className={theme}>{user.name}</div>;
}

// ❌ WRONG: Server data in Zustand
const useUserStore = create((set) => ({
  user: null,  // ❌ Never store server data in Zustand!
  setUser: (user) => set({ user }),
}));
```

**Why?**

- TanStack Query handles caching, background refetching, stale data
- Zustand is for simple client-side state only
- Mixing them creates sync issues and bugs

---

## TanStack Query (Server State)

### Setup

**src/lib/query-client.ts:**

```typescript
import { QueryClient } from '@tanstack/react-query';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60 * 1000,      // 1 minute
      cacheTime: 5 * 60 * 1000,  // 5 minutes
      retry: 1,
      refetchOnWindowFocus: false,
    },
    mutations: {
      retry: 0,
    },
  },
});
```

**src/main.tsx:**

```typescript
import { QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { queryClient } from './lib/query-client';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  </React.StrictMode>
);
```

### Query Hooks

**src/lib/queries/useUser.ts:**

```typescript
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { getUser, updateUser } from '@/api/users';
import type { User, UpdateUserData } from '@/types/user';

// Query keys - centralized for consistency
export const userKeys = {
  all: ['users'] as const,
  lists: () => [...userKeys.all, 'list'] as const,
  list: (filters: string) => [...userKeys.lists(), { filters }] as const,
  details: () => [...userKeys.all, 'detail'] as const,
  detail: (id: string) => [...userKeys.details(), id] as const,
};

// Get single user
export function useUser(userId: string) {
  return useQuery({
    queryKey: userKeys.detail(userId),
    queryFn: () => getUser(userId),
    enabled: !!userId, // Only fetch if userId exists
  });
}

// Get current user (from auth token)
export function useCurrentUser() {
  const { user } = useAuthStore();
  
  return useQuery({
    queryKey: userKeys.detail(user?.id || ''),
    queryFn: () => getUser(user?.id || ''),
    enabled: !!user?.id,
  });
}

// Update user mutation
export function useUpdateUser() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateUserData }) =>
      updateUser(id, data),
    
    // Optimistic update
    onMutate: async ({ id, data }) => {
      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: userKeys.detail(id) });
      
      // Snapshot previous value
      const previousUser = queryClient.getQueryData<User>(userKeys.detail(id));
      
      // Optimistically update to new value
      if (previousUser) {
        queryClient.setQueryData<User>(userKeys.detail(id), {
          ...previousUser,
          ...data,
        });
      }
      
      return { previousUser };
    },
    
    // Rollback on error
    onError: (err, { id }, context) => {
      if (context?.previousUser) {
        queryClient.setQueryData(userKeys.detail(id), context.previousUser);
      }
    },
    
    // Refetch after success or error
    onSettled: (data, error, { id }) => {
      queryClient.invalidateQueries({ queryKey: userKeys.detail(id) });
    },
  });
}

// Delete user mutation
export function useDeleteUser() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (userId: string) => deleteUser(userId),
    onSuccess: () => {
      // Invalidate user lists
      queryClient.invalidateQueries({ queryKey: userKeys.lists() });
    },
  });
}
```

### Infinite Queries

**src/lib/queries/usePosts.ts:**

```typescript
import { useInfiniteQuery } from '@tanstack/react-query';
import { getPosts } from '@/api/posts';

export function useInfinitePosts() {
  return useInfiniteQuery({
    queryKey: ['posts', 'infinite'],
    queryFn: ({ pageParam = 1 }) => getPosts({ page: pageParam, limit: 20 }),
    getNextPageParam: (lastPage, pages) => {
      if (lastPage.hasMore) {
        return pages.length + 1;
      }
      return undefined;
    },
    initialPageParam: 1,
  });
}

// Usage in component
function PostList() {
  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
    isLoading,
  } = useInfinitePosts();

  const posts = data?.pages.flatMap((page) => page.posts) ?? [];

  return (
    <div>
      {posts.map((post) => (
        <PostCard key={post.id} post={post} />
      ))}
      
      {hasNextPage && (
        <Button onClick={() => fetchNextPage()} disabled={isFetchingNextPage}>
          {isFetchingNextPage ? 'Loading...' : 'Load More'}
        </Button>
      )}
    </div>
  );
}
```

### Dependent Queries

```typescript
// Query B depends on Query A
function UserPosts({ userId }: { userId: string }) {
  // First, get the user
  const { data: user, isLoading: userLoading } = useUser(userId);
  
  // Then, get their posts (only if we have the user)
  const { data: posts, isLoading: postsLoading } = useQuery({
    queryKey: ['posts', 'user', user?.id],
    queryFn: () => getUserPosts(user!.id),
    enabled: !!user?.id, // Only run if user.id exists
  });
  
  if (userLoading) return <Spinner />;
  if (postsLoading) return <Spinner />;
  
  return <PostList posts={posts} />;
}
```

### Parallel Queries

```typescript
// Fetch multiple things at once
function Dashboard() {
  const { data: user } = useCurrentUser();
  const { data: stats } = useQuery({
    queryKey: ['stats'],
    queryFn: getStats,
  });
  const { data: notifications } = useQuery({
    queryKey: ['notifications'],
    queryFn: getNotifications,
  });
  
  // All queries run in parallel!
  
  return (
    <div>
      <UserHeader user={user} />
      <StatsPanel stats={stats} />
      <NotificationList notifications={notifications} />
    </div>
  );
}
```

---

## Zustand (Client State)

### Auth Store

**src/stores/authStore.ts:**

```typescript
import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

interface User {
  id: string;
  email: string;
  username: string;
}

interface AuthState {
  accessToken: string | null;
  refreshToken: string | null;
  user: User | null;
  isAuthenticated: boolean;
}

interface AuthActions {
  setTokens: (accessToken: string, refreshToken: string) => void;
  setUser: (user: User) => void;
  logout: () => void;
}

type AuthStore = AuthState & AuthActions;

export const useAuthStore = create<AuthStore>()(
  persist(
    (set) => ({
      // Initial state
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,

      // Actions
      setTokens: (accessToken, refreshToken) =>
        set({ accessToken, refreshToken, isAuthenticated: true }),

      setUser: (user) =>
        set({ user }),

      logout: () =>
        set({
          accessToken: null,
          refreshToken: null,
          user: null,
          isAuthenticated: false,
        }),
    }),
    {
      name: 'auth-storage',
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        // Only persist these fields
        accessToken: state.accessToken,
        refreshToken: state.refreshToken,
        user: state.user,
        isAuthenticated: state.isAuthenticated,
      }),
    }
  )
);

// Selector hooks for better performance
export const useAccessToken = () => useAuthStore((state) => state.accessToken);
export const useUser = () => useAuthStore((state) => state.user);
export const useIsAuthenticated = () => useAuthStore((state) => state.isAuthenticated);
```

### UI Store

**src/stores/uiStore.ts:**

```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface UIState {
  theme: 'light' | 'dark' | 'system';
  sidebarOpen: boolean;
  notifications: boolean;
}

interface UIActions {
  setTheme: (theme: UIState['theme']) => void;
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  toggleNotifications: () => void;
}

type UIStore = UIState & UIActions;

export const useUIStore = create<UIStore>()(
  persist(
    (set) => ({
      // Initial state
      theme: 'system',
      sidebarOpen: true,
      notifications: true,

      // Actions
      setTheme: (theme) => set({ theme }),
      
      toggleSidebar: () =>
        set((state) => ({ sidebarOpen: !state.sidebarOpen })),
      
      setSidebarOpen: (open) =>
        set({ sidebarOpen: open }),
      
      toggleNotifications: () =>
        set((state) => ({ notifications: !state.notifications })),
    }),
    {
      name: 'ui-storage',
    }
  )
);

// Selector hooks
export const useTheme = () => useUIStore((state) => state.theme);
export const useSidebarOpen = () => useUIStore((state) => state.sidebarOpen);
```

---

## Auth State Management

### Login Flow

**src/pages/LoginPage.tsx:**

```typescript
import { useAuthStore } from '@/stores/authStore';
import { useLogin } from '@/lib/queries/useAuth';

export function LoginPage() {
  const { setTokens, setUser } = useAuthStore();
  const navigate = useNavigate();
  
  const loginMutation = useLogin({
    onSuccess: (data) => {
      // Store tokens in Zustand
      setTokens(data.access_token, data.refresh_token);
      setUser(data.user);
      
      // Redirect
      navigate('/dashboard');
    },
  });

  const handleSubmit = (formData: LoginFormData) => {
    loginMutation.mutate(formData);
  };

  return (
    <LoginForm
      onSubmit={handleSubmit}
      isLoading={loginMutation.isPending}
      error={loginMutation.error}
    />
  );
}
```

### Protected Routes

**src/components/ProtectedRoute.tsx:**

```typescript
import { Navigate } from 'react-router-dom';
import { useAuthStore } from '@/stores/authStore';

export function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}

// Usage in router
<Route
  path="/dashboard"
  element={
    <ProtectedRoute>
      <DashboardPage />
    </ProtectedRoute>
  }
/>
```

### Token Refresh

**src/lib/api-client.ts:**

```typescript
import axios from 'axios';
import { useAuthStore } from '@/stores/authStore';

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  timeout: 30000,
});

// Request interceptor - add auth token
apiClient.interceptors.request.use(
  (config) => {
    const token = useAuthStore.getState().accessToken;
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor - handle token refresh
let isRefreshing = false;
let failedQueue: Array<{
  resolve: (value?: unknown) => void;
  reject: (reason?: unknown) => void;
}> = [];

const processQueue = (error: unknown, token: string | null = null) => {
  failedQueue.forEach((prom) => {
    if (error) {
      prom.reject(error);
    } else {
      prom.resolve(token);
    }
  });
  failedQueue = [];
};

apiClient.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    // If 401 and not already retrying
    if (error.response?.status === 401 && !originalRequest._retry) {
      if (isRefreshing) {
        // Queue this request
        return new Promise((resolve, reject) => {
          failedQueue.push({ resolve, reject });
        })
          .then((token) => {
            originalRequest.headers.Authorization = `Bearer ${token}`;
            return apiClient(originalRequest);
          })
          .catch((err) => Promise.reject(err));
      }

      originalRequest._retry = true;
      isRefreshing = true;

      const { refreshToken, setTokens, logout } = useAuthStore.getState();

      if (!refreshToken) {
        logout();
        return Promise.reject(error);
      }

      try {
        const { data } = await axios.post('/api/auth/refresh', {
          refresh_token: refreshToken,
        });

        setTokens(data.access_token, data.refresh_token);
        
        apiClient.defaults.headers.common.Authorization = `Bearer ${data.access_token}`;
        originalRequest.headers.Authorization = `Bearer ${data.access_token}`;
        
        processQueue(null, data.access_token);
        
        return apiClient(originalRequest);
      } catch (refreshError) {
        processQueue(refreshError, null);
        logout();
        return Promise.reject(refreshError);
      } finally {
        isRefreshing = false;
      }
    }

    return Promise.reject(error);
  }
);

export default apiClient;
```

---

## UI State Management

### Theme Management

**src/hooks/useTheme.ts:**

```typescript
import { useEffect } from 'react';
import { useUIStore } from '@/stores/uiStore';

export function useTheme() {
  const { theme, setTheme } = useUIStore();

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove('light', 'dark');

    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';
      root.classList.add(systemTheme);
    } else {
      root.classList.add(theme);
    }
  }, [theme]);

  return { theme, setTheme };
}

// Usage
function ThemeToggle() {
  const { theme, setTheme } = useTheme();

  return (
    <Select value={theme} onValueChange={setTheme}>
      <SelectTrigger>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="light">Light</SelectItem>
        <SelectItem value="dark">Dark</SelectItem>
        <SelectItem value="system">System</SelectItem>
      </SelectContent>
    </Select>
  );
}
```

---

## Advanced Patterns

### Cache Invalidation

```typescript
import { useQueryClient } from '@tanstack/react-query';

function useCreatePost() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createPost,
    onSuccess: () => {
      // Invalidate all post queries
      queryClient.invalidateQueries({ queryKey: ['posts'] });
      
      // Or be more specific
      queryClient.invalidateQueries({ queryKey: ['posts', 'list'] });
      
      // Or invalidate multiple
      queryClient.invalidateQueries({
        predicate: (query) =>
          query.queryKey[0] === 'posts' || query.queryKey[0] === 'feed',
      });
    },
  });
}
```

### Prefetching

```typescript
function PostList() {
  const queryClient = useQueryClient();
  const { data: posts } = usePosts();

  const prefetchPost = (postId: string) => {
    queryClient.prefetchQuery({
      queryKey: ['posts', postId],
      queryFn: () => getPost(postId),
      staleTime: 60 * 1000, // Cache for 1 minute
    });
  };

  return (
    <div>
      {posts.map((post) => (
        <div
          key={post.id}
          onMouseEnter={() => prefetchPost(post.id)}
        >
          {post.title}
        </div>
      ))}
    </div>
  );
}
```

### Polling

```typescript
function LiveNotifications() {
  const { data: notifications } = useQuery({
    queryKey: ['notifications'],
    queryFn: getNotifications,
    refetchInterval: 30 * 1000, // Poll every 30 seconds
  });

  return <NotificationList notifications={notifications} />;
}
```

---

## Performance Optimization

### Use Selectors in Zustand

```typescript
// ❌ Bad: Re-renders on any auth state change
function Component() {
  const authState = useAuthStore(); // Subscribes to entire store
  return <div>{authState.user?.name}</div>;
}

// ✅ Good: Only re-renders when user changes
function Component() {
  const user = useAuthStore((state) => state.user); // Subscribes to user only
  return <div>{user?.name}</div>;
}
```

### Split Large Stores

```typescript
// ❌ Bad: One giant store
const useAppStore = create((set) => ({
  user: null,
  theme: 'light',
  posts: [],
  comments: [],
  // ... 50 more fields
}));

// ✅ Good: Multiple focused stores
const useAuthStore = create((set) => ({ user: null, ... }));
const useUIStore = create((set) => ({ theme: 'light', ... }));
const usePostsStore = create((set) => ({ posts: [], ... }));
```

---

## Common Pitfalls

### ❌ Don't Mix Server Data and Client State

```typescript
// ❌ WRONG
const useStore = create((set) => ({
  user: null, // Server data - should use TanStack Query!
  theme: 'light', // Client state - OK in Zustand
}));

// ✅ CORRECT
const { data: user } = useUser(userId); // Server data
const theme = useUIStore((state) => state.theme); // Client state
```

### ❌ Don't Use Zustand for Forms

```typescript
// ❌ WRONG
const useFormStore = create((set) => ({
  email: '',
  password: '',
  setEmail: (email) => set({ email }),
  setPassword: (password) => set({ password }),
}));

// ✅ CORRECT - use react-hook-form
const { register, handleSubmit } = useForm<LoginFormData>();
```

### ❌ Don't Forget Query Keys

```typescript
// ❌ BAD: Inconsistent query keys
useQuery({ queryKey: ['user', id], ... });
useQuery({ queryKey: ['users', id], ... }); // Inconsistent!

// ✅ GOOD: Centralized query keys
export const userKeys = {
  detail: (id: string) => ['users', 'detail', id] as const,
};
useQuery({ queryKey: userKeys.detail(id), ... });
```

---

## Related Guides

- [Development Guide](./development-guide.md) - Setup and workflow
- [Testing Guide](./testing-guide.md) - Testing state
- [Component Patterns](./component-patterns.md) - Using state in components

---

**Remember:** TanStack Query for server data, Zustand for auth/UI state only!
