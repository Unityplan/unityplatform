import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { CenteredLayout } from '@/components/layouts/CenteredLayout';
import { Logo } from '@/components/ui/logo';
import { ModeToggle } from '@/components/mode-toggle';
import { useNavigate } from '@tanstack/react-router';
import { Mail, Users, UserCog, AlertCircle } from 'lucide-react';

// Form validation schema
const usernameSchema = z.object({
    username: z.string().min(1, 'Username is required'),
});

type UsernameFormValues = z.infer<typeof usernameSchema>;

// Mock response for available recovery methods
interface RecoveryMethodsResponse {
    available_methods: ('email' | 'friend' | 'manager')[];
    recovery_friends?: string[]; // Usernames of recovery friends
    has_email: boolean;
    has_recovery_friends: boolean;
    has_manager: boolean;
}

export function ForgotPasswordPage() {
    const navigate = useNavigate();
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string>('');
    const [recoveryMethods, setRecoveryMethods] = useState<RecoveryMethodsResponse | null>(null);

    const form = useForm<UsernameFormValues>({
        resolver: zodResolver(usernameSchema),
        defaultValues: {
            username: '',
        },
    });

    // Mock API call to detect available recovery methods
    const checkRecoveryMethods = async (username: string): Promise<RecoveryMethodsResponse> => {
        // Simulate API delay
        await new Promise(resolve => setTimeout(resolve, 800));

        // Mock response based on username (for testing different scenarios)
        if (username === 'test_email') {
            return {
                available_methods: ['email'],
                has_email: true,
                has_recovery_friends: false,
                has_manager: false,
            };
        } else if (username === 'test_friend') {
            return {
                available_methods: ['friend'],
                recovery_friends: ['alice', 'bob', 'charlie'],
                has_email: false,
                has_recovery_friends: true,
                has_manager: false,
            };
        } else if (username === 'test_all') {
            return {
                available_methods: ['email', 'friend', 'manager'],
                recovery_friends: ['alice', 'bob'],
                has_email: true,
                has_recovery_friends: true,
                has_manager: true,
            };
        } else if (username === 'test_manager') {
            return {
                available_methods: ['manager'],
                has_email: false,
                has_recovery_friends: false,
                has_manager: true,
            };
        }

        // Default: return all methods available
        return {
            available_methods: ['email', 'friend', 'manager'],
            recovery_friends: ['friend1', 'friend2', 'friend3'],
            has_email: true,
            has_recovery_friends: true,
            has_manager: true,
        };
    };

    const onSubmit = async (data: UsernameFormValues) => {
        try {
            setIsLoading(true);
            setError('');

            // Mock API call to check available recovery methods
            const methods = await checkRecoveryMethods(data.username);
            setRecoveryMethods(methods);

            // If only one method available, auto-navigate
            if (methods.available_methods.length === 1) {
                const method = methods.available_methods[0];
                if (method === 'email') {
                    navigate({ 
                        to: '/forgot-password/email',
                        search: { username: data.username }
                    });
                } else if (method === 'friend') {
                    navigate({ 
                        to: '/forgot-password/friend',
                        search: { username: data.username }
                    });
                } else if (method === 'manager') {
                    navigate({ 
                        to: '/forgot-password/manager',
                        search: { username: data.username }
                    });
                }
            }
        } catch (err) {
            setError('Failed to check recovery methods. Please try again.');
            console.error('Recovery method check failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    const handleMethodSelection = (method: 'email' | 'friend' | 'manager') => {
        const username = form.getValues('username');
        
        if (method === 'email') {
            navigate({ 
                to: '/forgot-password/email',
                search: { username }
            });
        } else if (method === 'friend') {
            navigate({ 
                to: '/forgot-password/friend',
                search: { username }
            });
        } else if (method === 'manager') {
            navigate({ 
                to: '/forgot-password/manager',
                search: { username }
            });
        }
    };

    return (
        <CenteredLayout>
            {/* Dark Mode Toggle */}
            <div className="fixed top-4 right-4 z-10">
                <ModeToggle />
            </div>

            <div className="w-full max-w-md">
                <div className="mb-8 flex justify-center">
                    <Logo className="text-foreground" />
                </div>

                <Card className="w-full">
                    <CardHeader>
                        <CardTitle className="text-foreground">Reset Your Password</CardTitle>
                        <CardDescription className="text-muted-foreground">
                            Enter your username to see available recovery options
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <Form {...form}>
                            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                                {/* Username Field */}
                                <FormField
                                    control={form.control}
                                    name="username"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Username</FormLabel>
                                            <FormControl>
                                                <Input
                                                    type="text"
                                                    placeholder="Enter your username"
                                                    autoComplete="username"
                                                    disabled={!!recoveryMethods}
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Error Message */}
                                {error && (
                                    <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive flex items-start gap-2">
                                        <AlertCircle className="h-4 w-4 mt-0.5 flex-shrink-0" />
                                        <span>{error}</span>
                                    </div>
                                )}

                                {/* Submit Button (only show if no methods detected yet) */}
                                {!recoveryMethods && (
                                    <Button type="submit" className="w-full" disabled={isLoading}>
                                        {isLoading ? 'Checking...' : 'Continue'}
                                    </Button>
                                )}
                            </form>
                        </Form>

                        {/* Recovery Method Selection (show after username checked) */}
                        {recoveryMethods && recoveryMethods.available_methods.length > 1 && (
                            <div className="mt-6 space-y-3">
                                <div className="text-sm font-medium text-foreground mb-4">
                                    Choose a recovery method:
                                </div>

                                {/* Email Recovery Option */}
                                {recoveryMethods.has_email && (
                                    <Button
                                        variant="outline"
                                        className="w-full justify-start h-auto py-4 px-4"
                                        onClick={() => handleMethodSelection('email')}
                                    >
                                        <div className="flex items-start gap-3 text-left">
                                            <Mail className="h-5 w-5 mt-0.5 text-primary" />
                                            <div className="flex-1">
                                                <div className="font-medium text-foreground">Email Recovery</div>
                                                <div className="text-sm text-muted-foreground">
                                                    We'll send a reset link to your registered email address
                                                </div>
                                            </div>
                                        </div>
                                    </Button>
                                )}

                                {/* Friend Recovery Option */}
                                {recoveryMethods.has_recovery_friends && (
                                    <Button
                                        variant="outline"
                                        className="w-full justify-start h-auto py-4 px-4"
                                        onClick={() => handleMethodSelection('friend')}
                                    >
                                        <div className="flex items-start gap-3 text-left">
                                            <Users className="h-5 w-5 mt-0.5 text-primary" />
                                            <div className="flex-1">
                                                <div className="font-medium text-foreground">Friend-Based Recovery</div>
                                                <div className="text-sm text-muted-foreground">
                                                    Get help from one of your {recoveryMethods.recovery_friends?.length} trusted friends
                                                </div>
                                            </div>
                                        </div>
                                    </Button>
                                )}

                                {/* Manager Recovery Option */}
                                {recoveryMethods.has_manager && (
                                    <Button
                                        variant="outline"
                                        className="w-full justify-start h-auto py-4 px-4"
                                        onClick={() => handleMethodSelection('manager')}
                                    >
                                        <div className="flex items-start gap-3 text-left">
                                            <UserCog className="h-5 w-5 mt-0.5 text-primary" />
                                            <div className="flex-1">
                                                <div className="font-medium text-foreground">Manager Assistance</div>
                                                <div className="text-sm text-muted-foreground">
                                                    Request help from your community or territory manager
                                                </div>
                                            </div>
                                        </div>
                                    </Button>
                                )}

                                {/* Start Over Button */}
                                <Button
                                    variant="ghost"
                                    className="w-full"
                                    onClick={() => {
                                        setRecoveryMethods(null);
                                        form.reset();
                                    }}
                                >
                                    Start Over
                                </Button>
                            </div>
                        )}
                    </CardContent>
                    <CardFooter className="flex flex-col space-y-2">
                        <div className="text-sm text-muted-foreground">
                            Remember your password?{' '}
                            <a href="/login" className="text-primary hover:underline">
                                Sign in
                            </a>
                        </div>
                    </CardFooter>
                </Card>

                {/* Mock Info (Development only - remove in production) */}
                <div className="mt-6 p-4 rounded-lg border border-border bg-muted/50">
                    <div className="text-xs text-muted-foreground space-y-1">
                        <div className="font-semibold mb-2">🧪 Mock Testing Usernames:</div>
                        <div>• <code className="bg-background px-1 py-0.5 rounded">test_email</code> - Email recovery only</div>
                        <div>• <code className="bg-background px-1 py-0.5 rounded">test_friend</code> - Friend recovery only</div>
                        <div>• <code className="bg-background px-1 py-0.5 rounded">test_manager</code> - Manager recovery only</div>
                        <div>• <code className="bg-background px-1 py-0.5 rounded">test_all</code> - All methods available</div>
                        <div>• Any other username - All methods (default)</div>
                    </div>
                </div>

                {/* Platform branding */}
                <div className="mt-6 text-center text-sm text-muted-foreground">
                    Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                </div>
            </div>
        </CenteredLayout>
    );
}
