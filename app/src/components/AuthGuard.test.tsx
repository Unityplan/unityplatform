import { describe, it, beforeEach } from 'vitest';
import { clearAuthState } from '@/test/test-utils';

describe('AuthGuard', () => {
    beforeEach(() => {
        clearAuthState();
    });

    // TODO: AuthGuard unit tests are complex because they depend on full router context
    // and route tree rendering. These should be tested as part of integration tests
    // that test actual protected routes (e.g., /dashboard, /profile) instead of
    // testing the AuthGuard component in isolation.
    //
    // See LoginPage.test.tsx for examples of integration tests that verify navigation
    // and route protection behavior.

    it.todo('renders children when user is authenticated');
    it.todo('redirects to /login when user is not authenticated');
    it.todo('redirects to /login when user is null');
});
