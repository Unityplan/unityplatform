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
import { Textarea } from '@/components/ui/textarea';

// Validation schemas
const profileSchema = z.object({
    full_name: z.string().max(255, 'Name must be 255 characters or less').optional(),
    display_name: z.string().max(255, 'Display name must be 255 characters or less').optional(),
    bio: z.string().max(500, 'Bio must be 500 characters or less').optional(),
    about: z.string().max(2000, 'About must be 2000 characters or less').optional(),
    location: z.string().max(255, 'Location must be 255 characters or less').optional(),
    interests: z.string().optional(), // Comma-separated string, will be split into array
    skills: z.string().optional(), // Comma-separated string, will be split into array
    languages: z.string().optional(), // Comma-separated string, will be split into array
});

type ProfileFormData = z.infer<typeof profileSchema>;

// Helper functions for array fields
const arrayToString = (arr: string[] | null | undefined): string => {
    return arr?.join(', ') || '';
};

const stringToArray = (str: string | undefined): string[] | null => {
    if (!str || str.trim() === '') return null;
    return str.split(',').map(s => s.trim()).filter(s => s.length > 0);
};

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
                    display_name: profileData.display_name || '',
                    bio: profileData.bio || '',
                    about: profileData.about || '',
                    location: profileData.location || '',
                    interests: arrayToString(profileData.interests),
                    skills: arrayToString(profileData.skills),
                    languages: arrayToString(profileData.languages),
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
                display_name: data.display_name || null,
                bio: data.bio || null,
                about: data.about || null,
                location: data.location || null,
                interests: stringToArray(data.interests),
                skills: stringToArray(data.skills),
                languages: stringToArray(data.languages),
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
                            <CardDescription>Your identity and public display information</CardDescription>
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

                            {/* Display Name */}
                            <div className="space-y-2">
                                <Label htmlFor="display_name">Display Name</Label>
                                <Input
                                    id="display_name"
                                    {...register('display_name')}
                                    disabled={isSaving}
                                    placeholder="How you'd like to be called (optional)"
                                />
                                {errors.display_name && (
                                    <p className="text-sm text-destructive">{errors.display_name.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Alternative name to show instead of your full name
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
                                {/* TODO: Add interactive map component with marker
                                    - Use shadcn-map (https://shadcn-map.vercel.app/docs)
                                    - Show location on map with draggable marker
                                    - Geocode address to coordinates
                                    - Allow marker placement to set location
                                */}
                            </div>
                        </CardContent>
                    </Card>

                    {/* Bio & About Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Bio & About</CardTitle>
                            <CardDescription>Tell others about yourself</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            {/* Short Bio */}
                            <div className="space-y-2">
                                <Label htmlFor="bio">Short Bio</Label>
                                <Textarea
                                    id="bio"
                                    {...register('bio')}
                                    disabled={isSaving}
                                    placeholder="A brief description about yourself..."
                                    className="min-h-[100px] resize-none"
                                    maxLength={500}
                                />
                                {errors.bio && (
                                    <p className="text-sm text-destructive">{errors.bio.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Brief introduction shown on your profile (max 500 characters)
                                </p>
                            </div>

                            {/* Extended About */}
                            <div className="space-y-2">
                                <Label htmlFor="about">About Me</Label>
                                <Textarea
                                    id="about"
                                    {...register('about')}
                                    disabled={isSaving}
                                    placeholder="Tell your story, share your background, goals, and what you're passionate about..."
                                    className="min-h-[200px] resize-y"
                                    maxLength={2000}
                                />
                                {errors.about && (
                                    <p className="text-sm text-destructive">{errors.about.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Extended information shown in the About tab (max 2000 characters)
                                </p>
                            </div>
                        </CardContent>
                    </Card>

                    {/* Skills & Interests Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Skills & Interests</CardTitle>
                            <CardDescription>Share your expertise and passions</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            {/* Interests */}
                            <div className="space-y-2">
                                <Label htmlFor="interests">Interests</Label>
                                <Input
                                    id="interests"
                                    {...register('interests')}
                                    disabled={isSaving}
                                    placeholder="e.g., Permaculture, Forest Ecology, Beekeeping"
                                />
                                {errors.interests && (
                                    <p className="text-sm text-destructive">{errors.interests.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Separate multiple interests with commas
                                </p>
                            </div>

                            {/* Skills */}
                            <div className="space-y-2">
                                <Label htmlFor="skills">Skills</Label>
                                <Input
                                    id="skills"
                                    {...register('skills')}
                                    disabled={isSaving}
                                    placeholder="e.g., Composting, Rainwater Harvesting, Natural Building"
                                />
                                {errors.skills && (
                                    <p className="text-sm text-destructive">{errors.skills.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Separate multiple skills with commas
                                </p>
                            </div>

                            {/* Languages */}
                            <div className="space-y-2">
                                <Label htmlFor="languages">Languages</Label>
                                <Input
                                    id="languages"
                                    {...register('languages')}
                                    disabled={isSaving}
                                    placeholder="e.g., English (Native), Danish (Fluent), Swedish (Intermediate)"
                                />
                                {errors.languages && (
                                    <p className="text-sm text-destructive">{errors.languages.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Primary language first, then secondary languages with proficiency levels.
                                    This helps with translation features.
                                </p>
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
