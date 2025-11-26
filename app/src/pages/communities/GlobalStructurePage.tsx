import { AppLayout } from '@/components/layouts/AppLayout'
import { PageContainer } from '@/components/layouts/PageContainer'
import {
    Breadcrumb,
    BreadcrumbList,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbSeparator,
    BreadcrumbPage,
} from '@/components/ui/breadcrumb'
import { Link, useNavigate } from '@tanstack/react-router'
import { Home } from 'lucide-react'
import { StructureMap } from '@/components/communities/StructureMap'
import { CommunityGeoMap } from '@/components/communities/CommunityGeoMap'
import { CommunityNodeFlow } from '@/components/communities/CommunityNodeFlow'
import { InvitationList } from '@/components/invitations/InvitationList'
import { CreateInvitationDialog } from '@/components/invitations/CreateInvitationDialog'
import { CommunityStats } from '@/components/communities/CommunityStats'
import { CommunityNotifications } from '@/components/communities/CommunityNotifications'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Route } from '@/routes/communities/structure'

export function GlobalStructurePage() {
    const { tab } = Route.useSearch()
    const navigate = useNavigate()

    const handleTabChange = (value: string) => {
        navigate({
            to: '/communities/structure',
            search: { tab: value as 'hierarchy' | 'map' | 'flow' | 'stats' },
            replace: true,
        })
    }

    const breadcrumbs = (
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink asChild>
                        <Link to="/">
                            <Home className="size-4" />
                        </Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbLink asChild>
                        <Link to="/communities">Communities</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>Structure</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    )

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <PageContainer>
                <div className="mb-8">
                    <h1 className="text-3xl font-bold tracking-tight">Community Structure</h1>
                    <p className="text-muted-foreground">
                        Visualize the relationships and hierarchy of communities in your territory.
                    </p>
                </div>

                <Tabs value={tab} onValueChange={handleTabChange} className="w-full">
                    <TabsList className="mb-4">
                        <TabsTrigger value="hierarchy">Hierarchy</TabsTrigger>
                        <TabsTrigger value="map">Map</TabsTrigger>
                        <TabsTrigger value="flow">Node Flow</TabsTrigger>
                        <TabsTrigger value="stats">Stats & Alerts</TabsTrigger>
                    </TabsList>
                    <TabsContent value="hierarchy">
                        <StructureMap />
                    </TabsContent>
                    <TabsContent value="map">
                        <CommunityGeoMap />
                    </TabsContent>
                    <TabsContent value="flow">
                        <CommunityNodeFlow />
                    </TabsContent>
                    <TabsContent value="stats">
                        <div className="grid gap-4 md:grid-cols-3">
                            <div className="md:col-span-2">
                                <CommunityStats />
                            </div>
                            <div>
                                <CommunityNotifications />
                            </div>
                        </div>
                    </TabsContent>
                </Tabs>
            </PageContainer>
        </AppLayout>
    )
}
