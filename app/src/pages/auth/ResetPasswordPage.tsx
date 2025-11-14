import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { CenteredLayout } from '@/components/layouts/CenteredLayout';
import { Logo } from '@/components/ui/logo';
import { ModeToggle } from '@/components/mode-toggle';
import { CheckCircle, AlertCircle, Loader2, Key } from 'lucide-react';

// Form validation schema (same as registration)
const resetPasswordSchema = z.object({
    password: z.string()
        .min(8, 'Password must be at least 8 characters')
        .regex(/[A-Z]/, 'Password must contain at least one uppercase letter')
        .regex(/[a-z]/, 'Password must contain at least one lowercase letter')
        .regex(/[0-9]/, 'Password must contain at least one number')
        .regex(/[^A-Za-z0-9]/, 'Password must contain at least one special character'),
    confirm_password: z.string().min(1, 'Please confirm your password'),
}).refine((data) => data.password === data.confirm_password, {
    message: "Passwords don't match",
    path: ['confirm_password'],
});

type ResetPasswordFormValues = z.infer<typeof resetPasswordSchema>;

export function ResetPasswordPage() {
    // Get token from URL path
    const token = window.location.pathname.split('/').pop() || '';

    const [isLoading, setIsLoading] = useState(false);
    const [isValidating, setIsValidating] = useState(true);
    const [isTokenValid, setIsTokenValid] = useState(false);
    const [tokenError, setTokenError] = useState<string>('');
    const [resetSuccess, setResetSuccess] = useState(false);
    const [resetError, setResetError] = useState<string>('');
    const [showPassword, setShowPassword] = useState(false);
    const [showConfirmPassword, setShowConfirmPassword] = useState(false);

    const form = useForm<ResetPasswordFormValues>({
        resolver: zodResolver(resetPasswordSchema),
        defaultValues: {
            password: '',
            confirm_password: '',
        },
    });

    // Mock API call to validate token on page load
    useState(() => {
        const validateToken = async () => {
            try {
                setIsValidating(true);
                // Simulate API delay
                await new Promise(resolve => setTimeout(resolve, 800));

                // Mock validation - in real app, this would call backend
                if (!token || token.length < 20) {
                    setTokenError('Invalid or missing reset token');
                    setIsTokenValid(false);
                } else if (token.includes('expired')) {
                    setTokenError('This reset link has expired. Please request a new one.');
                    setIsTokenValid(false);
                } else if (token.includes('used')) {
                    setTokenError('This reset link has already been used. Please request a new one.');
                    setIsTokenValid(false);
                } else {
                    setIsTokenValid(true);
                }
            } catch (err) {
                setTokenError('Failed to validate reset token. Please try again.');
                console.error('Token validation failed:', err);
            } finally {
                setIsValidating(false);
            }
        };

        validateToken();
    });

    // Mock API call to reset password
    const onSubmit = async (data: ResetPasswordFormValues) => {
        try {
            setIsLoading(true);
            setResetError('');

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1500));

            // In production, this would call: await resetPassword(token, data.password)
            console.log('Resetting password with:', { token, password_length: data.password.length });

            // Mock success
            setResetSuccess(true);

            // Redirect to login after 2 seconds
            setTimeout(() => {
                window.location.href = '/login';
            }, 2000);

        } catch (err) {
            setResetError('Failed to reset password. Please try again.');
            console.error('Password reset failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    // Loading state while validating token
    if (isValidating) {
        return (
            <CenteredLayout>
                <div className="fixed top-4 right-4 z-10">
                    <ModeToggle />
                </div>

                <div className="w-full max-w-md">
                    <div className="mb-8 flex justify-center">
                        <Logo className="text-foreground" />
                    </div>

                    <Card className="w-full">
                        <CardContent className="pt-6 pb-6">
                            <div className="flex flex-col items-center justify-center space-y-4 py-8">
                                <Loader2 className="h-8 w-8 animate-spin text-primary" />
                                <div className="text-sm text-muted-foreground">Validating reset link...</div>
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </CenteredLayout>
        );
    }

    // Error state - invalid token
    if (!isTokenValid) {
        return (
            <CenteredLayout>
                <div className="fixed top-4 right-4 z-10">
                    <ModeToggle />
                </div>

                <div className="w-full max-w-md">
                    <div className="mb-8 flex justify-center">
                        <Logo className="text-foreground" />
                    </div>

                    <Card className="w-full">
                        <CardHeader>
                            <div className="flex justify-center mb-4">
                                <div className="rounded-full bg-destructive/10 p-3">
                                    <AlertCircle className="h-8 w-8 text-destructive" />
                                </div>
                            </div>
                            <CardTitle className="text-foreground text-center">Invalid Reset Link</CardTitle>
                            <CardDescription className="text-muted-foreground text-center">
                                {tokenError}
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="text-sm text-muted-foreground text-center space-y-2">
                                <p>This could happen if:</p>
                                <ul className="list-disc list-inside text-left">
                                    <li>The link has expired (links are valid for 1 hour)</li>
                                    <li>The link has already been used</li>
                                    <li>The link was copied incorrectly</li>
                                </ul>
                            </div>

                            <Button
                                className="w-full"
                                onClick={() => window.location.href = '/forgot-password'}
                            >
                                Request a new reset link
                            </Button>
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

                    <div className="mt-6 text-center text-sm text-muted-foreground">
                        Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                    </div>
                </div>
            </CenteredLayout>
        );
    }

    // Success state
    if (resetSuccess) {
        return (
            <CenteredLayout>
                <div className="fixed top-4 right-4 z-10">
                    <ModeToggle />
                </div>

                <div className="w-full max-w-md">
                    <div className="mb-8 flex justify-center">
                        <Logo className="text-foreground" />
                    </div>

                    <Card className="w-full">
                        <CardHeader>
                            <div className="flex justify-center mb-4">
                                <div className="rounded-full bg-green-500/10 p-3">
                                    <CheckCircle className="h-8 w-8 text-green-500" />
                                </div>
                            </div>
                            <CardTitle className="text-foreground text-center">Password Reset Complete!</CardTitle>
                            <CardDescription className="text-muted-foreground text-center">
                                Your password has been successfully updated
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="text-sm text-muted-foreground text-center">
                                Redirecting you to the login page...
                            </div>
                            <div className="flex justify-center">
                                <Loader2 className="h-5 w-5 animate-spin text-primary" />
                            </div>
                        </CardContent>
                    </Card>

                    <div className="mt-6 text-center text-sm text-muted-foreground">
                        Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                    </div>
                </div>
            </CenteredLayout>
        );
    }

    // Main form state - valid token, ready to reset
    return (
        <CenteredLayout>
            <div className="fixed top-4 right-4 z-10">
                <ModeToggle />
            </div>

            <div className="w-full max-w-md">
                <div className="mb-8 flex justify-center">
                    <Logo className="text-foreground" />
                </div>

                <Card className="w-full">
                    <CardHeader>
                        <div className="flex justify-center mb-4">
                            <div className="rounded-full bg-primary/10 p-3">
                                <Key className="h-8 w-8 text-primary" />
                            </div>
                        </div>
                        <CardTitle className="text-foreground text-center">Create New Password</CardTitle>
                        <CardDescription className="text-muted-foreground text-center">
                            Enter a strong password for your account
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <Form {...form}>
                            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                                {/* New Password Field */}
                                <FormField
                                    control={form.control}
                                    name="password"
                                    render={({ field }) => (
                                        <FormItem>
                                            <div className="flex items-center justify-between">
                                                <FormLabel>New Password</FormLabel>
                                                <button
                                                    type="button"
                                                    onClick={() => setShowPassword(!showPassword)}
                                                    className="text-sm text-muted-foreground hover:text-primary"
                                                >
                                                    {showPassword ? 'Hide' : 'Show'}
                                                </button>
                                            </div>
                                            <FormControl>
                                                <Input
                                                    type={showPassword ? 'text' : 'password'}
                                                    placeholder="Enter your new password"
                                                    autoComplete="new-password"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormDescription>
                                                At least 8 characters with uppercase, lowercase, number, and special character
                                            </FormDescription>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Confirm Password Field */}
                                <FormField
                                    control={form.control}
                                    name="confirm_password"
                                    render={({ field }) => (
                                        <FormItem>
                                            <div className="flex items-center justify-between">
                                                <FormLabel>Confirm Password</FormLabel>
                                                <button
                                                    type="button"
                                                    onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                                                    className="text-sm text-muted-foreground hover:text-primary"
                                                >
                                                    {showConfirmPassword ? 'Hide' : 'Show'}
                                                </button>
                                            </div>
                                            <FormControl>
                                                <Input
                                                    type={showConfirmPassword ? 'text' : 'password'}
                                                    placeholder="Confirm your new password"
                                                    autoComplete="new-password"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Error Message */}
                                {resetError && (
                                    <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive flex items-start gap-2">
                                        <AlertCircle className="h-4 w-4 mt-0.5 shrink-0" />
                                        <span>{resetError}</span>
                                    </div>
                                )}

                                {/* Submit Button */}
                                <Button type="submit" className="w-full" disabled={isLoading}>
                                    {isLoading ? (
                                        <>
                                            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                                            Resetting password...
                                        </>
                                    ) : (
                                        'Reset Password'
                                    )}
                                </Button>
                            </form>
                        </Form>
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

                {/* Mock Info (Development only) */}
                <div className="mt-6 p-4 rounded-lg border border-border bg-muted/50">
                    <div className="text-xs text-muted-foreground space-y-1">
                        <div className="font-semibold mb-2">🧪 Mock Token Validation:</div>
                        <div>• Token: <code className="bg-background px-1 py-0.5 rounded">{token}</code></div>
                        <div>• URL patterns:</div>
                        <div className="ml-4">- Contains "expired" → Shows expired error</div>
                        <div className="ml-4">- Contains "used" → Shows already used error</div>
                        <div className="ml-4">- Less than 20 chars → Invalid token</div>
                        <div className="ml-4">- Otherwise → Valid (allows reset)</div>
                    </div>
                </div>

                <div className="mt-6 text-center text-sm text-muted-foreground">
                    Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                </div>
            </div>
        </CenteredLayout>
    );
}
