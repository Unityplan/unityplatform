import { http, HttpResponse } from 'msw';
import { mockUser, mockTokens } from '../test-utils';

const API_URL = import.meta.env.VITE_AUTH_SERVICE_URL || 'http://localhost:8001';

export const handlers = [
  // Auth endpoints
  http.post(`${API_URL}/api/v1/auth/login`, async ({ request }) => {
    const body = await request.json();
    
    // Simulate successful login
    if (body.username === 'testuser' && body.password === 'TestPassword123!') {
      return HttpResponse.json({
        ...mockTokens,
        user: mockUser,
      });
    }
    
    // Simulate failed login
    return HttpResponse.json(
      { message: 'Invalid credentials' },
      { status: 401 }
    );
  }),

  http.post(`${API_URL}/api/v1/auth/register`, async ({ request }) => {
    const body = await request.json();
    
    return HttpResponse.json({
      ...mockTokens,
      user: {
        ...mockUser,
        username: body.username,
        email: body.email,
      },
    });
  }),

  http.post(`${API_URL}/api/v1/auth/logout`, () => {
    return HttpResponse.json({ message: 'Logged out successfully' });
  }),

  http.post(`${API_URL}/api/v1/auth/refresh`, () => {
    return HttpResponse.json({
      access_token: 'new-access-token',
      token_type: 'Bearer',
      expires_in: 900,
    });
  }),

  http.get(`${API_URL}/api/v1/auth/me`, () => {
    return HttpResponse.json(mockUser);
  }),

  // User endpoints
  http.get(`${API_URL}/api/v1/users/me`, () => {
    return HttpResponse.json({
      ...mockUser,
      bio: 'Test user bio',
      avatar_url: null,
      privacy_settings: {
        profile_visibility: 'public',
        show_email: false,
        show_full_name: true,
        allow_messages: 'all',
      },
    });
  }),

  http.put(`${API_URL}/api/v1/users/me`, async ({ request }) => {
    const body = await request.json();
    
    return HttpResponse.json({
      ...mockUser,
      ...body,
    });
  }),

  http.get(`${API_URL}/api/v1/users/:userId`, ({ params }) => {
    return HttpResponse.json({
      id: params.userId,
      username: 'otheruser',
      email: 'other@unityplan.dk',
      full_name: 'Other User',
      territory_code: 'dk',
      is_active: true,
      created_at: '2025-11-08T12:00:00Z',
    });
  }),
];
