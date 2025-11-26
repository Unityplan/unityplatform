export interface User {
  id: string;
  email: string;
  username: string;
  fullName: string | null;
  territory: string;
  isActive: boolean;
  createdAt: string;
  /** Badge slugs from JWT claims (e.g., ["code-of-conduct", "community-manager"]) */
  badges: string[];
}

/**
 * Check if a user has a specific badge
 */
export function hasBadge(user: User | null, badgeSlug: string): boolean {
  return user?.badges?.includes(badgeSlug) ?? false;
}

/**
 * Check if a user has all specified badges
 */
export function hasAllBadges(user: User | null, badgeSlugs: string[]): boolean {
  if (!user?.badges) return false;
  return badgeSlugs.every(slug => user.badges.includes(slug));
}

/**
 * Check if a user has any of the specified badges
 */
export function hasAnyBadge(user: User | null, badgeSlugs: string[]): boolean {
  if (!user?.badges) return false;
  return badgeSlugs.some(slug => user.badges.includes(slug));
}

export interface LoginRequest {
  username: string;  // Login by username (privacy-first)
  password: string;
  territory: string;
}

export interface RegisterRequest {
  email?: string;
  username: string;
  password: string;
  territory: string;
  invitationToken?: string; // Optional for registration
}

export interface AuthResponse {
  accessToken: string;
  refreshToken: string;
  tokenType: string;
  expiresIn: number;
}

export interface AuthState {
  user: User | null;
  accessToken: string | null;
  refreshToken: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  isLocked: boolean; // Session lock state
}

export interface AuthActions {
  login: (credentials: LoginRequest) => Promise<void>;
  register: (data: RegisterRequest) => Promise<void>;
  logout: () => Promise<void>;
  refreshAccessToken: () => Promise<void>;
  loadUser: () => Promise<void>;
  setTokens: (accessToken: string, refreshToken: string) => void;
  setUser: (user: User) => void;
  clearAuth: () => void;
  lockSession: () => void; // Lock the session
  unlockSession: (email: string, password: string) => Promise<void>; // Unlock with re-auth
}

export type AuthStore = AuthState & AuthActions;
