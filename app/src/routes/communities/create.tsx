import { createFileRoute } from '@tanstack/react-router'
import { CreateCommunityPage } from '@/pages/communities/CreateCommunityPage'

export const Route = createFileRoute('/communities/create')({
    component: CreateCommunityPage,
})
