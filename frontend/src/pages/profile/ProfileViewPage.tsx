import { useEffect, useState } from 'react';
import { useAuthStore } from '@/stores/authStore';
import { getUserProfile } from '@/api/users';
import type { UserProfile } from '@/api/users';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { AppLayout } from '@/components/layouts/AppLayout';
import { ProfileHeader } from '@/components/user';
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb';
import { Link, useNavigate } from '@tanstack/react-router';
import { Home } from 'lucide-react';

export function ProfileViewPage() {
    const { user } = useAuthStore();
    const navigate = useNavigate();
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
                // Get profile for current user
                const profileData = await getUserProfile(user.id);
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
            <AppLayout
                breadcrumbs={
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink asChild>
                                    <Link to="/"><Home className="size-4" /></Link>
                                </BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbPage>Profile</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                }
            >
                <div className="flex min-h-[50vh] items-center justify-center">
                    <div className="text-muted-foreground">Loading profile...</div>
                </div>
            </AppLayout>
        );
    }

    if (error || !profile) {
        return (
            <AppLayout
                breadcrumbs={
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink asChild>
                                    <Link to="/"><Home className="size-4" /></Link>
                                </BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbPage>Profile</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                }
            >
                <div className="flex min-h-[50vh] items-center justify-center p-4">
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
            </AppLayout>
        );
    }

    return (
        <AppLayout
            breadcrumbs={
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink asChild>
                                <Link to="/"><Home className="size-4" /></Link>
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage>Profile</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                {/* Profile Header - Social Network Style */}
                <ProfileHeader
                    user={{
                        id: user!.id,
                        username: user!.username,
                        full_name: profile.full_name,
                        avatar_url: profile.avatar_url,
                        bio: profile.bio,
                        location: profile.location,
                        website: undefined, // TODO: Add website field to UserProfile type
                        created_at: profile.created_at || new Date().toISOString(),
                        is_verified: false, // TODO: Add is_verified field to backend
                    }}
                    stats={{
                        following: 0, // TODO: Fetch real stats from connections API
                        followers: 0,
                        posts: 0,
                    }}
                    isOwnProfile={true}
                    onEditProfile={() => navigate({ to: '/profile/edit' })}
                />

                {/* Account Actions Card */}
                <Card>
                    <CardHeader>
                        <CardTitle>Account Actions</CardTitle>
                    </CardHeader>
                    <CardContent className="space-y-2">
                        <Button
                            className="w-full justify-start"
                            onClick={() => navigate({ to: '/profile/edit' })}
                        >
                            Edit Profile
                        </Button>
                        <Button
                            className="w-full justify-start"
                            onClick={() => navigate({ to: '/settings/privacy' })}
                        >
                            Privacy Settings
                        </Button>
                        <Button
                            className="w-full justify-start"
                            onClick={() => {
                                useAuthStore.getState().logout();
                                navigate({ to: '/login' });
                            }}
                        >
                            Sign Out
                        </Button>
                    </CardContent>
                </Card>
            </div>
        </AppLayout>
    );
}
