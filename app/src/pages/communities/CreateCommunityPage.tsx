import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { communityService, CommunityType, RequirementContext } from '@/api/community'
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
import { Switch } from '@/components/ui/switch'
import { Badge } from '@/components/ui/badge'
import { toast } from 'sonner'
import {
    Loader2,
    Home,
    X,
    Search,
    Plus,
    Check,
    ChevronsUpDown
} from 'lucide-react'
import { AppLayout } from '@/components/layouts/AppLayout'
import { useState, useEffect } from 'react'
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

const createCommunitySchema = z.object({
    name: z.string().min(3, 'Name must be at least 3 characters').max(100),
    description: z.string().optional(),
    community_type: z.enum([CommunityType.Zone, CommunityType.Neighborhood, CommunityType.Guild, CommunityType.StudyGroup, CommunityType.Group]),
    territory_id: z.string().optional(),
    parent_community_id: z.string().optional(),
    location_lat: z.number().optional(),
    location_lng: z.number().optional(),
    coverage_area: z.any().optional(), // Using any for complex object, could be refined
    inherit_requirements: z.boolean().default(true),
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
        // Optional: Enforce location for physical communities?
        // For now, let's keep it optional but recommended
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
            territory_id: 'dk', // Default to DK for MVP
            inherit_requirements: true,
            initial_requirements: [],
        },
    })

    const mutation = useMutation({
        mutationFn: communityService.createCommunity,
        onSuccess: (data) => {
            toast.success('Community created successfully')
            queryClient.invalidateQueries({ queryKey: ['communities'] })
            // @ts-expect-error - Route not yet generated
            navigate({ to: '/communities/$communityId', params: { communityId: data.id } })
        },
        onError: (error: Error) => {
            // @ts-expect-error - Axios error handling
            toast.error(error.response?.data?.message || 'Failed to create community')
        },
    })

    const [searchQuery, setSearchQuery] = useState('')
    const communityType = form.watch('community_type')
    const initialRequirements = form.watch('initial_requirements') || []

    // Effect to enforce Code of Conduct and Physical type restrictions
    useEffect(() => {
        if (!badges) return

        const cocBadge = badges.find(b => b.slug === 'code-of-conduct')
        if (!cocBadge) return

        const currentReqs = form.getValues('initial_requirements') || []
        const hasCoc = currentReqs.some(r => r.badge_id === cocBadge.id)

        if (communityType === CommunityType.Neighborhood || communityType === CommunityType.Zone) {
            // If Neighborhood (Flower) or Zone (Structure), ensure ONLY CoC is present
            // (Zone also needs CoC as per requirements, even if it has no members, it sets the standard)
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

    function onSubmit(data: CreateCommunityFormValues) {
        mutation.mutate(data)
    }

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
                    <BreadcrumbPage>Create</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    )

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <div className="container mx-auto max-w-2xl py-8">
                <div className="mb-8">
                    <h1 className="text-3xl font-bold tracking-tight">Create Community</h1>
                    <p className="text-muted-foreground">
                        Start a new community to collaborate and share knowledge.
                    </p>
                </div>

                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
                        <FormField
                            control={form.control as any}
                            name="name"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Name</FormLabel>
                                    <FormControl>
                                        <Input placeholder="e.g. Permaculture Guild Copenhagen" {...field} />
                                    </FormControl>
                                    <FormDescription>
                                        The display name of your community.
                                    </FormDescription>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <FormField
                            control={form.control as any}
                            name="description"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Description</FormLabel>
                                    <FormControl>
                                        <Textarea
                                            placeholder="Describe the purpose of your community and how it contributes to the ecosystem..."
                                            className="resize-none"
                                            {...field}
                                        />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <div className="grid gap-6 md:grid-cols-2">
                            <FormField
                                control={form.control as any}
                                name="community_type"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Type</FormLabel>
                                        <Select onValueChange={field.onChange} defaultValue={field.value}>
                                            <FormControl>
                                                <SelectTrigger>
                                                    <SelectValue placeholder="Select a community type" />
                                                </SelectTrigger>
                                            </FormControl>
                                            <SelectContent>
                                                <SelectItem value={CommunityType.Zone}>Zone (Structure)</SelectItem>
                                                <SelectItem value={CommunityType.Neighborhood}>Neighborhood (Community)</SelectItem>
                                                <SelectItem value={CommunityType.Guild}>Guild (Skill-based)</SelectItem>
                                                <SelectItem value={CommunityType.StudyGroup}>Study Group (Learning-based)</SelectItem>
                                                <SelectItem value={CommunityType.Group}>Group (Container)</SelectItem>
                                            </SelectContent>
                                        </Select>
                                        <FormDescription className="min-h-[40px]">
                                            {communityType === CommunityType.Zone && "A structural area (e.g., District, Region) for organization. No direct members."}
                                            {communityType === CommunityType.Neighborhood && "A local community where people gather and interact. Restricted to Code of Conduct."}
                                            {communityType === CommunityType.Guild && "A community of practitioners focused on a specific craft or skill."}
                                            {communityType === CommunityType.StudyGroup && "A group dedicated to learning a specific subject together."}
                                            {communityType === CommunityType.Group && "A container to organize related communities together. Can have badge requirements."}
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />

                            <FormField
                                control={form.control as any}
                                name="parent_community_id"
                                render={({ field }) => {
                                    // Filter logic
                                    const filteredCommunities = communities?.filter(c => {
                                        if (communityType === CommunityType.Zone || communityType === CommunityType.Neighborhood) {
                                            // Only Zone or Neighborhood parents
                                            return c.type === CommunityType.Zone || c.type === CommunityType.Neighborhood
                                        }
                                        // Guild/StudyGroup/Group can have any parent
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
                                            <FormDescription className="min-h-[40px]">
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
                                        Define the physical location and coverage area of this community.
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

                        {communityType !== CommunityType.Zone && communityType !== CommunityType.Neighborhood && (
                            <FormField
                                control={form.control as any}
                                name="inherit_requirements"
                                render={({ field }) => (
                                    <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                                        <div className="space-y-0.5">
                                            <FormLabel className="text-base">Inherit Requirements</FormLabel>
                                            <FormDescription>
                                                Inherit badge requirements from parent community or territory.
                                            </FormDescription>
                                        </div>
                                        <FormControl>
                                            <Switch
                                                checked={field.value}
                                                onCheckedChange={field.onChange}
                                            />
                                        </FormControl>
                                    </FormItem>
                                )}
                            />
                        )}

                        <div className="space-y-4">
                            <h3 className="text-lg font-medium">Required Badges</h3>
                            <p className="text-sm text-muted-foreground">
                                Select badges required to participate in this community.
                                {(communityType === CommunityType.Neighborhood || communityType === CommunityType.Zone) && " Zones and Neighborhoods are restricted to the Code of Conduct badge only."}
                            </p>

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
                                                            !initialRequirements.some(r => r.badge_id === b.id)
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
                                                    {badges?.filter(b => b.name.toLowerCase().includes(searchQuery.toLowerCase()) && !initialRequirements.some(r => r.badge_id === b.id)).length === 0 && (
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
                        </div>
                        {/* Hidden territory_id field */}
                        <FormField
                            control={form.control as any}
                            name="territory_id"
                            render={({ field }) => (
                                <input type="hidden" {...field} />
                            )}
                        />

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
            </div>
        </AppLayout>
    )
}
