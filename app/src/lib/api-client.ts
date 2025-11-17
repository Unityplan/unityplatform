import axios from 'axios';
import type { AxiosError, InternalAxiosRequestConfig } from 'axios';
import { useAuthStore } from '@/stores/authStore';

/**
 * Axios instance with automatic token management and refresh
 * 
 * **Security Features:**
 * - Automatically adds Authorization header with access token
 * - Intercepts 401 responses and attempts token refresh
 * - Retries failed requests after successful token refresh
 * - Redirects to login if token refresh fails
 * - Prevents infinite retry loops with _retry flag
 * - Clears auth state on refresh failure
 * - Request timeout of 30 seconds
 * 
 * @example
 * ```typescript
 * // Automatically handles authentication
 * const response = await apiClient.get('/api/v1/users/me');
 * ```
 */
const apiClient = axios.create({
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

    // If error is 401 Unauthorized and we haven't retried yet
    if (error.response?.status === 401 && originalRequest && !originalRequest._retry) {
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
        // Token refresh failed - clear auth and redirect to login
        isRefreshing = false;
        refreshSubscribers = [];
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
        
        return Promise.reject(refreshError);
      }
    }

    // Handle other errors
    if (error.response?.status === 403) {
      console.error('Forbidden: You do not have permission to access this resource');
    }

    if (error.response?.status === 500) {
      console.error('Server error: Please try again later');
    }

    return Promise.reject(error);
  }
);

export default apiClient;
