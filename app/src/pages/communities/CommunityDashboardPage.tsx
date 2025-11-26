import { Outlet, Link, useParams } from '@tanstack/react-router'
import { AppLayout } from '@/components/layouts/AppLayout'
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'
import { Home, LayoutDashboard, Users, Settings, Map as MapIcon, Hammer, BookOpen } from 'lucide-react'

import { cn } from '@/lib/utils'
import { buttonVariants } from '@/components/ui/button'
import { useQuery } from '@tanstack/react-query'
import { communityService, CommunityType } from '@/api/community'

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

export function CommunityDashboardPage() {
    const { communityId } = useParams({ from: '/communities/$communityId/dashboard' })
    const { data: community } = useQuery({
        queryKey: ['community', communityId],
        queryFn: () => communityService.getCommunity(communityId),
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
                    <BreadcrumbLink asChild>
                        <Link to="/communities">Communities</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbLink asChild>
                        <Link to="/communities/$communityId" params={{ communityId }}>
                            {community?.name || 'Community'}
                        </Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>Dashboard</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    )

    const sidebarItems = [
        {
            title: 'Overview',
            href: '/communities/$communityId/dashboard',
            icon: LayoutDashboard,
            exact: true,
        },
        {
            title: 'Structure',
            href: '/communities/$communityId/dashboard/structure',
            icon: MapIcon,
        },
        {
            title: 'Members',
            href: '/communities/$communityId/dashboard/members',
            icon: Users,
        },
        {
            title: 'Settings',
            href: '/communities/$communityId/dashboard/settings',
            icon: Settings,
        },
    ]

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <div className="w-full px-4 py-8 md:px-8">
                <div className="mb-8 flex items-center gap-4">
                    {community && (
                        <div className={cn(
                            "flex h-10 w-10 items-center justify-center rounded-full border bg-background",
                            getIconColors(community.type)
                        )}>
                            {(() => {
                                const Icon = getCommunityIcon(community.type)
                                return <Icon className="h-5 w-5" />
                            })()}
                        </div>
                    )}
                    <div>
                        <h1 className="text-3xl font-bold tracking-tight">{community?.name || 'Community'}</h1>
                        <p className="text-muted-foreground">Manage your community settings and members.</p>
                    </div>
                </div>
                <div className="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
                    <aside className="lg:w-1/5">
                        <nav className="flex space-x-2 lg:flex-col lg:space-x-0 lg:space-y-1">
                            {sidebarItems.map((item) => (
                                <Link
                                    key={item.href}
                                    to={item.href}
                                    params={{ communityId }}
                                    activeProps={{
                                        className: 'bg-muted hover:bg-muted',
                                    }}
                                    activeOptions={{ exact: item.exact }}
                                    className={cn(
                                        buttonVariants({ variant: 'ghost' }),
                                        'justify-start hover:bg-transparent hover:underline',
                                    )}
                                >
                                    <item.icon className="mr-2 h-4 w-4" />
                                    {item.title}
                                </Link>
                            ))}
                        </nav>
                    </aside>
                    <div className="flex-1">
                        <Outlet />
                    </div>
                </div>
            </div>
        </AppLayout>
    )
}
