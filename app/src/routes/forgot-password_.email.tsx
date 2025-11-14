import { createFileRoute } from '@tanstack/react-router';
import { EmailRecoveryPage } from '@/pages/auth/EmailRecoveryPage';

export const Route = createFileRoute('/forgot-password_/email')({
    component: EmailRecovery,
});

function EmailRecovery() {
    return <EmailRecoveryPage />;
}
