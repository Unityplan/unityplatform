import { AppLayout } from '@/components/layouts/AppLayout';
import { Card, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb';
import { Link } from '@tanstack/react-router';
import {
    Home,
    Settings as SettingsIcon,
    Shield,
    Lock,
    Bell,
    FileText,
    ChevronRight,
    KeyRound,
    Palette
} from 'lucide-react';
import { Separator } from '@/components/ui/separator';

/**
 * Settings Page
 * 
 * Central hub for all user settings and preferences.
 * Provides navigation to various settings sections:
 * - Privacy Settings
 * - Security (TOTP, Password)
 * - Notifications
 * - Account Recovery (Friends, Manager)
 * - GDPR & Data
 * - Account Deletion
 * 
 * @example
 * Route: /settings
 */
export function SettingsPage() {
    const settingsSections = [
        {
            title: 'Appearance',
            description: 'Customize how Unity Platform looks and feels',
            items: [
                {
                    icon: Palette,
                    title: 'Theme & Display',
                    description: 'Choose your color theme and display preferences',
                    href: '/settings/appearance',
                    available: true,
                },
            ],
        },
        {
            title: 'Account',
            description: 'Manage your account settings',
            items: [
                {
                    icon: KeyRound,
                    title: 'Account Settings',
                    description: 'Email, password, and two-factor authentication',
                    href: '/settings/account',
                    available: true,
                },
            ],
        },
        {
            title: 'Privacy & Security',
            description: 'Control your privacy and security settings',
            items: [
                {
                    icon: Shield,
                    title: 'Privacy Settings',
                    description: 'Manage who can see your profile and content',
                    href: '/settings/privacy',
                    available: true,
                },
                {
                    icon: Lock,
                    title: 'Security Settings',
                    description: 'Account recovery, active sessions, and login history',
                    href: '/settings/security',
                    available: true,
                },
            ],
        },
        {
            title: 'Notifications',
            description: 'Manage your notification preferences',
            items: [
                {
                    icon: Bell,
                    title: 'Notification Preferences',
                    description: 'Email, in-app, and push notification settings',
                    href: '/settings/notifications',
                    available: true,
                },
            ],
        },
        {
            title: 'Data & Privacy',
            description: 'Manage your data and privacy rights',
            items: [
                {
                    icon: FileText,
                    title: 'Data Management',
                    description: 'Export your data or delete your account',
                    href: '/settings/data',
                    available: true,
                },
            ],
        },
    ];

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
                            <BreadcrumbPage className="flex items-center gap-1">
                                <SettingsIcon className="size-4" />
                                Settings
                            </BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                {/* Page Header */}
                <div className="mb-8">
                    <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
                    <p className="text-muted-foreground">
                        Manage your account settings and preferences
                    </p>
                </div>

                {/* Settings Sections */}
                <div className="space-y-8">
                    {settingsSections.map((section, sectionIndex) => (
                        <div key={sectionIndex}>
                            <div className="mb-4">
                                <h2 className="text-xl font-semibold">{section.title}</h2>
                                <p className="text-sm text-muted-foreground">{section.description}</p>
                            </div>

                            <div className="space-y-3">
                                {section.items.map((item, itemIndex) => (
                                    <Card key={itemIndex}>
                                        <CardHeader className="pb-3">
                                            <div className="flex items-center justify-between">
                                                <div className="flex items-center gap-3 flex-1">
                                                    <div className="p-2 rounded-lg bg-primary/10">
                                                        <item.icon className="size-5 text-primary" />
                                                    </div>
                                                    <div className="flex-1">
                                                        <CardTitle className="text-base">
                                                            {item.title}
                                                        </CardTitle>
                                                        <CardDescription className="text-sm">
                                                            {item.description}
                                                        </CardDescription>
                                                    </div>
                                                </div>
                                                <Button asChild size="icon">
                                                    <Link to={item.href}>
                                                        <ChevronRight className="size-5" />
                                                    </Link>
                                                </Button>
                                            </div>
                                        </CardHeader>
                                    </Card>
                                ))}
                            </div>

                            {sectionIndex < settingsSections.length - 1 && (
                                <Separator className="mt-8" />
                            )}
                        </div>
                    ))}
                </div>

                {/* Help Text */}
                <div className="rounded-lg border bg-muted/50 p-4 text-sm text-muted-foreground">
                    <p className="font-semibold mb-1">Need help?</p>
                    <p>
                        If you have questions about any settings, visit our Help Center or contact support.
                    </p>
                </div>
            </div>
        </AppLayout>
    );
}
