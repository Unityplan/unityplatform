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
import { Users, CheckCircle, AlertCircle, Key, Shield, ArrowRight, ArrowLeft } from 'lucide-react';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';

// Step 1: Select friend and validate username
const friendSelectionSchema = z.object({
    selected_friend: z.string().min(1, 'Please select a recovery friend'),
    friend_username_confirmation: z.string().min(1, 'Please enter the friend\'s username'),
});

// Step 2: Enter friend's token
const tokenEntrySchema = z.object({
    friend_token: z.string()
        .min(1, 'Recovery token is required')
        .regex(/^FRT-[A-Z0-9]{4}-[A-Z0-9]{4}-[A-Z0-9]{4}$/i, 'Invalid token format (FRT-XXXX-XXXX-XXXX)'),
});

// Step 3: Answer personal validation question
const validationAnswerSchema = z.object({
    validation_answer: z.string().min(1, 'Please answer your security question'),
});

// Step 4: Reset password
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

type FriendSelectionFormValues = z.infer<typeof friendSelectionSchema>;
type TokenEntryFormValues = z.infer<typeof tokenEntrySchema>;
type ValidationAnswerFormValues = z.infer<typeof validationAnswerSchema>;
type ResetPasswordFormValues = z.infer<typeof resetPasswordSchema>;

type RecoveryStep = 'select_friend' | 'enter_token' | 'answer_question' | 'reset_password' | 'success';

