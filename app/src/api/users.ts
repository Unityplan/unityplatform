import apiClient from '@/lib/api-client';

const USER_BASE_URL = import.meta.env.VITE_USER_SERVICE_URL || 'http://localhost:8002';

export interface UserProfile {
  id: string;
  username: string;
  email?: string | null;
  fullName?: string | null;
  displayName?: string | null;
  avatarUrl?: string | null;
  bio?: string | null;
  about?: string | null;
  interests?: string[] | null;
  skills?: string[] | null;
  languages?: string[] | null;
  location?: string | null;
  website?: string | null;
  theme?: string | null;
  privacy?: PrivacySettings;
  createdAt?: string;
  updatedAt?: string;
}

export interface UpdateProfileRequest {
  fullName?: string | null;
  displayName?: string | null;
  bio?: string | null;
  about?: string | null;
  interests?: string[] | null;
  skills?: string[] | null;
  languages?: string[] | null;
  location?: string | null;
  website?: string | null;
  theme?: string | null;
  privacy?: Partial<PrivacySettings>;
}

// Profile Links - New flexible system for external links
export interface ProfileLink {
  id: string;
  label: string;
  url: string;
  icon?: string | null;
  displayOrder: number;
  isVisible: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateProfileLinkRequest {
  label: string;
  url: string;
  icon?: string | null;
  displayOrder?: number;
  isVisible?: boolean;
}

export interface UpdateProfileLinkRequest {
  label?: string;
  url?: string;
  icon?: string | null;
  displayOrder?: number;
  isVisible?: boolean;
}

export interface ReorderLinksRequest {
  linkIds: string[];
}

// Language Proficiency
export type ProficiencyLevel = 'none' | 'basic' | 'intermediate' | 'fluent' | 'native';

export interface LanguageProficiency {
  id: string;
  languageCode: string;
  languageName: string;
  spokenLevel: ProficiencyLevel;
  writtenLevel: ProficiencyLevel;
  readingLevel: ProficiencyLevel;
  listeningLevel: ProficiencyLevel;
  displayOrder: number;
  isPreferred: boolean;
  showOnProfile: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateLanguageProficiencyRequest {
  languageCode: string;
  languageName: string;
  spokenLevel: ProficiencyLevel;
  writtenLevel: ProficiencyLevel;
  readingLevel: ProficiencyLevel;
  listeningLevel: ProficiencyLevel;
  displayOrder?: number;
  isPreferred?: boolean;
  showOnProfile?: boolean;
}

export interface UpdateLanguageProficiencyRequest {
  languageCode?: string;
  languageName?: string;
  spokenLevel?: ProficiencyLevel;
  writtenLevel?: ProficiencyLevel;
  readingLevel?: ProficiencyLevel;
  listeningLevel?: ProficiencyLevel;
  displayOrder?: number;
  isPreferred?: boolean;
  showOnProfile?: boolean;
}

export interface PrivacySettings {
  profile_visibility: 'public' | 'connections_only' | 'private';
  show_email: boolean;
  show_full_name: boolean;
  show_location: boolean;
  show_connections: boolean;
  allow_messages_from: 'everyone' | 'connections_only' | 'nobody';
}

export interface UpdatePrivacySettingsRequest {
  profile_visibility?: 'public' | 'connections_only' | 'private';
  show_email?: boolean;
  show_full_name?: boolean;
  show_location?: boolean;
  show_connections?: boolean;
  allow_messages_from?: 'everyone' | 'connections_only' | 'nobody';
}

export interface UserConnection {
  followerId: string;
  followingId: string;
  createdAt: string;
  followerUsername?: string;
  followingUsername?: string;
}

export interface ConnectionsListResponse {
  connections: UserConnection[];
  total: number;
  offset: number;
  limit: number;
}

/**
 * Get user profile by user ID
 * 
 * @param userId - User ID to fetch profile for
 * @returns User profile
 */
export async function getUserProfile(userId: string): Promise<UserProfile> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/user/profile/${userId}`);
  return response.data; // Backend returns data directly (camelCase)
}

/**
 * Get full profile (own profile with all details)
 * 
 * @returns Full user profile
 */
export async function getFullProfile(): Promise<UserProfile> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/user/profile`);
  return response.data; // Backend returns data directly (camelCase)
}

/**
 * Update current user's profile
 * 
 * @param data - Profile data to update
 * @returns Updated profile
 */
export async function updateProfile(data: UpdateProfileRequest): Promise<UserProfile> {
  const response = await apiClient.put(`${USER_BASE_URL}/api/v1/user/profile`, data);
  return response.data; // Backend returns data directly (camelCase)
}

/**
 * Upload user avatar
 * 
 * @param userId - User ID
 * @param file - Image file to upload
 * @returns Updated profile with new avatar URL
 */
