# Unity Platform - Frontend Application

**Framework:** React 19 + Vite 5  
**Status:** In Development (Stage 5)  
**Progress:** 15% Complete (4 of 27 tasks)

**🔗 Issue Tracking:** [Frontend Issues](http://localhost:3000/henrik/unity_platform/issues?labels=14) | [Stage 5 Issues #4-#30](http://localhost:3000/henrik/unity_platform/issues?milestone=1&labels=14)

---

## 📊 Development Status

**Current Sprint:** Setting up frontend foundation

### Completed

- ✅ [#4: Vite + React + TypeScript project setup](http://localhost:3000/henrik/unity_platform/issues/4)
- ✅ [#5: Install and configure shadcn/ui](http://localhost:3000/henrik/unity_platform/issues/5)
- ✅ [#6: Set up TailwindCSS theming](http://localhost:3000/henrik/unity_platform/issues/6)
- ✅ [#7: Install TanStack Router](http://localhost:3000/henrik/unity_platform/issues/7)

### In Progress

- 🔄 [#8: Create route structure](http://localhost:3000/henrik/unity_platform/issues/8)
- 🔄 [#9: Set up TanStack Query](http://localhost:3000/henrik/unity_platform/issues/9)
- 🔄 [#10: Create API service layer](http://localhost:3000/henrik/unity_platform/issues/10)

### Upcoming (27 total issues)

View all frontend tasks in [Forgejo](http://localhost:3000/henrik/unity_platform/issues?milestone=1&labels=14&state=open)

---

## 🎯 Project Overview

Modern, responsive frontend application for the Unity Platform built with React 19, featuring:

- **Type-Safe Routing:** TanStack Router with automatic route generation
- **Data Management:** TanStack Query for server state with caching & refetching
- **UI Components:** shadcn/ui with TailwindCSS for consistent styling
- **Form Handling:** react-hook-form + zod validation
- **State Management:** Zustand for auth/UI state only (not data fetching)
- **Testing:** Vitest (unit) + Testing Library (component) + Playwright (E2E)

---

## 🛠️ Technology Stack

| Technology | Version | Purpose |
|------------|---------|---------|
| React | 19.2.0 | Component-based UI |
| Vite | 5.x | Dev server & bundler |
| TypeScript | Latest | Type safety |
| TailwindCSS | 4.1.16 | Utility-first styling |
| shadcn/ui | 3.5.0 | Component library |
| TanStack Router | 1.134.10 | Type-safe routing |
| TanStack Query | v5 | Data fetching/caching |
| Zustand | Latest | Auth/UI state management |
| react-hook-form | Latest | Form handling |
| zod | Latest | Schema validation |
| Vitest | Latest | Unit testing |
| Testing Library | Latest | Component testing |
| Playwright | Latest | E2E testing (future) |

**Stack Rationale:** React 19 for stable ecosystem. TanStack Query handles all server data. Zustand only for auth tokens and UI state. See [Frontend Stack Rationale](../docs/architecture/frontend-stack-rationale.md).

---

## 🚀 Getting Started

### Prerequisites

```bash
# Node.js 20+ and npm
node --version  # Should be 20+
npm --version
```

### Installation

```bash
# Navigate to app directory
cd app/

# Install dependencies
npm install

# Start development server
npm run dev
```

### Development

```bash
# Start dev server (http://localhost:5173)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Run linter
npm run lint

# Run type checking
npm run type-check

# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Run tests in watch mode
npm run test:watch
```

---

## 📁 Project Structure

```
app/
├── public/              # Static assets
│   ├── fonts/           # Custom fonts
│   └── images/          # Images, icons
├── src/
│   ├── main.tsx         # Application entry point
│   ├── router.tsx       # TanStack Router configuration
│   ├── routeTree.gen.ts # Auto-generated route tree
│   ├── App.tsx          # Root component
│   ├── index.css        # Global styles
│   ├── api/             # API service layer (TanStack Query)
│   │   ├── client.ts    # HTTP client config
│   │   ├── auth.ts      # Auth endpoints
│   │   ├── users.ts     # User endpoints
│   │   └── ...
│   ├── components/      # Reusable components
│   │   ├── ui/          # shadcn/ui components
│   │   ├── layout/      # Layout components
│   │   ├── forms/       # Form components
│   │   └── ...
│   ├── pages/           # Page components
│   │   ├── HomePage.tsx
│   │   ├── LoginPage.tsx
│   │   ├── ProfilePage.tsx
│   │   └── ...
│   ├── routes/          # TanStack Router routes
│   │   ├── index.tsx    # Home route
│   │   ├── login.tsx    # Login route
│   │   └── ...
│   ├── stores/          # Zustand stores
│   │   ├── authStore.ts # Auth state
│   │   └── uiStore.ts   # UI state
│   ├── hooks/           # Custom React hooks
│   │   ├── useAuth.ts
│   │   └── ...
│   ├── lib/             # Utilities
│   │   ├── utils.ts
│   │   └── ...
│   ├── types/           # TypeScript types
│   │   ├── api.ts
│   │   └── ...
│   ├── config/          # Configuration
│   │   └── env.ts
│   └── test/            # Test utilities
│       ├── setup.ts
│       └── utils.tsx
├── package.json
├── tsconfig.json        # TypeScript config
├── vite.config.ts       # Vite config
├── vitest.config.ts     # Vitest config
├── tailwind.config.js   # TailwindCSS config
└── components.json      # shadcn/ui config
```

---

## 🧩 Key Features

### Authentication

- Login/Register forms with validation
- JWT token management (access + refresh)
- Protected routes with auth guards
- Persistent auth state (localStorage)

### User Profiles

- View/edit profile (bio, avatar, location)
- Social links management
- Privacy settings
- Language preferences

### Responsive Design

- Mobile-first approach
- Tablet & desktop optimized
- Touch-friendly interactions
- Accessible (ARIA, keyboard nav)

### Developer Experience

- Hot Module Replacement (HMR)
- TypeScript autocomplete
- ESLint + Prettier
- Component testing
- API mocking for tests

---

## 🧪 Testing

### Unit Tests (Vitest)

```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Watch mode
npm run test:watch

# Run specific test file
npm test -- src/components/Button.test.tsx
```

### Component Tests (Testing Library)

```bash
# Test user interactions
npm test -- src/pages/LoginPage.test.tsx
```

### E2E Tests (Playwright) - Future

```bash
# Run E2E tests
npm run test:e2e

# Run E2E tests in UI mode
npm run test:e2e:ui
```

---

## 🎨 Styling

### TailwindCSS

Utility-first CSS framework for rapid development:

```tsx
<button className="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded-md">
  Click me
</button>
```

### shadcn/ui Components

Pre-built accessible components:

```tsx
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'

<Card>
  <Button variant="default">Submit</Button>
</Card>
```

### Theme Customization

Edit `src/index.css` for theme variables:

```css
:root {
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  --primary: 221.2 83.2% 53.3%;
  /* ... */
}
```

---

## 🔌 API Integration

### TanStack Query

All server data fetching uses TanStack Query:

```tsx
import { useQuery, useMutation } from '@tanstack/react-query'
import { getProfile, updateProfile } from '@/api/users'

function ProfilePage() {
  // Fetch data
  const { data, isLoading, error } = useQuery({
    queryKey: ['profile', 'me'],
    queryFn: getProfile
  })

  // Mutate data
  const mutation = useMutation({
    mutationFn: updateProfile,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['profile', 'me'] })
    }
  })

  return <div>{data?.bio}</div>
}
```

### API Service Layer

API functions in `src/api/`:

```typescript
// src/api/users.ts
export async function getProfile() {
  const res = await apiClient.get('/api/v1/users/profiles/me')
  return res.data
}

export async function updateProfile(data: ProfileUpdate) {
  const res = await apiClient.patch('/api/v1/users/profiles/me', data)
  return res.data
}
```

---

## 🔐 Environment Variables

Create `.env.local` file:

```bash
# API Base URL
VITE_API_BASE_URL=http://localhost:8000

# Environment
VITE_ENV=development

# Feature Flags
VITE_ENABLE_ANALYTICS=false
```

Access in code:

```typescript
const apiUrl = import.meta.env.VITE_API_BASE_URL
```

---

## 📚 Documentation

- **Component Library:** [shadcn/ui](https://ui.shadcn.com/)
- **Routing:** [TanStack Router Docs](https://tanstack.com/router/latest)
- **Data Fetching:** [TanStack Query Docs](https://tanstack.com/query/latest)
- **Testing:** [Vitest Docs](https://vitest.dev/) | [Testing Library](https://testing-library.com/)
- **Styling:** [TailwindCSS Docs](https://tailwindcss.com/docs)

**Internal Docs:**

- [Frontend Architecture](../docs/architecture/frontend/)
- [Component Guidelines](../docs/guides/frontend/component-guidelines.md)
- [Testing Guide](../docs/guides/frontend/testing.md)
- [Forgejo Workflow](../docs/guides/development/forgejo-workflow.md)

---

## 🐛 Troubleshooting

### Common Issues

**Port already in use:**

```bash
# Kill process on port 5173
lsof -ti:5173 | xargs kill -9

# Or use different port
npm run dev -- --port 5174
```

**Module not found:**

```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
```

**Type errors after update:**

```bash
# Regenerate route tree
npm run build

# Clear TypeScript cache
rm -rf node_modules/.vite
```

---

## 🤝 Contributing

1. **Pick an issue:** Browse [frontend issues](http://localhost:3000/henrik/unity_platform/issues?labels=14&state=open)
2. **Create branch:** `git checkout -b issue-XX-description`
3. **Implement:** Follow component patterns
4. **Test:** Write tests, achieve >80% coverage
5. **Document:** Add JSDoc comments
6. **Submit PR:** Link to issue, request review

See [Forgejo Workflow Guide](../docs/guides/development/forgejo-workflow.md) for details.

---

## 📊 Progress

**Stage 5 Issues:** [View All](http://localhost:3000/henrik/unity_platform/issues?milestone=1&labels=14)

- Authentication UI: Issues #8-#14
- Profile Pages: Issues #15-#19
- Settings Pages: Issues #20-#24
- Testing & Polish: Issues #25-#30

---

## 🔗 Related

- [Backend Services README](../services/README.md)
- [Phase 1 Status](../docs/status/current/phase-1-status.md)
- [All Forgejo Issues](http://localhost:3000/henrik/unity_platform/issues)
