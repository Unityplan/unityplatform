# UnityPlan Platform - Technology Stack

## 📋 Table of Contents

1. [Overview](#overview)
2. [Backend Technologies](#backend-technologies)
3. [Frontend Technologies](#frontend-technologies)
4. [Communication & Messaging](#communication--messaging)
5. [Data Storage](#data-storage)
6. [Infrastructure & DevOps](#infrastructure--devops)
7. [Security & Authentication](#security--authentication)
8. [Observability & Monitoring](#observability--monitoring)
9. [Future Technologies](#future-technologies)
10. [Development Tools](#development-tools)

---

## Overview

UnityPlan is built on a **microservices architecture** with a clear separation between backend services (Rust), frontend application (React), and communication infrastructure (Matrix Protocol). The stack is designed for:

- **Performance**: Rust for high-throughput, low-latency services
- **Developer Experience**: Modern tooling with Vite, TypeScript, and hot reload
- **Scalability**: Independent service scaling with Docker orchestration
- **Security**: Zero-trust architecture with mTLS and E2E encryption
- **Future-Proof**: Preparation for migration to fully decentralized systems

---

## Backend Technologies

### Core Language & Runtime

#### **Rust 1.91.0**
- **Why**: Memory safety, zero-cost abstractions, fearless concurrency
- **Benefits**: 
  - No garbage collection overhead
  - Compile-time error prevention
  - Native performance comparable to C/C++
  - Strong type system prevents entire classes of bugs
- **Use Cases**: All microservices, API servers, WebSocket handlers

---

### Web Framework & APIs

#### **actix-web 4.x**
```toml
actix-web = "4.5"
actix-rt = "2.9"
```

- **Purpose**: HTTP REST API framework
- **Features**:
  - Actor-based concurrency model
  - Middleware support for cross-cutting concerns
  - JSON serialization/deserialization
  - Route guards and extractors
  - Built-in compression and CORS
- **Performance**: One of the fastest web frameworks (any language)
- **Use Cases**: 
  - REST API endpoints
  - Request routing and validation
  - Authentication middleware
  - Rate limiting

**Example Service Structure**:
```rust
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1")
                .service(users::routes())
                .service(courses::routes())
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

---

### WebSocket Communication

#### **tokio-tungstenite**
```toml
tokio-tungstenite = "0.21"
tokio = { version = "1.35", features = ["full"] }
```

- **Purpose**: Real-time bidirectional communication
- **Features**:
  - WebSocket protocol implementation
  - Async/await support via Tokio
  - Message framing and ping/pong
  - TLS support
- **Use Cases**:
  - Live chat updates
  - Real-time notifications
  - Collaborative editing
  - Dashboard metrics streaming

**Connection Pattern**:
```rust
async fn handle_websocket(ws: WebSocket, state: AppState) {
    let (mut tx, mut rx) = ws.split();
    
    while let Some(msg) = rx.next().await {
        // Process incoming messages
        // Broadcast to other clients
        // Update state
    }
}
```

---

### Database Layer

#### **SQLx 0.7.x**
```toml
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "json"] }
```

- **Purpose**: Async, compile-time verified SQL queries
- **Features**:
  - **Compile-time query checking**: SQL errors caught at build time
  - Connection pooling
  - Migrations support
  - Transaction management
  - Type-safe query results
- **Why not an ORM**: 
  - Full SQL control for complex queries
  - No runtime query generation overhead
  - Explicit database operations
  - Better performance profiling

**Query Example**:
```rust
// Compile-time verified!
let user = sqlx::query_as!(
    User,
    r#"
    SELECT id, username, email, created_at
    FROM users
    WHERE id = $1
    "#,
    user_id
)
.fetch_one(&pool)
.await?;
```

#### **TimescaleDB (PostgreSQL Extension)**
```yaml
postgres:
  image: timescale/timescaledb:latest-pg16
```

- **Purpose**: Time-series data optimization
- **Features**:
  - Automatic partitioning by time
  - Compression for historical data
  - Continuous aggregates
  - Retention policies
- **Use Cases**:
  - User activity logs
  - Learning progress tracking
  - System metrics
  - Audit trails

**Schema Design**:
```sql
-- Hypertable for events
CREATE TABLE user_events (
    time TIMESTAMPTZ NOT NULL,
    user_id UUID NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    metadata JSONB
);

SELECT create_hypertable('user_events', 'time');
```

---

### Multi-Tenancy

#### **PostgreSQL Schema Isolation**
- **Strategy**: Separate schema per territory
- **Benefits**:
  - Data isolation at database level
  - Simpler queries (no tenant_id filtering)
  - Independent backups/restores
  - Easier compliance (GDPR, data residency)

**Implementation**:
```rust
// Connection pool per territory
pub struct TenantPool {
    pools: HashMap<TerritoryId, PgPool>,
}

impl TenantPool {
    pub async fn get(&self, territory: &TerritoryId) -> Result<&PgPool> {
        self.pools.get(territory)
            .ok_or(Error::TerritoryNotFound)
    }
}
```

**Schema Pattern**:
```
database: unityplan
├── schema: territory_dk (Denmark)
│   ├── users
│   ├── courses
│   └── communities
├── schema: territory_ca (Canada)
│   ├── users
│   ├── courses
│   └── communities
└── schema: global
    ├── territories
    ├── badge_templates
    └── translations
```

---

### Message Queue & Event Bus

#### **NATS 2.x**
```toml
async-nats = "0.33"
```

- **Purpose**: Inter-service communication and event distribution
- **Features**:
  - Publish/subscribe patterns
  - Request/reply for RPC
  - Queue groups for load balancing
  - JetStream for persistence
  - Message acknowledgment
- **Use Cases**:
  - Service-to-service messaging
  - Event-driven workflows
  - Background job processing
  - Cache invalidation
  - Webhook delivery

**Event Pattern**:
```rust
// Publisher
nats.publish(
    "events.course.completed",
    serde_json::to_vec(&CourseCompletedEvent {
        user_id,
        course_id,
        timestamp: Utc::now(),
    })?
).await?;

// Subscriber
let mut sub = nats.subscribe("events.course.*").await?;
while let Some(msg) = sub.next().await {
    let event: CourseEvent = serde_json::from_slice(&msg.payload)?;
    handle_course_event(event).await?;
}
```

---

### Configuration Management

#### **config + dotenvy**
```toml
config = "0.14"
dotenvy = "0.15"
serde = { version = "1.0", features = ["derive"] }
```

- **Purpose**: Environment-based configuration
- **Features**:
  - Multiple config file formats (TOML, YAML, JSON)
  - Environment variable overrides
  - Hierarchical settings
  - Type-safe config structs

**Configuration Pattern**:
```rust
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub nats: NatsConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

// Load from config/default.toml + .env overrides
let config = Config::builder()
    .add_source(config::File::with_name("config/default"))
    .add_source(config::Environment::with_prefix("APP"))
    .build()?;
```

---

## Frontend Technologies

> **Architecture Decision**: Production-grade SPA stack optimized for 2025
> 
> **Why React 18 (not React 19)**:
> - Fully stable ecosystem with all dependencies optimized for React 18
> - React 19's biggest gains (Server Components, Actions) are for SSR, not pure SPAs
> - Defers upgrade until Tauri mobile/desktop phase (incremental migration path)
> - Avoids bleeding-edge instability during MVP development
>
> **Future-Proofing for Tauri** (~1 year timeline):
> - Current stack will mostly just work — React + Vite officially supported in Tauri templates
> - Routing: Only needs `createHashHistory()` instead of browser history (file-based URLs)
> - Styling: Tailwind + shadcn work perfectly (CSS-based, not DOM-dependent)
> - Storage: Zustand's localStorage can swap to Tauri's secure storage APIs
> - Data Layer: TanStack Query is runtime-agnostic (browser vs. desktop webview)

### Build Tool & Dev Server

#### **Vite 5.x**
```json
{
  "devDependencies": {
    "vite": "^5.0.0",
    "rollup-plugin-visualizer": "^5.12.0"
  }
}
```

- **Purpose**: Lightning-fast development and optimized production builds
- **Features**:
  - Native ES modules in dev
  - Hot Module Replacement (HMR)
  - Optimized bundling with Rollup
  - Plugin ecosystem
  - CSS code splitting
  - Code splitting via dynamic imports
- **Benefits**:
  - Sub-second dev server startup
  - Instant hot reload
  - Tree-shaking for smaller bundles
  - Built-in TypeScript support
- **Performance**: Works perfectly with React 18, Tailwind, and TypeScript
- **Tooling**: Add bundle analyzer early to monitor dependency bloat

**Build Configuration**:
```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { visualizer } from 'rollup-plugin-visualizer';

export default defineConfig({
  plugins: [
    react(),
    visualizer({ open: true, gzipSize: true })
  ],
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          'react-vendor': ['react', 'react-dom'],
          'router': ['@tanstack/react-router'],
          'query': ['@tanstack/react-query']
        }
      }
    }
  }
});
```

---

### UI Framework

#### **React 18.x**
```json
{
  "dependencies": {
    "react": "^18.3.0",
    "react-dom": "^18.3.0"
  }
}
```

- **Purpose**: Component-based UI development with stable, production-ready ecosystem
- **Why React 18**:
  - **Library Ecosystem**: Every dependency fully stable and optimized for React 18
  - **Fewer Breaking Edges**: React 19 features (`use`, Actions, form features) not needed for pure SPA
  - **SSR Not in Play Yet**: React 19's biggest gains don't apply to client-only apps
  - **Upgrade Path**: Incremental migration to React 19 during Tauri phase (when SSR may be needed)
- **Key Features**:
  - **Concurrent Rendering**: Automatic batching, transitions, Suspense
  - **Hooks**: useState, useEffect, useContext, useReducer, useMemo, useCallback
  - **React 18 Specific**: useTransition, useDeferredValue, useId
  - **Automatic Batching**: Multiple state updates grouped for performance
  - **Streaming SSR** (future): Ready when needed for Tauri/Next.js migration

**Component Pattern**:
```tsx
import { useState, useEffect } from 'react';

export function CourseCard({ courseId }: { courseId: string }) {
  const [course, setCourse] = useState<Course | null>(null);
  
  useEffect(() => {
    fetchCourse(courseId).then(setCourse);
  }, [courseId]);
  
  return (
    <div className="course-card">
      <h3>{course?.title}</h3>
      <p>{course?.description}</p>
    </div>
  );
}
```

---

### Data Layer & Caching

#### **TanStack Query v5.x**
```json
{
  "dependencies": {
    "@tanstack/react-query": "^5.0.0",
    "@tanstack/react-query-devtools": "^5.0.0"
  }
}
```

- **Purpose**: Async data fetching, caching, and background refetching
- **Why TanStack Query**:
  - **Standardized Data Fetching**: Eliminates custom useEffect + useState patterns
  - **Automatic Caching**: Reduces backend load and improves UX
  - **Background Refetching**: Keeps data fresh without user intervention
  - **Optimistic Updates**: Instant UI updates with automatic rollback on error
  - **Request Deduplication**: Multiple components requesting same data only make one network call
  - **Simplified Global State**: Offloads data fetching from Zustand (use Zustand only for UI state)
- **Features**:
  - Automatic background refetching
  - Query invalidation and refetching
  - Parallel and dependent queries
  - Infinite scroll queries
  - Mutations with optimistic updates
  - Pagination support
  - DevTools for debugging
- **Integration**: Designed for co-usage with TanStack Router

**Query Pattern**:
```tsx
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

// Fetch data
function useCourse(courseId: string) {
  return useQuery({
    queryKey: ['course', courseId],
    queryFn: () => fetchCourse(courseId),
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// Mutate data
function useUpdateProfile() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: updateProfile,
    onSuccess: () => {
      // Invalidate and refetch
      queryClient.invalidateQueries({ queryKey: ['profile'] });
    },
  });
}

// Usage in component
function CourseDetail({ courseId }: { courseId: string }) {
  const { data: course, isLoading, error } = useCourse(courseId);
  
  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;
  
  return <div>{course.title}</div>;
}
```

**DevTools Integration**:
```tsx
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <YourApp />
      {process.env.NODE_ENV === 'development' && (
        <ReactQueryDevtools initialIsOpen={false} />
      )}
    </QueryClientProvider>
  );
}
```

---

### Routing

#### **TanStack Router v1.13x**
```json
{
  "dependencies": {
    "@tanstack/react-router": "^1.134.10"
  }
}
```

- **Purpose**: Type-safe client-side routing with seamless TanStack Query integration
- **Features**:
  - Full TypeScript support
  - Code splitting by route with lazy loading
  - Search param validation
  - Loaders and actions
  - Nested layouts
  - Route guards
  - Co-designed with TanStack Query
- **Benefits**:
  - Catch routing errors at compile time
  - Automatic loading states
  - Robust nested routing
  - Better DevX than React Router

**Route Definition with Lazy Loading**:
```tsx
import { createFileRoute } from '@tanstack/react-router';
import { lazy } from 'react';

// Lazy load components for code splitting
const CourseDetail = lazy(() => import('./pages/CourseDetail'));
const Dashboard = lazy(() => import('./pages/Dashboard'));

export const Route = createFileRoute('/courses/$courseId')({
  loader: async ({ params }) => {
    return fetchCourse(params.courseId);
  },
  component: CourseDetail,
});
```

**Route Guards (Protected Routes)**:
```tsx
import { redirect } from '@tanstack/react-router';

export const Route = createFileRoute('/dashboard')({
  beforeLoad: async ({ context }) => {
    if (!context.auth.isAuthenticated) {
      throw redirect({ to: '/login' });
    }
  },
  component: Dashboard,
});
```

---

### Styling

#### **TailwindCSS 4.x**
```json
{
  "devDependencies": {
    "tailwindcss": "^4.1.16"
  }
}
```

- **Purpose**: Utility-first CSS framework
- **Features**:
  - JIT (Just-In-Time) compilation
  - Custom design system with CSS variables for theming
  - Responsive utilities
  - Dark mode support
  - Plugin ecosystem
  - Improved build performance in v4
- **Benefits**:
  - Rapid prototyping
  - Consistent design language
  - Minimal CSS bundle size
  - No naming conflicts
- **Theming Strategy**: Define `theme.config.ts` early for Tauri migration (dark/light/system)

**Usage Example**:
```tsx
<div className="flex items-center gap-4 rounded-lg bg-white p-6 shadow-sm hover:shadow-md transition-shadow dark:bg-gray-800">
  <img src={avatar} className="h-12 w-12 rounded-full" />
  <div className="flex-1">
    <h3 className="text-lg font-semibold text-gray-900 dark:text-white">{name}</h3>
    <p className="text-sm text-gray-600 dark:text-gray-400">{role}</p>
  </div>
</div>
```

**Theme Configuration**:
```typescript
// theme.config.ts
export const theme = {
  colors: {
    primary: 'var(--color-primary)',
    secondary: 'var(--color-secondary)',
    background: 'var(--color-background)',
    foreground: 'var(--color-foreground)',
  },
};
```

#### **shadcn/ui 3.5.x**
```json
{
  "dependencies": {
    "@radix-ui/react-*": "latest"
  }
}
```

- **Purpose**: Headless, accessible component library with prebuilt primitives
- **Why shadcn/ui**:
  - **Not an npm package**: Copy-paste components into your codebase (full ownership)
  - **Built on Radix UI**: WAI-ARIA compliant, keyboard navigation, focus management
  - **Tailwind Integration**: Fully customizable with Tailwind classes
  - **Theme System**: Built-in dark mode and CSS variable-based theming
  - **Production-Grade**: Well-maintained, extensive component library
- **Features**:
  - Fully accessible (WAI-ARIA)
  - Customizable with Tailwind
  - Dark mode built-in
  - Theme system with CSS variables
  - TypeScript support
- **Components**: 
  - Buttons, Inputs, Modals, Dropdowns
  - Data Tables, Calendars, Popovers
  - Toast notifications, Tooltips
  - Command palette, Context menus
  - Forms with react-hook-form integration

**Component Example**:
```tsx
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export function Dashboard() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Welcome Back</CardTitle>
      </CardHeader>
      <CardContent>
        <Button variant="default">Continue Learning</Button>
        <Button variant="outline">View Profile</Button>
      </CardContent>
    </Card>
  );
}
```

---

### State Management

#### **Zustand (Local/Auth State Only)**
```json
{
  "dependencies": {
    "zustand": "^4.5.0"
  }
}
```

- **Purpose**: Lightweight state management for session, auth, and ephemeral UI state
- **Separation of Concerns**:
  - **Zustand**: Session/auth tokens, drawer toggles, theme mode, UI-only state
  - **TanStack Query**: All data fetching, caching, and server state
  - **Local Component State**: Component-specific UI state (useState)
- **Why This Split**:
  - Avoids unnecessary global state pollution
  - TanStack Query handles data caching better than manual state management
  - Zustand stays simple and focused
- **Features**:
  - Minimal boilerplate
  - No Provider wrapper needed
  - Persist to localStorage
  - TypeScript support
  - DevTools integration

**Auth Store Pattern**:
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
    {
      name: 'auth-storage',
    }
  )
);
```

