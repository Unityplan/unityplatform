import { useCallback, useEffect, useMemo, useState } from 'react'
import {
    ReactFlow,
    MiniMap,
    Controls,
    Background,
    useNodesState,
    useEdgesState,
    useReactFlow,
    ReactFlowProvider,
    Handle,
    Position,
    type Node,
    type Edge,
    MarkerType,
} from '@xyflow/react'
import '@xyflow/react/dist/style.css'
import './CommunityNodeFlow.css'
import { BookOpen, Users, Shield, Map as MapIcon, Hammer, Loader2, Package, ChevronRight, Home, Lock, X, UserPlus, LogOut, ExternalLink, GraduationCap, Award } from 'lucide-react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { communityService, CommunityType, type Community, type EffectiveBadgeRequirement } from '@/api/community'
import { badgeService, type Badge as BadgeType } from '@/api/badge'
import { useTheme } from 'next-themes'
import ELK from 'elkjs/lib/elk.bundled.js'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { toast } from 'sonner'
import { useAuthStore } from '@/stores/authStore'
import { Link } from '@tanstack/react-router'

const elk = new ELK()

// Types for our extended community data (using effective requirements with inheritance info)
type CommunityWithReqs = Omit<Community, 'requirements'> & { requirements: EffectiveBadgeRequirement[] }

// --- Custom Node Components ---

const BadgeNode = ({ data }: { data: { label: string } }) => {
    return (
        <div className="flex flex-col items-center justify-center gap-2">
            <div className="flex h-16 w-16 items-center justify-center rounded-full border-2 border-primary bg-background shadow-md transition-transform hover:scale-110">
                <Shield className="h-8 w-8 text-primary" />
                <Handle type="target" position={Position.Bottom} className="bg-muted-foreground!" />
                <Handle type="source" position={Position.Top} className="bg-muted-foreground!" />
            </div>
            <span className="max-w-[100px] text-center text-xs font-medium text-muted-foreground">
                {data.label}
            </span>
        </div>
    )
}

const CommunityNode = ({ data }: { data: { label: string; type: CommunityType; badges?: Array<{ id: string; name: string }>; onBadgeClick?: (badgeIds: string[]) => void } }) => {
    const Icon = useMemo(() => {
        switch (data.type) {
            case CommunityType.Zone: return MapIcon
            case CommunityType.Neighborhood: return Users
            case CommunityType.Guild: return Hammer
            case CommunityType.StudyGroup: return BookOpen
            case CommunityType.Group: return Package
            default: return Users
        }
    }, [data.type])

    const colorClass = useMemo(() => {
        switch (data.type) {
            case CommunityType.Zone: return "border-blue-500 bg-blue-50 dark:bg-blue-950/30"
            case CommunityType.Neighborhood: return "border-green-500 bg-green-50 dark:bg-green-950/30"
            case CommunityType.Guild: return "border-amber-500 bg-amber-50 dark:bg-amber-950/30"
            case CommunityType.StudyGroup: return "border-purple-500 bg-purple-50 dark:bg-purple-950/30"
            case CommunityType.Group: return "border-orange-500 bg-orange-50 dark:bg-orange-950/30"
            default: return "border-gray-500 bg-gray-50"
        }
    }, [data.type])

    const iconColorClass = useMemo(() => {
        switch (data.type) {
            case CommunityType.Zone: return "text-blue-600 dark:text-blue-400"
            case CommunityType.Neighborhood: return "text-green-600 dark:text-green-400"
            case CommunityType.Guild: return "text-amber-600 dark:text-amber-400"
            case CommunityType.StudyGroup: return "text-purple-600 dark:text-purple-400"
            case CommunityType.Group: return "text-orange-600 dark:text-orange-400"
            default: return "text-gray-600"
        }
    }, [data.type])

    const handleBadgeClick = (e: React.MouseEvent) => {
        e.stopPropagation()
        if (data.badges && data.badges.length > 0 && data.onBadgeClick) {
            data.onBadgeClick(data.badges.map(b => b.id))
        }
    }

    const hasBadges = data.badges && data.badges.length > 0

    return (
        <div className={`flex min-w-44 min-h-[140px] flex-col items-center gap-2 rounded-lg border-2 p-3 shadow-sm transition-all hover:shadow-md ${colorClass}`}>
            <div className={`flex h-10 w-10 items-center justify-center rounded-full bg-background/80 ${iconColorClass}`}>
                <Icon className="h-6 w-6" />
            </div>
            <div className="flex flex-col items-center text-center">
                <span className="text-sm font-bold">{data.label}</span>
                <span className="text-[10px] uppercase tracking-wider opacity-70">{data.type.replace('_', ' ')}</span>
            </div>

            {/* Badge requirements - clickable */}
            {hasBadges && (
                <div
                    className="flex items-center gap-2 p-2 rounded-md bg-primary/10 hover:bg-primary/20 transition-colors cursor-pointer border border-primary/20 w-full mt-auto"
                    onClick={handleBadgeClick}
                    title="Click to learn how to obtain these badges"
                >
                    <div className="flex h-5 w-5 items-center justify-center rounded-full bg-primary/20 text-primary shrink-0">
                        <Shield className="h-3 w-3" />
                    </div>
                    <div className="flex-1 min-w-0 text-left">
                        <div className="text-[10px] text-muted-foreground">
                            {data.badges!.length === 1 ? 'Badge required' : `${data.badges!.length} badges required`}
                        </div>
                    </div>
                    <Lock className="h-3 w-3 text-primary shrink-0" />
                </div>
            )}

            <Handle type="target" position={Position.Bottom} className="bg-muted-foreground!" />
            <Handle type="source" position={Position.Top} className="bg-muted-foreground!" />
        </div>
    )
}

