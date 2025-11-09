import { createFileRoute } from '@tanstack/react-router';
import { AuthGuard } from '@/components/AuthGuard';
import { ProfileViewPage } from '@/pages/profile/ProfileViewPage';

export const Route = createFileRoute('/profile')({
  component: Profile,
});

function Profile() {
  return (
    <AuthGuard>
      <ProfileViewPage />
    </AuthGuard>
  );
}
