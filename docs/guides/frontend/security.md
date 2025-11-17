# Frontend Security Implementation

## Overview

This document outlines the security best practices and implementations in the Unity Platform frontend application.

## 🔐 Authentication & Authorization

### Token-Based Authentication (JWT)

**Implementation:**

- Access tokens stored in Zustand store (persisted to localStorage)
- Refresh tokens for obtaining new access tokens
- Automatic token refresh on 401 errors
- Secure token storage with persistence layer

**Security Measures:**

```typescript
// authStore.ts - Persistent token storage
export const useAuthStore = create<AuthStore>()(
  persist(
    (set, get) => ({
      accessToken: null,
      refreshToken: null,
      // ...
    }),
    {
      name: 'auth-storage',
      storage: createJSONStorage(() => localStorage),
    }
  )
);
```

**Best Practices:**

- ✅ Tokens stored in localStorage (acceptable for SPAs without httpOnly option)
- ✅ Automatic token refresh before expiry
- ✅ Clear tokens on logout
- ✅ Tokens included in Authorization header for all API requests

### Session Lock (Inactivity Protection)

**Feature:** Auto-lock session after 10 minutes of inactivity

**Implementation:**

```typescript
// AuthGuard.tsx - Inactivity detection
useInactivityDetector({
  timeout: 10 * 60 * 1000, // 10 minutes
  onInactive: () => {
    if (isAuthenticated && !isLocked) {
      lockSession();
    }
  },
  enabled: isAuthenticated && !isLocked,
});
```

**User Experience:**

1. After 10 minutes of no activity, session locks
2. User sees SessionLockScreen
3. Options:
   - Re-enter password to unlock
   - Logout and return to login page

**Security Benefits:**

- Prevents unauthorized access if user leaves device unattended
- Requires password re-authentication (not just clicking a button)
- Maintains user session while enforcing security

## 🔄 Automatic Token Refresh

### Token Refresh Flow

**Implementation:** `api-client.ts` interceptor

```typescript
// Response interceptor
apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    if (error.response?.status === 401 && !originalRequest._retry) {
      // Attempt token refresh
      await useAuthStore.getState().refreshAccessToken();
      // Retry original request with new token
      return apiClient(originalRequest);
    }
  }
);
```

**Features:**

- ✅ Automatic retry of failed requests after token refresh
- ✅ Prevents concurrent refresh requests (single refresh for multiple 401s)
- ✅ Queues pending requests during refresh
- ✅ Redirects to login if refresh fails
- ✅ Stores redirect path for post-login navigation

**Security Measures:**

- Prevents infinite retry loops with `_retry` flag
- Clears auth state if refresh fails
- Only attempts refresh once per request
- Handles race conditions with refresh queue

## 🛡️ Route Protection

### AuthGuard Component

**Purpose:** Protect routes from unauthenticated access

**Implementation:**

```typescript
export function AuthGuard({ children }: AuthGuardProps) {
  const { isAuthenticated, isLocked } = useAuthStore();
  
  // Redirect to login if not authenticated
  if (!isAuthenticated) {
    router.navigate({ to: '/login', search: { redirect: currentPath } });
    return null;
  }
  
  // Show lock screen if session is locked
  if (isLocked) {
    return <SessionLockScreen />;
  }
  
  return <>{children}</>;
}
```

**Features:**

- Automatic redirect to login for unauthenticated users
- Stores current path for post-login redirect
- Shows session lock screen for locked sessions
- Loading state while fetching user data

**Usage:**

```tsx
// In route files
export const Route = createFileRoute('/profile')({
  component: () => (
    <AuthGuard>
      <ProfilePage />
    </AuthGuard>
  ),
});
```

## 🚫 Request Blocking

### Locked Session Request Prevention

**Implementation:** API client request interceptor

```typescript
apiClient.interceptors.request.use((config) => {
  const { isLocked } = useAuthStore.getState();
  
  // Block requests if session is locked (except auth endpoints)
  if (isLocked && !config.url?.includes('/auth/')) {
    return Promise.reject(new Error('Session is locked'));
  }
  
  return config;
});
```

**Benefits:**

- Prevents API calls while session is locked
- Allows auth-related requests (unlock, logout)
- Clear error messaging

## 📊 Error Handling

### API Error Handling

**HTTP Status Codes:**

- `401 Unauthorized` → Automatic token refresh → Retry request
- `403 Forbidden` → Log error, show permission denied message
- `500 Server Error` → Log error, show generic error message

**Token Refresh Failure:**

