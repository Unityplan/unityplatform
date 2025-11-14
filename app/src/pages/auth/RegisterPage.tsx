import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { useAuthStore } from '@/stores/authStore';
import { validateInvitation } from '@/api/auth';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';
import { CenteredLayout } from '@/components/layouts/CenteredLayout';
import { Logo } from '@/components/ui/logo';
import { ModeToggle } from '@/components/mode-toggle';
import { Shield, Users } from 'lucide-react';

// Invitation validation schema (⭐ NO territory_code - backend looks it up)
const invitationSchema = z.object({
    invitation_token: z.string().min(1, 'Invitation token is required'),
});

// Registration form validation schema (⭐ NO territory_code - derived from token)
const registerSchema = z.object({
    username: z.string()
        .min(3, 'Username must be at least 3 characters')
        .max(50, 'Username must be less than 50 characters')
        .regex(/^[a-zA-Z0-9_-]+$/, 'Username can only contain letters, numbers, underscores, and hyphens'),
    email: z.string().email('Invalid email address').optional().or(z.literal('')),
    password: z.string()
        .min(8, 'Password must be at least 8 characters')
        .regex(/[A-Z]/, 'Password must contain at least one uppercase letter')
        .regex(/[a-z]/, 'Password must contain at least one lowercase letter')
        .regex(/[0-9]/, 'Password must contain at least one number')
        .regex(/[^A-Za-z0-9]/, 'Password must contain at least one special character'),
    confirm_password: z.string().min(1, 'Please confirm your password'),
    full_name: z.string().max(255, 'Name must be less than 255 characters').optional(),
}).refine((data) => data.password === data.confirm_password, {
    message: "Passwords don't match",
    path: ['confirm_password'],
});

type InvitationFormValues = z.infer<typeof invitationSchema>;
type RegisterFormValues = z.infer<typeof registerSchema>;

// ⭐ NEW: Type for validated invitation response (from backend)
interface ValidatedInvitation {
    valid: boolean;
    token_type: string;
    territory: {
        code: string;
        name: string;
    };
    community?: {
        id: string;
        name: string;
    };
    email?: string;
    expires_at?: string;
    remaining_uses?: number;
}

