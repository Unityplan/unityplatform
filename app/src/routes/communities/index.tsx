import { createFileRoute } from '@tanstack/react-router'
import { CommunitiesIndexPage } from '@/pages/communities/CommunitiesIndexPage'

export const Route = createFileRoute('/communities/')({
    component: CommunitiesIndexPage,
})

