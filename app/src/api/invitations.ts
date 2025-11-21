import { apiClient } from '@/lib/api-client';

const INVITATION_SERVICE_URL = import.meta.env.VITE_INVITATION_SERVICE_URL || 'http://localhost:8004';

// --- Types ---

export interface CreateInvitationRequest {
  max_uses: number;
  expires_in_days?: number;
  metadata?: Record<string, any>;
}

export interface CreateInvitationResponse {
  id: string;
  token: string;
  created_by: string;
  max_uses: number;
  expires_at?: string;
  invite_url: string;
}

export interface InvitationUse {
  user_id: string;
  username: string;
  used_at: string;
  ip_address?: string;
}

export interface Invitation {
  id: string;
  token: string;
  max_uses: number;
  uses_count: number;
  is_active: boolean;
  expires_at?: string;
  created_at: string;
  revoked_at?: string;
  status: string; // 'active' | 'used' | 'expired' | 'revoked'
  uses: InvitationUse[];
}

export interface PaginationInfo {
  page: number;
  limit: number;
  total: number;
}

export interface ListInvitationsResponse {
  invitations: Invitation[];
  pagination: PaginationInfo;
}

export interface ValidateInvitationRequest {
  token: string;
}

export interface ValidateInvitationResponse {
  valid: boolean;
  invitation_id?: string;
  created_by?: string;
  uses_remaining?: number;
  expires_at?: string;
  metadata?: Record<string, any>;
}

export interface GetInvitationUsesResponse {
  invitation_id: string;
  token: string;
  uses: InvitationUse[];
  total_uses: number;
  max_uses: number;
}

export interface RevokeInvitationResponse {
  invitation_id: string;
  revoked_at: string;
}

// --- API Client ---

export const invitationApi = {
  /**
   * Create a new invitation token
   */
  createInvitation: (data: CreateInvitationRequest) =>
    apiClient.post<CreateInvitationResponse>(`${INVITATION_SERVICE_URL}/api/v1/invitations`, data),

  /**
   * Validate an invitation token (Public)
   */
  validateInvitation: (token: string) =>
    apiClient.post<ValidateInvitationResponse>(`${INVITATION_SERVICE_URL}/api/v1/invitations/validate`, { token }),

  /**
   * List invitations created by the current user
   */
  listMyInvitations: (params?: { status?: string; page?: number; limit?: number }) =>
    apiClient.get<ListInvitationsResponse>(`${INVITATION_SERVICE_URL}/api/v1/invitations/me`, { params }),

  /**
   * Get detailed usage history for an invitation
   */
  getInvitationUses: (id: string) =>
    apiClient.get<GetInvitationUsesResponse>(`${INVITATION_SERVICE_URL}/api/v1/invitations/${id}/uses`),

  /**
   * Revoke an invitation
   */
  revokeInvitation: (id: string) =>
    apiClient.delete<RevokeInvitationResponse>(`${INVITATION_SERVICE_URL}/api/v1/invitations/${id}`),
};
