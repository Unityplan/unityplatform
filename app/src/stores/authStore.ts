import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import type { AuthStore, LoginRequest, RegisterRequest, User } from '@/types/auth';
import * as authApi from '@/api/auth';

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

      // Actions
      login: async (credentials: LoginRequest) => {
        set({ isLoading: true, error: null });
        try {
          const response = await authApi.login(credentials);
          set({
            user: response.user,
            accessToken: response.access_token,
            refreshToken: response.refresh_token,
            isAuthenticated: true,
            isLoading: false,
            error: null,
          });
        } catch (error: unknown) {
          const errorMessage = getErrorMessage(error, 'Login failed');
          set({ error: errorMessage, isLoading: false });
          throw error;
        }
      },

      register: async (data: RegisterRequest) => {
        set({ isLoading: true, error: null });
        try {
          const response = await authApi.register(data);
          set({
            user: response.user,
            accessToken: response.access_token,
            refreshToken: response.refresh_token,
            isAuthenticated: true,
            isLoading: false,
            error: null,
          });
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
            accessToken: response.access_token,
            refreshToken: response.refresh_token,
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
          const user = await authApi.getCurrentUser();
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
        });
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