// Collapsed Group Node - shows summary of guilds/study groups
interface GroupNodeData {
    label: string
    badges?: Array<{ id: string; name: string }>
    groupId: string
    onDrillDown: (groupId: string) => void
    onBadgeClick?: (badgeIds: string[]) => void
    [key: string]: unknown // Index signature to satisfy Record<string, unknown>
}

const GroupNode = ({ data }: { data: GroupNodeData }) => {
    const handleBadgeClick = (e: React.MouseEvent) => {
        e.stopPropagation() // Prevent drill-down when clicking badge
        if (data.badges && data.badges.length > 0 && data.onBadgeClick) {
            data.onBadgeClick(data.badges.map(b => b.id))
        }
    }

    const hasBadges = data.badges && data.badges.length > 0

    return (
        <div
            className="flex min-w-44 min-h-[140px] flex-col gap-2 rounded-lg border-2 border-dashed border-orange-500 bg-orange-100 dark:bg-orange-950 p-3 shadow-sm transition-all hover:shadow-md hover:border-solid cursor-pointer"
            onClick={() => data.onDrillDown(data.groupId)}
        >
            <div className="flex items-center gap-2">
                <div className="flex h-8 w-8 items-center justify-center rounded-full bg-orange-200 dark:bg-orange-900/50 text-orange-600 dark:text-orange-400">
                    <Package className="h-4 w-4" />
                </div>
                <span className="text-sm font-bold text-orange-700 dark:text-orange-300">{data.label}</span>
            </div>

            {/* Badge requirements - clickable */}
            {hasBadges && (
                <div
                    className="flex items-center gap-2 p-2 rounded-md bg-primary/10 hover:bg-primary/20 transition-colors cursor-pointer border border-primary/20"
                    onClick={handleBadgeClick}
                    title="Click to learn how to obtain these badges"
                >
                    <div className="flex h-5 w-5 items-center justify-center rounded-full bg-primary/20 text-primary shrink-0">
                        <Shield className="h-3 w-3" />
                    </div>
                    <div className="flex-1 min-w-0">
                        <div className="text-[10px] text-muted-foreground">
                            {data.badges!.length === 1 ? 'Badge required' : `${data.badges!.length} badges required`}
                        </div>
                    </div>
                    <Lock className="h-3 w-3 text-primary shrink-0" />
                </div>
            )}

            <div className="mt-auto text-[10px] text-center text-muted-foreground/70">
                Click to explore →
            </div>
            <Handle type="target" position={Position.Bottom} className="bg-muted-foreground!" />
            <Handle type="source" position={Position.Top} className="bg-muted-foreground!" />
        </div>
    )
}

// Info Panel Component
interface InfoPanelProps {
    community: Community | null
    requirements: EffectiveBadgeRequirement[]
    badges: BadgeType[]
    isMember: boolean
    isLoading: boolean
    onClose: () => void
    onJoin: () => void
    onLeave: () => void
}

