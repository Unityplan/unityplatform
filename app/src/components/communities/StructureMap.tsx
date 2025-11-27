import { useQuery, useQueryClient } from '@tanstack/react-query'
import { communityService, CommunityType, type Community, type EffectiveBadgeRequirement } from '@/api/community'
import { Loader2, Map as MapIcon, ChevronRight, ChevronDown, Users, MapPin, BookOpen, Hammer, Shield, Package } from 'lucide-react'
import { useState, useMemo, useCallback } from 'react'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
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
    /** Whether children are loaded */
    childrenLoaded: boolean
    /** Whether this node has children (may be lazy loaded) */
    hasChildren: boolean
}

export function StructureMap() {
    const [showOnlyPhysical, setShowOnlyPhysical] = useState(false)
    const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set())
    const queryClient = useQueryClient()

    // Load first 2 levels of hierarchy
    const { data: initialCommunities, isLoading, error } = useQuery({
        queryKey: ['communities-hierarchy', 2],
        queryFn: () => communityService.getHierarchy(2),
    })

    // Fetch all requirements for visible communities
    const visibleCommunityIds = useMemo(() => {
        if (!initialCommunities) return []
        return initialCommunities.map(c => c.id)
    }, [initialCommunities])

    const { data: allRequirements } = useQuery({
        queryKey: ['all-community-requirements', visibleCommunityIds],
        queryFn: async () => {
            if (!visibleCommunityIds.length) return []
            const results = await Promise.all(
                visibleCommunityIds.map(async (id) => {
                    try {
                        const reqs = await communityService.getEffectiveRequirements(id)
                        return { communityId: id, requirements: reqs }
                    } catch {
                        return { communityId: id, requirements: [] }
                    }
                })
            )
            return results
        },
        enabled: visibleCommunityIds.length > 0,
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

    // Lazy load children of a community
    const loadChildren = useCallback(async (communityId: string) => {
        const children = await communityService.getChildren(communityId)
        // Update cache with children
        queryClient.setQueryData(['community-children', communityId], children)
        // Also fetch requirements for new children
        const newReqs = await Promise.all(
            children.map(async (c) => {
                try {
                    const reqs = await communityService.getEffectiveRequirements(c.id)
                    return { communityId: c.id, requirements: reqs }
                } catch {
                    return { communityId: c.id, requirements: [] }
                }
            })
        )
        // Merge new requirements into cache
        queryClient.setQueryData(['all-community-requirements', visibleCommunityIds], (old: typeof allRequirements) => {
            if (!old) return newReqs
            return [...old, ...newReqs]
        })
        return children
    }, [queryClient, visibleCommunityIds])

    const toggleExpanded = useCallback((nodeId: string) => {
        setExpandedNodes(prev => {
            const next = new Set(prev)
            if (next.has(nodeId)) {
                next.delete(nodeId)
            } else {
                next.add(nodeId)
            }
            return next
        })
    }, [])

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

    const filteredCommunities = (initialCommunities || []).filter(c => {
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
                    <div className="flex items-center gap-3">
                        <Switch
                            id="physical-only"
                            checked={showOnlyPhysical}
                            onCheckedChange={setShowOnlyPhysical}
                        />
                        <Label htmlFor="physical-only" className="flex items-center gap-2 cursor-pointer">
                            <MapPin className="h-4 w-4 text-muted-foreground" />
                            <span>Physical only</span>
                        </Label>
                    </div>
                </div>

                {tree.length === 0 ? (
                    <div className="text-center py-12 text-muted-foreground">
                        No communities found matching the filter.
                    </div>
                ) : (
                    <div className="space-y-2">
                        {tree.map((node) => (
                            <TreeNodeItem 
                                key={node.id} 
                                node={node} 
                                level={0}
                                expandedNodes={expandedNodes}
                                onToggleExpand={toggleExpanded}
                                onLoadChildren={loadChildren}
                            />
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
    const childCounts = new Map<string, number>()

    // Sort order: Zone first, then Neighborhood, then Group, then others
    const getTypeSortOrder = (type: CommunityType): number => {
        switch (type) {
            case CommunityType.Zone: return 0
            case CommunityType.Neighborhood: return 1
            case CommunityType.Group: return 2
            case CommunityType.Guild: return 3
            case CommunityType.StudyGroup: return 4
            default: return 5
        }
    }

    const sortNodes = (nodes: TreeNode[]): TreeNode[] => {
        return nodes.sort((a, b) => {
            const typeOrder = getTypeSortOrder(a.type) - getTypeSortOrder(b.type)
            if (typeOrder !== 0) return typeOrder
            // Same type: sort alphabetically by name
            return a.name.localeCompare(b.name)
        })
    }

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

    // Count children for each parent
    communities.forEach(c => {
        if (c.parent_community_id) {
            childCounts.set(c.parent_community_id, (childCounts.get(c.parent_community_id) || 0) + 1)
        }
    })

    // First pass: create nodes
    communities.forEach((c) => {
        map.set(c.id, {
            ...c,
            children: [],
            badgeRequirements: getBadgeRequirements(c.id),
            childrenLoaded: true, // Initially loaded from hierarchy endpoint
            hasChildren: childCounts.has(c.id) || false, // Will be updated
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

    // Third pass: update hasChildren based on actual children and sort all children recursively
    const sortChildrenRecursive = (nodes: TreeNode[]) => {
        for (const node of nodes) {
            node.hasChildren = node.children.length > 0
            if (node.children.length > 0) {
                node.children = sortNodes(node.children)
                sortChildrenRecursive(node.children)
            }
        }
    }

    // Sort roots and all children
    const sortedRoots = sortNodes(roots)
    sortChildrenRecursive(sortedRoots)

    return sortedRoots
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

interface TreeNodeItemProps {
    node: TreeNode
    level: number
    expandedNodes: Set<string>
    onToggleExpand: (nodeId: string) => void
    onLoadChildren: (nodeId: string) => Promise<Community[]>
}

function TreeNodeItem({ node, level, expandedNodes, onToggleExpand, onLoadChildren }: TreeNodeItemProps) {
    const [isLoading, setIsLoading] = useState(false)
    const [loadedChildren, setLoadedChildren] = useState<TreeNode[]>([])
    
    const isExpanded = expandedNodes.has(node.id)
    const hasChildren = node.hasChildren || node.children.length > 0
    const hasBadges = node.badgeRequirements.length > 0
    
    // Use loaded children if available, otherwise use tree children
    const childrenToShow = loadedChildren.length > 0 ? loadedChildren : node.children

    const handleToggle = async () => {
        if (!hasChildren) return
        
        // If expanding and children not yet loaded, load them
        if (!isExpanded && node.children.length === 0 && !loadedChildren.length) {
            setIsLoading(true)
            try {
                const children = await onLoadChildren(node.id)
                // Convert to TreeNode format
                const treeChildren: TreeNode[] = children.map(c => ({
                    ...c,
                    children: [],
                    badgeRequirements: [],
                    childrenLoaded: false,
                    hasChildren: false, // Unknown until expanded
                }))
                setLoadedChildren(treeChildren)
            } finally {
                setIsLoading(false)
            }
        }
        
        onToggleExpand(node.id)
    }

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
                    onClick={handleToggle}
                    disabled={isLoading}
                >
                    {isLoading ? (
                        <Loader2 className="h-4 w-4 animate-spin" />
                    ) : isExpanded ? (
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

            {isExpanded && hasChildren && childrenToShow.length > 0 && (
                <div className={cn(
                    "relative",
                    // Desktop: Tree structure with guide line
                    "md:ml-5 md:pl-4 md:border-l md:border-border/40"
                )}>
                    {childrenToShow.map((child) => (
                        <TreeNodeItem 
                            key={child.id} 
                            node={child} 
                            level={level + 1}
                            expandedNodes={expandedNodes}
                            onToggleExpand={onToggleExpand}
                            onLoadChildren={onLoadChildren}
                        />
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
