import { useQuery } from '@tanstack/react-query'
import { communityService, CommunityType } from '@/api/community'
import type { Community } from '@/api/community'
import { Loader2, Home, Map as MapIcon, Hammer, Users, BookOpen, Edit } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Link } from '@tanstack/react-router'
import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
    CardDescription,
} from '@/components/ui/card'
import { AppLayout } from '@/components/layouts/AppLayout'
import {
    Breadcrumb,
    BreadcrumbList,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbSeparator,
    BreadcrumbPage,
} from '@/components/ui/breadcrumb'
import { cn } from '@/lib/utils'

function getCommunityIcon(type: CommunityType) {
    switch (type) {
        case CommunityType.Zone:
            return MapIcon
        case CommunityType.Neighborhood:
            return Users
        case CommunityType.Guild:
            return Hammer
        case CommunityType.StudyGroup:
            return BookOpen
        default:
            return Users
    }
}

function getIconColors(type: CommunityType) {
    switch (type) {
        case CommunityType.Zone:
            return "border-blue-500 text-blue-600 dark:text-blue-400"
        case CommunityType.Neighborhood:
            return "border-green-500 text-green-600 dark:text-green-400"
        case CommunityType.Guild:
            return "border-amber-500 text-amber-600 dark:text-amber-400"
        case CommunityType.StudyGroup:
            return "border-purple-500 text-purple-600 dark:text-purple-400"
        default:
            return "border-gray-500 text-gray-600"
    }
}

export function CommunitiesIndexPage() {
    const { data: communities, isLoading, error } = useQuery({
        queryKey: ['communities'],
        queryFn: () => communityService.listCommunities({}),
    })

    const { data: managedCommunityIds } = useQuery({
        queryKey: ['managed-communities'],
        queryFn: () => communityService.getManagedCommunities(),
    })

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
                    <BreadcrumbPage>Communities</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    )

    if (isLoading) {
        return (
            <AppLayout breadcrumbs={breadcrumbs}>
                <div className="flex h-[50vh] items-center justify-center">
                    <Loader2 className="h-8 w-8 animate-spin text-primary" />
                </div>
            </AppLayout>
        )
    }

    if (error) {
        return (
            <AppLayout breadcrumbs={breadcrumbs}>
                <div className="flex h-[50vh] flex-col items-center justify-center gap-4">
                    <p className="text-destructive">Failed to load communities</p>
                    <Button variant="outline" onClick={() => window.location.reload()}>
                        Retry
                    </Button>
                </div>
            </AppLayout>
        )
    }

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <div className="w-full px-4 py-8 md:px-8">
                <div className="mb-8 flex items-center justify-between">
                    <div>
                        <h1 className="text-3xl font-bold tracking-tight">Communities</h1>
                        <p className="text-muted-foreground">
                            Discover and join communities in your territory.
                        </p>
                    </div>
                    <Button asChild>
                        <Link to="/communities/create">Create Community</Link>
                    </Button>
                </div>

                <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                    {communities?.map((community: Community) => {
                        const Icon = getCommunityIcon(community.type)
                        const isManager = managedCommunityIds?.includes(community.id)
                        return (
                            <Card key={community.id} className="flex flex-col">
                                <CardHeader>
                                    <div className="flex items-start justify-between">
                                        <div className="flex items-center gap-3">
                                            <div className={cn(
                                                "flex h-8 w-8 items-center justify-center rounded-full border bg-background",
                                                getIconColors(community.type)
                                            )}>
                                                <Icon className="h-4 w-4" />
                                            </div>
                                            <CardTitle className="line-clamp-1">{community.name}</CardTitle>
                                        </div>
                                    </div>
                                    <CardDescription className="line-clamp-2 mt-2">
                                        {community.description || 'No description provided.'}
                                    </CardDescription>
                                </CardHeader>
                                <CardContent className="mt-auto pt-0">
                                    <div className="flex gap-2">
                                        <Button asChild className="flex-1" variant="secondary">
                                            <Link
                                                to="/communities/$communityId"
                                                params={{ communityId: community.id }}
                                            >
                                                View Community
                                            </Link>
                                        </Button>
                                        {isManager && (
                                            <Button asChild size="icon">
                                                <Link
                                                    to="/communities/$communityId/dashboard"
                                                    params={{ communityId: community.id }}
                                                >
                                                    <Edit className="h-4 w-4" />
                                                </Link>
                                            </Button>
                                        )}
                                    </div>
                                </CardContent>
                            </Card>
                        )
                    })}
                </div>
            </div>
        </AppLayout>
    )
}
