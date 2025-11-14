import { useRouter } from '@tanstack/react-router';

/**
 * Custom hook for programmatic navigation
 * Provides a simpler API for common navigation tasks
 */
export function useNavigation() {
  const router = useRouter();

  const navigateTo = (path: string) => {
    router.navigate({ to: path as any });
  };

  return { navigateTo };
}
