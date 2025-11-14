import apiClient from '@/lib/api-client';

const USER_BASE_URL = import.meta.env.VITE_USER_SERVICE_URL || 'http://localhost:8002';

export interface UserProfile {
  user_id: string;
  username: string;
  email?: string | null;
  full_name?: string | null;
  display_name?: string | null;
  avatar_url?: string | null;
  bio?: string | null;
  about?: string | null;
  interests?: string[] | null;
  skills?: string[] | null;
  languages?: string[] | null;
  location?: string | null;
  
  // @deprecated - Use profile_links instead. Will be removed in next version.
  // See: docs/architecture/profile-links-system.md
  website_url?: string | null;
  github_url?: string | null;
  linkedin_url?: string | null;
  twitter_handle?: string | null;
  
  theme?: string | null;
  privacy?: PrivacySettings;
  created_at?: string;
  updated_at?: string;
}

export interface UpdateProfileRequest {
  full_name?: string | null;
  display_name?: string | null;
  bio?: string | null;
  about?: string | null;
  interests?: string[] | null;
  skills?: string[] | null;
  languages?: string[] | null;
  location?: string | null;
  
  // @deprecated - Use profile links API instead
  website_url?: string | null;
  github_url?: string | null;
  linkedin_url?: string | null;
  twitter_handle?: string | null;
  
  theme?: string | null;
  privacy?: Partial<PrivacySettings>;
}

// Profile Links - New flexible system for external links
export interface ProfileLink {
  id: string;
  user_id: string;
  label: string;
  url: string;
  icon?: string | null;
  display_order: number;
  is_visible: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateProfileLinkRequest {
  label: string;
  url: string;
  icon?: string | null;
  display_order?: number;
  is_visible?: boolean;
}

export interface UpdateProfileLinkRequest {
  label?: string;
  url?: string;
  icon?: string | null;
  display_order?: number;
  is_visible?: boolean;
}

export interface ReorderLinksRequest {
  link_ids: string[];
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
  follower_id: string;
  following_id: string;
  created_at: string;
  follower_username?: string;
  following_username?: string;
}

/**
 * Get user profile by user ID
 * 
 * @param userId - User ID to fetch profile for
 * @returns User profile
 */
export async function getUserProfile(userId: string): Promise<UserProfile> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/profiles/${userId}`);
  return response.data.data; // Backend wraps in ApiResponse
}

/**
 * Get full profile (own profile with all details)
 * 
 * @param userId - User ID to fetch full profile for
 * @returns Full user profile
 */
export async function getFullProfile(userId: string): Promise<UserProfile> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/profiles/${userId}/full`);
  return response.data.data;
}

/**
 * Update current user's profile
 * 
 * @param userId - User ID
 * @param data - Profile data to update
 * @returns Updated profile
 */
export async function updateProfile(userId: string, data: UpdateProfileRequest): Promise<UserProfile> {
  const response = await apiClient.put(`${USER_BASE_URL}/api/v1/profiles/${userId}`, data);
  return response.data.data;
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
 * @param userId - User ID
 * @param targetId - Target user ID to follow
 */
export async function followUser(userId: string, targetId: string): Promise<void> {
  await apiClient.post(`${USER_BASE_URL}/api/v1/connections/follow/${targetId}`, null, {
    params: { user_id: userId },
  });
}

/**
 * Unfollow a user
 * 
 * @param userId - User ID  
 * @param targetId - Target user ID to unfollow
 */
export async function unfollowUser(userId: string, targetId: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/connections/follow/${targetId}`, {
    params: { user_id: userId },
  });
}

/**
 * Get user's followers
 * 
 * @param userId - User ID to get followers for
 * @returns List of follower connections
 */
export async function getFollowers(userId: string): Promise<UserConnection[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/connections/followers`, {
    params: { user_id: userId },
  });
  return response.data;
}

/**
 * Get users that the user is following
 * 
 * @param userId - User ID to get following list for
 * @returns List of following connections
 */
export async function getFollowing(userId: string): Promise<UserConnection[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/connections/following`, {
    params: { user_id: userId },
  });
  return response.data;
}

/**
 * Block a user
 * 
 * @param userId - User ID
 * @param targetId - Target user ID to block
 */
export async function blockUser(userId: string, targetId: string): Promise<void> {
  await apiClient.post(`${USER_BASE_URL}/api/v1/connections/block/${targetId}`, null, {
    params: { user_id: userId },
  });
}

/**
 * Unblock a user
 * 
 * @param userId - User ID
 * @param targetId - Target user ID to unblock
 */
export async function unblockUser(userId: string, targetId: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/connections/block/${targetId}`, {
    params: { user_id: userId },
  });
}

/**
 * Get list of blocked users
 * 
 * @param userId - User ID
 * @returns List of blocked users
 */
export async function getBlockedUsers(userId: string): Promise<UserConnection[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/connections/blocked`, {
    params: { user_id: userId },
  });
  return response.data;
}

// ============================================================================
// Profile Links API
// ============================================================================

/**
 * Get user's profile links
 * 
 * @param userId - User ID to get links for
 * @returns List of profile links
 */
export async function getProfileLinks(userId: string): Promise<ProfileLink[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/profiles/${userId}/links`);
  return response.data.data;
}

/**
 * Create a new profile link
 * 
 * @param userId - User ID
 * @param data - Link data to create
 * @returns Created profile link
 */
export async function createProfileLink(userId: string, data: CreateProfileLinkRequest): Promise<ProfileLink> {
  const response = await apiClient.post(`${USER_BASE_URL}/api/v1/profiles/${userId}/links`, data);
  return response.data.data;
}

/**
 * Update a profile link
 * 
 * @param userId - User ID
 * @param linkId - Link ID to update
 * @param data - Link data to update
 * @returns Updated profile link
 */
export async function updateProfileLink(
  userId: string,
  linkId: string,
  data: UpdateProfileLinkRequest
): Promise<ProfileLink> {
  const response = await apiClient.put(`${USER_BASE_URL}/api/v1/profiles/${userId}/links/${linkId}`, data);
  return response.data.data;
}

/**
 * Delete a profile link
 * 
 * @param userId - User ID
 * @param linkId - Link ID to delete
 */
export async function deleteProfileLink(userId: string, linkId: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/profiles/${userId}/links/${linkId}`);
}

/**
 * Reorder profile links
 * 
 * @param userId - User ID
 * @param data - Array of link IDs in desired order
 */
export async function reorderProfileLinks(userId: string, data: ReorderLinksRequest): Promise<void> {
  await apiClient.patch(`${USER_BASE_URL}/api/v1/profiles/${userId}/links/reorder`, data);
}
