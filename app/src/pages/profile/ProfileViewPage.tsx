import { useEffect, useState } from 'react';
import { useAuthStore } from '@/stores/authStore';
import { getUserProfile, getFollowers, getFollowing } from '@/api/users';
import type { UserProfile } from '@/api/users';
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
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Link, useNavigate } from '@tanstack/react-router';
import { Home, Calendar, Heart, MessageCircle, Share2, MapPin, Link2, Mail } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';
import { getLocationDisplayName } from '@/lib/geocoding';

// Mock data - TODO: Replace with real API calls
const MOCK_POSTS = [
    {
        id: '1',
        content: 'Just completed my first course on Unity Platform! The learning experience has been amazing. Looking forward to more challenges ahead. 🎓',
        created_at: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(), // 2 hours ago
        likes: 12,
        comments: 3,
    },
    {
        id: '2',
        content: 'Excited to join the Data Science community! Anyone else interested in machine learning and AI?',
        created_at: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(), // 1 day ago
        likes: 8,
        comments: 5,
    },
    {
        id: '3',
        content: 'Great discussion in today\'s study group session. Thanks to everyone who participated!',
        created_at: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000).toISOString(), // 3 days ago
        likes: 15,
        comments: 2,
    },
];

const MOCK_ACTIVITY = [
    {
        id: '1',
        type: 'post',
        action: 'shared a new post',
        timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(), // 2 hours ago
    },
    {
        id: '2',
        type: 'follow',
        action: 'started following 3 new users',
        timestamp: new Date(Date.now() - 5 * 60 * 60 * 1000).toISOString(), // 5 hours ago
    },
    {
        id: '3',
        type: 'join',
        action: 'joined the Web Development community',
        timestamp: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(), // 1 day ago
    },
    {
        id: '4',
        type: 'post',
        action: 'commented on a discussion',
        timestamp: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(), // 2 days ago
    },
];


