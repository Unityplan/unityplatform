import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { communityService, CommunityType, RequirementContext, type EffectiveBadgeRequirement } from '@/api/community'
import { badgeService } from '@/api/badge'
import { useNavigate, Link } from '@tanstack/react-router'
import { Button } from '@/components/ui/button'
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { toast } from 'sonner'
import {
    Loader2,
    Home,
    X,
    Search,
    Plus,
    Check,
    ChevronsUpDown,
    Map as MapIcon,
    Users,
    Hammer,
    BookOpen,
    Package,
    Shield,
    Info,
} from 'lucide-react'
import { AppLayout } from '@/components/layouts/AppLayout'
import { PageContainer } from '@/components/layouts/PageContainer'
import { useState, useEffect, useMemo } from 'react'
import {
    Breadcrumb,
    BreadcrumbList,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbSeparator,
    BreadcrumbPage,
} from '@/components/ui/breadcrumb'
import { LocationPicker } from '@/components/communities/LocationPicker'
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from "@/components/ui/command"
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import { cn } from "@/lib/utils"
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Label } from '@/components/ui/label'

// Community type configuration with icons and descriptions
const communityTypeConfig = {
    [CommunityType.Zone]: {
        icon: MapIcon,
        label: 'Zone',
        description: 'A structural area (e.g., District, Region) for organization. No direct members.',
        color: 'text-blue-600 dark:text-blue-400',
    },
    [CommunityType.Neighborhood]: {
        icon: Users,
        label: 'Neighborhood',
        description: 'A local community where people gather and interact. Restricted to Code of Conduct.',
        color: 'text-green-600 dark:text-green-400',
    },
    [CommunityType.Guild]: {
        icon: Hammer,
        label: 'Guild',
        description: 'A community of practitioners focused on a specific craft or skill.',
        color: 'text-amber-600 dark:text-amber-400',
    },
    [CommunityType.StudyGroup]: {
        icon: BookOpen,
        label: 'Study Group',
        description: 'A group dedicated to learning a specific subject together.',
        color: 'text-purple-600 dark:text-purple-400',
    },
    [CommunityType.Group]: {
        icon: Package,
        label: 'Group',
        description: 'A container to organize related communities together. Can have badge requirements.',
        color: 'text-orange-600 dark:text-orange-400',
    },
}

const createCommunitySchema = z.object({
    name: z.string().min(3, 'Name must be at least 3 characters').max(100),
    description: z.string().optional(),
    community_type: z.enum([CommunityType.Zone, CommunityType.Neighborhood, CommunityType.Guild, CommunityType.StudyGroup, CommunityType.Group]),
    territory_id: z.string().optional(),
    parent_community_id: z.string().optional(),
    location_lat: z.number().optional(),
    location_lng: z.number().optional(),
    coverage_area: z.any().optional(),
    initial_requirements: z.array(z.object({
        badge_id: z.string(),
        context: z.enum([RequirementContext.View, RequirementContext.Participate, RequirementContext.Admin]),
    })).optional(),
}).superRefine((data, ctx) => {
    if ((data.community_type === CommunityType.Zone || data.community_type === CommunityType.Neighborhood)) {
        if (!data.parent_community_id) {
            ctx.addIssue({
                code: z.ZodIssueCode.custom,
                message: "Parent community is required for Zones and Neighborhoods",
                path: ["parent_community_id"],
            });
        }
    }
})

type CreateCommunityFormValues = z.infer<typeof createCommunitySchema>

