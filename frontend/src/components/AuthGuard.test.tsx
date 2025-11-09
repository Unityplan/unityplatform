import { describe, it, expect, beforeEach } from 'vitest';
import { renderWithProviders, screen, mockAuthState, clearAuthState } from '@/test/test-utils';
import { AuthGuard } from './AuthGuard';

describe('AuthGuard', () => {
  beforeEach(() => {
    clearAuthState();
  });

  it('renders children when user is authenticated', () => {
    mockAuthState();
    
    renderWithProviders(
      <AuthGuard>
        <div>Protected Content</div>
      </AuthGuard>
    );

    expect(screen.getByText('Protected Content')).toBeInTheDocument();
  });

  it('returns null when user is not authenticated', () => {
    const { container } = renderWithProviders(
      <AuthGuard>
        <div>Protected Content</div>
      </AuthGuard>
    );

    expect(container.firstChild).toBeNull();
  });

  it('does not render children when user is null', () => {
    // Set auth state with null user
    localStorage.setItem(
      'auth-storage',
      JSON.stringify({
        state: {
          user: null,
          accessToken: 'token',
          refreshToken: 'refresh',
          isAuthenticated: false,
          isLoading: false,
          error: null,
        },
        version: 0,
      })
    );

    const { container } = renderWithProviders(
      <AuthGuard>
        <div>Protected Content</div>
      </AuthGuard>
    );

    expect(container.firstChild).toBeNull();
  });
});
