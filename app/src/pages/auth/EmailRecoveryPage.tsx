import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { CenteredLayout } from '@/components/layouts/CenteredLayout';
import { Logo } from '@/components/ui/logo';
import { ModeToggle } from '@/components/mode-toggle';
import { Mail, CheckCircle, AlertCircle, RefreshCw } from 'lucide-react';
import { Alert, AlertDescription } from '@/components/ui/alert';

export function EmailRecoveryPage() {
    // Get username from URL query parameter
    const urlParams = new URLSearchParams(window.location.search);
    const username = urlParams.get('username') || '';

    const [isResending, setIsResending] = useState(false);
    const [resendSuccess, setResendSuccess] = useState(false);
    const [resendError, setResendError] = useState<string>('');
    const [resendCount, setResendCount] = useState(0);
    const [countdown, setCountdown] = useState(0);

    // Countdown timer for rate limiting
    useEffect(() => {
        if (countdown > 0) {
            const timer = setTimeout(() => setCountdown(countdown - 1), 1000);
            return () => clearTimeout(timer);
        }
    }, [countdown]);

    // Mock API call to resend email
    const handleResend = async () => {
        if (countdown > 0 || resendCount >= 3) {
            return; // Rate limited
        }

        try {
            setIsResending(true);
            setResendSuccess(false);
            setResendError('');

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1000));

            // TODO: Replace with actual API call
            // await fetch('/api/v1/auth/password-reset/email', {
            //     method: 'POST',
            //     headers: { 'Content-Type': 'application/json' },
            //     body: JSON.stringify({ username }),
            // });

            setResendSuccess(true);
            setResendCount(prev => prev + 1);
            setCountdown(60); // 60 second cooldown

            // Hide success message after 5 seconds
            setTimeout(() => setResendSuccess(false), 5000);

        } catch (err) {
            setResendError('Failed to resend email. Please try again.');
            console.error('Email resend failed:', err);
        } finally {
            setIsResending(false);
        }
    };

    const canResend = countdown === 0 && resendCount < 3;

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
                            <div className="rounded-full bg-primary/10 p-4">
                                <Mail className="h-8 w-8 text-primary" />
                            </div>
                        </div>
                        <CardTitle className="text-foreground text-center">Check Your Email</CardTitle>
                        <CardDescription className="text-muted-foreground text-center">
                            We've sent a password reset link to the email address associated with <span className="font-medium">@{username}</span>
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-4">
                        <div className="bg-muted/50 rounded-lg p-4 space-y-2 text-sm">
                            <p className="text-foreground">
                                <strong>What's next?</strong>
                            </p>
                            <ul className="list-disc list-inside space-y-1 text-muted-foreground">
                                <li>Check your inbox and spam folder</li>
                                <li>Click the reset link in the email</li>
                                <li>The link will expire in 1 hour</li>
                                <li>The link can only be used once</li>
                            </ul>
                        </div>

                        {resendSuccess && (
                            <Alert className="bg-green-500/10 border-green-500/20">
                                <CheckCircle className="h-4 w-4 text-green-500" />
                                <AlertDescription className="text-green-700 dark:text-green-400">
                                    Email resent successfully! Check your inbox.
                                </AlertDescription>
                            </Alert>
                        )}

                        {resendError && (
                            <Alert>
                                <AlertCircle className="h-4 w-4" />
                                <AlertDescription>{resendError}</AlertDescription>
                            </Alert>
                        )}

                        {resendCount >= 3 && (
                            <Alert>
                                <AlertCircle className="h-4 w-4" />
                                <AlertDescription>
                                    Maximum resend attempts reached. Please try again later or contact support.
                                </AlertDescription>
                            </Alert>
                        )}

                        <div className="pt-2">
                            <Button
                                onClick={handleResend}
                                disabled={!canResend || isResending}
                                className="w-full gap-2"
                            >
                                {isResending ? (
                                    <>
                                        <RefreshCw className="h-4 w-4 animate-spin" />
                                        Sending...
                                    </>
                                ) : countdown > 0 ? (
                                    <>Resend available in {countdown}s</>
                                ) : resendCount >= 3 ? (
                                    <>Maximum attempts reached</>
                                ) : (
                                    <>
                                        <RefreshCw className="h-4 w-4" />
                                        Didn't receive the email? Resend
                                    </>
                                )}
                            </Button>
                            {resendCount > 0 && resendCount < 3 && (
                                <p className="text-xs text-center text-muted-foreground mt-2">
                                    {3 - resendCount} attempts remaining
                                </p>
                            )}
                        </div>
                    </CardContent>
                    <CardFooter className="flex flex-col space-y-2">
                        <div className="text-sm text-muted-foreground text-center">
                            Remember your password?{' '}
                            <a href="/login" className="text-primary hover:underline">
                                Sign in
                            </a>
                        </div>
                        <div className="text-sm text-muted-foreground text-center">
                            Try a different recovery method?{' '}
                            <a href="/forgot-password" className="text-primary hover:underline">
                                Go back
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
