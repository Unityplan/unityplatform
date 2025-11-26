import { useQuery } from '@tanstack/react-query'
import { communityService, CommunityType, type Community, type EffectiveBadgeRequirement } from '@/api/community'
import { Loader2, Map as MapIcon, ChevronRight, ChevronDown, Users, MapPin, BookOpen, Hammer, Shield, Package } from 'lucide-react'
import { useState, useMemo } from 'react'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Link } from '@tanstack/react-router'
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from '@/components/ui/tooltip'

interface TreeNode extends Community {
    children: TreeNode[]
    /** Badge requirements excluding Code of Conduct */
    badgeRequirements: Array<{ id: string; name: string; isInherited: boolean }>
}

export function StructureMap() {
    const [showOnlyPhysical, setShowOnlyPhysical] = useState(false)

    const { data: communities, isLoading, error } = useQuery({
        queryKey: ['communities'],
        queryFn: () => communityService.listCommunities({}),
    })

    // Fetch all requirements for all communities
    const { data: allRequirements } = useQuery({
        queryKey: ['all-community-requirements'],
        queryFn: async () => {
            if (!communities) return []
            const results = await Promise.all(
                communities.map(async (c) => {
                    try {
                        const reqs = await communityService.getEffectiveRequirements(c.id)
                        return { communityId: c.id, requirements: reqs }
                    } catch {
                        return { communityId: c.id, requirements: [] }
                    }
                })
            )
            return results
        },
        enabled: !!communities && communities.length > 0,
    })

    // Build requirements map for quick lookup
    const requirementsMap = useMemo(() => {
        const map = new Map<string, EffectiveBadgeRequirement[]>()
        if (allRequirements) {
            for (const item of allRequirements) {
                map.set(item.communityId, item.requirements)
            }
        }
        return map
    }, [allRequirements])

    if (isLoading) {
        return (
            <div className="flex h-[400px] items-center justify-center">
                <Loader2 className="h-8 w-8 animate-spin text-primary" />
            </div>
        )
    }

    if (error) {
        return (
            <div className="flex h-[400px] flex-col items-center justify-center gap-4 text-destructive">
                <p>Failed to load community structure</p>
                <Button variant="outline" onClick={() => window.location.reload()}>
                    Retry
                </Button>
            </div>
        )
    }

    const filteredCommunities = (communities || []).filter(c => {
        if (showOnlyPhysical && (c.type === CommunityType.Guild || c.type === CommunityType.StudyGroup || c.type === CommunityType.Group)) return false
        return true
    })

    const tree = buildTree(filteredCommunities, requirementsMap)

    return (
        <div className="rounded-lg border bg-card text-card-foreground shadow-sm">
            <div className="p-6">
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
                    <div className="flex items-center gap-2">
                        <MapIcon className="h-5 w-5 text-primary" />
                        <h3 className="text-lg font-semibold">Ecosystem Structure</h3>
                    </div>
                    <div className="flex items-center gap-2">
                        <Button
                            variant={!showOnlyPhysical ? "default" : "outline"}
                            size="sm"
                            onClick={() => setShowOnlyPhysical(!showOnlyPhysical)}
                            className="h-8 text-xs"
                        >
                            <MapPin className="mr-2 h-3 w-3" />
                            Show only physical level
                        </Button>
                    </div>
                </div>

                {tree.length === 0 ? (
                    <div className="text-center py-12 text-muted-foreground">
                        No communities found matching the filter.
                    </div>
                ) : (
                    <div className="space-y-2">
                        {tree.map((node) => (
                            <TreeNodeItem key={node.id} node={node} level={0} />
                        ))}
                    </div>
                )}
            </div>
        </div>
    )
}

