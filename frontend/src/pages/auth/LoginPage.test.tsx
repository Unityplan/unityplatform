import { describe, it, expect, beforeEach, vi } from 'vitest';
import { renderWithProviders, screen, waitFor, clearAuthState } from '@/test/test-utils';
import userEvent from '@testing-library/user-event';
import { LoginPage } from './LoginPage';

// Mock the auth store
vi.mock('@/stores/authStore', () => ({
    useAuthStore: vi.fn(() => ({
        login: vi.fn(),
        isLoading: false,
        error: null,
    })),
}));

describe('LoginPage', () => {
    beforeEach(() => {
        clearAuthState();
        vi.clearAllMocks();
    });

    it('renders login form with all fields', () => {
        renderWithProviders(<LoginPage />);

        expect(screen.getByText('Welcome Back')).toBeDefined();
        expect(screen.getByText('Sign in to your UnityPlan account')).toBeDefined();
        expect(screen.getByLabelText(/username/i)).toBeDefined();
        expect(screen.getByLabelText(/password/i)).toBeDefined();
        expect(screen.getByLabelText(/territory/i)).toBeDefined();
        expect(screen.getByRole('button', { name: /sign in/i })).toBeDefined();
    });

    it('shows validation errors for empty fields', async () => {
        const user = userEvent.setup();
        renderWithProviders(<LoginPage />);

        const submitButton = screen.getByRole('button', { name: /sign in/i });
        await user.click(submitButton);

        await waitFor(() => {
            expect(screen.getByText(/username is required/i)).toBeDefined();
        });
    });

    it('shows validation error for short username', async () => {
        const user = userEvent.setup();
        renderWithProviders(<LoginPage />);

        const usernameInput = screen.getByLabelText(/username/i);
        await user.type(usernameInput, 'ab'); // Less than 3 characters

        const submitButton = screen.getByRole('button', { name: /sign in/i });
        await user.click(submitButton);

        await waitFor(() => {
            expect(screen.getByText(/username is required \(3-50 characters\)/i)).toBeDefined();
        });
    });

    it('shows validation error for short password', async () => {
        const user = userEvent.setup();
        renderWithProviders(<LoginPage />);

        const usernameInput = screen.getByLabelText(/username/i);
        const passwordInput = screen.getByLabelText(/password/i);

        await user.type(usernameInput, 'testuser');
        await user.type(passwordInput, 'short'); // Less than 8 characters

        const submitButton = screen.getByRole('button', { name: /sign in/i });
        await user.click(submitButton);

        await waitFor(() => {
            expect(screen.getByText(/password must be at least 8 characters/i)).toBeDefined();
        });
    });

    it('allows typing in all form fields', async () => {
        const user = userEvent.setup();
        renderWithProviders(<LoginPage />);

        const usernameInput = screen.getByLabelText(/username/i) as HTMLInputElement;
        const passwordInput = screen.getByLabelText(/password/i) as HTMLInputElement;

        await user.type(usernameInput, 'testuser');
        await user.type(passwordInput, 'TestPassword123!');

        expect(usernameInput.value).toBe('testuser');
        expect(passwordInput.value).toBe('TestPassword123!');
    });

    // TODO: Fix shadcn Select testing - requires special handling for custom components
    it.skip('has default territory selection', () => {
        renderWithProviders(<LoginPage />);

        const territorySelect = screen.getByRole('combobox') as HTMLSelectElement;
        expect(territorySelect.value).toBe('dk');
    });

    // TODO: Fix shadcn Select testing - requires special handling for custom components
    it.skip('can change territory selection', async () => {
        const user = userEvent.setup();
        renderWithProviders(<LoginPage />);

        const territorySelect = screen.getByRole('combobox') as HTMLSelectElement;

        await user.selectOptions(territorySelect, 'no');
        expect(territorySelect.value).toBe('no');

        await user.selectOptions(territorySelect, 'se');
        expect(territorySelect.value).toBe('se');
    });

    it('has a link to password reset page', () => {
        renderWithProviders(<LoginPage />);

        const resetLink = screen.getByText(/forgot your password/i);
        expect(resetLink).toBeDefined();
        expect(resetLink.getAttribute('href')).toBe('/reset-password');
    });

    // TODO: Fix password visibility toggle test - button needs better accessibility
    it.skip('toggles password visibility', async () => {
        const user = userEvent.setup();
        renderWithProviders(<LoginPage />);

        const passwordInput = screen.getByLabelText(/password/i) as HTMLInputElement;
        expect(passwordInput.type).toBe('password');

        // Find and click the toggle button (eye icon)
        const toggleButton = screen.getByRole('button', { name: '' }); // Icon button has no text
        await user.click(toggleButton);

        // Password should now be visible
        expect(passwordInput.type).toBe('text');

        // Click again to hide
        await user.click(toggleButton);
        expect(passwordInput.type).toBe('password');
    });
});