**UI State Example**:
```typescript
interface UIState {
  sidebarOpen: boolean;
  theme: 'light' | 'dark' | 'system';
  toggleSidebar: () => void;
  setTheme: (theme: 'light' | 'dark' | 'system') => void;
}

export const useUIStore = create<UIState>((set) => ({
  sidebarOpen: false,
  theme: 'system',
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
  setTheme: (theme) => set({ theme }),
}));
```

---

### Forms & Validation

#### **react-hook-form + zod**
```json
{
  "dependencies": {
    "react-hook-form": "^7.51.0",
    "zod": "^3.22.0",
    "@hookform/resolvers": "^3.3.0"
  }
}
```

- **Purpose**: Type-safe form handling with validation
- **Why This Combo**:
  - **Best Practice**: Industry standard for React forms
  - **shadcn Integration**: Works seamlessly with shadcn UI components
  - **Static Typing**: Zod schemas provide TypeScript types
  - **Unification Potential**: Can share schemas between frontend and backend (tRPC, Fastify+zod)
- **Features**:
  - Minimal re-renders
  - Built-in validation
  - Error handling
  - Field arrays and nested forms
  - Async validation

**Form Pattern**:
```tsx
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

const loginSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
});

type LoginForm = z.infer<typeof loginSchema>;

export function LoginForm() {
  const { register, handleSubmit, formState: { errors } } = useForm<LoginForm>({
    resolver: zodResolver(loginSchema),
  });
  
  const onSubmit = (data: LoginForm) => {
    console.log(data);
  };
  
  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div>
        <Input {...register('email')} placeholder="Email" />
        {errors.email && <p className="text-sm text-red-500">{errors.email.message}</p>}
      </div>
      <div>
        <Input {...register('password')} type="password" placeholder="Password" />
        {errors.password && <p className="text-sm text-red-500">{errors.password.message}</p>}
      </div>
      <Button type="submit">Login</Button>
    </form>
  );
}
```

