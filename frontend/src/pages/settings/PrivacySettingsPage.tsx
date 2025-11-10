import { useState } from 'react';
import { AppLayout } from '@/components/layouts/AppLayout';
import { PrivacySettingsForm } from '@/components/user/PrivacySettingsForm';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb';
import { Link, useRouter } from '@tanstack/react-router';
import { Home, User, Shield, CheckCircle2, AlertCircle } from 'lucide-react';

// Privacy settings type (matches PrivacySettingsForm schema)
type PrivacySettings = {
    profile_visibility: 'public' | 'connections' | 'private';
    show_email: boolean;
    show_full_name: boolean;
    allow_messages_from: 'everyone' | 'connections' | 'nobody';
    show_activity_status: boolean;
    show_connections: boolean;
};

// Local storage key for mock privacy settings
const PRIVACY_SETTINGS_KEY = 'unityplan_privacy_settings';

/**
 * Privacy Settings Page
 * 
 * Allows users to manage their privacy and visibility settings.
 * Uses localStorage to persist settings until backend API is implemented.
 * 
 * Features:
 * - Profile visibility control (public/connections/private)
 * - Email and full name visibility toggles
 * - Message permissions
 * - Activity status visibility
 * - Connections list visibility
 * 
 * @example
 * Route: /settings/privacy
 */
export function PrivacySettingsPage() {
    const router = useRouter();
    const [isLoading, setIsLoading] = useState(false);
    const [saveSuccess, setSaveSuccess] = useState(false);
    const [saveError, setSaveError] = useState(false);

    // Load initial settings from localStorage
    const getInitialSettings = (): Partial<PrivacySettings> => {
        try {
            const stored = localStorage.getItem(PRIVACY_SETTINGS_KEY);
            return stored ? JSON.parse(stored) : {};
        } catch (error) {
            console.error('Failed to load privacy settings:', error);
            return {};
        }
    };

    const [initialValues] = useState<Partial<PrivacySettings>>(getInitialSettings());

    // Mock submit handler - TODO: Replace with actual API call
    const handleSubmit = async (values: PrivacySettings) => {
        setIsLoading(true);
        setSaveSuccess(false);
        setSaveError(false);

        try {
            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 800));

            // Save to localStorage for now
            localStorage.setItem(PRIVACY_SETTINGS_KEY, JSON.stringify(values));

            // TODO: Replace with actual API call
            // await updatePrivacySettings(values);

            setSaveSuccess(true);

            // Auto-hide success message after 3 seconds
            setTimeout(() => setSaveSuccess(false), 3000);
        } catch (error) {
            console.error('Failed to save privacy settings:', error);
            setSaveError(true);
        } finally {
            setIsLoading(false);
        }
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
                            <BreadcrumbLink asChild>
                                <Link to="/profile" className="flex items-center gap-1">
                                    <User className="size-4" /> Profile
                                </Link>
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage className="flex items-center gap-1">
                                <Shield className="size-4" />
                                Privacy Settings
                            </BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                {/* Page Header */}
                <div className="flex items-center justify-between">
                    <div className="space-y-2">
                        <h1 className="text-3xl font-bold tracking-tight">Privacy Settings</h1>
                        <p className="text-muted-foreground">
                            Manage who can see your profile and how others can interact with you.
                        </p>
                    </div>
                    <Button
                        onClick={() => router.history.back()}
                        disabled={isLoading}
                    >
                        Cancel
                    </Button>
                </div>

                {/* Privacy Settings Form */}
                <PrivacySettingsForm
                    initialValues={initialValues}
                    onSubmit={handleSubmit}
                    isLoading={isLoading}
                />

                {/* Helper Text */}
                <div className="rounded-lg border bg-muted/50 p-4 text-sm text-muted-foreground">
                    <p className="font-semibold">Note:</p>
                    <p>
                        Privacy settings are currently stored locally and will be synced with the server
                        once the backend API is implemented. Changes you make here will affect how your
                        profile appears to other users.
                    </p>
                </div>
            </div>
        </AppLayout>
    );
}
