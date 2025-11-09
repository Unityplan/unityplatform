import { createFileRoute } from '@tanstack/react-router';
import { AuthGuard } from '@/components/AuthGuard';
import { ProfileEditPage } from '@/pages/profile/ProfileEditPage';

export const Route = createFileRoute('/profile_/edit')({
    component: ProfileEdit,
});

function ProfileEdit() {
    return (
        <AuthGuard>
            <ProfileEditPage />
        </AuthGuard>
    );
}
