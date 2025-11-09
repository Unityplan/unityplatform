import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { RegisterPage } from '@/pages/auth/RegisterPage';
import { useAuthStore } from '@/stores/authStore';
import { useEffect } from 'react';

export const Route = createFileRoute('/register')({
  component: Register,
});

function Register() {
  const { isAuthenticated } = useAuthStore();
  const navigate = useNavigate();

  useEffect(() => {
    // Redirect to dashboard if already authenticated
    if (isAuthenticated) {
      navigate({ to: '/dashboard' });
    }
  }, [isAuthenticated, navigate]);

  return <RegisterPage />;
}
