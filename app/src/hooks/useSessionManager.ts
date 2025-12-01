import { useEffect, useRef, useCallback } from 'react';
import { useAuthStore } from '@/stores/authStore';
import { parseJwt } from '@/lib/utils';

interface UseSessionManagerOptions {
  /**
   * Time in milliseconds before token expiry to trigger refresh
   * @default 60000 (1 minute before expiry)
   */
  refreshBuffer?: number;
  /**
   * Whether session management is enabled
   * @default true
   */
  enabled?: boolean;
}

/**
 * Session management hook that handles:
 * - Proactive token refresh before expiry
 * - Works correctly even when browser tab is inactive
 * - Uses document.visibilitychange to catch up when tab becomes active
 * - Handles browser sleep/wake scenarios
 * 
 * **Best Practices Implemented:**
 * - Refresh tokens before they expire (not on 401)
 * - Use visibility API to handle background tabs
 * - Check token validity on tab focus
 * - Graceful degradation on refresh failure
 * 
 * @example
 * ```tsx
 * // In your main App or AuthProvider
 * useSessionManager({ enabled: isAuthenticated });
 * ```
 */
export function useSessionManager({
  refreshBuffer = 60 * 1000, // 1 minute before expiry
  enabled = true,
}: UseSessionManagerOptions = {}) {
  const refreshTimeoutRef = useRef<number | null>(null);
  const lastCheckRef = useRef<number>(Date.now());
  
  const { accessToken, refreshToken, isAuthenticated, refreshAccessToken } = useAuthStore();

  /**
   * Calculate time until token expires
   * Returns null if token is invalid or already expired
   */
  const getTimeUntilExpiry = useCallback((): number | null => {
    if (!accessToken) return null;
    
    const claims = parseJwt(accessToken);
    if (!claims?.exp) return null;
    
    const expiryTime = claims.exp * 1000; // Convert to milliseconds
    const now = Date.now();
    const timeUntilExpiry = expiryTime - now;
    
    return timeUntilExpiry > 0 ? timeUntilExpiry : null;
  }, [accessToken]);

  /**
   * Check if token needs refresh and schedule accordingly
   */
  const scheduleRefresh = useCallback(async () => {
    // Clear any existing scheduled refresh
    if (refreshTimeoutRef.current) {
      clearTimeout(refreshTimeoutRef.current);
      refreshTimeoutRef.current = null;
    }

    if (!enabled || !isAuthenticated || !accessToken || !refreshToken) {
      return;
    }

    const timeUntilExpiry = getTimeUntilExpiry();
    
    if (timeUntilExpiry === null) {
      // Token is already expired or invalid - try to refresh immediately
      console.log('[SessionManager] Token expired, attempting refresh...');
      try {
        await refreshAccessToken();
      } catch (error) {
        console.error('[SessionManager] Token refresh failed:', error);
        // Don't clear auth here - let the user continue with potentially stale token
        // The API interceptor will handle 401s
      }
      return;
    }

    // Calculate when to refresh (before expiry minus buffer)
    const refreshIn = Math.max(timeUntilExpiry - refreshBuffer, 0);
    
    if (refreshIn === 0) {
      // Need to refresh now
      console.log('[SessionManager] Token expiring soon, refreshing now...');
      try {
        await refreshAccessToken();
        // After successful refresh, schedule next refresh
        scheduleRefresh();
      } catch (error) {
        console.error('[SessionManager] Token refresh failed:', error);
      }
      return;
    }

    // Schedule refresh
    console.log(`[SessionManager] Scheduling token refresh in ${Math.round(refreshIn / 1000)}s`);
    refreshTimeoutRef.current = window.setTimeout(async () => {
      try {
        await refreshAccessToken();
        // After successful refresh, schedule next refresh
        scheduleRefresh();
      } catch (error) {
        console.error('[SessionManager] Scheduled token refresh failed:', error);
      }
    }, refreshIn);
  }, [enabled, isAuthenticated, accessToken, refreshToken, refreshBuffer, getTimeUntilExpiry, refreshAccessToken]);

  /**
   * Handle visibility change - check token when tab becomes visible
   * This catches cases where:
   * - Laptop was asleep and woke up
   * - User switched back to this tab after a long time
   * - Mobile browser was in background
   */
  const handleVisibilityChange = useCallback(() => {
    if (document.visibilityState === 'visible') {
      const now = Date.now();
      const timeSinceLastCheck = now - lastCheckRef.current;
      lastCheckRef.current = now;

      // If more than 30 seconds have passed, verify token and reschedule
      if (timeSinceLastCheck > 30 * 1000) {
        console.log('[SessionManager] Tab became visible after', Math.round(timeSinceLastCheck / 1000), 's - checking token');
        scheduleRefresh();
      }
    }
  }, [scheduleRefresh]);

  /**
   * Handle online event - check token when network is restored
   */
  const handleOnline = useCallback(() => {
    console.log('[SessionManager] Network restored - checking token');
    scheduleRefresh();
  }, [scheduleRefresh]);

  // Set up token refresh scheduling
  useEffect(() => {
    if (enabled && isAuthenticated) {
      scheduleRefresh();
    }

    return () => {
      if (refreshTimeoutRef.current) {
        clearTimeout(refreshTimeoutRef.current);
        refreshTimeoutRef.current = null;
      }
    };
  }, [enabled, isAuthenticated, accessToken, scheduleRefresh]);

  // Set up visibility change listener
  useEffect(() => {
    if (!enabled) return;

    document.addEventListener('visibilitychange', handleVisibilityChange);
    window.addEventListener('online', handleOnline);

    return () => {
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      window.removeEventListener('online', handleOnline);
    };
  }, [enabled, handleVisibilityChange, handleOnline]);

  // Return utility functions
  return {
    /**
     * Force a token refresh check
     */
    checkAndRefresh: scheduleRefresh,
    /**
     * Get remaining time until token expires (in milliseconds)
     */
    getTimeUntilExpiry,
  };
}
