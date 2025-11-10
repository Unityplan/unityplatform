import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { CenteredLayout } from '@/components/layouts/CenteredLayout';
import { Logo } from '@/components/ui/logo';
import { ModeToggle } from '@/components/mode-toggle';
import { Mail, CheckCircle, AlertCircle, ArrowLeft } from 'lucide-react';

export function ForgotPasswordEmailPage() {
    // TODO: Use TanStack Router once routes are configured
    // For now, get username from URL params manually
    const urlParams = new URLSearchParams(window.location.search);
    const username = urlParams.get('username') || '';
    
    const [isResending, setIsResending] = useState(false);
    const [resendSuccess, setResendSuccess] = useState(false);
    const [resendError, setResendError] = useState<string>('');
    const [resendCount, setResendCount] = useState(0);
    const [canResend, setCanResend] = useState(true);

    // Mock API call to resend email
    const handleResend = async () => {
        if (!canResend || resendCount >= 3) {
            setResendError('You have reached the maximum number of resend attempts. Please try again later.');
            return;
        }

        try {
            setIsResending(true);
            setResendError('');
            setResendSuccess(false);

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1000));

            // Mock success
            setResendSuccess(true);
            setResendCount(prev => prev + 1);

            // Temporarily disable resend button
            setCanResend(false);
            setTimeout(() => {
                setCanResend(true);
                setResendSuccess(false);
            }, 60000); // 1 minute cooldown

        } catch (err) {
            setResendError('Failed to resend email. Please try again.');
            console.error('Resend failed:', err);
        } finally {
            setIsResending(false);
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
                        <div className="flex justify-center mb-4">
                            <div className="rounded-full bg-primary/10 p-3">
                                <Mail className="h-8 w-8 text-primary" />
                            </div>
                        </div>
                        <CardTitle className="text-foreground text-center">Check Your Email</CardTitle>
                        <CardDescription className="text-muted-foreground text-center">
                            We've sent a password reset link to your email address
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-6">
                        {/* Instructions */}
                        <div className="space-y-3 text-sm text-muted-foreground">
                            <p className="text-center">
                                If an account exists for <span className="font-medium text-foreground">{username || 'this username'}</span>, 
                                you will receive an email with instructions to reset your password.
                            </p>
                            
                            <div className="rounded-lg border border-border bg-muted/50 p-4 space-y-2">
                                <div className="font-medium text-foreground text-sm">What to do next:</div>
                                <ol className="list-decimal list-inside space-y-1 text-sm">
                                    <li>Check your email inbox</li>
                                    <li>Look for an email from Unity Platform</li>
                                    <li>Click the reset link in the email</li>
                                    <li>Create a new password</li>
                                </ol>
                            </div>

                            <p className="text-xs text-center">
                                The reset link will expire in <span className="font-medium text-foreground">1 hour</span> for security reasons.
                            </p>
                        </div>

                        {/* Success Message */}
                        {resendSuccess && (
                            <div className="rounded-md bg-green-500/15 border border-green-500/30 p-3 text-sm text-green-700 dark:text-green-400 flex items-start gap-2">
                                <CheckCircle className="h-4 w-4 mt-0.5 shrink-0" />
                                <span>Email resent successfully! Please check your inbox.</span>
                            </div>
                        )}

                        {/* Error Message */}
                        {resendError && (
                            <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive flex items-start gap-2">
                                <AlertCircle className="h-4 w-4 mt-0.5 shrink-0" />
                                <span>{resendError}</span>
                            </div>
                        )}

                        {/* Resend Button */}
                        <div className="space-y-2">
                            <Button
                                className="w-full"
                                onClick={handleResend}
                                disabled={isResending || !canResend || resendCount >= 3}
                            >
                                {isResending ? 'Sending...' : 
                                 !canResend ? 'Wait 1 minute to resend' : 
                                 resendCount >= 3 ? 'Maximum resends reached' :
                                 'Resend Email'}
                            </Button>
                            
                            {resendCount > 0 && resendCount < 3 && (
                                <div className="text-xs text-center text-muted-foreground">
                                    Resent {resendCount}/3 times
                                </div>
                            )}
                        </div>

                        {/* Troubleshooting Tips */}
                        <div className="rounded-lg border border-border bg-muted/50 p-4">
                            <div className="text-xs text-muted-foreground space-y-2">
                                <div className="font-medium text-foreground">Didn't receive the email?</div>
                                <ul className="list-disc list-inside space-y-1">
                                    <li>Check your spam or junk folder</li>
                                    <li>Make sure you entered the correct username</li>
                                    <li>Wait a few minutes - it can take time to arrive</li>
                                    <li>Contact your territory administrator if problems persist</li>
                                </ul>
                            </div>
                        </div>

                        {/* Back Button */}
                        <Button
                            className="w-full gap-2"
                            onClick={() => window.location.href = '/forgot-password'}
                        >
                            <ArrowLeft className="h-4 w-4" />
                            Try a different method
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

                {/* Mock Info (Development only) */}
                <div className="mt-6 p-4 rounded-lg border border-border bg-muted/50">
                    <div className="text-xs text-muted-foreground space-y-1">
                        <div className="font-semibold mb-2">🧪 Mock Email Recovery:</div>
                        <div>• Email service is not yet implemented in backend</div>
                        <div>• In production, a real email would be sent to the user's address</div>
                        <div>• Reset link format: <code className="bg-background px-1 py-0.5 rounded">/reset-password/[token]</code></div>
                        <div>• Rate limit: 3 emails per hour (mocked)</div>
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
