import { apiClient } from '@/lib/api-client'

export interface Badge {
  id: string
  slug: string
  name: string
  description: string
  icon: string
  category: string
  criteria_type: string
  created_at: string
}

export const badgeService = {
  listBadges: async (): Promise<Badge[]> => {
    const response = await apiClient.get('/api/v1/badges')
    return response.data
  },
}
