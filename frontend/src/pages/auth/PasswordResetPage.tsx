import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';

// Request reset schema
const requestResetSchema = z.object({
    email: z.string().email('Invalid email address'),
});

// Reset password schema
const resetPasswordSchema = z.object({
    token: z.string().min(1, 'Reset token is required'),
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

type RequestResetFormValues = z.infer<typeof requestResetSchema>;
type ResetPasswordFormValues = z.infer<typeof resetPasswordSchema>;

export function PasswordResetPage() {
    const [step, setStep] = useState<'request' | 'reset'>('request');
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string>('');
    const [success, setSuccess] = useState<string>('');
    const [showPassword, setShowPassword] = useState(false);
    const [showConfirmPassword, setShowConfirmPassword] = useState(false);

    const requestForm = useForm<RequestResetFormValues>({
        resolver: zodResolver(requestResetSchema),
        defaultValues: {
            email: '',
        },
    });

    const resetForm = useForm<ResetPasswordFormValues>({
        resolver: zodResolver(resetPasswordSchema),
        defaultValues: {
            token: '',
            password: '',
            confirm_password: '',
        },
    });

    const onRequestReset = async (data: RequestResetFormValues) => {
        try {
            setIsLoading(true);
            setError('');
            setSuccess('');

            // TODO: Implement password reset request API call
            // await authApi.requestPasswordReset(data.email);

            console.log('Password reset requested for:', data.email);

            setSuccess('If an account exists with this email, you will receive a password reset link shortly.');

            // For now, just show success and switch to reset step
            setTimeout(() => {
                setStep('reset');
            }, 3000);
        } catch (err) {
            setError('Failed to send reset email. Please try again.');
            console.error('Password reset request failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    const onResetPassword = async (data: ResetPasswordFormValues) => {
        try {
            setIsLoading(true);
            setError('');
            setSuccess('');

            // TODO: Implement password reset API call
            // await authApi.resetPassword(data.token, data.password);

            console.log('Password reset with token:', data.token);

            setSuccess('Your password has been reset successfully. You can now log in with your new password.');

            // Redirect to login after 3 seconds
            setTimeout(() => {
                window.location.href = '/login';
            }, 3000);
        } catch (err) {
            setError('Failed to reset password. The token may be invalid or expired.');
            console.error('Password reset failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    if (step === 'request') {
        return (
            <div className="flex min-h-screen items-center justify-center bg-background p-4">
                <Card className="w-full max-w-md">
                    <CardHeader>
                        <CardTitle>Reset Your Password</CardTitle>
                        <CardDescription>
                            Enter your email address and we'll send you a link to reset your password
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <Form {...requestForm}>
                            <form onSubmit={requestForm.handleSubmit(onRequestReset)} className="space-y-4">
                                {/* Email Field */}
                                <FormField
                                    control={requestForm.control}
                                    name="email"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Email</FormLabel>
                                            <FormControl>
                                                <Input
                                                    type="email"
                                                    placeholder="your.email@example.com"
                                                    autoComplete="email"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormDescription>
                                                Enter the email address associated with your account
                                            </FormDescription>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Error Message */}
                                {error && (
                                    <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive">
                                        {error}
                                    </div>
                                )}

                                {/* Success Message */}
                                {success && (
                                    <div className="rounded-md bg-green-500/15 p-3 text-sm text-green-600 dark:text-green-400">
                                        {success}
                                    </div>
                                )}

                                {/* Submit Button */}
                                <Button type="submit" className="w-full" disabled={isLoading}>
                                    {isLoading ? 'Sending...' : 'Send Reset Link'}
                                </Button>

                                {/* Alternative Action */}
                                <div className="text-center">
                                    <button
                                        type="button"
                                        onClick={() => setStep('reset')}
                                        className="text-sm text-muted-foreground hover:text-primary"
                                    >
                                        Already have a reset token?
                                    </button>
                                </div>
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
            </div>
        );
    }

    return (
        <div className="flex min-h-screen items-center justify-center bg-background p-4">
            <Card className="w-full max-w-md">
                <CardHeader>
                    <CardTitle>Set New Password</CardTitle>
                    <CardDescription>
                        Enter your reset token and choose a new password
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <Form {...resetForm}>
                        <form onSubmit={resetForm.handleSubmit(onResetPassword)} className="space-y-4">
                            {/* Reset Token Field */}
                            <FormField
                                control={resetForm.control}
                                name="token"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Reset Token</FormLabel>
                                        <FormControl>
                                            <Input
                                                type="text"
                                                placeholder="Enter the token from your email"
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormDescription>
                                            Copy the reset token from the email we sent you
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />

                            {/* New Password Field */}
                            <FormField
                                control={resetForm.control}
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
                                                placeholder="Create a strong password"
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
                                control={resetForm.control}
                                name="confirm_password"
                                render={({ field }) => (
                                    <FormItem>
                                        <div className="flex items-center justify-between">
                                            <FormLabel>Confirm New Password</FormLabel>
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
                            {error && (
                                <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive">
                                    {error}
                                </div>
                            )}

                            {/* Success Message */}
                            {success && (
                                <div className="rounded-md bg-green-500/15 p-3 text-sm text-green-600 dark:text-green-400">
                                    {success}
                                </div>
                            )}

                            {/* Submit Button */}
                            <Button type="submit" className="w-full" disabled={isLoading}>
                                {isLoading ? 'Resetting...' : 'Reset Password'}
                            </Button>

                            {/* Alternative Action */}
                            <div className="text-center">
                                <button
                                    type="button"
                                    onClick={() => setStep('request')}
                                    className="text-sm text-muted-foreground hover:text-primary"
                                >
                                    Need to request a new token?
                                </button>
                            </div>
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
        </div>
    );
}