const InfoPanel = ({ community, requirements, badges, isMember, isLoading, onClose, onJoin, onLeave }: InfoPanelProps) => {
    const Icon = useMemo(() => {
        if (!community) return Users
        switch (community.type) {
            case CommunityType.Zone: return MapIcon
            case CommunityType.Neighborhood: return Users
            case CommunityType.Guild: return Hammer
            case CommunityType.StudyGroup: return BookOpen
            case CommunityType.Group: return Package
            default: return Users
        }
    }, [community])

    const colorClass = useMemo(() => {
        if (!community) return "text-gray-600"
        switch (community.type) {
            case CommunityType.Zone: return "text-blue-600 dark:text-blue-400"
            case CommunityType.Neighborhood: return "text-green-600 dark:text-green-400"
            case CommunityType.Guild: return "text-amber-600 dark:text-amber-400"
            case CommunityType.StudyGroup: return "text-purple-600 dark:text-purple-400"
            case CommunityType.Group: return "text-orange-600 dark:text-orange-400"
            default: return "text-gray-600"
        }
    }, [community])

    const typeLabel = useMemo(() => {
        if (!community) return "Community"
        switch (community.type) {
            case CommunityType.Zone: return "Zone"
            case CommunityType.Neighborhood: return "Neighborhood"
            case CommunityType.Guild: return "Guild"
            case CommunityType.StudyGroup: return "Study Group"
            case CommunityType.Group: return "Group"
            default: return "Community"
        }
    }, [community])

    // EffectiveBadgeRequirement already has badge name and inheritance info
    const canJoin = community && community.type !== CommunityType.Zone && community.type !== CommunityType.Group

    if (!community) return null

    return (
        <div className="absolute top-0 right-0 w-80 h-full bg-background border-l shadow-lg z-50 flex flex-col">
            {/* Header */}
            <div className="flex items-center justify-between p-4 border-b">
                <div className="flex items-center gap-2">
                    <Icon className={`h-5 w-5 ${colorClass}`} />
                    <span className="font-semibold text-sm">{typeLabel}</span>
                </div>
                <Button variant="ghost" size="icon" onClick={onClose} className="h-8 w-8">
                    <X className="h-4 w-4" />
                </Button>
            </div>

            {/* Content */}
            <div className="flex-1 overflow-auto p-4 space-y-4">
                {/* Title */}
                <div>
                    <h3 className="text-lg font-bold">{community.name}</h3>
                    {community.description && (
                        <p className="text-sm text-muted-foreground mt-1">{community.description}</p>
                    )}
                </div>

                <Separator />

                {/* Stats */}
                <div className="grid grid-cols-2 gap-3">
                    <div className="flex items-center gap-2 text-sm">
                        <Users className="h-4 w-4 text-muted-foreground" />
                        <span>{community.member_count} member{community.member_count !== 1 ? 's' : ''}</span>
                    </div>
                    {community.territory_id && (
                        <div className="flex items-center gap-2 text-sm">
                            <MapIcon className="h-4 w-4 text-muted-foreground" />
                            <span className="uppercase">{community.territory_id}</span>
                        </div>
                    )}
                </div>

                {/* Badge Requirements (including inherited) */}
                {requirements.length > 0 && (
                    <>
                        <Separator />
                        <div>
                            <h4 className="text-sm font-semibold mb-2 flex items-center gap-1">
                                <Shield className="h-4 w-4" />
                                Badge Requirements
                            </h4>
                            <div className="flex flex-col gap-2">
                                {requirements.map((req) => (
                                    <div key={req.id} className="flex items-center gap-2">
                                        <Badge variant={req.isInherited ? "outline" : "secondary"} className="text-xs">
                                            {req.badgeName}
                                            <span className="ml-1 text-muted-foreground">({req.context})</span>
                                        </Badge>
                                        {req.isInherited && (
                                            <span className="text-[10px] text-muted-foreground">
                                                from {req.sourceCommunityName}
                                            </span>
                                        )}
                                    </div>
                                ))}
                            </div>
                        </div>
                    </>
                )}

                {/* Membership Status */}
                {canJoin && (
                    <>
                        <Separator />
                        <div>
                            <h4 className="text-sm font-semibold mb-2">Membership</h4>
                            {isMember ? (
                                <div className="flex items-center justify-between">
                                    <Badge variant="outline" className="text-green-600 border-green-600">
                                        <Users className="h-3 w-3 mr-1" />
                                        You are a member
                                    </Badge>
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        onClick={onLeave}
                                        disabled={isLoading}
                                        className="text-destructive hover:text-destructive"
                                    >
                                        <LogOut className="h-4 w-4 mr-1" />
                                        Leave
                                    </Button>
                                </div>
                            ) : (
                                <Button
                                    onClick={onJoin}
                                    disabled={isLoading}
                                    className="w-full"
                                >
                                    <UserPlus className="h-4 w-4 mr-1" />
                                    Join Community
                                </Button>
                            )}
                        </div>
                    </>
                )}
            </div>

            {/* Footer */}
            <div className="p-4 border-t">
                <Link
                    to="/communities/$communityId"
                    params={{ communityId: community.id }}
                    className="w-full"
                >
                    <Button variant="outline" className="w-full">
                        <ExternalLink className="h-4 w-4 mr-1" />
                        View Details
                    </Button>
                </Link>
            </div>
        </div>
    )
}

// Badge Info Panel - shows how to obtain badges (supports multiple)
interface BadgeInfoPanelProps {
    badges: BadgeType[]
    onClose: () => void
}