---

### Type Safety

#### **TypeScript (Latest)**
```json
{
  "devDependencies": {
    "typescript": "^5.3.0"
  }
}
```

- **Purpose**: Static type checking for JavaScript
- **Why Essential**: 
  - Improves safety, refactoring capabilities, and scalability
  - Self-documenting code with type annotations
  - Catch errors before runtime
  - Better IDE autocomplete and IntelliSense
- **Configuration**:
  - Strict mode enabled
  - Path aliases (@/ for src/)
  - ESNext target
  - React JSX transform
- **Benefits**:
  - Catch errors before runtime
  - Better IDE autocomplete
  - Safer refactoring
  - Self-documenting code

**Type Pattern**:
```typescript
// API response types
interface Course {
  id: string;
  title: string;
  description: string;
  badges: Badge[];
  createdAt: Date;
}

interface Badge {
  id: string;
  name: string;
  imageUrl: string;
}

// API client with types
async function fetchCourse(id: string): Promise<Course> {
  const response = await fetch(`/api/courses/${id}`);
  return response.json();
}
```

---

### Testing & Quality

#### **Vitest (Unit Tests)**
```json
{
  "devDependencies": {
    "vitest": "^1.0.0",
    "@testing-library/react": "^14.0.0",
    "@testing-library/jest-dom": "^6.0.0"
  }
}
```

