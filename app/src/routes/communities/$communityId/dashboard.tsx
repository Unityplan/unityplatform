import { createFileRoute } from '@tanstack/react-router'
import { CommunityDashboardPage } from '@/pages/communities/CommunityDashboardPage'

export const Route = createFileRoute('/communities/$communityId/dashboard')({
    component: CommunityDashboardPage,
})
