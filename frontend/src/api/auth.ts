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
 * ⭐ SECURE: Territory is looked up from global registry (client cannot manipulate)
 * 
 * @param token - Invitation token to validate
 * @returns Invitation details including territory and community info
 */
export async function validateInvitation(token: string): Promise<{
  valid: boolean;
  token_type: string;
  territory: {
    code: string;
    name: string;
  };
  community?: {
    id: string;
    name: string;
  };
  email?: string;
  expires_at?: string;
  remaining_uses?: number;
}> {
  // ⭐ No territory_code parameter - backend looks it up from global registry
  const response = await apiClient.get(`${AUTH_BASE_URL}/api/v1/invitations/validate/${token}`);
  return response.data;
}
