import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel } from '@/components/ui/form';
import { Switch } from '@/components/ui/switch';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select';
import { Separator } from '@/components/ui/separator';
import { Shield, Eye, Mail, MessageCircle, Users, User, Activity } from 'lucide-react';

// Privacy settings validation schema
const privacySettingsSchema = z.object({
    profile_visibility: z.enum(['public', 'connections', 'private']),
    show_email: z.boolean(),
    show_full_name: z.boolean(),
    allow_messages_from: z.enum(['everyone', 'connections', 'nobody']),
    show_activity_status: z.boolean(),
    show_connections: z.boolean(),
});

type PrivacySettingsFormValues = z.infer<typeof privacySettingsSchema>;

interface PrivacySettingsFormProps {
    initialValues?: Partial<PrivacySettingsFormValues>;
    onSubmit: (values: PrivacySettingsFormValues) => Promise<void>;
}

/**
 * PrivacySettingsForm component
 * 
 * Manages user privacy and visibility settings:
 * - Profile visibility (public/connections/private)
 * - Email visibility
 * - Full name visibility
 * - Message permissions
 * - Activity status visibility
 * - Connections list visibility
 * 
 * @example
 * ```tsx
 * <PrivacySettingsForm 
 *   initialValues={user.privacy_settings}
 *   onSubmit={handleUpdatePrivacy}
 *   isLoading={isUpdating}
 * />
 * ```
 */
