import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

export type Theme = 'light' | 'dark' | 'system';

interface UIState {
  theme: Theme;
  sidebarOpen: boolean;
  setTheme: (theme: Theme) => void;
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
}

/**
 * UI state store using Zustand with localStorage persistence
 * 
 * Stores:
 * - Theme preference (light/dark/system)
 * - Sidebar open/closed state
 * 
 * Persisted in localStorage to survive page refreshes
 */
export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      theme: 'system',
      sidebarOpen: true,

      setTheme: (theme) => {
        set({ theme });
        
        // Apply theme to document
        const root = window.document.documentElement;
        root.classList.remove('light', 'dark');

        if (theme === 'system') {
          const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
            ? 'dark'
            : 'light';
          root.classList.add(systemTheme);
        } else {
          root.classList.add(theme);
        }
      },

      toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
      
      setSidebarOpen: (open) => set({ sidebarOpen: open }),
    }),
    {
      name: 'ui-storage', // localStorage key
      storage: createJSONStorage(() => localStorage),
    }
  )
);

// Initialize theme on app load
if (typeof window !== 'undefined') {
  const storedTheme = useUIStore.getState().theme;
  useUIStore.getState().setTheme(storedTheme);
}
