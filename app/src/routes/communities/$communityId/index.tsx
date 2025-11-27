import { createFileRoute } from '@tanstack/react-router'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { communityService, CommunityType, type EffectiveBadgeRequirement } from '@/api/community'
import { territoryService } from '@/api/territory'
import { Loader2, Home, Map as MapIcon, Hammer, Users, BookOpen, Shield, Package, Lock, Settings } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Link } from '@tanstack/react-router'
import { AppLayout } from '@/components/layouts/AppLayout'
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'
import { useAuthStore } from '@/stores/authStore'
import { hasBadge } from '@/types/auth'
import { cn } from '@/lib/utils'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { useMemo } from 'react'

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

export const Route = createFileRoute('/communities/$communityId/')({
    component: CommunityDetail,
})

function CommunityDetail() {
    const { communityId } = Route.useParams()
    const { user } = useAuthStore()
    const queryClient = useQueryClient()

    const { data: community, isLoading, error } = useQuery({
        queryKey: ['community', communityId],
        queryFn: () => communityService.getCommunity(communityId),
    })

    const { data: managers } = useQuery({
        queryKey: ['community-managers', communityId],
        queryFn: () => communityService.getEffectiveManagers(communityId),
        enabled: !!communityId,
    })

    const { data: territory } = useQuery({
        queryKey: ['territory', community?.territory_id],
        queryFn: () => community?.territory_id ? territoryService.getTerritory(community.territory_id) : null,
        enabled: !!community?.territory_id,
    })

    const { data: membership, refetch: refetchMembership } = useQuery({
        queryKey: ['community-membership', communityId],
        queryFn: () => communityService.getMembership(communityId),
        enabled: !!communityId && !!user,
    })

    // Fetch badge requirements
    const { data: requirements } = useQuery({
        queryKey: ['community-requirements', communityId],
        queryFn: () => communityService.getEffectiveRequirements(communityId),
        enabled: !!communityId,
    })

    // Deduplicate requirements (prefer direct over inherited, exclude CoC)
    const badgeRequirements = useMemo(() => {
        if (!requirements) return []
        const byBadge = new Map<string, EffectiveBadgeRequirement>()
        for (const req of requirements) {
            if (req.badgeSlug === 'code-of-conduct') continue
            const existing = byBadge.get(req.badgeId)
            if (!existing || (!req.isInherited && existing.isInherited)) {
                byBadge.set(req.badgeId, req)
            }
        }
        return Array.from(byBadge.values())
    }, [requirements])

    // Check if user has all required badges
    const missingBadges = useMemo(() => {
        if (!badgeRequirements.length) return []
        return badgeRequirements.filter(req => !hasBadge(user, req.badgeSlug))
    }, [badgeRequirements, user])

    const hasAllBadges = missingBadges.length === 0

    // Check if current user is a manager (closest distance)
    const isManager = managers && user && managers.length > 0 && (() => {
        const minDistance = Math.min(...managers.map(m => m.distance))
        return managers.some(m => m.user_id === user.id && m.distance === minDistance)
    })()

    const joinMutation = useMutation({
        mutationFn: () => communityService.joinCommunity(communityId),
        onSuccess: () => {
            toast.success('Joined community successfully')
            refetchMembership()
            queryClient.invalidateQueries({ queryKey: ['community', communityId] })
        },
        onError: (error: unknown) => {
            // Check for 403 Forbidden (missing badges)
            if (error && typeof error === 'object' && 'response' in error) {
                const response = (error as { response?: { status?: number; data?: { message?: string } } }).response
                if (response?.status === 403) {
                    toast.error(response.data?.message || 'Missing required badges to join this community')
                    return
                }
            }
            toast.error('Failed to join community')
        },
    })

    const leaveMutation = useMutation({
        mutationFn: () => communityService.leaveCommunity(communityId),
        onSuccess: () => {
            toast.success('Left community successfully')
            refetchMembership()
            queryClient.invalidateQueries({ queryKey: ['community', communityId] })
        },
        onError: () => toast.error('Failed to leave community'),
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
                    <BreadcrumbPage>{community?.name || 'Loading...'}</BreadcrumbPage>
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

    if (error || !community) {
        return (
            <AppLayout breadcrumbs={breadcrumbs}>
                <div className="flex h-[50vh] flex-col items-center justify-center gap-4">
                    <p className="text-destructive">Failed to load community</p>
                    <Button variant="outline" asChild>
                        <Link to="/communities">Back to Communities</Link>
                    </Button>
                </div>
            </AppLayout>
        )
    }

    const Icon = getCommunityIcon(community.type)

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <div className="w-full px-4 py-8 md:px-8">
                <div className="mb-8">
                    <div className="flex items-center gap-4">
                        <div className={cn(
                            "flex h-10 w-10 items-center justify-center rounded-full border bg-background",
                            getIconColors(community.type)
                        )}>
                            <Icon className="h-5 w-5" />
                        </div>

                        <h1 className="text-3xl font-bold tracking-tight">{community.name}</h1>

                        <div className="ml-auto flex items-center gap-2">
                            {isManager && (
                                <Button variant="outline" size="sm" asChild>
                                    <Link to="/communities/$communityId/dashboard/settings" params={{ communityId }}>
                                        <Settings className="mr-1.5 h-4 w-4" />
                                        Settings
                                    </Link>
                                </Button>
                            )}
                            <Button variant="outline" size="sm" asChild>
                                <Link to="/communities">Back</Link>
                            </Button>
                        </div>
                    </div>
                    <p className="mt-2 text-muted-foreground">
                        {community.description || 'No description provided.'}
                    </p>
                </div>

                <div className="grid gap-6 md:grid-cols-3">
                    <div className="md:col-span-2">
                        <div className="rounded-lg border bg-card p-6 text-card-foreground shadow-sm">
                            <h2 className="mb-4 text-xl font-semibold">Content</h2>
                            <p className="text-muted-foreground">
                                Community content will appear here.
                            </p>
                        </div>
                    </div>

                    <div>
                        <div className="rounded-lg border bg-card p-6 text-card-foreground shadow-sm">
                            <h2 className="mb-4 text-xl font-semibold">Details</h2>
                            <dl className="space-y-4 text-sm">
                                <div>
                                    <dt className="font-medium text-muted-foreground">Members</dt>
                                    <dd>{community.member_count}</dd>
                                </div>
                                <div>
                                    <dt className="font-medium text-muted-foreground">Created</dt>
                                    <dd>{new Date(community.created_at).toLocaleDateString()}</dd>
                                </div>
                                {community.territory_id && (
                                    <div>
                                        <dt className="font-medium text-muted-foreground">Territory</dt>
                                        <dd className="flex items-center gap-2">
                                            {territory ? (
                                                <>
                                                    <span className="text-lg">{territory.flag_icon}</span>
                                                    <span>{territory.name}</span>
                                                </>
                                            ) : (
                                                community.territory_id
                                            )}
                                        </dd>
                                    </div>
                                )}
                                {managers && managers.length > 0 && (
                                    <div>
                                        <dt className="font-medium text-muted-foreground mb-2">Managers</dt>
                                        <dd className="space-y-2">
                                            {managers.map((manager) => (
                                                <div key={manager.user_id} className="flex items-center gap-2">
                                                    <Avatar className="h-6 w-6">
                                                        <AvatarImage src={manager.avatar_url || undefined} />
                                                        <AvatarFallback>{manager.username.substring(0, 2).toUpperCase()}</AvatarFallback>
                                                    </Avatar>
                                                    <span>{manager.username}</span>
                                                    {manager.source === 'territory' && (
                                                        <span className="text-xs text-muted-foreground">(Territory)</span>
                                                    )}
                                                </div>
                                            ))}
                                        </dd>
                                    </div>
                                )}
                                {badgeRequirements.length > 0 && (
                                    <div>
                                        <dt className="font-medium text-muted-foreground mb-2 flex items-center gap-1">
                                            <Shield className="h-4 w-4" />
                                            Required Badges
                                        </dt>
                                        <dd className="space-y-2">
                                            {badgeRequirements.map((req) => {
                                                const userHasBadge = hasBadge(user, req.badgeSlug)
                                                return (
                                                    <div key={req.badgeId} className="flex items-center gap-2">
                                                        <Badge
                                                            variant={userHasBadge ? "default" : "outline"}
                                                            className={cn(
                                                                "text-xs",
                                                                !userHasBadge && "border-destructive/50 text-destructive"
                                                            )}
                                                        >
                                                            {userHasBadge ? (
                                                                <Shield className="mr-1 h-3 w-3" />
                                                            ) : (
                                                                <Lock className="mr-1 h-3 w-3" />
                                                            )}
                                                            {req.badgeName}
                                                        </Badge>
                                                        {req.isInherited && (
                                                            <span className="text-[10px] text-muted-foreground">
                                                                from {req.sourceCommunityName}
                                                            </span>
                                                        )}
                                                    </div>
                                                )
                                            })}
                                            {!hasAllBadges && (
                                                <p className="text-xs text-destructive mt-2">
                                                    You need all required badges to join this community.
                                                </p>
                                            )}
                                        </dd>
                                    </div>
                                )}
                            </dl>

                            <div className="mt-6">
                                {membership?.is_member ? (
                                    <Button
                                        variant="outline"
                                        className="w-full text-destructive hover:text-destructive hover:bg-destructive/10"
                                        onClick={() => leaveMutation.mutate()}
                                        disabled={leaveMutation.isPending}
                                    >
                                        {leaveMutation.isPending ? <Loader2 className="mr-2 h-4 w-4 animate-spin" /> : null}
                                        Leave Community
                                    </Button>
                                ) : community.type === CommunityType.Zone ? (
                                    <div className="text-center text-sm text-muted-foreground p-2 bg-muted rounded-md">
                                        Zones are administrative areas and cannot be joined directly.
                                    </div>
                                ) : !hasAllBadges && badgeRequirements.length > 0 ? (
                                    <div className="space-y-2">
                                        <Button
                                            className="w-full"
                                            disabled
                                            variant="outline"
                                        >
                                            <Lock className="mr-2 h-4 w-4" />
                                            Badge Required
                                        </Button>
                                        <p className="text-xs text-center text-muted-foreground">
                                            Obtain the required badges to join
                                        </p>
                                    </div>
                                ) : (
                                    <Button
                                        className="w-full"
                                        onClick={() => joinMutation.mutate()}
                                        disabled={joinMutation.isPending}
                                    >
                                        {joinMutation.isPending ? <Loader2 className="mr-2 h-4 w-4 animate-spin" /> : null}
                                        Join Community
                                    </Button>
                                )}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </AppLayout>
    )
}

function getCommunityIcon(type: CommunityType) {
    switch (type) {
        case CommunityType.Zone:
            return MapIcon
        case CommunityType.Neighborhood:
            return Users
        case CommunityType.Guild:
            return Hammer // Tool icon for Guild
        case CommunityType.StudyGroup:
            return BookOpen
        case CommunityType.Group:
            return Package
        default:
            return Users
    }
}