const BadgeInfoPanel = ({ badges, onClose }: BadgeInfoPanelProps) => {
    if (!badges || badges.length === 0) return null

    const renderBadgeInfo = (badge: BadgeType) => {
        // Determine how the badge can be obtained based on its slug/category
        const isManagerBadge = badge.slug?.includes('manager') || badge.category === 'role'
        const isCourseCompletionBadge = badge.criteria_type === 'course_completion'
        const isCodeOfConduct = badge.slug === 'code-of-conduct'

        return (
            <div key={badge.id} className="space-y-3">
                {/* Badge Icon & Name */}
                <div className="flex items-center gap-3">
                    <div className="flex h-12 w-12 items-center justify-center rounded-full bg-primary/10 text-primary shrink-0">
                        <Shield className="h-6 w-6" />
                    </div>
                    <div>
                        <h4 className="font-semibold">{badge.name}</h4>
                        <Badge variant="secondary" className="text-xs">{badge.category}</Badge>
                    </div>
                </div>

                {/* Description */}
                {badge.description && (
                    <p className="text-sm text-muted-foreground">{badge.description}</p>
                )}

                {/* How to Obtain */}
                {isCodeOfConduct && (
                    <div className="p-3 rounded-lg bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800">
                        <div className="flex items-start gap-2">
                            <div className="flex h-6 w-6 items-center justify-center rounded-full bg-green-100 dark:bg-green-900 text-green-600 dark:text-green-400 shrink-0">
                                <Users className="h-3 w-3" />
                            </div>
                            <div>
                                <p className="text-xs font-medium text-green-700 dark:text-green-300">Automatic</p>
                                <p className="text-xs text-green-600 dark:text-green-400 mt-0.5">
                                    Granted when you agree to the Code of Conduct during registration.
                                </p>
                            </div>
                        </div>
                    </div>
                )}

                {isManagerBadge && !isCodeOfConduct && (
                    <div className="p-3 rounded-lg bg-amber-50 dark:bg-amber-950 border border-amber-200 dark:border-amber-800">
                        <div className="flex items-start gap-2">
                            <div className="flex h-6 w-6 items-center justify-center rounded-full bg-amber-100 dark:bg-amber-900 text-amber-600 dark:text-amber-400 shrink-0">
                                <Users className="h-3 w-3" />
                            </div>
                            <div>
                                <p className="text-xs font-medium text-amber-700 dark:text-amber-300">Granted by Others</p>
                                <p className="text-xs text-amber-600 dark:text-amber-400 mt-0.5">
                                    Nominated and approved by a user with authority to grant this badge.
                                </p>
                            </div>
                        </div>
                    </div>
                )}

                {isCourseCompletionBadge && (
                    <div className="p-3 rounded-lg bg-purple-50 dark:bg-purple-950 border border-purple-200 dark:border-purple-800">
                        <div className="flex items-start gap-2">
                            <div className="flex h-6 w-6 items-center justify-center rounded-full bg-purple-100 dark:bg-purple-900 text-purple-600 dark:text-purple-400 shrink-0">
                                <GraduationCap className="h-3 w-3" />
                            </div>
                            <div>
                                <p className="text-xs font-medium text-purple-700 dark:text-purple-300">Complete a Course</p>
                                <p className="text-xs text-purple-600 dark:text-purple-400 mt-0.5">
                                    Earned by completing a specific course in the Learning section.
                                </p>
                            </div>
                        </div>
                    </div>
                )}

                {!isCodeOfConduct && !isManagerBadge && !isCourseCompletionBadge && (
                    <div className="p-3 rounded-lg bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800">
                        <div className="flex items-start gap-2">
                            <div className="flex h-6 w-6 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400 shrink-0">
                                <Shield className="h-3 w-3" />
                            </div>
                            <div>
                                <p className="text-xs font-medium text-blue-700 dark:text-blue-300">Custom Requirements</p>
                                <p className="text-xs text-blue-600 dark:text-blue-400 mt-0.5">
                                    Check with community administrators for details.
                                </p>
                            </div>
                        </div>
                    </div>
                )}
            </div>
        )
    }

    return (
        <div className="absolute top-0 right-0 w-80 h-full bg-background border-l shadow-lg z-50 flex flex-col">
            {/* Header */}
            <div className="flex items-center justify-between p-4 border-b">
                <div className="flex items-center gap-2">
                    <Shield className="h-5 w-5 text-primary" />
                    <span className="font-semibold text-sm">
                        {badges.length === 1 ? 'Badge Required' : `${badges.length} Badges Required`}
                    </span>
                </div>
                <Button variant="ghost" size="icon" onClick={onClose} className="h-8 w-8">
                    <X className="h-4 w-4" />
                </Button>
            </div>

            {/* Content */}
            <div className="flex-1 overflow-auto p-4 space-y-4">
                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                    <Award className="h-4 w-4" />
                    <span>How to obtain {badges.length === 1 ? 'this badge' : 'these badges'}:</span>
                </div>

                <div className="space-y-6">
                    {badges.map(badge => renderBadgeInfo(badge))}
                </div>
            </div>

            {/* Footer */}
            <div className="p-4 border-t">
                <Button variant="outline" className="w-full" disabled>
                    <ExternalLink className="h-4 w-4 mr-1" />
                    View Badge Details (Coming Soon)
                </Button>
            </div>
        </div>
    )
}

