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
import { UserCog, CheckCircle, AlertCircle, Send, Clock } from 'lucide-react';
import { Alert, AlertDescription } from '@/components/ui/alert';

// Form validation schema
const managerRequestSchema = z.object({
    reason: z.string()
        .min(10, 'Please provide at least 10 characters explaining your situation')
        .max(500, 'Reason must be less than 500 characters'),
    validation_answer: z.string().optional(),
});

type ManagerRequestFormValues = z.infer<typeof managerRequestSchema>;

// Mock data for configured validation question
interface ValidationQuestion {
    question: string;
    configured: boolean;
}

export function ManagerRecoveryPage() {
    // Get username from URL query parameter
    const urlParams = new URLSearchParams(window.location.search);
    const username = urlParams.get('username') || '';

    const [isLoading, setIsLoading] = useState(false);
    const [requestSubmitted, setRequestSubmitted] = useState(false);
    const [requestId, setRequestId] = useState<string>('');
    const [error, setError] = useState<string>('');
    const [validationQuestion, setValidationQuestion] = useState<ValidationQuestion>({
        question: '',
        configured: false,
    });

    const form = useForm<ManagerRequestFormValues>({
        resolver: zodResolver(managerRequestSchema),
        defaultValues: {
            reason: '',
            validation_answer: '',
        },
    });

    // Mock: Load validation question on mount
    useState(() => {
        const loadValidationQuestion = async () => {
            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 500));

            // Mock response - user may or may not have configured a validation question
            const hasQuestion = Math.random() > 0.5;
            setValidationQuestion({
                question: hasQuestion ? 'What was the name of your first pet?' : '',
                configured: hasQuestion,
            });
        };

        loadValidationQuestion();
    });

    // Mock API call to submit recovery request
    const onSubmit = async (data: ManagerRequestFormValues) => {
        try {
            setIsLoading(true);
            setError('');

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1500));

            // TODO: Replace with actual API call
            // const response = await fetch('/api/v1/auth/password-reset/manager/request', {
            //     method: 'POST',
            //     headers: { 'Content-Type': 'application/json' },
            //     body: JSON.stringify({
            //         username,
            //         reason: data.reason,
            //         validation_answer: data.validation_answer,
            //     }),
            // });
            // const result = await response.json();

            // Mock success
            const mockRequestId = `MRR-${Date.now().toString(36).toUpperCase()}`;
            setRequestId(mockRequestId);
            setRequestSubmitted(true);

        } catch (err) {
            setError('Failed to submit recovery request. Please try again.');
            console.error('Manager recovery request failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    // Success state after submission
    if (requestSubmitted) {
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
                                <div className="rounded-full bg-green-500/10 p-4">
                                    <CheckCircle className="h-8 w-8 text-green-500" />
                                </div>
                            </div>
                            <CardTitle className="text-foreground text-center">Request Submitted</CardTitle>
                            <CardDescription className="text-muted-foreground text-center">
                                Your password recovery request has been sent to your manager
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="bg-muted/50 rounded-lg p-4 space-y-3">
                                <div className="flex items-start gap-3">
                                    <Clock className="h-5 w-5 text-muted-foreground shrink-0 mt-0.5" />
                                    <div className="space-y-1">
                                        <p className="text-sm font-medium text-foreground">Request ID</p>
                                        <p className="text-sm text-muted-foreground font-mono">{requestId}</p>
                                    </div>
                                </div>

                                <div className="border-t border-border pt-3">
                                    <p className="text-sm text-foreground font-medium mb-2">What happens next?</p>
                                    <ul className="list-disc list-inside space-y-1 text-sm text-muted-foreground">
                                        <li>Your community or territory manager will review your request</li>
                                        <li>They may contact you for additional verification</li>
                                        <li>Once approved, you'll receive instructions to reset your password</li>
                                        <li>This typically takes 1-3 business days</li>
                                    </ul>
                                </div>
                            </div>

                            <Alert className="bg-blue-500/10 border-blue-500/20">
                                <AlertCircle className="h-4 w-4 text-blue-500" />
                                <AlertDescription className="text-blue-700 dark:text-blue-400">
                                    You can check the status of your request by contacting your manager with request ID: <span className="font-mono font-semibold">{requestId}</span>
                                </AlertDescription>
                            </Alert>
                        </CardContent>
                        <CardFooter className="flex flex-col space-y-2">
                            <div className="text-sm text-muted-foreground text-center">
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

    // Request form
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
                        <div className="flex items-center gap-3 mb-2">
                            <UserCog className="h-6 w-6 text-primary" />
                            <CardTitle className="text-foreground">Request Manager Assistance</CardTitle>
                        </div>
                        <CardDescription className="text-muted-foreground">
                            Submit a password recovery request for <span className="font-medium">@{username}</span>
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <Form {...form}>
                            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                                {/* Reason Field */}
                                <FormField
                                    control={form.control}
                                    name="reason"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Reason for Recovery Request</FormLabel>
                                            <FormControl>
                                                <Textarea
                                                    placeholder="Please explain why you need to recover your account (e.g., 'I forgot my password and don't have access to my email')"
                                                    className="min-h-[100px] resize-none"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormDescription>
                                                {form.watch('reason')?.length || 0}/500 characters
                                            </FormDescription>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Optional Validation Question */}
                                {validationQuestion.configured && (
                                    <FormField
                                        control={form.control}
                                        name="validation_answer"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Security Question (Optional)</FormLabel>
                                                <FormDescription className="mb-2">
                                                    {validationQuestion.question}
                                                </FormDescription>
                                                <FormControl>
                                                    <Input
                                                        type="text"
                                                        placeholder="Your answer"
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormDescription>
                                                    Providing the correct answer will speed up approval
                                                </FormDescription>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                )}

                                {/* Info Alert */}
                                <Alert className="bg-muted/50">
                                    <AlertCircle className="h-4 w-4" />
                                    <AlertDescription>
                                        Your request will be reviewed by your community or territory manager. 
                                        They may contact you for additional verification before approving the reset.
                                    </AlertDescription>
                                </Alert>

                                {/* Error Message */}
                                {error && (
                                    <Alert>
                                        <AlertCircle className="h-4 w-4" />
                                        <AlertDescription>{error}</AlertDescription>
                                    </Alert>
                                )}

                                {/* Submit Button */}
                                <Button type="submit" className="w-full gap-2" disabled={isLoading}>
                                    {isLoading ? (
                                        <>
                                            <Send className="h-4 w-4 animate-pulse" />
                                            Submitting Request...
                                        </>
                                    ) : (
                                        <>
                                            <Send className="h-4 w-4" />
                                            Submit Recovery Request
                                        </>
                                    )}
                                </Button>
                            </form>
                        </Form>
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
