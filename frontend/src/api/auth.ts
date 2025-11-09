import apiClient from '@/lib/api-client';
import type { LoginRequest, RegisterRequest, AuthResponse, User } from '@/types/auth';

const AUTH_BASE_URL = import.meta.env.VITE_AUTH_SERVICE_URL || 'http://localhost:8080';

/**
 * Register a new user
 * 
 * @param data - Registration data including email, username, password, territory_code, and invitation_token
 * @returns Auth response with tokens and user data
 */
export async function register(data: RegisterRequest): Promise<AuthResponse> {
  const response = await apiClient.post(`${AUTH_BASE_URL}/api/v1/auth/register`, data);
  return response.data;
}

/**
 * Login with email and password
 * 
 * @param credentials - Login credentials (email, password, territory_code)
 * @returns Auth response with tokens and user data
 */
export async function login(credentials: LoginRequest): Promise<AuthResponse> {
  const response = await apiClient.post(`${AUTH_BASE_URL}/api/v1/auth/login`, credentials);
  return response.data;
}

/**
 * Logout and invalidate refresh token
 * 
 * @param refreshToken - The refresh token to invalidate
 */
export async function logout(refreshToken: string): Promise<void> {
  await apiClient.post(`${AUTH_BASE_URL}/api/v1/auth/logout`, { refresh_token: refreshToken });
}

/**
 * Refresh access token using refresh token
 * 
 * @param refreshToken - The refresh token
 * @returns New auth response with fresh tokens
 */
export async function refreshToken(refreshToken: string): Promise<AuthResponse> {
  const response = await apiClient.post(`${AUTH_BASE_URL}/api/v1/auth/refresh`, {
    refresh_token: refreshToken,
  });
  return response.data;
}

/**
 * Get current authenticated user information
 * 
 * @returns Current user data
 */
export async function getCurrentUser(): Promise<User> {
  const response = await apiClient.get(`${AUTH_BASE_URL}/api/v1/auth/me`);
  return response.data;
}

/**
 * Validate an invitation token
 * 
 * @param token - Invitation token to validate
 * @param territoryCode - Territory code
 * @returns Invitation details if valid
 */
export async function validateInvitation(token: string, territoryCode: string): Promise<{
  valid: boolean;
  invitation_id?: string;
  created_by?: string;
  max_uses?: number;
  current_uses?: number;
  expires_at?: string;
}> {
  const response = await apiClient.get(`${AUTH_BASE_URL}/api/v1/invitations/validate/${token}`, {
    params: { territory_code: territoryCode },
  });
  return response.data;
}