- **Purpose**: Unit and component testing
- **Features**:
  - Native Vite integration
  - Fast test execution
  - Jest-compatible API
  - Component testing with Testing Library

#### **Playwright or Cypress (E2E Tests)**
```json
{
  "devDependencies": {
    "@playwright/test": "^1.40.0"
  }
}
```

- **Purpose**: End-to-end testing of user flows
- **Use Cases**: 
  - Login/registration flows
  - Profile editing
  - Course enrollment
  - Forum interactions

#### **ESLint + Prettier**
```json
{
  "devDependencies": {
    "eslint": "^8.55.0",
    "prettier": "^3.1.0",
    "@typescript-eslint/parser": "^6.15.0",
    "@typescript-eslint/eslint-plugin": "^6.15.0"
  }
}
```

- **Purpose**: Code quality and formatting
- **Configuration**: TypeScript strict mode enabled
- **Benefits**: Consistent code style, catch common errors

---

### Build Optimization

#### **Code Splitting & Lazy Loading**
```tsx
import { lazy, Suspense } from 'react';

// Lazy load route components
const Home = lazy(() => import('./pages/Home'));
const Dashboard = lazy(() => import('./pages/Dashboard'));

// Use with Suspense
function App() {
  return (
    <Suspense fallback={<Loading />}>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/dashboard" element={<Dashboard />} />
      </Routes>
    </Suspense>
  );
}
```

