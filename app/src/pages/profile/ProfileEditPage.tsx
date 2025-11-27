import { useEffect, useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { useDropzone } from 'react-dropzone';
import { useAuthStore } from '@/stores/authStore';
import {
    getUserProfile,
    updateProfile,
    uploadAvatar,
    deleteAvatar,
    type UserProfile,
    type ProfileLink,
    type LanguageProficiency,
    getProfileLinks,
    createProfileLink,
    updateProfileLink,
    deleteProfileLink,
    getLanguageProficiencies,
    createLanguageProficiency,
    deleteLanguageProficiency,
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
import { Home, Upload, X } from 'lucide-react';
import { Textarea } from '@/components/ui/textarea';
import { TagInput } from '@/components/ui/tag-input';
import { LanguageProficiencyManager } from '@/components/LanguageProficiencyManager';
import { ProfileLinksManager } from '@/components/ProfileLinksManager';
import { LocationPicker } from '@/components/LocationPicker';
import { toast } from 'sonner';

// Validation schemas
const profileSchema = z.object({
    displayName: z.string().max(100, 'Display name must be 100 characters or less').optional(),
    bio: z.string().max(280, 'Bio must be 280 characters or less').optional(),
    about: z.string().max(2000, 'About must be 2000 characters or less').optional(),
    location: z.string().max(100, 'Location must be 100 characters or less').optional(),
    website: z.string().url('Must be a valid URL').optional().or(z.literal('')),
    interests: z.array(z.string()).optional(), // Array of interest tags
    skills: z.array(z.string()).optional(), // Array of skill tags
});

type ProfileFormData = z.infer<typeof profileSchema>;

export function ProfileEditPage() {
    const { user } = useAuthStore();
    const router = useRouter();
    const [profile, setProfile] = useState<UserProfile | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [isSaving, setIsSaving] = useState(false);
    const [avatarFile, setAvatarFile] = useState<File | null>(null);
    const [avatarPreview, setAvatarPreview] = useState<string>('');

    // Links state
    const [links, setLinks] = useState<ProfileLink[]>([]);

    // Languages state
    const [languages, setLanguages] = useState<LanguageProficiency[]>([]);

    const {
        register,
        handleSubmit,
        formState: { errors },
        reset,
        watch,
        setValue,
    } = useForm<ProfileFormData>({
        resolver: zodResolver(profileSchema),
    });

    // Load profile data
    useEffect(() => {
        const loadData = async () => {
            if (!user) {
                toast.error('You must be logged in to edit your profile');
                setIsLoading(false);
                return;
            }

            try {
                setIsLoading(true);

                // Load profile, links, and languages in parallel
                const [profileData, linksData, languagesData] = await Promise.all([
                    getUserProfile(user.id),
                    getProfileLinks().catch(() => []),
                    getLanguageProficiencies().catch(() => []),
                ]);

                setProfile(profileData);
                // Ensure we always have arrays even if API returns unexpected data
                setLinks(Array.isArray(linksData) ? linksData : []);
                setLanguages(Array.isArray(languagesData) ? languagesData : []);

                // Set form default values
                reset({
                    displayName: profileData.displayName || '',
                    bio: profileData.bio || '',
                    about: profileData.about || '',
                    location: profileData.location || '',
                    website: profileData.website || '',
                    interests: profileData.interests || [],
                    skills: profileData.skills || [],
                });

                if (profileData.avatarUrl) {
                    setAvatarPreview(profileData.avatarUrl);
                }
            } catch (err) {
                toast.error('Failed to load profile data');
                console.error('Failed to load profile:', err);
            } finally {
                setIsLoading(false);
            }
        };

        loadData();
    }, [user, reset]);

    // Load profile links
    const loadLinks = async () => {
        try {
            const linksData = await getProfileLinks();
            setLinks(linksData);
        } catch (err) {
            console.error('Failed to load links:', err);
        }
    };

    // Load language proficiencies
    const loadLanguages = async () => {
        try {
            const languagesData = await getLanguageProficiencies();
            setLanguages(languagesData);
        } catch (err) {
            console.error('Failed to load languages:', err);
        }
    };

    // Handle avatar file selection with react-dropzone
    const onDrop = (acceptedFiles: File[]) => {
        const file = acceptedFiles[0];
        if (file) {
            setAvatarFile(file);
            const reader = new FileReader();
            reader.onloadend = () => {
                setAvatarPreview(reader.result as string);
            };
            reader.readAsDataURL(file);
        }
    };

    const { getRootProps, getInputProps, isDragActive } = useDropzone({
        onDrop,
        accept: {
            'image/*': ['.png', '.jpg', '.jpeg', '.gif', '.webp']
        },
        maxFiles: 1,
        multiple: false,
    });

    // Handle avatar deletion
    const handleDeleteAvatar = async () => {
        if (!user) return;

        try {
            setIsSaving(true);
            await deleteAvatar(user.id);
            setAvatarFile(null);
            setAvatarPreview('');
            toast.success('Avatar deleted successfully');
        } catch (err) {
            toast.error('Failed to delete avatar');
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

            // Update profile info
            await updateProfile({
                displayName: data.displayName || null,
                bio: data.bio || null,
                about: data.about || null,
                location: data.location || null,
                website: data.website || null,
                interests: data.interests && data.interests.length > 0 ? data.interests : null,
                skills: data.skills && data.skills.length > 0 ? data.skills : null,
            });

            // Upload avatar if changed
            if (avatarFile) {
                await uploadAvatar(user.id, avatarFile);
            }

            toast.success('Profile updated successfully');

            // Reload user data and navigate
            setTimeout(() => {
                router.navigate({ to: '/profile' });
            }, 1500);
        } catch (err) {
            toast.error('Failed to update profile');
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

    if (!profile || !user) {
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
                        <CardTitle>Profile Not Found</CardTitle>
                        <CardDescription>Unable to load profile data</CardDescription>
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
                    <div className="flex gap-2">
                        <Button
                            variant="outline"
                            onClick={() => router.navigate({ to: '/profile' })}
                            disabled={isSaving}
                        >
                            Back to Profile
                        </Button>
                        <Button
                            onClick={() => {
                                const submitButton = document.querySelector('button[type="submit"]') as HTMLButtonElement;
                                submitButton?.click();
                            }}
                            disabled={isSaving}
                        >
                            {isSaving ? 'Saving...' : 'Save Changes'}
                        </Button>
                    </div>
                </div>

                <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                    {/* Profile Picture & Basic Info Grid */}
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        {/* Avatar Section */}
                        <Card>
                            <CardHeader>
                                <CardTitle>Profile Picture</CardTitle>
                                <CardDescription>Upload a photo to represent yourself</CardDescription>
                            </CardHeader>
                            <CardContent className="space-y-4">
                                {/* Avatar Preview */}
                                <div className="flex justify-center">
                                    <div className="h-32 w-32 rounded-full bg-muted flex items-center justify-center text-4xl font-bold overflow-hidden ring-2 ring-border">
                                        {avatarPreview ? (
                                            <img
                                                src={avatarPreview}
                                                alt="Avatar preview"
                                                className="h-full w-full object-cover"
                                            />
                                        ) : (
                                            <span>{user.username?.charAt(0).toUpperCase() ?? 'U'}</span>
                                        )}
                                    </div>
                                </div>

                                {/* Upload Dropzone */}
                                <div
                                    {...getRootProps()}
                                    className={`
                                        relative border-2 border-dashed rounded-lg p-6 text-center cursor-pointer transition-colors
                                        ${isDragActive
                                            ? 'border-primary bg-primary/5'
                                            : 'border-muted-foreground/25 hover:border-primary/50 hover:bg-accent/50'
                                        }
                                        ${isSaving ? 'opacity-50 cursor-not-allowed' : ''}
                                    `}
                                >
                                    <input {...getInputProps()} disabled={isSaving} />
                                    <Upload className="mx-auto h-8 w-8 text-muted-foreground mb-2" />
                                    <p className="text-sm font-medium mb-1">
                                        {isDragActive ? 'Drop the image here' : 'Click to upload or drag and drop'}
                                    </p>
                                    <p className="text-xs text-muted-foreground">
                                        PNG, JPG, GIF up to 10MB
                                    </p>
                                </div>

                                {/* Delete Button */}
                                {avatarPreview && (
                                    <Button
                                        type="button"
                                        variant="outline"
                                        size="sm"
                                        onClick={handleDeleteAvatar}
                                        disabled={isSaving}
                                        className="w-full"
                                    >
                                        <X className="mr-2 h-4 w-4" />
                                        Remove Avatar
                                    </Button>
                                )}
                            </CardContent>
                        </Card>

                        {/* Basic Info Section */}
                        <Card>
                            <CardHeader>
                                <CardTitle>Basic Information</CardTitle>
                                <CardDescription>Your identity and display name</CardDescription>
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

                                {/* Display Name */}
                                <div className="space-y-2">
                                    <Label htmlFor="displayName">Display Name</Label>
                                    <Input
                                        id="displayName"
                                        {...register('displayName')}
                                        disabled={isSaving}
                                        placeholder="How you'd like to be called"
                                    />
                                    {errors.displayName && (
                                        <p className="text-sm text-destructive">{errors.displayName.message}</p>
                                    )}
                                    <p className="text-sm text-muted-foreground">
                                        This name will be displayed on your profile
                                    </p>
                                </div>

                                {/* Website */}
                                <div className="space-y-2">
                                    <Label htmlFor="website">Website</Label>
                                    <Input
                                        id="website"
                                        {...register('website')}
                                        disabled={isSaving}
                                        placeholder="https://example.com"
                                        type="url"
                                    />
                                    {errors.website && (
                                        <p className="text-sm text-destructive">{errors.website.message}</p>
                                    )}
                                </div>
                            </CardContent>
                        </Card>
                    </div>

                    {/* Location Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Location</CardTitle>
                            <CardDescription>Where you're located (optional)</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            {/* Interactive Location Picker with Map */}
                            <LocationPicker
                                value={watch('location')}
                                onChange={(value) => setValue('location', value)}
                                disabled={isSaving}
                            />
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
                                    maxLength={280}
                                />
                                {errors.bio && (
                                    <p className="text-sm text-destructive">{errors.bio.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Brief introduction shown on your profile (max 280 characters)
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
                                <TagInput
                                    value={watch('interests') || []}
                                    onChange={(tags) => setValue('interests', tags)}
                                    placeholder="Type an interest and press Enter"
                                    disabled={isSaving}
                                />
                                {errors.interests && (
                                    <p className="text-sm text-destructive">{errors.interests.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Add tags for your interests (e.g., Permaculture, Forest Ecology)
                                </p>
                            </div>

                            {/* Skills */}
                            <div className="space-y-2">
                                <Label htmlFor="skills">Skills</Label>
                                <TagInput
                                    value={watch('skills') || []}
                                    onChange={(tags) => setValue('skills', tags)}
                                    placeholder="Type a skill and press Enter"
                                    disabled={isSaving}
                                />
                                {errors.skills && (
                                    <p className="text-sm text-destructive">{errors.skills.message}</p>
                                )}
                                <p className="text-sm text-muted-foreground">
                                    Add tags for your skills (e.g., Composting, Rainwater Harvesting)
                                </p>
                            </div>
                        </CardContent>
                    </Card>

                    {/* Profile Links Section */}
                    <ProfileLinksManager
                        links={links}
                        onAdd={async (linkData) => {
                            await createProfileLink(linkData);
                            await loadLinks();
                        }}
                        onUpdate={async (id, updates) => {
                            await updateProfileLink(id, updates);
                            await loadLinks();
                        }}
                        onDelete={async (id) => {
                            await deleteProfileLink(id);
                            await loadLinks();
                        }}
                        disabled={isSaving}
                    />

                    {/* Language Proficiencies Section */}
                    <LanguageProficiencyManager
                        languages={languages}
                        onAdd={async (languageData) => {
                            await createLanguageProficiency(languageData);
                            await loadLanguages();
                        }}
                        onUpdate={async (id, updates) => {
                            // We need to add an update endpoint - for now we'll delete and recreate
                            await deleteLanguageProficiency(id);
                            const existingLang = languages.find((l) => l.id === id);
                            if (existingLang) {
                                await createLanguageProficiency({
                                    languageCode: existingLang.languageCode,
                                    languageName: existingLang.languageName,
                                    spokenLevel: updates.spokenLevel ?? existingLang.spokenLevel,
                                    writtenLevel: updates.writtenLevel ?? existingLang.writtenLevel,
                                    readingLevel: updates.readingLevel ?? existingLang.readingLevel,
                                    listeningLevel: updates.listeningLevel ?? existingLang.listeningLevel,
                                    displayOrder: existingLang.displayOrder,
                                    isPreferred: existingLang.isPreferred,
                                    showOnProfile: existingLang.showOnProfile,
                                });
                            }
                            await loadLanguages();
                        }}
                        onDelete={async (id) => {
                            await deleteLanguageProficiency(id);
                            await loadLanguages();
                        }}
                        disabled={isSaving}
                    />

                    {/* Action Buttons */}
                    <div className="flex justify-end gap-2">
                        <Button
                            type="button"
                            variant="outline"
                            onClick={() => router.navigate({ to: '/profile' })}
                            disabled={isSaving}
                        >
                            Back to Profile
                        </Button>
                        <Button type="submit" disabled={isSaving}>
                            {isSaving ? 'Saving...' : 'Save Changes'}
                        </Button>
                    </div>
                </form>
            </div>
        </AppLayout>
    );
}

