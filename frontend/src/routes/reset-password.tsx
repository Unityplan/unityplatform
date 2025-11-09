import { createFileRoute } from '@tanstack/react-router';
import { PasswordResetPage } from '@/pages/auth/PasswordResetPage';

export const Route = createFileRoute('/reset-password')({
    component: ResetPassword,
});

function ResetPassword() {
    return <PasswordResetPage />;
}