#### **Bundle Analysis**
```json
{
  "devDependencies": {
    "rollup-plugin-visualizer": "^5.12.0"
  }
}
```

- **Purpose**: Monitor dependency bloat and bundle size
- **Strategy**: Add early in development, review regularly
- **Target**: Keep initial bundle < 200KB gzipped

---

### Optional Enhancements (When Ready)

#### **tRPC or Hono RPC**
- **Purpose**: Type-safe client-server communication
- **When**: If backend moves to TypeScript or needs RPC
- **Benefits**: End-to-end type safety from backend to frontend

#### **Jotai or Recoil**
- **Purpose**: Fine-grained reactivity for complex shared state
- **When**: If Zustand becomes insufficient for state complexity
- **Current Status**: Zustand likely sufficient for MVP

#### **Vite PWA Plugin**
- **Purpose**: Offline-capable Progressive Web App
- **When**: Before Tauri migration for offline support
- **Benefits**: Service worker, offline caching, install prompt

#### **Framer Motion or Motion One**
- **Purpose**: Micro-interactions and animations
- **When**: After MVP, for polish and delight
- **Integration**: Works nicely with shadcn components

---

## Communication & Messaging

### Matrix Protocol

#### **Ruma (Rust)**
```toml
ruma = { version = "0.9", features = ["client-api", "federation-api"] }
ruma-client = "0.12"
```

