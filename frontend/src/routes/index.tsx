import { createFileRoute, Navigate } from '@tanstack/react-router';
import { useAuthStore } from '@/stores/authStore';

export const Route = createFileRoute('/')({
    component: Index,
});

function Index() {
    const { isAuthenticated } = useAuthStore();

    // Redirect to dashboard if authenticated, otherwise to login
    if (isAuthenticated) {
        return <Navigate to="/dashboard" />;
    }

    return <Navigate to="/login" />;
}
