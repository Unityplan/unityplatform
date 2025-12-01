import { useInfiniteQuery, useQuery } from '@tanstack/react-query'
import { communityService, CommunityType } from '@/api/community'
import type { Community, PaginatedCommunities } from '@/api/community'
import { Loader2, Home, Map as MapIcon, Hammer, Users, BookOpen, Edit, Search, Package } from 'lucide-react'
import { useState, useEffect, useRef, useCallback } from 'react'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Link } from '@tanstack/react-router'
import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
    CardDescription,
} from '@/components/ui/card'
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select'
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
        case CommunityType.Group:
            return Package
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
        case CommunityType.Group:
            return "border-orange-500 text-orange-600 dark:text-orange-400"
        default:
            return "border-gray-500 text-gray-600"
    }
}

const PAGE_SIZE = 50

export function CommunitiesIndexPage() {
    const [searchQuery, setSearchQuery] = useState('')
    const [typeFilter, setTypeFilter] = useState<CommunityType | 'all'>('all')
    const loadMoreRef = useRef<HTMLDivElement>(null)

    // Debounced search query
    const [debouncedSearch, setDebouncedSearch] = useState('')
    useEffect(() => {
        const timer = setTimeout(() => setDebouncedSearch(searchQuery), 300)
        return () => clearTimeout(timer)
    }, [searchQuery])

    const {
        data,
        isLoading,
        error,
        fetchNextPage,
        hasNextPage,
        isFetchingNextPage,
    } = useInfiniteQuery({
        queryKey: ['communities-paginated', debouncedSearch, typeFilter],
        queryFn: ({ pageParam = 0 }) =>
            communityService.listCommunitiesPaginated({
                search: debouncedSearch || undefined,
                community_type: typeFilter === 'all' ? undefined : typeFilter,
                limit: PAGE_SIZE,
                offset: pageParam,
            }),
        getNextPageParam: (lastPage: PaginatedCommunities) =>
            lastPage.hasMore ? lastPage.offset + lastPage.limit : undefined,
        initialPageParam: 0,
    })

    // Flatten all pages into a single array
    const communities = data?.pages.flatMap(page => page.items) ?? []
    const totalCount = data?.pages[0]?.total ?? 0

    // Intersection observer for infinite scroll
    const handleObserver = useCallback((entries: IntersectionObserverEntry[]) => {
        const [entry] = entries
        if (entry.isIntersecting && hasNextPage && !isFetchingNextPage) {
            fetchNextPage()
        }
    }, [fetchNextPage, hasNextPage, isFetchingNextPage])

    useEffect(() => {
        const element = loadMoreRef.current
        if (!element) return

        const observer = new IntersectionObserver(handleObserver, {
            root: null,
            rootMargin: '100px',
            threshold: 0,
        })

        observer.observe(element)
        return () => observer.disconnect()
    }, [handleObserver])

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

                {/* Search and Filter */}
                <div className="mb-6 flex flex-col sm:flex-row gap-4">
                    <div className="relative flex-1">
                        <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                        <Input
                            placeholder="Search communities..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            className="pl-10"
                        />
                    </div>
                    <Select
                        value={typeFilter}
                        onValueChange={(value) => setTypeFilter(value as CommunityType | 'all')}
                    >
                        <SelectTrigger className="w-full sm:w-48">
                            <SelectValue placeholder="Filter by type" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="all">All Types</SelectItem>
                            <SelectItem value={CommunityType.Zone}>Zones</SelectItem>
                            <SelectItem value={CommunityType.Neighborhood}>Neighborhoods</SelectItem>
                            <SelectItem value={CommunityType.Guild}>Guilds</SelectItem>
                            <SelectItem value={CommunityType.StudyGroup}>Study Groups</SelectItem>
                            <SelectItem value={CommunityType.Group}>Groups</SelectItem>
                        </SelectContent>
                    </Select>
                </div>

                {/* Results count */}
                {!isLoading && (
                    <p className="mb-4 text-sm text-muted-foreground">
                        Showing {communities.length} of {totalCount} communities
                    </p>
                )}

                {/* Loading state for initial load */}
                {isLoading && (
                    <div className="flex h-[50vh] items-center justify-center">
                        <Loader2 className="h-8 w-8 animate-spin text-primary" />
                    </div>
                )}

                {/* Error state */}
                {error && (
                    <div className="flex h-[50vh] flex-col items-center justify-center gap-4">
                        <p className="text-destructive">Failed to load communities</p>
                        <Button variant="outline" onClick={() => window.location.reload()}>
                            Retry
                        </Button>
                    </div>
                )}

                {/* Community grid */}
                {!isLoading && !error && (
                    <>
                        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                            {communities.map((community: Community) => {
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

                        {/* Load more trigger */}
                        <div ref={loadMoreRef} className="mt-8 flex justify-center">
                            {isFetchingNextPage && (
                                <Loader2 className="h-6 w-6 animate-spin text-primary" />
                            )}
                            {!hasNextPage && communities.length > 0 && (
                                <p className="text-sm text-muted-foreground">
                                    You've reached the end
                                </p>
                            )}
                        </div>

                        {/* Empty state */}
                        {communities.length === 0 && (
                            <div className="flex flex-col items-center justify-center py-12 text-center">
                                <Users className="h-12 w-12 text-muted-foreground/50 mb-4" />
                                <h3 className="text-lg font-semibold">No communities found</h3>
                                <p className="text-sm text-muted-foreground">
                                    {debouncedSearch || typeFilter !== 'all'
                                        ? 'Try adjusting your search or filters'
                                        : 'Be the first to create a community!'}
                                </p>
                            </div>
                        )}
                    </>
                )}
            </div>
        </AppLayout>
    )
}