- **Purpose**: Decentralized communication protocol implementation
- **Features**:
  - Type-safe Matrix API bindings
  - Client and server APIs
  - Event types and schemas
  - End-to-end encryption support
- **Use Cases**:
  - Forum backend
  - Chat room management
  - Federation with other servers
  - Event streaming

#### **matrix-js-sdk (Frontend)**
```json
{
  "dependencies": {
    "matrix-js-sdk": "^32.0.0"
  }
}
```

- **Purpose**: Client-side Matrix integration
- **Features**:
  - Room creation and management
  - Message sending/receiving
  - User presence
  - E2E encryption
  - File uploads
- **Use Cases**:
  - Real-time chat UI
  - Forum interface
  - Notifications
  - User directory

**Matrix Integration Example**:
```typescript
import { createClient } from 'matrix-js-sdk';

const client = createClient({
  baseUrl: 'https://matrix.unityplan.org',
  accessToken: userToken,
  userId: '@user:unityplan.org',
});

// Join a room
await client.joinRoom('!roomId:unityplan.org');

// Send a message
await client.sendTextMessage(roomId, 'Hello, world!');

// Listen for messages
client.on('Room.timeline', (event) => {
  if (event.getType() === 'm.room.message') {
    console.log(event.getContent().body);
  }
});
```

---

### WebSocket API

- **Protocol**: RFC 6455 WebSocket
- **Library**: tokio-tungstenite (backend), native WebSocket API (frontend)
- **Use Cases**:
  - Real-time notifications
  - Live updates
  - Collaborative features
  - Status changes

---

## Data Storage

### Primary Database

#### **PostgreSQL 16**
```yaml
services:
  postgres:
    image: postgres:16-alpine
```

- **Purpose**: Primary relational database
- **Features**:
  - ACID compliance
  - JSONB support for flexible schemas
  - Full-text search
  - Row-level security
  - Partitioning
  - Replication
- **Extensions Used**:
  - `uuid-ossp`: UUID generation
  - `pg_trgm`: Fuzzy text search
  - `pg_stat_statements`: Query performance monitoring

### Time-Series Data

#### **TimescaleDB**
- Built on PostgreSQL 16
- Automatic time-based partitioning
- Compression (90%+ savings)
- Continuous aggregates for analytics
- Downsampling for historical data

### Decentralized Storage

#### **IPFS (InterPlanetary File System)**
```toml
ipfs-api = "0.17"
```

- **Purpose**: Decentralized file storage
- **Features**:
  - Content-addressed storage
  - Deduplication
  - Peer-to-peer distribution
  - Immutable references
- **Use Cases**:
  - Course materials (PDFs, videos)
  - User-uploaded content
  - Avatar images
  - Static assets
  - Backup storage

**IPFS Integration**:
```rust
use ipfs_api::IpfsClient;

async fn upload_file(client: &IpfsClient, data: Vec<u8>) -> Result<String> {
    let cursor = std::io::Cursor::new(data);
    let response = client.add(cursor).await?;
    Ok(response.hash) // CID (Content Identifier)
}

async fn download_file(client: &IpfsClient, cid: &str) -> Result<Vec<u8>> {
    let bytes = client.cat(cid).map_ok(|chunk| chunk.to_vec()).try_concat().await?;
    Ok(bytes)
}
```

---

## Infrastructure & DevOps

### Containerization

#### **Docker**
```yaml
version: '3.8'
```

- **Purpose**: Application containerization
- **Features**:
  - Isolated environments
  - Reproducible builds
  - Version control for infrastructure
  - Multi-stage builds for optimization

**Rust Service Dockerfile**:
```dockerfile
# Build stage
FROM rust:1.91-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev openssl-dev
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/service /usr/local/bin/
CMD ["service"]
```

#### **Docker Compose**
- **Purpose**: Multi-container orchestration
- **Features**:
  - Service dependencies
  - Network isolation
  - Volume management
  - Environment variables
  - Health checks

---

