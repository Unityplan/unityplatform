import { useCallback, useEffect, useState, useMemo } from 'react'
import {
    ReactFlow,
    MiniMap,
    Controls,
    Background,
    useNodesState,
    useEdgesState,
    addEdge,
    Handle,
    Position,
    type Node,
    type Edge,
    type Connection,
    Panel,
    MarkerType,
} from '@xyflow/react'
import '@xyflow/react/dist/style.css'
import { BookOpen, Users, Hammer, Plus, Loader2, Map as MapIcon } from 'lucide-react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { communityService, CommunityType } from '@/api/community'
import { Button } from '@/components/ui/button'
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { toast } from 'sonner'
import { useTheme } from 'next-themes'

// --- Custom Node Components ---

const CommunityNode = ({ data }: { data: { label: string; type: CommunityType } }) => {
    const Icon = useMemo(() => {
        switch (data.type) {
            case CommunityType.Zone: return MapIcon
            case CommunityType.Neighborhood: return Users
            case CommunityType.Guild: return Hammer
            case CommunityType.StudyGroup: return BookOpen
            default: return Users
        }
    }, [data.type])

    const colorClass = useMemo(() => {
        switch (data.type) {
            case CommunityType.Zone: return "border-blue-500 bg-blue-50 dark:bg-blue-950/30"
            case CommunityType.Neighborhood: return "border-green-500 bg-green-50 dark:bg-green-950/30"
            case CommunityType.Guild: return "border-amber-500 bg-amber-50 dark:bg-amber-950/30"
            case CommunityType.StudyGroup: return "border-purple-500 bg-purple-50 dark:bg-purple-950/30"
            default: return "border-gray-500 bg-gray-50"
        }
    }, [data.type])

    const iconColorClass = useMemo(() => {
        switch (data.type) {
            case CommunityType.Zone: return "text-blue-600 dark:text-blue-400"
            case CommunityType.Neighborhood: return "text-green-600 dark:text-green-400"
            case CommunityType.Guild: return "text-amber-600 dark:text-amber-400"
            case CommunityType.StudyGroup: return "text-purple-600 dark:text-purple-400"
            default: return "text-gray-600"
        }
    }, [data.type])

    return (
        <div className={`flex min-w-40 flex-col items-center gap-2 rounded-lg border-2 p-3 shadow-sm transition-all hover:shadow-md ${colorClass}`}>
            <div className={`flex h-10 w-10 items-center justify-center rounded-full bg-background/80 ${iconColorClass}`}>
                <Icon className="h-6 w-6" />
            </div>
            <div className="flex flex-col items-center text-center">
                <span className="text-sm font-bold">{data.label}</span>
                <span className="text-[10px] uppercase tracking-wider opacity-70">{data.type}</span>
            </div>
            <Handle type="target" position={Position.Top} className="bg-muted-foreground!" />
            <Handle type="source" position={Position.Bottom} className="bg-muted-foreground!" />
        </div>
    )
}

const nodeTypes = {
    central: CommunityNode,
    child: CommunityNode,
}

interface DashboardNodeFlowProps {
    communityId: string
}

