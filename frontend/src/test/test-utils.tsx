import type { ReactElement, ReactNode } from 'react';
import { render, type RenderOptions } from '@testing-library/react';
import { createMemoryHistory, createRouter, RouterProvider } from '@tanstack/react-router';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { routeTree } from '@/routeTree.gen';
import { useAuthStore } from '@/stores/authStore';

// Create test query client
export const createTestQueryClient = () =>
    new QueryClient({
        defaultOptions: {
            queries: {
                retry: false,
                gcTime: 0,
                staleTime: 0,
            },
            mutations: {
                retry: false,
            },
        },
    });

// Create test router
export const createTestRouter = (initialPath = '/') => {
    const memoryHistory = createMemoryHistory({
        initialEntries: [initialPath],
    });

    return createRouter({
        routeTree,
        history: memoryHistory,
        context: undefined!,
    });
};

interface WrapperProps {
    children: ReactNode;
    initialPath?: string;
}

// Custom render with all providers
export function renderWithProviders(
    ui: ReactElement,
    options?: Omit<RenderOptions, 'wrapper'>
) {
    const testQueryClient = createTestQueryClient();

    function Wrapper({ children }: WrapperProps) {
        return (
            <QueryClientProvider client={testQueryClient}>
                {children}
            </QueryClientProvider>
        );
    }

    return {
        ...render(ui, { wrapper: Wrapper, ...options }),
        queryClient: testQueryClient,
    };
}

// Custom render with router (for components that use useRouter)
export function renderWithRouter(
    ui: ReactElement,
    options?: { initialPath?: string } & Omit<RenderOptions, 'wrapper'>
) {
    const { initialPath = '/', ...renderOptions } = options || {};
    const testQueryClient = createTestQueryClient();
    const testRouter = createTestRouter(initialPath);

    function Wrapper({ children }: WrapperProps) {
        return (
            <QueryClientProvider client={testQueryClient}>
                {
                    // RouterProvider's types don't accept children in this lib version for tests.
                    // Use a type-ignore here so we can mount children under the router context.
                }
                {/* @ts-expect-error - allow children under RouterProvider for test rendering */}
                <RouterProvider router={testRouter}>
                    {children}
                </RouterProvider>
            </QueryClientProvider>
        );
    }

    return {
        ...render(ui, { wrapper: Wrapper, ...renderOptions }),
        queryClient: testQueryClient,
        router: testRouter,
    };
}

// Mock user for testing
export const mockUser = {
    id: '0e8fefa6-3570-482c-b89d-c2ca8c96c873',
    username: 'testuser',
    email: 'test@unityplatform.dk',
    full_name: 'Test User',
    territory_code: 'dk',
    is_active: true,
    created_at: '2025-11-09T12:00:00Z',
};

// Mock auth tokens
export const mockTokens = {
    access_token: 'mock-access-token',
    refresh_token: 'mock-refresh-token',
    token_type: 'Bearer',
    expires_in: 900,
};

// Helper to mock authenticated state
export function mockAuthState() {
    // Mock localStorage
    const authState = {
        state: {
            user: mockUser,
            accessToken: mockTokens.access_token,
            refreshToken: mockTokens.refresh_token,
            isAuthenticated: true,
            isLoading: false,
            error: null,
        },
        version: 0,
    };

    localStorage.setItem('auth-storage', JSON.stringify(authState));

    // Also directly set the zustand auth store to ensure synchronous test state
    try {
        const store = useAuthStore.getState();
        if (store.setUser) {
            const setter = store.setUser as (u: typeof mockUser) => void;
            setter(mockUser);
        }
        if (store.setTokens) {
            const tokenSetter = store.setTokens as (a: string, r: string) => void;
            tokenSetter(mockTokens.access_token, mockTokens.refresh_token);
        }
    } catch (e) {
        // If store isn't ready in this environment, ignore - localStorage still helps
        // but prefer direct store update for deterministic test behavior
        console.warn('mockAuthState: unable to set zustand store directly', e);
    }
}

// Helper to clear auth state
export function clearAuthState() {
    localStorage.removeItem('auth-storage');
}

// Re-export common testing utilities
export { screen, waitFor, within } from '@testing-library/react';
export { default as userEvent } from '@testing-library/user-event';