```typescript
catch (refreshError) {
  // Clear auth state
  useAuthStore.getState().clearAuth();
  
  // Store redirect path
  sessionStorage.setItem('redirectAfterLogin', currentPath);
  
  // Redirect to login
  window.location.href = '/login';
}
```

## 🔑 Password Security

### Password Handling

**Best Practices:**

- ✅ Never store passwords in state or localStorage
- ✅ Only send passwords over HTTPS
- ✅ Clear password input after use
- ✅ Use type="password" for all password fields

**Re-authentication for Unlock:**

```typescript
// SessionLockScreen.tsx
const handleUnlock = async (password: string) => {
  await unlockSession(user.email, password);
  setPassword(''); // Clear password from state
};
```

## 🛠️ Additional Security Measures

### 1. **XSS Protection**

- React's built-in JSX escaping prevents XSS attacks
- `dangerouslySetInnerHTML` avoided throughout codebase
- User input sanitized before rendering

### 2. **CSRF Protection**

- Not currently implemented (tokens in localStorage, not cookies)
- If switching to cookie-based auth, add CSRF tokens
- Set `withCredentials: true` in axios config when needed

### 3. **Content Security Policy (CSP)**

- Should be configured at server/CDN level
- Recommended headers:

  ```
  Content-Security-Policy: default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';
  ```

### 4. **Request Timeout**

- All API requests timeout after 30 seconds
- Prevents hanging requests from locking UI

### 5. **Public Route Handling**

```typescript
const publicPaths = ['/login', '/register', '/forgot-password', '/reset-password'];
const isPublicPath = publicPaths.some(path => currentPath.includes(path));

if (!isPublicPath) {
  // Redirect to login
}
```

## 📋 Security Checklist

- ✅ Token-based authentication (JWT)
- ✅ Automatic token refresh
- ✅ Session lock after 10 minutes inactivity
- ✅ Password re-authentication for unlock
- ✅ Protected routes with AuthGuard
- ✅ Request blocking for locked sessions
- ✅ Automatic redirect on auth failure
- ✅ Post-login redirect to original page
- ✅ Token persistence in localStorage
- ✅ Error handling for all auth failures
- ✅ XSS protection via React
- ✅ Request timeouts
- ✅ Loading states during auth operations
- ✅ Clear error messages for users

## 🔮 Future Enhancements

### Recommended Additions

1. **Rate Limiting (Frontend)**
   - Throttle login attempts
   - Prevent brute force attacks

2. **Biometric Authentication**
   - WebAuthn API for fingerprint/face recognition
   - Passwordless authentication

3. **Two-Factor Authentication (2FA)**
   - TOTP codes
   - SMS/Email verification

4. **Session Management**
   - View all active sessions
   - Revoke sessions remotely
   - Device fingerprinting

5. **Security Headers**
   - Implement at CDN/server level
   - X-Frame-Options
   - X-Content-Type-Options
   - Strict-Transport-Security

6. **Audit Logging**
   - Log all auth events
   - Track suspicious activity
   - Alert on anomalies

## 📚 References

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [JWT Best Practices](https://tools.ietf.org/html/rfc8725)
- [Web Authentication API](https://www.w3.org/TR/webauthn/)
- [React Security Best Practices](https://react.dev/learn/keeping-components-pure#side-effects-unintended-consequences)

## 🚀 Usage Examples

### Protecting a Route

```tsx
// src/routes/protected-page.tsx
import { createFileRoute } from '@tanstack/react-router';
import { AuthGuard } from '@/components/AuthGuard';
import { ProtectedPage } from '@/pages/ProtectedPage';

export const Route = createFileRoute('/protected')({
  component: () => (
    <AuthGuard>
      <ProtectedPage />
    </AuthGuard>
  ),
});
```

### Making Authenticated Requests

```typescript
// Automatically includes auth token and handles refresh
import apiClient from '@/lib/api-client';

const response = await apiClient.get('/api/v1/users/me');
```

### Manual Logout

```typescript
import { useAuthStore } from '@/stores/authStore';

const { logout } = useAuthStore();
await logout();
```

### Checking Auth State

```typescript
import { useAuthStore } from '@/stores/authStore';

const { isAuthenticated, user, isLocked } = useAuthStore();

if (!isAuthenticated) {
  // Show login
}

if (isLocked) {
  // Show lock screen
}
```

---

**Last Updated:** November 16, 2025  
**Version:** 0.1.0-alpha.1  
**Maintained by:** Unity Platform Security Team
