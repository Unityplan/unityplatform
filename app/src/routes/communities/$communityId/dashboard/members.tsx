import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/communities/$communityId/dashboard/members')({
    component: DashboardMembers,
})

function DashboardMembers() {
    return (
        <div className="space-y-6">
            <div>
                <h3 className="text-lg font-medium">Members</h3>
                <p className="text-sm text-muted-foreground">
                    Manage community members and roles.
                </p>
            </div>
            <div className="rounded-lg border border-dashed p-8 text-center">
                <p className="text-muted-foreground">Member management coming soon.</p>
            </div>
        </div>
    )
}
