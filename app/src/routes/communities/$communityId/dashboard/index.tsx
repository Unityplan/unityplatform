import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { communityService } from '@/api/community'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Users, Activity, Calendar, ShieldCheck } from 'lucide-react'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'

export const Route = createFileRoute('/communities/$communityId/dashboard/')({
    component: DashboardOverview,
})

function DashboardOverview() {
    const { communityId } = Route.useParams()
    const { data: community } = useQuery({
        queryKey: ['community', communityId],
        queryFn: () => communityService.getCommunity(communityId),
    })

    const { data: managers } = useQuery({
        queryKey: ['community-managers', communityId],
        queryFn: () => communityService.getEffectiveManagers(communityId),
    })

    return (
        <div className="space-y-6">
            <div>
                <h3 className="text-lg font-medium">Overview</h3>
                <p className="text-sm text-muted-foreground">
                    Welcome to the dashboard for {community?.name}.
                </p>
            </div>

            <div className="grid gap-4 md:grid-cols-3">
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Total Members
                        </CardTitle>
                        <Users className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">{community?.member_count || 0}</div>
                        <p className="text-xs text-muted-foreground">
                            +20.1% from last month
                        </p>
                    </CardContent>
                </Card>
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Active Discussions
                        </CardTitle>
                        <Activity className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">+12</div>
                        <p className="text-xs text-muted-foreground">
                            +19% from last month
                        </p>
                    </CardContent>
                </Card>
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Upcoming Events
                        </CardTitle>
                        <Calendar className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">3</div>
                        <p className="text-xs text-muted-foreground">
                            Next event in 2 days
                        </p>
                    </CardContent>
                </Card>
            </div>

            <div className="grid gap-4 md:grid-cols-2">
                <Card>
                    <CardHeader>
                        <CardTitle className="flex items-center gap-2">
                            <ShieldCheck className="h-5 w-5 text-primary" />
                            Responsible Managers
                        </CardTitle>
                        <CardDescription>
                            The effective managers responsible for this community.
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        {managers && managers.length > 0 ? (
                            <div className="space-y-4">
                                {managers.map((manager) => (
                                    <div key={manager.user_id} className="flex items-center justify-between">
                                        <div className="flex items-center gap-3">
                                            <Avatar className="h-9 w-9">
                                                <AvatarImage src={manager.avatar_url} alt={manager.username} />
                                                <AvatarFallback>{manager.username.substring(0, 2).toUpperCase()}</AvatarFallback>
                                            </Avatar>
                                            <div>
                                                <p className="text-sm font-medium leading-none">{manager.username}</p>
                                                <p className="text-xs text-muted-foreground capitalize">
                                                    {manager.source} (Distance: {manager.distance})
                                                </p>
                                            </div>
                                        </div>
                                        <div className="text-xs text-muted-foreground capitalize">
                                            {manager.role}
                                        </div>
                                    </div>
                                ))}
                            </div>
                        ) : (
                            <p className="text-sm text-muted-foreground">No managers found.</p>
                        )}
                    </CardContent>
                </Card>
            </div>
        </div>
    )
}