export function ProfileViewPage() {
    const { user } = useAuthStore();
    const navigate = useNavigate();
    const [profile, setProfile] = useState<UserProfile | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string>('');

    // Real stats from API
    const [stats, setStats] = useState({
        following: 0,
        followers: 0,
        posts: MOCK_POSTS.length, // TODO: Replace with real posts count when posts API is ready
    });

    // Mock following state - for when viewing other profiles
    const [isFollowing, setIsFollowing] = useState(false);

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

                // Load profile and connection stats in parallel
                const [profileData, followersData, followingData] = await Promise.all([
                    getUserProfile(user.id),
                    getFollowers(user.id, 1, 0), // Just get total count (limit 1)
                    getFollowing(user.id, 1, 0), // Just get total count (limit 1)
                ]);

                setProfile(profileData);
                setStats({
                    followers: followersData.total,
                    following: followingData.total,
                    posts: MOCK_POSTS.length,
                });
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

    if (error || !profile || !user) {
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
                            <button onClick={() => navigate({ to: '/login' })} className="text-primary hover:underline">
                                Go to Login
                            </button>
                        </CardContent>
                    </Card>
                </div>
            </AppLayout>
        );
    }

    const handleSignOut = () => {
        useAuthStore.getState().logout();
        navigate({ to: '/login' });
    };

    // Mock follow handler - TODO: Replace with real API call when viewing other profiles
    const handleFollow = () => {
        setIsFollowing(true);
        const newStats = { ...stats, followers: stats.followers + 1 };
        setStats(newStats);
        console.log('Followed user');
    };

    // Mock unfollow handler - TODO: Replace with real API call when viewing other profiles
    const handleUnfollow = () => {
        setIsFollowing(false);
        const newStats = { ...stats, followers: Math.max(0, stats.followers - 1) };
        setStats(newStats);
        console.log('Unfollowed user');
    };

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
                        id: user.id,
                        username: user.username,
                        fullName: profile.fullName || null,
                        avatarUrl: profile.avatarUrl || null,
                        bio: profile.bio || null,
                        location: profile.location || null,
                        website: profile.website || null,
                        createdAt: profile.createdAt || new Date().toISOString(),
                        isVerified: false, // TODO: Add isVerified field to backend
                    }}
                    stats={stats}
                    isOwnProfile={true}
                    isFollowing={isFollowing}
                    onFollow={handleFollow}
                    onUnfollow={handleUnfollow}
                    onEditProfile={() => navigate({ to: '/profile/edit' })}
                    onPrivacySettings={() => navigate({ to: '/settings/privacy' })}
                    onSignOut={handleSignOut}
                />

                {/* Profile Tabs */}
                <Tabs defaultValue="posts" className="w-full">
                    <TabsList className="grid w-full grid-cols-3">
                        <TabsTrigger value="posts">Posts</TabsTrigger>
                        <TabsTrigger value="activity">Activity</TabsTrigger>
                        <TabsTrigger value="about">About</TabsTrigger>
                    </TabsList>

                    {/* Posts Tab */}
                    <TabsContent value="posts" className="space-y-4">
                        {/* Mock Posts - TODO: Replace with real posts API */}
                        {MOCK_POSTS.map((post) => (
                            <Card key={post.id}>
                                <CardHeader>
                                    <div className="flex items-start justify-between">
                                        <div className="flex gap-3">
                                            <div className="flex size-10 items-center justify-center rounded-full bg-primary/10 text-sm font-semibold text-primary">
                                                {user.username?.substring(0, 2).toUpperCase() ?? 'UN'}
                                            </div>
                                            <div>
                                                <CardTitle className="text-base">{user.username ?? 'Unknown'}</CardTitle>
                                                <CardDescription className="flex items-center gap-1">
                                                    <Calendar className="size-3" />
                                                    {formatDistanceToNow(new Date(post.created_at), { addSuffix: true })}
                                                </CardDescription>
                                            </div>
                                        </div>
                                    </div>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <p className="text-sm">{post.content}</p>
                                    <div className="flex items-center gap-6 text-sm text-muted-foreground">
                                        <button className="flex items-center gap-1 transition-colors hover:text-primary">
                                            <Heart className="size-4" />
                                            <span>{post.likes}</span>
                                        </button>
                                        <button className="flex items-center gap-1 transition-colors hover:text-primary">
                                            <MessageCircle className="size-4" />
                                            <span>{post.comments}</span>
                                        </button>
                                        <button className="flex items-center gap-1 transition-colors hover:text-primary">
                                            <Share2 className="size-4" />
                                            <span>Share</span>
                                        </button>
                                    </div>
                                </CardContent>
                            </Card>
                        ))}
                        {MOCK_POSTS.length === 0 && (
                            <Card>
                                <CardContent className="flex min-h-[200px] items-center justify-center">
                                    <p className="text-muted-foreground">No posts yet</p>
                                </CardContent>
                            </Card>
                        )}
                    </TabsContent>

                    {/* Activity Tab */}
                    <TabsContent value="activity" className="space-y-4">
                        {/* Mock Activity - TODO: Replace with real activity API */}
                        {MOCK_ACTIVITY.map((activity) => (
                            <Card key={activity.id}>
                                <CardContent className="flex items-start gap-3 pt-6">
                                    <div className="flex size-8 items-center justify-center rounded-full bg-primary/10">
                                        {activity.type === 'follow' && <Heart className="size-4 text-primary" />}
                                        {activity.type === 'post' && <MessageCircle className="size-4 text-primary" />}
                                        {activity.type === 'join' && <Calendar className="size-4 text-primary" />}
                                    </div>
                                    <div className="flex-1 space-y-1">
                                        <p className="text-sm">
                                            <span className="font-semibold">{user.username ?? 'Unknown'}</span> {activity.action}
                                        </p>
                                        <p className="text-xs text-muted-foreground">
                                            {formatDistanceToNow(new Date(activity.timestamp), { addSuffix: true })}
                                        </p>
                                    </div>
                                </CardContent>
                            </Card>
                        ))}
                        {MOCK_ACTIVITY.length === 0 && (
                            <Card>
                                <CardContent className="flex min-h-[200px] items-center justify-center">
                                    <p className="text-muted-foreground">No recent activity</p>
                                </CardContent>
                            </Card>
                        )}
                    </TabsContent>

                    {/* About Tab */}
                    <TabsContent value="about">
                        <Card>
                            <CardHeader>
                                <CardTitle>About</CardTitle>
                                <CardDescription>Personal information and details</CardDescription>
                            </CardHeader>
                            <CardContent className="space-y-6">
                                {/* Bio */}
                                {profile.bio && (
                                    <div>
                                        <h3 className="mb-2 text-sm font-semibold">Bio</h3>
                                        <p className="text-sm text-muted-foreground">{profile.bio}</p>
                                    </div>
                                )}

                                {/* Details */}
                                <div className="space-y-3">
                                    <h3 className="text-sm font-semibold">Details</h3>

                                    {profile.location && (
                                        <div className="flex items-center gap-2 text-sm">
                                            <MapPin className="size-4 text-muted-foreground shrink-0" />
                                            <span>{getLocationDisplayName(profile.location)}</span>
                                        </div>
                                    )}

                                    {profile.website && (
                                        <div className="flex items-center gap-2 text-sm">
                                            <Link2 className="size-4 text-muted-foreground" />
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

                                    {user?.email && (
                                        <div className="flex items-center gap-2 text-sm">
                                            <Mail className="size-4 text-muted-foreground" />
                                            <span className="text-muted-foreground">{user.email}</span>
                                        </div>
                                    )}

                                    <div className="flex items-center gap-2 text-sm">
                                        <Calendar className="size-4 text-muted-foreground" />
                                        <span className="text-muted-foreground">
                                            Joined {formatDistanceToNow(new Date(profile.createdAt || new Date()), { addSuffix: true })}
                                        </span>
                                    </div>
                                </div>

                                {/* Empty State */}
                                {!profile.bio && !profile.location && !profile.website && (
                                    <div className="rounded-lg border border-dashed p-8 text-center">
                                        <p className="text-sm text-muted-foreground">
                                            Complete your profile by adding a bio, location, and website.
                                        </p>
                                        <button
                                            onClick={() => navigate({ to: '/profile/edit' })}
                                            className="mt-4 text-sm text-primary hover:underline"
                                        >
                                            Edit Profile
                                        </button>
                                    </div>
                                )}
                            </CardContent>
                        </Card>
                    </TabsContent>
                </Tabs>
            </div>
        </AppLayout>
    );
}
