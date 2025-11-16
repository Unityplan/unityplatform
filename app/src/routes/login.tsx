import { createFileRoute, redirect } from '@tanstack/react-router';
import { LoginPage } from '@/pages/auth/LoginPage';
import { useAuthStore } from '@/stores/authStore';

export const Route = createFileRoute('/login')({
    beforeLoad: () => {
        const { isAuthenticated } = useAuthStore.getState();
        if (isAuthenticated) {
            throw redirect({ to: '/dashboard' });
        }
    },
    component: LoginPage,
});
