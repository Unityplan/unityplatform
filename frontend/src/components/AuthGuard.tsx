import { useEffect } from 'react';
import { useRouter } from '@tanstack/react-router';
import { useAuthStore } from '@/stores/authStore';

interface AuthGuardProps {
    children: React.ReactNode;
}

/**
 * AuthGuard component that protects routes from unauthenticated access.
 * Redirects to /login if user is not authenticated.
 */
export function AuthGuard({ children }: AuthGuardProps) {
    const { isAuthenticated, user } = useAuthStore();
    const router = useRouter();

    useEffect(() => {
        if (!isAuthenticated || !user) {
            // Store the current path to redirect back after login
            const currentPath = window.location.pathname;
            router.navigate({
                to: '/login',
                search: { redirect: currentPath },
            });
        }
    }, [isAuthenticated, user, router]);

    // Don't render children if not authenticated
    if (!isAuthenticated || !user) {
        return null;
    }

    return <>{children}</>;
}