### Service Mesh

#### **Traefik (Primary Option)**
```yaml
services:
  traefik:
    image: traefik:v3.0
```

- **Purpose**: Reverse proxy and load balancer
- **Features**:
  - Automatic service discovery
  - TLS termination
  - Rate limiting
  - Circuit breakers
  - Metrics export

#### **Linkerd (Alternative/Future)**
- **Purpose**: Service mesh with mTLS
- **Features**:
  - Zero-trust security between services
  - Automatic mTLS
  - Load balancing
  - Retry budgets
  - Traffic splitting

---

### CI/CD

#### **GitHub Actions**
```yaml
name: CI/CD
on: [push, pull_request]
```

- **Purpose**: Automated testing and deployment
- **Workflows**:
  - **Test**: Run unit and integration tests
  - **Lint**: Code quality checks (Clippy, ESLint)
  - **Build**: Docker image creation
  - **Deploy**: Push to registry, update services
  - **Security**: Dependency scanning, SAST

**Example Workflow**:
```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --all-features
      
  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: docker/build-push-action@v5
        with:
          push: true
          tags: ghcr.io/unityplan/service:${{ github.sha }}
```

---

## Security & Authentication

### Authentication

#### **OpenID Connect (OIDC)**
```toml
openidconnect = "3.5"
jsonwebtoken = "9.2"
```

- **Purpose**: Single Sign-On (SSO) and federated identity
- **Features**:
  - OAuth 2.0 authorization
  - JWT token issuance
  - Token validation
  - Refresh tokens
  - PKCE flow for public clients
- **Providers**: Support for multiple OIDC providers
  - Keycloak (self-hosted)
  - Auth0, Okta (managed)
  - Custom implementations

**OIDC Flow**:
```rust
use openidconnect::{
    core::CoreClient,
    AuthenticationFlow,
    AuthorizationCode,
    CsrfToken,
    Nonce,
    PkceCodeChallenge,
};

// Initiate login
let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
let (auth_url, csrf_token, nonce) = client
    .authorize_url(
        AuthenticationFlow::AuthorizationCode,
        CsrfToken::new_random,
        Nonce::new_random,
    )
    .set_pkce_challenge(pkce_challenge)
    .url();

// Exchange code for token
let token_response = client
    .exchange_code(AuthorizationCode::new(code))
    .set_pkce_verifier(pkce_verifier)
    .request_async(async_http_client)
    .await?;
```

### Session Management

#### **JWT (JSON Web Tokens)**
- **Purpose**: Stateless authentication
- **Claims**:
  - `sub`: User ID
  - `territory`: Territory ID
  - `roles`: User roles
  - `badges`: Earned badges
  - `exp`: Expiration timestamp
- **Security**:
  - RS256 signing (asymmetric keys)
  - Short expiration (15 minutes)
  - Refresh tokens for renewal
  - Token rotation on refresh

### Encryption

#### **TLS/mTLS**
- **TLS 1.3**: All external communication
- **mTLS**: Service-to-service authentication
- **Certificate Management**: Let's Encrypt + cert-manager

#### **End-to-End Encryption**
- **Matrix E2E**: Olm/Megolm for message encryption
- **libsodium**: Additional crypto primitives
- **User Control**: Users hold encryption keys

---

## Observability & Monitoring

### Logging

#### **tracing + tracing-subscriber**
```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-opentelemetry = "0.22"
```

- **Purpose**: Structured logging and distributed tracing
- **Features**:
  - Structured log events
  - Contextual spans
  - Multiple output formats (JSON, pretty)
  - Log level filtering
  - Integration with OpenTelemetry

**Logging Pattern**:
```rust
use tracing::{info, error, instrument};

#[instrument(skip(db))]
async fn create_user(db: &PgPool, username: String) -> Result<User> {
    info!("Creating user: {}", username);
    
    let user = sqlx::query_as!(...)
        .fetch_one(db)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            Error::DatabaseError(e)
        })?;
    
    info!(user_id = %user.id, "User created successfully");
    Ok(user)
}
```

### Metrics

#### **OpenTelemetry + Prometheus**
```toml
opentelemetry = "0.21"
opentelemetry-prometheus = "0.14"
```

- **Purpose**: Application metrics collection
- **Metrics Types**:
  - **Counters**: Total requests, errors
  - **Gauges**: Active connections, memory usage
  - **Histograms**: Request duration, query time
- **Dashboards**: Grafana visualization

### Distributed Tracing

#### **Jaeger**
- **Purpose**: Request flow visualization across services
- **Features**:
  - End-to-end request tracing
  - Performance bottleneck identification
  - Dependency mapping
  - Error tracking

