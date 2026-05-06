import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import type { AuthStore, LoginRequest, RegisterRequest, User } from '@/types/auth';
import * as authApi from '@/api/auth';
import { getFullProfile } from '@/api/users';
import { parseJwt } from '@/lib/utils';

/**
 * Extract error message from API error
 */
function getErrorMessage(error: unknown, defaultMessage: string): string {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === 'object' && error !== null && 'response' in error) {
    const response = (error as { response?: { data?: { detail?: string; error?: string } } }).response;
    return response?.data?.error || response?.data?.detail || defaultMessage;
  }
  return defaultMessage;
}

/**
 * Authentication store using Zustand with localStorage persistence
 * 
 * Stores:
 * - Access token (JWT for API authentication)
 * - Refresh token (for obtaining new access tokens)
 * - User information
 * - Authentication state
 * 
 * Persisted in localStorage to survive page refreshes
 */
export const useAuthStore = create<AuthStore>()(
  persist(
    (set, get) => ({
      // State
      user: null,
      accessToken: null,
      refreshToken: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,
      isLocked: false, // Session lock state

      // Actions
      login: async (credentials: LoginRequest) => {
        set({ isLoading: true, error: null });
        try {
          // Step 1: Login and get tokens from auth-service
          const response = await authApi.login(credentials);
          set({
            accessToken: response.accessToken,
            refreshToken: response.refreshToken,
            isAuthenticated: true,
            error: null,
          });

          // Step 2: Load user profile from user-service
          // loadUser() handles its own loading state
          await get().loadUser();
        } catch (error: unknown) {
          const errorMessage = getErrorMessage(error, 'Login failed');
          set({ error: errorMessage, isLoading: false });
          throw error;
        }
      },

      register: async (data: RegisterRequest) => {
        set({ isLoading: true, error: null });
        try {
          // Step 1: Register and get tokens from auth-service
          const response = await authApi.register(data);
          set({
            accessToken: response.accessToken,
            refreshToken: response.refreshToken,
            isAuthenticated: true,
            error: null,
          });

          // Step 2: Load user profile from user-service
          // loadUser() handles its own loading state
          await get().loadUser();
        } catch (error: unknown) {
          const errorMessage = getErrorMessage(error, 'Registration failed');
          set({ error: errorMessage, isLoading: false });
          throw error;
        }
      },

      logout: async () => {
        const { refreshToken } = get();
        set({ isLoading: true, error: null });
        try {
          if (refreshToken) {
            await authApi.logout(refreshToken);
          }
        } catch (error: unknown) {
          console.error('Logout error:', error);
        } finally {
          // Clear authentication state regardless of API call success
          get().clearAuth();
        }
      },

      refreshAccessToken: async () => {
        const { refreshToken } = get();
        if (!refreshToken) {
          get().clearAuth();
          throw new Error('No refresh token available');
        }

        set({ isLoading: true, error: null });
        try {
          const response = await authApi.refreshToken(refreshToken);
          set({
            accessToken: response.accessToken,
            refreshToken: response.refreshToken,
            isAuthenticated: true,
            isLoading: false,
            error: null,
          });
        } catch (error: unknown) {
          // Only clear auth if it's a 401/403 error (invalid token)
          // or if it's a specific "invalid_grant" error
          const isAuthError =
            (typeof error === 'object' && error !== null && 'response' in error &&
              ((error as any).response?.status === 401 || (error as any).response?.status === 403)) ||
            getErrorMessage(error, '').includes('invalid_grant');

          if (isAuthError) {
            get().clearAuth();
          }

          const errorMessage = getErrorMessage(error, 'Token refresh failed');
          set({ error: errorMessage, isLoading: false });
          throw error;
        }
      },

      loadUser: async () => {
        const { accessToken } = get();
        if (!accessToken) {
          return;
        }

        set({ isLoading: true, error: null });
        try {
          // Parse JWT to extract claims (badges, territory)
          const claims = parseJwt(accessToken);
          const badges = claims?.badges ?? [];
          const territory = claims?.territory ?? 'dk';

          // Fetch user profile from user-service
          const profile = await getFullProfile();

          // Map UserProfile to User type for auth store
          const user: User = {
            id: profile.id,
            username: profile.username,
            email: profile.email || '',
            fullName: profile.fullName || null,
            territory,
            isActive: true,
            createdAt: profile.createdAt || new Date().toISOString(),
            badges, // Include badges from JWT claims
          };

          set({
            user,
            isLoading: false,
            error: null,
          });
        } catch (error: unknown) {
          // DON'T clear auth state just because user-service fails
          // The user may still have a valid token - let them continue
          // Auth will be cleared if token refresh fails (401/403 from auth-service)
          console.error('Load user profile failed:', error);

          // Try to create a minimal user from JWT claims
          const claims = parseJwt(accessToken);
          if (claims?.sub) {
            const minimalUser: User = {
              id: claims.sub,
              username: 'Unknown', // Will be updated when user-service is available
              email: '',
              fullName: null,
              territory: claims.territory ?? 'dk',
              isActive: true,
              createdAt: new Date().toISOString(),
              badges: claims.badges ?? [],
            };
            set({
              user: minimalUser,
              isLoading: false,
              error: 'Unable to load full profile. Some features may be limited.',
            });
          } else {
            // Only now do we have a real auth problem
            const errorMessage = getErrorMessage(error, 'Failed to load user');
            set({ error: errorMessage, isLoading: false });
          }
        }
      },

      setTokens: (accessToken: string, refreshToken: string) => {
        set({
          accessToken,
          refreshToken,
          isAuthenticated: true,
          isLoading: false,
          error: null,
        });
      },

      setUser: (user: User) => {
        set({
          user,
          isAuthenticated: true,
          isLoading: false,
          error: null,
        });
      },

      clearAuth: () => {
        set({
          user: null,
          accessToken: null,
          refreshToken: null,
          isAuthenticated: false,
          isLoading: false,
          error: null,
          isLocked: false, // Reset lock state on clear
        });
      },

      lockSession: () => {
        set({ isLocked: true });
      },

      unlockSession: async (_email: string, password: string) => {
        const { user } = get();
        if (!user) {
          throw new Error('No user session to unlock');
        }

        set({ isLoading: true, error: null });
        try {
          // Re-authenticate with the provided credentials
          const credentials: LoginRequest = {
            username: user.username, // Use stored username
            password,
            territory: user.territory,
          };

          // Verify credentials by attempting login
          const response = await authApi.login(credentials);

          // Update tokens and unlock session
          set({
            accessToken: response.accessToken,
            refreshToken: response.refreshToken,
            isLocked: false,
            isLoading: false,
            error: null,
          });
        } catch (error: unknown) {
          // If login fails with 401 (invalid credentials or expired session),
          // clear auth and let user login fresh
          const is401 = typeof error === 'object' && error !== null &&
            'response' in error && (error as any).response?.status === 401;

          if (is401) {
            // Session truly expired or credentials wrong - clear and redirect
            get().clearAuth();
            // Redirect to login page
            if (typeof window !== 'undefined') {
              window.location.href = '/login';
            }
            return;
          }

          const errorMessage = getErrorMessage(error, 'Unlock failed');
          set({ error: errorMessage, isLoading: false });
          throw error;
        }
      },
    }),
    {
      name: 'auth-storage', // localStorage key
      storage: createJSONStorage(() => localStorage),
      // Only persist tokens and user, not loading/error states
      partialize: (state) => ({
        accessToken: state.accessToken,
        refreshToken: state.refreshToken,
        user: state.user,
        isAuthenticated: state.isAuthenticated,
        isLocked: state.isLocked,
      }),
    }
  )
);