const nodeTypes = {
    badge: BadgeNode,
    community: CommunityNode,
    group: GroupNode,
}

const getLayoutedElements = async (nodes: Node[], edges: Edge[]) => {
    const graph = {
        id: 'root',
        layoutOptions: {
            'elk.algorithm': 'layered',
            'elk.direction': 'UP',
            'elk.spacing.nodeNode': '80',
            'elk.layered.spacing.nodeNodeBetweenLayers': '120',
            'elk.layered.considerModelOrder.strategy': 'NODES_AND_EDGES',
        },
        children: nodes.map((node) => ({
            id: node.id,
            width: 180,
            height: 150, // Consistent height for all nodes to accommodate badge area
        })),
        edges: edges.map((edge) => ({
            id: edge.id,
            sources: [edge.source],
            targets: [edge.target],
        })),
    }

    const layoutedGraph = await elk.layout(graph)

    const layoutedNodes = nodes.map((node) => {
        const layoutedNode = layoutedGraph.children?.find((n) => n.id === node.id)
        return {
            ...node,
            position: {
                x: layoutedNode?.x || 0,
                y: layoutedNode?.y || 0,
            },
        }
    })

    return { nodes: layoutedNodes, edges }
}

export function CommunityNodeFlow() {
    return (
        <ReactFlowProvider>
            <CommunityNodeFlowInner />
        </ReactFlowProvider>
    )
}

