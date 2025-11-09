import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { LoginPage } from '@/pages/auth/LoginPage';
import { useAuthStore } from '@/stores/authStore';
import { useEffect } from 'react';

export const Route = createFileRoute('/login')({
  component: Login,
});

function Login() {
  const { isAuthenticated } = useAuthStore();
  const navigate = useNavigate();

  useEffect(() => {
    // Redirect to dashboard if already authenticated
    if (isAuthenticated) {
      navigate({ to: '/dashboard' });
    }
  }, [isAuthenticated, navigate]);

  return <LoginPage />;
}
