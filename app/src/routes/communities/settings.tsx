import { createFileRoute } from '@tanstack/react-router'
import { GlobalSettingsPage } from '@/pages/communities/GlobalSettingsPage'

export const Route = createFileRoute('/communities/settings')({
    component: GlobalSettingsPage,
})
