import { createFileRoute } from '@tanstack/react-router'
import { SecuritySettingsPage } from '@/pages/settings/SecuritySettingsPage'

export const Route = createFileRoute('/settings/security')({
    component: SecuritySettingsPage,
})