function buildTree(communities: Community[], requirementsMap: Map<string, EffectiveBadgeRequirement[]>): TreeNode[] {
    const map = new Map<string, TreeNode>()
    const roots: TreeNode[] = []

    // Helper to get non-CoC badge requirements, deduplicated
    const getBadgeRequirements = (communityId: string) => {
        const reqs = requirementsMap.get(communityId) || []
        // Deduplicate by badgeId, prefer direct over inherited
        const byBadge = new Map<string, { id: string; name: string; isInherited: boolean }>()
        for (const req of reqs) {
            if (req.badgeSlug === 'code-of-conduct') continue
            const existing = byBadge.get(req.badgeId)
            if (!existing || (!req.isInherited && existing.isInherited)) {
                byBadge.set(req.badgeId, {
                    id: req.badgeId,
                    name: req.badgeName,
                    isInherited: req.isInherited,
                })
            }
        }
        return Array.from(byBadge.values())
    }

    // First pass: create nodes
    communities.forEach((c) => {
        map.set(c.id, {
            ...c,
            children: [],
            badgeRequirements: getBadgeRequirements(c.id),
        })
    })

    // Second pass: link children
    communities.forEach((c) => {
        const node = map.get(c.id)!
        if (c.parent_community_id && map.has(c.parent_community_id)) {
            map.get(c.parent_community_id)!.children.push(node)
        } else {
            roots.push(node)
        }
    })

    return roots
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

function TreeNodeItem({ node, level }: { node: TreeNode; level: number }) {
    const [isExpanded, setIsExpanded] = useState(true)
    const hasChildren = node.children.length > 0
    const hasBadges = node.badgeRequirements.length > 0

    const Icon = getCommunityIcon(node.type)

    return (
        <div className="select-none">
            <div
                className={cn(
                    "flex items-center gap-2 rounded-md p-2 hover:bg-accent/50 transition-colors",
                    // Mobile: Indent items directly (compact)
                    // Desktop: No item indent (handled by container)
                    level > 0 && "ml-4 md:ml-0"
                )}
            >
                <Button
                    variant="ghost"
                    size="icon"
                    className={cn("h-6 w-6 shrink-0", !hasChildren && "opacity-0 pointer-events-none")}
                    onClick={() => setIsExpanded(!isExpanded)}
                >
                    {isExpanded ? (
                        <ChevronDown className="h-4 w-4" />
                    ) : (
                        <ChevronRight className="h-4 w-4" />
                    )}
                </Button>

                <div className="flex flex-1 items-center gap-3 min-w-0">
                    <div className={cn(
                        "flex h-8 w-8 items-center justify-center rounded-full border bg-background shrink-0",
                        getIconColors(node.type)
                    )}>
                        <Icon className="h-4 w-4" />
                    </div>

                    {/* Main content - responsive layout */}
                    <div className="flex flex-1 flex-col md:flex-row md:items-center md:justify-between gap-1 md:gap-4 min-w-0">
                        {/* Left side: Name and basic info */}
                        <div className="flex flex-col min-w-0 flex-1">
                            <Link
                                to="/communities/$communityId/dashboard"
                                params={{ communityId: node.id }}
                                className="font-medium hover:underline truncate"
                            >
                                {node.name}
                            </Link>
                            <div className="flex items-center gap-2 text-xs text-muted-foreground">
                                <span className="flex items-center gap-1 shrink-0">
                                    <Users className="h-3 w-3" />
                                    {node.member_count}
                                </span>
                                {node.description && (
                                    <span className="hidden md:inline truncate">
                                        • {node.description}
                                    </span>
                                )}
                            </div>
                        </div>

                        {/* Right side: Badge requirements (desktop only, or as small indicator on mobile) */}
                        {hasBadges && (
                            <TooltipProvider>
                                <Tooltip>
                                    <TooltipTrigger asChild>
                                        <div className="flex items-center gap-1 shrink-0">
                                            {/* Mobile: Just show icon with count */}
                                            <div className="flex md:hidden items-center gap-1 text-primary">
                                                <Shield className="h-3.5 w-3.5" />
                                                <span className="text-xs">{node.badgeRequirements.length}</span>
                                            </div>
                                            {/* Desktop: Show badge pills */}
                                            <div className="hidden md:flex items-center gap-1.5 flex-wrap justify-end">
                                                {node.badgeRequirements.map((badge) => (
                                                    <Badge
                                                        key={badge.id}
                                                        variant={badge.isInherited ? "outline" : "secondary"}
                                                        className="text-xs py-0 h-5 gap-1"
                                                    >
                                                        <Shield className="h-3 w-3" />
                                                        {badge.name}
                                                    </Badge>
                                                ))}
                                            </div>
                                        </div>
                                    </TooltipTrigger>
                                    <TooltipContent side="left" className="md:hidden">
                                        <div className="text-xs">
                                            <p className="font-semibold mb-1">Required badges:</p>
                                            {node.badgeRequirements.map((badge) => (
                                                <p key={badge.id}>
                                                    • {badge.name}
                                                    {badge.isInherited && <span className="text-muted-foreground"> (inherited)</span>}
                                                </p>
                                            ))}
                                        </div>
                                    </TooltipContent>
                                </Tooltip>
                            </TooltipProvider>
                        )}
                    </div>
                </div>
            </div>

            {isExpanded && hasChildren && (
                <div className={cn(
                    "relative",
                    // Desktop: Tree structure with guide line
                    "md:ml-5 md:pl-4 md:border-l md:border-border/40"
                )}>
                    {node.children.map((child) => (
                        <TreeNodeItem key={child.id} node={child} level={level + 1} />
                    ))}
                </div>
            )}
        </div>
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
