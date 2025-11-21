import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { RegisterPage } from '@/pages/auth/RegisterPage';
import { useAuthStore } from '@/stores/authStore';
import { useEffect } from 'react';
import { z } from 'zod';

const registerSearchSchema = z.object({
    token: z.string().optional(),
    invite: z.string().optional(),
});

export const Route = createFileRoute('/register')({
    validateSearch: (search) => registerSearchSchema.parse(search),
    component: Register,
});

function Register() {
    const { isAuthenticated } = useAuthStore();
    const navigate = useNavigate();
    const search = Route.useSearch();

    useEffect(() => {
        // Redirect to dashboard if already authenticated
        if (isAuthenticated) {
            navigate({ to: '/dashboard' });
        }
    }, [isAuthenticated, navigate]);

    return <RegisterPage initialToken={search.token || search.invite} />;
}
