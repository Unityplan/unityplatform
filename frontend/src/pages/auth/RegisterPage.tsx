import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { useAuthStore } from '@/stores/authStore';
import { validateInvitation } from '@/api/auth';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage, FormDescription } from '@/components/ui/form';

// Invitation validation schema
const invitationSchema = z.object({
    invitation_token: z.string().min(1, 'Invitation token is required'),
    territory_code: z.string().min(2, 'Please select a territory'),
});

// Registration form validation schema
const registerSchema = z.object({
    invitation_token: z.string().min(1, 'Invitation token is required'),
    territory_code: z.string().min(2, 'Please select a territory'),
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

// Available territories
const TERRITORIES = [
    { code: 'dk', name: 'Denmark' },
    { code: 'no', name: 'Norway' },
    { code: 'se', name: 'Sweden' },
    { code: 'eu', name: 'Europe' },
];

export function RegisterPage() {
    const { register: registerUser, isLoading, error } = useAuthStore();
    const [step, setStep] = useState<'invitation' | 'registration'>('invitation');
    const [validatedToken, setValidatedToken] = useState<string>('');
    const [validatedTerritory, setValidatedTerritory] = useState<string>('');
    const [invitationError, setInvitationError] = useState<string>('');
    const [showPassword, setShowPassword] = useState(false);
    const [showConfirmPassword, setShowConfirmPassword] = useState(false);

    const invitationForm = useForm<InvitationFormValues>({
        resolver: zodResolver(invitationSchema),
        defaultValues: {
            invitation_token: '',
            territory_code: 'dk',
        },
    });

    const registerForm = useForm<RegisterFormValues>({
        resolver: zodResolver(registerSchema),
        defaultValues: {
            invitation_token: '',
            territory_code: '',
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
            const response = await validateInvitation(data.invitation_token, data.territory_code);

            if (response.valid) {
                // Token is valid, move to registration step
                setValidatedToken(data.invitation_token);
                setValidatedTerritory(data.territory_code);
                registerForm.setValue('invitation_token', data.invitation_token);
                registerForm.setValue('territory_code', data.territory_code);
                if (response.email) {
                    registerForm.setValue('email', response.email);
                }
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
            await registerUser({
                email: data.email || undefined,
                username: data.username,
                password: data.password,
                full_name: data.full_name || undefined,
                territory_code: data.territory_code,
                invitation_token: data.invitation_token,
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
            <div className="flex min-h-screen items-center justify-center bg-background p-4">
                <Card className="w-full max-w-md">
                    <CardHeader>
                        <CardTitle>Welcome to UnityPlan</CardTitle>
                        <CardDescription>Enter your invitation token to create an account</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <Form {...invitationForm}>
                            <form onSubmit={invitationForm.handleSubmit(onValidateInvitation)} className="space-y-4">
                                {/* Territory Selection */}
                                <FormField
                                    control={invitationForm.control}
                                    name="territory_code"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Territory</FormLabel>
                                            <Select onValueChange={field.onChange} defaultValue={field.value}>
                                                <FormControl>
                                                    <SelectTrigger>
                                                        <SelectValue placeholder="Select your territory" />
                                                    </SelectTrigger>
                                                </FormControl>
                                                <SelectContent>
                                                    {TERRITORIES.map((territory) => (
                                                        <SelectItem key={territory.code} value={territory.code}>
                                                            {territory.name}
                                                        </SelectItem>
                                                    ))}
                                                </SelectContent>
                                            </Select>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />

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
            </div>
        );
    }

    return (
        <div className="flex min-h-screen items-center justify-center bg-background p-4">
            <Card className="w-full max-w-md">
                <CardHeader>
                    <CardTitle>Create Your Account</CardTitle>
                    <CardDescription>
                        Complete your profile to join {TERRITORIES.find(t => t.code === validatedTerritory)?.name}
                    </CardDescription>
                </CardHeader>
                <CardContent>
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
        </div>
    );
}