export function FriendRecoveryPage() {
    // Get username and recovery friends from URL query parameters
    const urlParams = new URLSearchParams(window.location.search);
    const username = urlParams.get('username') || '';
    // Mock: In real app, this would come from API response
    const recoveryFriends = ['alice', 'bob', 'charlie', 'dana', 'eve'];

    const [currentStep, setCurrentStep] = useState<RecoveryStep>('select_friend');
    const [selectedFriend, setSelectedFriend] = useState<string>('');
    const [friendToken, setFriendToken] = useState<string>('');
    const [validationQuestion, setValidationQuestion] = useState<string>('');
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string>('');
    const [showPassword, setShowPassword] = useState(false);
    const [showConfirmPassword, setShowConfirmPassword] = useState(false);

    const friendSelectionForm = useForm<FriendSelectionFormValues>({
        resolver: zodResolver(friendSelectionSchema),
        defaultValues: {
            selected_friend: '',
            friend_username_confirmation: '',
        },
    });

    const tokenEntryForm = useForm<TokenEntryFormValues>({
        resolver: zodResolver(tokenEntrySchema),
        defaultValues: {
            friend_token: '',
        },
    });

    const validationAnswerForm = useForm<ValidationAnswerFormValues>({
        resolver: zodResolver(validationAnswerSchema),
        defaultValues: {
            validation_answer: '',
        },
    });

    const resetPasswordForm = useForm<ResetPasswordFormValues>({
        resolver: zodResolver(resetPasswordSchema),
        defaultValues: {
            password: '',
            confirm_password: '',
        },
    });

    // Step 1: Select friend and validate username
    const onSubmitFriendSelection = async (data: FriendSelectionFormValues) => {
        try {
            setIsLoading(true);
            setError('');

            // Validate that selected friend matches confirmation
            if (data.selected_friend !== data.friend_username_confirmation) {
                setError('The username you entered does not match the selected friend. Please try again.');
                setIsLoading(false);
                return;
            }

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1000));

            // TODO: Replace with actual API call
            // await fetch('/api/v1/auth/password-reset/friend/initiate', {
            //     method: 'POST',
            //     headers: { 'Content-Type': 'application/json' },
            //     body: JSON.stringify({
            //         username,
            //         friend_username: data.selected_friend,
            //     }),
            // });

            setSelectedFriend(data.selected_friend);
            setCurrentStep('enter_token');

        } catch (err) {
            setError('Failed to initiate friend recovery. Please try again.');
            console.error('Friend selection failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    // Step 2: Enter friend's token
    const onSubmitToken = async (data: TokenEntryFormValues) => {
        try {
            setIsLoading(true);
            setError('');

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 800));

            // TODO: Replace with actual API call to validate token
            // const response = await fetch('/api/v1/auth/password-reset/friend/validate-token', {
            //     method: 'POST',
            //     headers: { 'Content-Type': 'application/json' },
            //     body: JSON.stringify({
            //         username,
            //         friend_token: data.friend_token,
            //     }),
            // });

            // Mock validation
            if (!data.friend_token.toUpperCase().startsWith('FRT-')) {
                setError('Invalid token format');
                setIsLoading(false);
                return;
            }

            // Mock: Load user's personal validation question
            setValidationQuestion('What was the name of your first pet?');
            setFriendToken(data.friend_token);
            setCurrentStep('answer_question');

        } catch (err) {
            setError('Failed to validate token. Please check the token and try again.');
            console.error('Token validation failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    // Step 3: Answer personal validation question
    const onSubmitValidationAnswer = async (data: ValidationAnswerFormValues) => {
        try {
            setIsLoading(true);
            setError('');

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1000));

            // TODO: Replace with actual API call
            // const response = await fetch('/api/v1/auth/password-reset/friend/validate', {
            //     method: 'POST',
            //     headers: { 'Content-Type': 'application/json' },
            //     body: JSON.stringify({
            //         username,
            //         friend_token: friendToken,
            //         validation_answer: data.validation_answer,
            //     }),
            // });

            // Mock: Check answer (in real app, backend validates hash)
            if (data.validation_answer.toLowerCase().length < 2) {
                setError('Incorrect answer. Please try again.');
                setIsLoading(false);
                return;
            }

            setCurrentStep('reset_password');

        } catch (err) {
            setError('Validation failed. Please check your answer and try again.');
            console.error('Validation answer failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    // Step 4: Reset password
    const onSubmitResetPassword = async (data: ResetPasswordFormValues) => {
        try {
            setIsLoading(true);
            setError('');

            // Simulate API delay
            await new Promise(resolve => setTimeout(resolve, 1500));

            // TODO: Replace with actual API call
            // await fetch('/api/v1/auth/password-reset/friend/complete', {
            //     method: 'POST',
            //     headers: { 'Content-Type': 'application/json' },
            //     body: JSON.stringify({
            //         username,
            //         friend_token: friendToken,
            //         new_password: data.password,
            //     }),
            // });

            setCurrentStep('success');

            // Redirect to login after 3 seconds
            setTimeout(() => {
                window.location.href = '/login';
            }, 3000);

        } catch (err) {
            setError('Failed to reset password. Please try again.');
            console.error('Password reset failed:', err);
        } finally {
            setIsLoading(false);
        }
    };

    // Success state
    if (currentStep === 'success') {
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
                            <CardTitle className="text-foreground text-center">Password Reset Successful!</CardTitle>
                            <CardDescription className="text-muted-foreground text-center">
                                Your password for <span className="font-medium">@{username}</span> has been successfully reset
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <Alert className="bg-green-500/10 border-green-500/20">
                                <CheckCircle className="h-4 w-4 text-green-500" />
                                <AlertDescription className="text-green-700 dark:text-green-400">
                                    Redirecting to login page in a few seconds...
                                </AlertDescription>
                            </Alert>
                        </CardContent>
                        <CardFooter>
                            <Button onClick={() => window.location.href = '/login'} className="w-full">
                                Go to Login
                            </Button>
                        </CardFooter>
                    </Card>

                    <div className="mt-6 text-center text-sm text-muted-foreground">
                        Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                    </div>
                </div>
            </CenteredLayout>
        );
    }

    // Multi-step form layout
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
                            <Users className="h-6 w-6 text-primary" />
                            <CardTitle className="text-foreground">Friend-Based Recovery</CardTitle>
                        </div>
                        <CardDescription className="text-muted-foreground">
                            Recover your account with help from a trusted friend
                        </CardDescription>

                        {/* Progress indicators */}
                        <div className="flex items-center justify-between mt-6 px-2">
                            <div className={`flex flex-col items-center gap-2 ${currentStep === 'select_friend' ? 'text-primary' : currentStep === 'enter_token' || currentStep === 'answer_question' || currentStep === 'reset_password' ? 'text-green-500' : 'text-muted-foreground'}`}>
                                <div className={`rounded-full p-2 ${currentStep === 'select_friend' ? 'bg-primary/10' : currentStep === 'enter_token' || currentStep === 'answer_question' || currentStep === 'reset_password' ? 'bg-green-500/10' : 'bg-muted'}`}>
                                    <Users className="h-4 w-4" />
                                </div>
                                <span className="text-xs font-medium">Friend</span>
                            </div>
                            <div className="h-[2px] flex-1 mx-2 bg-border" />
                            <div className={`flex flex-col items-center gap-2 ${currentStep === 'enter_token' ? 'text-primary' : currentStep === 'answer_question' || currentStep === 'reset_password' ? 'text-green-500' : 'text-muted-foreground'}`}>
                                <div className={`rounded-full p-2 ${currentStep === 'enter_token' ? 'bg-primary/10' : currentStep === 'answer_question' || currentStep === 'reset_password' ? 'bg-green-500/10' : 'bg-muted'}`}>
                                    <Key className="h-4 w-4" />
                                </div>
                                <span className="text-xs font-medium">Token</span>
                            </div>
                            <div className="h-[2px] flex-1 mx-2 bg-border" />
                            <div className={`flex flex-col items-center gap-2 ${currentStep === 'answer_question' ? 'text-primary' : currentStep === 'reset_password' ? 'text-green-500' : 'text-muted-foreground'}`}>
                                <div className={`rounded-full p-2 ${currentStep === 'answer_question' ? 'bg-primary/10' : currentStep === 'reset_password' ? 'bg-green-500/10' : 'bg-muted'}`}>
                                    <Shield className="h-4 w-4" />
                                </div>
                                <span className="text-xs font-medium">Verify</span>
                            </div>
                            <div className="h-[2px] flex-1 mx-2 bg-border" />
                            <div className={`flex flex-col items-center gap-2 ${currentStep === 'reset_password' ? 'text-primary' : 'text-muted-foreground'}`}>
                                <div className={`rounded-full p-2 ${currentStep === 'reset_password' ? 'bg-primary/10' : 'bg-muted'}`}>
                                    <Key className="h-4 w-4" />
                                </div>
                                <span className="text-xs font-medium">Reset</span>
                            </div>
                        </div>
                    </CardHeader>

                    <CardContent className="space-y-4">
                        {/* Step 1: Select Friend */}
                        {currentStep === 'select_friend' && (
                            <Form {...friendSelectionForm}>
                                <form onSubmit={friendSelectionForm.handleSubmit(onSubmitFriendSelection)} className="space-y-4">
                                    <Alert className="bg-muted/50">
                                        <AlertCircle className="h-4 w-4" />
                                        <AlertDescription className="text-sm">
                                            Select one of your trusted recovery friends. You'll need to contact them outside of the platform to get a recovery code.
                                        </AlertDescription>
                                    </Alert>

                                    <FormField
                                        control={friendSelectionForm.control}
                                        name="selected_friend"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Select Recovery Friend</FormLabel>
                                                <FormControl>
                                                    <div className="grid grid-cols-2 gap-2">
                                                        {recoveryFriends.map(friend => (
                                                            <button
                                                                key={friend}
                                                                type="button"
                                                                onClick={() => field.onChange(friend)}
                                                                className={`p-3 rounded-lg border-2 transition-colors ${field.value === friend
                                                                        ? 'border-primary bg-primary/10'
                                                                        : 'border-border hover:border-primary/50'
                                                                    }`}
                                                            >
                                                                <div className="font-medium">@{friend}</div>
                                                            </button>
                                                        ))}
                                                    </div>
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />

                                    <FormField
                                        control={friendSelectionForm.control}
                                        name="friend_username_confirmation"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Confirm Friend's Username</FormLabel>
                                                <FormControl>
                                                    <Input
                                                        type="text"
                                                        placeholder="Enter the friend's username"
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormDescription>
                                                    Type the username of the friend you selected to confirm your choice
                                                </FormDescription>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />

                                    {error && (
                                        <Alert>
                                            <AlertCircle className="h-4 w-4" />
                                            <AlertDescription>{error}</AlertDescription>
                                        </Alert>
                                    )}

                                    <Button type="submit" className="w-full gap-2" disabled={isLoading}>
                                        {isLoading ? 'Processing...' : (
                                            <>
                                                Continue
                                                <ArrowRight className="h-4 w-4" />
                                            </>
                                        )}
                                    </Button>
                                </form>
                            </Form>
                        )}

                        {/* Step 2: Enter Friend's Token */}
                        {currentStep === 'enter_token' && (
                            <Form {...tokenEntryForm}>
                                <form onSubmit={tokenEntryForm.handleSubmit(onSubmitToken)} className="space-y-4">
                                    <Alert className="bg-blue-500/10 border-blue-500/20">
                                        <AlertCircle className="h-4 w-4 text-blue-500" />
                                        <AlertDescription className="text-blue-700 dark:text-blue-400">
                                            A recovery token has been sent to <span className="font-semibold">@{selectedFriend}</span>.
                                            Contact them outside of the platform (phone, in-person, etc.) to get the token.
                                        </AlertDescription>
                                    </Alert>

                                    <FormField
                                        control={tokenEntryForm.control}
                                        name="friend_token"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Recovery Token</FormLabel>
                                                <FormControl>
                                                    <Input
                                                        type="text"
                                                        placeholder="FRT-XXXX-XXXX-XXXX"
                                                        className="font-mono uppercase"
                                                        {...field}
                                                        onChange={(e) => field.onChange(e.target.value.toUpperCase())}
                                                    />
                                                </FormControl>
                                                <FormDescription>
                                                    Enter the recovery token your friend received
                                                </FormDescription>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />

                                    {error && (
                                        <Alert>
                                            <AlertCircle className="h-4 w-4" />
                                            <AlertDescription>{error}</AlertDescription>
                                        </Alert>
                                    )}

                                    <div className="flex gap-2">
                                        <Button
                                            type="button"
                                            onClick={() => {
                                                setCurrentStep('select_friend');
                                                setError('');
                                            }}
                                            className="flex-1 gap-2 bg-secondary text-secondary-foreground hover:bg-secondary/80"
                                        >
                                            <ArrowLeft className="h-4 w-4" />
                                            Back
                                        </Button>
                                        <Button type="submit" className="flex-1 gap-2" disabled={isLoading}>
                                            {isLoading ? 'Validating...' : (
                                                <>
                                                    Continue
                                                    <ArrowRight className="h-4 w-4" />
                                                </>
                                            )}
                                        </Button>
                                    </div>
                                </form>
                            </Form>
                        )}

                        {/* Step 3: Answer Personal Validation Question */}
                        {currentStep === 'answer_question' && (
                            <Form {...validationAnswerForm}>
                                <form onSubmit={validationAnswerForm.handleSubmit(onSubmitValidationAnswer)} className="space-y-4">
                                    <Alert className="bg-muted/50">
                                        <Shield className="h-4 w-4" />
                                        <AlertDescription className="text-sm">
                                            Answer your personal security question to unlock the password reset
                                        </AlertDescription>
                                    </Alert>

                                    <div className="bg-muted/50 rounded-lg p-4">
                                        <p className="text-sm font-medium text-muted-foreground mb-2">Security Question</p>
                                        <p className="text-foreground font-medium">{validationQuestion}</p>
                                    </div>

                                    <FormField
                                        control={validationAnswerForm.control}
                                        name="validation_answer"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Your Answer</FormLabel>
                                                <FormControl>
                                                    <Input
                                                        type="text"
                                                        placeholder="Enter your answer"
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormDescription>
                                                    This is the answer you configured when setting up recovery friends
                                                </FormDescription>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />

                                    {error && (
                                        <Alert>
                                            <AlertCircle className="h-4 w-4" />
                                            <AlertDescription>{error}</AlertDescription>
                                        </Alert>
                                    )}

                                    <div className="flex gap-2">
                                        <Button
                                            type="button"
                                            onClick={() => {
                                                setCurrentStep('enter_token');
                                                setError('');
                                            }}
                                            className="flex-1 gap-2 bg-secondary text-secondary-foreground hover:bg-secondary/80"
                                        >
                                            <ArrowLeft className="h-4 w-4" />
                                            Back
                                        </Button>
                                        <Button type="submit" className="flex-1 gap-2" disabled={isLoading}>
                                            {isLoading ? 'Verifying...' : (
                                                <>
                                                    Continue
                                                    <ArrowRight className="h-4 w-4" />
                                                </>
                                            )}
                                        </Button>
                                    </div>
                                </form>
                            </Form>
                        )}

                        {/* Step 4: Reset Password */}
                        {currentStep === 'reset_password' && (
                            <Form {...resetPasswordForm}>
                                <form onSubmit={resetPasswordForm.handleSubmit(onSubmitResetPassword)} className="space-y-4">
                                    <Alert className="bg-green-500/10 border-green-500/20">
                                        <CheckCircle className="h-4 w-4 text-green-500" />
                                        <AlertDescription className="text-green-700 dark:text-green-400">
                                            Verification successful! You can now create a new password.
                                        </AlertDescription>
                                    </Alert>

                                    <FormField
                                        control={resetPasswordForm.control}
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

                                    <FormField
                                        control={resetPasswordForm.control}
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
                                                        placeholder="Confirm your password"
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />

                                    {error && (
                                        <Alert>
                                            <AlertCircle className="h-4 w-4" />
                                            <AlertDescription>{error}</AlertDescription>
                                        </Alert>
                                    )}

                                    <Button type="submit" className="w-full" disabled={isLoading}>
                                        {isLoading ? 'Resetting Password...' : 'Reset Password'}
                                    </Button>
                                </form>
                            </Form>
                        )}
                    </CardContent>

                    <CardFooter className="flex flex-col space-y-2">
                        <div className="text-sm text-muted-foreground text-center">
                            Remember your password?{' '}
                            <a href="/login" className="text-primary hover:underline">
                                Sign in
                            </a>
                        </div>
                        {currentStep === 'select_friend' && (
                            <div className="text-sm text-muted-foreground text-center">
                                Try a different recovery method?{' '}
                                <a href="/forgot-password" className="text-primary hover:underline">
                                    Go back
                                </a>
                            </div>
                        )}
                    </CardFooter>
                </Card>

                <div className="mt-6 text-center text-sm text-muted-foreground">
                    Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                </div>
            </div>
        </CenteredLayout>
    );
}
