import { createFileRoute } from '@tanstack/react-router';
import { PrivacySettingsPage } from '@/pages/settings/PrivacySettingsPage';

export const Route = createFileRoute('/settings/privacy')({
    component: PrivacySettingsPage,
});
