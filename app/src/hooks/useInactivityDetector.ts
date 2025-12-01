import { useEffect, useRef, useCallback } from 'react';

interface UseInactivityDetectorOptions {
  /**
   * Time in milliseconds before triggering inactivity
   * @default 600000 (10 minutes)
   */
  timeout?: number;
  /**
   * Callback function to execute when inactivity is detected
   */
  onInactive: () => void;
  /**
   * Whether the detector is enabled
   * @default true
   */
  enabled?: boolean;
  /**
   * Events to listen for user activity
   * @default ['mousedown', 'mousemove', 'keypress', 'scroll', 'touchstart', 'click']
   */
  events?: string[];
}

/**
 * Hook to detect user inactivity and trigger a callback
 * 
 * This hook tracks user activity (mouse movements, keyboard input, touch, etc.)
 * and triggers a callback after a specified period of inactivity.
 * 
 * **Handles Background Tabs:**
 * - Uses localStorage to persist last activity time across tabs
 * - Checks on visibility change when tab becomes active
 * - Works correctly even when browser throttles setTimeout in background tabs
 * 
 * **Security Use Cases:**
 * - Auto-lock session after 10 minutes of inactivity
 * - Auto-logout after 30 minutes of inactivity
 * - Show session expiry warnings
 * 
 * @example
 * ```tsx
 * useInactivityDetector({
 *   timeout: 10 * 60 * 1000, // 10 minutes
 *   onInactive: () => {
 *     lockSession();
 *   },
 * });
 * ```
 */
export function useInactivityDetector({
  timeout = 10 * 60 * 1000, // 10 minutes default
  onInactive,
  enabled = true,
  events = ['mousedown', 'mousemove', 'keypress', 'scroll', 'touchstart', 'click'],
}: UseInactivityDetectorOptions) {
  const timeoutRef = useRef<number | null>(null);
  const onInactiveRef = useRef(onInactive);
  const lastActivityKey = 'lastActivityTime';

  // Keep onInactive ref up to date
  useEffect(() => {
    onInactiveRef.current = onInactive;
  }, [onInactive]);

  /**
   * Get last activity timestamp from localStorage (shared across tabs)
   */
  const getLastActivity = useCallback((): number => {
    const stored = localStorage.getItem(lastActivityKey);
    return stored ? parseInt(stored, 10) : Date.now();
  }, []);

  /**
   * Update last activity timestamp in localStorage
   */
  const updateLastActivity = useCallback(() => {
    localStorage.setItem(lastActivityKey, Date.now().toString());
  }, []);

  /**
   * Check if inactivity timeout has been exceeded
   */
  const checkInactivity = useCallback(() => {
    const lastActivity = getLastActivity();
    const elapsed = Date.now() - lastActivity;
    
    if (elapsed >= timeout) {
      onInactiveRef.current();
      return true;
    }
    return false;
  }, [timeout, getLastActivity]);

  const resetTimer = useCallback(() => {
    // Update last activity timestamp
    updateLastActivity();
    
    // Clear existing timer
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    // Set new timer
    timeoutRef.current = window.setTimeout(() => {
      // Double-check using stored timestamp (handles background throttling)
      if (!checkInactivity()) {
        // Not actually inactive yet (timer was throttled), reschedule
        const lastActivity = getLastActivity();
        const remaining = timeout - (Date.now() - lastActivity);
        if (remaining > 0) {
          timeoutRef.current = window.setTimeout(() => {
            checkInactivity();
          }, remaining);
        }
      }
    }, timeout);
  }, [timeout, updateLastActivity, checkInactivity, getLastActivity]);

  /**
   * Handle visibility change - check inactivity when tab becomes visible
   */
  const handleVisibilityChange = useCallback(() => {
    if (document.visibilityState === 'visible' && enabled) {
      // Check if we should have triggered inactivity while in background
      if (!checkInactivity()) {
        // Still active, reset the timer with remaining time
        const lastActivity = getLastActivity();
        const elapsed = Date.now() - lastActivity;
        const remaining = timeout - elapsed;
        
        if (remaining > 0) {
          if (timeoutRef.current) {
            clearTimeout(timeoutRef.current);
          }
          timeoutRef.current = window.setTimeout(() => {
            checkInactivity();
          }, remaining);
        }
      }
    }
  }, [enabled, timeout, checkInactivity, getLastActivity]);

  useEffect(() => {
    if (!enabled) {
      // Clean up if disabled
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
      return;
    }

    // Initialize timer
    resetTimer();

    // Attach event listeners for activity
    const handleActivity = () => {
      resetTimer();
    };

    events.forEach((event) => {
      window.addEventListener(event, handleActivity, { passive: true });
    });

    // Listen for visibility changes to handle background tabs
    document.addEventListener('visibilitychange', handleVisibilityChange);

    // Cleanup
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
      events.forEach((event) => {
        window.removeEventListener(event, handleActivity);
      });
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  }, [enabled, events, resetTimer, handleVisibilityChange]);

  // Public method to manually reset the timer
  return {
    resetTimer,
    checkInactivity,
  };
}
