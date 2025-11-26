import { apiClient } from '@/lib/api-client'

export interface Territory {
    code: string
    name: string
    flag_icon: string
    api_url: string
    is_active: boolean
}

export const territoryService = {
    listTerritories: async (): Promise<Territory[]> => {
        const response = await apiClient.get('/territory/territories')
        return response.data
    },

    getTerritory: async (code: string): Promise<Territory> => {
        const response = await apiClient.get(`/territory/territories/${code}`)
        return response.data
    },
}
