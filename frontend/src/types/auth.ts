export interface User {
  id: string;
  email: string;
  username: string;
  full_name: string | null;
  territory_code: string;
  is_active: boolean;
  created_at: string;
}

export interface LoginRequest {
  username: string;  // Login by username (privacy-first)
  password: string;
  territory_code: string;
}

export interface RegisterRequest {
  email: string;
  username: string;
  password: string;
  full_name?: string;
  territory_code: string;
  invitation_token: string; // Required for registration
}

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
  user: User;
}

export interface AuthState {
  user: User | null;
  accessToken: string | null;
  refreshToken: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
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
}

export type AuthStore = AuthState & AuthActions;
