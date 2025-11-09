import apiClient from '@/lib/api-client';

const USER_BASE_URL = import.meta.env.VITE_USER_SERVICE_URL || 'http://localhost:8081';

export interface UserProfile {
  user_id: string;
  username: string;
  full_name: string | null;
  bio: string | null;
  location: string | null;
  website: string | null;
  avatar_url: string | null;
  banner_url: string | null;
  created_at: string;
  updated_at: string;
}

export interface UpdateProfileRequest {
  full_name?: string | null;
  bio?: string | null;
  location?: string | null;
  website?: string | null;
}

export interface PrivacySettings {
  user_id: string;
  profile_visibility: 'public' | 'followers_only' | 'private';
  show_email: boolean;
  show_location: boolean;
  show_connections: boolean;
  allow_messages_from: 'everyone' | 'followers_only' | 'nobody';
  updated_at: string;
}

export interface UpdatePrivacySettingsRequest {
  profile_visibility?: 'public' | 'followers_only' | 'private';
  show_email?: boolean;
  show_location?: boolean;
  show_connections?: boolean;
  allow_messages_from?: 'everyone' | 'followers_only' | 'nobody';
}

export interface UserConnection {
  follower_id: string;
  following_id: string;
  created_at: string;
  follower_username?: string;
  following_username?: string;
}

/**
 * Get user profile by username
 * 
 * @param username - Username to fetch profile for
 * @param territoryCode - Territory code
 * @returns User profile
 */
export async function getUserProfile(username: string, territoryCode: string): Promise<UserProfile> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/users/${username}`, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}

/**
 * Update current user's profile
 * 
 * @param data - Profile data to update
 * @param territoryCode - Territory code
 * @returns Updated profile
 */
export async function updateProfile(data: UpdateProfileRequest, territoryCode: string): Promise<UserProfile> {
  const response = await apiClient.put(`${USER_BASE_URL}/api/v1/users/profile`, data, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}

/**
 * Upload user avatar
 * 
 * @param file - Image file to upload
 * @param territoryCode - Territory code
 * @returns Updated profile with new avatar URL
 */
export async function uploadAvatar(file: File, territoryCode: string): Promise<UserProfile> {
  const formData = new FormData();
  formData.append('file', file);

  const response = await apiClient.post(`${USER_BASE_URL}/api/v1/users/avatar`, formData, {
    params: { territory_code: territoryCode },
    headers: {
      'Content-Type': 'multipart/form-data',
    },
  });
  return response.data;
}

/**
 * Delete user avatar
 * 
 * @param territoryCode - Territory code
 * @returns Updated profile without avatar
 */
export async function deleteAvatar(territoryCode: string): Promise<UserProfile> {
  const response = await apiClient.delete(`${USER_BASE_URL}/api/v1/users/avatar`, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}

/**
 * Get user's privacy settings
 * 
 * @param territoryCode - Territory code
 * @returns Privacy settings
 */
export async function getPrivacySettings(territoryCode: string): Promise<PrivacySettings> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/users/privacy`, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}

/**
 * Update user's privacy settings
 * 
 * @param data - Privacy settings to update
 * @param territoryCode - Territory code
 * @returns Updated privacy settings
 */
export async function updatePrivacySettings(
  data: UpdatePrivacySettingsRequest,
  territoryCode: string
): Promise<PrivacySettings> {
  const response = await apiClient.put(`${USER_BASE_URL}/api/v1/users/privacy`, data, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}

/**
 * Follow a user
 * 
 * @param username - Username to follow
 * @param territoryCode - Territory code
 */
export async function followUser(username: string, territoryCode: string): Promise<void> {
  await apiClient.post(`${USER_BASE_URL}/api/v1/users/${username}/follow`, null, {
    params: { territory_code: territoryCode },
  });
}

/**
 * Unfollow a user
 * 
 * @param username - Username to unfollow
 * @param territoryCode - Territory code
 */
export async function unfollowUser(username: string, territoryCode: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/users/${username}/follow`, {
    params: { territory_code: territoryCode },
  });
}

/**
 * Get user's followers
 * 
 * @param username - Username to get followers for
 * @param territoryCode - Territory code
 * @returns List of follower connections
 */
export async function getFollowers(username: string, territoryCode: string): Promise<UserConnection[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/users/${username}/followers`, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}

/**
 * Get users that the user is following
 * 
 * @param username - Username to get following list for
 * @param territoryCode - Territory code
 * @returns List of following connections
 */
export async function getFollowing(username: string, territoryCode: string): Promise<UserConnection[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/users/${username}/following`, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}

/**
 * Block a user
 * 
 * @param username - Username to block
 * @param territoryCode - Territory code
 */
export async function blockUser(username: string, territoryCode: string): Promise<void> {
  await apiClient.post(`${USER_BASE_URL}/api/v1/users/${username}/block`, null, {
    params: { territory_code: territoryCode },
  });
}

/**
 * Unblock a user
 * 
 * @param username - Username to unblock
 * @param territoryCode - Territory code
 */
export async function unblockUser(username: string, territoryCode: string): Promise<void> {
  await apiClient.delete(`${USER_BASE_URL}/api/v1/users/${username}/block`, {
    params: { territory_code: territoryCode },
  });
}

/**
 * Get list of blocked users
 * 
 * @param territoryCode - Territory code
 * @returns List of blocked user IDs
 */
export async function getBlockedUsers(territoryCode: string): Promise<string[]> {
  const response = await apiClient.get(`${USER_BASE_URL}/api/v1/users/blocks`, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}