export function PrivacySettingsForm({
    initialValues = {},
    onSubmit,
}: PrivacySettingsFormProps) {
    const form = useForm<PrivacySettingsFormValues>({
        resolver: zodResolver(privacySettingsSchema),
        defaultValues: {
            profile_visibility: initialValues.profile_visibility || 'public',
            show_email: initialValues.show_email ?? false,
            show_full_name: initialValues.show_full_name ?? true,
            allow_messages_from: initialValues.allow_messages_from || 'everyone',
            show_activity_status: initialValues.show_activity_status ?? true,
            show_connections: initialValues.show_connections ?? true,
        },
    });

    const handleSubmit = async (values: PrivacySettingsFormValues) => {
        try {
            await onSubmit(values);
        } catch (error) {
            console.error('Failed to update privacy settings:', error);
        }
    };

    return (
        <Card>
            <CardHeader>
                <div className="flex items-center gap-2">
                    <Shield className="h-5 w-5 text-primary" />
                    <CardTitle>Privacy Settings</CardTitle>
                </div>
                <CardDescription>
                    Control who can see your information and interact with you
                </CardDescription>
            </CardHeader>
            <CardContent>
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(handleSubmit)} className="space-y-6">
                        {/* Profile Visibility */}
                        <FormField
                            control={form.control}
                            name="profile_visibility"
                            render={({ field }) => (
                                <FormItem>
                                    <div className="flex items-center gap-2 mb-2">
                                        <Eye className="h-4 w-4 text-muted-foreground" />
                                        <FormLabel>Profile Visibility</FormLabel>
                                    </div>
                                    <Select onValueChange={field.onChange} defaultValue={field.value}>
                                        <FormControl>
                                            <SelectTrigger className="h-auto min-h-12 py-2">
                                                <SelectValue placeholder="Select visibility" />
                                            </SelectTrigger>
                                        </FormControl>
                                        <SelectContent>
                                            <SelectItem value="public">
                                                <div>
                                                    <div className="font-medium">Public</div>
                                                    <div className="text-sm text-muted-foreground">Anyone can view your profile</div>
                                                </div>
                                            </SelectItem>
                                            <SelectItem value="connections">
                                                <div>
                                                    <div className="font-medium">Connections Only</div>
                                                    <div className="text-sm text-muted-foreground">Only people you follow can view</div>
                                                </div>
                                            </SelectItem>
                                            <SelectItem value="private">
                                                <div>
                                                    <div className="font-medium">Private</div>
                                                    <div className="text-sm text-muted-foreground">Only you can view your profile</div>
                                                </div>
                                            </SelectItem>
                                        </SelectContent>
                                    </Select>
                                    <FormDescription>
                                        Choose who can see your profile information
                                    </FormDescription>
                                </FormItem>
                            )}
                        />

                        <Separator />

                        {/* Email Visibility */}
                        <FormField
                            control={form.control}
                            name="show_email"
                            render={({ field }) => (
                                <FormItem className="flex items-center justify-between space-y-0">
                                    <div className="space-y-1">
                                        <div className="flex items-center gap-2">
                                            <Mail className="h-4 w-4 text-muted-foreground" />
                                            <FormLabel>Show Email Address</FormLabel>
                                        </div>
                                        <FormDescription>
                                            Display your email address on your public profile
                                        </FormDescription>
                                    </div>
                                    <FormControl>
                                        <Switch
                                            checked={field.value}
                                            onCheckedChange={field.onChange}
                                        />
                                    </FormControl>
                                </FormItem>
                            )}
                        />

                        {/* Full Name Visibility */}
                        <FormField
                            control={form.control}
                            name="show_full_name"
                            render={({ field }) => (
                                <FormItem className="flex items-center justify-between space-y-0">
                                    <div className="space-y-1">
                                        <div className="flex items-center gap-2">
                                            <User className="h-4 w-4 text-muted-foreground" />
                                            <FormLabel>Show Full Name</FormLabel>
                                        </div>
                                        <FormDescription>
                                            Display your full name instead of just username
                                        </FormDescription>
                                    </div>
                                    <FormControl>
                                        <Switch
                                            checked={field.value}
                                            onCheckedChange={field.onChange}
                                        />
                                    </FormControl>
                                </FormItem>
                            )}
                        />

                        <Separator />

                        {/* Message Permissions */}
                        <FormField
                            control={form.control}
                            name="allow_messages_from"
                            render={({ field }) => (
                                <FormItem>
                                    <div className="flex items-center gap-2 mb-2">
                                        <MessageCircle className="h-4 w-4 text-muted-foreground" />
                                        <FormLabel>Who Can Message You</FormLabel>
                                    </div>
                                    <Select onValueChange={field.onChange} defaultValue={field.value}>
                                        <FormControl>
                                            <SelectTrigger>
                                                <SelectValue placeholder="Select messaging permissions" />
                                            </SelectTrigger>
                                        </FormControl>
                                        <SelectContent>
                                            <SelectItem value="everyone">Everyone</SelectItem>
                                            <SelectItem value="connections">Connections Only</SelectItem>
                                            <SelectItem value="nobody">Nobody</SelectItem>
                                        </SelectContent>
                                    </Select>
                                    <FormDescription>
                                        Control who can send you direct messages
                                    </FormDescription>
                                </FormItem>
                            )}
                        />

                        <Separator />

                        {/* Activity Status */}
                        <FormField
                            control={form.control}
                            name="show_activity_status"
                            render={({ field }) => (
                                <FormItem className="flex items-center justify-between space-y-0">
                                    <div className="space-y-1">
                                        <div className="flex items-center gap-2">
                                            <Activity className="h-4 w-4 text-muted-foreground" />
                                            <FormLabel>Show Activity Status</FormLabel>
                                        </div>
                                        <FormDescription>
                                            Let others see when you're online or active
                                        </FormDescription>
                                    </div>
                                    <FormControl>
                                        <Switch
                                            checked={field.value}
                                            onCheckedChange={field.onChange}
                                        />
                                    </FormControl>
                                </FormItem>
                            )}
                        />

                        {/* Connections Visibility */}
                        <FormField
                            control={form.control}
                            name="show_connections"
                            render={({ field }) => (
                                <FormItem className="flex items-center justify-between space-y-0">
                                    <div className="space-y-1">
                                        <div className="flex items-center gap-2">
                                            <Users className="h-4 w-4 text-muted-foreground" />
                                            <FormLabel>Show Connections List</FormLabel>
                                        </div>
                                        <FormDescription>
                                            Display your followers and following lists publicly
                                        </FormDescription>
                                    </div>
                                    <FormControl>
                                        <Switch
                                            checked={field.value}
                                            onCheckedChange={field.onChange}
                                        />
                                    </FormControl>
                                </FormItem>
                            )}
                        />
                    </form>
                </Form>
            </CardContent>
        </Card>
    );
}
