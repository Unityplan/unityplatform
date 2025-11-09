# Frontend Testing Guide

Comprehensive guide for testing the UnityPlan React frontend application.

**Testing Stack:** Vitest, Testing Library, Playwright  
**Coverage Target:** >80%  
**Last Updated:** November 9, 2025

---

## 📋 Table of Contents

- [Testing Philosophy](#testing-philosophy)
- [Testing Stack](#testing-stack)
- [Setup](#setup)
- [Unit Testing](#unit-testing)
- [Component Testing](#component-testing)
- [Integration Testing](#integration-testing)
- [E2E Testing](#e2e-testing)
- [Best Practices](#best-practices)
- [Common Patterns](#common-patterns)
- [Mocking](#mocking)
- [Coverage](#coverage)
- [CI/CD Integration](#cicd-integration)

---

## Testing Philosophy

### Testing Pyramid

```
     /\
    /E2E\         ← Few, critical user journeys (5-10%)
   /------\
  /  INT   \      ← API + component integration (20-30%)
 /----------\
/   UNIT     \    ← Pure functions, hooks, utils (60-70%)
--------------
```

**Principles:**

1. **Write tests that give confidence** - Test behavior, not implementation
2. **Keep tests simple** - Tests should be easier to understand than the code
3. **Test user interactions** - Click buttons, type text, navigate
4. **Avoid testing implementation details** - Don't test internal state
5. **Fast feedback loop** - Unit tests run in <1s, integration in <5s

---

## Testing Stack

| Tool | Purpose | When to Use |
|------|---------|-------------|
| **Vitest** | Test runner | All tests (fast, Vite-native) |
| **Testing Library** | Component testing | React components, user interactions |
| **MSW** | API mocking | Mock backend responses |
| **Playwright** | E2E testing | Critical user flows |
| **@testing-library/user-event** | User simulation | Click, type, navigate |
| **@testing-library/jest-dom** | DOM assertions | `.toBeInTheDocument()`, etc. |

---

## Setup

### Install Dependencies

```bash
# Core testing
npm install -D vitest @vitest/ui jsdom

# Testing Library
npm install -D @testing-library/react @testing-library/jest-dom @testing-library/user-event

# API mocking
npm install -D msw

# E2E testing
npm install -D @playwright/test
```

### Configuration

**vitest.config.ts:**

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
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'lcov'],
      exclude: [
        'node_modules/',
        'src/test/',
        '**/*.d.ts',
        '**/*.config.*',
        '**/mockData',
      ],
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
```

**src/test/setup.ts:**

```typescript
import { expect, afterEach } from 'vitest';
import { cleanup } from '@testing-library/react';
import * as matchers from '@testing-library/jest-dom/matchers';

// Extend Vitest matchers
expect.extend(matchers);

// Cleanup after each test
afterEach(() => {
  cleanup();
});
```

**playwright.config.ts:**

```typescript
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
```

---

## Unit Testing

### Pure Functions

**src/lib/utils.ts:**

```typescript
export function formatDate(date: Date): string {
  return new Intl.DateTimeFormat('en-US').format(date);
}

export function validateEmail(email: string): boolean {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
}
```

**src/lib/utils.test.ts:**

```typescript
import { describe, it, expect } from 'vitest';
import { formatDate, validateEmail } from './utils';

describe('formatDate', () => {
  it('formats date correctly', () => {
    const date = new Date('2025-01-15');
    expect(formatDate(date)).toBe('1/15/2025');
  });
});

describe('validateEmail', () => {
  it('returns true for valid email', () => {
    expect(validateEmail('user@example.com')).toBe(true);
  });

  it('returns false for invalid email', () => {
    expect(validateEmail('invalid')).toBe(false);
    expect(validateEmail('missing@domain')).toBe(false);
    expect(validateEmail('@example.com')).toBe(false);
  });
});
```

### Custom Hooks

**src/hooks/useDebounce.test.ts:**

```typescript
import { renderHook, waitFor } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { useDebounce } from './useDebounce';

describe('useDebounce', () => {
  it('debounces value changes', async () => {
    const { result, rerender } = renderHook(
      ({ value, delay }) => useDebounce(value, delay),
      { initialProps: { value: 'initial', delay: 500 } }
    );

    expect(result.current).toBe('initial');

    // Change value
    rerender({ value: 'updated', delay: 500 });
    
    // Should still be initial
    expect(result.current).toBe('initial');

    // Wait for debounce
    await waitFor(() => {
      expect(result.current).toBe('updated');
    }, { timeout: 600 });
  });
});
```

---

## Component Testing

### Basic Component Test

**src/components/UserCard.test.tsx:**

```typescript
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { UserCard } from './UserCard';

describe('UserCard', () => {
  const mockUser = {
    id: '1',
    username: 'testuser',
    email: 'test@example.com',
    avatar_url: null,
  };

  it('renders user information', () => {
    render(<UserCard user={mockUser} />);
    
    expect(screen.getByText('testuser')).toBeInTheDocument();
    expect(screen.getByText('test@example.com')).toBeInTheDocument();
  });

  it('displays avatar when provided', () => {
    const userWithAvatar = { ...mockUser, avatar_url: '/avatar.jpg' };
    render(<UserCard user={userWithAvatar} />);
    
    const avatar = screen.getByRole('img');
    expect(avatar).toHaveAttribute('src', '/avatar.jpg');
  });

  it('displays initials when no avatar', () => {
    render(<UserCard user={mockUser} />);
    
    expect(screen.getByText('T')).toBeInTheDocument(); // First letter
  });
});
```

### Testing User Interactions

**src/components/LoginForm.test.tsx:**

```typescript
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi } from 'vitest';
import { LoginForm } from './LoginForm';

describe('LoginForm', () => {
  it('handles form submission', async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    
    render(<LoginForm onSubmit={onSubmit} />);
    
    // Type in inputs
    await user.type(screen.getByLabelText(/email/i), 'test@example.com');
    await user.type(screen.getByLabelText(/password/i), 'password123');
    
    // Submit form
    await user.click(screen.getByRole('button', { name: /sign in/i }));
    
    // Verify submission
    expect(onSubmit).toHaveBeenCalledWith({
      email: 'test@example.com',
      password: 'password123',
    });
  });

  it('displays validation errors', async () => {
    const user = userEvent.setup();
    render(<LoginForm onSubmit={vi.fn()} />);
    
    // Submit without filling
    await user.click(screen.getByRole('button', { name: /sign in/i }));
    
    // Check for error messages
    expect(screen.getByText(/email is required/i)).toBeInTheDocument();
    expect(screen.getByText(/password is required/i)).toBeInTheDocument();
  });

  it('disables submit button while loading', () => {
    render(<LoginForm onSubmit={vi.fn()} isLoading={true} />);
    
    const button = screen.getByRole('button', { name: /signing in/i });
    expect(button).toBeDisabled();
  });
});
```

### Testing with Providers

**src/test/test-utils.tsx:**

```typescript
import { ReactElement } from 'react';
import { render, RenderOptions } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Create test query client
const createTestQueryClient = () => new QueryClient({
  defaultOptions: {
    queries: { retry: false },
    mutations: { retry: false },
  },
});

// Custom render with providers
export function renderWithProviders(
  ui: ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>
) {
  const testQueryClient = createTestQueryClient();
  
  function Wrapper({ children }: { children: React.ReactNode }) {
    return (
      <QueryClientProvider client={testQueryClient}>
        {children}
      </QueryClientProvider>
    );
  }
  
  return render(ui, { wrapper: Wrapper, ...options });
}

// Re-export everything
export * from '@testing-library/react';
```

**Usage:**

```typescript
import { renderWithProviders, screen } from '@/test/test-utils';
import { UserProfile } from './UserProfile';

describe('UserProfile', () => {
  it('renders with providers', () => {
    renderWithProviders(<UserProfile userId="1" />);
    // ... assertions
  });
});
```

---

## Integration Testing

### Testing with API Calls

**src/test/mocks/handlers.ts:**

```typescript
import { http, HttpResponse } from 'msw';

export const handlers = [
  // Login
  http.post('/api/auth/login', async ({ request }) => {
    const { email, password } = await request.json();
    
    if (email === 'test@example.com' && password === 'password123') {
      return HttpResponse.json({
        access_token: 'mock-access-token',
        refresh_token: 'mock-refresh-token',
        user: {
          id: '1',
          email: 'test@example.com',
          username: 'testuser',
        },
      });
    }
    
    return HttpResponse.json(
      { error: 'Invalid credentials' },
      { status: 401 }
    );
  }),

  // Get user
  http.get('/api/users/:id', ({ params }) => {
    return HttpResponse.json({
      id: params.id,
      email: 'test@example.com',
      username: 'testuser',
      avatar_url: null,
    });
  }),
];
```

**src/test/mocks/server.ts:**

```typescript
import { setupServer } from 'msw/node';
import { handlers } from './handlers';

export const server = setupServer(...handlers);
```

**src/test/setup.ts:**

```typescript
import { beforeAll, afterEach, afterAll } from 'vitest';
import { server } from './mocks/server';

// Start server before tests
beforeAll(() => server.listen());

// Reset handlers after each test
afterEach(() => server.resetHandlers());

// Clean up after all tests
afterAll(() => server.close());
```

**src/pages/LoginPage.test.tsx:**

```typescript
import { renderWithProviders, screen, waitFor } from '@/test/test-utils';
import userEvent from '@testing-library/user-event';
import { describe, it, expect } from 'vitest';
import { LoginPage } from './LoginPage';

describe('LoginPage Integration', () => {
  it('logs in successfully with valid credentials', async () => {
    const user = userEvent.setup();
    renderWithProviders(<LoginPage />);
    
    // Fill form
    await user.type(screen.getByLabelText(/email/i), 'test@example.com');
    await user.type(screen.getByLabelText(/password/i), 'password123');
    
    // Submit
    await user.click(screen.getByRole('button', { name: /sign in/i }));
    
    // Wait for success
    await waitFor(() => {
      expect(screen.getByText(/welcome/i)).toBeInTheDocument();
    });
  });

  it('shows error with invalid credentials', async () => {
    const user = userEvent.setup();
    renderWithProviders(<LoginPage />);
    
    // Fill with invalid credentials
    await user.type(screen.getByLabelText(/email/i), 'wrong@example.com');
    await user.type(screen.getByLabelText(/password/i), 'wrongpassword');
    
    // Submit
    await user.click(screen.getByRole('button', { name: /sign in/i }));
    
    // Wait for error
    await waitFor(() => {
      expect(screen.getByText(/invalid credentials/i)).toBeInTheDocument();
    });
  });
});
```

---

## E2E Testing

### Playwright Tests

**tests/e2e/auth.spec.ts:**

```typescript
import { test, expect } from '@playwright/test';

test.describe('Authentication', () => {
  test('user can login successfully', async ({ page }) => {
    await page.goto('/login');
    
    // Fill login form
    await page.getByLabel(/email/i).fill('test@example.com');
    await page.getByLabel(/password/i).fill('password123');
    
    // Submit
    await page.getByRole('button', { name: /sign in/i }).click();
    
    // Verify redirect to dashboard
    await expect(page).toHaveURL('/dashboard');
    expect(page.getByText(/welcome/i)).toBeVisible();
  });

  test('user can logout', async ({ page }) => {
    // Login first
    await page.goto('/login');
    await page.getByLabel(/email/i).fill('test@example.com');
    await page.getByLabel(/password/i).fill('password123');
    await page.getByRole('button', { name: /sign in/i }).click();
    
    // Wait for dashboard
    await page.waitForURL('/dashboard');
    
    // Logout
    await page.getByRole('button', { name: /logout/i }).click();
    
    // Verify redirect to login
    await expect(page).toHaveURL('/login');
  });
});
```

**tests/e2e/profile.spec.ts:**

```typescript
import { test, expect } from '@playwright/test';

test.describe('User Profile', () => {
  test.beforeEach(async ({ page }) => {
    // Login before each test
    await page.goto('/login');
    await page.getByLabel(/email/i).fill('test@example.com');
    await page.getByLabel(/password/i).fill('password123');
    await page.getByRole('button', { name: /sign in/i }).click();
    await page.waitForURL('/dashboard');
  });

  test('user can view profile', async ({ page }) => {
    await page.goto('/profile');
    
    expect(page.getByText('testuser')).toBeVisible();
    expect(page.getByText('test@example.com')).toBeVisible();
  });

  test('user can edit profile', async ({ page }) => {
    await page.goto('/profile/edit');
    
    // Update username
    await page.getByLabel(/username/i).fill('newusername');
    
    // Save
    await page.getByRole('button', { name: /save/i }).click();
    
    // Verify success
    await expect(page.getByText(/profile updated/i)).toBeVisible();
  });
});
```

---

## Best Practices

### 1. Test Behavior, Not Implementation

```typescript
// ❌ Bad: Testing implementation details
it('sets loading state to true', () => {
  const { result } = renderHook(() => useUser('1'));
  expect(result.current.isLoading).toBe(true);
});

// ✅ Good: Testing user-visible behavior
it('shows loading spinner while fetching', () => {
  render(<UserProfile userId="1" />);
  expect(screen.getByRole('progressbar')).toBeInTheDocument();
});
```

### 2. Use Accessible Queries

Priority order:

1. `getByRole` - Most accessible
2. `getByLabelText` - Form elements
3. `getByPlaceholderText` - Input hints
4. `getByText` - Non-interactive content
5. `getByTestId` - Last resort

```typescript
// ✅ Good: Accessible queries
screen.getByRole('button', { name: /submit/i });
screen.getByLabelText(/email/i);
screen.getByText(/welcome/i);

// ❌ Bad: Non-semantic queries
screen.getByClassName('submit-button');
screen.getByTestId('email-input');
```

### 3. Keep Tests Independent

```typescript
// ❌ Bad: Tests depend on each other
describe('User flow', () => {
  let userId: string;
  
  it('creates user', () => {
    userId = createUser();
  });
  
  it('updates user', () => {
    updateUser(userId); // Depends on previous test!
  });
});

// ✅ Good: Independent tests
describe('User flow', () => {
  it('creates user', () => {
    const userId = createUser();
    expect(userId).toBeDefined();
  });
  
  it('updates user', () => {
    const userId = createUser(); // Each test sets up its own data
    updateUser(userId);
  });
});
```

### 4. Use Descriptive Test Names

```typescript
// ❌ Bad: Vague names
it('works', () => { /* ... */ });
it('test 1', () => { /* ... */ });

// ✅ Good: Descriptive names
it('displays error when email is invalid', () => { /* ... */ });
it('disables submit button while form is submitting', () => { /* ... */ });
```

---

## Common Patterns

### Testing Async Data Fetching

```typescript
it('loads and displays user data', async () => {
  renderWithProviders(<UserProfile userId="1" />);
  
  // Loading state
  expect(screen.getByRole('progressbar')).toBeInTheDocument();
  
  // Wait for data
  await waitFor(() => {
    expect(screen.getByText('testuser')).toBeInTheDocument();
  });
  
  // Loading indicator gone
  expect(screen.queryByRole('progressbar')).not.toBeInTheDocument();
});
```

### Testing Error States

```typescript
it('displays error message when fetch fails', async () => {
  // Override handler to return error
  server.use(
    http.get('/api/users/:id', () => {
      return HttpResponse.json(
        { error: 'User not found' },
        { status: 404 }
      );
    })
  );
  
  renderWithProviders(<UserProfile userId="999" />);
  
  await waitFor(() => {
    expect(screen.getByText(/user not found/i)).toBeInTheDocument();
  });
});
```

### Testing Form Validation

```typescript
it('validates required fields', async () => {
  const user = userEvent.setup();
  render(<RegisterForm onSubmit={vi.fn()} />);
  
  // Submit empty form
  await user.click(screen.getByRole('button', { name: /register/i }));
  
  // Check all validation errors
  expect(screen.getByText(/username is required/i)).toBeInTheDocument();
  expect(screen.getByText(/email is required/i)).toBeInTheDocument();
  expect(screen.getByText(/password is required/i)).toBeInTheDocument();
});
```

---

## Mocking

### Mock API Responses

See [Integration Testing](#integration-testing) section for MSW setup.

### Mock Zustand Store

```typescript
import { vi } from 'vitest';
import * as authStore from '@/stores/authStore';

vi.mock('@/stores/authStore', () => ({
  useAuthStore: vi.fn(() => ({
    accessToken: 'mock-token',
    user: { id: '1', email: 'test@example.com' },
    logout: vi.fn(),
  })),
}));
```

### Mock TanStack Query

```typescript
import { vi } from 'vitest';
import * as useUser from '@/lib/queries/useUser';

vi.mock('@/lib/queries/useUser', () => ({
  useUser: vi.fn(() => ({
    data: { id: '1', username: 'testuser' },
    isLoading: false,
    error: null,
  })),
}));
```

---

## Coverage

### Run Coverage

```bash
npm run test:coverage
```

### Coverage Targets

| Metric | Target |
|--------|--------|
| Statements | >80% |
| Branches | >75% |
| Functions | >80% |
| Lines | >80% |

### Exclude from Coverage

**vitest.config.ts:**

```typescript
coverage: {
  exclude: [
    'src/test/',
    '**/*.config.*',
    '**/*.d.ts',
    '**/mockData/',
    'src/main.tsx',
  ],
}
```

---

## CI/CD Integration

### GitHub Actions

**.github/workflows/frontend-test.yml:**

```yaml
name: Frontend Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
          cache-dependency-path: frontend/package-lock.json
      
      - name: Install dependencies
        working-directory: frontend
        run: npm ci
      
      - name: Run tests
        working-directory: frontend
        run: npm run test:coverage
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./frontend/coverage/lcov.info
```

---

## Next Steps

- **Read:** [Component Patterns](./component-patterns.md) for reusable patterns
- **Read:** [State Management](./state-management.md) for data flow
- **Implement:** Add tests for new features as you build them

**Test-Driven Development (TDD) workflow:**

1. Write failing test
2. Implement minimum code to pass
3. Refactor
4. Repeat
