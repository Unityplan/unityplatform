import { createFileRoute } from '@tanstack/react-router';
import { ManagerRecoveryPage } from '@/pages/auth/ManagerRecoveryPage';

export const Route = createFileRoute('/forgot-password_/manager')({
    component: ManagerRecovery,
});

function ManagerRecovery() {
    return <ManagerRecoveryPage />;
}
