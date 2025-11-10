import { useEffect, useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { useAuthStore } from '@/stores/authStore';
import {
    getUserProfile,
    updateProfile,
    uploadAvatar,
    deleteAvatar,
    type UserProfile,
} from '@/api/users';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { AppLayout } from '@/components/layouts/AppLayout';
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb';
import { Link, useRouter } from '@tanstack/react-router';
import { Home } from 'lucide-react';

// Validation schemas
const profileSchema = z.object({
    full_name: z.string().max(255, 'Name must be 255 characters or less').optional(),
    bio: z.string().max(500, 'Bio must be 500 characters or less').optional(),
    location: z.string().max(255, 'Location must be 255 characters or less').optional(),
    website: z.string().url('Must be a valid URL').max(255).optional().or(z.literal('')),
});

type ProfileFormData = z.infer<typeof profileSchema>;

export function ProfileEditPage() {
    const { user } = useAuthStore();
    const router = useRouter();
    const [profile, setProfile] = useState<UserProfile | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [isSaving, setIsSaving] = useState(false);
    const [error, setError] = useState<string>('');
    const [success, setSuccess] = useState<string>('');
    const [avatarFile, setAvatarFile] = useState<File | null>(null);
    const [avatarPreview, setAvatarPreview] = useState<string>('');

    const {
        register,
        handleSubmit,
        formState: { errors },
        reset,
    } = useForm<ProfileFormData>({
        resolver: zodResolver(profileSchema),
    });

    // Load profile data
    useEffect(() => {
        const loadData = async () => {
            if (!user) {
                setError('You must be logged in to edit your profile');
                setIsLoading(false);
                return;
            }

            try {
                setIsLoading(true);
                setError('');

                const profileData = await getUserProfile(user.id);

                setProfile(profileData);

                // Set form default values
                reset({
                    full_name: profileData.full_name || '',
                    bio: profileData.bio || '',
                    location: profileData.location || '',
                    website: profileData.website_url || '',
                });

                if (profileData.avatar_url) {
                    setAvatarPreview(profileData.avatar_url);
                }
            } catch (err) {
                setError('Failed to load profile data');
                console.error('Failed to load profile:', err);
            } finally {
                setIsLoading(false);
            }
        };

        loadData();
    }, [user, reset]);

    // Handle avatar file selection
    const handleAvatarChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const file = e.target.files?.[0];
        if (file) {
            setAvatarFile(file);
            const reader = new FileReader();
            reader.onloadend = () => {
                setAvatarPreview(reader.result as string);
            };
            reader.readAsDataURL(file);
        }
    };

    // Handle avatar deletion
    const handleDeleteAvatar = async () => {
        if (!user) return;

        try {
            setIsSaving(true);
            setError('');
            await deleteAvatar(user.id);
            setAvatarFile(null);
            setAvatarPreview('');
            setSuccess('Avatar deleted successfully');
        } catch (err) {
            setError('Failed to delete avatar');
            console.error('Failed to delete avatar:', err);
        } finally {
            setIsSaving(false);
        }
    };

    // Handle profile form submission
    const onSubmit = async (data: ProfileFormData) => {
        if (!user) return;

        try {
            setIsSaving(true);
            setError('');
            setSuccess('');

            // Update profile info
            await updateProfile(user.id, {
                full_name: data.full_name || null,
                bio: data.bio || null,
                location: data.location || null,
                website_url: data.website || null,
            });

            // Upload avatar if changed
            if (avatarFile) {
                await uploadAvatar(user.id, avatarFile);
            }

            setSuccess('Profile updated successfully');

            // Reload user data and navigate
            setTimeout(() => {
                router.navigate({ to: '/profile' });
            }, 1500);
        } catch (err) {
            setError('Failed to update profile');
            console.error('Failed to update profile:', err);
        } finally {
            setIsSaving(false);
        }
    };

    if (isLoading) {
        return (
            <AppLayout
                breadcrumbs={
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink asChild>
                                    <Link to="/">
                                        <Home className="size-4" />
                                    </Link>
                                </BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbLink asChild>
                                    <Link to="/profile">Profile</Link>
                                </BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbPage>Edit</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                }
            >
                <div className="flex items-center justify-center py-12">
                    <div className="text-muted-foreground">Loading profile...</div>
                </div>
            </AppLayout>
        );
    }

    if (error && !profile) {
        return (
            <AppLayout
                breadcrumbs={
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink asChild>
                                    <Link to="/">
                                        <Home className="size-4" />
                                    </Link>
                                </BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbLink asChild>
                                    <Link to="/profile">Profile</Link>
                                </BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbPage>Edit</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                }
            >
                <Card className="w-full max-w-md mx-auto">
                    <CardHeader>
                        <CardTitle>Error</CardTitle>
                        <CardDescription>{error}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <Button onClick={() => router.navigate({ to: '/profile' })}>
                            Back to Profile
                        </Button>
                    </CardContent>
                </Card>
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
                                <Link to="/">
                                    <Home className="size-4" />
                                </Link>
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbLink asChild>
                                <Link to="/profile">Profile</Link>
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage>Edit</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                {/* Header */}
                <div className="flex items-center justify-between">
                    <h1 className="text-3xl font-bold">Edit Profile</h1>
                    <Button
                        onClick={() => router.navigate({ to: '/profile' })}
                        disabled={isSaving}
                    >
                        Cancel
                    </Button>
                </div>

                {/* Success/Error Messages */}
                {success && (
                    <div className="rounded-lg bg-green-100 dark:bg-green-900/20 p-4 text-green-800 dark:text-green-200 border border-green-200 dark:border-green-800">
                        {success}
                    </div>
                )}
                {error && (
                    <div className="rounded-lg bg-red-100 dark:bg-red-900/20 p-4 text-red-800 dark:text-red-200 border border-red-200 dark:border-red-800">
                        {error}
                    </div>
                )}

                <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                    {/* Avatar Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Profile Picture</CardTitle>
                            <CardDescription>Upload a photo to represent yourself</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="flex items-center gap-4">
                                {/* Avatar Preview */}
                                <div className="h-24 w-24 rounded-full bg-muted flex items-center justify-center text-3xl font-bold overflow-hidden">
                                    {avatarPreview ? (
                                        <img
                                            src={avatarPreview}
                                            alt="Avatar preview"
                                            className="h-full w-full object-cover"
                                        />
                                    ) : (
                                        <span>{user?.username.charAt(0).toUpperCase()}</span>
                                    )}
                                </div>

                                {/* Upload Controls */}
                                <div className="flex-1 space-y-2">
                                    <Input
                                        type="file"
                                        accept="image/*"
                                        onChange={handleAvatarChange}
                                        disabled={isSaving}
                                    />
                                    {avatarPreview && (
                                        <Button
                                            type="button"
                                            size="sm"
                                            onClick={handleDeleteAvatar}
                                            disabled={isSaving}
                                        >
                                            Delete Avatar
                                        </Button>
                                    )}
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    {/* Basic Info Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Basic Information</CardTitle>
                            <CardDescription>Update your public profile information</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            {/* Username (readonly) */}
                            <div className="space-y-2">
                                <Label htmlFor="username">Username</Label>
                                <Input
                                    id="username"
                                    value={user?.username}
                                    disabled
                                    className="bg-muted"
                                />
                                <p className="text-sm text-muted-foreground">
                                    Username cannot be changed
                                </p>
                            </div>

                            {/* Full Name */}
                            <div className="space-y-2">
                                <Label htmlFor="full_name">Full Name</Label>
                                <Input
                                    id="full_name"
                                    {...register('full_name')}
                                    disabled={isSaving}
                                    placeholder="Your full name"
                                />
                                {errors.full_name && (
                                    <p className="text-sm text-destructive">{errors.full_name.message}</p>
                                )}
                            </div>

                            {/* Bio */}
                            <div className="space-y-2">
                                <Label htmlFor="bio">Bio</Label>
                                <textarea
                                    id="bio"
                                    {...register('bio')}
                                    disabled={isSaving}
                                    placeholder="Tell us about yourself..."
                                    className="flex min-h-[100px] w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
                                    maxLength={500}
                                />
                                {errors.bio && (
                                    <p className="text-sm text-destructive">{errors.bio.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    {register('bio').name ? '0' : '0'}/500 characters
                                </p>
                            </div>

                            {/* Location */}
                            <div className="space-y-2">
                                <Label htmlFor="location">Location</Label>
                                <Input
                                    id="location"
                                    {...register('location')}
                                    disabled={isSaving}
                                    placeholder="City, Country"
                                />
                                {errors.location && (
                                    <p className="text-sm text-destructive">{errors.location.message}</p>
                                )}
                            </div>

                            {/* Website */}
                            <div className="space-y-2">
                                <Label htmlFor="website">Website</Label>
                                <Input
                                    id="website"
                                    type="url"
                                    {...register('website')}
                                    disabled={isSaving}
                                    placeholder="https://example.com"
                                />
                                {errors.website && (
                                    <p className="text-sm text-destructive">{errors.website.message}</p>
                                )}
                            </div>
                        </CardContent>
                    </Card>

                    {/* Action Buttons */}
                    <div className="flex gap-4">
                        <Button type="submit" disabled={isSaving} className="flex-1">
                            {isSaving ? 'Saving...' : 'Save Changes'}
                        </Button>
                        <Button
                            type="button"
                            onClick={() => router.navigate({ to: '/profile' })}
                            disabled={isSaving}
                        >
                            Cancel
                        </Button>
                    </div>
                </form>
            </div>
        </AppLayout>
    );
}
