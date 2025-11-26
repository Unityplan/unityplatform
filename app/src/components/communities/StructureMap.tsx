import { useQuery } from '@tanstack/react-query'
import { communityService, CommunityType, type Community } from '@/api/community'
import { Loader2, Map as MapIcon, ChevronRight, ChevronDown, Users, MapPin, BookOpen, Hammer } from 'lucide-react'
import { useState } from 'react'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Link } from '@tanstack/react-router'

interface TreeNode extends Community {
    children: TreeNode[]
}

export function StructureMap() {
    const [showOnlyPhysical, setShowOnlyPhysical] = useState(false)

    const { data: communities, isLoading, error } = useQuery({
        queryKey: ['communities'],
        queryFn: () => communityService.listCommunities({}),
    })

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
        if (showOnlyPhysical && (c.type === CommunityType.Guild || c.type === CommunityType.StudyGroup)) return false
        return true
    })

    const tree = buildTree(filteredCommunities)

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

function buildTree(communities: Community[]): TreeNode[] {
    const map = new Map<string, TreeNode>()
    const roots: TreeNode[] = []

    // First pass: create nodes
    communities.forEach((c) => {
        map.set(c.id, { ...c, children: [] })
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
                    className={cn("h-6 w-6 shrink-0", !hasChildren && "opacity-0")}
                    onClick={() => setIsExpanded(!isExpanded)}
                >
                    {isExpanded ? (
                        <ChevronDown className="h-4 w-4" />
                    ) : (
                        <ChevronRight className="h-4 w-4" />
                    )}
                </Button>

                <div className="flex flex-1 items-center gap-3">
                    <div className={cn(
                        "flex h-8 w-8 items-center justify-center rounded-full border bg-background",
                        getIconColors(node.type)
                    )}>
                        <Icon className="h-4 w-4" />
                    </div>

                    <div className="flex flex-col">
                        <Link
                            to="/communities/$communityId/dashboard"
                            params={{ communityId: node.id }}
                            className="font-medium hover:underline"
                        >
                            {node.name}
                        </Link>
                        <div className="flex flex-col gap-1 text-xs text-muted-foreground">
                            <span className="flex items-center gap-1">
                                <Users className="h-3 w-3" />
                                {node.member_count} members
                            </span>
                            {node.description && (
                                <span className="line-clamp-1">
                                    {node.description}
                                </span>
                            )}
                        </div>
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
        default:
            return Users
    }
}
