import { useEffect } from 'react';
import { useRouter } from '@tanstack/react-router';
import { useAuthStore } from '@/stores/authStore';
import { useInactivityDetector } from '@/hooks/useInactivityDetector';
import { SessionLockScreen } from '@/components/SessionLockScreen';

interface AuthGuardProps {
    children: React.ReactNode;
}

/**
 * AuthGuard component that protects routes from unauthenticated access.
 * 
 * **Security Features:**
 * - Redirects to /login if user is not authenticated
 * - Auto-locks session after 10 minutes of inactivity
 * - Shows session lock screen for re-authentication
 * - Stores redirect path for post-login navigation
 * 
 * @example
 * ```tsx
 * <AuthGuard>
 *   <ProtectedPage />
 * </AuthGuard>
 * ```
 */
export function AuthGuard({ children }: AuthGuardProps) {
    const { isAuthenticated, user, isLoading, isLocked, lockSession } = useAuthStore();
    const router = useRouter();

    // Inactivity detection - lock session after 10 minutes
    useInactivityDetector({
        timeout: 10 * 60 * 1000, // 10 minutes
        onInactive: () => {
            if (isAuthenticated && !isLocked) {
                lockSession();
            }
        },
        enabled: isAuthenticated && !isLocked,
    });

    useEffect(() => {
        if (!isAuthenticated) {
            // Store the current path to redirect back after login
            const currentPath = window.location.pathname;
            router.navigate({
                to: '/login',
                search: { redirect: currentPath },
            });
        }
    }, [isAuthenticated, router]);

    // Show session lock screen if locked
    if (isAuthenticated && isLocked) {
        return <SessionLockScreen />;
    }

    // Show loading while user data is being fetched
    if (isAuthenticated && !user && isLoading) {
        return (
            <div className="flex h-screen items-center justify-center">
                <div className="text-center">
                    <div className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]" />
                    <p className="mt-4 text-muted-foreground">Loading...</p>
                </div>
            </div>
        );
    }

    // Don't render children if not authenticated
    if (!isAuthenticated) {
        return null;
    }

    return <>{children}</>;
}