---

## Future Technologies

### Full Decentralization

#### **Holochain**
```toml
# Future dependency
hdk = "0.3"  # Holochain Development Kit
```

- **Purpose**: Peer-to-peer application framework
- **Vision**: Migrate frontend and core logic to DNA modules
- **Benefits**:
  - True data ownership (user's device)
  - No server infrastructure
  - Cryptographic integrity
  - Agent-centric design
  - Offline-first by default

**Planned Migration**:
1. **Phase 1**: User profiles as Holochain entries
2. **Phase 2**: Badges and credentials on-chain
3. **Phase 3**: Learning progress tracking
4. **Phase 4**: Full LMS on Holochain
5. **Phase 5**: Sunset centralized services

#### **Holochain Client (Frontend)**
```typescript
import { AppWebsocket } from '@holochain/client';

const client = await AppWebsocket.connect(
  'ws://localhost:8888',
  'unityplan'
);

// Create an entry
await client.callZome({
  role_name: 'unityplan',
  zome_name: 'profiles',
  fn_name: 'create_profile',
  payload: { username, bio },
});
```

---

### Cross-Platform Applications

#### **Tauri**
```toml
[dependencies]
tauri = "2.0"
```

- **Purpose**: Desktop and mobile applications
- **Features**:
  - Native OS integration
  - Smaller bundle size than Electron
  - Rust backend, web frontend
  - System tray support
  - Auto-updates
- **Platforms**: 
  - macOS, Windows, Linux
  - iOS, Android (via Tauri Mobile)

**Use Cases**:
- Offline-first desktop app
- Native notifications
- System integration (calendars, contacts)
- Background sync
- Mobile learning app

---

## Development Tools

### Language Tools

#### **Rust Toolchain**
- **rustc**: Rust compiler
- **cargo**: Package manager and build tool
- **clippy**: Linting and code suggestions
- **rustfmt**: Code formatting
- **rust-analyzer**: LSP for IDE integration

#### **Node.js Ecosystem**
- **pnpm**: Fast, disk-efficient package manager
- **ESLint**: JavaScript/TypeScript linting
- **Prettier**: Code formatting
- **TypeScript**: Type checking

### Testing

#### **Backend Testing**
```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"
```

- **Unit Tests**: Built-in `#[cfg(test)]`
- **Integration Tests**: `tests/` directory
- **Mocking**: mockall for dependencies
- **Property Testing**: proptest for generative tests

#### **Frontend Testing**
```json
{
  "devDependencies": {
    "vitest": "^1.0.0",
    "@testing-library/react": "^14.0.0"
  }
}
```

- **Unit Tests**: Vitest (Vite-native)
- **Component Tests**: React Testing Library
- **E2E Tests**: Playwright
- **Visual Regression**: Chromatic (future)

### Development Environment

#### **VS Code Extensions**
- **rust-analyzer**: Rust language support
- **Error Lens**: Inline error display
- **GitLens**: Git integration
- **Tailwind IntelliSense**: CSS class autocomplete
- **ESLint/Prettier**: Code quality

#### **Docker Development**
- **Dev Containers**: Consistent dev environment
- **docker-compose.dev.yml**: Development orchestration
- **Hot reload**: Volume mounts for live updates

---

## Summary: Technology Decision Rationale

| Decision | Rationale |
|----------|-----------|
| **Rust Backend** | Memory safety, performance, fearless concurrency |
| **React Frontend** | Mature ecosystem, component reusability, large community |
| **Matrix Protocol** | Decentralized, federated, open standard |
| **PostgreSQL** | ACID compliance, JSON support, mature tooling |
| **Docker** | Reproducible deployments, microservice isolation |
| **NATS** | Lightweight, high-performance message queue |
| **TailwindCSS** | Rapid development, consistent design system |
| **TypeScript** | Type safety, better refactoring, self-documenting |
| **IPFS** | Decentralized storage, content addressing, future-proof |
| **Holochain (future)** | True decentralization, user data ownership |

---

## Architecture Principles

1. **Microservices**: Independent deployment and scaling
2. **API-First**: Well-defined service contracts
3. **Security by Default**: mTLS, encryption, zero trust
4. **Observability**: Comprehensive logging and metrics
5. **Developer Experience**: Fast feedback loops, type safety
6. **Progressive Enhancement**: Works offline, enhanced online
7. **Future-Proof**: Designed for eventual decentralization

---

*This technology stack balances immediate needs (performance, security, developer experience) with long-term vision (decentralization, user sovereignty) while maintaining flexibility to evolve as requirements change.*
