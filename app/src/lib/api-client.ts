import axios from 'axios';
import type { AxiosError, InternalAxiosRequestConfig } from 'axios';
import { useAuthStore } from '@/stores/authStore';

/**
 * Check if the request is to the auth service
 */
function isAuthServiceRequest(url?: string): boolean {
  if (!url) return false;
  return url.includes('/auth/') || url.includes(':8001/');
}

/**
 * Axios instance with automatic token management and refresh
 * 
 * **Security Features:**
 * - Automatically adds Authorization header with access token
 * - Intercepts 401 responses and attempts token refresh
 * - Retries failed requests after successful token refresh
 * - Only clears auth on auth-service failures (not other services)
 * - Prevents infinite retry loops with _retry flag
 * - Request timeout of 30 seconds
 * 
 * **Best Practices:**
 * - Differentiates between auth failures and service failures
 * - User stays logged in even if non-auth services are down
 * - Token refresh is handled transparently
 * 
 * @example
 * ```typescript
 * // Automatically handles authentication
 * const response = await apiClient.get('/api/v1/users/me');
 * ```
 */
export const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:8080',
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 30000, // 30 seconds
  withCredentials: false, // Set to true if using cookies for CSRF tokens
});

// Request interceptor: Add auth token to requests
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    const { accessToken, isLocked } = useAuthStore.getState();

    // Block requests if session is locked (except auth endpoints)
    if (isLocked && !config.url?.includes('/auth/')) {
      return Promise.reject(new Error('Session is locked. Please unlock to continue.'));
    }

    if (accessToken && config.headers) {
      config.headers.Authorization = `Bearer ${accessToken}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Track ongoing refresh to prevent concurrent refreshes
let isRefreshing = false;
let refreshSubscribers: Array<(token: string) => void> = [];

/**
 * Add subscriber to wait for token refresh
 */
function subscribeTokenRefresh(callback: (token: string) => void) {
  refreshSubscribers.push(callback);
}

/**
 * Notify all subscribers when token is refreshed
 */
function onRefreshed(token: string) {
  refreshSubscribers.forEach((callback) => callback(token));
  refreshSubscribers = [];
}

// Response interceptor: Handle token refresh on 401
apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    const originalRequest = error.config as InternalAxiosRequestConfig & { _retry?: boolean };

    // Check if this is a network error (service unavailable)
    if (!error.response) {
      // Network error - don't clear auth, just reject
      console.warn('[API] Network error or service unavailable:', error.message);
      return Promise.reject(error);
    }

    // Skip token refresh for login/register endpoints - these use credentials, not tokens
    const isCredentialRequest = originalRequest?.url?.includes('/auth/login') ||
      originalRequest?.url?.includes('/auth/register');

    // If error is 401 Unauthorized and we haven't retried yet (and it's not a credential request)
    if (error.response?.status === 401 && originalRequest && !originalRequest._retry && !isCredentialRequest) {
      originalRequest._retry = true;

      // If already refreshing, wait for the new token
      if (isRefreshing) {
        return new Promise((resolve) => {
          subscribeTokenRefresh((token: string) => {
            if (originalRequest.headers) {
              originalRequest.headers.Authorization = `Bearer ${token}`;
            }
            resolve(apiClient(originalRequest));
          });
        });
      }

      isRefreshing = true;

      try {
        // Attempt to refresh the access token
        await useAuthStore.getState().refreshAccessToken();

        const { accessToken } = useAuthStore.getState();
        if (!accessToken) {
          throw new Error('No access token after refresh');
        }

        // Notify all waiting requests
        onRefreshed(accessToken);
        isRefreshing = false;

        // Retry the original request with the new token
        if (originalRequest.headers) {
          originalRequest.headers.Authorization = `Bearer ${accessToken}`;
        }
        return apiClient(originalRequest);
      } catch (refreshError) {
        // Token refresh failed
        isRefreshing = false;
        refreshSubscribers = [];

        // Only clear auth and redirect if this was an auth-service call that failed
        // This prevents logging out users just because user-service is down
        if (isAuthServiceRequest(originalRequest?.url)) {
          useAuthStore.getState().clearAuth();

          // Only redirect if not already on login/register page
          if (typeof window !== 'undefined') {
            const publicPaths = ['/login', '/register', '/forgot-password', '/reset-password'];
            const currentPath = window.location.pathname;
            const isPublicPath = publicPaths.some(path => currentPath.includes(path));

            if (!isPublicPath) {
              // Store current path for post-login redirect
              sessionStorage.setItem('redirectAfterLogin', currentPath);
              window.location.href = '/login';
            }
          }
        }

        return Promise.reject(refreshError);
      }
    }

    // Handle other errors - log but don't clear auth for non-auth services
    if (error.response?.status === 403) {
      console.warn('[API] Forbidden: You do not have permission to access this resource');
    }

    if (error.response?.status === 500) {
      console.warn('[API] Server error:', originalRequest?.url);
    }

    if (error.response?.status === 503) {
      console.warn('[API] Service unavailable:', originalRequest?.url);
    }

    return Promise.reject(error);
  }
);

export default apiClient;
