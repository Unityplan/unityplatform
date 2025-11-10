import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { CenteredLayout } from '@/components/layouts/CenteredLayout';
import { Logo } from '@/components/ui/logo';
import { ModeToggle } from '@/components/mode-toggle';
import { UserCog, CheckCircle, AlertCircle, Info, ArrowLeft } from 'lucide-react';

// Form validation schema
const managerRecoverySchema = z.object({
    username: z.string().min(1, 'Username is required'),
    reason: z.string().min(10, 'Please provide at least 10 characters explaining your situation').max(500, 'Reason must be less than 500 characters'),
    validation_answer: z.string().optional(),
});

type ManagerRecoveryFormValues = z.infer<typeof managerRecoverySchema>;

// Mock response for user's configured security question
interface UserSecurityQuestion {
    has_security_question: boolean;
    question?: string;
}

export function ForgotPasswordManagerPage() {
    // Get username from URL params if passed from previous page
    const urlParams = new URLSearchParams(window.location.search);
    const initialUsername = urlParams.get('username') || '';
    
    const [isLoading, setIsLoading] = useState(false);
    const [isCheckingUser, setIsCheckingUser] = useState(false);
    const [error, setError] = useState<string>('');
    const [requestSuccess, setRequestSuccess] = useState(false);
    const [requestId, setRequestId] = useState<string>('');
    const [securityQuestion, setSecurityQuestion] = useState<UserSecurityQuestion | null>(null);
    const [usernameChecked, setUsernameChecked] = useState(false);

    const form = useForm<ManagerRecoveryFormValues>({
        resolver: zodResolver(managerRecoverySchema),
        defaultValues: {
            username: initialUsername,
            reason: '',
            validation_answer: '',
        },
    });

    // Mock API call to check if user has security question configured
    const checkSecurityQuestion = async (username: string) => {
        try {
            setIsCheckingUser(true);
            setError('');

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 600));

            // Mock response - in production, this would query backend
            if (username.includes('secure')) {
                setSecurityQuestion({
                    has_security_question: true,
                    question: "What city were you born in?",
                });
            } else {
                setSecurityQuestion({
                    has_security_question: false,
                });
            }
            
            setUsernameChecked(true);
        } catch (err) {
            setError('Failed to check user information. Please try again.');
            console.error('User check failed:', err);
        } finally {
            setIsCheckingUser(false);
        }
    };

    // Auto-check security question when username is provided from previous page
    useState(() => {
        if (initialUsername && !usernameChecked) {
            checkSecurityQuestion(initialUsername);
        }
    });

    const onSubmit = async (data: ManagerRecoveryFormValues) => {
        // Check security question first if not already checked
        if (!usernameChecked) {
            await checkSecurityQuestion(data.username);
            return;
        }

        try {
            setIsLoading(true);
            setError('');

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1200));

            // Mock success - generate request ID
            const mockRequestId = `MRR-${Math.random().toString(36).substr(2, 12).toUpperCase()}`;
            setRequestId(mockRequestId);
            setRequestSuccess(true);

        } catch (err) {
            setError('Failed to submit recovery request. Please try again.');
            console.error('Manager recovery request failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    // Success state
    if (requestSuccess) {
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
                            <CardTitle className="text-foreground text-center">Request Submitted</CardTitle>
                            <CardDescription className="text-muted-foreground text-center">
                                Your password reset request has been sent to your manager
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-6">
                            {/* Request ID */}
                            <div className="rounded-lg border border-border bg-muted/50 p-4">
                                <div className="text-xs text-muted-foreground mb-1">Request ID</div>
                                <div className="font-mono text-sm text-foreground">{requestId}</div>
                            </div>

                            {/* Next Steps */}
                            <div className="space-y-3">
                                <div className="text-sm font-medium text-foreground">What happens next:</div>
                                <ol className="list-decimal list-inside space-y-2 text-sm text-muted-foreground">
                                    <li>Your community or territory manager will review your request</li>
                                    <li>They may contact you for additional verification</li>
                                    {securityQuestion?.has_security_question && (
                                        <li>If your answer matches, they can approve immediately</li>
                                    )}
                                    {!securityQuestion?.has_security_question && (
                                        <li>They can reset your password or send you a reset link</li>
                                    )}
                                    <li>You'll receive a notification when your request is processed</li>
                                </ol>
                            </div>

                            {/* Info Box */}
                            <div className="rounded-lg border border-blue-500/30 bg-blue-500/10 p-4 flex items-start gap-3">
                                <Info className="h-5 w-5 text-blue-600 dark:text-blue-400 mt-0.5 shrink-0" />
                                <div className="text-sm text-blue-800 dark:text-blue-300">
                                    <div className="font-medium mb-1">Average response time: 24-48 hours</div>
                                    <div className="text-xs">
                                        Managers are volunteers from your community. Please be patient while they review your request.
                                    </div>
                                </div>
                            </div>

                            {/* Return to Login */}
                            <Button
                                className="w-full"
                                onClick={() => window.location.href = '/login'}
                            >
                                Return to Login
                            </Button>
                        </CardContent>
                    </Card>

                    <div className="mt-6 text-center text-sm text-muted-foreground">
                        Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                    </div>
                </div>
            </CenteredLayout>
        );
    }

    // Main form
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
                                <UserCog className="h-8 w-8 text-primary" />
                            </div>
                        </div>
                        <CardTitle className="text-foreground text-center">Manager-Assisted Recovery</CardTitle>
                        <CardDescription className="text-muted-foreground text-center">
                            Request help from your community or territory manager
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
                                                    disabled={usernameChecked}
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Security Question (if user has one configured) */}
                                {usernameChecked && securityQuestion?.has_security_question && (
                                    <div className="rounded-lg border border-border bg-muted/50 p-4 space-y-3">
                                        <div className="flex items-start gap-2">
                                            <Info className="h-4 w-4 text-primary mt-0.5 shrink-0" />
                                            <div className="text-sm text-muted-foreground">
                                                You configured a security question. Answering it correctly will speed up the approval process.
                                            </div>
                                        </div>

                                        <FormField
                                            control={form.control}
                                            name="validation_answer"
                                            render={({ field }) => (
                                                <FormItem>
                                                    <FormLabel>{securityQuestion.question}</FormLabel>
                                                    <FormControl>
                                                        <Input
                                                            type="text"
                                                            placeholder="Your answer (optional)"
                                                            {...field}
                                                        />
                                                    </FormControl>
                                                    <FormDescription className="text-xs">
                                                        Optional, but recommended for faster approval
                                                    </FormDescription>
                                                    <FormMessage />
                                                </FormItem>
                                            )}
                                        />
                                    </div>
                                )}

                                {/* Reason Field */}
                                <FormField
                                    control={form.control}
                                    name="reason"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Reason for Request</FormLabel>
                                            <FormControl>
                                                <Textarea
                                                    placeholder="Explain why you need a password reset (e.g., 'I forgot my password and don't have email configured')"
                                                    className="min-h-[100px] resize-none"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormDescription>
                                                {field.value?.length || 0}/500 characters
                                            </FormDescription>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Info Box */}
                                {usernameChecked && !securityQuestion?.has_security_question && (
                                    <div className="rounded-lg border border-orange-500/30 bg-orange-500/10 p-4 flex items-start gap-3">
                                        <Info className="h-5 w-5 text-orange-600 dark:text-orange-400 mt-0.5 shrink-0" />
                                        <div className="text-sm text-orange-800 dark:text-orange-300">
                                            You haven't configured a security question. Your manager will need to verify your identity through other means, which may take longer.
                                        </div>
                                    </div>
                                )}

                                {/* Error Message */}
                                {error && (
                                    <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive flex items-start gap-2">
                                        <AlertCircle className="h-4 w-4 mt-0.5 shrink-0" />
                                        <span>{error}</span>
                                    </div>
                                )}

                                {/* Submit Button */}
                                <Button 
                                    type="submit" 
                                    className="w-full" 
                                    disabled={isLoading || isCheckingUser}
                                >
                                    {isCheckingUser ? 'Checking user...' :
                                     isLoading ? 'Submitting request...' :
                                     !usernameChecked ? 'Continue' :
                                     'Submit Recovery Request'}
                                </Button>

                                {/* Back Button */}
                                {usernameChecked && (
                                    <Button
                                        type="button"
                                        className="w-full gap-2"
                                        onClick={() => {
                                            setUsernameChecked(false);
                                            setSecurityQuestion(null);
                                            form.setValue('username', '');
                                        }}
                                    >
                                        <ArrowLeft className="h-4 w-4" />
                                        Change Username
                                    </Button>
                                )}
                            </form>
                        </Form>

                        {/* Alternative Methods */}
                        {!usernameChecked && (
                            <div className="mt-6 pt-6 border-t border-border">
                                <Button
                                    className="w-full gap-2"
                                    onClick={() => window.location.href = '/forgot-password'}
                                >
                                    <ArrowLeft className="h-4 w-4" />
                                    Try a different recovery method
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

                {/* Mock Info (Development only) */}
                <div className="mt-6 p-4 rounded-lg border border-border bg-muted/50">
                    <div className="text-xs text-muted-foreground space-y-1">
                        <div className="font-semibold mb-2">🧪 Mock Manager Recovery:</div>
                        <div>• Username containing "secure" → Has security question configured</div>
                        <div>• Other usernames → No security question</div>
                        <div>• Request creates mock request ID for tracking</div>
                        <div>• In production, manager receives notification and reviews request</div>
                    </div>
                </div>

                <div className="mt-6 text-center text-sm text-muted-foreground">
                    Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                </div>
            </div>
        </CenteredLayout>
    );
}
