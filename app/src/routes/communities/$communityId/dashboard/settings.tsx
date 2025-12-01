import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { communityService, CommunityType, RequirementContext, type EffectiveBadgeRequirement } from '@/api/community'
import { badgeService } from '@/api/badge'
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
import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Label } from '@/components/ui/label'
import { toast } from 'sonner'
import {
    Loader2,
    X,
    Check,
    ChevronsUpDown,
    Map as MapIcon,
    Users,
    Hammer,
    BookOpen,
    Package,
    Shield,
    Info,
    Search,
    Plus,
    Lock,
} from 'lucide-react'
import { useEffect, useMemo, useState } from 'react'
import type { LucideIcon } from 'lucide-react'

import { LocationPicker } from '@/components/communities/LocationPicker'

export const Route = createFileRoute('/communities/$communityId/dashboard/settings')({
    component: DashboardSettings,
})

// Community type configuration with icons and descriptions
const communityTypeConfig: Record<CommunityType, { icon: LucideIcon; label: string; description: string; color: string }> = {
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

const editCommunitySchema = z.object({
    description: z.string().optional(),
    parent_community_id: z.string().optional().nullable(),
    location_lat: z.number().optional().nullable(),
    location_lng: z.number().optional().nullable(),
    coverage_area: z.any().optional().nullable(),
    initial_requirements: z.array(z.object({
        badge_id: z.string(),
        context: z.enum([RequirementContext.View, RequirementContext.Participate, RequirementContext.Admin]),
    })).optional(),
})

type EditCommunityFormValues = z.infer<typeof editCommunitySchema>

function DashboardSettings() {
    const { communityId } = Route.useParams()
    const navigate = useNavigate()
    const queryClient = useQueryClient()

    const [searchQuery, setSearchQuery] = useState('')
    const [geoJsonInput, setGeoJsonInput] = useState('')
    const [geoJsonError, setGeoJsonError] = useState<string | null>(null)
    const [locationInputMode, setLocationInputMode] = useState<'map' | 'geojson'>('map')

    const { data: community, isLoading: isLoadingCommunity } = useQuery({
        queryKey: ['community', communityId],
        queryFn: () => communityService.getCommunity(communityId),
    })

    const { data: badges, isLoading: isLoadingBadges } = useQuery({
        queryKey: ['badges'],
        queryFn: badgeService.listBadges,
    })

    const { data: communities } = useQuery({
        queryKey: ['communities'],
        queryFn: () => communityService.listCommunities({}),
    })

    const { data: requirements, isLoading: isLoadingRequirements } = useQuery({
        queryKey: ['community-requirements', communityId],
        queryFn: () => communityService.listRequirements(communityId),
        enabled: !!communityId,
    })

    // Fetch inherited badges from parent community
    const parentCommunityId = community?.parent_community_id
    const { data: parentRequirements } = useQuery({
        queryKey: ['community-effective-requirements', parentCommunityId],
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

    const form = useForm<EditCommunityFormValues>({
        resolver: zodResolver(editCommunitySchema),
        defaultValues: {
            description: '',
            parent_community_id: '',
            initial_requirements: [],
        },
    })

    // Populate form when data is loaded
    useEffect(() => {
        if (community && requirements) {
            form.reset({
                description: community.description || '',
                parent_community_id: community.parent_community_id || '',
                location_lat: community.location_lat,
                location_lng: community.location_lng,
                coverage_area: community.coverage_area,
                initial_requirements: requirements.map(r => ({
                    badge_id: r.badge_id,
                    context: r.context,
                })),
            })
        }
    }, [community, requirements, form])

    // Handle GeoJSON input
    const handleGeoJsonChange = (value: string) => {
        setGeoJsonInput(value)
        setGeoJsonError(null)

        if (!value.trim()) return

        try {
            const parsed = JSON.parse(value)
            let coordinates: number[][][] | undefined

            if (parsed.type === 'FeatureCollection' && parsed.features?.[0]?.geometry) {
                const geom = parsed.features[0].geometry
                if (geom.type === 'Polygon') {
                    coordinates = geom.coordinates
                } else if (geom.type === 'MultiPolygon') {
                    coordinates = geom.coordinates[0]
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
                const points = coordinates[0].map((coord: number[]) => ({
                    lat: coord[1],
                    lng: coord[0],
                }))

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

    const mutation = useMutation({
        mutationFn: async (data: EditCommunityFormValues) => {
            // 1. Update community details
            await communityService.updateCommunity(communityId, {
                description: data.description,
                parent_community_id: data.parent_community_id || undefined,
                location_lat: data.location_lat === null ? undefined : data.location_lat,
                location_lng: data.location_lng === null ? undefined : data.location_lng,
                coverage_area: data.coverage_area === null ? undefined : data.coverage_area,
            })

            // 2. Sync requirements (naive implementation: remove all, add new)
            if (!requirements) return

            const currentReqs = requirements
            const newReqs = data.initial_requirements || []

            // Remove requirements that are no longer present
            for (const req of currentReqs) {
                if (!newReqs.find(r => r.badge_id === req.badge_id)) {
                    await communityService.removeRequirement(communityId, req.badge_id, req.context)
                }
            }

            // Add new requirements that were not present
            for (const req of newReqs) {
                if (!currentReqs.find(r => r.badge_id === req.badge_id)) {
                    await communityService.addRequirement(communityId, {
                        badge_id: req.badge_id,
                        context: req.context,
                    })
                }
            }
        },
        onSuccess: () => {
            toast.success('Community updated successfully')
            queryClient.invalidateQueries({ queryKey: ['community', communityId] })
            queryClient.invalidateQueries({ queryKey: ['community-requirements', communityId] })
        },
        onError: (error: Error) => {
            // @ts-expect-error - Axios error handling
            toast.error(error.response?.data?.message || 'Failed to update community')
        },
    })

    const communityType = community?.type
    const initialRequirements = form.watch('initial_requirements') || []

    // Effect to enforce Code of Conduct and Physical type restrictions
    useEffect(() => {
        if (!badges || !communityType) return

        const cocBadge = badges.find(b => b.slug === 'code-of-conduct')
        if (!cocBadge) return

        const currentReqs = form.getValues('initial_requirements') || []
        const hasCoc = currentReqs.some(r => r.badge_id === cocBadge.id)

        if (communityType === CommunityType.Neighborhood || communityType === CommunityType.Zone) {
            const nonCoc = currentReqs.filter(r => r.badge_id !== cocBadge.id)
            if (nonCoc.length > 0 || !hasCoc) {
                form.setValue('initial_requirements', [{
                    badge_id: cocBadge.id,
                    context: RequirementContext.Participate
                }])
            }
        } else {
            if (!hasCoc) {
                form.setValue('initial_requirements', [
                    ...currentReqs,
                    { badge_id: cocBadge.id, context: RequirementContext.Participate }
                ])
            }
        }
    }, [badges, communityType, form])

    // Filter available communities for parent selection
    const filteredCommunities = useMemo(() => {
        if (!communities || !community) return []
        return communities.filter(c => {
            // Don't allow selecting itself as parent
            if (c.id === community.id) return false
            if (communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood) {
                return c.type === CommunityType.Zone || c.type === CommunityType.Neighborhood
            }
            return true
        })
    }, [communities, community, communityType])

    function onSubmit(data: EditCommunityFormValues) {
        mutation.mutate(data)
    }

    if (isLoadingCommunity || isLoadingRequirements || isLoadingBadges) {
        return (
            <div className="flex h-[50vh] items-center justify-center">
                <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
        )
    }

    if (!community) {
        return (
            <div className="rounded-lg border border-destructive/50 bg-destructive/10 p-4 text-destructive">
                Community not found
            </div>
        )
    }

    const TypeIcon = communityTypeConfig[communityType!]?.icon || Users

    return (
        <div className="space-y-6">
            <div>
                <h3 className="text-lg font-medium">Settings</h3>
                <p className="text-sm text-muted-foreground">
                    Manage community settings and preferences.
                </p>
            </div>

            <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit, (errors) => console.error('Form errors:', errors))} className="space-y-8">
                    {/* Basic Info Card */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Basic Information</CardTitle>
                            <CardDescription>Core identity of the community.</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-6">
                            {/* Read-only Name */}
                            <div className="space-y-2">
                                <FormLabel className="flex items-center gap-2">
                                    Name
                                    <Lock className="h-3 w-3 text-muted-foreground" />
                                </FormLabel>
                                <Input value={community.name} disabled className="bg-muted" />
                                <FormDescription>
                                    Community name cannot be changed.
                                </FormDescription>
                            </div>

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
                                {/* Read-only Type with Icon */}
                                <div className="space-y-2">
                                    <FormLabel className="flex items-center gap-2">
                                        Type
                                        <Lock className="h-3 w-3 text-muted-foreground" />
                                    </FormLabel>
                                    <div className="flex items-center gap-2 h-10 px-3 rounded-md border bg-muted">
                                        <TypeIcon className={cn("h-4 w-4", communityTypeConfig[communityType!]?.color)} />
                                        <span>{communityTypeConfig[communityType!]?.label}</span>
                                    </div>
                                    <FormDescription className="min-h-[40px]">
                                        {communityTypeConfig[communityType!]?.description}
                                    </FormDescription>
                                </div>

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
                                                                    onSelect={() => form.setValue("parent_community_id", null)}
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
                                <CardDescription>Update the physical location and coverage area.</CardDescription>
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
                                                lat: form.watch('location_lat') ?? undefined,
                                                lng: form.watch('location_lng') ?? undefined,
                                                coverage: form.watch('coverage_area') ?? undefined
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
                                                        lat: form.watch('location_lat') ?? undefined,
                                                        lng: form.watch('location_lng') ?? undefined,
                                                        coverage: form.watch('coverage_area') ?? undefined
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
                        </CardContent>
                    </Card>

                    {/* Actions */}
                    <div className="flex justify-end gap-4">
                        <Button type="submit" disabled={mutation.isPending}>
                            {mutation.isPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                            Save Changes
                        </Button>
                    </div>
                </form>
            </Form>
        </div>
    )
}