function CommunityNodeFlowInner() {
    const { resolvedTheme } = useTheme()
    const queryClient = useQueryClient()
    const { user } = useAuthStore()
    const reactFlowInstance = useReactFlow()
    const [nodes, setNodes, onNodesChange] = useNodesState<Node>([])
    const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([])

    // Drill-down state: null = show full tree, string = show subtree from that community
    const [viewRoot, setViewRoot] = useState<string | null>(null)
    // Breadcrumb trail for navigation
    const [breadcrumbs, setBreadcrumbs] = useState<Array<{ id: string; name: string }>>([])
    // Selected community for info panel
    const [selectedCommunityId, setSelectedCommunityId] = useState<string | null>(null)
    // Selected badges for badge info panel (supports multiple)
    const [selectedBadgeIds, setSelectedBadgeIds] = useState<string[]>([])

    const { data: communities, isLoading: isLoadingCommunities } = useQuery({
        queryKey: ['communities'],
        queryFn: () => communityService.listCommunities({}),
    })

    // Fetch EFFECTIVE requirements for all communities (includes inherited from parent Groups)
    const { data: allRequirements } = useQuery({
        queryKey: ['all-community-effective-requirements', communities?.map(c => c.id)],
        queryFn: async () => {
            if (!communities) return []
            const promises = communities.map(async (c) => {
                try {
                    const reqs = await communityService.getEffectiveRequirements(c.id)
                    return { communityId: c.id, requirements: reqs }
                } catch (e) {
                    console.error(`Failed to fetch effective requirements for community ${c.id}`, e)
                    return { communityId: c.id, requirements: [] }
                }
            })
            return Promise.all(promises)
        },
        enabled: !!communities && communities.length > 0,
    })

    const { data: badges, isLoading: isLoadingBadges } = useQuery({
        queryKey: ['badges'],
        queryFn: () => badgeService.listBadges(),
    })

    // Drill-down handler
    const handleDrillDown = useCallback((groupId: string) => {
        const community = communities?.find(c => c.id === groupId)
        if (community) {
            setBreadcrumbs(prev => [...prev, { id: groupId, name: community.name }])
            setViewRoot(groupId)
            // Fit view after drill-down with a small delay to let the graph re-render
            setTimeout(() => {
                reactFlowInstance.fitView({ padding: 0.2, duration: 300 })
            }, 100)
        }
    }, [communities, reactFlowInstance])

    // Badge click handler - shows badge info panel (supports multiple badges)
    const handleBadgeClick = useCallback((badgeIds: string[]) => {
        setSelectedCommunityId(null) // Close community panel if open
        setSelectedBadgeIds(badgeIds)
    }, [])

    // Navigate back via breadcrumb
    const handleBreadcrumbClick = useCallback((index: number) => {
        // Close all info panels when navigating
        setSelectedCommunityId(null)
        setSelectedBadgeIds([])

        if (index === -1) {
            // Go to root
            setViewRoot(null)
            setBreadcrumbs([])
        } else {
            const newBreadcrumbs = breadcrumbs.slice(0, index + 1)
            setBreadcrumbs(newBreadcrumbs)
            setViewRoot(newBreadcrumbs[newBreadcrumbs.length - 1]?.id ?? null)
        }
        // Fit view after navigation with a small delay
        setTimeout(() => {
            reactFlowInstance.fitView({ padding: 0.2, duration: 300 })
        }, 100)
    }, [breadcrumbs, reactFlowInstance])

    // Handle window resize - fit view when container size changes
    useEffect(() => {
        const handleResize = () => {
            // Debounce the fitView call
            setTimeout(() => {
                reactFlowInstance.fitView({ padding: 0.2, duration: 200 })
            }, 100)
        }

        window.addEventListener('resize', handleResize)
        return () => window.removeEventListener('resize', handleResize)
    }, [reactFlowInstance])

    useEffect(() => {
        if (!communities || !badges) return

        const buildGraph = async () => {
            // Merge requirements into communities
            const communitiesWithReqs: CommunityWithReqs[] = communities.map(c => {
                const reqs = allRequirements?.find(r => r.communityId === c.id)?.requirements || []
                return { ...c, requirements: reqs }
            })

            const newNodes: Node[] = []
            const newEdges: Edge[] = []

            // 1. Find Code of Conduct Badge
            const cocBadge = badges.find(b => b.name.toLowerCase().includes('code of conduct') || b.slug.includes('conduct'))

            let cocNodeId: string | null = null

            // Helper to sort communities by type priority (zone > neighborhood > guild > study_group > group) then by name
            const typePriority: Record<string, number> = {
                [CommunityType.Zone]: 0,
                [CommunityType.Neighborhood]: 1,
                [CommunityType.Guild]: 2,
                [CommunityType.StudyGroup]: 3,
                [CommunityType.Group]: 4,
            }
            const sortCommunities = (comms: CommunityWithReqs[]) => {
                return [...comms].sort((a, b) => {
                    const priorityA = typePriority[a.type] ?? 99
                    const priorityB = typePriority[b.type] ?? 99
                    if (priorityA !== priorityB) return priorityA - priorityB
                    return a.name.localeCompare(b.name)
                })
            }

            // Helper to get ALL badge requirements for a community (excluding CoC)
            // Uses EffectiveBadgeRequirement which already has badgeName
            const getNonCocBadgeRequirements = (community: CommunityWithReqs): Array<{ id: string; name: string }> => {
                const requirements = community.requirements || []
                return requirements
                    .filter(r => !cocBadge || r.badgeId !== cocBadge.id)
                    .filter(r => r.badgeSlug !== 'code-of-conduct') // Also filter by slug as fallback
                    .map(r => ({ id: r.badgeId, name: r.badgeName }))
            }

            // 2. Build Nodes and Edges
            let roots: CommunityWithReqs[]

            if (viewRoot) {
                // Drill-down view: start from the selected community
                const rootCommunity = communitiesWithReqs.find(c => c.id === viewRoot)
                roots = rootCommunity ? [rootCommunity] : []
            } else {
                // Main view: start from top-level communities
                roots = sortCommunities(
                    communitiesWithReqs.filter(c => !c.parent_community_id)
                )

                // Add CoC badge only in main view
                if (cocBadge) {
                    cocNodeId = `badge-${cocBadge.id}`
                    newNodes.push({
                        id: cocNodeId,
                        type: 'badge',
                        position: { x: 0, y: 0 },
                        data: { label: cocBadge.name },
                    })
                }
            }

            // Helper to process nodes recursively
            // isDirectViewRoot: true if this community IS the viewRoot (should show expanded content)
            const processNode = (community: CommunityWithReqs, isDirectViewRoot: boolean = false) => {
                const children = sortCommunities(
                    communitiesWithReqs.filter(c => c.parent_community_id === community.id)
                )

                // Separate children into:
                // - Group type communities - shown as collapsible "Group" cards (unless expanded)
                // - All other types (Zone, Neighborhood, Guild, StudyGroup) - shown as regular nodes
                const groupChildren = children.filter(c => c.type === CommunityType.Group)
                const regularChildren = children.filter(c => c.type !== CommunityType.Group)

                // Add this community as a node (with badges if any)
                const communityBadges = getNonCocBadgeRequirements(community)
                newNodes.push({
                    id: community.id,
                    type: 'community',
                    position: { x: 0, y: 0 },
                    data: {
                        label: community.name,
                        type: community.type,
                        badges: communityBadges.length > 0 ? communityBadges : undefined,
                        onBadgeClick: communityBadges.length > 0 ? handleBadgeClick : undefined,
                    },
                })

                // Connect CoC Badge to Root Zones (only in main view)
                if (!community.parent_community_id && cocNodeId && !viewRoot) {
                    newEdges.push({
                        id: `e-${cocNodeId}-${community.id}`,
                        source: cocNodeId,
                        target: community.id,
                        label: 'Required',
                        type: 'smoothstep',
                        animated: true,
                        style: { stroke: 'var(--primary)', strokeWidth: 2 },
                        markerEnd: { type: MarkerType.ArrowClosed, color: 'var(--primary)' },
                    })
                }

                // Process regular children (Zone, Neighborhood, Guild, StudyGroup) - always shown as nodes
                // Badges are now inside the child nodes, so we just add simple edges
                regularChildren.forEach(child => {
                    addSimpleEdge(community, child)
                    processNode(child, false)
                })

                // Process Group type children
                // - In main view (no viewRoot): show as collapsed "Group" card
                // - If this is the viewRoot OR we're in drill-down mode: expand to show contents
                groupChildren.forEach(groupChild => {
                    const shouldExpand = isDirectViewRoot || viewRoot === groupChild.id

                    if (!shouldExpand && !viewRoot) {
                        // Collapsed view: show as "Group" card
                        const groupNodeId = `group-${groupChild.id}`

                        // Get all badge requirements for this group
                        const groupBadges = getNonCocBadgeRequirements(groupChild)

                        newNodes.push({
                            id: groupNodeId,
                            type: 'group',
                            position: { x: 0, y: 0 },
                            data: {
                                label: groupChild.name,
                                badges: groupBadges.length > 0 ? groupBadges : undefined,
                                groupId: groupChild.id,
                                onDrillDown: handleDrillDown,
                                onBadgeClick: groupBadges.length > 0 ? handleBadgeClick : undefined,
                            } as GroupNodeData,
                        })

                        // Direct edge from parent to group node (no separate badge node - badge is inside the card now)
                        newEdges.push({
                            id: `e-${community.id}-${groupNodeId}`,
                            source: community.id,
                            target: groupNodeId,
                            type: 'smoothstep',
                            animated: true,
                            style: { stroke: 'var(--muted-foreground)', strokeDasharray: '5,5' },
                        })
                    } else {
                        // Expanded view: show the group as a regular community node and recurse
                        // Badges are inside the nodes, so just use simple edge
                        addSimpleEdge(community, groupChild)
                        processNode(groupChild, viewRoot === groupChild.id)
                    }
                })
            }

            // Helper to add a simple edge (no badge nodes - badges are inside community nodes now)
            const addSimpleEdge = (parent: CommunityWithReqs, child: CommunityWithReqs) => {
                newEdges.push({
                    id: `e-${parent.id}-${child.id}`,
                    source: parent.id,
                    target: child.id,
                    type: 'smoothstep',
                    animated: true,
                    style: { stroke: 'var(--muted-foreground)' },
                })
            }

            // Process roots - if we have a viewRoot, mark it as the direct view root
            roots.forEach(root => processNode(root, viewRoot === root.id))

            // Apply ELK Layout
            const layouted = await getLayoutedElements(newNodes, newEdges)
            setNodes(layouted.nodes)
            setEdges(layouted.edges)

            // Fit view after graph is built with a small delay to ensure render is complete
            setTimeout(() => {
                reactFlowInstance.fitView({ padding: 0.2, duration: 300 })
            }, 50)
        }

        buildGraph()

    }, [communities, badges, allRequirements, setNodes, setEdges, handleDrillDown, handleBadgeClick, viewRoot, reactFlowInstance])

    // Node click handler for selection
    const handleNodeClick = useCallback((_event: React.MouseEvent, node: Node) => {
        // Close badge panel when selecting a community
        setSelectedBadgeIds([])

        // Only handle community nodes, not badge nodes
        if (node.type === 'community') {
            setSelectedCommunityId(node.id)
        } else if (node.type === 'group') {
            // For group nodes, extract the actual community ID
            const communityId = (node.data as GroupNodeData).groupId
            if (communityId) {
                setSelectedCommunityId(communityId)
            }
        }
    }, [])

    // Handle click on pane (background) to deselect
    const handlePaneClick = useCallback(() => {
        setSelectedCommunityId(null)
        setSelectedBadgeIds([])
    }, [])

    // Get the selected community
    const selectedCommunity = useMemo(() => {
        if (!selectedCommunityId || !communities) return null
        return communities.find(c => c.id === selectedCommunityId) || null
    }, [selectedCommunityId, communities])

    // Get the selected badges (supports multiple)
    const selectedBadges = useMemo(() => {
        if (selectedBadgeIds.length === 0 || !badges) return []
        return badges.filter(b => selectedBadgeIds.includes(b.id))
    }, [selectedBadgeIds, badges])

    // Get requirements for selected community
    const selectedRequirements = useMemo(() => {
        if (!selectedCommunityId || !allRequirements) return []
        const found = allRequirements.find(r => r.communityId === selectedCommunityId)
        return found?.requirements || []
    }, [selectedCommunityId, allRequirements])

    // Check membership for selected community
    const { data: membershipData, isLoading: isLoadingMembership } = useQuery({
        queryKey: ['community-membership', selectedCommunityId],
        queryFn: () => selectedCommunityId ? communityService.getMembership(selectedCommunityId) : null,
        enabled: !!selectedCommunityId && !!user,
    })

    // Join mutation
    const joinMutation = useMutation({
        mutationFn: (communityId: string) => communityService.joinCommunity(communityId),
        onSuccess: () => {
            toast.success('Successfully joined community!')
            queryClient.invalidateQueries({ queryKey: ['community-membership', selectedCommunityId] })
            queryClient.invalidateQueries({ queryKey: ['communities'] })
        },
        onError: () => {
            toast.error('Failed to join community')
        },
    })

    // Leave mutation
    const leaveMutation = useMutation({
        mutationFn: (communityId: string) => communityService.leaveCommunity(communityId),
        onSuccess: () => {
            toast.success('Successfully left community')
            queryClient.invalidateQueries({ queryKey: ['community-membership', selectedCommunityId] })
            queryClient.invalidateQueries({ queryKey: ['communities'] })
        },
        onError: () => {
            toast.error('Failed to leave community')
        },
    })

    const handleJoin = useCallback(() => {
        if (selectedCommunityId) {
            joinMutation.mutate(selectedCommunityId)
        }
    }, [selectedCommunityId, joinMutation])

    const handleLeave = useCallback(() => {
        if (selectedCommunityId) {
            leaveMutation.mutate(selectedCommunityId)
        }
    }, [selectedCommunityId, leaveMutation])

    if (isLoadingCommunities || isLoadingBadges) {
        return (
            <div className="flex h-[600px] items-center justify-center rounded-lg border bg-muted/10">
                <Loader2 className="h-8 w-8 animate-spin text-primary" />
            </div>
        )
    }

    return (
        <div className="flex flex-col gap-2">
            {/* Breadcrumb Navigation - always visible with consistent height */}
            <div className="flex items-center gap-1 text-sm h-8">
                <Button
                    variant={breadcrumbs.length === 0 ? "secondary" : "ghost"}
                    size="sm"
                    onClick={() => handleBreadcrumbClick(-1)}
                    className="h-7 px-2"
                >
                    <Home className="h-4 w-4 mr-1" />
                    Root
                </Button>
                {breadcrumbs.map((crumb, index) => (
                    <div key={crumb.id} className="flex items-center">
                        <ChevronRight className="h-4 w-4 text-muted-foreground" />
                        <Button
                            variant={index === breadcrumbs.length - 1 ? "secondary" : "ghost"}
                            size="sm"
                            onClick={() => handleBreadcrumbClick(index)}
                            className="h-7 px-2"
                        >
                            {crumb.name}
                        </Button>
                    </div>
                ))}
            </div>

            <div className="h-[600px] lg:h-[800px] w-full rounded-lg border bg-background shadow-sm relative">
                <ReactFlow
                    nodes={nodes}
                    edges={edges}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    onNodeClick={handleNodeClick}
                    onPaneClick={handlePaneClick}
                    nodeTypes={nodeTypes}
                    fitView
                    colorMode={resolvedTheme === 'dark' ? 'dark' : 'light'}
                    attributionPosition="bottom-right"
                >
                    <MiniMap
                        nodeStrokeColor={(n) => {
                            if (n.type === 'badge') return '#e11d48';
                            if (n.type === 'community') return '#2563eb';
                            if (n.type === 'group') return '#d97706';
                            return '#64748b';
                        }}
                        nodeColor={(n) => {
                            if (n.type === 'badge') return '#fff1f2';
                            if (n.type === 'community') return '#eff6ff';
                            if (n.type === 'group') return '#fef3c7';
                            return '#f8fafc';
                        }}
                    />
                    <Controls />
                    <Background color={resolvedTheme === 'dark' ? '#334155' : '#94a3b8'} gap={16} size={1} />
                </ReactFlow>

                {/* Info Panel */}
                {selectedCommunity && badges && (
                    <InfoPanel
                        community={selectedCommunity}
                        requirements={selectedRequirements}
                        badges={badges}
                        isMember={!!membershipData?.is_member}
                        isLoading={isLoadingMembership || joinMutation.isPending || leaveMutation.isPending}
                        onClose={() => setSelectedCommunityId(null)}
                        onJoin={handleJoin}
                        onLeave={handleLeave}
                    />
                )}

                {/* Badge Info Panel */}
                {selectedBadges.length > 0 && (
                    <BadgeInfoPanel
                        badges={selectedBadges}
                        onClose={() => setSelectedBadgeIds([])}
                    />
                )}
            </div>
        </div>
    )
}