export function CreateCommunityPage() {
    const navigate = useNavigate()
    const queryClient = useQueryClient()

    const { data: badges, isLoading: isLoadingBadges } = useQuery({
        queryKey: ['badges'],
        queryFn: badgeService.listBadges,
    })

    const { data: communities } = useQuery({
        queryKey: ['communities'],
        queryFn: () => communityService.listCommunities({}),
    })

    const form = useForm<CreateCommunityFormValues>({
        resolver: zodResolver(createCommunitySchema),
        defaultValues: {
            name: '',
            description: '',
            community_type: CommunityType.Guild,
            territory_id: 'dk',
            initial_requirements: [],
        },
    })

    const mutation = useMutation({
        mutationFn: communityService.createCommunity,
        onSuccess: (data) => {
            toast.success('Community created successfully')
            queryClient.invalidateQueries({ queryKey: ['communities'] })
            // @ts-expect-error - Route might not be generated yet
            navigate({ to: '/communities/$communityId', params: { communityId: data.id } })
        },
        onError: (error: Error) => {
            // @ts-expect-error - Axios error handling
            toast.error(error.response?.data?.message || 'Failed to create community')
        },
    })

    const [searchQuery, setSearchQuery] = useState('')
    const [geoJsonInput, setGeoJsonInput] = useState('')
    const [geoJsonError, setGeoJsonError] = useState<string | null>(null)
    const [locationInputMode, setLocationInputMode] = useState<'map' | 'geojson'>('map')

    const communityType = form.watch('community_type')
    const parentCommunityId = form.watch('parent_community_id')
    const initialRequirements = form.watch('initial_requirements') || []

    // Fetch inherited badges from parent community
    const { data: parentRequirements } = useQuery({
        queryKey: ['community-requirements', parentCommunityId],
        queryFn: () => parentCommunityId ? communityService.getEffectiveRequirements(parentCommunityId) : Promise.resolve([]),
        enabled: !!parentCommunityId,
    })

    // Deduplicate and filter inherited badges (exclude CoC)
    const inheritedBadges = useMemo(() => {
        if (!parentRequirements) return []
        const byBadge = new Map<string, EffectiveBadgeRequirement>()
        for (const req of parentRequirements) {
            if (req.badgeSlug === 'code-of-conduct') continue
            const existing = byBadge.get(req.badgeId)
            if (!existing || (!req.isInherited && existing.isInherited)) {
                byBadge.set(req.badgeId, req)
            }
        }
        return Array.from(byBadge.values())
    }, [parentRequirements])

    // Handle GeoJSON input
    const handleGeoJsonChange = (value: string) => {
        setGeoJsonInput(value)
        setGeoJsonError(null)

        if (!value.trim()) return

        try {
            const parsed = JSON.parse(value)

            // Support both FeatureCollection and direct Polygon/MultiPolygon
            let coordinates: number[][][] | undefined

            if (parsed.type === 'FeatureCollection' && parsed.features?.[0]?.geometry) {
                const geom = parsed.features[0].geometry
                if (geom.type === 'Polygon') {
                    coordinates = geom.coordinates
                } else if (geom.type === 'MultiPolygon') {
                    coordinates = geom.coordinates[0] // Use first polygon
                }
            } else if (parsed.type === 'Feature' && parsed.geometry) {
                const geom = parsed.geometry
                if (geom.type === 'Polygon') {
                    coordinates = geom.coordinates
                } else if (geom.type === 'MultiPolygon') {
                    coordinates = geom.coordinates[0]
                }
            } else if (parsed.type === 'Polygon') {
                coordinates = parsed.coordinates
            } else if (parsed.type === 'MultiPolygon') {
                coordinates = parsed.coordinates[0]
            }

            if (coordinates && coordinates[0]) {
                // Convert GeoJSON coordinates [lng, lat] to our format [lat, lng]
                const points = coordinates[0].map((coord: number[]) => ({
                    lat: coord[1],
                    lng: coord[0],
                }))

                // Calculate centroid
                const centroid = points.reduce(
                    (acc, p) => ({ lat: acc.lat + p.lat / points.length, lng: acc.lng + p.lng / points.length }),
                    { lat: 0, lng: 0 }
                )

                form.setValue('location_lat', centroid.lat)
                form.setValue('location_lng', centroid.lng)
                form.setValue('coverage_area', {
                    type: 'polygon',
                    coordinates: points,
                })

                toast.success('GeoJSON parsed successfully')
            } else {
                setGeoJsonError('Could not find valid Polygon coordinates')
            }
        } catch {
            setGeoJsonError('Invalid JSON format')
        }
    }

    // Effect to enforce Code of Conduct
    useEffect(() => {
        if (!badges) return

        const cocBadge = badges.find(b => b.slug === 'code-of-conduct')
        if (!cocBadge) return

        const currentReqs = form.getValues('initial_requirements') || []
        const hasCoc = currentReqs.some(r => r.badge_id === cocBadge.id)

        if (communityType === CommunityType.Neighborhood || communityType === CommunityType.Zone) {
            // Physical types: Only CoC allowed
            const nonCoc = currentReqs.filter(r => r.badge_id !== cocBadge.id)
            if (nonCoc.length > 0 || !hasCoc) {
                form.setValue('initial_requirements', [{
                    badge_id: cocBadge.id,
                    context: RequirementContext.Participate
                }])
            }
        } else {
            // Other types: Ensure CoC is present
            if (!hasCoc) {
                form.setValue('initial_requirements', [
                    ...currentReqs,
                    { badge_id: cocBadge.id, context: RequirementContext.Participate }
                ])
            }
        }
    }, [badges, communityType, form])

    function onSubmit(data: CreateCommunityFormValues) {
        mutation.mutate(data)
    }

    // Filter available communities for parent selection
    const filteredCommunities = useMemo(() => {
        if (!communities) return []
        return communities.filter(c => {
            if (communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood) {
                return c.type === CommunityType.Zone || c.type === CommunityType.Neighborhood
            }
            return true
        })
    }, [communities, communityType])

    const breadcrumbs = (
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink asChild>
                        <Link to="/"><Home className="size-4" /></Link>
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
                    <BreadcrumbPage>Create</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    )

    const TypeIcon = communityTypeConfig[communityType]?.icon || Users

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <PageContainer>
                <div className="mb-8">
                    <h1 className="text-3xl font-bold tracking-tight">Create Community</h1>
                    <p className="text-muted-foreground">
                        Start a new community to collaborate and share knowledge.
                    </p>
                </div>

                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
                        {/* Basic Info Card */}
                        <Card>
                            <CardHeader>
                                <CardTitle>Basic Information</CardTitle>
                                <CardDescription>Define the core identity of your community.</CardDescription>
                            </CardHeader>
                            <CardContent className="space-y-6">
                                <FormField
                                    control={form.control}
                                    name="name"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Name</FormLabel>
                                            <FormControl>
                                                <Input placeholder="e.g. Permaculture Guild Copenhagen" {...field} />
                                            </FormControl>
                                            <FormDescription>The display name of your community.</FormDescription>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                <FormField
                                    control={form.control}
                                    name="description"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Description</FormLabel>
                                            <FormControl>
                                                <Textarea
                                                    placeholder="Describe the purpose of your community..."
                                                    className="resize-none min-h-[100px]"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                <div className="grid gap-6 md:grid-cols-2">
                                    <FormField
                                        control={form.control}
                                        name="community_type"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Type</FormLabel>
                                                <Select onValueChange={field.onChange} defaultValue={field.value}>
                                                    <FormControl>
                                                        <SelectTrigger>
                                                            <SelectValue>
                                                                <div className="flex items-center gap-2">
                                                                    <TypeIcon className={cn("h-4 w-4", communityTypeConfig[communityType]?.color)} />
                                                                    <span>{communityTypeConfig[communityType]?.label}</span>
                                                                </div>
                                                            </SelectValue>
                                                        </SelectTrigger>
                                                    </FormControl>
                                                    <SelectContent>
                                                        {Object.entries(communityTypeConfig).map(([type, config]) => {
                                                            const Icon = config.icon
                                                            return (
                                                                <SelectItem key={type} value={type}>
                                                                    <div className="flex items-center gap-2">
                                                                        <Icon className={cn("h-4 w-4", config.color)} />
                                                                        <span>{config.label}</span>
                                                                    </div>
                                                                </SelectItem>
                                                            )
                                                        })}
                                                    </SelectContent>
                                                </Select>
                                                <FormDescription className="min-h-[40px]">
                                                    {communityTypeConfig[communityType]?.description}
                                                </FormDescription>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />

                                    <FormField
                                        control={form.control}
                                        name="parent_community_id"
                                        render={({ field }) => (
                                            <FormItem className="flex flex-col">
                                                <FormLabel>
                                                    Parent Community
                                                    {(communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood) && ' *'}
                                                </FormLabel>
                                                <Popover>
                                                    <PopoverTrigger asChild>
                                                        <FormControl>
                                                            <Button
                                                                variant="outline"
                                                                role="combobox"
                                                                className={cn(
                                                                    "w-full justify-between",
                                                                    !field.value && "text-muted-foreground"
                                                                )}
                                                            >
                                                                {field.value
                                                                    ? filteredCommunities.find(c => c.id === field.value)?.name
                                                                    : "Select parent community"}
                                                                <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                                                            </Button>
                                                        </FormControl>
                                                    </PopoverTrigger>
                                                    <PopoverContent className="w-[400px] p-0 z-[1100]">
                                                        <Command>
                                                            <CommandInput placeholder="Search community..." />
                                                            <CommandList>
                                                                <CommandEmpty>No community found.</CommandEmpty>
                                                                <CommandGroup>
                                                                    {/* Option to clear */}
                                                                    <CommandItem
                                                                        onSelect={() => form.setValue("parent_community_id", undefined)}
                                                                    >
                                                                        <Check className={cn("mr-2 h-4 w-4", !field.value ? "opacity-100" : "opacity-0")} />
                                                                        <span className="text-muted-foreground">No parent (top-level)</span>
                                                                    </CommandItem>
                                                                    {filteredCommunities.map((c) => {
                                                                        const config = communityTypeConfig[c.type]
                                                                        const Icon = config?.icon || Users
                                                                        return (
                                                                            <CommandItem
                                                                                value={c.name}
                                                                                key={c.id}
                                                                                onSelect={() => form.setValue("parent_community_id", c.id)}
                                                                            >
                                                                                <Check className={cn("mr-2 h-4 w-4", c.id === field.value ? "opacity-100" : "opacity-0")} />
                                                                                <Icon className={cn("mr-2 h-4 w-4", config?.color)} />
                                                                                {c.name}
                                                                            </CommandItem>
                                                                        )
                                                                    })}
                                                                </CommandGroup>
                                                            </CommandList>
                                                        </Command>
                                                    </PopoverContent>
                                                </Popover>
                                                <FormDescription className="min-h-[40px]">
                                                    {(communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood)
                                                        ? "Required for physical community types."
                                                        : "Optional parent to nest this community under."}
                                                </FormDescription>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                </div>
                            </CardContent>
                        </Card>

                        {/* Location Card - Only for physical types */}
                        {(communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood) && (
                            <Card>
                                <CardHeader>
                                    <CardTitle className="flex items-center gap-2">
                                        <MapIcon className="h-5 w-5" />
                                        Location & Coverage
                                    </CardTitle>
                                    <CardDescription>Define the physical location and coverage area.</CardDescription>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <Tabs value={locationInputMode} onValueChange={(v) => setLocationInputMode(v as 'map' | 'geojson')}>
                                        <TabsList>
                                            <TabsTrigger value="map">Interactive Map</TabsTrigger>
                                            <TabsTrigger value="geojson">GeoJSON Input</TabsTrigger>
                                        </TabsList>
                                        <TabsContent value="map" className="mt-4">
                                            <LocationPicker
                                                value={{
                                                    lat: form.watch('location_lat'),
                                                    lng: form.watch('location_lng'),
                                                    coverage: form.watch('coverage_area')
                                                }}
                                                onChange={(val) => {
                                                    form.setValue('location_lat', val.lat)
                                                    form.setValue('location_lng', val.lng)
                                                    form.setValue('coverage_area', val.coverage)
                                                }}
                                            />
                                        </TabsContent>
                                        <TabsContent value="geojson" className="mt-4 space-y-4">
                                            <div className="space-y-2">
                                                <Label>Paste GeoJSON</Label>
                                                <Textarea
                                                    placeholder='{"type": "Polygon", "coordinates": [[[lng, lat], ...]]}'
                                                    className="font-mono text-xs min-h-[150px]"
                                                    value={geoJsonInput}
                                                    onChange={(e) => handleGeoJsonChange(e.target.value)}
                                                />
                                                {geoJsonError && (
                                                    <p className="text-sm text-destructive">{geoJsonError}</p>
                                                )}
                                                <p className="text-xs text-muted-foreground">
                                                    Supports Polygon, MultiPolygon, Feature, or FeatureCollection.
                                                </p>
                                            </div>
                                            {/* Preview map for GeoJSON */}
                                            {form.watch('coverage_area') && (
                                                <div className="space-y-2">
                                                    <Label>Preview</Label>
                                                    <LocationPicker
                                                        value={{
                                                            lat: form.watch('location_lat'),
                                                            lng: form.watch('location_lng'),
                                                            coverage: form.watch('coverage_area')
                                                        }}
                                                        onChange={(val) => {
                                                            form.setValue('location_lat', val.lat)
                                                            form.setValue('location_lng', val.lng)
                                                            form.setValue('coverage_area', val.coverage)
                                                        }}
                                                    />
                                                </div>
                                            )}
                                        </TabsContent>
                                    </Tabs>
                                </CardContent>
                            </Card>
                        )}

                        {/* Badge Requirements Card */}
                        <Card>
                            <CardHeader>
                                <CardTitle className="flex items-center gap-2">
                                    <Shield className="h-5 w-5" />
                                    Badge Requirements
                                </CardTitle>
                                <CardDescription>
                                    {(communityType === CommunityType.Neighborhood || communityType === CommunityType.Zone)
                                        ? "Zones and Neighborhoods only require the Code of Conduct badge."
                                        : "Select badges required to join this community."}
                                </CardDescription>
                            </CardHeader>
                            <CardContent className="space-y-4">
                                {/* Inherited badges from parent */}
                                {inheritedBadges.length > 0 && (
                                    <div className="rounded-lg border border-dashed p-4 bg-muted/30">
                                        <div className="flex items-center gap-2 mb-3">
                                            <Info className="h-4 w-4 text-muted-foreground" />
                                            <span className="text-sm font-medium">Inherited from Parent</span>
                                        </div>
                                        <div className="flex flex-wrap gap-2">
                                            {inheritedBadges.map((req) => (
                                                <Badge key={req.badgeId} variant="outline" className="text-xs">
                                                    <Shield className="mr-1 h-3 w-3" />
                                                    {req.badgeName}
                                                    <span className="ml-1 text-muted-foreground">
                                                        (from {req.sourceCommunityName})
                                                    </span>
                                                </Badge>
                                            ))}
                                        </div>
                                        <p className="text-xs text-muted-foreground mt-2">
                                            These badges are automatically required. You can add additional badges below.
                                        </p>
                                    </div>
                                )}

                                {isLoadingBadges ? (
                                    <div className="flex items-center justify-center p-4">
                                        <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
                                    </div>
                                ) : (
                                    <>
                                        {/* Selected Badges List */}
                                        <div className="flex flex-wrap gap-2">
                                            {initialRequirements.map((req) => {
                                                const badge = badges?.find(b => b.id === req.badge_id)
                                                if (!badge) return null
                                                const isCoc = badge.slug === 'code-of-conduct'
                                                const isPhysicalType = communityType === CommunityType.Neighborhood || communityType === CommunityType.Zone

                                                return (
                                                    <Badge key={req.badge_id} variant="secondary" className="flex items-center gap-1 px-3 py-1">
                                                        {badge.icon} {badge.name}
                                                        {!isCoc && !isPhysicalType && (
                                                            <button
                                                                type="button"
                                                                onClick={() => {
                                                                    const current = form.getValues('initial_requirements') || []
                                                                    form.setValue('initial_requirements', current.filter(r => r.badge_id !== req.badge_id))
                                                                }}
                                                                className="ml-1 rounded-full hover:bg-muted p-0.5"
                                                            >
                                                                <X className="h-3 w-3" />
                                                            </button>
                                                        )}
                                                    </Badge>
                                                )
                                            })}
                                        </div>

                                        {/* Badge Search - only for non-physical types */}
                                        {communityType !== CommunityType.Neighborhood && communityType !== CommunityType.Zone && (
                                            <div className="relative">
                                                <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
                                                <Input
                                                    placeholder="Search badges to add..."
                                                    value={searchQuery}
                                                    onChange={(e) => setSearchQuery(e.target.value)}
                                                    className="pl-8"
                                                />
                                                {searchQuery && (
                                                    <div className="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md border bg-popover p-1 shadow-md">
                                                        {badges
                                                            ?.filter(b =>
                                                                b.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
                                                                !initialRequirements.some(r => r.badge_id === b.id) &&
                                                                !inheritedBadges.some(ib => ib.badgeId === b.id)
                                                            )
                                                            .map(badge => (
                                                                <Button
                                                                    key={badge.id}
                                                                    variant="ghost"
                                                                    className="w-full justify-start font-normal"
                                                                    onClick={() => {
                                                                        const current = form.getValues('initial_requirements') || []
                                                                        form.setValue('initial_requirements', [
                                                                            ...current,
                                                                            { badge_id: badge.id, context: RequirementContext.Participate }
                                                                        ])
                                                                        setSearchQuery('')
                                                                    }}
                                                                >
                                                                    <Plus className="mr-2 h-4 w-4" />
                                                                    <span className="mr-2">{badge.icon}</span>
                                                                    {badge.name}
                                                                </Button>
                                                            ))}
                                                        {badges?.filter(b =>
                                                            b.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
                                                            !initialRequirements.some(r => r.badge_id === b.id) &&
                                                            !inheritedBadges.some(ib => ib.badgeId === b.id)
                                                        ).length === 0 && (
                                                                <div className="p-2 text-sm text-muted-foreground text-center">
                                                                    No matching badges found
                                                                </div>
                                                            )}
                                                    </div>
                                                )}
                                            </div>
                                        )}
                                    </>
                                )}
                            </CardContent>
                        </Card>

                        {/* Hidden territory_id field */}
                        <input type="hidden" {...form.register('territory_id')} />

                        {/* Actions */}
                        <div className="flex justify-end gap-4">
                            <Button variant="outline" asChild>
                                <Link to="/communities">Cancel</Link>
                            </Button>
                            <Button type="submit" disabled={mutation.isPending}>
                                {mutation.isPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                                Create Community
                            </Button>
                        </div>
                    </form>
                </Form>
            </PageContainer>
        </AppLayout>
    )
}
