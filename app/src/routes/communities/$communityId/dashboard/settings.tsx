import { createFileRoute, useNavigate, useParams } from '@tanstack/react-router'
import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { communityService, CommunityType, RequirementContext } from '@/api/community'
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
import { toast } from 'sonner'
import { Loader2, X, Check, ChevronsUpDown } from 'lucide-react'
import { useEffect } from 'react'

import { LocationPicker } from '@/components/communities/LocationPicker'

export const Route = createFileRoute('/communities/$communityId/dashboard/settings')({
    component: DashboardSettings,
})

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
            // If Neighborhood (Flower) or Zone (Structure), ensure ONLY CoC is present
            const nonCoc = currentReqs.filter(r => r.badge_id !== cocBadge.id)
            if (nonCoc.length > 0 || !hasCoc) {
                form.setValue('initial_requirements', [{
                    badge_id: cocBadge.id,
                    context: RequirementContext.Participate
                }])
            }
        } else {
            // If not physical types, just ensure CoC is present
            if (!hasCoc) {
                form.setValue('initial_requirements', [
                    ...currentReqs,
                    {
                        badge_id: cocBadge.id,
                        context: RequirementContext.Participate
                    }
                ])
            }
        }
    }, [badges, communityType, form])

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

    return (
        <div className="space-y-6">
            <div>
                <h3 className="text-lg font-medium">Settings</h3>
                <p className="text-sm text-muted-foreground">
                    Manage community settings and preferences.
                </p>
            </div>

            <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit, (errors) => console.error('Form errors:', errors))} className="space-y-6">
                    {/* Read-only Name */}
                    <div className="space-y-2">
                        <FormLabel>Name</FormLabel>
                        <Input value={community.name} disabled className="bg-muted" />
                        <FormDescription>
                            Community name cannot be changed.
                        </FormDescription>
                    </div>

                    <FormField
                        control={form.control as any}
                        name="description"
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>Description</FormLabel>
                                <FormControl>
                                    <Textarea
                                        placeholder="Describe the purpose of your community..."
                                        className="resize-none"
                                        {...field}
                                    />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />

                    <div className="grid gap-6 md:grid-cols-2">
                        {/* Read-only Type */}
                        <div className="space-y-2">
                            <FormLabel>Type</FormLabel>
                            <Input value={community.type} disabled className="bg-muted capitalize" />
                            <FormDescription className="min-h-10">
                                Community type cannot be changed.
                            </FormDescription>
                        </div>

                        <FormField
                            control={form.control as any}
                            name="parent_community_id"
                            render={({ field }) => {
                                // Filter logic
                                const filteredCommunities = communities?.filter(c => {
                                    // Don't allow selecting itself as parent
                                    if (c.id === community.id) return false

                                    if (communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood) {
                                        // Only Zone or Neighborhood parents
                                        return c.type === CommunityType.Zone || c.type === CommunityType.Neighborhood
                                    }
                                    // Guild/StudyGroup can have any parent
                                    return true
                                }) || []

                                return (
                                    <FormItem className="flex flex-col">
                                        <FormLabel>Parent Community {communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood ? '*' : '(Optional)'}</FormLabel>
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
                                                            ? filteredCommunities.find(
                                                                (c) => c.id === field.value
                                                            )?.name
                                                            : "Select a parent community"}
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
                                                            {filteredCommunities.map((c) => (
                                                                <CommandItem
                                                                    value={c.name}
                                                                    key={c.id}
                                                                    onSelect={() => {
                                                                        form.setValue("parent_community_id", c.id)
                                                                    }}
                                                                >
                                                                    <Check
                                                                        className={cn(
                                                                            "mr-2 h-4 w-4",
                                                                            c.id === field.value
                                                                                ? "opacity-100"
                                                                                : "opacity-0"
                                                                        )}
                                                                    />
                                                                    {c.name} ({c.type})
                                                                </CommandItem>
                                                            ))}
                                                        </CommandGroup>
                                                    </CommandList>
                                                </Command>
                                            </PopoverContent>
                                        </Popover>
                                        <FormDescription className="min-h-10">
                                            The parent community this community belongs to.
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )
                            }}
                        />
                    </div>

                    {(communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood) && (
                        <div className="space-y-4 rounded-lg border p-4">
                            <div className="space-y-1">
                                <h3 className="text-lg font-medium">Location & Coverage</h3>
                                <p className="text-sm text-muted-foreground">
                                    Update the physical location and coverage area.
                                </p>
                            </div>
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

                    <div className="space-y-4">
                        <h3 className="text-lg font-medium">Required Badges</h3>
                        <p className="text-sm text-muted-foreground">
                            Select badges required to participate in this community.
                            {(communityType === CommunityType.Neighborhood || communityType === CommunityType.Zone) && " Zones and Neighborhoods are restricted to the Code of Conduct badge only."}
                        </p>

                        {/* Selected Badges List */}
                        <div className="flex flex-wrap gap-2">
                            {initialRequirements.map((req) => {
                                const badge = badges?.find(b => b.id === req.badge_id)
                                if (!badge) return null
                                const isCoc = badge.slug === 'code-of-conduct'

                                return (
                                    <Badge key={req.badge_id} variant="secondary" className="flex items-center gap-1 px-3 py-1">
                                        {badge.icon} {badge.name}
                                        {!isCoc && communityType !== CommunityType.Neighborhood && communityType !== CommunityType.Zone && (
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

                        {/* Badge Search */}
                        {communityType !== CommunityType.Neighborhood && communityType !== CommunityType.Zone && (
                            <Popover>
                                <PopoverTrigger asChild>
                                    <Button
                                        variant="outline"
                                        role="combobox"
                                        className="w-full justify-between"
                                    >
                                        Add a required badge...
                                        <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                                    </Button>
                                </PopoverTrigger>
                                <PopoverContent className="w-[400px] p-0">
                                    <Command>
                                        <CommandInput placeholder="Search badges..." />
                                        <CommandList>
                                            <CommandEmpty>No badge found.</CommandEmpty>
                                            <CommandGroup>
                                                {badges
                                                    ?.filter(b => !initialRequirements.some(r => r.badge_id === b.id))
                                                    .map((badge) => (
                                                        <CommandItem
                                                            value={badge.name}
                                                            key={badge.id}
                                                            onSelect={() => {
                                                                const current = form.getValues('initial_requirements') || []
                                                                form.setValue('initial_requirements', [
                                                                    ...current,
                                                                    {
                                                                        badge_id: badge.id,
                                                                        context: RequirementContext.Participate
                                                                    }
                                                                ])
                                                            }}
                                                        >
                                                            <Check
                                                                className={cn(
                                                                    "mr-2 h-4 w-4",
                                                                    initialRequirements.some(r => r.badge_id === badge.id)
                                                                        ? "opacity-100"
                                                                        : "opacity-0"
                                                                )}
                                                            />
                                                            <span className="mr-2">{badge.icon}</span>
                                                            {badge.name}
                                                        </CommandItem>
                                                    ))}
                                            </CommandGroup>
                                        </CommandList>
                                    </Command>
                                </PopoverContent>
                            </Popover>
                        )}
                    </div>

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
