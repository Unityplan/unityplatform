import { createFileRoute } from '@tanstack/react-router'
import { GlobalMembersPage } from '@/pages/communities/GlobalMembersPage'

export const Route = createFileRoute('/communities/members')({
    component: GlobalMembersPage,
})