export async function uploadAvatar(userId: string, file: File): Promise<UserProfile> {
  const formData = new FormData();
  formData.append('file', file);

  const response = await apiClient.post(`${USER_BASE_URL}/api/v1/avatars/${userId}`, formData, {
    headers: {
      'Content-Type': 'multipart/form-data',
    },
  });
  return response.data.data;
}

/**
 * Delete user avatar
 * 
 * @param userId - User ID
 * @returns Updated profile without avatar
 */
export async function deleteAvatar(userId: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/avatars/${userId}`);
}

/**
 * Follow a user
 * 
 * @param targetId - Target user ID to follow
 */
export async function followUser(targetId: string): Promise<void> {
  await apiClient.post(`${USER_BASE_URL}/api/v1/user/connections/${targetId}/follow`);
}

/**
 * Unfollow a user
 * 
 * @param targetId - Target user ID to unfollow
 */
export async function unfollowUser(targetId: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/user/connections/${targetId}/follow`);
}

/**
 * Get user's followers
 * 
 * @param userId - User ID to get followers for
 * @param limit - Maximum number of results (default: 20, max: 100)
 * @param offset - Offset for pagination (default: 0)
 * @returns Paginated list of follower connections
 */
export async function getFollowers(
  userId: string,
  limit: number = 20,
  offset: number = 0
): Promise<ConnectionsListResponse> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/user/connections/${userId}/followers`, {
    params: { limit, offset },
  });
  return response.data;
}

/**
 * Get users that the user is following
 * 
 * @param userId - User ID to get following list for
 * @param limit - Maximum number of results (default: 20, max: 100)
 * @param offset - Offset for pagination (default: 0)
 * @returns Paginated list of following connections
 */
export async function getFollowing(
  userId: string,
  limit: number = 20,
  offset: number = 0
): Promise<ConnectionsListResponse> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/user/connections/${userId}/following`, {
    params: { limit, offset },
  });
  return response.data;
}

/**
 * Block a user
 * 
 * @param targetId - Target user ID to block
 */
export async function blockUser(targetId: string): Promise<void> {
  await apiClient.post(`${USER_BASE_URL}/api/v1/user/connections/${targetId}/block`);
}

/**
 * Unblock a user
 * 
 * @param targetId - Target user ID to unblock
 */
export async function unblockUser(targetId: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/user/connections/${targetId}/block`);
}

// ============================================================================
// Profile Links API
// ============================================================================

/**
 * Get user's profile links
 * 
 * @returns List of profile links
 */
export async function getProfileLinks(): Promise<ProfileLink[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/user/profile/links`);
  return response.data;
}

/**
 * Create a new profile link
 * 
 * @param data - Link data to create
 * @returns Created profile link
 */
export async function createProfileLink(data: CreateProfileLinkRequest): Promise<ProfileLink> {
  const response = await apiClient.post(`${USER_BASE_URL}/api/v1/user/profile/links`, data);
  return response.data;
}

/**
 * Update a profile link
 * 
 * @param linkId - Link ID to update
 * @param data - Link data to update
 * @returns Updated profile link
 */
export async function updateProfileLink(
  linkId: string,
  data: UpdateProfileLinkRequest
): Promise<ProfileLink> {
  const response = await apiClient.put(`${USER_BASE_URL}/api/v1/user/profile/links/${linkId}`, data);
  return response.data;
}

/**
 * Delete a profile link
 * 
 * @param linkId - Link ID to delete
 */
export async function deleteProfileLink(linkId: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/user/profile/links/${linkId}`);
}

// ============================================================================
// Language Proficiency API
// ============================================================================

/**
 * Get user's language proficiencies
 * 
 * @returns List of language proficiencies
 */
export async function getLanguageProficiencies(): Promise<LanguageProficiency[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/user/profile/languages`);
  return response.data;
}

/**
 * Create a new language proficiency
 * 
 * @param data - Language proficiency data to create
 * @returns Created language proficiency
 */
export async function createLanguageProficiency(data: CreateLanguageProficiencyRequest): Promise<LanguageProficiency> {
  const response = await apiClient.post(`${USER_BASE_URL}/api/v1/user/profile/languages`, data);
  return response.data;
}

/**
 * Update a language proficiency
 * 
 * @param langId - Language proficiency ID to update
 * @param data - Language proficiency data to update
 * @returns Updated language proficiency
 */
export async function updateLanguageProficiency(
  langId: string,
  data: UpdateLanguageProficiencyRequest
): Promise<LanguageProficiency> {
  const response = await apiClient.put(`${USER_BASE_URL}/api/v1/user/profile/languages/${langId}`, data);
  return response.data;
}

/**
 * Delete a language proficiency
 * 
 * @param langId - Language proficiency ID to delete
 */
export async function deleteLanguageProficiency(langId: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/user/profile/languages/${langId}`);
}
