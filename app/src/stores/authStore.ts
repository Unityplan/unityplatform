import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import type { AuthStore, LoginRequest, RegisterRequest, User } from '@/types/auth';
import * as authApi from '@/api/auth';
import { getFullProfile } from '@/api/users';

/**
 * Extract error message from API error
 */
function getErrorMessage(error: unknown, defaultMessage: string): string {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === 'object' && error !== null && 'response' in error) {
    const response = (error as { response?: { data?: { detail?: string } } }).response;
    return response?.data?.detail || defaultMessage;
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
          // If refresh fails, clear auth state and throw
          get().clearAuth();
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
          // Fetch user profile from user-service
          const profile = await getFullProfile();
          
          // Map UserProfile to User type for auth store
          const user: User = {
            id: profile.id,
            username: profile.username,
            email: profile.email || '',
            fullName: profile.fullName || null,
            territory: 'dk', // TODO: Get from JWT token claims
            isActive: true,
            createdAt: profile.createdAt || new Date().toISOString(),
          };
          
          set({
            user,
            isLoading: false,
            error: null,
          });
        } catch (error: unknown) {
          // If loading user fails, clear auth state
          console.error('Load user failed:', error);
          get().clearAuth();
          const errorMessage = getErrorMessage(error, 'Failed to load user');
          set({ error: errorMessage, isLoading: false });
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
      }),
    }
  )
);
