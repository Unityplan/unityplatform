import { useEffect, useState } from 'react';
import { useAuthStore } from '@/stores/authStore';
import { getUserProfile } from '@/api/users';
import type { UserProfile } from '@/api/users';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

export function ProfileViewPage() {
  const { user } = useAuthStore();
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string>('');

  useEffect(() => {
    const loadProfile = async () => {
      if (!user) {
        setError('You must be logged in to view your profile');
        setIsLoading(false);
        return;
      }

      try {
        setIsLoading(true);
        setError('');
        const profileData = await getUserProfile(user.username, user.territory_code);
        setProfile(profileData);
      } catch (err) {
        setError('Failed to load profile');
        console.error('Failed to load profile:', err);
      } finally {
        setIsLoading(false);
      }
    };

    loadProfile();
  }, [user]);

  if (isLoading) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-background">
        <div className="text-muted-foreground">Loading profile...</div>
      </div>
    );
  }

  if (error || !profile) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-background p-4">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle>Error</CardTitle>
            <CardDescription>{error || 'Profile not found'}</CardDescription>
          </CardHeader>
          <CardContent>
            <Button onClick={() => window.location.href = '/login'}>
              Go to Login
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-background p-4">
      <div className="mx-auto max-w-4xl space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <h1 className="text-3xl font-bold">Profile</h1>
          <Button onClick={() => window.location.href = '/profile/edit'}>
            Edit Profile
          </Button>
        </div>

        {/* Profile Card */}
        <Card>
          <CardHeader>
            <div className="flex items-start gap-4">
              {/* Avatar */}
              <div className="h-20 w-20 rounded-full bg-muted flex items-center justify-center text-2xl font-bold">
                {profile.avatar_url ? (
                  <img
                    src={profile.avatar_url}
                    alt={user?.username}
                    className="h-full w-full rounded-full object-cover"
                  />
                ) : (
                  <span>{user?.username.charAt(0).toUpperCase()}</span>
                )}
              </div>

              {/* User Info */}
              <div className="flex-1">
                <CardTitle className="text-2xl">{profile.full_name || user?.username}</CardTitle>
                <CardDescription className="text-lg">@{user?.username}</CardDescription>
                {user?.email && (
                  <p className="text-sm text-muted-foreground mt-1">{user.email}</p>
                )}
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Bio */}
            {profile.bio && (
              <div>
                <h3 className="text-sm font-medium text-muted-foreground mb-1">About</h3>
                <p className="text-foreground">{profile.bio}</p>
              </div>
            )}

            {/* Location */}
            {profile.location && (
              <div>
                <h3 className="text-sm font-medium text-muted-foreground mb-1">Location</h3>
                <p className="text-foreground">{profile.location}</p>
              </div>
            )}

            {/* Website */}
            {profile.website && (
              <div>
                <h3 className="text-sm font-medium text-muted-foreground mb-1">Website</h3>
                <a
                  href={profile.website}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-primary hover:underline"
                >
                  {profile.website}
                </a>
              </div>
            )}

            {/* Member Since */}
            <div>
              <h3 className="text-sm font-medium text-muted-foreground mb-1">Member Since</h3>
              <p className="text-foreground">
                {new Date(profile.created_at).toLocaleDateString('en-US', {
                  year: 'numeric',
                  month: 'long',
                  day: 'numeric',
                })}
              </p>
            </div>

            {/* Territory */}
            <div>
              <h3 className="text-sm font-medium text-muted-foreground mb-1">Territory</h3>
              <p className="text-foreground uppercase">{user?.territory_code}</p>
            </div>
          </CardContent>
        </Card>

        {/* Actions Card */}
        <Card>
          <CardHeader>
            <CardTitle>Account Actions</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            <Button
              variant="outline"
              className="w-full justify-start"
              onClick={() => window.location.href = '/profile/edit'}
            >
              Edit Profile
            </Button>
            <Button
              variant="outline"
              className="w-full justify-start"
              onClick={() => window.location.href = '/settings/privacy'}
            >
              Privacy Settings
            </Button>
            <Button
              variant="outline"
              className="w-full justify-start"
              onClick={() => {
                useAuthStore.getState().logout();
                window.location.href = '/login';
              }}
            >
              Sign Out
            </Button>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
