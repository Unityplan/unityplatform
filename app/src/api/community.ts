import { apiClient } from '@/lib/api-client'

export const CommunityType = {
  Zone: 'zone',
  Neighborhood: 'neighborhood',
  Guild: 'guild',
  StudyGroup: 'study_group',
  /** A container community that groups other communities together */
  Group: 'group',
} as const

export type CommunityType = (typeof CommunityType)[keyof typeof CommunityType]

export interface GeoPoint {
  lat: number
  lng: number
}

export interface CoverageArea {
  type: 'circle' | 'polygon'
  center?: GeoPoint
  radius?: number // in meters
  coordinates?: GeoPoint[]
}

export interface Community {
  id: string
  slug: string
  name: string
  description?: string
  type: CommunityType
  territory_id?: string
  parent_community_id?: string
  avatar_url?: string
  banner_url?: string
  location_lat?: number
  location_lng?: number
  coverage_area?: CoverageArea
  is_public: boolean
  member_count: number
  created_at: string
  updated_at: string
  requirements?: CommunityBadgeRequirement[]
}

export interface CreateCommunityRequest {
  name: string
  description?: string
  community_type: CommunityType
  territory_id?: string
  parent_community_id?: string
  avatar_url?: string
  banner_url?: string
  location_lat?: number
  location_lng?: number
  coverage_area?: CoverageArea
  inherit_requirements?: boolean
  initial_requirements?: AddRequirementRequest[]
}

export interface UpdateCommunityRequest {
  name?: string
  description?: string
  parent_community_id?: string
  avatar_url?: string
  banner_url?: string
  location_lat?: number
  location_lng?: number
  coverage_area?: CoverageArea
}

export interface CommunityFilter {
  community_type?: CommunityType
  parent_id?: string
  territory_id?: string
  search?: string
}

export const RequirementContext = {
  View: 'view',
  Participate: 'participate',
  Admin: 'admin',
} as const

export type RequirementContext = (typeof RequirementContext)[keyof typeof RequirementContext]

export interface CommunityBadgeRequirement {
  id: string
  community_id: string
  badge_id: string
  context: RequirementContext
  created_at: string
}

/** A badge requirement with inheritance information */
export interface EffectiveBadgeRequirement {
  id: string
  communityId: string
  badgeId: string
  context: RequirementContext
  createdAt: string
  /** Name of the badge */
  badgeName: string
  /** Slug of the badge */
  badgeSlug: string
  /** Name of the community that defines this requirement */
  sourceCommunityName: string
  /** Whether this requirement is inherited from a parent community */
  isInherited: boolean
  /** How many levels up this requirement comes from (0 = direct, 1 = parent, etc.) */
  inheritanceDepth: number
}

export interface AddRequirementRequest {
  badge_id: string
  context: RequirementContext
}

export interface EffectiveManager {
  user_id: string
  username: string
  avatar_url?: string
  role: string
  source: string
  distance: number
}

/** Summary of child groups (Guilds and Study Groups) under a community */
export interface GroupSummary {
  communityId: string
  guildCount: number
  studyGroupCount: number
  totalGuildCount: number
  totalStudyGroupCount: number
  totalMembers: number
  hasBadgeRequirement: boolean
  badgeName?: string
  badgeId?: string
  children: GroupChild[]
}

/** A child group community summary */
export interface GroupChild {
  id: string
  name: string
  slug: string
  communityType: CommunityType
  memberCount: number
  hasBadgeRequirement: boolean
  badgeName?: string
}

export const communityService = {
  listCommunities: async (filter: CommunityFilter): Promise<Community[]> => {
    const response = await apiClient.get('/api/v1/communities', { params: filter })
    return response.data
  },

  createCommunity: async (data: CreateCommunityRequest): Promise<Community> => {
    const response = await apiClient.post('/api/v1/communities', data)
    return response.data
  },

  getCommunity: async (id: string): Promise<Community> => {
    const response = await apiClient.get(`/api/v1/communities/${id}`)
    return response.data
  },

  addRequirement: async (communityId: string, data: AddRequirementRequest): Promise<void> => {
    await apiClient.post(`/api/v1/communities/${communityId}/requirements`, data)
  },

  removeRequirement: async (communityId: string, badgeId: string, context: RequirementContext): Promise<void> => {
    await apiClient.delete(`/api/v1/communities/${communityId}/requirements/${badgeId}`, {
      params: { context },
    })
  },

  listRequirements: async (communityId: string): Promise<CommunityBadgeRequirement[]> => {
    const response = await apiClient.get(`/api/v1/communities/${communityId}/requirements`)
    return response.data
  },

  assignManager: async (communityId: string, userId: string): Promise<void> => {
    await apiClient.post(`/api/v1/communities/${communityId}/manager`, { user_id: userId })
  },

  revokeManager: async (communityId: string, userId: string): Promise<void> => {
    await apiClient.delete(`/api/v1/communities/${communityId}/manager/${userId}`)
  },

  updateCommunity: async (id: string, data: UpdateCommunityRequest): Promise<Community> => {
    const response = await apiClient.put(`/api/v1/communities/${id}`, data)
    return response.data
  },

  getEffectiveManagers: async (id: string): Promise<EffectiveManager[]> => {
    const response = await apiClient.get(`/api/v1/communities/${id}/managers`)
    return response.data
  },

  getManagedCommunities: async (): Promise<string[]> => {
    const response = await apiClient.get('/api/v1/communities/managed-by-me')
    return response.data
  },

  joinCommunity: async (id: string): Promise<void> => {
    await apiClient.post(`/api/v1/communities/${id}/join`)
  },

  leaveCommunity: async (id: string): Promise<void> => {
    await apiClient.post(`/api/v1/communities/${id}/leave`)
  },

  getMembership: async (id: string): Promise<{ is_member: boolean }> => {
    const response = await apiClient.get(`/api/v1/communities/${id}/membership`)
    return response.data
  },

  /** Get a summary of child groups (Guilds and Study Groups) under a community */
  getGroupSummary: async (communityId: string): Promise<GroupSummary> => {
    const response = await apiClient.get(`/api/v1/communities/${communityId}/group-summary`)
    return response.data
  },

  /** Get effective badge requirements including inherited ones from parent communities */
  getEffectiveRequirements: async (communityId: string): Promise<EffectiveBadgeRequirement[]> => {
    const response = await apiClient.get(`/api/v1/communities/${communityId}/effective-requirements`)
    return response.data
  },
}