export function DashboardNodeFlow({ communityId }: DashboardNodeFlowProps) {
    const { resolvedTheme } = useTheme()
    const queryClient = useQueryClient()
    const [nodes, setNodes, onNodesChange] = useNodesState<Node>([])
    const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([])

    // Create Dialog State
    const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false)
    const [createType, setCreateType] = useState<CommunityType | null>(null)
    const [newName, setNewName] = useState('')
    const [newDescription, setNewDescription] = useState('')

    const { data: community } = useQuery({
        queryKey: ['community', communityId],
        queryFn: () => communityService.getCommunity(communityId),
    })

    const { data: childCommunities } = useQuery({
        queryKey: ['communities', 'children', communityId],
        queryFn: () => communityService.listCommunities({ parent_id: communityId }),
    })

    const createMutation = useMutation({
        mutationFn: async () => {
            if (!createType) return
            await communityService.createCommunity({
                name: newName,
                description: newDescription,
                community_type: createType,
                parent_community_id: communityId,
                inherit_requirements: true,
            })
        },
        onSuccess: () => {
            toast.success(`${createType} created successfully`)
            setIsCreateDialogOpen(false)
            setNewName('')
            setNewDescription('')
            queryClient.invalidateQueries({ queryKey: ['communities', 'children', communityId] })
        },
        onError: (error: Error) => {
            // @ts-expect-error - Axios error handling
            toast.error(error.response?.data?.message || 'Failed to create community')
        }
    })

    useEffect(() => {
        if (!community) return

        // Center Node: The Community Itself
        const centerNode: Node = {
            id: community.id,
            type: 'central',
            position: { x: 400, y: 50 },
            data: { label: community.name, type: community.type },
        }

        const newNodes: Node[] = [centerNode]
        const newEdges: Edge[] = []

        // Add Child Communities (Guilds, Study Groups, etc.)
        if (childCommunities) {
            const radius = 400
            const count = childCommunities.length

            childCommunities.forEach((child, index) => {
                // Calculate position in an arc below the parent (from 30 degrees to 150 degrees)
                // 0 degrees is right (0), 90 is down (PI/2), 180 is left (PI)
                // We want roughly PI/6 (30deg) to 5*PI/6 (150deg)

                let angle;
                if (count === 1) {
                    angle = Math.PI / 2; // Directly below
                } else {
                    const startAngle = Math.PI / 4; // 45 deg
                    const endAngle = (3 * Math.PI) / 4; // 135 deg
                    const range = endAngle - startAngle;
                    angle = startAngle + (index / (count - 1)) * range;
                }

                const x = 400 + Math.cos(angle) * radius
                const y = 50 + Math.sin(angle) * radius

                newNodes.push({
                    id: child.id,
                    type: 'child',
                    position: { x, y },
                    data: { label: child.name, type: child.type },
                })

                newEdges.push({
                    id: `edge-${community.id}-${child.id}`,
                    source: community.id,
                    target: child.id,
                    type: 'default',
                    animated: true,
                    style: { stroke: 'var(--muted-foreground)', strokeWidth: 2 },
                    markerEnd: {
                        type: MarkerType.ArrowClosed,
                        color: 'var(--muted-foreground)',
                    },
                })
            })
        }

        setNodes(newNodes)
        setEdges(newEdges)
    }, [community, childCommunities, setNodes, setEdges])

    const onConnect = useCallback(
        (params: Connection) => setEdges((eds) => addEdge(params, eds)),
        [setEdges],
    )

    const handleAddChild = (type: CommunityType) => {
        setCreateType(type)
        setNewName('')
        setNewDescription('')
        setIsCreateDialogOpen(true)
    }

    const handleCreateSubmit = () => {
        if (!newName) {
            toast.error("Name is required")
            return
        }
        createMutation.mutate()
    }

    return (
        <div className="h-[900px] w-full rounded-lg border bg-background relative">
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                onConnect={onConnect}
                nodeTypes={nodeTypes}
                fitView
                fitViewOptions={{ padding: 0.2, maxZoom: 1 }}
                colorMode={resolvedTheme === 'dark' ? 'dark' : 'light'}
            >
                <Controls />
                <MiniMap
                    nodeStrokeColor={(n) => {
                        if (n.type === 'badge') return '#e11d48';
                        if (n.type === 'central' || n.type === 'child') return '#2563eb';
                        return '#64748b';
                    }}
                    nodeColor={(n) => {
                        if (n.type === 'badge') return '#fff1f2';
                        if (n.type === 'central' || n.type === 'child') return '#eff6ff';
                        return '#f8fafc';
                    }}
                />
                <Background color={resolvedTheme === 'dark' ? '#334155' : '#94a3b8'} gap={16} size={1} />
                <Panel position="top-right" className="flex gap-2 bg-background/80 p-2 rounded-lg border backdrop-blur-sm">
                    <Button size="sm" variant="outline" onClick={() => handleAddChild(CommunityType.Guild)}>
                        <Plus className="mr-2 h-4 w-4" />
                        Add Guild
                    </Button>
                    <Button size="sm" variant="outline" onClick={() => handleAddChild(CommunityType.StudyGroup)}>
                        <Plus className="mr-2 h-4 w-4" />
                        Add Study Group
                    </Button>
                </Panel>
            </ReactFlow>

            <Dialog open={isCreateDialogOpen} onOpenChange={setIsCreateDialogOpen}>
                <DialogContent className="sm:max-w-[425px]">
                    <DialogHeader>
                        <DialogTitle>Create {createType}</DialogTitle>
                        <DialogDescription>
                            Add a new {createType} to this community. It will inherit requirements from the parent.
                        </DialogDescription>
                    </DialogHeader>
                    <div className="grid gap-4 py-4">
                        <div className="grid gap-2">
                            <Label htmlFor="name">Name</Label>
                            <Input
                                id="name"
                                value={newName}
                                onChange={(e) => setNewName(e.target.value)}
                                placeholder={`e.g., ${createType === CommunityType.Guild ? 'Rust Developers' : 'Rust Basics'}`}
                            />
                        </div>
                        <div className="grid gap-2">
                            <Label htmlFor="description">Description</Label>
                            <Textarea
                                id="description"
                                value={newDescription}
                                onChange={(e) => setNewDescription(e.target.value)}
                                placeholder="Briefly describe the purpose..."
                            />
                        </div>
                    </div>
                    <DialogFooter>
                        <Button variant="outline" onClick={() => setIsCreateDialogOpen(false)}>Cancel</Button>
                        <Button onClick={handleCreateSubmit} disabled={createMutation.isPending}>
                            {createMutation.isPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                            Create
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    )
}
