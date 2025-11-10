import { createFileRoute } from '@tanstack/react-router';
import { FriendRecoveryPage } from '@/pages/auth/FriendRecoveryPage';

export const Route = createFileRoute('/forgot-password_/friend')({
    component: RouteComponent,
});

function RouteComponent() {
    return <FriendRecoveryPage />;
}
