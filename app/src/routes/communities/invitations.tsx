import { createFileRoute } from '@tanstack/react-router';
import { AuthGuard } from '@/components/AuthGuard';
import { InvitationsPage } from '@/pages/communities/InvitationsPage';

export const Route = createFileRoute('/communities/invitations')({
    component: InvitationsRoute,
});

function InvitationsRoute() {
    return (
        <AuthGuard>
            <InvitationsPage />
        </AuthGuard>
    );
}
