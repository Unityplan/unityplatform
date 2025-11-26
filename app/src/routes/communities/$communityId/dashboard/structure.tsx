import { createFileRoute } from '@tanstack/react-router'
import { DashboardNodeFlow } from '@/components/communities/DashboardNodeFlow'

export const Route = createFileRoute('/communities/$communityId/dashboard/structure')({
    component: DashboardStructure,
})

function DashboardStructure() {
    const { communityId } = Route.useParams()

    return (
        <div className="space-y-6">
            <div>
                <h3 className="text-lg font-medium">Community Structure</h3>
                <p className="text-sm text-muted-foreground">
                    Visualize and manage the structure of your community.
                </p>
            </div>
            <DashboardNodeFlow communityId={communityId} />
        </div>
    )
}