export function RegisterPage() {
    const { register: registerUser, isLoading, error } = useAuthStore();
    const [step, setStep] = useState<'invitation' | 'registration'>('invitation');
    const [validatedInvitation, setValidatedInvitation] = useState<ValidatedInvitation | null>(null);
    const [invitationToken, setInvitationToken] = useState<string>('');
    const [invitationError, setInvitationError] = useState<string>('');
    const [showPassword, setShowPassword] = useState(false);
    const [showConfirmPassword, setShowConfirmPassword] = useState(false);

    const invitationForm = useForm<InvitationFormValues>({
        resolver: zodResolver(invitationSchema),
        defaultValues: {
            invitation_token: '',
        },
    });

    const registerForm = useForm<RegisterFormValues>({
        resolver: zodResolver(registerSchema),
        defaultValues: {
            username: '',
            email: '',
            password: '',
            confirm_password: '',
            full_name: '',
        },
    });

    const onValidateInvitation = async (data: InvitationFormValues) => {
        try {
            setInvitationError('');
            // ⭐ SECURE: Backend looks up territory from global registry
            const response = await validateInvitation(data.invitation_token);

            if (response.valid) {
                // Token is valid, store invitation details and move to registration step
                setValidatedInvitation(response);
                setInvitationToken(data.invitation_token);
                setStep('registration');
            } else {
                setInvitationError('Invalid or expired invitation token');
            }
        } catch (err) {
            setInvitationError('Failed to validate invitation token. Please try again.');
            console.error('Invitation validation failed:', err);
        }
    };

    const onSubmitRegistration = async (data: RegisterFormValues) => {
        try {
            // ⭐ SECURE: No territory_code sent - backend derives it from invitation token
            await registerUser({
                email: data.email || '',
                username: data.username,
                password: data.password,
                full_name: data.full_name || undefined,
                invitation_token: invitationToken,  // From Step 1 validation
            });
            // TODO: Redirect to dashboard once routing is set up
            console.log('Registration successful!');
        } catch (err) {
            // Error is already handled by the store
            console.error('Registration failed:', err);
        }
    };

    if (step === 'invitation') {
        return (
            <CenteredLayout>
                {/* Dark Mode Toggle - Fixed to top right */}
                <div className="fixed top-4 right-4 z-10">
                    <ModeToggle />
                </div>

                <div className="w-full max-w-md">
                    <div className="mb-8 flex justify-center">
                        <Logo className="text-foreground" />
                    </div>
                    <Card className="w-full">
                        <CardHeader>
                            <CardTitle className="text-foreground">Welcome to Unity Platform</CardTitle>
                            <CardDescription className="text-muted-foreground">Enter your invitation token to create an account</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <Form {...invitationForm}>
                                <form onSubmit={invitationForm.handleSubmit(onValidateInvitation)} className="space-y-4">
                                    {/* ⭐ REMOVED: Territory selector - backend looks it up from token */}

                                    {/* Invitation Token Field */}
                                    <FormField
                                        control={invitationForm.control}
                                        name="invitation_token"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Invitation Token</FormLabel>
                                                <FormControl>
                                                    <Input
                                                        type="text"
                                                        placeholder="inv_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormDescription>
                                                    Enter the invitation token you received from your territory administrator
                                                </FormDescription>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />

                                    {/* Error Message */}
                                    {invitationError && (
                                        <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive">
                                            {invitationError}
                                        </div>
                                    )}

                                    {/* Submit Button */}
                                    <Button type="submit" className="w-full">
                                        Validate Invitation
                                    </Button>
                                </form>
                            </Form>
                        </CardContent>
                        <CardFooter className="flex flex-col space-y-2">
                            <div className="text-sm text-muted-foreground">
                                Already have an account?{' '}
                                <a href="/login" className="text-primary hover:underline">
                                    Sign in
                                </a>
                            </div>
                        </CardFooter>
                    </Card>

                    {/* Platform branding */}
                    <div className="mt-6 text-center text-sm text-muted-foreground">
                        Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                    </div>
                </div>
            </CenteredLayout>
        );
    }

    return (
        <CenteredLayout>
            {/* Dark Mode Toggle - Fixed to top right */}
            <div className="fixed top-4 right-4 z-10">
                <ModeToggle />
            </div>

            <div className="w-full max-w-md">
                <div className="mb-8 flex justify-center">
                    <Logo className="text-foreground" />
                </div>
                <Card className="w-full">
                    <CardHeader>
                        <CardTitle className="text-foreground">Create Your Account</CardTitle>
                        <CardDescription className="text-muted-foreground">
                            Complete your profile to join the Unity Platform
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        {/* ⭐ NEW: Display invitation context from backend */}
                        {validatedInvitation && (
                            <div className="mb-6 space-y-4 rounded-lg border border-border bg-muted/50 p-4">
                                <div className="flex items-center gap-2">
                                    <Shield className="size-5 text-primary" />
                                    <h3 className="font-semibold text-foreground">Invitation Details</h3>
                                </div>

                                <div className="space-y-2 text-sm">
                                    <div>
                                        <span className="text-muted-foreground">Territory:</span>{' '}
                                        <span className="font-medium text-foreground">{validatedInvitation.territory.name}</span>
                                    </div>

                                    {validatedInvitation.community && (
                                        <div className="flex items-center gap-2">
                                            <Users className="size-4 text-muted-foreground" />
                                            <div>
                                                <span className="text-muted-foreground">Community:</span>{' '}
                                                <span className="font-medium text-foreground">{validatedInvitation.community.name}</span>
                                            </div>
                                        </div>
                                    )}
                                </div>
                            </div>
                        )}

                        <Form {...registerForm}>
                            <form onSubmit={registerForm.handleSubmit(onSubmitRegistration)} className="space-y-4">
                                {/* Username Field */}
                                <FormField
                                    control={registerForm.control}
                                    name="username"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Username *</FormLabel>
                                            <FormControl>
                                                <Input
                                                    type="text"
                                                    placeholder="username"
                                                    autoComplete="username"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormDescription>
                                                Your unique identifier (letters, numbers, _ and - only)
                                            </FormDescription>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Email Field (optional unless token is email-specific) */}
                                <FormField
                                    control={registerForm.control}
                                    name="email"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Email (optional)</FormLabel>
                                            <FormControl>
                                                <Input
                                                    type="email"
                                                    placeholder="email@example.com"
                                                    autoComplete="email"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Full Name Field */}
                                <FormField
                                    control={registerForm.control}
                                    name="full_name"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Full Name (optional)</FormLabel>
                                            <FormControl>
                                                <Input
                                                    type="text"
                                                    placeholder="Your full name"
                                                    autoComplete="name"
                                                    {...field}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

                                {/* Password Field */}
                                <FormField
                                    control={registerForm.control}
                                    name="password"
                                    render={({ field }) => (
                                        <FormItem>
                                            <div className="flex items-center justify-between">
                                                <FormLabel>Password *</FormLabel>
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
                                    control={registerForm.control}
                                    name="confirm_password"
                                    render={({ field }) => (
                                        <FormItem>
                                            <div className="flex items-center justify-between">
                                                <FormLabel>Confirm Password *</FormLabel>
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

                                {/* Submit Button */}
                                <Button type="submit" className="w-full" disabled={isLoading}>
                                    {isLoading ? 'Creating account...' : 'Create Account'}
                                </Button>
                            </form>
                        </Form>
                    </CardContent>
                    <CardFooter className="flex flex-col space-y-2">
                        <div className="text-sm text-muted-foreground">
                            Already have an account?{' '}
                            <a href="/login" className="text-primary hover:underline">
                                Sign in
                            </a>
                        </div>
                    </CardFooter>
                </Card>

                {/* Platform branding */}
                <div className="mt-6 text-center text-sm text-muted-foreground">
                    Powered by Unity Platform <span className="font-mono">v0.1.0-alpha.1</span>
                </div>
            </div>
        </CenteredLayout>
    );
}
