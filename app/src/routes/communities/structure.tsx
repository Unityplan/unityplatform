import { createFileRoute } from '@tanstack/react-router'
import { GlobalStructurePage } from '@/pages/communities/GlobalStructurePage'
import { z } from 'zod'

const structureSearchSchema = z.object({
    tab: z.enum(['hierarchy', 'map', 'flow', 'stats']).optional().default('hierarchy'),
})

export const Route = createFileRoute('/communities/structure')({
    validateSearch: (search) => structureSearchSchema.parse(search),
    component: GlobalStructurePage,
})
