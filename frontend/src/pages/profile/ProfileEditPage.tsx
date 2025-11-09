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
  getPrivacySettings,
  updatePrivacySettings,
  type UserProfile,
  type PrivacySettings,
  type UpdateProfileRequest,
  type UpdatePrivacySettingsRequest,
} from '@/api/users';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

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
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [privacy, setPrivacy] = useState<PrivacySettings | null>(null);
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
        
        const [profileData, privacyData] = await Promise.all([
          getUserProfile(user.username, user.territory_code),
          getPrivacySettings(user.territory_code),
        ]);

        setProfile(profileData);
        setPrivacy(privacyData);

        // Set form default values
        reset({
          full_name: profileData.full_name || '',
          bio: profileData.bio || '',
          location: profileData.location || '',
          website: profileData.website || '',
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
      await deleteAvatar(user.territory_code);
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
      await updateProfile({
        full_name: data.full_name || null,
        bio: data.bio || null,
        location: data.location || null,
        website: data.website || null,
      }, user.territory_code);

      // Upload avatar if changed
      if (avatarFile) {
        await uploadAvatar(avatarFile, user.territory_code);
      }

      // Update privacy settings if changed
      if (privacy) {
        await updatePrivacySettings({
          profile_visibility: privacy.profile_visibility,
          show_email: privacy.show_email,
          show_location: privacy.show_location,
          show_connections: privacy.show_connections,
          allow_messages_from: privacy.allow_messages_from,
        }, user.territory_code);
      }

      setSuccess('Profile updated successfully');
      
      // Reload user data
      // TODO: Implement proper user data refresh
      setTimeout(() => {
        window.location.href = '/profile';
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
      <div className="flex min-h-screen items-center justify-center bg-background">
        <div className="text-muted-foreground">Loading profile...</div>
      </div>
    );
  }

  if (error && !profile) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-background p-4">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle>Error</CardTitle>
            <CardDescription>{error}</CardDescription>
          </CardHeader>
          <CardContent>
            <Button onClick={() => window.location.href = '/profile'}>
              Back to Profile
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-background p-4">
      <div className="mx-auto max-w-2xl space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <h1 className="text-3xl font-bold">Edit Profile</h1>
          <Button
            variant="outline"
            onClick={() => window.location.href = '/profile'}
            disabled={isSaving}
          >
            Cancel
          </Button>
        </div>

        {/* Success/Error Messages */}
        {success && (
          <div className="rounded-lg bg-green-50 p-4 text-green-800 border border-green-200">
            {success}
          </div>
        )}
        {error && (
          <div className="rounded-lg bg-red-50 p-4 text-red-800 border border-red-200">
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
                      variant="destructive"
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

          {/* Privacy Settings Section */}
          {privacy && (
            <Card>
              <CardHeader>
                <CardTitle>Privacy Settings</CardTitle>
                <CardDescription>Control who can see your information</CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                {/* Profile Visibility */}
                <div className="space-y-2">
                  <Label htmlFor="profile_visibility">Profile Visibility</Label>
                  <select
                    id="profile_visibility"
                    value={privacy.profile_visibility}
                    onChange={(e) =>
                      setPrivacy({
                        ...privacy,
                        profile_visibility: e.target.value as 'public' | 'followers_only' | 'private',
                      })
                    }
                    disabled={isSaving}
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
                  >
                    <option value="public">Public - Anyone can view</option>
                    <option value="followers_only">Followers Only - Only followers can view</option>
                    <option value="private">Private - Only you can view</option>
                  </select>
                </div>

                {/* Privacy Toggles */}
                <div className="space-y-3">
                  <div className="flex items-center space-x-2">
                    <Checkbox
                      id="show_email"
                      checked={privacy.show_email}
                      onCheckedChange={(checked) =>
                        setPrivacy({ ...privacy, show_email: checked as boolean })
                      }
                      disabled={isSaving}
                    />
                    <Label htmlFor="show_email" className="cursor-pointer">
                      Show email address on profile
                    </Label>
                  </div>

                  <div className="flex items-center space-x-2">
                    <Checkbox
                      id="show_location"
                      checked={privacy.show_location}
                      onCheckedChange={(checked) =>
                        setPrivacy({ ...privacy, show_location: checked as boolean })
                      }
                      disabled={isSaving}
                    />
                    <Label htmlFor="show_location" className="cursor-pointer">
                      Show location on profile
                    </Label>
                  </div>

                  <div className="flex items-center space-x-2">
                    <Checkbox
                      id="show_connections"
                      checked={privacy.show_connections}
                      onCheckedChange={(checked) =>
                        setPrivacy({ ...privacy, show_connections: checked as boolean })
                      }
                      disabled={isSaving}
                    />
                    <Label htmlFor="show_connections" className="cursor-pointer">
                      Show connections/followers on profile
                    </Label>
                  </div>

                  {/* Allow Messages From */}
                  <div className="space-y-2">
                    <Label htmlFor="allow_messages_from">Who can send you messages</Label>
                    <select
                      id="allow_messages_from"
                      value={privacy.allow_messages_from}
                      onChange={(e) =>
                        setPrivacy({
                          ...privacy,
                          allow_messages_from: e.target.value as 'everyone' | 'followers_only' | 'nobody',
                        })
                      }
                      disabled={isSaving}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
                    >
                      <option value="everyone">Everyone</option>
                      <option value="followers_only">Followers Only</option>
                      <option value="nobody">Nobody</option>
                    </select>
                  </div>
                </div>
              </CardContent>
            </Card>
          )}

          {/* Action Buttons */}
          <div className="flex gap-4">
            <Button type="submit" disabled={isSaving} className="flex-1">
              {isSaving ? 'Saving...' : 'Save Changes'}
            </Button>
            <Button
              type="button"
              variant="outline"
              onClick={() => window.location.href = '/profile'}
              disabled={isSaving}
            >
              Cancel
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
}
