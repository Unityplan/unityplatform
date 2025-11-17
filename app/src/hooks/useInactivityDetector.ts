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

  // Keep onInactive ref up to date
  useEffect(() => {
    onInactiveRef.current = onInactive;
  }, [onInactive]);

  const resetTimer = useCallback(() => {
    // Clear existing timer
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    // Set new timer
    timeoutRef.current = window.setTimeout(() => {
      onInactiveRef.current();
    }, timeout);
  }, [timeout]);

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

    // Attach event listeners
    const handleActivity = () => {
      resetTimer();
    };

    events.forEach((event) => {
      window.addEventListener(event, handleActivity, { passive: true });
    });

    // Cleanup
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
      events.forEach((event) => {
        window.removeEventListener(event, handleActivity);
      });
    };
  }, [enabled, events, resetTimer]);

  // Public method to manually reset the timer
  return {
    resetTimer,
  };
}
